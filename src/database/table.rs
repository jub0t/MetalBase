use std::collections::{BTreeMap, HashMap};

use serde_json::Value;
use uuid::Uuid;

use crate::database::row::Row;

pub struct TableConfig {
    custom_ids: bool,
    no_unwanted_fields: bool,
}

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
            no_unwanted_fields: false,
        };
    }
}