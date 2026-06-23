use std::{collections::HashMap, path::PathBuf, pin::Pin, sync::Arc};

use fs::Fs;
use futures::{AsyncRead, AsyncReadExt, io::BufReader};
use http::Request;
use http_client::{AsyncBody, HttpClient, Response};
use serde::{Deserialize, Serialize};

use crate::devcontainer_api::DevContainerError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokenResponse {
    pub(crate) token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DockerManifestsResponse {
    pub(crate) layers: Vec<ManifestLayer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ManifestLayer {
    pub(crate) digest: String,
}

/// Acquires a bearer token (if any) for pulling from an OCI Distribution
/// registry, following the spec's challenge-response flow.
///
/// 1. Probe `GET /v2/` unauthenticated.
/// 2. If the registry responds with `200`, no auth is required — return an
///    empty token. Callers must omit the `Authorization` header in that case.
/// 3. If the registry responds with `401`, parse the `Www-Authenticate: Bearer`
///    challenge header and fetch a token from the advertised `realm`, passing
///    along the registry-supplied `service` and a `repository:<repo>:pull`
///    scope.
///
/// This handles both `ghcr.io`-style registries (which advertise a token realm
/// via the challenge) and Distribution registries that allow anonymous reads
/// (which do not host a `/token` endpoint at all).
pub(crate) async fn get_oci_auth_token(
    registry: &str,
    repository_path: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<TokenResponse, String> {
    let probe_url = format!("https://{registry}/v2/");
    log::debug!("Probing OCI registry for auth requirements: {}", probe_url);
    let probe = send_oci_request(&probe_url, "", client).await?;

    if probe.status().is_success() {
        return Ok(TokenResponse {
            token: String::new(),
        });
    }

    if probe.status() != http::StatusCode::UNAUTHORIZED {
        return Err(format!(
            "OCI probe to {} returned unexpected status {}",
            probe_url,
            probe.status().as_u16(),
        ));
    }

    let header = probe
        .headers()
        .get("www-authenticate")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            format!("Registry {registry} returned 401 with no Www-Authenticate header")
        })?
        .to_owned();

    let challenge = parse_bearer_challenge(&header)?;
    let realm = challenge
        .realm
        .ok_or_else(|| format!("Bearer challenge from {registry} omitted required `realm`"))?;

    let mut token_url = url::Url::parse(&realm)
        .map_err(|e| format!("Invalid realm URL {realm} in challenge from {registry}: {e}"))?;
    {
        let mut query = token_url.query_pairs_mut();
        if let Some(service) = challenge.service {
            query.append_pair("service", &service);
        }
        query.append_pair("scope", &format!("repository:{repository_path}:pull"));
    }

    log::debug!("Fetching OCI token from: {}", token_url);
    get_deserialized_response("", token_url.as_str(), client).await
}

/// A parsed `Www-Authenticate: Bearer ...` challenge.
#[derive(Debug, Default, PartialEq, Eq)]
struct BearerChallenge {
    realm: Option<String>,
    service: Option<String>,
    scope: Option<String>,
}

/// Parse a `Www-Authenticate` header value carrying the `Bearer` scheme into
/// the OCI-relevant fields. Anything other than `realm`, `service`, `scope` is
/// ignored.
///
/// The auth-param parser is adapted from the RFC 7235 implementation in
/// `crates/context_server/src/oauth.rs::parse_auth_params`. It is duplicated
/// here rather than shared to avoid a cross-crate dependency for ~60 lines of
/// code; consolidating into a shared crate is a worthwhile follow-up.
fn parse_bearer_challenge(header: &str) -> Result<BearerChallenge, String> {
    let header = header.trim();
    let params_str = if header.len() >= 6 && header[..6].eq_ignore_ascii_case("bearer") {
        header[6..].trim()
    } else {
        return Err(format!(
            "Www-Authenticate header does not use the Bearer scheme: {header}"
        ));
    };

    let params = parse_auth_params(params_str);
    Ok(BearerChallenge {
        realm: params.get("realm").cloned(),
        service: params.get("service").cloned(),
        scope: params.get("scope").cloned(),
    })
}

/// Parse comma-separated `key="value"` or `key=token` parameters from an
/// auth-param list (RFC 7235 Section 2.1). Keys are lowercased.
fn parse_auth_params(input: &str) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();
    let mut remaining = input.trim();

    while !remaining.is_empty() {
        remaining = remaining.trim_start_matches(|c: char| c == ',' || c.is_whitespace());
        if remaining.is_empty() {
            break;
        }

        let eq_pos = match remaining.find('=') {
            Some(pos) => pos,
            None => break,
        };

        let key = remaining[..eq_pos].trim().to_lowercase();
        remaining = &remaining[eq_pos + 1..];
        remaining = remaining.trim_start();

        let value;
        if remaining.starts_with('"') {
            remaining = &remaining[1..];
            let mut val = String::new();
            let mut chars = remaining.char_indices();
            loop {
                match chars.next() {
                    Some((_, '\\')) => {
                        if let Some((_, c)) = chars.next() {
                            val.push(c);
                        }
                    }
                    Some((i, '"')) => {
                        remaining = &remaining[i + 1..];
                        break;
                    }
                    Some((_, c)) => val.push(c),
                    None => {
                        remaining = "";
                        break;
                    }
                }
            }
            value = val;
        } else {
            let end = remaining
                .find(|c: char| c == ',' || c.is_whitespace())
                .unwrap_or(remaining.len());
            value = remaining[..end].to_string();
            remaining = &remaining[end..];
        }

        if !key.is_empty() {
            params.insert(key, value);
        }
    }

    params
}

