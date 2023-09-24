use std::collections::HashMap;

use crate::database::types::FieldValue;

pub type RowData = HashMap<String, FieldValue>;

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