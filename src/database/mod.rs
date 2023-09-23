use std::collections::HashMap;

use uuid::Uuid;

use crate::database::table::Table;

mod table;
mod types;
mod row;

pub struct Database {
    id: String,
    name: String,
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(name: String) -> Self {
        return Self {
            name,
            tables: HashMap::new(),
            id: Uuid::new_v4().to_string(),
        };
    }

    pub fn get_table(&self, name: String) -> Option<&Table> {
        return self.tables.get(&name);
    }

    pub fn create_table(&mut self, name: &str) -> Option<&Table> {
        self.tables.insert(name.to_string(), Table::new(name, table::Table::default_config()));
        return self.tables.get(name);
    }
}