pub mod functions;
use std::collections::HashMap;

use actix_web::http::StatusCode;
use functions::csv_function::{data_reader, transaction_reader};
use functions::structures::TransactionType;
use functions::transaction_update::{self, get_path};

//Remove comments for CLI implementation
// use clap::Parser;

// #[derive(Parser, Debug)]
// struct Cli {
//     #[clap(short = 'u', long = "user")]
//     user: String,
//     #[clap(short = 't', long = "transaction", default_value = "credit")]
//     transaction_type: String,
//     #[clap(short = 'a', long = "amount")]
//     amount: String,
// }

// fn main() {
//     let args = Cli::parse();
//     let amount_in_float: f32 = args.amount.parse().expect("Invalid Amount");
//     let transaction_type = args.transaction_type.parse::<TransactionType>().unwrap();
//     transaction_update(&args.user, transaction_type, amount_in_float);
//     println!("Transaction successful");
// }

//Using actix web:-

use actix_web::{
    http::header::ContentType, web, App, HttpResponse, HttpServer,
};

async fn view_all_user() -> HttpResponse {
    let str_data = serde_json::to_string_pretty(&data_reader(&get_path(None))).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(str_data)
}

async fn view_user(info: web::Query<HashMap<String, String>>) -> HttpResponse {
    let data = data_reader(&get_path(None));
    let mut user_name: Option<String> = Option::None;
    data
        .into_iter()
        .for_each(|record| {
            if record.user_name.eq_ignore_ascii_case(&info.get("user_name").unwrap()) {
                user_name = Some(String::from(&record.user_name))
            }
        });
    match user_name {
        Some(user_name) => {
            let transaction = transaction_reader(&get_path(Some(&user_name)));
            let str_transaction = serde_json::to_string_pretty(&transaction).unwrap();
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(str_transaction)
        }
        None => return HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

async fn transaction_update(query: web::Query<HashMap<String, String>>, path: web::Path<String>) -> HttpResponse {
    let transaction_type = path.into_inner().parse::<TransactionType>().unwrap();
    let amount: f32 = query.get("amount").unwrap().parse().unwrap();
    transaction_update::transaction_update(&query.get("user_name").unwrap(), transaction_type, amount);
    HttpResponse::Ok()
        .body(String::from("Transaction Successfull"))
}

async fn create_user(query: web::Query<HashMap<String, String>>) -> HttpResponse {
    let amount: f32 = query.get("amount").unwrap().parse().unwrap();
    transaction_update::transaction_update(&query.get("user_name").unwrap(),TransactionType::Credit,amount);
    HttpResponse::Ok()
        .body(String::from("User Added"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/user")
                .route("/viewAllUsers", web::get().to(view_all_user))
                .route("/viewUser", web::get().to(view_user))
                .route("/createUser", web::post().to(create_user)),
        )
        .service(
            web::resource("/transaction/{t_type}")
                .route(web::post().to(transaction_update))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
