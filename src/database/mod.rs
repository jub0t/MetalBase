mod table;

use crate::database::table::Table;
use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;

pub struct Database {
    id: String,
    name: String,
    table: HashMap<String, Table>,
}

impl Database {
    pub fn new(name: String) -> Self {
        return Self {
            name,
            table: HashMap::new(),
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn get_table(&self, name: String) -> Option<&Table> {
        return self.table.get(&name);
    }
}