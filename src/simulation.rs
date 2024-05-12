use crate::blockchain_env::*;
use crate::entities::traits::Proposer;
use crate::entities::{builder, proposer};
use crate::setup::*;
use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError, Uniform};
use std::collections::HashSet;

pub fn execute_simulation(
    num_blocks: u32,
    builder_vec: &mut Vec<builder::BuilderType>,
    proposer_vec: &mut Vec<proposer::Proposer>,
    mut transaction_set: HashSet<transaction::Transaction>,
    random_number_vec: &Vec<f64>,
) -> Vec<block::Block> {
    let mut blockchain: Vec<block::Block> = vec![];
    let num_transactions = transaction_set.len() as u32;
    let mut rng = thread_rng();
    let uniform = Uniform::new(0, proposer_vec.len() as usize);
    for block_index in 1..=num_blocks {
        for b in builder_vec.iter_mut() {
            match b {
                builder::BuilderType::NormalBuilder(b) => {
                    b.builder.collect_transaction(&transaction_set)
                }
                builder::BuilderType::MevBuilder(b) => {
                    b.builder.collect_transaction(&transaction_set)
                }
            }
        }
        let block_proposer = uniform.sample(&mut rng);
        let mut proposed_block = proposer_vec[block_proposer].run_auction(
            builder_vec,
            10,
            &blockchain,
            random_number_vec,
        );
        proposer_vec[block_proposer]
            .propose_block(&proposer_vec[block_proposer], &mut proposed_block);
        transaction::Transaction::clean_transaction_set(
            &mut transaction_set,
            &proposed_block.transactions,
        );
        maintain::refill_transactions_default(num_transactions, &mut transaction_set);
        println!("{}", &proposed_block.get_block_index().unwrap());
        blockchain.push(proposed_block);
    }
    blockchain
}
