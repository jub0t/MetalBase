use std::sync::{Arc, Mutex};

use axum::Extension;
use axum::http::StatusCode;

use crate::database::Database;
use crate::database::row::Row;
use crate::database::types::FieldValue;
use crate::response::{Buffer, Response};

pub async fn table_hand(db: Extension<Arc<Mutex<Database>>>) -> (StatusCode, Buffer) {
    let db = db.lock().unwrap();
    let mut users = db.get_table("users").unwrap();
    let mut user = Row::new();
    &user.data.insert("username".to_string(), FieldValue::String("Bob".to_string()));

    users.insert(user.clone());

    let all = users.get_all();
    let mut res = Response::new(true, FieldValue::Null, Some(all));

    return (StatusCode::OK, res.as_buffer());
}
