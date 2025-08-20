use anyhow::{Result, anyhow};
use base64::Engine;
use futures::{AsyncReadExt, FutureExt, channel::oneshot};
use gpui::App;
use http_client::{HttpClient, Method, Request as HttpRequest};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use url::Url;

pub struct OracleOAuthClient;

#[derive(Debug)]
pub struct OAuthSession {
    pub redirect_url: Url,
    pub code_verifier: String,
    pub code_rx: oneshot::Receiver<String>,
}

struct Pkce;

impl Pkce {
    pub fn code_verifier() -> String {
        let mut rng = rand::thread_rng();
        let mut verifier_bytes = [0u8; 32];
        rng.fill(&mut verifier_bytes);
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(verifier_bytes)
    }

    pub fn code_challenge(verifier: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge_bytes = hasher.finalize();
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(challenge_bytes)
    }
}

impl OracleOAuthClient {
    const CLIENT_ID: &'static str = "a8331954c0cf48ba99b5dd223a14c6ea";
    const OAUTH_DOMAIN: &'static str =
        "https://idcs-9dc693e80d9b469480d7afe00e743931.identity.oraclecloud.com";
    const PORT_CANDIDATES: [u16; 3] = [8669, 8668, 8667];
    const SCOPES: [&'static str; 2] = ["openid", "offline_access"];
    const TIMEOUT: Duration = Duration::from_secs(120);

    fn build_authorize_url(pkce_challenge: &str, redirect_uri: &str) -> Result<Url> {
        let mut url = Url::parse(&format!("{}/oauth2/v1/authorize", Self::OAUTH_DOMAIN))?;
        url.query_pairs_mut()
            .append_pair("client_id", Self::CLIENT_ID)
            .append_pair("scope", &Self::SCOPES.join(" "))
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("code_challenge", pkce_challenge)
            .append_pair("code_challenge_method", "S256");
        Ok(url)
    }

    pub fn initiate_oauth(cx: &mut App) -> Result<OAuthSession> {
        let (redirect_url, code_rx) = Self::start_redirect_server()?;
        let redirect_url = redirect_url
            .parse::<Url>()
            .map_err(|e| anyhow!("Failed to parse redirect URL: {}", e))?;
        let code_verifier = Pkce::code_verifier();
        let code_challenge = Pkce::code_challenge(&code_verifier);
        let auth_url = Self::build_authorize_url(&code_challenge, redirect_url.as_str())?;
        cx.open_url(auth_url.as_str());

        Ok(OAuthSession {
            redirect_url,
            code_verifier,
            code_rx,
        })
    }

    pub async fn authenticate(
        client: Arc<dyn HttpClient>,
        session: OAuthSession,
    ) -> Result<OAuthToken> {
        let (timeout_tx, timeout_rx) = oneshot::channel::<()>();
        thread::spawn(move || {
            thread::sleep(Self::TIMEOUT);
            let _ = timeout_tx.send(());
        });

        let code = futures::select! {
            code = session.code_rx.fuse() => {
                code.map_err(|_| anyhow!("redirect server closed before code received"))
            }
            _ = timeout_rx.fuse() => {
                Err(anyhow!("authentication timeout"))
            }
        }?;

        Self::exchange_code_for_tokens(
            client,
            &code,
            &session.code_verifier,
            session.redirect_url.as_str(),
        )
        .await
    }

    fn start_redirect_server() -> Result<(String, oneshot::Receiver<String>)> {
        let mut last_err: Option<anyhow::Error> = None;

        for port in Self::PORT_CANDIDATES {
            let server = match tiny_http::Server::http(("127.0.0.1", port)) {
                Ok(s) => s,
                Err(e) => {
                    last_err = Some(anyhow!(e));
                    continue;
                }
            };

            let redirect_url = format!("http://localhost:{}/callback", port);
            let (code_tx, code_rx) = oneshot::channel::<String>();

            thread::spawn(move || {
                for request in server.incoming_requests() {
                    let url = request.url();

                    let (path, query_opt) = match url.split_once('?') {
                        Some((p, q)) => (p, Some(q)),
                        None => (url, None),
                    };

                    if path != "/callback" {
                        let _ = request.respond(
                            tiny_http::Response::from_string("Invalid callback")
                                .with_status_code(400),
                        );
                        continue;
                    }

                    let Some(q) = query_opt else {
                        let _ = request.respond(
                            tiny_http::Response::from_string("Invalid callback")
                                .with_status_code(400),
                        );
                        break;
                    };

                    let params: std::collections::HashMap<_, _> =
                        url::form_urlencoded::parse(q.as_bytes())
                            .into_owned()
                            .collect();

                    let Some(code) = params.get("code") else {
                        let _ = request.respond(
                            tiny_http::Response::from_string("Invalid callback")
                                .with_status_code(400),
                        );
                        break;
                    };

                    if code_tx.send(code.clone()).is_ok() {
                        let _ = request.respond(
                            tiny_http::Response::from_string(
                                "Authorization successful. You can close this window.",
                            )
                            .with_status_code(200),
                        );
                    } else {
                        let _ = request.respond(
                            tiny_http::Response::from_string("Invalid callback")
                                .with_status_code(400),
                        );
                    }

                    break;
                }
            });

            return Ok((redirect_url, code_rx));
        }

        Err(last_err.unwrap_or_else(|| anyhow!("no available redirect port")))
    }

    async fn exchange_code_for_tokens(
        client: Arc<dyn HttpClient>,
        code: &str,
        verifier: &str,
        redirect_uri: &str,
    ) -> Result<OAuthToken> {
        let form_data = format!(
            "grant_type=authorization_code&client_id={}&code={}&redirect_uri={}&code_verifier={}",
            Self::CLIENT_ID,
            urlencoding::encode(code),
            urlencoding::encode(redirect_uri),
            urlencoding::encode(verifier)
        );

        let request = HttpRequest::builder()
            .method(Method::POST)
            .uri(format!("{}/oauth2/v1/token", Self::OAUTH_DOMAIN))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data.into())?;

        let response = client.send(request).await?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "OAuth token exchange failed with status: {}",
                response.status()
            ));
        }

        let mut body = String::new();
        response.into_body().read_to_string(&mut body).await?;
        let parsed: OAuthResponse = serde_json::from_str(&body)?;

        Ok(OAuthToken::new(
            parsed.refresh_token,
            parsed.access_token,
            parsed.expires_in_secs,
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OAuthToken {
    pub refresh_token: String,
    pub access_token: String,
    #[serde(skip, default = "Instant::now")]
    pub expires_at: Instant,
}

impl OAuthToken {
    const RENEW_BUFFER: Duration = Duration::from_secs(180);

    pub fn new(
        refresh_token: impl Into<String>,
        access_token: impl Into<String>,
        expires_in: u64,
    ) -> Self {
        Self {
            refresh_token: refresh_token.into(),
            access_token: access_token.into(),
            expires_at: Instant::now() + Duration::from_secs(expires_in),
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() + Self::RENEW_BUFFER >= self.expires_at
    }

    pub async fn refresh(self, client: Arc<dyn HttpClient>) -> Result<Self> {
        let form_data = format!(
            "grant_type=refresh_token&refresh_token={}&client_id={}",
            urlencoding::encode(&self.refresh_token),
            OracleOAuthClient::CLIENT_ID
        );

        let request = HttpRequest::builder()
            .method(Method::POST)
            .uri(&format!(
                "{}/oauth2/v1/token",
                OracleOAuthClient::OAUTH_DOMAIN
            ))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data.into())?;

        let response = client.send(request).await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "OAuth token refresh failed with status: {}",
                response.status()
            ));
        }

        let mut body = String::new();
        response.into_body().read_to_string(&mut body).await?;

        let response: OAuthResponse = serde_json::from_str(&body)?;

        Ok(OAuthToken::new(
            response.refresh_token,
            response.access_token,
            response.expires_in_secs,
        ))
    }
}

#[derive(Debug, Deserialize)]
struct OAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in_secs: u64,
}
