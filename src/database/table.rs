use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;

pub struct TableConfig {
    custom_ids: bool,
    no_unwanted_fields: bool,
}

pub struct Table {
    id: String,
    name: String,
    config: TableConfig,
    fields: HashMap<String, Value>,
}

impl Table {
    pub fn new(name: String, config: TableConfig) -> Self {
        return Self {
            name,
            fields: HashMap::new(),
            id: Uuid::new_v4().to_string(),
            config: Table::default_config(),
        }
    }

    pub fn default_config() -> TableConfig {
        return TableConfig {
            custom_ids: false,
            no_unwanted_fields: false,
        }
    }
}