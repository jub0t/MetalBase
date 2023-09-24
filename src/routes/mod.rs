use axum::http::StatusCode;

use crate::database::types::FieldValue;
use crate::response::{Buffer, Response};

pub mod database;

pub async fn index_hand() -> (StatusCode, Buffer) {
    let mut res = Response::new(true, FieldValue::Null, Some(1));
    res.message("Greetings");

    return (StatusCode::OK, res.as_buffer());
}