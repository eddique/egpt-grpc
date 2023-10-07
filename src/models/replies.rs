use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::{db::Db, err::Result, grpc::pb::ReplyEntity};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Reply {
    pub id: i32,
    pub look_for: String,
    pub reply_with: String,
}
impl Into<ReplyEntity> for Reply {
    fn into(self) -> ReplyEntity {
        ReplyEntity {
            id: self.id,
            look_for: self.look_for,
            reply_with: self.reply_with,
        }
    }
}
pub struct ReplyMAC;

impl ReplyMAC {
    pub async fn find_reply(db: &Db, message: &str) -> Result<Option<Reply>> {
        let sql = r#"
            SELECT id, email, first_name, last_name, active, role, created_at, updated_at
            FROM users
            WHERE email = $1
        "#;
        let user = sqlx::query_as::<_, Reply>(sql)
            .bind(message)
            .fetch_optional(db)
            .await?;
        
        Ok(user)
    }
}