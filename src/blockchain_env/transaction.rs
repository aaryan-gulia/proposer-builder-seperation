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
