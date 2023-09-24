#![deny(elided_lifetimes_in_paths)]

extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::{Arc, Mutex};

use axum::{Extension, Server};
use axum::routing::get;

use routes::table::table_hand;

use crate::logger::Logger;

mod toml;
mod response;
mod database;
mod storage;
mod routes;
mod tests;
mod logger;


#[tokio::main]
async fn main() {
    let mut dbc = database::Database::new("master");
    dbc.create_table("users").unwrap();

    let logger = Logger::new();
    let mut db = Arc::new(Mutex::new(dbc));

    let app = axum::Router::new()
        .route("/database/:database/:table", get(table_hand))
        .layer(Extension(Arc::clone(&mut db)));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
