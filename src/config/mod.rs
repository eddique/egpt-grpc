use std::env;
use std::sync::OnceLock;
use crate::err::Result;

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct Config {
    pub PG_ADMIN_URL: String,
}

impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
            PG_ADMIN_URL: config_pg()?,
		})
	}
}

fn config_pg() -> Result<String> {
	let user = env::var("PG_USER")?;
	let pwd = env::var("PG_PASSWORD")?;
	let host = env::var("PG_HOST")?;
	let port = env::var("PG_PORT").unwrap_or(format!("5432"));
	let ns = env::var("PG_NAME")?;
	let conn_str = format!("postgres://{}:{}@{}:{}/{}", user, pwd, host, port, ns);
	tracing::info!("conn_str: {conn_str}");
	Ok(conn_str)
}
