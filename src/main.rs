#![deny(elided_lifetimes_in_paths)]

extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::{Arc, Mutex};

use axum::routing::get;
use axum::Server;

use routes::table::table_hand;

use crate::database::Database;
use crate::logger::Logger;
use crate::storage::StorageMan;

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

    // dbc.create_table("users");
    //
    // let mut user = Row::new();
    // user.data.insert("username".to_string(), Value::new("James"));
    // dbc.insert("users", user);
    //
    // for x in 0..100000 {
    //     let mut user = Row::new();
    //     user.data.insert("username".to_string(), Value::new("Bob"));
    //     dbc.insert("users", user);
    // }

    let mut db = Arc::new(Mutex::new(dbc));
    let app = axum::Router::new()
        .route("/table/:table/all", get(table_hand))
        .with_state(db);

    println!("Server Listening at http://127.0.0.1:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
