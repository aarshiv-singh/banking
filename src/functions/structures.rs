use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub s_no: usize,
    pub transaction_type: String,
    pub transaction_amount: f32,
    pub balance: f32,
    pub time_stamp: String,
}

impl Transaction {
    pub fn new(
        s_no: usize,
        transaction_type: String,
        transaction_amount: f32,
        balance: f32,
        time_stamp: String,
    ) -> Self {
        //Self with a capital 'S' means the datatype of the structure from which this trait belongs to, without having to explicitly return Transaction
        Transaction {
            s_no,
            transaction_type,
            transaction_amount,
            balance,
            time_stamp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub s_no: usize,
    pub user_name: String,
    pub amount: f32,
}

impl Data {
    pub fn new(s_no: usize, user_name: String, amount: f32) -> Self {
        Data {
            s_no,
            user_name,
            amount,
        }
    }
}

// Since transaction_type has only two values, its better to make it an enum
// and you can implement from_str for an enum, so that you can convert string to enum variants
#[derive(Debug)]
pub enum TransactionType {
    Debit,
    Credit,
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "debit" => Ok(TransactionType::Debit),
            "credit" => Ok(TransactionType::Credit),
            _ => Err(format!("{} is invalid transaction type", input)),
        }
    }
}

// to be able to convert TransactionType enum to string, you can implement Display, only write! is changed other things are same everywhere
impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
