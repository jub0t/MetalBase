use std::collections::{BTreeMap, HashMap};

use serde_json::Value;
use uuid::Uuid;

use crate::database::row::Row;

#[derive(Clone, Debug)]
pub struct TableConfig {
    custom_ids: bool,
    schemaless: bool,
}

#[derive(Clone, Debug)]
pub struct Table {
    id: String,
    name: String,
    config: TableConfig,
    schema: HashMap<String, Value>,
    rows: BTreeMap<usize, Row>,
}

impl Table {
    pub fn new(name: &str, config: TableConfig) -> Self {
        return Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            schema: HashMap::new(),
            rows: BTreeMap::new(),
            config,
        };
    }

    pub fn default_config() -> TableConfig {
        return TableConfig {
            custom_ids: false,
            schemaless: false,
        };
    }

    pub fn get_all_where(self, key: &str, value: &str) -> Vec<Row> {
        let mut data = Vec::new();
        let mut rows = self.rows.clone();

        if rows.len() == 0 {
            return data;
        }

        for (size, row) in rows.iter() {
            if row.data[key] == value {
                data.push(row.to_owned());
            }
        }

        return data;
    }

    pub fn get_all_where_multi(self, keys: Vec<&str>, values: Vec<&str>) -> Vec<Row> {
        let mut data = Vec::new();
        let mut rows = self.rows.clone();

        if rows.len() == 0 {
            return data;
        }

        for (size, row) in rows.iter() {
            for (key, value) in keys.iter().zip(values.iter()) {
                if row.data[key] == value {
                    data.push(row.to_owned());
                    break;
                }
            }
        }

        return data;
    }
}