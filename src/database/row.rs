use std::collections::HashMap;

use serde_json::{to_value, Value};

use crate::database::types::FieldValue;

pub type RowData = HashMap<String, FieldValue>;
pub type ValueData = HashMap<String, Value>;
pub type Rows = HashMap<String, Row>;
pub type VecRow = Vec<Row>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Row {
    pub data: RowData,

    #[serde(skip_serializing)]
    pub flat_data: Value,
}

impl Row {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            flat_data: Value::Null,
        }
    }

    pub fn get(&self, field_name: &str) -> Option<&FieldValue> {
        self.data.get(field_name)
    }

    pub fn get_raw(&self, field_name: &str) -> &FieldValue {
        &self.data[field_name]
    }

    pub fn get_mut(&mut self, field_name: &str) -> Option<&mut FieldValue> {
        self.data.get_mut(field_name)
    }

    pub fn get_flat(&self) -> &Value {
        &self.flat_data
    }

    pub fn flatten(&mut self) {
        // Convert "data" into "flat_data" if it's not already flattened
        if self.flat_data == Value::Null {
            self.flat_data = to_value(&self.data).expect("JSON serialization failed");
        }
    }
}
