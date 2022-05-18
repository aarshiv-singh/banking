use crate::functions::structures::{Data, Transaction};
use csv;
use std::{self, error::Error};

pub fn data_reader(path: &str) -> Result<Vec<Data>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut vector = vec![];

    for result in reader.deserialize() {
        let record: Data = result?;
        vector.push(record);
    }
    Ok(vector)
}

pub fn data_writer(path: &str, vector: Vec<Data>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;

    for record in vector {
        writer.serialize(&record)?;
    }

    Ok(())
}

pub fn transaction_reader(path: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut vector = vec![];

    for result in reader.deserialize() {
        let record: Transaction = result?;
        vector.push(record)
    }
    Ok(vector)
}

pub fn transaction_writer(path: &str, vector: Vec<Transaction>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;

    for record in vector {
        writer.serialize(&record)?;
    }

    Ok(())
}
