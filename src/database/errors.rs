pub enum DatabaseError {
    TableNotFound,
    TableAlreadyExists,
    RowNotFound,
    RowAlreadyExists,
    None,
}