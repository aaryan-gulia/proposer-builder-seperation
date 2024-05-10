use proposer_builder_seperation::blockchain_env::*;
use proposer_builder_seperation::entities::*;
use proposer_builder_seperation::setup::init::*;
use proposer_builder_seperation::simulation::*;
use proposer_builder_seperation::stop::*;
use std::collections::HashSet;
use std::panic::AssertUnwindSafe;

fn main() {
    simple_pbs();
}

fn simple_pbs() {
    let num_builders: u32 = 100;
    let num_proposers: u32 = 5;
    let num_transactions: u32 = 100;
    let builder_characteristic: f64 = 0.5;
    let num_blocks: u32 = 100;

    let mut builder_vec = initiate_builders(num_builders, builder_characteristic);
    let mut proposer_vec = initiate_proposers(num_proposers);
    let rand_num_vec: Vec<f64> = get_random_numbers::<f64>(100000, 0.0, 1.0);

    println!("{:?}", rand_num_vec);

    assert_eq!(100, builder_vec.len());
    assert_eq!(5, proposer_vec.len());

    let mut transaction_set: HashSet<transaction::Transaction> =
        initiate_transactions_default(num_transactions);
    let mut transaction_gas: i64 = 0;
    for t in &transaction_set {
        transaction_gas += t.gas_amount;
    }
    println!("{}", transaction_gas);
    assert_ne!(transaction_gas, 0);

    let blockchain = execute_simulation(
        num_blocks,
        &mut builder_vec,
        &mut proposer_vec,
        transaction_set,
        &rand_num_vec,
    );
    assert_ne!(blockchain.len(), 0);
    save_blockchain_to_csv(&blockchain, "simple_pbs_test.csv")
        .expect("save_blockchain_to_csv() failing from simple_pbs() test");
}
