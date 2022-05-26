use crate::functions::structures::{Data, Transaction};
use csv;

pub fn data_reader(path: &str) -> Vec<Data> {
    let mut reader = csv::Reader::from_path(path).expect("cannot read file");
    reader
        .deserialize()
        .into_iter()
        .map(|result| {
            let record: Data = result.expect("Could not deserialize the transaction");
            record
        })
        .collect::<Vec<Data>>()
}

//no need of data_writer as we are writing the data at time of checking

// pub fn data_writer(path: &str, vector: Vec<Data>) {
//     // for record in vector {
//     //     writer.serialize(&record)?;
//     // }
//     // Ok(())
//     let mut writer = csv::Writer::from_path(path).expect("Cannot open file");
//     vector
//         .into_iter()
//         //using for each instead of map since iterators are lazy and we do not need to return anything
//         .for_each(|record| writer.serialize(&record).expect("Cannot write to CSV"));
// }

pub fn transaction_reader(path: &str) -> Vec<Transaction> {
    let mut reader = csv::Reader::from_path(path).expect("cannot read file");
    // let mut vector = vec![];
    // for result in reader.deserialize() {
    //     let record: Transaction = result?;
    //     vector.push(record)
    // }
    // Ok(vector)

    // since the user_data.csv is the main storage, you want to end the program
    // if there is any problem with it. Also try to user iterators for for loops
    // as it is idiomatic as well as performant

    reader
        .deserialize()
        .into_iter()
        .map(|result| {
            let record: Transaction = result.expect("Could not deserialize the transaction");
            record
        })
        .collect::<Vec<Transaction>>()
}

pub fn transaction_writer(path: &str, vector: Vec<Transaction>) {
    let mut writer = csv::Writer::from_path(path).expect("Cannot open file");

    vector
        .into_iter()
        .for_each(|record| writer.serialize(&record).expect("Cannot write to CSV"));
}
