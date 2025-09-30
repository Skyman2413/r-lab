use rlab_core::models::User;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct DbUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl From<DbUser> for User {
    fn from(user: DbUser) -> User {
        User {
            id: user.id as u64,
            name: user.name,
            email: user.email,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
