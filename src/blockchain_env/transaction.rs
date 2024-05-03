pub struct Transaction {
    id: u32,
    pub gas_amount: f64,
    pub max_mev_amount: f64,
    pub transaction_type: TransactionType,
}

pub enum TransactionType {
    Normal,
    Attack,
}

pub struct TransactionBuilder {
    id: Option<u32>,
    gas_amount: Option<f64>,
    max_mev_amount: Option<f64>,
    transaction_type: Option<TransactionType>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        TransactionBuilder {
            id: None,
            gas_amount: None,
            max_mev_amount: None,
            transaction_type: None,
        }
    }
}
