use tonic::{Request, Response};
use crate::err::GrpcResult;
use crate::grpc::pb::{FindReplyRequest, ReplyResponse, ReplyEntity};
use crate::grpc::pb::replies_server::{Replies, RepliesServer};

use crate::db::Store;
use crate::models::ReplyMAC;

pub fn config(store: Store) ->  RepliesServer<RepliesService> {
    RepliesServer::new(RepliesService::new(store))
}

pub struct RepliesService {
    store: Store
}
impl RepliesService {
    pub fn new(store: Store) -> Self {
        RepliesService { store }
    }
}

#[tonic::async_trait]
impl Replies for RepliesService {
    async fn find_reply(&self, req: Request<FindReplyRequest>) -> GrpcResult<ReplyResponse> {
        metrics::increment_counter!("requests");
        metrics::increment_counter!("find_reply");

        let message = req.into_inner().message;
        let reply: Option<ReplyEntity> = match ReplyMAC::find_reply(self.store.db(), &message).await? {
            Some(reply) => {
                Some(reply.into())
            }
            None => None
        };
        let res = ReplyResponse {
            status: "ok".to_string(),
            reply,
        };
        Ok(Response::new(res))
    }
}
