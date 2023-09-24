use std::collections::HashMap;

use uuid::Uuid;

use crate::database::row::Row;
use crate::database::types::FieldValue;
use crate::ranid::RanID;

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
        return Self {
            id: RanID::new(),
            name: name.to_string(),
            schema: HashMap::new(),
            rows: HashMap::new(),
            config,
        };
    }

    pub fn default_config() -> TableConfig {
        return TableConfig {
            custom_ids: false,
            schemaless: false,
        };
    }

    pub fn get_all_where(self, field_name: &str, value: FieldValue) -> Vec<Row> {
        let mut data = Vec::new();

        let field = self.schema.get(field_name).unwrap();

        if value.match_with(&field.type_) {
            return data;
        }

        let mut rows = self.rows.clone();

        if rows.len() == 0 {
            return data;
        }

        match field.type_ {
            FieldValue::String(_) => {
                let target = value.to_string();
                for (_, row) in rows.iter() {
                    let f = row.data.get(field_name).unwrap().to_string();
                    if f == target {
                        data.push(row.clone());
                    }
                }
            }

            _ => {}
        }


        return data;
    }

    pub fn get_all_where_multi(self, keys: Vec<SchemaField>, values: Vec<FieldValue>) -> Vec<Row> {
        let mut data = Vec::new();
        let mut rows: Rows = self.rows.clone();

        if rows.len() == 0 {
            return data;
        }

        for (index, row) in rows.iter() {}

        return data;
    }

    pub fn get_all(&self) -> Vec<Row> {
        return self.rows.clone().into_values().collect::<Vec<Row>>();
    }

    pub fn insert(&mut self, row: Row) -> String {
        let rid = Uuid::new_v4().to_string();
        self.rows.insert(rid.clone(), row);

        return rid;
    }
}