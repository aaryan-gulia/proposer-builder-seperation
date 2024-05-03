use crate::blockchain_env::transaction;
#[test]
fn new_transaction() {
    let t = transaction::TransactionBuilder::new();
    let t = t.build();
}
