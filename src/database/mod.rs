use std::collections::HashMap;
use std::ops::DerefMut;

use crate::database::row::Row;
use crate::database::table::{Rows, Table};
use crate::database::types::FieldValue;
use crate::rid::RanID;

pub mod table;
pub mod types;
pub mod row;
pub mod image;
pub mod errors;

pub type Tables = HashMap<String, Table>;

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

    // Return a reference to the table for direct access
    pub fn get_table(&self, table_name: &str) -> Option<&Table> {
        self.tables.get(table_name)
    }

    pub fn create_table(&mut self, name: &str) -> Option<&Table> {
        self.tables.insert(name.to_string(), Table::new(name, table::Table::default_config()));
        return self.tables.get(name);
    }

    pub fn get(&self, table: &str, field: &str, value: FieldValue) -> Vec<Row> {
        if let Some(table) = self.tables.get(table) {
            return table.clone().get_all_where(field, &value);
        }

        Vec::new()
    }

    pub fn get_all_rows(&self, table: &str) -> Rows {
        if let Some(table) = self.tables.get(table) {
            return table.clone().get_all();
        }

        return Rows::default();
    }

    pub fn insert(&mut self, table: &str, row: Row) {
        if let Some(table) = self.tables.get_mut(table) {
            table.insert(row);
        }
    }
}