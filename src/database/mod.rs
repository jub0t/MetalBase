use std::collections::HashMap;
use std::ops::DerefMut;

use rayon::prelude::*;

use crate::database::errors::DatabaseError::TableNotFound;
use crate::database::row::Row;
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

    pub fn find_all(&self, table_name: &str, key: &str, value: Value) -> Result<Vec<Row>, errors::DatabaseError> {
        let table = self.tables.get(table_name).unwrap();
        let shards = &table.data;
        let mut results = vec![];

        // Iterate over shards
        for shard in &table.data {
            // Search for Value in rows within each shard
            for row in shard {
                if let Some(val) = row.get(key) {
                    if *val == value {
                        results.push(row.clone());
                    }
                }
            }
        }

        Ok(results)
    }
}