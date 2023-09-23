use crate::response::Response;
use axum::http::StatusCode;
use serde_json::Value;
use axum::Json;

pub mod database;

pub async fn index_hand() -> (StatusCode, Json<Response>) {
    let mut res = Response::new();
    res.success(true);
    res.message("Greetings");
    res.data_field("username", Value::String("John".to_string()));

    return (StatusCode::OK, res.json_response());
}