use axum::Json;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Response<T>
    where
        T: Serialize + Clone,
{
    pub success: bool,
    pub message: Value,
    pub data: Option<T>,
}

impl<T> Response<T>
    where
        T: Serialize + Clone,
{
    pub fn new(success: bool, message: Value, data: Option<T>) -> Self {
        Response {
            success,
            message,
            data,
        }
    }

    pub fn success(&mut self, success: bool) {
        self.success = success;
    }

    pub fn message(&mut self, message: &str) {
        self.message = Value::String(message.to_string());
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn as_struct(&self) -> Self {
        Response {
            success: self.success,
            message: self.message.clone(),
            data: self.data.clone(),
        }
    }

    pub fn as_json(&self) -> Json<Self> {
        Json(self.as_struct())
    }

    pub fn as_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let resp = self.as_struct();

        let _ = resp.serialize(&mut Serializer::new(&mut buffer)).unwrap();
        return buffer;
    }
}
