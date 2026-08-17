//! Authentication related types and functions.

use crate::models::AppId;
use crate::Result;
use either::Either;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::fmt;
#[cfg(feature = "tokio")]
use web_time::Duration;
use web_time::SystemTime;

use snafu::*;

/// The data necessary to authenticate as a Github App
#[derive(Clone)]
pub struct AppAuth {
    /// Github's app ID
    pub app_id: AppId,
    /// The app's RSA private key
    pub key: EncodingKey,
}

impl fmt::Debug for AppAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppAuth")
            .field("app_id", &self.app_id)
            .finish_non_exhaustive()
    }
}

/// The forms of authentication we support
pub enum Auth {
    /// No authentication
    None,
    // Basic HTTP authentication (username:password)
    Basic {
        /// Username
        username: String,
        /// Password
        password: String,
    },
    /// Authenticate using a Github personal access token
    PersonalToken(SecretString),
    /// Authenticate as a Github App
    App(AppAuth),
    /// Authenticate as a Github OAuth App
    OAuth(OAuth),
    /// Authenticate using a User Access Token
    UserAccessToken(SecretString),
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}

/// Create a JSON Web Token that can be used to authenticate an a GitHub application.
///
/// See: <https://docs.github.com/en/developers/apps/getting-started-with-apps/setting-up-your-development-environment-to-create-a-github-app#authenticating-as-a-github-app>
pub fn create_jwt(
    github_app_id: AppId,
    key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error> {
    #[derive(Serialize)]
    struct Claims {
        iss: AppId,
        iat: usize,
        exp: usize,
    }

    let now = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs() as usize;

    // Github only allows JWTs that expire in the next 10 minutes.
    // The token is issued 60 seconds in the past and expires in 9 minutes,
    // to allow some clock drift.
    let claims = Claims {
        iss: github_app_id,
        iat: now - 60,
        exp: now + (9 * 60),
    };

    let header = Header::new(Algorithm::RS256);

    jsonwebtoken::encode(&header, &claims, key)
}

impl AppAuth {
    /// Currently we don't cache these, but we could if we want to avoid
    /// an RSA signature operation per App-authorized API call.
    pub fn generate_bearer_token(&self) -> Result<String> {
        create_jwt(self.app_id, &self.key).context(crate::error::JWTSnafu)
    }
}

/// The data necessary to authenticate as a GitHub OAuth app.
#[derive(Clone, Deserialize)]
#[serde(from = "OAuthWire")]
pub struct OAuth {
    pub access_token: SecretString,
    pub token_type: String,
    pub scope: Vec<String>,
    pub expires_in: Option<usize>,
    pub refresh_token: Option<SecretString>,
    pub refresh_token_expires_in: Option<usize>,
}

/// The wire format of the OAuth struct.
#[derive(Deserialize)]
struct OAuthWire {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: Option<usize>,
    refresh_token: Option<String>,
    refresh_token_expires_in: Option<usize>,
}

impl From<OAuthWire> for OAuth {
    fn from(value: OAuthWire) -> Self {
        OAuth {
            access_token: SecretString::from(value.access_token),
            token_type: value.token_type,
            scope: value.scope.split(',').map(ToString::to_string).collect(),
            expires_in: value.expires_in,
            refresh_token: value.refresh_token.map(SecretString::from),
            refresh_token_expires_in: value.refresh_token_expires_in,
        }
    }
}

impl crate::Octocrab {
    /// Authenticate with Github's device flow. This starts the process to obtain a new `OAuth`.
    ///
    /// See <https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#device-flow> for details.
    ///
    /// Note: To authenticate against public Github, the `Octocrab` that calls this method
    /// *must* be constructed with `base_uri: "https://github.com"` and extra header
    /// "ACCEPT: application/json". For example:
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # use http::header::ACCEPT;
    /// let crab = octocrab::Octocrab::builder()
    /// .base_uri("https://github.com")?
    /// .add_header(ACCEPT, "application/json".to_string())
    /// .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn authenticate_as_device<I, S>(
        &self,
        client_id: &SecretString,
        scope: I,
    ) -> Result<DeviceCodes>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let scope = {
            let mut scopes = scope.into_iter();
            let first = scopes
                .next()
                .map(|s| s.as_ref().to_string())
                .unwrap_or_default();
            scopes.fold(first, |i: String, n| i + "," + n.as_ref())
        };
        let codes: DeviceCodes = self
            .post(
                "/login/device/code",
                Some(&DeviceFlow {
                    client_id: client_id.expose_secret(),
                    scope: &scope,
                }),
            )
            .await?;
        Ok(codes)
    }
}

