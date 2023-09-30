use serde::Serialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Value {
    // Basics
    String(String),
    Boolean(bool),

    // Numbers
    Int(i32),
    UInt(u32),
    Float(f32),

    None,
}

impl Default for Value {
    fn default() -> Self {
        return Self::None;
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        return match self {
            Self::String(s) => serializer.serialize_str(s),
            Self::Boolean(b) => serializer.serialize_bool(b.clone()),
            Self::Int(i) => serializer.serialize_i32(*i),
            Self::UInt(u) => serializer.serialize_u32(*u),
            Self::Float(f) => serializer.serialize_f32(*f),
            Self::None => serializer.serialize_none(),
        };
    }
}

impl Value {}
