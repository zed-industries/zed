// use super::{
//     db::{self, UserId},
//     errors::TideResultExt,
// };
// use crate::Request;
// use anyhow::{anyhow, Context};
// use rand::thread_rng;
// use rpc::auth as zed_auth;
// use scrypt::{
//     password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
//     Scrypt,
// };
// use std::convert::TryFrom;
// use surf::StatusCode;
// use tide::Error;

// pub async fn process_auth_header(request: &Request) -> tide::Result<UserId> {
//     let mut auth_header = request
//         .header("Authorization")
//         .ok_or_else(|| {
//             Error::new(
//                 StatusCode::BadRequest,
//                 anyhow!("missing authorization header"),
//             )
//         })?
//         .last()
//         .as_str()
//         .split_whitespace();
//     let user_id = UserId(auth_header.next().unwrap_or("").parse().map_err(|_| {
//         Error::new(
//             StatusCode::BadRequest,
//             anyhow!("missing user id in authorization header"),
//         )
//     })?);
//     let access_token = auth_header.next().ok_or_else(|| {
//         Error::new(
//             StatusCode::BadRequest,
//             anyhow!("missing access token in authorization header"),
//         )
//     })?;

//     let state = request.state().clone();
//     let mut credentials_valid = false;
//     for password_hash in state.db.get_access_token_hashes(user_id).await? {
//         if verify_access_token(&access_token, &password_hash)? {
//             credentials_valid = true;
//             break;
//         }
//     }

//     if !credentials_valid {
//         Err(Error::new(
//             StatusCode::Unauthorized,
//             anyhow!("invalid credentials"),
//         ))?;
//     }

//     Ok(user_id)
// }

// const MAX_ACCESS_TOKENS_TO_STORE: usize = 8;

// pub async fn create_access_token(db: &dyn db::Db, user_id: UserId) -> tide::Result<String> {
//     let access_token = zed_auth::random_token();
//     let access_token_hash =
//         hash_access_token(&access_token).context("failed to hash access token")?;
//     db.create_access_token_hash(user_id, &access_token_hash, MAX_ACCESS_TOKENS_TO_STORE)
//         .await?;
//     Ok(access_token)
// }

// fn hash_access_token(token: &str) -> tide::Result<String> {
//     // Avoid slow hashing in debug mode.
//     let params = if cfg!(debug_assertions) {
//         scrypt::Params::new(1, 1, 1).unwrap()
//     } else {
//         scrypt::Params::recommended()
//     };

//     Ok(Scrypt
//         .hash_password(
//             token.as_bytes(),
//             None,
//             params,
//             &SaltString::generate(thread_rng()),
//         )?
//         .to_string())
// }

// pub fn encrypt_access_token(access_token: &str, public_key: String) -> tide::Result<String> {
//     let native_app_public_key =
//         zed_auth::PublicKey::try_from(public_key).context("failed to parse app public key")?;
//     let encrypted_access_token = native_app_public_key
//         .encrypt_string(&access_token)
//         .context("failed to encrypt access token with public key")?;
//     Ok(encrypted_access_token)
// }

// pub fn verify_access_token(token: &str, hash: &str) -> tide::Result<bool> {
//     let hash = PasswordHash::new(hash)?;
//     Ok(Scrypt.verify_password(token.as_bytes(), &hash).is_ok())
// }
