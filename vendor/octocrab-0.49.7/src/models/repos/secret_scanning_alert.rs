use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningAlert {
    pub number: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub url: Url,
    pub html_url: Url,
    pub locations_url: Url,
    pub state: State,
    pub resolution: Option<Resolution>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<SimpleUser>,
    pub secret_type: String,
    pub secret_type_display_name: String,
    pub secret: String,
    pub push_protection_bypassed_by: Option<SimpleUser>,
    pub push_protection_bypassed: Option<bool>,
    pub push_protection_bypassed_at: Option<DateTime<Utc>>,
    pub resolution_comment: Option<String>,
    pub validity: Validity,
    pub publicly_leaked: Option<bool>,
    pub multi_repo: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Resolved,
    Open,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resolution {
    FalsePositive,
    WontFix,
    Revoked,
    UsedInTests,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Validity {
    Active,
    Inactive,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSecretScanningAlert<'a> {
    pub state: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_comment: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SecretsScanningAlertLocation {
    Commit {
        path: String,
        start_line: u32,
        end_line: u32,
        start_column: u32,
        end_column: u32,
        blob_sha: String,
        blob_url: String,
        commit_sha: String,
        commit_url: String,
    },
    WikiCommit {
        path: String,
        start_line: u32,
        end_line: u32,
        start_column: u32,
        end_column: u32,
        blob_sha: String,
        page_url: String,
        commit_sha: String,
        commit_url: String,
    },
    IssueTitle {
        issue_title_url: String,
    },
    IssueBody {
        issue_body_url: String,
    },
    IssueComment {
        issue_comment_url: String,
    },
    DiscussionTitle {
        discussion_title_url: String,
    },
    DiscussionBody {
        discussion_body_url: String,
    },
    DiscussionComment {
        discussion_comment_url: String,
    },
    PullRequestTitle {
        pull_request_title_url: String,
    },
    PullRequestBody {
        pull_request_body_url: String,
    },
    PullRequestComment {
        pull_request_comment_url: String,
    },
    PullRequestReview {
        pull_request_review_url: String,
    },
    PullRequestReviewComment {
        pull_request_review_comment_url: String,
    },
}
