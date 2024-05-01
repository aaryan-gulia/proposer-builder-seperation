use crate::blockchain_env::transaction::Transaction;
#[test]
fn new_transaction() {
    let t = Transaction::Normal {
        id: 1,
        gas_amount: 1.0,
        max_mev_amount: 1.0,
    };
    assert!(match t {
        Transaction::Normal {
            id,
            gas_amount,
            max_mev_amount,
        } => {
            true
        }
        Transaction::Attack {
            id,
            gas_amount,
            max_mev_amount,
        } => {
            false
        }
    });
}
