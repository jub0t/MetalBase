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

    pub async fn find_all(&self, table_name: &str, key: &str, value: Value) -> Result<Rows, errors::DatabaseError> {
        let mut table = self.tables.get(table_name).unwrap();
        let mut result = vec![];

        // Iterate over every shard at the same time using rayon's par_iter()
        let res = table.data.par_iter().map(|shard| {
            let rows = self.search_shard(shard, key, &value).iter().rev().collect::<Rows>();
            rows
        }).into_par_iter().rev().collect::<Rows>();

        Ok(result)
    }

    pub fn search_shard(&self, shard: &Rows, k: &str, v: &Value) -> Rows {
        let mut rows = Rows::new();

        for row in shard {
            if let Some(val) = row.get(k) {
                if val == v {
                    rows.push(row.clone());
                }
            }
        }

        rows
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