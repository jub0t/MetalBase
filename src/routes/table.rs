use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;

use crate::database::Database;
use crate::database::row::Rows;
use crate::database::types::Value;
use crate::response::Response;
use crate::time::Time;

pub async fn table_hand(db: State<Arc<Mutex<Database>>>) -> (StatusCode, Vec<u8>) {
    let db_lock = db.lock();

    return match db_lock {
        Ok(mut db) => {
            let start = std::time::Instant::now();
            let time = Time::new();
            let rows = db.find_all("users", "name", Value::String("Bob".to_string())).unwrap();
            let mut res = Response::new(true, None, Some(Rows::default()));

            res.set_data(rows);
            res.set_time(time.elapsed_fmt());
            (StatusCode::OK, res.as_buffer())
        }
        Err(poison_err) => {
            eprintln!("Mutex is poisoned: {:#?}", poison_err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, vec![])
        }
    };
}
