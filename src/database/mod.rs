use std::collections::HashMap;
use std::ops::DerefMut;

use rayon::prelude::*;

use crate::database::errors::DatabaseError::TableNotFound;
use crate::database::row::{Row, Rows};
use crate::database::table::{Table, Tables};
use crate::database::types::Value;
use crate::rid::RanID;
use crate::time::Time;

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

    pub async fn find_all(&self, table_name: &str, key: &str, value: Value) -> Result<Vec<Row>, errors::DatabaseError> {
        let mut table = self.tables.get(table_name).unwrap();
        let shards = &mut table.data.clone();

        // Scan All Shards Parallel
        let result: Vec<_> = shards.par_iter()
            .flat_map(|rows| {
                let mut data = Rows::default();
                for row in rows {
                    if row.get(key).unwrap() == &value {
                        data.push(row.clone());
                    }
                }

                data
            })
            .collect();


        Ok(result.into())
    }

    pub async fn search_shard(&self, shard: &Rows, k: &str, v: &Value) -> Rows {
        let mut rows = Rows::new();

        for row in shard {
            if let Some(val) = row.get(k) {
                if val == v {
                    rows.push(row.clone());
                }
            }
        }

        return rows;
    }

    pub fn init_test(&mut self) {
        self.create_table("users".to_string()).expect("TODO: panic message");

        let now = Time::new();
        for x in 0..500_000 {
            let mut row = Row::new();
            row.insert("name".to_string(), Value::String("James".to_string()));
            self.insert("users", row).unwrap();
        }

        let mut row = Row::new();
        row.insert("name".to_string(), Value::String("Bob".to_string()));
        self.insert("users", row).unwrap();
        println!("{}", now.elapsed_fmt());
    }
}