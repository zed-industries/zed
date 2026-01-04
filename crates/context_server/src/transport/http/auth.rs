use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result};
use base64::Engine as _;
use http_client::{AsyncBody, HttpClient, Request, Response, Uri};
use rand::distr::Distribution;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use sha2::{Digest, Sha256};
use smol::io::AsyncReadExt;
use thiserror::Error;
use url::Url;

use crate::{ContextServerId, transport::http::www_authenticate::WwwAuthenticate};
use abs_uri::AbsUri;

pub struct OAuthClient {
    registration: ClientRegistration,
    server: AuthorizationServer,
    scope: Option<String>,
    state: State,
    http_client: Arc<dyn HttpClient>,
}

enum State {
    Init,
    WaitingForCode {
        code_verifier: String,
    },
    Authenticated {
        access_token: String,
        token_type: String,
        expires_at: Option<Instant>,
        refresh_token: Option<String>,
    },
}

impl Default for State {
    fn default() -> Self {
        State::Init
    }
}

impl OAuthClient {
    pub async fn init(
        server_endpoint: &str,
        www_authenticate: Option<&WwwAuthenticate<'_>>,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self> {
        // https://modelcontextprotocol.io/specification/draft/basic/authorization#authorization-server-discovery
        // https://modelcontextprotocol.io/specification/draft/basic/authorization#protected-resource-metadata-discovery-requirements
        let resource =
            match www_authenticate.and_then(|challenge| challenge.resource_metadata.as_ref()) {
                Some(url) => ProtectedResource::fetch(url, http_client).await?,
                None => ProtectedResource::fetch_well_known(server_endpoint, http_client).await?,
            };

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#authorization-server-metadata-discovery
        let auth_server_url = resource
            .authorization_servers
            // todo! try others?
            .first()
            .ok_or(InitError::NoAuthorizationServers)?;

        let server = AuthorizationServer::fetch(auth_server_url, http_client).await?;

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#client-registration-approaches
        // TODO: Pre-registration from settings?
        let registration = if server.client_id_metadata_document_supported {
            todo!("host client id meta doc somewhere");
        } else if let Some(registration_endpoint) = server.registration_endpoint.as_ref() {
            Self::register(registration_endpoint, http_client).await?
        } else {
            todo!("allow user to specify custom client meta");
        };

        // https://modelcontextprotocol.io/specification/draft/basic/authorization#scope-selection-strategy
        let scope = www_authenticate
            .and_then(|challenge| challenge.scope.as_ref().map(|s| s.to_string()))
            .or_else(|| {
                if resource.scopes_supported.is_empty() {
                    None
                } else {
                    Some(resource.scopes_supported.join(" "))
                }
            });

        Ok(Self {
            registration,
            server,
            scope,
            state: State::Init,
            http_client: http_client.clone(),
        })
    }

    pub fn authorize_url(&mut self) -> Result<AuthorizeUrl> {
        let auth_endpoint = self
            .server
            .authorization_endpoint
            .as_ref()
            .ok_or(AuthorizeUrlError::MissingAuthorizationEndpoint)?;

        let code_verifier = generate_code_verifier();
        let code_challenge =
            base64::engine::general_purpose::URL_SAFE.encode(Sha256::digest(&code_verifier));

        let mut url = Url::parse(&auth_endpoint.to_string())?;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.registration.client_id)
            .append_pair("redirect_uri", OAuthCallback::URI)
            .append_pair("code_challenge", &code_challenge)
            .append_pair("code_challenge_method", "S256")
            .extend_pairs(self.scope.iter().map(|value| ("scope", value)));

        self.state = State::WaitingForCode { code_verifier };

