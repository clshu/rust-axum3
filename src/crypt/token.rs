use std::{fmt::Display, str::FromStr};

use super::{encrypt_into_b64u, Error, Result};
use crate::{
	config,
	utils::{
		self, b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc,
	},
};

// region:    --- Token Type

// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug)]
pub struct Token {
	pub ident: String,     // Identifier, e.g., username
	pub exp: String,       // Exipration date in Rfs3309
	pub sign_b64u: String, // signature, base64url encoded
}

// FromStr: e.g. let token: Token = token_str.parse()?
impl FromStr for Token {
	type Err = Error;

	fn from_str(token_str: &str) -> std::prelude::v1::Result<Self, Self::Err> {
		let splits: Vec<&str> = token_str.split('.').collect();
		if splits.len() != 3 {
			return Err(Error::TokenInvalidFormat);
		}

		let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

		Ok(Self {
			ident: b64u_decode(ident_b64u)
				.map_err(|_| Error::TokenCannotDecodeIdent)?,
			exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
			sign_b64u: sign_b64u.to_string(),
		})
	}
}

// Display: e.g. let token_str = token.to_string() where token is a Token
//          or println!("{token}"); without implementing Debug trait and using {:?}
impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}.{}.{}",
			b64u_encode(&self.ident),
			b64u_encode(&self.exp),
			self.sign_b64u
		)
	}
}

// endregion: --- Token Type

// region:    --- Web Token Gen and Validation

// endregion: --- Web Token Gen and Validation
pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
	let config = config();
	_generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(original_token: &Token, salt: &str) -> Result<()> {
	let config = config();
	_validate_token_sign_and_exp(original_token, salt, &config.TOKEN_KEY)
}
// region:    --- (private) Token Gen and Validation
fn _generate_token(
	ident: &str,
	duration_sec: f64,
	salt: &str,
	key: &[u8],
) -> Result<Token> {
	// -- Compute the first two components
	let ident = ident.to_string();
	let exp = now_utc_plus_sec_str(duration_sec);

	// -- Sign the two first components
	let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

	Ok(Token {
		ident,
		exp,
		sign_b64u,
	})
}

fn _validate_token_sign_and_exp(
	original_token: &Token,
	salt: &str,
	key: &[u8],
) -> Result<()> {
	// -- Validate signature
	let Token {
		ident,
		exp,
		sign_b64u,
	} = original_token;

	let new_sign_b64u = _token_sign_into_b64u(ident, exp, salt, key)?;

	if &new_sign_b64u != sign_b64u {
		return Err(Error::TokenSignatureNotMatching);
	}

	let origin_exp = parse_utc(exp).map_err(|_| Error::TokenExpNotIso)?;
	let now = now_utc();
	if origin_exp < now {
		return Err(Error::TokenExpired);
	}

	Ok(())
}
/// Create token signature from token parts and salt
fn _token_sign_into_b64u(
	ident: &str,
	exp: &str,
	salt: &str,
	key: &[u8],
) -> Result<String> {
	let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
	let signature = encrypt_into_b64u(
		key,
		&super::EncryptContent {
			content,
			salt: salt.to_string(),
		},
	)?;

	Ok(signature)
}

// endregion: --- (private) Token Gen and Validation

// region:    --- Tests

#[cfg(test)]
mod tests {
	use std::{thread, time::Duration};

	use super::*;
	use anyhow::Result;

	#[test]
	fn test_token_display() -> Result<()> {
		let fx_token = Token {
			ident: "fx-ident-01".to_string(),
			exp: "2024-05-16T16:30:00Z".to_string(),
			sign_b64u: "some-sign-b64-encoded".to_string(),
		};

		let fx_token_str =
			"ZngtaWRlbnQtMDE.MjAyNC0wNS0xNlQxNjozMDowMFo.some-sign-b64-encoded";

		// println!("->> {fx_token}");
		// Exec and check
		assert_eq!(fx_token.to_string(), fx_token_str);

		Ok(())
	}

	#[test]
	fn test_token_fromstr_ok() -> Result<()> {
		let fx_token = Token {
			ident: "fx-ident-01".to_string(),
			exp: "2024-05-16T16:30:00Z".to_string(),
			sign_b64u: "some-sign-b64-encoded".to_string(),
		};

		let fx_token_str =
			"ZngtaWRlbnQtMDE.MjAyNC0wNS0xNlQxNjozMDowMFo.some-sign-b64-encoded";

		// Exec
		let token: Token = fx_token_str.parse()?;

		// Check
		assert_eq!(format!("{token:?}"), format!("{fx_token:?}"));
		Ok(())
	}

	#[test]
	fn test_validate_web_token_ok() -> Result<()> {
		// -- Setup and Fixtures
		let fx_user = "user_one";
		let salt = "pepper";
		let fx_duration_sec = 0.02; //20 ms
		let token_key = &config().TOKEN_KEY;
		let fx_token = _generate_token(fx_user, fx_duration_sec, salt, token_key)?;

		// -- Exec
		thread::sleep(Duration::from_millis(10));
		let res = validate_web_token(&fx_token, salt, token_key);

		// -- Check
		res?;

		Ok(())
	}

	#[test]
	fn test_validate_web_token_expired() -> Result<()> {
		// -- Setup and Fixtures
		let fx_user = "user_one";
		let salt = "pepper";
		let fx_duration_sec = 0.02; //20 ms
		let token_key = &config().TOKEN_KEY;
		let fx_token = _generate_token(fx_user, fx_duration_sec, salt, token_key)?;

		// -- Exec
		// Sleep as long as fx_duration_sec or longer to force token to expire
		thread::sleep(Duration::from_millis(20));
		let res = validate_web_token(&fx_token, salt, token_key);

		// -- Check
		assert!(
			matches!(res, Err(Error::TokenExpired)),
			"Expect `Err(Error::TokenExpired)` but was `{res:?}`"
		);

		Ok(())
	}
}

// endregion: --- Tests
