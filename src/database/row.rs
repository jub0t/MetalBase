use std::collections::HashMap;

use crate::database::types::Value;

pub type RowData = HashMap<String, Value>;
pub type Rows = Vec<Row>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Row {
    pub data: RowData,
}

impl Row {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
