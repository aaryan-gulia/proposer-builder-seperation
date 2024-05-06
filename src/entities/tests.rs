use crate::blockchain_env::transaction;
use crate::entities::builder;
use crate::entities::proposer;
use crate::entities::traits::Proposer;
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

#[test]
fn builder_build_block() {
    let mut b = builder::Builder::new(1, 1.0);
    let mut transaction_set: HashSet<transaction::Transaction> = vec![].into_iter().collect();
    for i in 0..10 {
        let t = transaction::TransactionBuilder::new();
        let t = t
            .gas_amount(i * 10)
            .max_mev_amount(i * 10)
            .transaction_type(transaction::TransactionType::Normal)
            .build()
            .unwrap();
        transaction_set.insert(t);
    }
    b.collect_transaction(&transaction_set);
    let block_size: u32 = 5;
    let b_block = b.build_block(block_size);
    assert_eq!(350, b_block.gas_captured as i32);
    assert_eq!(0, b_block.mev_captured as i32);
}

#[test]
fn proposer_basic_auction() {
    let mut builders_vec: Vec<builder::Builder> = vec![];
    let mut transactions_set: HashSet<transaction::Transaction> = vec![].into_iter().collect();
    for i in 0..10 {
        let t = transaction::TransactionBuilder::new();
        let t = t
            .gas_amount(i * 10)
            .max_mev_amount(i * 10)
            .transaction_type(transaction::TransactionType::Normal)
            .build()
            .unwrap();
        transactions_set.insert(t);
    }
    for id in 1..=1 {
        builders_vec.push(builder::Builder::new(id, 0.5));
        builders_vec[(id - 1) as usize].collect_transaction(&transactions_set);
    }
    let p = proposer::Proposer::new(6);
    let mut proposed_block = p.run_auction(&mut builders_vec, 5);
    p.propose_block(&p, &mut proposed_block)
}
