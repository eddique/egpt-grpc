use std::time::{SystemTime, Duration};
use sqlx::types::time::OffsetDateTime;
use tonic::{Request, Response};
use crate::err::GrpcResult;
use crate::grpc::pb::{Void, GetMessagesResponse, MessageEntity};
use crate::grpc::pb::message_server::{Message, MessageServer};

use crate::db::Store;
use crate::models::MessageMAC;

pub fn config(store: Store) ->  MessageServer<MessageService> {
    MessageServer::new(MessageService::new(store))
}

pub struct MessageService {
    store: Store
}
impl MessageService {
    pub fn new(store: Store) -> Self {
        MessageService { store }
    }
}

#[tonic::async_trait]
impl Message for MessageService {
    async fn get_messages(&self, _: Request<Void>) -> GrpcResult<GetMessagesResponse> {
        let five_minutes = Duration::from_secs(5 * 60);
        let five_minutes_ago = SystemTime::now()
            .checked_sub(five_minutes)
            .expect("System clock was set back");
        
        let before = OffsetDateTime::from(five_minutes_ago);
        let messages = MessageMAC::get_messages(self.store.db(), before).await?
            .drain(..)
            .map(|msg| msg.into())
            .collect::<Vec<MessageEntity>>();
        let res = GetMessagesResponse { messages };
        Ok(Response::new(res))
    }
}
