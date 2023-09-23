#![deny(elided_lifetimes_in_paths)]

extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::Arc;

use axum::{Extension, Server};
use axum::routing::get;

use routes::database::database_hand;
use routes::index_hand;

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
    let logger = Logger::new();
    let mut db = Arc::new(database::Database::new("master"));

    let app = axum::Router::new()
        .route("/", get(index_hand))
        .route("/database/:database/:table", get(database_hand))
        .layer(Extension(Arc::clone(&mut db)));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
