use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::response::Response;

pub mod database;

pub async fn index_hand() -> (StatusCode, Json<Response<u8>>) {
    let mut res = Response::new(true, Value::Null, Some(1));
    res.message("Greetings");

    return (StatusCode::OK, res.as_json());
}