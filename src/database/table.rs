use std::collections::{BTreeMap, HashMap};

use crate::database::errors::DatabaseError;
use crate::database::row::{Row, Rows, RowShards};
use crate::database::schema::Schema;
use crate::rid::RanID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TableConfig {
    custom_ids: bool,
    schemaless: bool,
    search_limit: usize,
    shard_limit: usize,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            custom_ids: false,
            schemaless: false,
            shard_limit: 100000,
            search_limit: 100000,
        }
    }
}

pub type Tables = HashMap<String, Table>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    pub name: String,
    pub data: RowShards,
    config: TableConfig,
    schema: Schema,
}

impl Default for Table {
    fn default() -> Self {
        Self::new("", TableConfig::default())
    }
}

impl Table {
    pub fn new(name: &str, config: TableConfig) -> Self {
        Self {
            id: RanID::new(),
            name: name.to_string(),
            schema: BTreeMap::new(),
            data: RowShards::new(),
            config,
        }
    }

    pub fn default_config() -> TableConfig {
        TableConfig {
            search_limit: 100000,
            custom_ids: false,
            schemaless: false,
            shard_limit: 50000,
        }
    }

    pub fn insert(&mut self, row: Row) -> Result<(), DatabaseError> {
        match self.data.last_mut() {
            Some(shard) => {
                shard.push(row.clone());
            }
            None => {
                self.data.push(Rows::new());
            }
        }

        return Ok(());
    }
}
