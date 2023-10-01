use std::sync::{Arc, Mutex};

use crate::database::Database;
use crate::database::row::Rows;
use crate::response::{Bytes, Response};
use crate::time::Time;

pub fn table_hand(db: &Arc<Mutex<Database>>) -> Result<Bytes, ()> {
    let db_lock = db.lock();
    let mut res = Response::new(true, None, Some(Rows::default()));

    return match db_lock {
        Ok(mut db) => {
            let start = std::time::Instant::now();
            let time = Time::new();
            // let rows = db.find_all("users", "name", Value::String("Bob".to_string())).unwrap();
            //
            // res.set_data(rows);
            // res.set_time(time.elapsed_fmt());
            Ok(res.as_buffer())
        }
        Err(poison_err) => {
            eprintln!("Mutex is poisoned: {:#?}", poison_err.to_string());
            Ok(res.as_buffer())
        }
    };
}
