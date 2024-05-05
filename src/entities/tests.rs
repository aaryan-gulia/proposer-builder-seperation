use crate::blockchain_env::transaction;
use crate::entities::builder;
use crate::entities::proposer;
use crate::entities::traits;
#[test]
fn builder_mempool_function() {
    let mut b = builder::Builder::new();
    let mut transaction_vec: Vec<transaction::Transaction> = vec![];
    for _ in 0..10 {
        let t = transaction::TransactionBuilder::new();
        let t = t
            .gas_amount(100.0)
            .max_mev_amount(100.0)
            .transaction_type(transaction::TransactionType::Normal)
            .build()
            .unwrap();
        transaction_vec.push(t);
    }
    b.collect_transaction(&transaction_vec);
    assert_eq!(transaction_vec.len(), b.mempool.len());

    b.mempool = vec![];
    b.characteristic = 0.01;
    b.collect_transaction(&transaction_vec);
    assert!(transaction_vec.len() > b.mempool.len());
}