        anyhow::Ok(AuthorizeUrl { url })
    }

    pub async fn exchange_token(&mut self, code: &str) -> Result<()> {
        let State::WaitingForCode { code_verifier } = &self.state else {
            return Err(ExchangeTokenError::NotWaitingForAuthorizationCode.into());
        };

        let token_endpoint = self
            .server
            .token_endpoint
            .as_ref()
            // todo! implicit?
            .ok_or(ExchangeTokenError::MissingTokenEndpoint)?;

        let form = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "authorization_code")
            .append_pair("code", code)
            .append_pair("redirect_uri", OAuthCallback::URI)
            .append_pair("client_id", &self.registration.client_id)
            .append_pair("code_verifier", code_verifier)
            .finish();

        let request = Request::builder()
            .uri(token_endpoint.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(AsyncBody::from(form))
            .context(ExchangeTokenError::BuildTokenExchangeRequest)?;

        let requested_at = Instant::now();

        let mut response = self.http_client.send(request).await?;
        let token_response: TokenResponse = decode_response_json(&mut response).await?;

        self.state = State::Authenticated {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_at: token_response
                .expires_in
                .map(|expires_in| requested_at + Duration::from_secs(expires_in)),
            refresh_token: token_response.refresh_token,
        };

        anyhow::Ok(())
    }

    async fn register(
        registration_endpoint: &AbsUri,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<ClientRegistration> {
        let metadata = json!({
            "redirect_uris": [OAuthCallback::URI],
            "token_endpoint_auth_method": "none",
            "grant_types": ["authorization_code", "refresh_token"],
            "response_types": ["code"],
            "client_name": "Zed",
            "client_uri": "https://zed.dev",
            "logo_uri": "https://zed.dev/_next/static/media/stable-app-logo.9b5f959f.png"
        });

        post_json(&registration_endpoint.to_string(), metadata, http_client).await
    }

    pub async fn access_token(&mut self) -> Result<Option<&str>> {
        let State::Authenticated {
            expires_at,
            refresh_token,
            ..
        } = &self.state
        else {
            return Ok(None);
        };

        if expires_at.is_some_and(|expires_at| expires_at <= Instant::now()) {
            if refresh_token.is_none() {
                return Err(AccessTokenError::AccessTokenExpiredNoRefreshToken.into());
            }

            self.refresh_access_token().await?;
        }

        let State::Authenticated { access_token, .. } = &self.state else {
            return Ok(None);
        };

        Ok(Some(access_token.as_str()))
    }

    async fn refresh_access_token(&mut self) -> Result<()> {
        if matches!(self.state, State::WaitingForCode { .. }) {
            return Err(RefreshTokenError::WaitingForAuthorizationCode.into());
        }

        let State::Authenticated {
            refresh_token: previous_refresh_token,
            token_type: previous_token_type,
            ..
        } = std::mem::take(&mut self.state)
        else {
            return Err(RefreshTokenError::NotAuthenticated.into());
        };

        let refresh_token = previous_refresh_token
            .clone()
            .ok_or(RefreshTokenError::MissingRefreshToken)?;

        let token_endpoint = self
            .server
            .token_endpoint
            .as_ref()
            .ok_or(RefreshTokenError::MissingTokenEndpoint)?;

        let form = {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            serializer
                .append_pair("grant_type", "refresh_token")
                .append_pair("refresh_token", &refresh_token)
                .append_pair("client_id", &self.registration.client_id);

            if let Some(scope) = self.scope.as_ref() {
                serializer.append_pair("scope", scope);
            }

            serializer.finish()
        };

        let request = Request::builder()
            .uri(token_endpoint.clone())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(AsyncBody::from(form))
            .context(RefreshTokenError::BuildTokenRefreshRequest)?;

        let requested_at = Instant::now();

        let mut response = self.http_client.send(request).await?;
        let token_response: TokenResponse = decode_response_json(&mut response).await?;

        self.state = State::Authenticated {
            access_token: token_response.access_token,
            token_type: if token_response.token_type.is_empty() {
                previous_token_type
            } else {
                token_response.token_type
            },
            expires_at: token_response
                .expires_in
                .map(|expires_in| requested_at + Duration::from_secs(expires_in)),
            refresh_token: token_response.refresh_token.or(previous_refresh_token),
        };

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum InitError {
    #[error("resource metadata specified 0 authorization servers")]
    NoAuthorizationServers,
}

#[derive(Debug, Error)]
pub enum AuthorizeUrlError {
    #[error("authorization server metadata does not specify an authorization_endpoint")]
    MissingAuthorizationEndpoint,
}

#[derive(Debug, Error)]
pub enum ExchangeTokenError {
    #[error("cannot exchange token: oauth client is not waiting for an authorization code")]
    NotWaitingForAuthorizationCode,

    #[error("authorization server metadata does not specify a token_endpoint")]
    MissingTokenEndpoint,

    #[error("failed to build token exchange request")]
    BuildTokenExchangeRequest,
}

#[derive(Debug, Error)]
pub enum AccessTokenError {
    #[error("OAuth access token is expired and no refresh token is available")]
    AccessTokenExpiredNoRefreshToken,
}

#[derive(Debug, Error)]
pub enum RefreshTokenError {
    #[error("cannot refresh: OAuth client is waiting for an authorization code")]
    WaitingForAuthorizationCode,

    #[error("cannot refresh: OAuth client is not authenticated")]
    NotAuthenticated,

    #[error("cannot refresh: missing refresh token")]
    MissingRefreshToken,

    #[error("cannot refresh: authorization server metadata does not specify a token_endpoint")]
    MissingTokenEndpoint,

    #[error("failed to build token refresh request")]
    BuildTokenRefreshRequest,
}

#[derive(Debug, Error)]
pub enum CallbackParseError {
    #[error("invalid oauth callback query: missing code")]
    MissingCode,

    #[error("invalid oauth callback query: missing state")]
    MissingState,

    #[error("invalid oauth callback state: missing server id")]
    MissingServerId,
}

#[derive(Debug)]
pub struct AuthorizeUrl {
    url: Url,
}

impl AuthorizeUrl {
    pub fn url(mut self, server_id: ContextServerId) -> Url {
        self.url
            .query_pairs_mut()
            .append_pair("state", &server_id.0);
        self.url
    }
}

#[derive(Debug)]
pub struct OAuthCallback {
    pub server_id: ContextServerId,
    pub code: String,
}

impl OAuthCallback {
    pub const URI: &str = "zed://mcp/oauth/callback";

    pub fn parse_query(query: &str) -> Result<Self> {
        let mut code: Option<String> = None;
        let mut state: Option<String> = None;

        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "code" => {
                    if !value.is_empty() {
                        code = Some(value.into_owned());
                    }
                }
                "state" => {
                    if !value.is_empty() {
                        state = Some(value.into_owned());
                    }
                }
                _ => {}
            }

            if code.is_some() && state.is_some() {
                break;
            }
        }

        let code = code.ok_or(CallbackParseError::MissingCode)?;
        let state = state.ok_or(CallbackParseError::MissingState)?;

        let state = state.trim();
        if state.is_empty() {
            return Err(CallbackParseError::MissingServerId.into());
        }

        let server_id = ContextServerId(Arc::<str>::from(state.to_string()));

        Ok(Self { server_id, code })
    }
}

