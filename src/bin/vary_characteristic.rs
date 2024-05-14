use indicatif::ProgressBar;
use proposer_builder_seperation::blockchain_env;
use proposer_builder_seperation::blockchain_env::*;
use proposer_builder_seperation::entities::*;
use proposer_builder_seperation::setup::init::*;
use proposer_builder_seperation::simulation::*;
use proposer_builder_seperation::stop::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::thread::available_parallelism;

fn main() {
    vary_pos_characteristic();
}

fn vary_characteristic() {
    let NUM_BUILDERS: u32 = 50;
    let MEV_BUILDERS: u32 = 50;
    let NUM_TRANSACTIONS = 1000;
    let NUM_BLOCKS = 5000;

    let rand_num_vec: Vec<f64> = get_random_numbers::<f64>(100000000, 0.0, 1.0);
    let pb = ProgressBar::new((MEV_BUILDERS * 10) as u64);
    let num_builders_vec: Vec<u32> = (1..=MEV_BUILDERS).collect(); // Parallelize the outer loop iterating over num_builders
    num_builders_vec.into_par_iter().for_each(|mev_builders| {
        for characteristic in 1..=5 {
            let characteristic: f64 = characteristic as f64 / 5.0;
            let mut builder_vec = initiate_builders(NUM_BUILDERS - mev_builders, characteristic);
            builder_vec.append(&mut initiate_mev_builder(mev_builders, characteristic));
            let mut proposer_vec = initiate_proposers(5);

            let transaction_set: HashSet<transaction::Transaction> =
                initiate_transactions_default(NUM_TRANSACTIONS);

            let blockchain = execute_simulation(
                NUM_BLOCKS,
                &mut builder_vec,
                &mut proposer_vec,
                transaction_set.clone(), // Clone required for parallel execution
                &rand_num_vec,
            );
            let file_name = format!(
                "../data/pos_vary_mev_and_characteristic/mev_builders={}characteristic={}.csv",
                mev_builders, characteristic
            );

            save_continuous_simulation_to_csv(&blockchain, &file_name)
                .expect("save_blockchain_to_csv() failing from simple_pbs() test");

            pb.inc(1);
        }
    });
    pb.finish();
}

fn vary_pos_characteristic() {
    let NUM_BUILDERS: u32 = 50;
    let MEV_BUILDERS: u32 = 50;
    let NUM_TRANSACTIONS = 1000;
    let NUM_BLOCKS = 5000;

    let rand_num_vec: Vec<f64> = get_random_numbers::<f64>(100000000, 0.0, 1.0);
    let pb = ProgressBar::new((MEV_BUILDERS * 10) as u64);
    let num_builders_vec: Vec<u32> = (1..=MEV_BUILDERS).collect(); // Parallelize the outer loop iterating over num_builders
    num_builders_vec.into_par_iter().for_each(|mev_builders| {
        for characteristic in 1..=5 {
            let characteristic: f64 = characteristic as f64 / 5.0;
            let mut builder_vec = initiate_builders(NUM_BUILDERS - mev_builders, characteristic);
            builder_vec.append(&mut initiate_mev_builder(mev_builders, characteristic));

            let transaction_set: HashSet<transaction::Transaction> =
                initiate_transactions_default(NUM_TRANSACTIONS);

            let blockchain = execute_pos_simulation(
                NUM_BLOCKS,
                &mut builder_vec,
                transaction_set.clone(), // Clone required for parallel execution
            );
            let file_name = format!(
                "../data/pos_vary_mev_and_characteristic/mev_builders={}characteristic={}.csv",
                mev_builders, characteristic
            );

            save_pos_to_csv(&blockchain, &file_name)
                .expect("save_blockchain_to_csv() failing from simple_pbs() test");

            pb.inc(1);
        }
    });
    pb.finish();
}

fn vary_mev_builders() {}
