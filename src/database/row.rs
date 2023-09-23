use std::collections::HashMap;

use serde_json::Value;

pub type RowData = HashMap<String, Value>;

#[derive(Clone, Debug)]
pub struct Row {
    pub data: RowData,
}

impl Row {
    pub fn new() -> Self {
        return Self {
            data: HashMap::new(),
        };
    }
}