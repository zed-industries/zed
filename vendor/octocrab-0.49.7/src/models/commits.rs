use crate::models::{self, codes_of_conduct::CodeOfConduct};

use super::{reactions::ReactionContent, *};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    // TODO check actuality comparing with github json schema and pulls::ReviewComment
    pub html_url: Url,
    pub url: Url,
    pub id: CommentId,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u64>,
    pub commit_id: String,
    pub user: Author,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<CommentReactions>,
}

/// Reactions summary of a comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct CommentReactions {
    pub url: Url,
    pub total_count: u64,
    #[serde(flatten)]
    pub reactions: Option<HashMap<ReactionContent, u64>>,
}

/// Commit Comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitComparison {
    pub ahead_by: i64,
    /// Commit
    pub base_commit: Commit,
    pub behind_by: i64,
    pub commits: Vec<Commit>,
    pub diff_url: String,
    pub files: Option<Vec<repos::DiffEntry>>,
    pub html_url: String,
    /// Commit
    pub merge_base_commit: Commit,
    pub patch_url: String,
    pub permalink_url: String,
    pub status: GithubCommitStatus,
    pub total_commits: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitElement {
    pub author: Option<GitUser>,
    pub comment_count: i64,
    pub committer: Option<GitUser>,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub verification: Option<Verification>,
}

/// Metaproperties for Git author/committer information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitUser {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Verification {
    pub payload: Option<String>,
    pub reason: String,
    pub signature: Option<String>,
    pub verified: bool,
}

#[deprecated(note = "use repos::DiffEntryStatus instead")]
pub type FileStatus = repos::DiffEntryStatus;

