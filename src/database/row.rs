use std::collections::HashMap;

use crate::database::types::Value;

pub type Row = HashMap<String, Value>;
pub type Rows = Vec<Row>;
pub type RowShards = Vec<Rows>;