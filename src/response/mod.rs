use std::collections::HashMap;

use axum::Json;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Serialize)]
pub struct Response {
    pub success: bool,
    pub message: Value,
    pub data: HashMap<String, Value>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            success: false,
            message: Value::Null,
            data: HashMap::with_capacity(16),
        }
    }

    pub fn success(&mut self, success: bool) {
        self.success = success;
    }

    pub fn message(&mut self, message: &str) {
        self.message = Value::String(message.to_string());
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.data = HashMap::with_capacity(capacity);
    }

    pub fn data_field(&mut self, key: &str, value: Value) {
        self.data.insert(key.to_string(), value);
    }

    pub fn json_response(self) -> Json<Self> {
        Json(Response {
            success: self.success,
            message: self.message,
            data: self.data,
        })
    }
}
