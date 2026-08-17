use super::*;
use std::collections::BTreeMap;

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Gist {
    pub comments: u64,
    pub comments_url: Url,
    pub commits_url: Url,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub files: BTreeMap<String, GistFile>,
    pub forks_url: Url,
    pub git_pull_url: Url,
    pub git_push_url: Url,
    pub html_url: Url,
    pub id: String,
    pub node_id: String,
    pub public: bool,
    pub updated_at: DateTime<Utc>,
    pub url: Url,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct GistFile {
    pub content: Option<String>,
    pub filename: String,
    pub language: Option<String>,
    pub r#type: String,
    pub raw_url: Url,
    pub size: u64,
    pub truncated: Option<bool>,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct GistCommit {
    pub user: Option<Author>,
    pub version: String,
    pub committed_at: DateTime<Utc>,
    pub change_status: GistChangeStatus,
    pub url: Url,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct GistChangeStatus {
    pub total: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct GistRevision {
    pub id: String,
    pub node_id: String,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub description: Option<String>,
    pub files: BTreeMap<String, GistFile>,
    pub url: Url,
}
