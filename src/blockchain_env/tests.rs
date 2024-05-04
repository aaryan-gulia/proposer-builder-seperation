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
    let t = transaction::TransactionBuilder::new();
    let t = t
        .gas_amount(100.0)
        .max_mev_amount(100.0)
        .transaction_type(transaction::TransactionType::Normal)
        .build()
        .unwrap();
    assert_eq!(t.get_transaction_id().unwrap(), 1);
}
