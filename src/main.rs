#![deny(elided_lifetimes_in_paths)]

extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::{Arc, Mutex};

use axum::{Extension, Server};
use axum::routing::get;

use routes::table::table_hand;

use crate::database::row::Row;
use crate::database::types::FieldValue;
use crate::logger::Logger;

pub mod response;
pub mod database;
pub mod storage;
pub mod routes;
pub mod logger;
pub mod ranid;


#[tokio::main]
async fn main() {
    let mut dbc = database::Database::new("master");

    let mut user = Row::new();
    user.data.insert("username".to_string(), FieldValue::String("Bob".to_string()));
    dbc.insert("users", user);

    let logger = Logger::new();
    let mut db = Arc::new(Mutex::new(dbc));

    let app = axum::Router::new()
        .route("/table/:table/all", get(table_hand))
        .layer(Extension(Arc::clone(&mut db)));

    println!("Server Started");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
