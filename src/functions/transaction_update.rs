use chrono::{Datelike, Local, Timelike};
use std::{fs::File, path::Path};
// super refers to the scope just above this file.
// or you can also write
// use crate::functions::structures::{Transaction, Data, TransactionType};
use super::{
    csv_function::{data_reader, transaction_reader, transaction_writer},
    structures::{Data, Transaction, TransactionType},
};

pub fn transaction_update(user: &str, transaction_type: TransactionType, amount: f32) {
    let data_path = get_path(None);
    let transaction_path = get_path(Some(user));
    let mut transaction_csv = transaction_reader(&transaction_path);
    let data_csv = data_reader(&data_path);

    let dlen = data_csv.len();
    let mut user_found = false;
    let mut writer = csv::Writer::from_path(&data_path).expect("Cannot open file");
    //writing data_csv to user_data.csv, if the user is found in data_csv, update the data and write it to csv
    //otherwise append the new data in the user_csv (line 42)
    data_csv
        .into_iter()
        .for_each(|mut record| {
            if record.user_name.eq(user) {
                user_found = true;
                let initial_amount = record.amount;
                let final_amount = amount_update(&transaction_type, amount, initial_amount);
                record.amount = final_amount.unwrap();
                transaction_csv.push(Transaction::new(
                    transaction_csv.len() + 1,
                    transaction_type.to_string(),
                    amount,
                    record.amount,
                    timestamp(),
                ));
            }
            writer.serialize(&record).expect("Cannot write to CSV");
    });
    
    if !user_found {
        let initial_amount = 0.0;
        let final_amount = amount_update(&transaction_type, amount, initial_amount).unwrap();
        writer.serialize(&Data::new(
            dlen + 1,
            user.to_string(),
            final_amount,
        )).expect("Cannot write to CSV");
        
        transaction_csv.push(Transaction::new(
            1,
            transaction_type.to_string(),
            amount,
            final_amount,
            timestamp(),
        ));
    }

    transaction_writer(&transaction_path, transaction_csv);
}

pub fn timestamp() -> String {
    let now = Local::now();
    // Time stamp in the format of yyyy-mm-dd hh:mm:ss
    let str = format!(
        "{}-{}-{} {}:{}:{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    str
}

pub fn amount_update(
    transaction_type: &TransactionType,
    amount: f32,
    initial_amount: f32,
) -> Result<f32, String> {
    // again a faster and more idiomatic way would be to use match
    // if else are bad, in many languages

    match *transaction_type {
        TransactionType::Credit => Ok(initial_amount + amount),
        TransactionType::Debit => {
            if initial_amount < amount {
                return Err(String::from("Insufficient Balance"));
            }
            Ok(initial_amount - amount)
        }
    }
}

// combining both path functions into one
pub fn get_path(file_name: Option<&str>) -> String {
    let path = match file_name {
        Some(user) => format!("./transactions/{}.csv", user),
        None => String::from("user_data.csv"),
    };
    match Path::new(&path).exists() {
        true => path,
        false => {
            File::create(&path).expect("Error in creating file");
            path
        }
    }
}
