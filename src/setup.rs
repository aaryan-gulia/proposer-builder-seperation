pub mod init {

    use crate::blockchain_env::*;
    use crate::entities::*;
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal, NormalError, Uniform};
    use std::collections::HashSet;
    pub fn initiate_builders(
        num_builders: u32,
        builder_characteristic: f64,
    ) -> Vec<builder::BuilderType> {
        let mut builder_vec: Vec<builder::BuilderType> = vec![];
        for id in 1..=num_builders {
            builder_vec.push(builder::BuilderType::MevBuilder(builder::MevBuilder {
                builder: builder::Builder::new(id, builder_characteristic),
                proposer: None,
            }));
        }
        builder_vec
    }

    pub fn initiate_proposers(num_proposers: u32) -> Vec<proposer::Proposer> {
        let mut proposer_vec: Vec<proposer::Proposer> = vec![];
        for id in 1..=num_proposers {
            proposer_vec.push(proposer::Proposer::new(id));
        }
        proposer_vec
    }

    pub fn initiate_builders_from_normal_dist(
        num_builders: u32,
        mean_characteristic: f64,
        std: f64,
    ) -> Vec<builder::Builder> {
        let mut builder_vec: Vec<builder::Builder> = vec![];
        let mut rng = thread_rng();
        let normal = Normal::new(mean_characteristic, std).unwrap();
        for id in 1..=num_builders {
            builder_vec.push(builder::Builder::new(id, normal.sample(&mut rng)));
        }
        builder_vec
    }

    pub fn initiate_transactions_default(
        num_transaction: u32,
    ) -> HashSet<transaction::Transaction> {
        transaction::TransactionBuilder::reset();
        let mut transaction_set: HashSet<transaction::Transaction> = vec![].into_iter().collect();
        let mut rng = thread_rng();
        let uniform = Uniform::new(0.0, 100.0);
        for _ in 0..num_transaction {
            let mut t = transaction::TransactionBuilder::new();
            let mut t = t
                .gas_amount(uniform.sample(&mut rng) as i64)
                .max_mev_amount(uniform.sample(&mut rng) as i64)
                .transaction_type(transaction::TransactionType::Normal)
                .build()
                .expect("initiate_transaction_default() failing. transaction build failing");
            transaction_set.insert(t);
        }
        transaction_set
    }

    pub fn get_random_numbers<T: rand_distr::uniform::SampleUniform>(
        num_random_numbers: u32,
        range_begin: T,
        range_end: T,
    ) -> Vec<T> {
        let mut random_numbers: Vec<T> = Vec::with_capacity(num_random_numbers as usize);
        let mut rng = thread_rng();
        let uniform = Uniform::new(range_begin, range_end);
        for _ in 0..num_random_numbers {
            random_numbers.push(uniform.sample(&mut rng));
        }
        random_numbers
    }
}

pub mod maintain {

    use crate::blockchain_env::*;
    use crate::entities::*;
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal, NormalError, Uniform};
    use std::collections::HashSet;

    use super::init::initiate_transactions_default;

    pub fn refill_transactions_default(
        num_transactions: u32,
        transactions: &mut HashSet<transaction::Transaction>,
    ) {
        let temp_transactions = transactions.clone();
        let num_new_transactions = num_transactions - transactions.len() as u32;
        let new_transactions = initiate_transactions_default(num_new_transactions);
        let transactions_union: HashSet<&transaction::Transaction> =
            temp_transactions.union(&new_transactions).collect();

        transactions.clear();
        for t in transactions_union {
            transactions.insert(*t);
        }
    }
}
