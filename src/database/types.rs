use serde_json::Value;

use crate::database::image::ImagePointer;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FieldValue {
    // Basics
    String(String),
    Boolean(bool),

    // Signed Integers
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i64),

    // Unsigned Integer
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),

    // Floats(Decimals)
    Float32(f32),
    Float64(f64),

    // Specials
    Vector(Box<FieldValue>),
    Image(ImagePointer),

    // Defaults
    #[default]
    Null,
}

impl FieldValue {
    pub fn default() -> Self {
        FieldValue::Null
    }

    pub fn to_string(&self) -> String {
        match self {
            FieldValue::String(s) => s.clone(),
            _ => { panic!("Cannot convert to string") }
        }
    }

    pub fn match_with(&self, other: &FieldValue) -> bool {
        match (self, other) {
            (FieldValue::String(s1), FieldValue::String(s2)) => s1 == s2,
            _ => false
        }
    }

    pub fn flatten(&self) -> Value {
        match self {
            FieldValue::String(s) => Value::String(s.clone()),
            FieldValue::Boolean(b) => Value::Bool(b.clone()),
            FieldValue::Int8(i) => Value::Number(i.to_string().parse().unwrap()),
            FieldValue::Int16(i) => Value::Number(i.to_string().parse().unwrap()),

            (other) => {
                match other {
                    FieldValue::Null => Value::Null,
                    _ => {
                        Value::Number(other.to_string().parse().unwrap())
                    }
                }
            }
        }
    }
}
