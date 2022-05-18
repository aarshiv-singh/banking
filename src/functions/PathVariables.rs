use std::{fs::File, path::Path};

pub fn get_main_path() -> String {
    let path = String::from("user_data.csv");
    if Path::new(&path).exists() {
        return path;
    } else {
        File::create(&path).expect("Error in creating main data file");
        return path;
    }
}

pub fn get_transaction_path(user_name: &str) -> String {
    let path = format!("./transactions/{}.csv", user_name);
    if Path::new(&path).exists() {
        return path;
    } else {
        File::create(&path).expect("Error in creating transactions file");
        return path;
    }
}
