use std::sync::Arc;

use axum::Extension;
use axum::http::StatusCode;
use serde_json::Value;

use crate::database::Database;
use crate::database::row::Row;
use crate::response::Response;

pub async fn database_hand(db: Extension<Arc<Database>>) -> (StatusCode, Vec<u8>) {
    let row = Row::new();
    let mut res = Response::new(true, Value::String("Greetings".to_string()), Some(row.data));
    return (StatusCode::OK, res.as_buffer());
}
