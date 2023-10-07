mod app;
mod config;
mod db;
mod err;
mod metrics;
mod models;
mod server;
mod services;

#[tokio::main]
async fn main() -> err::Result<()> {
    app::run().await?;
    Ok(())
}
