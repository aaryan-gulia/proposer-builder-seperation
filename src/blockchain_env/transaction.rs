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

impl TransactionBuilder {
    pub fn build(self) -> Result<Transaction, String> {
        let gas_amount = self
            .gas_amount
            .ok_or(TransactionBuilderError::MissingGasAmount)?;
        let max_mev_amount = self
            .max_mev_amount
            .ok_or(TransactionBuilderError::MissingMaxMevAmount)?;
        let transaction_type = self
            .transaction_type
            .ok_or(TransactionBuilderError::MissingTransactionType)?;
        let id = self.id.ok_or(TransactionBuilderError::InvalidId)?;

        Ok(Transaction {
            id,
            gas_amount,
            max_mev_amount,
            transaction_type,
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
