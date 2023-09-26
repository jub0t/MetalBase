use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;

use crate::database::Database;
use crate::response::{Buffer, Response};
use crate::time::Time;

pub async fn table_hand(db: State<Arc<Mutex<Database>>>) -> (StatusCode, Buffer) {
    let db_lock = db.lock();

    return match db_lock {
        Ok(db) => {
            let start = std::time::Instant::now();
            let mut users = db.get_table("users").unwrap().get_first_till(1000);
            let time = Time::new();

            let mut res = Response::new(true, None, Some(users));
            res.set_time(time.elapsed_fmt());

            (StatusCode::OK, res.as_buffer())
        }
        Err(poison_err) => {
            eprintln!("Mutex is poisoned: {:#?}", poison_err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, vec![])
        }
    };
}
