use std::{
    error::Error,
    fmt::{self, Display},
    sync::Arc,
};

use anyhow::{Context as _, Result};
use http_client::{AsyncBody, HttpClient, Uri};
use serde::{Deserialize, de::DeserializeOwned};
use smol::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct ProtectedResourceMetadata {
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

impl ProtectedResourceMetadata {
    pub async fn fetch(url: &str, http_client: &Arc<dyn HttpClient>) -> Result<Self> {
        fetch_json(url, http_client)
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

#[derive(Debug, Deserialize)]
pub struct AuthorizationServerMetadata {
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
}

impl AuthorizationServerMetadata {
    pub async fn fetch(
        issuer_uri: &AbsUri,
        http_client: Arc<dyn HttpClient>,
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

            match fetch_json(&url, &http_client).await {
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

async fn fetch_json<T: DeserializeOwned>(
    url: &str,
    http_client: &Arc<dyn HttpClient>,
) -> Result<T> {
    let mut response = http_client.get(url, AsyncBody::empty(), true).await?;
    if response.status().is_success() {
        let mut content = Vec::new();
        response.body_mut().read_to_end(&mut content).await?;
        Ok(serde_json::from_slice(&content)?)
    } else {
        anyhow::bail!("HTTP: {}", response.status());
    }
}

use abs_uri::AbsUri;
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
                AuthorizationServerMetadata::fetch(&issuer_uri, client).await
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

            AuthorizationServerMetadata::fetch(&issuer_uri, http_client).await
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

            AuthorizationServerMetadata::fetch(&issuer_uri, http_client).await
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
