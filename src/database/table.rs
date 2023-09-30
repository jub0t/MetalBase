use std::collections::{BTreeMap, HashMap};

use crate::database::row::Rows;
use crate::database::schema::Schema;
use crate::rid::RanID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TableConfig {
    custom_ids: bool,
    schemaless: bool,
    search_limit: usize,
}

pub type Tables = HashMap<String, Table>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    pub name: String,
    config: TableConfig,
    schema: Schema,
    pub rows: Rows,
}

impl Table {
    pub fn new(name: &str, config: TableConfig) -> Self {
        Self {
            id: RanID::new(),
            name: name.to_string(),
            schema: BTreeMap::new(),
            rows: Rows::new(),
            config,
        }
    }

    pub fn default_config() -> TableConfig {
        TableConfig {
            search_limit: 100000,
            custom_ids: false,
            schemaless: false,
        }
    }
}
