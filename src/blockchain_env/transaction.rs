use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Transaction {
    id: u32,
    pub gas_amount: i64,
    pub max_mev_amount: i64,
    pub transaction_type: TransactionType,
    pub block_created: u32,
}

impl Transaction {
    pub fn get_transaction_id(&self) -> Option<u32> {
        if self.id >= 0 {
            Some(self.id)
        } else {
            None
        }
    }
    pub fn attack_transaction(&mut self) -> &mut Self {
        self.transaction_type = TransactionType::Attacked;
        self
    }
    pub fn compare_transaction_by_gas(a: &Transaction, b: &Transaction) -> std::cmp::Ordering {
        if a.gas_amount < b.gas_amount {
            return std::cmp::Ordering::Greater;
        }
        if a.gas_amount == b.gas_amount {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Less;
        }
    }

    pub fn compare_transaction_by_mev(a: &Transaction, b: &Transaction) -> std::cmp::Ordering {
        if a.gas_amount < b.max_mev_amount {
            return std::cmp::Ordering::Greater;
        }
        if a.gas_amount == b.max_mev_amount {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Less;
        }
    }

    pub fn compare_transaction_by_total(a: &Transaction, b: &Transaction) -> std::cmp::Ordering {
        if a.gas_amount + a.max_mev_amount < b.gas_amount + b.max_mev_amount {
            return std::cmp::Ordering::Greater;
        }
        if a.gas_amount + a.max_mev_amount == b.gas_amount + b.max_mev_amount {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Less;
        }
    }
    pub fn clean_transaction_set(first: &mut HashSet<Transaction>, second: &HashSet<Transaction>) {
        first.retain(|t| !second.contains(t));
    }
}

impl Default for Transaction {
    fn default() -> Self {
        TransactionBuilder::new()
            .block_created(0)
            .gas_amount(0)
            .max_mev_amount(0)
            .transaction_type(TransactionType::Empty)
            .build()
            .expect("DEFAULT TRANSACTION BUILD FAILING")
    }
}
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionType {
    Normal,
    Attack,
    Attacked,
    Empty,
}

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Normal => "normal".to_string(),
            TransactionType::Attack => "attack".to_string(),
            TransactionType::Attacked => "attacked".to_string(),
            TransactionType::Empty => "empty".to_string(),
        }
    }
}

pub struct TransactionBuilder {
    id: Option<u32>,
    gas_amount: Option<i64>,
    max_mev_amount: Option<i64>,
    transaction_type: Option<TransactionType>,
    block_created: Option<u32>,
}

impl TransactionBuilder {
    pub fn reset() {
        unsafe {
            NORMAL_TRANSACTION_COUNTER = 0;
            ATTACK_TRANSACTION_COUNTER = 0;
        }
    }
    pub fn new() -> Self {
        TransactionBuilder {
            id: None,
            gas_amount: None,
            max_mev_amount: None,
            transaction_type: None,
            block_created: None,
        }
    }
}

static mut NORMAL_TRANSACTION_COUNTER: u32 = 0;
static mut ATTACK_TRANSACTION_COUNTER: u32 = 0;

impl TransactionBuilder {
    pub fn gas_amount(mut self, gas_amount: i64) -> Self {
        self.gas_amount = Some(gas_amount);
        self
    }

    pub fn max_mev_amount(mut self, max_mev_amount: i64) -> Self {
        self.max_mev_amount = Some(max_mev_amount);
        self
    }

    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    pub fn block_created(mut self, block_created: u32) -> Self {
        self.block_created = Some(block_created);
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
        let block_created = self
            .block_created
            .ok_or(TransactionBuilderError::MissingBlockCreatedNumber)?;
        let id = match transaction_type {
            TransactionType::Normal => unsafe {
                NORMAL_TRANSACTION_COUNTER += 1;
                NORMAL_TRANSACTION_COUNTER
            },
            TransactionType::Attack => unsafe {
                ATTACK_TRANSACTION_COUNTER += 1;
                ATTACK_TRANSACTION_COUNTER
            },
            TransactionType::Empty => unsafe {
                NORMAL_TRANSACTION_COUNTER += 1;
                NORMAL_TRANSACTION_COUNTER
            },
            TransactionType::Attacked => unsafe {
                NORMAL_TRANSACTION_COUNTER += 1;
                NORMAL_TRANSACTION_COUNTER
            },
        };

        Ok(Transaction {
            id,
            gas_amount,
            max_mev_amount,
            transaction_type,
            block_created,
        })
    }
}

#[derive(Debug)]
pub enum TransactionBuilderError {
    MissingGasAmount,
    MissingMaxMevAmount,
    MissingTransactionType,
    InvalidId,
    MissingBlockCreatedNumber,
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
                TransactionBuilderError::MissingBlockCreatedNumber => {
                    "missing block created number"
                }
            }
        }
    }
}

pub mod serialize_as_string {
    use super::Transaction;
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(d: &[Transaction], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut v = Vec::new();
        let mut w = csv::Writer::from_writer(&mut v);
        for record in d {
            w.serialize(record).map_err(serde::ser::Error::custom)?;
        }
        drop(w);
        let s = String::from_utf8(v).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }
}
