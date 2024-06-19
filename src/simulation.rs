use crate::blockchain_env::*;
use crate::entities::traits::Proposer;
use crate::entities::{builder, proposer};
use crate::setup::*;
use indicatif::ProgressBar;
use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError, Uniform};
use rayon::prelude::*;
use std::collections::HashSet;
use std::thread::available_parallelism;

pub fn execute_pos_simulation(
    num_blocks: u32,
    builder_vec: &mut Vec<builder::BuilderType>,
    mut transaction_set: HashSet<transaction::Transaction>,
) -> Vec<block::Block> {
    let mut blockchain: Vec<block::Block> = vec![];
    let num_transactions = transaction_set.len() as u32;
    let mut rng = thread_rng;
    let uniform = Uniform::new(0, builder_vec.len() as u32);

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

        let block_builder = uniform.sample(&mut rng());
        let b = &builder_vec[block_builder as usize];
        let mut curr_block = match b {
            builder::BuilderType::NormalBuilder(b) => b.build_block(100),
            builder::BuilderType::MevBuilder(b) => b.build_block(100),
        };

        transaction::Transaction::clean_transaction_set(
            &mut transaction_set,
            &curr_block.transactions,
        );
        curr_block.block_index = Some(block_index);
        maintain::refill_transactions_default(
            num_transactions,
            block_index + 1,
            &mut transaction_set,
        );
        blockchain.push(curr_block);
    }
    blockchain
}

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
            100,
            &blockchain,
            random_number_vec,
        );
        proposer_vec[block_proposer].propose_block(
            &proposer_vec[block_proposer],
            &mut proposed_block,
            100,
        );
        transaction::Transaction::clean_transaction_set(
            &mut transaction_set,
            &proposed_block.transactions,
        );
        proposed_block.block_index = Some(block_index);
        maintain::refill_transactions_default(
            num_transactions,
            proposed_block.block_index.unwrap() + 1,
            &mut transaction_set,
        );
        //println!("{}", &proposed_block.get_block_index().unwrap());
        blockchain.push(proposed_block);
    }
    blockchain
}

pub fn setup_and_execute_simulation(
    total_builders: u32,
    mev_builders: u32,
    num_transactions: u32,
    num_blocks: u32,
    block_size: u32,
    characteristic: f64,
    dir: String,
    rand_num_vec: &Vec<f64>,
) {
    let mut builder_vec = init::initiate_builders(total_builders - mev_builders, characteristic);
    builder_vec.append(&mut init::initiate_mev_builder(
        mev_builders,
        characteristic,
    ));
    let mut proposer_vec = init::initiate_proposers(5);

    let transaction_set: HashSet<transaction::Transaction> =
        init::initiate_transactions_default(num_transactions, 0);

    let blockchain = execute_simulation(
        num_blocks,
        &mut builder_vec,
        &mut proposer_vec,
        transaction_set.clone(), // Clone required for parallel execution
        &rand_num_vec,
    );

    let file_name = format!(
        "data/{}/total_builders={}
        mev_builders={}
        num_transactions={}
        num_blocks={}
        block_size={}
        characteristic={}.csv",
        dir, total_builders, mev_builders, num_transactions, num_blocks, block_size, characteristic
    );

    crate::stop::save_continuous_simulation_to_csv(&blockchain, &file_name)
        .expect("save_blockchain_to_csv() failing from simple_pbs() test");
}
