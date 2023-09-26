use std::collections::{BTreeMap, HashMap};

use crate::database::row::{Row, Rows};
use crate::database::schema::Schema;
use crate::database::types::FieldValue;
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
            rows: HashMap::new(),
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

    pub fn search(&self, field_name: &str, value: &FieldValue) -> Rows {
        let mut rows = Rows::new();
        let mut i = 0;

        for (id, row) in &self.rows {
            if i >= self.config.search_limit {
                break;
            }

            let field = row.get_raw(field_name);
            if field.match_with(&value) {
                rows.insert(id.to_owned(), row.clone());
            }

            i += 1;
        }

        rows
    }

    pub fn get_all_where_multi(&self, values: &[FieldValue]) -> Vec<Row> {
        if self.rows.is_empty() {
            return Vec::new();
        }

        self.rows.values()
            .filter(|row| {
                values.iter().all(|value| {
                    let field_name = value.to_string();
                    if let Some(field) = self.schema.get(&field_name) {
                        if value.match_with(&field.type_) {
                            return false;
                        }
                        row.data.get(&field_name)
                            .map_or(false, |f| f.to_string() == value.to_string())
                    } else {
                        false
                    }
                })
            })
            .cloned()
            .collect()
    }

    pub fn get_all(&self) -> Rows {
        self.rows.clone()
    }

    pub fn get_all_vec(&self) -> Vec<Row> {
        self.rows.values().cloned().collect()
    }

    pub fn get_rows_data(&self) -> &Rows {
        return &self.rows;
    }

    pub fn get_first_till(&self, limit: usize) -> Rows {
        // Get the first "limit" number of elements from the table
        if self.rows.is_empty() || self.rows.len() < limit {
            return self.rows.to_owned();
        }

        let mut rows: Rows = Rows::new();

        for (rid, row) in self.rows.iter() {
            if rows.len() >= limit { break; }
            rows.insert(rid.to_owned(), row.to_owned());
        }

        return rows;
    }

    pub fn insert(&mut self, row: Row) -> String {
        let rid = RanID::new();
        self.rows.insert(rid.clone(), row);
        rid
    }

    pub fn batch_insert(&mut self, rows: Vec<Row>) {
        for row in rows {
            let rid = RanID::new();
            self.rows.insert(rid.clone(), row);
        }
    }
}
