#![deny(elided_lifetimes_in_paths)]

extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::{Arc, Mutex};

use axum::routing::get;
use axum::Server;
use rayon::prelude::*;

use crate::database::Database;
use crate::database::types::Value;
use crate::logger::Logger;
use crate::routes::table::table_hand;
use crate::storage::StorageMan;
use crate::time::Time;

pub mod response;
pub mod database;
pub mod storage;
pub mod routes;
pub mod logger;
pub mod rid;
pub mod time;

#[tokio::main]
async fn main() {
    let logger = Logger::new();
    let sman = StorageMan::new();
    let mut dbc = Database::new("master");
    dbc.init_test();

    let start = Time::new();
    match dbc.find_all("users", "name", Value::String("Bob".to_string())).await {
        Ok(rows) => {
            println!("Found In {}: {:#?}", start.elapsed_fmt(), rows);
        }
        Err(err) => {
            println!("Error: {:#?}", err);
        }
    }

    let mut db = Arc::new(Mutex::new(dbc));
    let app = axum::Router::new()
        .route("/table/:table/all", get(table_hand))
        .with_state(db);

    println!("Server Listening at http://127.0.0.1:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
