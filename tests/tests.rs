use proposer_builder_seperation::blockchain_env::*;
use proposer_builder_seperation::entities::*;
use proposer_builder_seperation::setup::init::*;
use proposer_builder_seperation::simulation::*;
use std::collections::HashSet;

#[test]
fn simple_pbs() {
    let num_builders: u32 = 100;
    let num_proposers: u32 = 5;
    let num_transactions: u32 = 100;
    let builder_characteristic: f64 = 0.5;
    let num_blocks: u32 = 500;

    let mut builder_vec = initiate_builders(num_builders, builder_characteristic);
    let mut proposer_vec = initiate_proposers(num_proposers);

    assert_eq!(100, builder_vec.len());
    assert_eq!(5, proposer_vec.len());

    let mut transaction_set: HashSet<transaction::Transaction> =
        initiate_transactions_default(num_transactions);

    let blockchain = execute_simulation(
        num_blocks,
        &mut builder_vec,
        &mut proposer_vec,
        transaction_set,
    );
}
