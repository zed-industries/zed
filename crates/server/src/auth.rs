use super::{
    db::{self, UserId},
    errors::TideResultExt,
};
use crate::{github, AppState, Request, RequestExt as _};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
pub use oauth2::basic::BasicClient as Client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl,
    TokenResponse as _, TokenUrl,
};
use rand::thread_rng;
use rpc::auth as zed_auth;
use scrypt::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, convert::TryFrom, sync::Arc};
use surf::{StatusCode, Url};
use tide::{log, Error, Server};

static CURRENT_GITHUB_USER: &'static str = "current_github_user";
static GITHUB_AUTH_URL: &'static str = "https://github.com/login/oauth/authorize";
static GITHUB_TOKEN_URL: &'static str = "https://github.com/login/oauth/access_token";

#[derive(Serialize)]
pub struct User {
    pub github_login: String,
    pub avatar_url: String,
    pub is_insider: bool,
    pub is_admin: bool,
}

pub async fn process_auth_header(request: &Request) -> tide::Result<UserId> {
    let mut auth_header = request
        .header("Authorization")
        .ok_or_else(|| {
            Error::new(
                StatusCode::BadRequest,
                anyhow!("missing authorization header"),
            )
        })?
        .last()
        .as_str()
        .split_whitespace();
    let user_id = UserId(auth_header.next().unwrap_or("").parse().map_err(|_| {
        Error::new(
            StatusCode::BadRequest,
            anyhow!("missing user id in authorization header"),
        )
    })?);
    let access_token = auth_header.next().ok_or_else(|| {
        Error::new(
            StatusCode::BadRequest,
            anyhow!("missing access token in authorization header"),
        )
    })?;

    let state = request.state().clone();
    let mut credentials_valid = false;
    for password_hash in state.db.get_access_token_hashes(user_id).await? {
        if verify_access_token(&access_token, &password_hash)? {
            credentials_valid = true;
            break;
        }
    }

    if !credentials_valid {
        Err(Error::new(
            StatusCode::Unauthorized,
            anyhow!("invalid credentials"),
        ))?;
    }

    Ok(user_id)
}

#[async_trait]
pub trait RequestExt {
    async fn current_user(&self) -> tide::Result<Option<User>>;
}

#[async_trait]
impl RequestExt for Request {
    async fn current_user(&self) -> tide::Result<Option<User>> {
        if let Some(details) = self.session().get::<github::User>(CURRENT_GITHUB_USER) {
            let user = self.db().get_user_by_github_login(&details.login).await?;
            Ok(Some(User {
                github_login: details.login,
                avatar_url: details.avatar_url,
                is_insider: user.is_some(),
                is_admin: user.map_or(false, |user| user.admin),
            }))
        } else {
            Ok(None)
        }
    }
}

pub fn build_client(client_id: &str, client_secret: &str) -> Client {
    Client::new(
        ClientId::new(client_id.to_string()),
        Some(oauth2::ClientSecret::new(client_secret.to_string())),
        AuthUrl::new(GITHUB_AUTH_URL.into()).unwrap(),
        Some(TokenUrl::new(GITHUB_TOKEN_URL.into()).unwrap()),
    )
}

pub fn add_routes(app: &mut Server<Arc<AppState>>) {
    app.at("/sign_in").get(get_sign_in);
    app.at("/sign_out").post(post_sign_out);
    app.at("/auth_callback").get(get_auth_callback);
    app.at("/native_app_signin").get(get_sign_in);
    app.at("/native_app_signin_succeeded")
        .get(get_app_signin_success);
}

#[derive(Debug, Deserialize)]
struct NativeAppSignInParams {
    native_app_port: String,
    native_app_public_key: String,
    impersonate: Option<String>,
}

async fn get_sign_in(mut request: Request) -> tide::Result {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    request
        .session_mut()
        .insert("pkce_verifier", pkce_verifier)?;

    let mut redirect_url = Url::parse(&format!(
        "{}://{}/auth_callback",
        request
            .header("X-Forwarded-Proto")
            .and_then(|values| values.get(0))
            .map(|value| value.as_str())
            .unwrap_or("http"),
        request.host().unwrap()
    ))?;

    let app_sign_in_params: Option<NativeAppSignInParams> = request.query().ok();
    if let Some(query) = app_sign_in_params {
        let mut redirect_query = redirect_url.query_pairs_mut();
        redirect_query
            .clear()
            .append_pair("native_app_port", &query.native_app_port)
            .append_pair("native_app_public_key", &query.native_app_public_key);

        if let Some(impersonate) = &query.impersonate {
            redirect_query.append_pair("impersonate", impersonate);
        }
    }

    let (auth_url, csrf_token) = request
        .state()
        .auth_client
        .authorize_url(CsrfToken::new_random)
        .set_redirect_uri(Cow::Owned(RedirectUrl::from_url(redirect_url)))
        .set_pkce_challenge(pkce_challenge)
        .url();

    request
        .session_mut()
        .insert("auth_csrf_token", csrf_token)?;

    Ok(tide::Redirect::new(auth_url).into())
}

