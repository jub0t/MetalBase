use std::collections::HashMap;

use axum::Json;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database::types::FieldValue;

pub type Buffer = Vec<u8>;

#[derive(Deserialize, Serialize)]
pub enum ResponseTypes<T> {
    Success(bool),
    Message(Option<FieldValue>),
    Time(usize),
    Data(T),
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Response<T>
    where
        T: Serialize + Clone,
{
    pub success: bool,
    pub message: Option<FieldValue>,
    pub time: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T>
    where
        T: Serialize + Clone,
{
    pub fn new(success: bool, message: Option<FieldValue>, data: Option<T>) -> Self {
        Response {
            time: None,
            success,
            message,
            data,
        }
    }

    pub fn success(&mut self, success: bool) {
        self.success = success;
    }

    pub fn message(&mut self, message: &str) {
        self.message = Some(FieldValue::String(message.to_string()));
    }

    pub fn set_time(&mut self, time: String) {
        self.time = Some(time);
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn as_struct(&self) -> Self {
        Response {
            time: self.time.clone(),
            success: self.success,
            message: self.message.clone(),
            data: self.data.clone(),
        }
    }

    pub fn as_hashmap(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("success".to_string(), serde_json::to_value(self.success.clone()).unwrap());

        if let Some(message) = &self.message {
            map.insert("message".to_string(), serde_json::to_value(self.message.clone()).unwrap());
        } else {
            map.insert("message".to_string(), serde_json::to_value(Value::Null).unwrap());
        }

        if let Some(time) = self.time.clone() {
            map.insert("time".to_string(), serde_json::to_value(self.time.clone()).unwrap());
        }

        if let Some(data) = &self.data {
            map.insert("data".to_string(), serde_json::to_value(data.clone()).unwrap());
        }

        return map;
    }


    pub fn as_json(&self) -> Json<Self> {
        Json(self.as_struct())
    }

    pub fn as_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let resp = self.as_hashmap();

        let _ = resp.serialize(&mut Serializer::new(&mut buffer)).unwrap();
        return buffer;
    }
}
