pub enum Transaction {
    Normal {
        id: u32,
        gas_amount: f64,
        max_mev_amount: f64,
    },
    Attack {
        id: u32,
        gas_amount: f64,
        max_mev_amount: f64,
    },
}
