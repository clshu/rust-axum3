//! Model Layer
//!
//! Design:
//!
//! - The Model Layer normalize the
//!   application's data structure and access
//! - All application code data access must go through Model layer
//! - The `ModelManager` holds the internal state/access
//!   needed by ModelControllers to access data
//!   (e.g., db_pool, S3 client, redis client)
//! - ModelControllers(e.g. `Taskmc`, `Projectmc`) implemt
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short fot BAckend Controllrt)
//! - In frameworks like Axum, Tauri, `ModelManager` is typically used as App State.
//! - ModelManager is designed to be passed as an argument to all
//!   Model Controllers' functions.

// region:    --- Modules

mod base;
mod error;
mod store;
pub mod task;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		Ok(ModelManager { db })
	}

	// Returns the sqlx db pool
	// Only for the model layer
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
