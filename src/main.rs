pub mod functions;
use clap::Parser;
use functions::structures::TransactionType;
use functions::transaction_update::transaction_update;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'u', long = "user")]
    user: String,
    #[clap(short = 't', long = "transaction", default_value = "credit")]
    transaction_type: String,
    #[clap(short = 'a', long = "amount")]
    amount: String,
}

fn main() {
    let args = Cli::parse();
    // no need to use f64 as it only increases the accuracy after the decimal
    // its faster to just use f32 when you know you dont need that much accurate decimal value
    let amount_in_float: f32 = args.amount.parse().expect("Invalid Amount");
    let transaction_type = args.transaction_type.parse::<TransactionType>().unwrap();
    //you dont need to manually panic the program.
    transaction_update(&args.user, transaction_type, amount_in_float);
    // instead of having a top level error message, you can put error message on the lower levels.
    println!("Transaction successful");
}
