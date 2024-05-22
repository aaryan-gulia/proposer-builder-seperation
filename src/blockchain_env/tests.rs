use crate::blockchain_env::block;
use crate::blockchain_env::transaction;

#[test]
fn new_error_transaction() {
    let t = transaction::TransactionBuilder::new();
    let t = t.build();
    assert!(!t.is_ok());
    println!("{:#?}", t);
}

#[test]
fn new_correct_transaction() {
    transaction::TransactionBuilder::reset();
    let t = transaction::TransactionBuilder::new();
    let t = t
        .gas_amount(100)
        .max_mev_amount(100)
        .transaction_type(transaction::TransactionType::Normal)
        .build()
        .unwrap();
    assert_eq!(t.get_transaction_id().unwrap(), 1);
}

// #[test]
// fn new_block() {
//     let mut b1 = block::Block::new(1, 1.0, 1.0, 1.0, vec![].into_iter().collect());
//     b1.add_to_chain(3);
//     assert_eq!(b1.builder_id, 1);
//     assert_eq!(b1.get_block_index().unwrap(), 1);
//     let mut b2 = block::Block::new(1, 1.0, 1.0, 1.0, vec![].into_iter().collect());
//     assert_eq!(b2.get_block_index(), None);
//     b2.add_to_chain(3);
//     assert_eq!(b2.get_block_index().unwrap(), 2);
// }
