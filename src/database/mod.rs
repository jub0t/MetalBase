use std::collections::HashMap;
use std::ops::DerefMut;

use crate::database::errors::DatabaseError::TableNotFound;
use crate::database::table::{Table, Tables};
use crate::database::types::Value;
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

    pub fn create_table(&mut self, table_name: String) -> Result<(), errors::DatabaseError> {
        self.tables.insert(table_name, Table::default());

        return Ok(());
    }

    pub fn insert(&mut self, table_name: &str, row: row::Row) -> Result<(), errors::DatabaseError> {
        let mut table = self.tables.get_mut(table_name);

        return match table {
            Some(t) => t.insert(row),
            None => return Err(TableNotFound(table_name.to_string()))
        };
    }

    pub fn find_all(&self, table_name: &str, key: &str, value: Value) -> Result<row::Rows, errors::DatabaseError> {
        let mut table = self.tables.get(table_name).unwrap();
        let shards = &table.data;

        // Iterate over all shards parallel
        let mut rows = Vec::new();
        for shard in shards {
            for row in shard {
                if row.get(key) == Some(&value) {
                    rows.push(row.clone());
                }
            }
        }

        return Ok(rows);
    }
}