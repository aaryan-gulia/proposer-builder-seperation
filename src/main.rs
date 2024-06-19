use indicatif::ProgressBar;
use proposer_builder_seperation::blockchain_env;
use proposer_builder_seperation::blockchain_env::*;
use proposer_builder_seperation::entities::*;
use proposer_builder_seperation::setup::init::*;
use proposer_builder_seperation::simulation::*;
use proposer_builder_seperation::stop::*;
use rayon::prelude::*;

fn main() {
    let num_transactions = vec![60, 80, 100, 120, 140, 160];
    let characteristics = vec![0.2, 0.6, 1.0];
    let mev_builders = vec![1, 25, 49];
    let rand_num_vec: Vec<f64> = get_random_numbers::<f64>(1000_000_00, 0.0, 1.0);
    println!("Starting Simulations");
    let pb = ProgressBar::new(
        (num_transactions.len() * characteristics.len() * mev_builders.len()) as u64,
    );

    for num_transaction in num_transactions.iter() {
        for characteristic in characteristics.iter() {
            for mev_builder in mev_builders.iter() {
                setup_and_execute_simulation(
                    50,
                    *mev_builder,
                    *num_transaction,
                    1000,
                    100,
                    *characteristic,
                    format!("tammy_asked_for_these"),
                    &rand_num_vec,
                );
                pb.inc(1);
            }
        }
    }
    pb.finish();
}
