// region:    --- Modules

mod error;

// use std::time::Duration;

// Reexport for this module
pub use self::error::{Error, Result};

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	info!("{:<12} - new_db_pool", "DB");
	PgPoolOptions::new()
		.max_connections(5)
		// .acquire_timeout(Duration::from_millis(500))
		.connect(&config().DB_URL)
		.await
		.map_err(|ex| Error::FailToCratePool(ex.to_string()))
}