fn generate_code_verifier() -> String {
    const LENGTH: usize = 64;
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

    let dist = rand::distr::slice::Choose::new(ALPHABET).unwrap();

    let bytes: Vec<u8> = dist
        .sample_iter(rand::rng())
        .take(LENGTH)
        .copied()
        .collect();

    // SAFETY: All bytes come from ALPHABET which is ASCII
    unsafe { String::from_utf8_unchecked(bytes) }
}

#[derive(Deserialize)]
struct ClientRegistration {
    client_id: String,
    client_secret: Option<String>,
    client_id_issued_at: Option<u64>,
    client_secret_expires_at: Option<u64>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

// Resource Metadata

#[derive(Deserialize)]
pub struct ProtectedResource {
    resource: String,

    #[serde(default)]
    authorization_servers: Vec<AbsUri>,

    #[serde(default)]
    scopes_supported: Vec<String>,

    #[serde(default)]
    bearer_methods_supported: Vec<String>,

    #[serde(default)]
    resource_name: Option<String>,
}

impl ProtectedResource {
    pub async fn fetch(url: &str, http_client: &Arc<dyn HttpClient>) -> Result<Self> {
        get_json(url, http_client)
            .await
            .context("Fetching resource metadata")
    }

    pub async fn fetch_well_known(
        server_endpoint: &str,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self> {
        let endpoint_uri = server_endpoint.parse::<Uri>()?.try_into()?;
        let well_known_uri = well_known_pre(&endpoint_uri, "oauth-protected-resource");

        return Self::fetch(&well_known_uri, http_client)
            .await
            .context("From well-known URL");
    }
}

// Server Metadata

#[derive(Debug, Deserialize)]
pub struct AuthorizationServer {
    issuer: String,

