use std::{collections::HashMap, path::PathBuf, sync::Arc};

use futures::AsyncReadExt;
use http::Request;
use http_client::{AsyncBody, HttpClient};
use serde::Deserialize;
use serde_json_lenient::Value;

use crate::{
    DevContainerErrorV2, DockerManifestsResponse, get_deserialized_response,
    model::{FeatureOptions, MountDefinition},
    safe_id_upper,
};

/// Parsed components of an OCI feature reference such as
/// `ghcr.io/devcontainers/features/aws-cli:1`.
///
/// Mirrors the CLI's `OCIRef` in `containerCollectionsOCI.ts`.
#[derive(Debug, Clone)]
pub(crate) struct OciFeatureRef {
    /// Registry hostname, e.g. `ghcr.io`
    pub registry: String,
    /// Full repository path within the registry, e.g. `devcontainers/features/aws-cli`
    pub path: String,
    /// Short feature identifier, e.g. `aws-cli`
    pub id: String,
    /// Version tag, digest, or `latest`
    pub version: String,
}

/// Minimal representation of a `devcontainer-feature.json` file, used to
/// extract option default values after the feature tarball is downloaded.
///
/// See: https://containers.dev/implementors/features/#devcontainer-featurejson-properties
#[derive(Debug, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainerFeatureJson {
    #[serde(rename = "id")]
    pub(crate) _id: Option<String>,
    #[serde(default)]
    pub(crate) options: HashMap<String, FeatureOptionDefinition>,
    pub(crate) mounts: Option<Vec<MountDefinition>>,
    pub(crate) privileged: Option<bool>,
    pub(crate) entrypoint: Option<String>,
}

/// A single option definition inside `devcontainer-feature.json`.
/// We only need the `default` field to populate env variables.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct FeatureOptionDefinition {
    pub(crate) default: Option<Value>,
}

