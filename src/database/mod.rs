use std::collections::HashMap;

use uuid::Uuid;

use crate::database::table::Table;

pub mod table;
pub mod types;
pub mod row;

pub type Tables = HashMap<String, Table>;

#[derive(Clone, Debug)]
pub struct Database {
    id: String,
    name: String,
    tables: Tables,
}

impl Database {
    pub fn new(name: String) -> Self {
        return Self {
            name,
            tables: HashMap::new(),
            id: Uuid::new_v4().to_string(),
        };
    }

    pub fn get_all_tables(self) -> Tables {
        return self.tables;
    }

    pub fn create_table(&mut self, name: &str) -> Option<&Table> {
        self.tables.insert(name.to_string(), Table::new(name, table::Table::default_config()));
        return self.tables.get(name);
    }
}