pub type Result<T> = std::result::Result<T, Error>;
use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] tokio::io::Error),

    #[error(transparent)]
    EnvironmentVar(#[from] std::env::VarError),
    
    #[error("error getting vector")]
    Vector,

    #[error("error: {0}")]
    Box(#[from] Box<dyn std::error::Error>),

    #[error("error: {0}")]
    Grpc(#[from] tonic::Status),

    #[error("error: {0}")]
    Server(#[from] tonic::transport::Error),

    #[error("error: {0}")]
    SocketParse(#[from] std::net::AddrParseError),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("SQL error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Sqlx(_) => Status::aborted("sqlx error"),
            Error::IO(_) => Status::aborted("io error"),
            _ => Status::aborted("internal error"),
        }
    }
}