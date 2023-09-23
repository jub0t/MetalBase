use axum::routing::get;
use axum::Server;

mod toml;
mod response;
mod database;
mod storage;
mod routes;
use routes::index_hand;
use routes::database::database_hand;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(index_hand))
        .route("/database", get(database_hand));


    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
