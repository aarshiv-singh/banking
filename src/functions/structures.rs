use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub s_no: usize,
    pub transaction_type: String,
    pub transaction_amount: f64,
    pub balance: f64,
    pub time_stamp: String,
}

pub fn build_transaction(
    s_no: usize,
    transaction_type: String,
    transaction_amount: f64,
    balance: f64,
    time_stamp: String,
) -> Transaction {
    Transaction {
        s_no,
        transaction_type,
        transaction_amount,
        balance,
        time_stamp,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub s_no: usize,
    pub user_name: String,
    pub amount: f64,
}

pub fn build_user(s_no: usize, user_name: String, amount: f64) -> Data {
    Data {
        s_no,
        user_name,
        amount,
    }
}
