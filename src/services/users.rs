use tonic::{Request, Response};
use crate::err::GrpcResult;
use crate::grpc::pb::{UserByEmailRequest, UserResponse, UserEntity};
use crate::grpc::pb::user_server::{User, UserServer};

use crate::db::Store;
use crate::models::UserMAC;

pub fn config(store: Store) -> UserServer<UserService> {
    UserServer::new(UserService::new(store))
}

pub struct UserService {
    store: Store
}
impl UserService {
    pub fn new(store: Store) -> Self {
        UserService { store }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn email_lookup(&self, req: Request<UserByEmailRequest>) -> GrpcResult<UserResponse> {
        metrics::increment_counter!("requests");
        metrics::increment_counter!("email_lookup");

        let email = req.into_inner().email;
        let user: Option<UserEntity> = match UserMAC::email_lookup(self.store.db(), &email).await? {
            Some(user) => {
                Some(user.into())
            }
            None => None
        };
        let res = UserResponse {
            status: "ok".to_string(),
            user,
        };
        Ok(Response::new(res))
    }
}
