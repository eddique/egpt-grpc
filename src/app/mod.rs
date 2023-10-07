use crate::{db, server};
use crate::err::Result;
use crate::metrics;

pub async fn run() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_target(false)
        .init();
    metrics::config();

    let store = db::Store::new().await?;
    server::init(store).await?;

    Ok(())
}
