use axum::routing::get;
use axum::Server;

mod toml;
mod routes;
mod response;
mod database;
mod storage;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(routes::index_hand));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
