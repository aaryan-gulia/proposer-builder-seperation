use crate::blockchain_env::transaction;
use crate::entities::traits;

pub struct Builder {
    pub builder_id: u32,
    pub characteristic: f64,
    pub mempool: Vec<transaction::Transaction>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            builder_id: 1,
            characteristic: 1.0,
            mempool: vec![],
        }
    }
    pub fn collect_transaction(&mut self, transaction_vec: &Vec<transaction::Transaction>) {
        for t in transaction_vec {
            self.mempool.push(*t);
        }
    }
}
