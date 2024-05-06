use crate::blockchain_env::transaction;
use crate::entities::builder;
use crate::entities::proposer;
use crate::entities::traits;
use std::collections::HashSet;

#[test]
fn builder_mempool_function() {
    let mut b = builder::Builder::new(1, 1.0);
    let mut transaction_vec: HashSet<transaction::Transaction> = vec![].into_iter().collect();
    for _ in 0..10 {
        let t = transaction::TransactionBuilder::new();
        let t = t
            .gas_amount(100)
            .max_mev_amount(100)
            .transaction_type(transaction::TransactionType::Normal)
            .build()
            .unwrap();
        transaction_vec.insert(t);
    }
    b.collect_transaction(&transaction_vec);
    assert_eq!(transaction_vec.len(), b.mempool.len());

    b.mempool = vec![].into_iter().collect();
    b.characteristic = 0.01;
    b.collect_transaction(&transaction_vec);
    assert!(transaction_vec.len() > b.mempool.len());
}