    #[serde(default)]
    authorization_endpoint: Option<AbsUri>,

    #[serde(default)]
    token_endpoint: Option<AbsUri>,

    #[serde(default)]
    jwks_uri: Option<AbsUri>,

    #[serde(default)]
    registration_endpoint: Option<AbsUri>,

    #[serde(default)]
    scopes_supported: Vec<String>,

    #[serde(default)]
    response_types_supported: Vec<String>,

    #[serde(default)]
    grant_types_supported: Vec<String>,

    #[serde(default)]
    token_endpoint_auth_methods_supported: Vec<String>,

    #[serde(default)]
    code_challenge_methods_supported: Vec<String>,

    #[serde(default)]
    client_id_metadata_document_supported: bool,
}

impl AuthorizationServer {
    pub async fn fetch(
        issuer_uri: &AbsUri,
        http_client: &Arc<dyn HttpClient>,
    ) -> Result<Self, AuthorizationServerMetadataDiscoveryError> {
        // We must attempt multiple well-known endpoints based on the issuer url
        //
        // https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization#authorization-server-metadata-discovery
        let candidates: [fn(&AbsUri) -> Option<String>; _] = [
            // 1. OAuth 2.0 Authorization Server Metadata
            |base| well_known_pre(base, "oauth-authorization-server").into(),
            // 2. OpenID Connect Discovery 1.0 with path insertion
            |base| well_known_pre(base, "openid-configuration").into(),
            // 3. OpenID Connect Discovery 1.0 with path appening
            |base| {
                if base.path() != "/" {
                    Some(well_known_post(base, "openid-configuration"))
                } else {
                    // We already tried the root in the previous step
                    None
                }
            },
        ];

        let mut attempted_urls = Vec::new();

        for build_url in candidates {
            let Some(url) = build_url(&issuer_uri) else {
                continue;
            };

            match get_json(&url, &http_client).await {
                Ok(meta) => return Ok(meta),
                Err(err) => {
                    attempted_urls.push((url, err));
                }
            }
        }

        Err(AuthorizationServerMetadataDiscoveryError { attempted_urls })
    }
}

fn well_known_pre(base_uri: &AbsUri, well_known_segment: &str) -> String {
    format!(
        "{}://{}/.well-known/{well_known_segment}{}",
        base_uri.scheme_str(),
        base_uri.authority(),
        base_uri.path().trim_end_matches('/')
    )
}

fn well_known_post(base_uri: &AbsUri, well_known_segment: &str) -> String {
    let path = base_uri.path();
    let separator = if path.ends_with('/') { "" } else { "/" };
    format!(
        "{}://{}{}{separator}.well-known/{well_known_segment}",
        base_uri.scheme_str(),
        base_uri.authority(),
        path,
    )
}

#[derive(Debug)]
pub struct AuthorizationServerMetadataDiscoveryError {
    attempted_urls: Vec<(String, anyhow::Error)>,
}

impl Error for AuthorizationServerMetadataDiscoveryError {}

impl Display for AuthorizationServerMetadataDiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Failed to discover authorization server metadata. Attempted URLs:"
        )?;

