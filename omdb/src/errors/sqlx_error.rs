#[derive(Debug)]
pub enum DbError {
    DuplicateKey(String),
    UnknownError(sqlx::Error),
    InvalidSql(sqlx::Error),
    NoConnection(sqlx::Error),
    CounstraintViolation(sqlx::Error)
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::DuplicateKey(val) => write!(f, "Duplicate key in entity {}", val),
            DbError::CounstraintViolation(e) => write!(f, "Constraint violated {}", e),
            DbError::InvalidSql(e) => write!(f, "invalid sql {}", e),
            DbError::NoConnection(e) => write!(f, "DB connection not found {}", e),
            DbError::UnknownError(e) => write!(f, "Unknown error has occured {}", e)
        }
    }
}
