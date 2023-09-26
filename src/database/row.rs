use std::collections::HashMap;

use serde_json::Value;

use crate::database::types::FieldValue;

pub type RowData = HashMap<String, FieldValue>;
pub type ValueData = HashMap<String, Value>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Row {
    pub data: RowData,
    
    #[serde(skip_serializing)]
    pub flat_data: Value,
}

impl Row {
    pub fn new() -> Self {
        return Self {
            data: HashMap::new(),
            flat_data: Value::Null,
        };
    }

    pub fn get(&self, field_name: &str) -> Option<&FieldValue> {
        return self.data.get(field_name);
    }

    pub fn get_mut(&mut self, field_name: &str) -> Option<&mut FieldValue> {
        return self.data.get_mut(field_name);
    }

    pub fn get_flat(&self) -> Value {
        return serde_json::to_value(&self.data).unwrap();
    }

    pub fn flatten(&mut self) {
        // Convert "data" into "flat_data"
        let flat_data = serde_json::to_value(&self.data).unwrap();
        self.flat_data = flat_data
    }
}