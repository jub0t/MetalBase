#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum FieldValue {
    // Basics
    String(String),
    Boolean(bool),

    // Floats(Decimals)
    Float32(f32),
    Float64(f64),

    // Signed Integers
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i64),

    // Unsigned Integer
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),

    // Specials
    ImagePointer,
    Vector(Box<FieldValue>),

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
}
