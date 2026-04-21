use std::{path::PathBuf, pin::Pin, sync::Arc};

use fs::Fs;
use futures::{AsyncRead, AsyncReadExt, io::BufReader};
use http::Request;
use http_client::{AsyncBody, HttpClient};
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

/// Gets a bearer token for pulling from a container registry repository.
///
/// This uses the registry's `/token` endpoint directly, which works for
/// `ghcr.io` and other registries that follow the same convention.  For
/// registries that require a full `WWW-Authenticate` negotiation flow this
/// would need to be extended.
pub(crate) async fn get_oci_token(
    registry: &str,
    repository_path: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<TokenResponse, String> {
    let url = format!(
        "https://{registry}/token?service={registry}&scope=repository:{repository_path}:pull",
    );
    log::debug!("Fetching OCI token from: {}", url);
    get_deserialized_response("", &url, client)
        .await
        .map_err(|e| {
            log::error!("OCI token request failed for {}: {e}", url);
            e
        })
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

    let request = Request::get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", accept_header)
        .body(AsyncBody::default())
        .map_err(|e| {
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
    let request = match Request::get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.oci.image.manifest.v1+json")
        .body(AsyncBody::default())
    {
        Ok(request) => request,
        Err(e) => return Err(format!("Failed to create request: {}", e)),
    };
    let response = match client.send(request).await {
        Ok(response) => response,
        Err(e) => {
            return Err(format!("Failed to send request to {}: {}", url, e));
        }
    };

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

    use crate::oci::{
        TokenResponse, download_oci_tarball, get_deserializable_oci_blob,
        get_deserialized_response, get_latest_oci_manifest, get_oci_token,
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
    async fn test_get_oci_token() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != test_oci_registry() {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path != "/token" {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            let query = request.uri().query();
            if query.is_none()
                || query.unwrap()
                    != format!(
                        "service=ghcr.io&scope=repository:{}:pull",
                        test_oci_repository()
                    )
            {
                return Err(anyhow!("Unexpected query: {}", query.unwrap_or_default()));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response = get_oci_token(test_oci_registry(), test_oci_repository(), &client).await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string());
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
