use std::sync::{Arc, Mutex};

use axum::Extension;
use axum::http::StatusCode;

use crate::database::Database;
use crate::response::{Buffer, Response};

pub async fn table_hand(db: Extension<Arc<Mutex<Database>>>) -> (StatusCode, Buffer) {
    let db_lock = db.lock();

    return match db_lock {
        Ok(db) => {
            let mut all = db.get_all_rows("users");
            let mut res = Response::new(true, None, Some(all));

            (StatusCode::OK, res.as_buffer())
        }
        Err(poison_err) => {
            eprintln!("Mutex is poisoned: {:#?}", poison_err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, vec![])
        }
    };
}
