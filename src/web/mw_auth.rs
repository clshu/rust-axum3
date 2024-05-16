use crate::crypt::token::{validate_web_token, Token};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForAuth};
use crate::model::ModelManager;
use crate::web::{set_token_cookie, AUTH_TOKEN};
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::body::HttpBody;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_ctx_require<B>(
	ctx: Result<Ctx>,
	req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
	mm: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

	let result_ctx = _ctx_resolve(mm, &cookies).await;

	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie))
	{
		cookies.remove(Cookie::named(AUTH_TOKEN))
	}

	// Store the ctx_result in the request extension.
	// for Ctx extractor
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
	// -- Get Token String
	let token = cookies
		.get(AUTH_TOKEN)
		.map(|c| c.value().to_string())
		.ok_or(CtxExtError::TokenNotInCookie)?;

	// -- Get UserForAuth
	let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

	// -- Validate Token
	let user: UserForAuth =
		UserBmc::first_by_username(&Ctx::root_ctx(), &mm, &token.ident)
			.await
			.map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
			.ok_or(CtxExtError::UserNotFound)?;

	validate_web_token(&token, &user.token_salt.to_string())
		.map_err(|_| CtxExtError::FailValidate)?;

	// -- Update Token
	// extend the exp time
	set_token_cookie(cookies, &user.username, &user.token_salt.to_string())
		.map_err(|_| CtxExtError::CanNotSetTokenCookie)?;
	// -- Create CtxExtResult
	Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}
// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!("{:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxExt)
	}
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
	// Token
	TokenNotInCookie,
	TokenWrongFormat,
	// Cookie
	FailValidate,
	CanNotSetTokenCookie,
	// Model
	UserNotFound,
	ModelAccessError(String),

	CtxNotInRequestExt,
	CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
