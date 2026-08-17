use super::*;
use crate::{error::SerdeSnafu, params::teams::Permission};
use bytes::Bytes;
use http_body::Body;
use http_body_util::BodyExt;
use hyper::Response;
use snafu::ResultExt;
use url::Url;

pub mod dependabot;
pub mod secret_scanning_alert;
pub mod secrets;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Ref {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub node_id: String,
    pub url: Url,
    pub object: Object,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Object {
    Commit { sha: String, url: Url },
    Tag { sha: String, url: Url },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoCommit {
    pub url: String,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: RepoCommitPage,
    #[serde(deserialize_with = "maybe_empty::deserialize")]
    pub author: Option<Author>,
    #[serde(deserialize_with = "maybe_empty::deserialize")]
    pub committer: Option<Author>,
    pub parents: Vec<Commit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<RepoChangeStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<DiffEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoCommitPage {
    pub url: Url,
    pub author: Option<CommitAuthor>,
    pub committer: Option<CommitAuthor>,
    pub message: String,
    pub comment_count: u64,
    pub tree: CommitObject,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoVariable {
    pub name: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoVariables {
    pub total_count: i32,
    pub variables: Vec<RepoVariable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub payload: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DiffEntry {
    // unlike the schema online, this can be null if only metadata changed
    pub sha: Option<String>,
    pub filename: String,
    pub status: DiffEntryStatus,
    pub additions: u64,
    pub deletions: u64,
    pub changes: u64,
    // unlike the schema online, this can be null
    pub blob_url: Option<String>,
    // unlike the schema online, this can be null
    pub raw_url: Option<String>,
    // never null
    pub contents_url: Url,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiffEntryStatus {
    Added,
    Removed,
    Modified,
    Renamed,
    Copied,
    Changed,
    Unchanged,
}

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RepoChangeStatus {
    pub total: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Commit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer: Option<CommitAuthor>,
}

/// The author of a commit, identified by its name and email, as well as (optionally) a time and a github username
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitUserTime {
    #[serde(flatten)]
    pub user: CommitAuthor,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// The author of a commit, identified by its name and email.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct FileUpdate {
    pub content: Content,
    pub commit: Commit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct FileDeletion {
    pub content: Option<Content>,
    pub commit: Commit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Content {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub encoding: Option<String>,
    /// File content, Base64 encoded. See also
    /// [Content::decoded_content].
    pub content: Option<String>,
    pub size: i64,
    pub url: String,
    pub html_url: Option<String>,
    pub git_url: Option<String>,
    pub download_url: Option<String>,
    pub r#type: String,
    #[serde(rename = "_links")]
    pub links: ContentLinks,
    pub license: Option<License>,
}

#[derive(Debug, Clone)]
pub struct ContentItems {
    pub items: Vec<Content>,
}

impl ContentItems {
    /// Returns the current set of items, replacing it with an empty Vec.
    pub fn take_items(&mut self) -> Vec<Content> {
        std::mem::take(&mut self.items)
    }
}

impl Content {
    /// Get content of a file from a repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///
    /// let mut content = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .get_content()
    ///     .path("path/to/file")
    ///     .r#ref("main")
    ///     .send()
    ///     .await?;
    /// let contents = content.take_items();
    /// let c = &contents[0];
    /// let decoded_content = c.decoded_content().unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub fn decoded_content(&self) -> Option<String> {
        use base64::Engine;
        self.content.as_ref().map(|c| {
            let mut content = c.as_bytes().to_owned();
            content.retain(|b| !b" \n\t\r\x0b\x0c".contains(b));
            let c = base64::prelude::BASE64_STANDARD.decode(content).unwrap();
            String::from_utf8_lossy(&c).into_owned()
        })
    }
}

#[async_trait::async_trait]
impl crate::FromResponse for ContentItems {
    async fn from_response<B>(response: Response<B>) -> crate::Result<Self>
    where
        B: Body<Data = Bytes, Error = crate::Error> + Send,
    {
        let json: serde_json::Value =
            serde_json::from_slice(response.into_body().collect().await?.to_bytes().as_ref())
                .context(SerdeSnafu)?;

        if json.is_array() {
            Ok(ContentItems {
                items: serde_json::from_value(json).context(crate::error::SerdeSnafu)?,
            })
        } else {
            let items = vec![serde_json::from_value(json).context(crate::error::SerdeSnafu)?];

            Ok(ContentItems { items })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct ContentLinks {
    pub git: Option<Url>,
    pub html: Option<Url>,
    #[serde(rename = "self")]
    pub _self: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Branch {
    pub name: String,
    pub commit: CommitObject,
    pub protected: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Tag {
    pub name: String,
    pub commit: CommitObject,
    pub zipball_url: Url,
    pub tarball_url: Url,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct CommitObject {
    pub sha: String,
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Release {
    pub url: Url,
    pub html_url: Url,
    pub assets_url: Url,
    pub upload_url: String,
    pub tarball_url: Option<Url>,
    pub zipball_url: Option<Url>,
    pub id: ReleaseId,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub author: Option<crate::models::Author>,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct ReleaseNotes {
    pub name: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Asset {
    pub url: Url,
    pub browser_download_url: Url,
    pub id: AssetId,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub digest: Option<String>,
    pub download_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uploader: Option<Uploader>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Uploader {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: UploaderId,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: Option<String>,
    pub url: Url,
    pub html_url: Url,
    pub followers_url: Url,
    pub following_url: Url,
    pub gists_url: Url,
    pub starred_url: Url,
    pub subscriptions_url: Url,
    pub organizations_url: Url,
    pub repos_url: Url,
    pub events_url: Url,
    pub received_events_url: Url,
    pub r#type: String,
    pub site_admin: bool,
    pub starred_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
/// Metadata for a Git tag
pub struct GitTag {
    pub node_id: String,
    /// Name of the tag. Example: v0.0.1
    pub tag: String,
    pub sha: String,
    pub url: Url,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergeCommit {
    pub url: Url,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoPermission {
    /// Provides the legacy base roles of admin, write, read, and none, where the
    /// maintain role is mapped to write and the triage role is mapped to read.
    pub permission: Permission,
    /// Provides the name of the assigned role, including custom roles.
    pub role_name: String,
    pub user: Collaborator,
}

/// A HashMap of languages and the number of bytes of code written in that language.
pub type Languages = std::collections::HashMap<String, i64>;

mod maybe_empty {
    use serde::{Deserialize, Deserializer};

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeEmpty<T> {
        Empty(Empty),
        Something(T),
    }

    impl<T> From<MaybeEmpty<T>> for Option<T> {
        fn from(value: MaybeEmpty<T>) -> Self {
            match value {
                MaybeEmpty::Something(t) => Some(t),
                _ => None,
            }
        }
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Ok(Option::<MaybeEmpty<T>>::deserialize(d)?.and_then(Into::into))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use serde_json::json;

        #[derive(Deserialize, Debug)]
        struct Struct {
            #[serde(deserialize_with = "deserialize")]
            value: Option<u32>,
        }

        #[test]
        fn deserialize_null_to_none() {
            let actual: Struct = serde_json::from_value(json! {
                {
                    "value": null,
                }
            })
            .unwrap();

            assert!(actual.value.is_none());
        }

        #[test]
        fn deserialize_empty_to_none() {
            let actual: Struct = serde_json::from_value(json! {
                {
                    "value": {},
                }
            })
            .unwrap();

            assert!(actual.value.is_none());
        }

        #[test]
        fn deserialize_invalid() {
            serde_json::from_value::<Struct>(json! {
                {
                    "value": "hello world",
                }
            })
            .unwrap_err();
        }
    }
}
