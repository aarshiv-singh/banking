pub mod functions;
use functions::TransactionUpdate;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    user: String,
    transaction_type: String,
    amount_as_str: String
}

fn main() {
    let args = Cli::parse();
    let amount: f64 = args.amount_as_str.parse().expect("Invalid Amount");
    TransactionUpdate::transaction_update(&args.user,&args.transaction_type,amount).unwrap_or_else(|error| {
            panic!("Error in transaction {}",error);
    });
    println!("Transaction successful");
}