use std::sync::Arc;

use axum::Extension;
use axum::http::StatusCode;
use rand::prelude::*;

use crate::database::Database;
use crate::database::row::Row;
use crate::database::types::FieldValue;
use crate::response::{Buffer, Response};

pub async fn database_hand(db: Extension<Arc<Database>>) -> (StatusCode, Buffer) {
    let row = Row::new();
    let mut rng = rand::thread_rng();

    let y: f64 = rng.gen(); // generates a float between 0 and 1
    let mut res = Response::new(true, FieldValue::Float64(y), Some(row.data));

    return (StatusCode::OK, res.as_buffer());
}
