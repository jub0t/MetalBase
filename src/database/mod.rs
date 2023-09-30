use std::collections::HashMap;
use std::ops::DerefMut;

use crate::database::table::Tables;
use crate::rid::RanID;

pub mod table;
pub mod types;
pub mod row;
pub mod image;
pub mod errors;
pub mod schema;
mod bytes;


#[derive(Clone, Debug)]
pub struct Database {
    id: String,
    name: String,
    tables: Tables,
}

impl Database {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            tables: HashMap::new(),
            id: RanID::new(),
        };
    }
}