/// Build and send a GET request to an OCI endpoint, attaching `Authorization`
/// only when `token` is non-empty.
async fn send_oci_request(
    url: &str,
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<Response<AsyncBody>, String> {
    let mut builder = Request::get(url).header("Accept", "application/vnd.oci.image.manifest.v1+json");
    if !token.is_empty() {
        builder = builder.header("Authorization", format!("Bearer {}", token));
    }
    let request = builder
        .body(AsyncBody::default())
        .map_err(|e| format!("Failed to create request: {e}"))?;
    client
        .send(request)
        .await
        .map_err(|e| format!("Failed to send request to {url}: {e}"))
}

pub(crate) async fn get_latest_oci_manifest(
    token: &str,
    registry: &str,
    repository_path: &str,
    client: &Arc<dyn HttpClient>,
    id: Option<&str>,
) -> Result<DockerManifestsResponse, String> {
    get_oci_manifest(registry, repository_path, token, client, "latest", id).await
}

pub(crate) async fn get_oci_manifest(
    registry: &str,
    repository_path: &str,
    token: &str,
    client: &Arc<dyn HttpClient>,
    version: &str,
    id: Option<&str>,
) -> Result<DockerManifestsResponse, String> {
    let url = match id {
        Some(id) => format!("https://{registry}/v2/{repository_path}/{id}/manifests/{version}"),
        None => format!("https://{registry}/v2/{repository_path}/manifests/{version}"),
    };

    get_deserialized_response(token, &url, client).await
}

pub(crate) async fn get_deserializable_oci_blob<T>(
    token: &str,
    registry: &str,
    repository_path: &str,
    blob_digest: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<T, String>
where
    T: for<'a> Deserialize<'a>,
{
    let url = format!("https://{registry}/v2/{repository_path}/blobs/{blob_digest}");
    get_deserialized_response(token, &url, client).await
}

pub(crate) async fn download_oci_tarball(
    token: &str,
    registry: &str,
    repository_path: &str,
    blob_digest: &str,
    accept_header: &str,
    dest_dir: &PathBuf,
    client: &Arc<dyn HttpClient>,
    fs: &Arc<dyn Fs>,
    id: Option<&str>,
) -> Result<(), DevContainerError> {
    let url = match id {
        Some(id) => format!("https://{registry}/v2/{repository_path}/{id}/blobs/{blob_digest}"),
        None => format!("https://{registry}/v2/{repository_path}/blobs/{blob_digest}"),
    };

    let mut builder = Request::get(&url).header("Accept", accept_header);
    if !token.is_empty() {
        builder = builder.header("Authorization", format!("Bearer {}", token));
    }
    let request = builder.body(AsyncBody::default()).map_err(|e| {
        log::error!("Failed to create blob request: {e}");
        DevContainerError::ResourceFetchFailed
    })?;

    let mut response = client.send(request).await.map_err(|e| {
        log::error!("Failed to download feature blob: {e}");
        DevContainerError::ResourceFetchFailed
    })?;
    let status = response.status();

    let body = BufReader::new(response.body_mut());

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(body.buffer());
        log::error!(
            "Feature blob download returned HTTP {}: {}",
            status.as_u16(),
            body_text,
        );
        return Err(DevContainerError::ResourceFetchFailed);
    }

    futures::pin_mut!(body);
    let body: Pin<&mut (dyn AsyncRead + Send)> = body;
    let archive = async_tar::Archive::new(body);
    fs.extract_tar_file(dest_dir, archive).await.map_err(|e| {
        log::error!("Failed to extract feature tarball: {e}");
        DevContainerError::FilesystemError
    })?;

    Ok(())
}

pub(crate) async fn get_deserialized_response<T>(
    token: &str,
    url: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let response = send_oci_request(url, token, client).await?;
    let status = response.status();
    let mut output = String::new();

    if let Err(e) = response.into_body().read_to_string(&mut output).await {
        return Err(format!("Failed to read response body from {}: {}", url, e));
    };

    if !status.is_success() {
        return Err(format!(
            "OCI request to {} returned HTTP {}: {}",
            url,
            status.as_u16(),
            &output[..output.len().min(500)],
        ));
    }

    match serde_json_lenient::from_str(&output) {
        Ok(response) => Ok(response),
        Err(e) => Err(format!(
            "Failed to deserialize response from {}: {} (body: {})",
            url,
            e,
            &output[..output.len().min(500)],
        )),
    }
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, sync::Arc};

    use fs::{FakeFs, Fs};
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, anyhow};
    use serde::Deserialize;

    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::oci::{
        BearerChallenge, TokenResponse, download_oci_tarball, get_deserializable_oci_blob,
        get_deserialized_response, get_latest_oci_manifest, get_oci_auth_token,
        parse_bearer_challenge,
    };

    async fn build_test_tarball() -> Vec<u8> {
        let devcontainer_json = concat!(
            "// For format details, see https://aka.ms/devcontainer.json. For config options, see the\n",
            "// README at: https://github.com/devcontainers/templates/tree/main/src/alpine\n",
            "{\n",
            "\t\"name\": \"Alpine\",\n",
            "\t// Or use a Dockerfile or Docker Compose file. More info: https://containers.dev/guide/dockerfile\n",
            "\t\"image\": \"mcr.microsoft.com/devcontainers/base:alpine-${templateOption:imageVariant}\"\n",
            "}\n",
        );

        let dependabot_yml = concat!(
            "version: 2\n",
            "updates:\n",
            " - package-ecosystem: \"devcontainers\"\n",
            "   directory: \"/\"\n",
            "   schedule:\n",
            "     interval: weekly\n",
        );

        let buffer = futures::io::Cursor::new(Vec::new());
        let mut builder = async_tar::Builder::new(buffer);

        let files: &[(&str, &[u8], u32)] = &[
            (
                ".devcontainer/devcontainer.json",
                devcontainer_json.as_bytes(),
                0o644,
            ),
            (".github/dependabot.yml", dependabot_yml.as_bytes(), 0o644),
            ("NOTES.md", b"Some notes", 0o644),
            ("README.md", b"# Alpine\n", 0o644),
        ];

        for (path, data, mode) in files {
            let mut header = async_tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(*mode);
            header.set_entry_type(async_tar::EntryType::Regular);
            header.set_cksum();
            builder.append_data(&mut header, path, *data).await.unwrap();
        }

        let buffer = builder.into_inner().await.unwrap();
        buffer.into_inner()
    }
    fn test_oci_registry() -> &'static str {
        "ghcr.io"
    }
    fn test_oci_repository() -> &'static str {
        "repository"
    }

    #[gpui::test]
    async fn test_get_deserialized_response(_cx: &mut TestAppContext) {
        let client = FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response =
            get_deserialized_response::<TokenResponse>("", "https://ghcr.io/token", &client).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string())
    }

    #[gpui::test]
    async fn test_anonymous_request_omits_authorization_header() {
        let client = FakeHttpClient::create(|request| async move {
            if request.headers().get("Authorization").is_some() {
                return Err(anyhow!("expected no Authorization header for anonymous"));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"\" }".into())
                .unwrap())
        });

        let response =
            get_deserialized_response::<TokenResponse>("", "https://ghcr.io/token", &client).await;
        assert!(response.is_ok());
    }

    #[gpui::test]
    async fn test_token_request_includes_authorization_header() {
        let client = FakeHttpClient::create(|request| async move {
            let auth = request
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default()
                .to_owned();
            if auth != "Bearer foo" {
                return Err(anyhow!("unexpected Authorization header: {auth}"));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"x\" }".into())
                .unwrap())
        });

        let response =
            get_deserialized_response::<TokenResponse>("foo", "https://ghcr.io/token", &client)
                .await;
        assert!(response.is_ok());
    }

    #[gpui::test]
    async fn test_get_oci_auth_token_anonymous_registry() {
        let request_count = Arc::new(AtomicUsize::new(0));
        let client = FakeHttpClient::create({
            let request_count = request_count.clone();
            move |request| {
                let request_count = request_count.clone();
                async move {
                    request_count.fetch_add(1, Ordering::SeqCst);
                    let path = request.uri().path();
                    if path != "/v2/" {
                        return Err(anyhow!("unexpected path: {path}"));
                    }
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body("{}".into())
                        .unwrap())
                }
            }
        });

        let response =
            get_oci_auth_token("anon.example.com", "some/repo", &client)
                .await
                .expect("anonymous probe should succeed");
        assert_eq!(response.token, "");
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_get_oci_auth_token_follows_challenge() {
        let request_count = Arc::new(AtomicUsize::new(0));
        let client = FakeHttpClient::create({
            let request_count = request_count.clone();
            move |request| {
                let request_count = request_count.clone();
                async move {
                    let n = request_count.fetch_add(1, Ordering::SeqCst);
                    if n == 0 {
                        let host = request.uri().host().unwrap_or_default();
                        if host != "registry.example.com" {
                            return Err(anyhow!("unexpected probe host: {host}"));
                        }
                        if request.uri().path() != "/v2/" {
                            return Err(anyhow!(
                                "unexpected probe path: {}",
                                request.uri().path()
                            ));
                        }
                        return Ok(http_client::Response::builder()
                            .status(401)
                            .header(
                                "Www-Authenticate",
                                "Bearer realm=\"https://auth.example.com/token\",service=\"registry.example.com\",scope=\"ignored\"",
                            )
                            .body("".into())
                            .unwrap());
                    }
                    let host = request.uri().host().unwrap_or_default();
                    if host != "auth.example.com" {
                        return Err(anyhow!("unexpected token host: {host}"));
                    }
                    if request.uri().path() != "/token" {
                        return Err(anyhow!(
                            "unexpected token path: {}",
                            request.uri().path()
                        ));
                    }
                    let query = request.uri().query().unwrap_or_default();
                    if !query.contains("service=registry.example.com") {
                        return Err(anyhow!("missing service in query: {query}"));
                    }
                    if !query.contains("scope=repository%3Asome%2Frepo%3Apull") {
                        return Err(anyhow!("missing scope in query: {query}"));
                    }
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body("{ \"token\": \"chocolate\" }".into())
                        .unwrap())
                }
            }
        });

        let response =
            get_oci_auth_token("registry.example.com", "some/repo", &client)
                .await
                .expect("challenge follow should succeed");
        assert_eq!(response.token, "chocolate");
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn test_get_oci_auth_token_rejects_401_without_challenge() {
        let client = FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(401)
                .body("".into())
                .unwrap())
        });

        let response =
            get_oci_auth_token("registry.example.com", "some/repo", &client).await;
        assert!(response.is_err());
        let err = response.unwrap_err();
        assert!(
            err.contains("Www-Authenticate"),
            "expected Www-Authenticate in error, got: {err}"
        );
    }

    #[test]
    fn test_parse_bearer_challenge() {
        let parsed = parse_bearer_challenge(
            "Bearer realm=\"https://auth.example.com/token\",service=\"example.com\",scope=\"repository:foo:pull\"",
        )
        .expect("quoted challenge");
        assert_eq!(
            parsed,
            BearerChallenge {
                realm: Some("https://auth.example.com/token".to_string()),
                service: Some("example.com".to_string()),
                scope: Some("repository:foo:pull".to_string()),
            }
        );

        let parsed = parse_bearer_challenge("bearer realm=https://x/token,service=x")
            .expect("unquoted challenge");
        assert_eq!(
            parsed,
            BearerChallenge {
                realm: Some("https://x/token".to_string()),
                service: Some("x".to_string()),
                scope: None,
            }
        );

        assert!(parse_bearer_challenge("Basic realm=\"x\"").is_err());
    }

    #[gpui::test]
    async fn test_get_latest_manifests() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != test_oci_registry() {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path != format!("/v2/{}/manifests/latest", test_oci_repository()) {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{
                    \"schemaVersion\": 2,
                    \"mediaType\": \"application/vnd.oci.image.manifest.v1+json\",
                    \"config\": {
                        \"mediaType\": \"application/vnd.devcontainers\",
                        \"digest\": \"sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a\",
                        \"size\": 2
                    },
                    \"layers\": [
                        {
                            \"mediaType\": \"application/vnd.devcontainers.collection.layer.v1+json\",
                            \"digest\": \"sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09\",
                            \"size\": 65235,
                            \"annotations\": {
                                \"org.opencontainers.image.title\": \"devcontainer-collection.json\"
                            }
                        }
                    ],
                    \"annotations\": {
                        \"com.github.package.type\": \"devcontainer_collection\"
                    }
                }".into())
                .unwrap())
        });

        let response = get_latest_oci_manifest(
            "",
            test_oci_registry(),
            test_oci_repository(),
            &client,
            None,
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();

        assert_eq!(response.layers.len(), 1);
        assert_eq!(
            response.layers[0].digest,
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09"
        );
    }

    #[gpui::test]
    async fn test_get_oci_blob() {
        #[derive(Debug, Deserialize)]
        struct DeserializableTestStruct {
            foo: String,
        }

        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != test_oci_registry() {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path != format!("/v2/{}/blobs/blobdigest", test_oci_repository()) {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body(
                    r#"
                    {
                        "foo": "bar"
                    }
                    "#
                    .into(),
                )
                .unwrap())
        });

        let response: Result<DeserializableTestStruct, String> = get_deserializable_oci_blob(
            "",
            test_oci_registry(),
            test_oci_repository(),
            "blobdigest",
            &client,
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();

        assert_eq!(response.foo, "bar".to_string());
    }

    #[gpui::test]
    async fn test_download_oci_tarball(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());

        let destination_dir = PathBuf::from("/tmp/extracted");
        fs.create_dir(&destination_dir).await.unwrap();

        let tarball_bytes = build_test_tarball().await;
        let tarball = std::sync::Arc::new(tarball_bytes);

        let client = FakeHttpClient::create(move |request| {
            let tarball = tarball.clone();
            async move {
                let host = request.uri().host();
                if host.is_none() || host.unwrap() != test_oci_registry() {
                    return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
                }
                let path = request.uri().path();
                if path != format!("/v2/{}/blobs/blobdigest", test_oci_repository()) {
                    return Err(anyhow!("Unexpected path: {}", path));
                }
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(tarball.to_vec().into())
                    .unwrap())
            }
        });

        let response = download_oci_tarball(
            "",
            test_oci_registry(),
            test_oci_repository(),
            "blobdigest",
            "header",
            &destination_dir,
            &client,
            &fs,
            None,
        )
        .await;
        assert!(response.is_ok());

        let expected_devcontainer_json = concat!(
            "// For format details, see https://aka.ms/devcontainer.json. For config options, see the\n",
            "// README at: https://github.com/devcontainers/templates/tree/main/src/alpine\n",
            "{\n",
            "\t\"name\": \"Alpine\",\n",
            "\t// Or use a Dockerfile or Docker Compose file. More info: https://containers.dev/guide/dockerfile\n",
            "\t\"image\": \"mcr.microsoft.com/devcontainers/base:alpine-${templateOption:imageVariant}\"\n",
            "}\n",
        );

        assert_eq!(
            fs.load(&destination_dir.join(".devcontainer/devcontainer.json"))
                .await
                .unwrap(),
            expected_devcontainer_json
        )
    }
}
