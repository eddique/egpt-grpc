use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;
use crate::config::config;
use crate::err::*;

pub type Db = Pool<Postgres>;

const SQL_RECREATE_DB: &str = "sql/00-recreate.sql";
const SQL_DIR: &str = "sql";

#[derive(Clone, Debug)]
pub struct Store {
    db: Db,
}
impl Store {
    pub async fn new() -> Result<Self> {
        init().await;
        let db = new_db_pool(&config().PG_ADMIN_URL).await?;
        let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db).await?;
        tracing::info!("db initialized: {}", row.0);
        Ok(Store { db  })
    }
    pub fn db(&self) -> &Db {
        &self.db
    }
}

pub async fn new_db_pool(conn_str: &str) -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(conn_str)
        .await
        .map_err(Error::Sqlx)
}
async fn pexec(db: &Db, path: &str) -> Result<()> {
    tracing::info!("{:<12} - pexec: {path}", "DEV-ONLY");
    let content = fs::read_to_string(path)?;
    let sqls: Vec<&str> = content.split(';').collect();
    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }
    Ok(())
}
pub async fn make_migrations() -> Result<()> {
    {
        let root_db = new_db_pool(&config().PG_ADMIN_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }
    let mut paths: Vec<PathBuf> = std::fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path())).collect();
    paths.sort();

    let app_conn = new_db_pool(&config().PG_ADMIN_URL).await?;

    for path in paths {
        if let Some(path) = path.to_str() {
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {

                pexec(&app_conn, path).await?;
            }
        }
    }

    Ok(())

}
pub async fn init() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        tracing::info!("{:<12} - db::init()", "DEV-ONLY");
        make_migrations().await.unwrap()
    }).await;
}
