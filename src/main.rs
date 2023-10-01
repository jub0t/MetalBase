#![deny(elided_lifetimes_in_paths)]

extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::{Arc, Mutex};

use axum::routing::get;
use axum::Server;
use rayon::prelude::*;

use routes::table::table_hand;

use crate::database::Database;
use crate::database::row::Row;
use crate::database::types::Value;
use crate::logger::Logger;
use crate::storage::StorageMan;
use crate::time::Time;

pub mod response;
pub mod database;
pub mod storage;
pub mod routes;
pub mod logger;
pub mod rid;
pub mod time;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let logger = Logger::new();
    let sman = StorageMan::new();
    let mut dbc = Database::new("master");

    dbc.create_table("users".to_string());

    let now = Time::new();
    for x in 0..500_000 {
        let mut row = Row::new();
        row.insert("name".to_string(), Value::String("James".to_string()));
        dbc.insert("users", row).unwrap();
    }

    println!("{}", now.elapsed_fmt());

    let mut row = Row::new();
    row.insert("name".to_string(), Value::String("Bob".to_string()));
    dbc.insert("users", row).unwrap();

    let mut db = Arc::new(Mutex::new(dbc));
    let app = axum::Router::new()
        .route("/table/:table/all", get(table_hand))
        .with_state(db);

    println!("Server Listening at http://127.0.0.1:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
