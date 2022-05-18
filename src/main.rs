pub mod functions;
use functions::TransactionUpdate;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'u',long = "user")]
    user: String,
    #[clap(short = 't',long = "transaction",default_value="credit")]
    transaction_type: String,
    #[clap(short = 'a',long = "amount")]
    amount: String
}

fn main() {
    let args = Cli::parse();
    let amount_in_float: f64 = args.amount.parse().expect("Invalid Amount");
    TransactionUpdate::transaction_update(&args.user,&args.transaction_type,amount_in_float).unwrap_or_else(|error| {
            panic!("Error in transaction {}",error);
    });
    println!("Transaction successful");
}