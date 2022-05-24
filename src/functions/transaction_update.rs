// use crate::functions::*;

use chrono::{Datelike, Local, Timelike};
use std::{fs::File, path::Path};
// super refers to the scope just above this file.
// or you can also write
// use crate::functions::structures::{Transaction, Data, TransactionType};
use super::{
    csv_function::{data_reader, data_writer, transaction_reader, transaction_writer},
    structures::{Data, Transaction, TransactionType},
};

pub fn transaction_update(user: &str, transaction_type: TransactionType, amount: f32) {
    //No need for Box<dyn Error> , since you can have all the errors into the same datatype
    let data_path = get_path(None);
    let transaction_path = get_path(Some(user));
    // again, no need for manual panic, either use unwrap or expect
    // also a better variable name would be something like user_data or all_user_data
    let mut transaction_csv = transaction_reader(&transaction_path);

    // let mut user_found = false;
    // for mut record in &mut data_csv {
    //     if record.user_name.eq(user) {
    //         user_found = true;
    //         let initial_amount = record.amount;
    //         let final_amount = amount_update(transaction_type, amount, initial_amount);
    //         record.amount = final_amount;
    //         let timestamp = timestamp();
    //         let new_transaction = Transaction::new(
    //             transaction_csv.len() + 1,
    //             transaction_type.to_ascii_lowercase(),
    //             amount,
    //             final_amount,
    //             timestamp,
    //         );
    //         transaction_csv.push(new_transaction);
    //         break;
    //     }
    // }

    // since rust if supports functional programming, you can chain functions with a '.'
    // try to use iterators like so:
    let mut user_found = false;
    let data_csv = data_reader(&data_path);
    let data_csv = data_csv
        .into_iter()
        .map(|mut record| {
            if record.user_name.eq(user) {
                user_found = true;
                let initial_amount = record.amount;
                // sending transaction_type reference since it's also needed further down the code
                // and we cannot use it exhaustively
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
            record
        })
        .collect::<Vec<Data>>();

    match user_found {
        true => data_writer(&data_path, data_csv),
        false => {
            let mut data_csv = data_reader(&data_path);
            let initial_amount = 0.0;
            let final_amount = amount_update(&transaction_type, amount, initial_amount).unwrap();
            data_csv.push(Data::new(
                data_csv.len() + 1,
                user.to_string(),
                final_amount,
            ));

            transaction_csv.push(Transaction::new(
                1,
                transaction_type.to_string(),
                amount,
                final_amount,
                timestamp(),
            ));
            data_writer(&data_path, data_csv)
        }
    };
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
    // if transaction_type.eq_ignore_ascii_case("credit") {
    //     initial_amount + amount
    // } else if transaction_type.eq_ignore_ascii_case("debit") {
    //     if (initial_amount - amount) < 0.0 {
    //         panic!("Insufficient balance!! Aborting transaction")
    //     }
    //     initial_amount - amount
    // } else {
    //     panic!("Invalid transaction type")
    // }

    // again a faster and more idiomatic way would be to use match
    // if else are bad, in many languages

    match *transaction_type {
        TransactionType::Credit => Ok(initial_amount + amount),
        TransactionType::Debit => {
            if initial_amount < amount {
                return Err(String::from("Insufficient Balance"));
            }
            Ok(initial_amount - amount)
        } //since we already made it as enum we do not need to check for last else case
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
