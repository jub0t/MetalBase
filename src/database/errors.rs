#[derive(Debug)]
pub enum DatabaseError {
    TableNotFound(String),
    TableAlreadyExists,
    RowNotFound,
    RowAlreadyExists,
    None,
}

impl DatabaseError {
    pub fn to_string(&self) -> String {
        return format!("{:?}", self);
    }
}