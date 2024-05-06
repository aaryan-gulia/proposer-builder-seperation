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
}
