#[derive(Debug)]
pub struct DbError {
    pub err_msg: String,
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        Self {
            err_msg: format!("{}", e),
        }
    }
}
