#[derive(Debug)]
pub struct Transaction {
    id: u32,
    pub gas_amount: f64,
    pub max_mev_amount: f64,
    pub transaction_type: TransactionType,
    pub node_signature: Option<String>,
}

impl Transaction {
    pub fn get_transaction_id(&self) -> Option<u32> {
        if self.id > 0 {
            Some(self.id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum TransactionType {
    Normal,
    Attack,
}

pub struct TransactionBuilder {
    id: Option<u32>,
    gas_amount: Option<f64>,
    max_mev_amount: Option<f64>,
    transaction_type: Option<TransactionType>,
    node_signature: Option<String>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        TransactionBuilder {
            id: None,
            gas_amount: None,
            max_mev_amount: None,
            transaction_type: None,
            node_signature: None,
        }
    }
}

static mut NORMAL_TRANSACTION_COUNTER: u32 = 0;
static mut ATTACK_TRANSACTION_COUNTER: u32 = 0;

impl TransactionBuilder {
    pub fn gas_amount(mut self, gas_amount: f64) -> Self {
        self.gas_amount = Some(gas_amount);
        self
    }

    pub fn max_mev_amount(mut self, max_mev_amount: f64) -> Self {
        self.max_mev_amount = Some(max_mev_amount);
        self
    }

    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    pub fn node_signature(mut self, node_signature: String) -> Self {
        self.node_signature = Some(node_signature);
        self
    }

    pub fn build(self) -> Result<Transaction, TransactionBuilderError> {
        let gas_amount = self
            .gas_amount
            .ok_or(TransactionBuilderError::MissingGasAmount)?;
        let max_mev_amount = self
            .max_mev_amount
            .ok_or(TransactionBuilderError::MissingMaxMevAmount)?;
        let transaction_type = self
            .transaction_type
            .ok_or(TransactionBuilderError::MissingTransactionType)?;
        let node_signature = self.node_signature;

        let id = match transaction_type {
            TransactionType::Normal => unsafe {
                NORMAL_TRANSACTION_COUNTER += 1;
                NORMAL_TRANSACTION_COUNTER
            },
            TransactionType::Attack => unsafe {
                ATTACK_TRANSACTION_COUNTER += 1;
                ATTACK_TRANSACTION_COUNTER
            },
        };

        Ok(Transaction {
            id,
            gas_amount,
            max_mev_amount,
            transaction_type,
            node_signature,
        })
    }
}

#[derive(Debug)]
pub enum TransactionBuilderError {
    MissingGasAmount,
    MissingMaxMevAmount,
    MissingTransactionType,
    InvalidId,
}

impl std::fmt::Display for TransactionBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TransactionBuilderError: {:?}", self)
    }
}

impl std::error::Error for TransactionBuilderError {}
impl From<TransactionBuilderError> for String {
    fn from(error: TransactionBuilderError) -> Self {
        "Transaction error:".to_string() + {
            match error {
                TransactionBuilderError::MissingGasAmount => "missing gas amount",
                TransactionBuilderError::MissingMaxMevAmount => "missing max mev amount",
                TransactionBuilderError::MissingTransactionType => "missing transaction type",
                TransactionBuilderError::InvalidId => "invalid id",
            }
        }
    }
}
