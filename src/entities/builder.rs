use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use crate::entities::traits;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;

pub struct Builder {
    pub builder_id: u32,
    pub characteristic: f64,
    pub mempool: HashSet<transaction::Transaction>,
}

impl Builder {
    pub fn new(builder_id: u32, characteristic: f64) -> Self {
        Builder {
            builder_id,
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
    pub fn build_block(&self, mut block_size: u32) -> block::Block {
        let mut gas_vec: Vec<transaction::Transaction> = vec![];
        gas_vec.reserve(self.mempool.len());
        for t in self.mempool.iter() {
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
        let bid = Builder::calculate_bid(gas_captured + mev_captured);
        block::Block::new(
            self.builder_id,
            gas_captured as f64,
            mev_captured as f64,
            bid,
            transactions_in_block,
        )
    }
    pub fn calculate_bid(block_value: i64) -> f64 {
        block_value as f64
    }
}
