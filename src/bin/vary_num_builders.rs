use proposer_builder_seperation::blockchain_env;
use proposer_builder_seperation::blockchain_env::*;
use proposer_builder_seperation::entities::*;
use proposer_builder_seperation::setup::init::*;
use proposer_builder_seperation::simulation::*;
use proposer_builder_seperation::stop::*;
use std::collections::HashSet;

fn main() {
    vary_num_builders();
}

fn vary_num_builders() {
    let NUM_BUILDERS = 100;
    let MEV_BUILDERS = 0;
    let NUM_TRANSACTIONS = 1000;
    let CHARACTERISTIC = 0.5;
    let NUM_BLOCKS = 5000;

    let rand_num_vec: Vec<f64> = get_random_numbers::<f64>(100000000, 0.0, 1.0);

    for num_builders in 1..=NUM_BUILDERS {
        for mev_builders in 0..=num_builders {
            println!(
                "Completeing => num_builders: {}, mev_builders: {}",
                num_builders, mev_builders
            );
            let mut builder_vec = initiate_builders(num_builders - mev_builders, CHARACTERISTIC);
            builder_vec.append(&mut initiate_mev_builder(mev_builders, CHARACTERISTIC));
            let mut proposer_vec = initiate_proposers(5);

            let transaction_set: HashSet<transaction::Transaction> =
                initiate_transactions_default(NUM_TRANSACTIONS);

            let blockchain = execute_simulation(
                NUM_BLOCKS,
                &mut builder_vec,
                &mut proposer_vec,
                transaction_set,
                &rand_num_vec,
            );
        }
    }
}

fn vary_mev_builders() {}
