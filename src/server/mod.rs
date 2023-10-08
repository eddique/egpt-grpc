use tonic::transport::Server;

use crate::err::Result;
use crate::db::Store;
use crate::services;

pub async fn init(store: Store) ->  Result<()> {
    let addr = "0.0.0.0:50051".parse()?;
    tracing::info!("grpc::server listening on {addr}");

    let users_svc = services::users::config(store.clone());
    let message_svc = services::messages::config(store.clone());
    let replies_svc = services::replies::config(store);
    Server::builder()
        .add_service(users_svc)
        .add_service(message_svc)
        .add_service(replies_svc)
        .serve(addr)
        .await?;
    Ok(())
}