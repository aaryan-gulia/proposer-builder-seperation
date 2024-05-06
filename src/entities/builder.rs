use crate::blockchain_env::transaction;
use crate::entities::traits;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub struct Builder {
    pub builder_id: u32,
    pub characteristic: f64,
    pub mempool: Vec<transaction::Transaction>,
}

impl Builder {
    pub fn new(builder_id: u32, characteristic: f64) -> Self {
        Builder {
            builder_id,
            characteristic,
            mempool: vec![],
        }
    }
    pub fn collect_transaction(&mut self, transaction_vec: &Vec<transaction::Transaction>) {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(0.0..100.0);
        for t in transaction_vec {
            if self.characteristic * 100.0 > dist.sample(&mut rng) {
                self.mempool.push(*t);
            }
        }
    }
}