/// Commit
#[deprecated(note = "use repos::DiffEntry instead")]
pub type CommitFile = repos::DiffEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitParent {
    pub html_url: Option<String>,
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitStats {
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    pub total: Option<i64>,
}

/// Commit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Commit {
    pub author: Option<SimpleUser>,
    pub comments_url: Url,
    pub commit: CommitElement,
    pub committer: Option<ItemGitUser>,
    pub html_url: Url,
    pub node_id: String,
    pub parents: Vec<CommitParent>,
    /// Minimal Repository
    pub repository: MinimalRepository,
    pub score: f64,
    pub sha: String,
    pub text_matches: Option<Vec<SearchResultTextMatch>>,
    pub url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MinimalRepository {
    pub allow_forking: Option<bool>,
    pub archive_url: Url,
    pub archived: Option<bool>,
    pub assignees_url: String,
    pub blobs_url: Url,
    pub branches_url: String,
    pub clone_url: Option<String>,
    /// Code Of Conduct
    pub code_of_conduct: Option<CodeOfConduct>,
    pub collaborators_url: String,
    pub comments_url: Url,
    pub commits_url: Url,
    pub compare_url: Url,
    pub contents_url: Url,
    pub contributors_url: Url,
    pub created_at: Option<String>,
    /// The custom properties that were defined for the repository. The keys are the custom
    /// property names, and the values are the corresponding custom property values.
    pub custom_properties: Option<HashMap<String, Option<serde_json::Value>>>,
    pub default_branch: Option<String>,
    pub delete_branch_on_merge: Option<bool>,
    pub deployments_url: String,
    pub description: Option<String>,
    pub disabled: Option<bool>,
    pub downloads_url: Url,
    pub events_url: Url,
    pub fork: bool,
    pub forks: Option<i64>,
    pub forks_count: Option<i64>,
    pub forks_url: Url,
    pub full_name: String,
    pub git_commits_url: Url,
    pub git_refs_url: Url,
    pub git_tags_url: Url,
    pub git_url: Option<String>,
    pub has_discussions: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_pages: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_wiki: Option<bool>,
    pub homepage: Option<String>,
    pub hooks_url: Url,
    pub html_url: Url,
    pub id: i64,
    pub is_template: Option<bool>,
    pub issue_comment_url: Url,
    pub issue_events_url: Url,
    pub issues_url: Url,
    pub keys_url: Url,
    pub labels_url: Url,
    pub language: Option<String>,
    pub languages_url: Url,
    pub license: Option<License>,
    pub merges_url: Url,
    pub milestones_url: Url,
    pub mirror_url: Option<String>,
    pub name: String,
    pub network_count: Option<i64>,
    pub node_id: String,
    pub notifications_url: Url,
    pub open_issues: Option<i64>,
    pub open_issues_count: Option<i64>,
    /// A GitHub user.
    pub owner: SimpleUser,
    pub permissions: Option<Permissions>,
    pub private: bool,
    pub pulls_url: Url,
    pub pushed_at: Option<String>,
    pub releases_url: Url,
    pub role_name: Option<String>,
    pub security_and_analysis: Option<SecurityAndAnalysis>,
    /// The size of the repository, in kilobytes. Size is calculated hourly. When a repository is
    /// initially created, the size is 0.
    pub size: Option<i64>,
    pub ssh_url: Option<String>,
    pub stargazers_count: Option<i64>,
    pub stargazers_url: Url,
    pub statuses_url: Url,
    pub subscribers_count: Option<i64>,
    pub subscribers_url: Url,
    pub subscription_url: Url,
    pub svn_url: Option<String>,
    pub tags_url: Url,
    pub teams_url: Url,
    pub temp_clone_token: Option<String>,
    pub topics: Option<Vec<String>>,
    pub trees_url: Url,
    pub updated_at: Option<String>,
    pub url: Url,
    pub visibility: Option<String>,
    pub watchers: Option<i64>,
    pub watchers_count: Option<i64>,
    pub web_commit_signoff_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityAndAnalysis {
    /// Enable or disable GitHub Advanced Security for the repository.
    ///
    /// For standalone Code Scanning or Secret Protection products, this parameter cannot be used.
    pub advanced_security: Option<AdvancedSecurity>,
    pub code_security: Option<CodeSecurity>,
    /// Enable or disable Dependabot security updates for the repository.
    pub dependabot_security_updates: Option<DependabotSecurityUpdates>,
    pub secret_scanning: Option<SecretScanning>,
    pub secret_scanning_ai_detection: Option<SecretScanningAiDetection>,
    pub secret_scanning_non_provider_patterns: Option<SecretScanningNonProviderPatterns>,
    pub secret_scanning_push_protection: Option<SecretScanningPushProtection>,
}

/// Enable or disable GitHub Advanced Security for the repository.
///
/// For standalone Code Scanning or Secret Protection products, this parameter cannot be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AdvancedSecurity {
    pub status: Option<AdvSecStatus>,
}

/// The enablement status of Dependabot security updates for the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AdvSecStatus {
    Disabled,
    Enabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CodeSecurity {
    pub status: Option<AdvSecStatus>,
}

/// Enable or disable Dependabot security updates for the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DependabotSecurityUpdates {
    /// The enablement status of Dependabot security updates for the repository.
    pub status: Option<AdvSecStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanning {
    pub status: Option<AdvSecStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningAiDetection {
    pub status: Option<AdvSecStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningNonProviderPatterns {
    pub status: Option<AdvSecStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecretScanningPushProtection {
    pub status: Option<AdvSecStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SearchResultTextMatch {
    pub fragment: Option<String>,
    pub matches: Option<Vec<Match>>,
    pub object_type: Option<String>,
    pub object_url: Option<String>,
    pub property: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Match {
    pub indices: Option<Vec<i64>>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GithubCommitStatus {
    Ahead,
    Behind,
    Diverged,
    Identical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitCommitObject {
    pub sha: String,
    pub node_id: String,
    pub url: String,
    pub author: models::repos::CommitAuthor,
    pub committer: repos::CommitAuthor,
    pub message: String,
    pub tree: models::repos::CommitObject,
    pub parents: Vec<models::repos::Commit>,
    pub verification: models::repos::Verification,
    pub html_url: String,
}
