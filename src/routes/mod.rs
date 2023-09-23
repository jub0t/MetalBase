use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

use crate::response::Response;

pub async fn index_hand() -> (StatusCode, Json<Response>) {
    let mut res = Response::new();
    res.success(true);
    res.message("Greetings");
    res.data_field("username", Value::String("John".to_string()));

    return (StatusCode::OK, res.json_response());
}