/// The device codes as returned from step 1 of Github's device flow.
///
/// See <https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#response-parameters>
#[derive(Deserialize, Clone)]
#[non_exhaustive]
pub struct DeviceCodes {
    /// The device verification code is 40 characters and used to verify the device.
    pub device_code: String,
    /// The user verification code is displayed on the device so the user can enter the
    /// code in a browser. This code is 8 characters with a hyphen in the middle.
    pub user_code: String,
    /// The verification URL where users need to enter the user_code: <https://github.com/login/device>
    pub verification_uri: String,
    /// The number of seconds before the device_code and user_code expire. The default is
    /// 900 seconds or 15 minutes.
    pub expires_in: u64,
    /// The minimum number of seconds that must pass before you can make a new access
    /// token request (POST <https://github.com/login/oauth/access_token>) to complete the
    /// device authorization. For example, if the interval is 5, then you cannot make a
    /// new request until 5 seconds pass. If you make more than one request over 5
    /// seconds, then you will hit the rate limit and receive a slow_down error.
    pub interval: u64,
}

impl DeviceCodes {
    /// Poll Github to see if authentication codes are available.
    ///
    /// See `https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps` for details.
    pub async fn poll_once(
        &self,
        crab: &crate::Octocrab,
        client_id: &SecretString,
    ) -> Result<Either<OAuth, Continue>> {
        let poll: TokenResponse = crab
            .post(
                "/login/oauth/access_token",
                Some(&PollForDevice {
                    client_id: client_id.expose_secret(),
                    device_code: &self.device_code,
                    grant_type: "urn:ietf:params:oauth:grant-type:device_code",
                }),
            )
            .await?;
        Ok(match poll {
            TokenResponse::Ok(k) => Either::Left(k),
            TokenResponse::Continue { error } => Either::Right(error),
        })
    }

    /// Poll Github in a loop until authentication codes become available.
    #[cfg(feature = "tokio")]
    pub async fn poll_until_available(
        &self,
        crab: &crate::Octocrab,
        client_id: &SecretString,
    ) -> Result<OAuth> {
        let mut interval = Duration::from_secs(self.interval);
        let mut clock = tokio::time::interval(interval);

        loop {
            clock.tick().await;
            match self.poll_once(crab, client_id).await? {
                Either::Left(auth) => return Ok(auth),
                Either::Right(cont) => match cont {
                    Continue::SlowDown => {
                        // We were requested to slow down, so add five seconds to the polling
                        // duration.
                        interval += Duration::from_secs(5);
                        clock = tokio::time::interval(interval);
                        // The first tick happens instantly, so we tick that off immediately.
                        clock.tick().await;
                    }
                    Continue::AuthorizationPending => {
                        // The user has not clicked authorize yet, so we keep polling as normal.
                    }
                },
            }
        }
    }
}

/// See https://docs.github.com/en/developers/apps/building-oauth-apps/authorizing-oauth-apps#input-parameters
#[derive(Serialize)]
struct DeviceFlow<'a> {
    client_id: &'a str,
    scope: &'a str,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TokenResponse {
    // We got the auth information.
    Ok(OAuth),
    // We got an error that allows us to continue polling.
    Continue { error: Continue },
}

/// Control flow when polling the device flow authorization.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Continue {
    /// When you receive the slow_down error, 5 extra seconds are added to the minimum
    /// interval or timeframe required between your requests using POST
    /// <https://github.com/login/oauth/access_token>. For example, if the starting interval
    /// required at least 5 seconds between requests and you get a slow_down error response,
    /// you must now wait a minimum of 10 seconds before making a new request for an OAuth
    /// access token. The error response includes the new interval that you must use.
    SlowDown,
    /// This error occurs when the authorization request is pending and the user hasn't
    /// entered the user code yet. The app is expected to keep polling the POST
    /// <https://github.com/login/oauth/access_token> request without exceeding the
    /// interval, which requires a minimum number of seconds between each request.
    AuthorizationPending,
}

#[derive(Serialize)]
struct PollForDevice<'a> {
    /// Required. The client ID you received from GitHub for your OAuth App.
    client_id: &'a str,
    /// Required. The device verification code you received from the POST <https://github.com/login/device/code> request.
    device_code: &'a str,
    /// Required. The grant type must be urn:ietf:params:oauth:grant-type:device_code.
    grant_type: &'static str,
}
