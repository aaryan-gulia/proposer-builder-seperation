use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Transaction {
    id: u32,
    pub gas_amount: i64,
    pub max_mev_amount: i64,
    pub transaction_type: TransactionType,
}

impl Transaction {
    pub fn get_transaction_id(&self) -> Option<u32> {
        if self.id > 0 {
            Some(self.id)
        } else {
            None
        }
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

    pub fn clean_transaction_set(first: &mut HashSet<Transaction>, second: &HashSet<Transaction>) {
        first.retain(|t| !second.contains(t));
    }
}
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionType {
    Normal,
    Attack,
}

pub struct TransactionBuilder {
    id: Option<u32>,
    gas_amount: Option<i64>,
    max_mev_amount: Option<i64>,
    transaction_type: Option<TransactionType>,
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