        for (url, err) in &self.attempted_urls {
            writeln!(f, "- {url}: {err}")?;
        }

        fmt::Result::Ok(())
    }
}

async fn get_json<Out: DeserializeOwned>(
    url: &str,
    http_client: &Arc<dyn HttpClient>,
) -> Result<Out> {
    {
        let mut response = http_client.get(url, AsyncBody::empty(), true).await?;
        decode_response_json(&mut response).await
    }
    .with_context(|| format!("GET {url}"))
}

async fn post_json<In: Serialize, Out: DeserializeOwned>(
    url: &str,
    payload: In,
    http_client: &Arc<dyn HttpClient>,
) -> Result<Out> {
    {
        let mut response = http_client
            .post_json(url, serde_json::to_string(&payload)?.into())
            .await?;
        decode_response_json(&mut response).await
    }
    .with_context(|| format!("POST {url}"))
}

async fn decode_response_json<T: DeserializeOwned>(
    response: &mut Response<AsyncBody>,
) -> Result<T> {
    let mut content = Vec::new();
    response.body_mut().read_to_end(&mut content).await?;
    if response.status().is_success() {
        Ok(serde_json::from_slice(&content)?)
    } else {
        anyhow::bail!(
            "Status: {}.\nBody: {}",
            response.status(),
            String::from_utf8_lossy(&content)
        );
    }
}

mod abs_uri {
    use std::{
        error::Error,
        fmt::{self, Display},
        ops::Deref,
    };

    use http_client::{Uri, http::uri::Authority};
    use serde::Deserialize;

    #[derive(Debug, Clone)]
    pub struct AbsUri(Uri);

    impl AbsUri {
        pub fn authority(&self) -> &Authority {
            self.0.authority().unwrap()
        }

        pub fn scheme_str(&self) -> &str {
            self.0.scheme_str().unwrap()
        }
    }

    impl Into<Uri> for AbsUri {
        fn into(self) -> Uri {
            self.0
        }
    }

    impl TryFrom<Uri> for AbsUri {
        type Error = AbsUriError;

        fn try_from(uri: Uri) -> Result<Self, Self::Error> {
            if uri.scheme().is_none() {
                return Err(AbsUriError::MissingScheme);
            }
            if uri.authority().is_none() {
                return Err(AbsUriError::MissingAuthority);
            }
            Ok(Self(uri))
        }
    }

    impl Deref for AbsUri {
        type Target = Uri;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'de> Deserialize<'de> for AbsUri {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse::<Uri>()
                .map_err(serde::de::Error::custom)?
                .try_into()
                .map_err(|e| serde::de::Error::custom(format!("{e:?}")))
        }
    }

    #[derive(Debug)]
    pub enum AbsUriError {
        MissingScheme,
        MissingAuthority,
    }

    impl Error for AbsUriError {}

