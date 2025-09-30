use sqlx::{query_as, PgPool};
use rlab_core::models::User;
use crate::errors::DbError;
use crate::models::{NewUser, DbUser};

pub struct DbUserRepository {
    pool: PgPool
}

impl DbUserRepository {
    pub fn new(pool: PgPool) -> DbUserRepository {
        DbUserRepository { pool }
    }
    pub async fn create(&self, new_user: &NewUser) -> Result<User, DbError> {
        let created_db_user = query_as!(
            DbUser,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
            new_user.name,
            new_user.email
        ).fetch_one(&self.pool).await?;

        // TODO logging

        println!("New user created: {:?}", &created_db_user);
        Ok(created_db_user.into())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        let user = query_as!(
            DbUser,
            "SELECT * FROM users WHERE email = $1",
            email
        ).fetch_optional(&self.pool).await?;
        Ok(user.map(Into::into))
    }
}
