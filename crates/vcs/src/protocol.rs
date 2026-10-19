use anyhow::{Context as _, Result, ensure};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const PROTOCOL_VERSION: u32 = 1;
pub const MAX_MESSAGE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInfo {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub protocol_version: u32,
    pub client_info: ProviderInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilities {
    pub read_only: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub protocol_version: u32,
    pub server_info: ProviderInfo,
    pub capabilities: ProviderCapabilities,
}

impl InitializeResult {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.protocol_version == PROTOCOL_VERSION,
            "unsupported VCS protocol version {}",
            self.protocol_version
        );
        ensure!(
            self.capabilities.read_only,
            "this VCS client requires a read-only provider"
        );
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Baseline {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryDescriptor {
    pub id: String,
    pub root_uri: String,
    pub scope_uri: String,
    pub display_name: String,
    pub baselines: Vec<Baseline>,
}

impl RepositoryDescriptor {
    pub fn validate(&self, requested_scope: &str) -> Result<()> {
        ensure!(!self.id.is_empty(), "empty VCS repository id");
        let requested_scope = local_path(requested_scope)?;
        let scope = local_path(&self.scope_uri)?;
        let root = local_path(&self.root_uri)?;
        ensure!(
            scope == requested_scope,
            "VCS provider changed the requested scope"
        );
        ensure!(
            scope.starts_with(root),
            "VCS scope is outside its repository"
        );
        ensure!(
            !self.baselines.is_empty(),
            "VCS repository has no comparison baseline"
        );
        let mut ids = std::collections::HashSet::new();
        for baseline in &self.baselines {
            ensure!(
                !baseline.id.is_empty() && ids.insert(&baseline.id),
                "invalid or duplicate VCS baseline id"
            );
        }
        Ok(())
    }
}

pub fn local_path(uri: &str) -> Result<PathBuf> {
    let uri = url::Url::parse(uri).context("invalid VCS file URI")?;
    ensure!(
        uri.query().is_none() && uri.fragment().is_none(),
        "VCS file URI cannot contain a query or fragment"
    );
    let path = uri
        .to_file_path()
        .map_err(|()| anyhow::anyhow!("VCS URI must identify a local file"))?;
    ensure!(path.is_absolute(), "VCS file URI must be absolute");
    Ok(path)
}

/// Protocol paths are relative to scope_uri, with slash-separated components.
pub fn validate_path(path: &str) -> Result<()> {
    ensure!(
        !path.is_empty() && !path.contains(['\\', '\0']) && !path.starts_with('/'),
        "invalid VCS relative path"
    );
    ensure!(
        !path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == ".." || part.contains(':')),
        "VCS path escapes its scope or uses a non-portable component"
    );
    Ok(())
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ChangeStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Untracked,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ContentKind {
    File,
    Binary,
    Symlink,
    Submodule,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_path: Option<String>,
    pub status: ChangeStatus,
    pub kind: ContentKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_mode: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_mode: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub id: String,
    pub repository_id: String,
    pub baseline_id: String,
    /// Immutable identity of the selected baseline contents, not necessarily the current revision.
    /// Reusing this identity allows the host to retain already loaded baseline contents.
    pub revision_id: String,
    pub changes: Vec<Change>,
}

impl Snapshot {
    pub fn validate(&self, repository_id: &str, baseline_id: &str) -> Result<()> {
        ensure!(
            !self.id.is_empty() && !self.revision_id.is_empty(),
            "empty VCS snapshot or revision id"
        );
        ensure!(
            self.repository_id == repository_id && self.baseline_id == baseline_id,
            "VCS snapshot belongs to another repository or baseline"
        );
        let mut paths = std::collections::HashSet::new();
        for change in &self.changes {
            validate_path(&change.path)?;
            ensure!(paths.insert(&change.path), "duplicate path in VCS snapshot");
            if let Some(old_path) = &change.old_path {
                validate_path(old_path)?;
            }
            ensure!(
                change.status != ChangeStatus::Renamed || change.old_path.is_some(),
                "VCS rename has no original path"
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Content {
    Absent,
    Text { text: String },
    Binary,
    Symlink { target: String },
}

#[derive(Clone, Debug, Deserialize, Serialize, thiserror::Error)]
#[error("{message} (VCS error {code})")]
pub struct ProviderError {
    pub code: i32,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_snapshot_uses_opaque_ids_and_requires_no_index() {
        let snapshot: Snapshot = serde_json::from_str(r#"{"id":"snapshot:9","repositoryId":"example","baselineId":"submitted","revisionId":"r42","changes":[{"path":"src/main.rs","status":"modified","kind":"file"}]}"#).unwrap();
        snapshot.validate("example", "submitted").unwrap();
        assert_eq!(snapshot.revision_id, "r42");
        assert!(snapshot.validate("example", "other").is_err());
    }

    #[test]
    fn changes_cannot_escape_the_requested_scope() {
        for invalid in [
            "",
            "../secret",
            "/tmp/secret",
            "a/../../b",
            "a//b",
            "C:/secret",
            "..\\secret",
            "a\0b",
        ] {
            assert!(validate_path(invalid).is_err(), "{invalid:?}");
        }
        validate_path("source/Пример с пробелами.rs").unwrap();
    }

    #[test]
    fn provider_must_explicitly_negotiate_read_only_protocol() {
        let missing = r#"{"protocolVersion":1,"serverInfo":{"name":"fixture","version":"1"},"capabilities":{}}"#;
        assert!(serde_json::from_str::<InitializeResult>(missing).is_err());
        let mut result = InitializeResult {
            protocol_version: 1,
            server_info: ProviderInfo {
                name: "fixture".into(),
                version: "1".into(),
            },
            capabilities: ProviderCapabilities { read_only: true },
        };
        result.validate().unwrap();
        result.protocol_version = 2;
        assert!(result.validate().is_err());
        result.protocol_version = 1;
        result.capabilities.read_only = false;
        assert!(result.validate().is_err());
    }
}