    impl Display for AbsUriError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AbsUriError::MissingScheme => write!(f, "URI is not absolute: Missing scheme"),
                AbsUriError::MissingAuthority => {
                    write!(f, "URI is not absolute: Missing authority")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use futures::StreamExt;
    use futures::channel::{mpsc, oneshot};
    use gpui::{TestAppContext, prelude::*};
    use http_client::{FakeHttpClient, Request, Response};

    #[gpui::test]
    async fn fetch_server_metadata_chain(cx: &mut TestAppContext) {
        expect_fallback_chain(
            "https://auth.example.com/tenant/123",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com/tenant/123/",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server/tenant/123",
                "https://auth.example.com/.well-known/openid-configuration/tenant/123",
                "https://auth.example.com/tenant/123/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;

        expect_fallback_chain(
            "https://auth.example.com",
            &[
                "https://auth.example.com/.well-known/oauth-authorization-server",
                "https://auth.example.com/.well-known/openid-configuration",
            ],
            cx,
        )
        .await;
    }

    async fn expect_fallback_chain(issuer_uri: &str, urls: &[&str], cx: &mut TestAppContext) {
        let issuer_uri: AbsUri = issuer_uri.parse::<Uri>().unwrap().try_into().unwrap();
        let (client, mut request_rx) = fake_client();

        for i in 0..urls.len() {
            let issuer_uri = issuer_uri.clone();
            let client = client.clone();
            let fetch_task = cx.background_spawn(async move {
                AuthorizationServer::fetch(&issuer_uri, &client).await
            });

            for request_url in &urls[..i] {
                let request = request_rx.next().await.unwrap();
                assert_eq!(request.uri, *request_url);
                respond(request, not_found());
            }

            let request = request_rx.next().await.unwrap();
            assert_eq!(request.uri, *urls[i]);
            respond(
                request,
                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(AsyncBody::from(valid_metadata_json(
                        "https://auth.example.com",
                    )))
                    .unwrap(),
            );

            let metadata = fetch_task.await.expect("fetch should succeed");
            assert_eq!(metadata.issuer, "https://auth.example.com");
        }
    }

    #[gpui::test]
    async fn fetch_server_metadata_openid_root_stops_on_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        let request = requests.next().await.expect("Expected first request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/oauth-authorization-server"
        );
        respond(request, not_found());

        let request = requests.next().await.expect("Expected second request");
        assert_eq!(
            request.uri,
            "https://auth.example.com/.well-known/openid-configuration"
        );
        respond(request, not_found());

        // should not attempt well_known_post since it'd be the same as well_known_pre
        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 2);
    }

    #[gpui::test]
    async fn fetch_server_metadata_all_fail(cx: &mut TestAppContext) {
        let (client, mut requests) = fake_client();
        let http_client = client.clone();

        let fetch_task = cx.background_spawn(async move {
            let issuer_uri: AbsUri = "https://auth.example.com/tenant/123"
                .parse::<Uri>()
                .unwrap()
                .try_into()
                .unwrap();

            AuthorizationServer::fetch(&issuer_uri, &http_client).await
        });

        for _ in 0..3 {
            let request = requests.next().await.expect("Expected request");
            respond(request, not_found());
        }

        let error = fetch_task.await.expect_err("fetch should fail");
        assert_eq!(error.attempted_urls.len(), 3);
    }

    struct FakeRequest {
        uri: String,
        respond: oneshot::Sender<Response<AsyncBody>>,
    }

    fn fake_client() -> (
        Arc<http_client::HttpClientWithUrl>,
        mpsc::UnboundedReceiver<FakeRequest>,
    ) {
        let (request_sender, request_receiver) = mpsc::unbounded::<FakeRequest>();

        let client = FakeHttpClient::create(move |req: Request<AsyncBody>| {
            let request_sender = request_sender.clone();
            async move {
                let (respond, response_receiver) = oneshot::channel();
                request_sender
                    .unbounded_send(FakeRequest {
                        uri: req.uri().to_string(),
                        respond,
                    })
                    .expect("Test receiver dropped");

                response_receiver
                    .await
                    .map_err(|_| anyhow::anyhow!("Test dropped response sender"))
            }
        });

        (client, request_receiver)
    }

    fn not_found() -> Response<AsyncBody> {
        Response::builder()
            .status(404)
            .body(AsyncBody::from("Not found".to_string()))
            .unwrap()
    }

    fn valid_metadata_json(issuer: &str) -> String {
        serde_json::json!({
            "issuer": issuer,
            "authorization_endpoint": format!("{}/authorize", issuer),
            "token_endpoint": format!("{}/token", issuer),
        })
        .to_string()
    }

    fn respond(request: FakeRequest, response: Response<AsyncBody>) {
        request.respond.send(response).ok();
    }
}
