use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use crate::entities::{proposer, traits};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

pub enum BuilderType {
    NormalBuilder(NormalBuilder),
    MevBuilder(MevBuilder),
}

pub struct NormalBuilder {
    pub builder: Builder,
    pub proposer: Option<proposer::Proposer>,
}
pub struct MevBuilder {
    pub builder: Builder,
    pub proposer: Option<proposer::Proposer>,
}
pub struct Builder {
    pub id: u32,
    pub characteristic: f64,
    pub mempool: HashSet<transaction::Transaction>,
}

impl Builder {
    pub fn new(id: u32, characteristic: f64) -> Self {
        Builder {
            id,
            characteristic,
            mempool: vec![].into_iter().collect(),
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
                        .block_inclusion_bid as u32
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
}

impl NormalBuilder {
    pub fn build_block(
        &self,
        mut block_size: u32,
        blockchain: &Vec<block::Block>,
        random_numbers: &Vec<f64>,
    ) -> block::Block {
        let mut gas_vec: Vec<transaction::Transaction> = vec![];
        gas_vec.reserve(self.builder.mempool.len());
        for t in self.builder.mempool.iter() {
            gas_vec.push(*t);
        }
        gas_vec.sort_unstable_by(transaction::Transaction::compare_transaction_by_gas);
        println!("{:?}", gas_vec);
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
        let mut bid: f64 = 0.0;
        if blockchain.len() > 10 {
            bid = Builder::calculate_bid(
                gas_captured + mev_captured,
                &blockchain[blockchain.len() - 10..],
                10,
                random_numbers,
            );
        } else if blockchain.len() == 0 {
            bid = ((gas_captured + mev_captured) / 2) as f64
        } else {
            bid = Builder::calculate_bid(
                gas_captured + mev_captured,
                &blockchain[..],
                blockchain.len() as u32,
                random_numbers,
            )
        }
        block::Block::new(
            self.builder.id,
            gas_captured as f64,
            mev_captured as f64,
            bid,
            transactions_in_block,
        )
    }
}

impl MevBuilder {
    pub fn build_block(
        &self,
        mut block_size: u32,
        blockchain: &Vec<block::Block>,
        random_numbers: &Vec<f64>,
    ) -> block::Block {
        todo!()
    }
}
