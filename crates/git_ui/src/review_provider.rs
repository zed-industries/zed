use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReviewProviderKind {
    #[default]
    GitHub,
    GitLab,
}

impl ReviewProviderKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::GitHub => "GitHub",
            Self::GitLab => "GitLab",
        }
    }

    pub(crate) fn request_name(self) -> &'static str {
        match self {
            Self::GitHub => "PR",
            Self::GitLab => "MR",
        }
    }

    pub(crate) fn request_prefix(self) -> &'static str {
        match self {
            Self::GitHub => "#",
            Self::GitLab => "!",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ReviewRepositoryChoice {
    pub provider: ReviewProviderKind,
    pub host: String,
    pub full_name: String,
    pub remote_url: String,
}

impl ReviewRepositoryChoice {
    pub(crate) fn storage_id(&self) -> String {
        format!("{}:{}:{}", self.provider.name(), self.host, self.full_name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReviewProviderIdentity {
    pub kind: ReviewProviderKind,
    pub name: String,
    pub repository: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReviewRequestSummary {
    pub number: u64,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReviewHeader {
    pub number: u64,
    pub title: String,
    pub repository: String,
    pub base_branch: String,
    pub review_branch: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ReviewCheckoutIdentity {
    pub provider: String,
    pub repository_id: String,
    pub review_number: u64,
    pub source_host: String,
    pub source_repository: String,
    pub source_available: bool,
    pub source_branch: String,
    pub target_branch: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ReviewCheckoutAssociation {
    pub identity: ReviewCheckoutIdentity,
    pub last_head_sha: String,
    pub managed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReviewDiffSide {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ReviewThreadId(pub String);

#[derive(Clone, Debug)]
pub(crate) struct ReviewThreadState {
    pub id: ReviewThreadId,
    pub resolved: bool,
    pub outdated: bool,
    pub can_reply: bool,
    pub can_resolve: bool,
    pub can_reopen: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct PlacedReviewComment {
    pub id: String,
    pub thread: ReviewThreadState,
    pub root_comment_id: String,
    pub author: String,
    pub body: String,
    pub provider_name: String,
    pub url: Option<String>,
    pub path: String,
    pub side: ReviewDiffSide,
    pub line: u32,
    pub current: Option<String>,
    pub base: Option<String>,
}

pub(crate) trait ReviewBackend {
    fn identity(&self) -> ReviewProviderIdentity;
    fn request_summaries(&self) -> &[ReviewRequestSummary];
}
