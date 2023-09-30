use std::collections::BTreeMap;

use crate::database::types::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SchemaField {
    pub id: String,
    pub name: String,
    pub type_: Value,
}

pub type Schema = BTreeMap<String, SchemaField>;
