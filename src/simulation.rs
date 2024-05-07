use crate::blockchain_env::*;
use crate::entities::traits::Proposer;
use crate::entities::{builder, proposer};
use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError, Uniform};
use std::collections::HashSet;

pub fn execute_simulation(
    num_blocks: u32,
    builder_vec: &mut Vec<builder::Builder>,
    proposer_vec: &mut Vec<proposer::Proposer>,
    transaction_set: HashSet<transaction::Transaction>,
) -> Vec<block::Block> {
    let mut blockchain: Vec<block::Block> = vec![];
    let mut rng = thread_rng();
    let uniform = Uniform::new(0, proposer_vec.len() as usize);
    for block_index in 1..=num_blocks {
        let block_proposer = uniform.sample(&mut rng);
        let mut proposed_block = proposer_vec[block_proposer].run_auction(builder_vec, 10);
        proposer_vec[block_proposer]
            .propose_block(&proposer_vec[block_proposer], &mut proposed_block);
    }
    blockchain
}
