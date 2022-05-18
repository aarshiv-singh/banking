use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub S_No: usize,
    pub transaction_type: String,
    pub transaction_amount: f64,
    pub balance: f64,
    pub timeStamp: String
}

pub fn build_transaction(S_No: usize, transaction_type: String, transaction_amount: f64, balance: f64, timeStamp: String) -> Transaction {
    Transaction { S_No, transaction_type, transaction_amount, balance, timeStamp }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub S_no: usize,
    pub user_name: String,
    pub Amount: f64
}

pub fn build_user(S_no: usize, user_name: String, Amount: f64) -> Data {
    Data {S_no, user_name, Amount}
}