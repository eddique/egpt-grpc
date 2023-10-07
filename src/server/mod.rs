mod pb {
    tonic::include_proto!("users");
}

use tonic::transport::Server;
use tonic::{Request, Response, Status};
use pb::{UserByEmailRequest, UserResponse, UserEntity};
use pb::user_server::{UserServer, User};

use crate::err::Result;
use crate::db::Store;
use crate::models::UserMAC;


type GrpcResult<T> = std::result::Result<Response<T>, Status>;
pub struct UserService {
    store: Store
}
impl Into<UserEntity> for crate::models::User {
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
impl UserService {
    pub fn new(store: Store) -> Self {
        UserService { store }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn email_lookup(&self, req: Request<UserByEmailRequest>) -> GrpcResult<UserResponse> {
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

pub async fn init(store: Store) ->  Result<()> {
    let addr = "[::1]:50051".parse()?;

    let user_service = UserServer::new(UserService::new(store));
    tracing::info!("grpc::server listening on {addr}");
    Server::builder()
        .add_service(user_service)
        .serve(addr)
        .await?;
    Ok(())
}