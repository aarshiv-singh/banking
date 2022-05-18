use crate::functions::*;

use std::{self,error::Error};
use chrono::{Local, Timelike, Datelike};

pub fn transaction_update(user: &str, transaction_type: &str, amount: f64) -> Result<(), Box<dyn Error>> {
    let data_path = PathVariables::get_main_path();
    let transaction_path = PathVariables::get_transaction_path(user);
    let mut data_csv = CsvFunction::data_reader(&data_path).unwrap_or_else(|error| {
        panic!("Error in getting main data from csv with error : {}",error);
    });
    let mut transaction_csv = CsvFunction::transaction_reader(&transaction_path).unwrap_or_else(|error| {
        panic!("Error in getting transaction data from csv with error: {}",error);
    });
    let mut user_found = false;

    for mut record in &mut data_csv {
        if record.user_name.eq(user) {
            user_found = true;
            let initial_amount = record.Amount;
            let final_amount = amount_update(transaction_type, amount, initial_amount);
            record.Amount = final_amount;
            let timestamp = timestamp();
            let new_transaction = Structures::build_transaction(transaction_csv.len()+1, transaction_type.to_ascii_lowercase(), amount, final_amount, timestamp);
            transaction_csv.push(new_transaction);
            break;
        }
    }
    if !user_found {
        let initial_amount = 0.0;
        let final_amount = amount_update(transaction_type, amount, initial_amount);
        let new_user = Structures::build_user(1, user.to_string(), final_amount);
        data_csv.push(new_user);
        let timestamp = timestamp();
        let new_transaction = Structures::build_transaction(1, transaction_type.to_ascii_lowercase(), amount, final_amount, timestamp);
        transaction_csv.push(new_transaction);
    }
    
    CsvFunction::data_writer(&data_path, data_csv).unwrap_or_else(|error| {
        panic!("Cannot write to the data file csv with error: {}", error);
    });

    CsvFunction::transaction_writer(&transaction_path, transaction_csv).unwrap_or_else(|error| {
        panic!("Cannot write to the transaction file csv with error: {}", error);
    });

    Ok(())
}

pub fn timestamp() -> String {
    let now = Local::now();
    // Time stamp in the format of yyyy-mm-dd hh:mm:ss
    let str = format!("{}-{}-{} {}:{}:{}",now.year(),now.month(),now.day(),now.hour(),now.minute(),now.second());
    str
}

pub fn amount_update(transaction_type: &str, amount: f64, initial_amount: f64) -> f64 {
    if transaction_type.eq_ignore_ascii_case("credit") {
        return initial_amount + amount;
    }
    else if transaction_type.eq_ignore_ascii_case("debit") {
        if (initial_amount - amount)<0.0 {
            panic!("Insufficient balance!! Aborting transaction")
        }
        return initial_amount - amount;
    }
    else {
        panic!("Invalid transaction type")
    }
}

