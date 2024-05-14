use crate::{
	ctx::Ctx,
	model::{Error, ModelManager, Result},
};
use sqlb::HasFields;
// use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};

pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();

	// let fields = data.all_fields();
	let fields = data.not_none_fields();
	let (id,) = sqlb::insert()
		.table(MC::TABLE)
		.data(fields)
		.returning(&["id"])
		.fetch_one::<_, (i64,)>(db)
		.await?;

	Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	//higer trait bound
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let db = mm.db();

	// let sql = format!("SELECT id title FROM {} WHERE id = $1", MC::TABLE);
	let entity: E = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.and_where("id", "=", id)
		.fetch_optional(db)
		.await?
		.ok_or(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})?;

	Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
	MC: DbBmc,
	//higer trait bound
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let db = mm.db();

	let entities: Vec<E> = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.order_by("id")
		.fetch_all(db)
		.await?;

	Ok(entities)
}

pub async fn update<MC, E>(
	_ctx: &Ctx,
	mm: &ModelManager,
	id: i64,
	data: E,
) -> Result<()>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();

	// let fields = data.all_fields();
	let fields = data.not_none_fields();
	let count = sqlb::update()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.data(fields)
		.exec(db)
		.await?;

	if count == 0 {
		return (Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		}));
	}

	Ok(())
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	let db = mm.db();

	let count = sqlb::delete()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.exec(db)
		.await?;

	if count == 0 {
		return (Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		}));
	}

	Ok(())
}
