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
    vary_num_builders();
}

fn vary_num_builders() {
    let NUM_BUILDERS: u32 = 100;
    let MEV_BUILDERS = 0;
    let NUM_TRANSACTIONS = 1000;
    let CHARACTERISTIC = 0.5;
    let NUM_BLOCKS = 5000;

    let rand_num_vec: Vec<f64> = get_random_numbers::<f64>(100000000, 0.0, 1.0);
    let pb = ProgressBar::new(NUM_BUILDERS as u64);
    let num_builders_vec: Vec<u32> = (1..=NUM_BUILDERS).collect(); // Parallelize the outer loop iterating over num_builders
    num_builders_vec.into_par_iter().for_each(|num_builders| {
        // Inner loop for mev_builders can remain sequential
        for mev_builders in 0..=num_builders {
            let mut builder_vec = initiate_builders(num_builders - mev_builders, CHARACTERISTIC);
            builder_vec.append(&mut initiate_mev_builder(mev_builders, CHARACTERISTIC));
            let mut proposer_vec = initiate_proposers(5);

            let transaction_set: HashSet<transaction::Transaction> =
                initiate_transactions_default(NUM_TRANSACTIONS, 0);

            let blockchain = execute_simulation(
                NUM_BLOCKS,
                &mut builder_vec,
                &mut proposer_vec,
                transaction_set.clone(), // Clone required for parallel execution
                &rand_num_vec,
            );
            let file_name = format!(
                "../data/vary_builder_and_mev/num_builders={}mev_builder={}.csv",
                num_builders, mev_builders
            );

            save_continuous_simulation_to_csv(&blockchain, &file_name)
                .expect("save_blockchain_to_csv() failing from simple_pbs() test");
        }
        pb.inc(1);
    });
    pb.finish();
}

fn vary_mev_builders() {}
