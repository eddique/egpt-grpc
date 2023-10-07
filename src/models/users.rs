use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::{db::Db, err::Result};
use crate::grpc::pb::UserEntity;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub active: bool,
    pub role: String,
}

impl Into<UserEntity> for User {
    fn into(self) -> UserEntity {
        UserEntity {
            id: self.id,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            active: self.active,
            role: self.role,
        }
    }
}
pub struct UserMAC;

impl UserMAC {
    pub async fn email_lookup(db: &Db, email: &str) -> Result<Option<User>> {
        let sql = r#"
            SELECT id, email, first_name, last_name, active, role, created_at, updated_at
            FROM users
            WHERE email = $1
        "#;
        let user = sqlx::query_as::<_, User>(sql)
            .bind(email)
            .fetch_optional(db)
            .await?;
        
        Ok(user)
    }
}