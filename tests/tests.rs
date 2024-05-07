use proposer_builder_seperation::blockchain_env::*;
use proposer_builder_seperation::entities::*;
use proposer_builder_seperation::setup::*;
#[test]
fn simple_pbs() {
    let builder_vec = setup::initiate_builders(100, 0.5);
    let proposer_vec = setup::initiate_proposers(5);

    assert_eq!(100, builder_vec.len());
    assert_eq!(5, proposer_vec.len());
}
