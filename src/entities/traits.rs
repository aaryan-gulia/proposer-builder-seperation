use crate::blockchain_env::block;
use crate::blockchain_env::transaction;
use rand::Rng;

pub trait Entity {
    fn get_characteristic(&self) -> f64;
    fn access_mempool(&self) -> &mut Vec<&Box<transaction::Transaction>>;
}
pub trait Builder: Entity {
    fn collect_transactions(&mut self, all_transactions: &Vec<Box<transaction::Transaction>>) {
        let mut rng = rand::thread_rng();
        for t in all_transactions.iter() {
            if self.get_characteristic() * 100.0 > rng.gen_range(0.0..100.0) {
                let mut mempool = self.access_mempool();
                mempool.push(t);
            }
        }
    }
}