impl FeatureOptionDefinition {
    fn serialize_default(&self) -> Option<String> {
        self.default.as_ref().map(|some_value| match some_value {
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.to_string(),
            Value::Number(n) => n.to_string(),
            other => other.to_string(),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub(crate) struct FeatureManifest {
    file_path: PathBuf,
    feature_json: DevContainerFeatureJson,
}

impl FeatureManifest {
    pub(crate) fn new(file_path: PathBuf, feature_json: DevContainerFeatureJson) -> Self {
        Self {
            file_path,
            feature_json,
        }
    }

    /// Merges user options from devcontainer.json with default options defined in this feature manifest
    pub(crate) fn genereate_merged_env(&self, options: &FeatureOptions) -> HashMap<String, String> {
        let mut merged: HashMap<String, String> = self
            .feature_json
            .options
            .iter()
            .filter_map(|(k, v)| {
                v.serialize_default()
                    .map(|v_some| (safe_id_upper(k), v_some))
            })
            .collect();

        match options {
            FeatureOptions::Bool(_) => {} // TODO what?
            FeatureOptions::String(version) => {
                merged.insert("VERSION".to_string(), version.clone());
            }
            FeatureOptions::Options(map) => {
                for (key, value) in map {
                    merged.insert(safe_id_upper(key), value.to_string());
                }
            }
        }
        merged
    }

    pub(crate) fn write_feature_env(
        &self,
        options: &FeatureOptions,
    ) -> Result<String, DevContainerErrorV2> {
        let merged_env = self.genereate_merged_env(options);

        let env_vars: Vec<String> = merged_env
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        let env_file_content = env_vars.join("\n");

        std::fs::write(
            self.file_path.join("devcontainer-features.env"),
            env_file_content.clone(),
        )
        .map_err(|e| {
            log::error!("error writing devcontainer feature environment: {e}");
            DevContainerErrorV2::UnmappedError
        })?;

        // TODO should probably handle what's returned here in-struct, but we'll take it step by step
        Ok(env_file_content)
    }

    pub(crate) fn mounts(&self) -> Vec<MountDefinition> {
        if let Some(mounts) = &self.feature_json.mounts {
            mounts.clone()
        } else {
            vec![]
        }
    }

    pub(crate) fn privileged(&self) -> bool {
        self.feature_json.privileged.unwrap_or(false)
    }

    pub(crate) fn entrypoint(&self) -> Option<String> {
        self.feature_json.entrypoint.clone()
    }
}

/// Downloads an OCI blob (feature tarball) and extracts it into `dest_dir`.
///
/// The blob is expected to be a gzip-compressed tar archive containing the
/// feature's `install.sh`, `devcontainer-feature.json`, and any other files.
pub(crate) async fn download_and_extract_oci_feature(
    feature_ref: &OciFeatureRef,
    layer_digest: &str,
    token: &str,
    dest_dir: &PathBuf,
    client: &Arc<dyn HttpClient>,
) -> Result<FeatureManifest, String> {
    let url = format!(
        "https://{}/v2/{}/blobs/{}",
        feature_ref.registry, feature_ref.path, layer_digest,
    );
    log::info!(
        "Downloading OCI blob for feature '{}': {}",
        feature_ref.id,
        url
    );

    let request = Request::get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.devcontainers.layer.v1+tar")
        .body(AsyncBody::default())
        .map_err(|e| format!("Failed to create blob request: {e}"))?;

    let response = client
        .send(request)
        .await
        .map_err(|e| format!("Failed to download feature blob: {e}"))?;

    let status = response.status();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("<none>")
        .to_string();
    log::info!(
        "OCI blob response for '{}': status={}, content-type={}",
        feature_ref.id,
        status.as_u16(),
        content_type,
    );

    // Read the entire body into memory so we can inspect it before feeding
    // it to the gzip decoder.
    let mut body_bytes = Vec::new();
    response
        .into_body()
        .read_to_end(&mut body_bytes)
        .await
        .map_err(|e| format!("Failed to read feature blob body: {e}"))?;

    log::info!(
        "OCI blob body for '{}': {} bytes, first 16 bytes: {:02x?}",
        feature_ref.id,
        body_bytes.len(),
        &body_bytes[..body_bytes.len().min(16)],
    );

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(&body_bytes);
        return Err(format!(
            "Feature blob download returned HTTP {}: {}",
            status.as_u16(),
            body_text,
        ));
    }

    // Per the dev container features distribution spec, feature layers use
    // media type `application/vnd.devcontainers.layer.v1+tar` (plain tar).
    // https://containers.dev/implementors/features-distribution/#oci-registry
    let cursor = futures::io::Cursor::new(body_bytes);
    let archive = async_tar::Archive::new(cursor);
    archive
        .unpack(dest_dir)
        .await
        .map_err(|e| format!("Failed to extract feature tarball: {e}"))?;

    let json_path = dest_dir.join("devcontainer-feature.json");
    if !json_path.exists() {
        let message = format!(
            "No devcontainer-feature.json found in {:?}, no defaults to apply",
            dest_dir
        );
        log::error!("{}", &message);
        return Err(message);
    }

    let contents = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("error reading devcontainer-feature.json: {:?}", e))?;

    let feature_json: DevContainerFeatureJson = serde_json_lenient::from_str(&contents)
        .map_err(|e| format!("Failed to parse devcontainer-feature.json: {e}"))?;

    Ok(FeatureManifest::new(dest_dir.clone(), feature_json))
}

/// Fetches the OCI manifest for a feature, returning its layer descriptors.
pub(crate) async fn fetch_oci_feature_manifest(
    feature_ref: &OciFeatureRef,
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "https://{}/v2/{}/manifests/{}",
        feature_ref.registry, feature_ref.path, feature_ref.version,
    );
    log::info!("Fetching OCI manifest from: {}", url);
    let manifest: DockerManifestsResponse = get_deserialized_response(token, &url, client)
        .await
        .map_err(|e| {
            log::error!("OCI manifest request failed for {}: {e}", url);
            e
        })?;
    log::info!(
        "OCI manifest for '{}': {} layer(s), digests: {:?}",
        feature_ref.id,
        manifest.layers.len(),
        manifest
            .layers
            .iter()
            .map(|l| &l.digest)
            .collect::<Vec<_>>(),
    );
    Ok(manifest)
}

/// Parses an OCI feature reference string into its components.
///
/// Handles formats like:
/// - `ghcr.io/devcontainers/features/aws-cli:1`
/// - `ghcr.io/user/repo/go`  (implicitly `:latest`)
/// - `ghcr.io/devcontainers/features/rust@sha256:abc123`
///
/// Returns `None` for local paths (`./…`) and direct tarball URIs (`https://…`).
pub(crate) fn parse_oci_feature_ref(input: &str) -> Option<OciFeatureRef> {
    if input.starts_with('.')
        || input.starts_with('/')
        || input.starts_with("https://")
        || input.starts_with("http://")
    {
        return None;
    }

    let input_lower = input.to_lowercase();

    let (resource, version) = if let Some(at_idx) = input_lower.rfind('@') {
        // Digest-based: ghcr.io/foo/bar@sha256:abc
        (
            input_lower[..at_idx].to_string(),
            input_lower[at_idx + 1..].to_string(),
        )
    } else {
        let last_slash = input_lower.rfind('/');
        let last_colon = input_lower.rfind(':');
        match (last_slash, last_colon) {
            (Some(slash), Some(colon)) if colon > slash => (
                input_lower[..colon].to_string(),
                input_lower[colon + 1..].to_string(),
            ),
            _ => (input_lower.clone(), "latest".to_string()),
        }
    };

    let parts: Vec<&str> = resource.split('/').collect();
    if parts.len() < 3 {
        return None;
    }

    let registry = parts[0].to_string();
    let id = parts[parts.len() - 1].to_string();
    let path = parts[1..].join("/");

    Some(OciFeatureRef {
        registry,
        path,
        id,
        version,
    })
}
