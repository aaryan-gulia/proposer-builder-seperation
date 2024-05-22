use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use crate::entities::{proposer, traits};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::de::value::EnumAccessDeserializer;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub enum BuilderType {
    NormalBuilder(NormalBuilder),
    MevBuilder(MevBuilder),
}

#[derive(Debug, Clone, Serialize)]
pub struct NormalBuilder {
    pub builder: Builder,
    pub proposer: Option<proposer::Proposer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MevBuilder {
    pub builder: Builder,
    pub proposer: Option<proposer::Proposer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Builder {
    pub id: u32,
    pub characteristic: f64,
    pub mempool: HashSet<transaction::Transaction>,
}
static mut BUILDER_ID_COUNTER: u32 = 0;
impl Builder {
    pub fn new(id: u32, characteristic: f64) -> Self {
        unsafe {
            BUILDER_ID_COUNTER += 1;
        }

        Builder {
            id: unsafe { BUILDER_ID_COUNTER },
            characteristic,
            mempool: vec![].into_iter().collect(),
        }
    }
    pub fn reset() {
        unsafe {
            BUILDER_ID_COUNTER = 0;
        }
    }
    pub fn collect_transaction(&mut self, transaction_vec: &HashSet<transaction::Transaction>) {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(0.0..100.0);
        for t in transaction_vec {
            if self.characteristic * 100.0 > dist.sample(&mut rng) {
                self.mempool.insert(*t);
            }
        }
    }

    pub fn clean_mempools(&mut self, remove_transactions: &HashSet<transaction::Transaction>) {
        self.mempool.retain(|t| !remove_transactions.contains(t));
    }
    pub fn calculate_bid(
        block_value: i64,
        blockchain: &[block::Block],
        sample_size: u32,
        random_numbers: &Vec<f64>,
    ) -> f64 {
        let mut optimal_bid = 0;
        let mut max_utility = 0;
        let mut rng = thread_rng();
        let uniform = Uniform::new(0, random_numbers.len());
        let mut start = uniform.sample(&mut rng);
        for bid in 0..block_value as u32 {
            let mut total_utility = 0;
            for _ in 0..(sample_size * 10) as u32 {
                let random_number =
                    (random_numbers.get(start).unwrap() * sample_size as f64) as u32;
                if random_number > sample_size {
                    panic!();
                }
                if bid
                    > blockchain
                        .get(random_number as usize)
                        .unwrap()
                        .block_inclusion_bid
                        .expect(
                            "blocks included in blockchain must have a bid value
                            if calculate_bid is being called!",
                        ) as u32
                {
                    total_utility += block_value - bid as i64;
                }
            }
            if total_utility / 10 >= max_utility {
                max_utility = total_utility / 10;
                optimal_bid = bid;
            }
        }
        optimal_bid as f64
    }

    pub fn parse_bid_calculation(
        block_value: i64,
        blockchain: &Vec<block::Block>,
        random_numbers: &Vec<f64>,
    ) -> f64 {
        let mut bid: f64 = 0.0;
        if blockchain.len() > 10 {
            bid = Builder::calculate_bid(
                block_value,
                &blockchain[blockchain.len() - 10..],
                10,
                random_numbers,
            );
        } else if blockchain.len() == 0 {
            bid = ((block_value) / 2) as f64
        } else {
            bid = Builder::calculate_bid(
                block_value,
                &blockchain[..],
                blockchain.len() as u32,
                random_numbers,
            )
        }
        bid
    }
}

impl NormalBuilder {
    pub fn build_block(&self, mut block_size: u32) -> block::Block {
        let mut gas_vec: Vec<transaction::Transaction> = vec![];
        gas_vec.reserve(self.builder.mempool.len());
        for t in self.builder.mempool.iter() {
            gas_vec.push(*t);
        }
        gas_vec.sort_unstable_by(transaction::Transaction::compare_transaction_by_gas);
        if block_size > gas_vec.len() as u32 {
            block_size = gas_vec.len() as u32;
        }
        let mev_captured = 0;
        let mut gas_captured = 0;
        let mut transactions_in_block: HashSet<transaction::Transaction> =
            vec![].into_iter().collect();
        for i in 0..std::cmp::min(block_size as usize, gas_vec.len()) {
            gas_captured += gas_vec[i as usize].gas_amount;
            transactions_in_block.insert(gas_vec[i as usize].clone());
        }
        block::Block::new(
            self.builder.id,
            gas_captured as f64,
            mev_captured as f64,
            None,
            transactions_in_block,
            BuilderType::NormalBuilder(self.clone()),
        )
    }
}

impl MevBuilder {
    pub fn build_block(&self, mut block_size: u32) -> block::Block {
        let mut gas_vec: Vec<transaction::Transaction> = vec![];
        let mut mev_gas_vec: Vec<transaction::Transaction> = vec![];
        let curr_block_size = std::cmp::min(self.builder.mempool.len(), block_size as usize);
        gas_vec.reserve(curr_block_size);
        mev_gas_vec.reserve(curr_block_size);
        for t in self.builder.mempool.iter() {
            gas_vec.push(*t);
            mev_gas_vec.push(*t);
        }
        gas_vec[0..curr_block_size]
            .sort_unstable_by(transaction::Transaction::compare_transaction_by_gas);
        gas_vec.drain(curr_block_size..);
        mev_gas_vec[0..curr_block_size]
            .sort_unstable_by(transaction::Transaction::compare_transaction_by_total);
        mev_gas_vec.drain(curr_block_size..);
        assert_eq!(mev_gas_vec.len(), curr_block_size);
        assert_eq!(gas_vec.len(), curr_block_size);
        let mut mev_captured = 0;
        let mut gas_captured = 0;
        let mut transactions_in_block: HashSet<transaction::Transaction> =
            vec![].into_iter().collect();
        let mut gas_ptr: usize = 0;
        let mut mev_ptr: usize = 0;
        while transactions_in_block.len() <= curr_block_size {
            let curr_gas_tx = gas_vec.get(gas_ptr);
            let curr_mev_tx = mev_gas_vec.get(mev_ptr);
            if curr_gas_tx.is_none() && curr_mev_tx.is_none() {
                break;
            } else if curr_gas_tx.is_none() {
                if transactions_in_block.contains(curr_mev_tx.unwrap()) {
                    mev_ptr += 1;
                    continue;
                }
                if curr_block_size - 1 > transactions_in_block.len() {
                    let next_mev_tx = mev_gas_vec.get(mev_ptr + 1);
                    match next_mev_tx {
                        Some(t) => {
                            if t.gas_amount
                                + curr_mev_tx
                                    .expect("this block should have a valid curr_mev_tx")
                                    .gas_amount
                                < curr_mev_tx.unwrap().gas_amount
                                    + curr_mev_tx.unwrap().max_mev_amount
                            {
                                mev_captured += curr_mev_tx.unwrap().max_mev_amount;
                                gas_captured += curr_mev_tx.unwrap().gas_amount;
                                self.attack_transaction(
                                    &mut transactions_in_block,
                                    *curr_mev_tx.unwrap(),
                                );
                                mev_ptr += 1;
                            } else {
                                gas_captured += curr_mev_tx.unwrap().gas_amount;
                                transactions_in_block.insert(*curr_mev_tx.unwrap());
                                mev_ptr += 1;
                            }
                        }
                        None => {
                            mev_captured += curr_mev_tx.unwrap().max_mev_amount;
                            gas_captured += curr_mev_tx.unwrap().gas_amount;
                            self.attack_transaction(
                                &mut transactions_in_block,
                                *curr_mev_tx.unwrap(),
                            );
                            mev_ptr += 1;
                        }
                    }
                } else {
                    gas_captured += curr_mev_tx.unwrap().gas_amount;
                    transactions_in_block.insert(*curr_mev_tx.unwrap());
                    mev_ptr += 1
                }
            } else if curr_mev_tx.is_none() {
                if transactions_in_block.contains(curr_gas_tx.unwrap()) {
                    gas_ptr += 1;
                    continue;
                }
                gas_captured += curr_gas_tx.unwrap().gas_amount;
                transactions_in_block.insert(*curr_gas_tx.unwrap());
                gas_ptr += 1;
            } else {
                if transactions_in_block.contains(curr_mev_tx.unwrap()) {
                    mev_ptr += 1;
                    continue;
                }
                if transactions_in_block.contains(curr_gas_tx.unwrap()) {
                    gas_ptr += 1;
                    continue;
                }
                if curr_block_size - 1 <= transactions_in_block.len() {
                    gas_captured += curr_gas_tx.unwrap().gas_amount;
                    transactions_in_block.insert(*curr_gas_tx.unwrap());
                    gas_ptr += 1;
                    continue;
                }
                let next_gas_tx = gas_vec.get(gas_ptr + 1);
                match next_gas_tx {
                    Some(t) => {
                        if t.gas_amount + curr_gas_tx.unwrap().gas_amount
                            < curr_mev_tx.unwrap().max_mev_amount + curr_mev_tx.unwrap().gas_amount
                        {
                            mev_captured += curr_mev_tx.unwrap().max_mev_amount;
                            gas_captured += curr_mev_tx.unwrap().gas_amount;
                            self.attack_transaction(
                                &mut transactions_in_block,
                                *curr_mev_tx.unwrap(),
                            );
                            mev_ptr += 1;
                        } else {
                            gas_captured += curr_gas_tx.unwrap().gas_amount;
                            transactions_in_block.insert(*curr_gas_tx.unwrap());
                            gas_ptr += 1;
                        }
                    }
                    None => {
                        if curr_gas_tx.unwrap().gas_amount
                            < curr_mev_tx.unwrap().gas_amount + curr_mev_tx.unwrap().max_mev_amount
                        {
                            mev_captured += curr_mev_tx.unwrap().max_mev_amount;
                            gas_captured += curr_mev_tx.unwrap().gas_amount;
                            self.attack_transaction(
                                &mut transactions_in_block,
                                *curr_mev_tx.unwrap(),
                            );
                            mev_ptr += 1;
                        } else {
                            gas_captured += curr_gas_tx.unwrap().gas_amount;
                            transactions_in_block.insert(*curr_gas_tx.unwrap());
                            gas_ptr += 1;
                        }
                    }
                }
            }
        }

        block::Block::new(
            self.builder.id,
            gas_captured as f64,
            mev_captured as f64,
            None,
            transactions_in_block,
            BuilderType::MevBuilder(self.clone()),
        )
    }

    pub fn attack_transaction(
        &self,
        transactions: &mut HashSet<transaction::Transaction>,
        mut mev_tx: transaction::Transaction,
    ) {
        transactions.insert(*mev_tx.attack_transaction());
        let mut attack_tx = transaction::TransactionBuilder::new();
        let attack_tx = attack_tx
            .gas_amount(0)
            .max_mev_amount(0)
            .transaction_type(transaction::TransactionType::Attack)
            .block_created(0)
            .build();
        transactions.insert(attack_tx.unwrap());
    }
}
