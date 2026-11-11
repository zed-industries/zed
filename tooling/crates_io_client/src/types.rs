use std::fmt;

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionId(pub i32);
#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct GitHubConfigId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitHubAccountId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CrateName(pub String);

impl CrateName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CrateName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiteMetadata {
    pub deployed_sha: String,
    pub commit: String,
    pub cdn_base: Url,
    pub read_only: bool,
    #[serde(default)]
    pub banner_message: Option<String>,
}

/// Filters for listing crates. crates.io ignores `user_id` when `keyword` is
/// set.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CrateListQuery {
    /// Results are sorted by relevance with a search and alphabetically
    /// otherwise. Only the first 1000 results of a search can be listed.
    pub search: Option<String>,
    /// Also matches crates in subcategories.
    pub category: Option<String>,
    pub keyword: Option<String>,
    pub user_id: Option<UserId>,
    /// Excludes crates whose versions are all yanked.
    pub exclude_yanked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crate {
    pub name: CrateName,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub downloads: u64,
    /// Downloads in the last 90 days.
    pub recent_downloads: Option<u64>,
    /// The version shown on the crate's page. `None` for crates without a
    /// published version.
    pub default_version: Option<semver::Version>,
    pub num_versions: u32,
    /// Whether all versions are yanked.
    pub yanked: bool,
    pub trustpub_only: bool,
    /// Keyword and category IDs, only included when requesting a single crate.
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub id: VersionId,
    #[serde(rename = "crate")]
    pub crate_name: CrateName,
    pub num: semver::Version,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub downloads: u64,
    pub yanked: bool,
    pub yank_message: Option<String>,
    pub license: Option<String>,
    pub rust_version: Option<String>,
    pub edition: Option<String>,
    /// Size of the compressed crate file in bytes.
    pub crate_size: u64,
    /// Hex-encoded SHA256 of the compressed crate file.
    pub checksum: String,
    /// `None` for versions published before crates.io recorded publishers.
    pub published_by: Option<User>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub login: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: TeamId,
    /// For example `github:zed-industries:crates-io`.
    pub login: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Owner {
    User(User),
    Team(Team),
}

/// Identifies an owner to add or remove.
#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum OwnerLogin {
    /// A crates.io user, or a GitHub user when removing owners. crates.io
    /// rejects names that match different accounts.
    #[display("{_0}")]
    Username(String),
    #[display("crates.io:{_0}")]
    CratesIoUser(String),
    #[display("github:{_0}")]
    GitHubUser(String),
    #[display("github:{organization}:{team}")]
    GitHubTeam { organization: String, team: String },
}

/// The outcome of adding one owner, as crates.io describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnerAddition {
    /// The user becomes an owner once they accept the invitation.
    UserInvited {
        username: String,
    },
    UserAlreadyInvited {
        username: String,
    },
    /// Teams become owners immediately.
    TeamAdded {
        login: String,
    },
    /// A message this client does not recognize.
    Other(String),
}

impl Serialize for OwnerLogin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateDetails {
    pub krate: Crate,
    pub versions: Vec<Version>,
}

/// Crate settings to change. Settings left as `None` keep their value.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CrateUpdate {
    /// Whether the crate can only be published through Trusted Publishing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trustpub_only: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub id: GitHubConfigId,
    #[serde(rename = "crate")]
    pub crate_name: CrateName,
    pub repository_owner: String,
    /// Lets crates.io recognize the repository after its owner is renamed.
    pub repository_owner_id: GitHubAccountId,
    pub repository_name: String,
    pub workflow_filename: String,
    pub environment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NewGitHubConfig {
    #[serde(rename = "crate")]
    pub crate_name: CrateName,
    pub repository_owner: String,
    pub repository_name: String,
    pub workflow_filename: String,
    pub environment: Option<String>,
}
