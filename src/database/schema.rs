use std::collections::BTreeMap;

use crate::database::types::FieldValue;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SchemaField {
    pub id: String,
    pub name: String,
    pub type_: FieldValue,
}

pub type Schema = BTreeMap<String, SchemaField>;
