use std::collections::HashMap;

use uuid::Uuid;

use crate::database::row::Row;
use crate::database::types::FieldValue;
use crate::rid::RanID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TableConfig {
    custom_ids: bool,
    schemaless: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SchemaField {
    id: String,
    name: String,
    type_: FieldValue,
}

pub type Schema = HashMap<String, SchemaField>;
pub type Rows = HashMap<String, Row>;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    id: String,
    name: String,
    config: TableConfig,
    schema: Schema,
    rows: Rows,
}

impl Table {
    pub fn new(name: &str, config: TableConfig) -> Self {
        Self {
            id: RanID::new(),
            name: name.to_string(),
            schema: HashMap::new(),
            rows: HashMap::new(),
            config,
        }
    }

    pub fn default_config() -> TableConfig {
        TableConfig {
            custom_ids: false,
            schemaless: false,
        }
    }

    pub fn search(&self, field_name: &str, value: &FieldValue) -> Vec<Row> {
        let target = value.to_string();
        self.rows.values()
            .filter(|row| {
                row.data
                    .get(field_name)
                    .map_or(false,
                            |f| f.to_string() == target)
            })
            .cloned()
            .collect()
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
        let rid = Uuid::new_v4().to_string();
        self.rows.insert(rid.clone(), row);
        rid
    }
}