async fn get_app_signin_success(_: Request) -> tide::Result {
    Ok(tide::Redirect::new("/").into())
}

async fn get_auth_callback(mut request: Request) -> tide::Result {
    #[derive(Debug, Deserialize)]
    struct Query {
        code: String,
        state: String,

        #[serde(flatten)]
        native_app_sign_in_params: Option<NativeAppSignInParams>,
    }

    let query: Query = request.query()?;

    let pkce_verifier = request
        .session()
        .get("pkce_verifier")
        .ok_or_else(|| anyhow!("could not retrieve pkce_verifier from session"))?;

    let csrf_token = request
        .session()
        .get::<CsrfToken>("auth_csrf_token")
        .ok_or_else(|| anyhow!("could not retrieve auth_csrf_token from session"))?;

    if &query.state != csrf_token.secret() {
        return Err(anyhow!("csrf token does not match").into());
    }

    let github_access_token = request
        .state()
        .auth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2_surf::http_client)
        .await
        .context("failed to exchange oauth code")?
        .access_token()
        .secret()
        .clone();

    let user_details = request
        .state()
        .github_client
        .user(github_access_token)
        .details()
        .await
        .context("failed to fetch user")?;

    let user = request
        .db()
        .get_user_by_github_login(&user_details.login)
        .await?;

    request
        .session_mut()
        .insert(CURRENT_GITHUB_USER, user_details.clone())?;

    // When signing in from the native app, generate a new access token for the current user. Return
    // a redirect so that the user's browser sends this access token to the locally-running app.
    if let Some((user, app_sign_in_params)) = user.zip(query.native_app_sign_in_params) {
        let mut user_id = user.id;
        if let Some(impersonated_login) = app_sign_in_params.impersonate {
            log::info!("attempting to impersonate user @{}", impersonated_login);
            if let Some(user) = request.db().get_users_by_ids(vec![user_id]).await?.first() {
                if user.admin {
                    user_id = request.db().create_user(&impersonated_login, false).await?;
                    log::info!("impersonating user {}", user_id.0);
                } else {
                    log::info!("refusing to impersonate user");
                }
            }
        }

        let access_token = create_access_token(request.db().as_ref(), user_id).await?;
        let encrypted_access_token = encrypt_access_token(
            &access_token,
            app_sign_in_params.native_app_public_key.clone(),
        )?;

        return Ok(tide::Redirect::new(&format!(
            "http://127.0.0.1:{}?user_id={}&access_token={}",
            app_sign_in_params.native_app_port, user_id.0, encrypted_access_token,
        ))
        .into());
    }

    Ok(tide::Redirect::new("/").into())
}

async fn post_sign_out(mut request: Request) -> tide::Result {
    request.session_mut().remove(CURRENT_GITHUB_USER);
    Ok(tide::Redirect::new("/").into())
}

const MAX_ACCESS_TOKENS_TO_STORE: usize = 8;

pub async fn create_access_token(db: &dyn db::Db, user_id: UserId) -> tide::Result<String> {
    let access_token = zed_auth::random_token();
    let access_token_hash =
        hash_access_token(&access_token).context("failed to hash access token")?;
    db.create_access_token_hash(user_id, &access_token_hash, MAX_ACCESS_TOKENS_TO_STORE)
        .await?;
    Ok(access_token)
}

fn hash_access_token(token: &str) -> tide::Result<String> {
    // Avoid slow hashing in debug mode.
    let params = if cfg!(debug_assertions) {
        scrypt::Params::new(1, 1, 1).unwrap()
    } else {
        scrypt::Params::recommended()
    };

    Ok(Scrypt
        .hash_password(
            token.as_bytes(),
            None,
            params,
            &SaltString::generate(thread_rng()),
        )?
        .to_string())
}

pub fn encrypt_access_token(access_token: &str, public_key: String) -> tide::Result<String> {
    let native_app_public_key =
        zed_auth::PublicKey::try_from(public_key).context("failed to parse app public key")?;
    let encrypted_access_token = native_app_public_key
        .encrypt_string(&access_token)
        .context("failed to encrypt access token with public key")?;
    Ok(encrypted_access_token)
}

pub fn verify_access_token(token: &str, hash: &str) -> tide::Result<bool> {
    let hash = PasswordHash::new(hash)?;
    Ok(Scrypt.verify_password(token.as_bytes(), &hash).is_ok())
}
