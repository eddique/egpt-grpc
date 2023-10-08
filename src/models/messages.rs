use prost_types::Timestamp;
use sqlx::{FromRow, types::time::OffsetDateTime};
use crate::{db::Db, err::Result, grpc::pb::MessageEntity};

#[derive(Debug, FromRow)]
pub struct Message {
    pub id: i32,
    pub channel: String,
    pub text: String,
    pub post_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

fn timestamp(ts: OffsetDateTime) -> Option<Timestamp> {
    let nanos = ts.unix_timestamp_nanos() as i32;
    let seconds = ts.unix_timestamp() as i64;
    let ts = Timestamp { seconds, nanos };
    Some(ts)
}

impl From<Message> for MessageEntity {
    fn from(value: Message) -> Self {
        MessageEntity { 
            id: value.id, 
            channel: value.channel, 
            text: value.text, 
            post_at: timestamp(value.post_at), 
            created_at: timestamp(value.created_at), 
            updated_at: timestamp(value.updated_at),
        }
    }
}

pub struct MessageMAC;

impl MessageMAC {
    pub async fn get_messages(db: &Db, ts: OffsetDateTime) -> Result<Vec<Message>> {
        let sql = r#"
            SELECT id, channel, text, post_at, created_at, updated_at
            FROM scheduled_messages
            WHERE post_at NOT NULL AND post_at < $1
        "#;
        let messages = sqlx::query_as::<_, Message>(sql)
            .bind(ts)
            .fetch_all(db)
            .await?;
        
        Ok(messages)
    }
}