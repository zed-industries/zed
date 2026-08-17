use super::*;
use crate::models::commits::CommentReactions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequest {
    pub url: String,
    pub id: PullRequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_url: Option<Url>,
    /// The pull request number.  Note that GitHub's REST API
    /// considers every pull-request an issue with the same number.
    pub number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<IssueState>,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub maintainer_can_modify: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Box<Milestone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<MergeableState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<Box<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_reviewers: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_teams: Option<Vec<teams::RequestedTeam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub head: Box<Head>,
    pub base: Box<Base>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<Links>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Box<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Head {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Links {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<SelfLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<HtmlLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_link: Option<IssueLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_link: Option<CommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments_link: Option<ReviewCommentsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment_link: Option<ReviewCommentLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_link: Option<CommitsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_link: Option<StatusesLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pull_request")]
    pub pull_request_link: Option<PullRequestLink>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HtmlLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewCommentLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommitsLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusesLink {
    pub href: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PullRequestLink {
    pub href: Url,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Review {
    pub id: ReviewId,
    pub node_id: String,
    pub html_url: Url,
    pub user: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ReviewState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum ReviewState {
    Open,
    Approved,
    Pending,
    ChangesRequested,
    Commented,
    Dismissed,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum ReviewAction {
    Approve,
    RequestChanges,
    Comment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    pub url: Url,
    pub pull_request_review_id: Option<ReviewId>,
    pub id: CommentId,
    pub node_id: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: Option<u64>,
    pub original_position: Option<u64>,
    pub commit_id: String,
    pub original_commit_id: String,
    #[serde(default)]
    pub in_reply_to_id: Option<CommentId>,
    pub user: Option<Author>,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(rename = "_links")]
    pub links: Links,
    pub start_line: Option<u64>,
    pub original_start_line: Option<u64>,
    pub start_side: Option<String>,
    pub line: Option<u64>,
    pub original_line: Option<u64>,
    pub side: Option<String>,
}

///Legacy Review Comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReviewComment {
    pub url: Url,
    pub pull_request_review_id: Option<ReviewId>,
    pub id: CommentId,
    pub node_id: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: Option<u64>,
    pub original_position: Option<u64>,
    pub commit_id: String,
    pub original_commit_id: String,
    #[serde(default)]
    pub in_reply_to_id: Option<CommentId>,
    pub user: Option<Author>,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
    pub pull_request_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<CommentReactions>,
    pub side: Option<Side>,
    pub start_side: Option<Side>,
    pub line: Option<u64>,
    pub original_line: Option<u64>,
    pub start_line: Option<u64>,
    pub original_start_line: Option<u64>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
#[non_exhaustive]
pub enum Side {
    Left,
    Right,
}

/// A Thread in a pull request review
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Thread {
    pub comments: Vec<Comment>,
    pub node_id: String,
}

// This is rather annoying, but Github uses both SCREAMING_SNAKE_CASE and snake_case
// for the review state, it's uppercase when coming from an API request, but
// lowercase when coming from a webhook payload, so we need to deserialize both,
// but still use uppercase for serialization
impl<'de> Deserialize<'de> for ReviewState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = ReviewState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match value {
                    "OPEN" | "open" => ReviewState::Open,
                    "APPROVED" | "approved" => ReviewState::Approved,
                    "PENDING" | "pending" => ReviewState::Pending,
                    "CHANGES_REQUESTED" | "changes_requested" => ReviewState::ChangesRequested,
                    "COMMENTED" | "commented" => ReviewState::Commented,
                    "DISMISSED" | "dismissed" => ReviewState::Dismissed,
                    unknown => return Err(E::custom(format!("unknown variant `{unknown}`, expected one of `open`, `approved`, `pending`, `changes_requested`, `commented`, `dismissed`"))),
                })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

//same, see above
impl<'de> Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Side;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match value.to_uppercase().as_str() {
                    "LEFT" => Side::Left,
                    "RIGHT" => Side::Right,
                    unknown => {
                        return Err(E::custom(format!(
                            "unknown variant `{unknown}`, expected one of `left`, `right`"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestReviewAction {
    Submitted,
    Edited,
    Dismissed,
}

/// The complete list of actions that can trigger the sending of a
/// `pull_request` webhook
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PullRequestAction {
    Opened,
    Edited,
    Closed,
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    ReadyForReview,
    Labeled,
    Unlabeled,
    Synchronize,
    Locked,
    Unlocked,
    Reopened,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Merge {
    pub sha: Option<String>,
    pub message: Option<String>,
    pub merged: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MergeableState {
    /// The head ref is out of date.
    Behind,
    /// The merge is blocked, eg. the base branch is protected by a required
    /// status check that is pending
    Blocked,
    /// Mergeable and passing commit status.
    Clean,
    /// The merge commit cannot be cleanly created.
    Dirty,
    /// The merge is blocked due to the pull request being a draft.
    Draft,
    /// Mergeable with passing commit status and pre-receive hooks.
    HasHooks,
    /// The state cannot currently be determined.
    Unknown,
    /// Mergeable with non-passing commit status.
    Unstable,
}

#[deprecated(note = "use repos::DiffEntry instead")]
pub type FileDiff = repos::DiffEntry;

#[deprecated(note = "use repos::DiffEntryStatus instead")]
pub type FileDiffStatus = repos::DiffEntryStatus;

#[cfg(test)]
mod test {
    #[test]
    fn deserializes_review_state() {
        use super::ReviewState;

        let states: Vec<ReviewState> = serde_json::from_str(
            r#"["APPROVED","pending","CHANGES_REQUESTED","commented", "dismissed"]"#,
        )
        .unwrap();

        assert_eq!(
            states,
            &[
                ReviewState::Approved,
                ReviewState::Pending,
                ReviewState::ChangesRequested,
                ReviewState::Commented,
                ReviewState::Dismissed,
            ]
        );
    }
}
