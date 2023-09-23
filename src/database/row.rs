use std::collections::HashMap;

use serde_json::Value;

pub struct Row {
    pub data: HashMap<String, Value>,
}
