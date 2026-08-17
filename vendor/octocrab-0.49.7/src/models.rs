//! Serde mappings from GitHub's JSON to structs.

use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize};
use url::Url;

use crate::params::users::emails::EmailVisibilityState;
pub use apps::App;

pub mod actions;
pub mod activity;
pub mod apps;
pub mod checks;
pub mod classroom;
pub mod code_scannings;
pub mod codes_of_conduct;
pub mod commits;
pub mod events;
pub mod gists;
pub mod hooks;
pub mod interaction_limits;
pub mod issues;
pub mod orgs;
pub mod orgs_copilot;
pub mod pulls;
pub mod reactions;
pub mod repos;
pub mod teams;
pub mod timelines;
pub mod webhook_events;
pub mod workflows;

mod date_serde;

type BaseIdType = u64;

macro_rules! id_type {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($($name:ident),+) => {$(
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        pub struct $name(pub BaseIdType);
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl Deref for $name {
            type Target = BaseIdType;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl $name {
            pub fn into_inner(self) -> BaseIdType {
                self.0
            }
        }
        impl From<BaseIdType> for $name {
            fn from(value: BaseIdType) -> Self {
                Self(value)
            }
        }
        impl AsRef<BaseIdType> for $name {
            fn as_ref(&self) -> &BaseIdType {
                &self.0
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                struct IdVisitor;
                impl<'de> de::Visitor<'de> for IdVisitor {
                    type Value = $name;
                    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                        where E: de::Error {
                        Ok($name(value))
                    }
                    fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
                        where E: de::Error {
                        id.parse::<u64>().map($name).map_err(de::Error::custom)
                    }
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "expected {} as number or string", stringify!($name)) // TODO: $name
                    }
                }

                deserializer.deserialize_any(IdVisitor)
            }
         }
    )+};
}

id_type!(
    ActorId, // A Bot, EnterpriseUserAccount, Mannequin, Organization or User
    AppId,
    ArtifactId,
    AssetId,
    BranchProtectionRuleId,
    CardId,
    CheckSuiteId,
    CheckRunId,
    CodeScanningId,
    CommentId,
    InstallationId,
    IssueEventId,
    IssueId,
    JobId,
    HookId,
    HookDeliveryId,
    LabelId,
    MilestoneId,
    NotificationId,
    OrgId,
    ProjectId,
    ProjectColumnId,
    PullRequestId,
    PushId,
    ReactionId,
    ReleaseId,
    RepositoryId,
    ReviewId,
    RunId,
    RunnerId,
    RunnerGroupId,
    RunnerLabelId,
    StatusId,
    TeamId,
    TimelineEventId,
    ThreadId,
    UploaderId,
    UserId,
    UserOrOrgId,
    WorkflowId,
    TeamInvitationId,
    AssignmentId,
    ClassroomId
);

macro_rules! convert_into {
    ($($from:ident -> $to:ident),+) => {$(
        impl From<$from> for $to {
            fn from(v: $from) -> $to {
                $to(v.0)
            }
        }
    )+};
}

convert_into!(OrgId -> ActorId,
              UserId -> ActorId,
              OrgId -> UserOrOrgId,
              UserId -> UserOrOrgId,
              PullRequestId -> IssueId);

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Contents {
    #[serde(rename = "type")]
    pub contents_type: String,
    pub encoding: String,
    pub size: u64,
    pub name: String,
    pub path: String,
    pub content: String,
    pub sha: String,
    pub url: Url,
    pub git_url: Url,
    pub html_url: Url,
    pub download_url: Url,
}

/// Issue events are triggered by activity in issues and pull requests.
/// <https://docs.github.com/en/webhooks-and-events/events/issue-event-types>
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Event {
    /// The issue or pull request was added to a project board.
    AddedToProject,
    /// The issue or pull request was added to a project board.
    AddedToProjectV2,
    /// Not documented in the Github issue events documentation.
    AddedToMergeQueue,
    /// The issue or pull request was assigned to a user.
    Assigned,
    /// Auto merge was disabled for a pull request.
    AutoMergeDisabled,
    /// Auto merge was enabled for a pull request.
    AutoMergeEnabled,
    /// GitHub unsuccessfully attempted to automatically change the base branch of the pull request.
    AutomaticBaseChangeFailed,
    /// GitHub successfully attempted to automatically change the base branch of the pull request.
    AutomaticBaseChangeSucceeded,
    /// Auto Rebase Enable
    AutoRebaseEnabled,
    /// Auto Squash Enable
    AutoSquashEnabled,
    /// The base reference branch of the pull request changed.
    BaseRefChanged,
    /// Not documented in the Github issue events documentation.
    BaseRefDeleted,
    /// Not documented in the Github issue events documentation.
    BaseRefForcePushed,
    /// The issue or pull request was blocked by another issue or pull request. This is not documented in the Github issue events documentation, but has been observed in the wild.
    BlockingAdded,
    /// The issue or pull request was closed. When the commit_id is present, it identifies the commit that closed the issue using "closes / fixes" syntax.
    Closed,
    /// A comment was added to the issue or pull request.
    Commented,
    /// A comment that was removed from the issue or pull request.
    /// This isn't documented as part of the issues-event-types API but returned by the API.
    CommentDeleted,
    /// A commit was added to the pull request's HEAD branch.
    Committed,
    /// The issue or pull request was linked to another issue or pull request.
    Connected,
    /// The pull request was converted to draft mode.
    ConvertToDraft,
    /// The issue was created by converting a note in a project board to an issue.
    ConvertedNoteToIssue,
    /// The issue was closed and converted to a discussion.
    ConvertedToDiscussion,
    /// Copilot started working on this pull request. This is undocumented, but has been observed in the wild.
    CopilotWorkStarted,
    /// Copilot finished working on this pull request. This is undocumented, but has been observed in the wild.
    CopilotWorkFinished,
    /// The issue or pull request was referenced from another issue or pull request.
    #[serde(rename = "cross-referenced")]
    CrossReferenced,
    /// The issue or pull request was removed from a milestone.
    Demilestoned,
    /// The pull request was deployed.
    Deployed,
    /// The pull request deployment environment was changed.
    DeploymentEnvironmentChanged,
    /// The issue or pull request was unlinked from another issue or pull request.
    Disconnected,
    /// The pull request's HEAD branch was deleted.
    HeadRefDeleted,
    /// The pull request's HEAD branch was force pushed.
    HeadRefForcePushed,
    /// The pull request's HEAD branch was restored to the last known commit.
    HeadRefRestored,
    /// A label was added to the issue or pull request.
    Labeled,
    /// A comment on a line of source in a pull request. Not documented in the issue and events documentation.
    #[serde(rename = "line-commented")]
    LineCommented,
    /// The issue or pull request was locked.
    Locked,
    /// The actor was @mentioned in an issue or pull request body.
    Mentioned,
    /// A user with write permissions marked an issue as a duplicate of another issue, or a pull request as a duplicate of another pull request.
    MarkedAsDuplicate,
    /// The pull request was merged. The commit_id attribute is the SHA1 of the HEAD commit that was merged. The commit_repository is always the same as the main repository.
    Merged,
    /// The issue or pull request was added to a milestone.
    Milestoned,
    /// The issue or pull request was moved between columns in a project board.
    MovedColumnsInProject,
    /// The issue or pull request was marked as sub issue of another issue. This is not documented in the Github issue events documentation, but has been observed in the wild.
    ParentIssueAdded,
    /// The issue or pull request was removed as a sub issue of another issue. This is not documented in the Github issue events documentation, but has been observed in the wild.
    ParentIssueRemoved,
    /// The issue was pinned.
    Pinned,
    /// Not documented in the Github issue events documentation.
    ProjectV2ItemStatusChanged,
    /// Not documented in the Github issue events documentation.
    IssueTypeAdded,
    /// Not documented in the Github issue events documentation.
    IssueTypeRemoved,
    /// A draft pull request was marked as ready for review.
    ReadyForReview,
    /// The issue was referenced from a commit message. The commit_id attribute is the commit SHA1 of where that happened and the commit_repository is where that commit was pushed.
    Referenced,
    /// Not documented in the Github issue events documentation.
    RemovedFromMergeQueue,
    /// The issue or pull request was removed from a project board.
    RemovedFromProject,
    /// Not documented in the Github issue events documentation.
    RemovedFromProjectV2,
    /// The issue or pull request title was changed.
    Renamed,
    /// The issue or pull request was reopened.
    Reopened,
    /// The pull request review was dismissed.
    ReviewDismissed,
    /// A pull request review was requested.
    ReviewRequested,
    /// A pull request review request was removed.
    ReviewRequestRemoved,
    /// The pull request was reviewed.
    Reviewed,
    /// Some issue or pull request was added as a sub issue of this issue. This is not documented in the Github issue events documentation, but has been observed in the wild.
    SubIssueAdded,
    /// Some issue or pull request was removed as a sub issue of this issue. This is not documented in the Github issue events documentation, but has been observed in the wild.
    SubIssueRemoved,
    /// Someone subscribed to receive notifications for an issue or pull request.
    Subscribed,
    /// The issue was transferred to another repository.
    Transferred,
    /// A user was unassigned from the issue.
    Unassigned,
    /// A label was removed from the issue.
    Unlabeled,
    /// The issue was unlocked.
    Unlocked,
    /// An issue that a user had previously marked as a duplicate of another issue is no longer considered a duplicate, or a pull request that a user had previously marked as a duplicate of another pull request is no longer considered a duplicate.
    UnmarkedAsDuplicate,
    /// The issue was unpinned.
    Unpinned,
    /// Someone unsubscribed from receiving notifications for an issue or pull request.
    Unsubscribed,
    /// An organization owner blocked a user from the organization.
    UserBlocked,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum IssueState {
    Open,
    Closed,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<IssueEventId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub actor: Author,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_card: Option<ProjectCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectCard {
    pub id: CardId,
    pub url: Url,
    pub project_id: ProjectId,
    pub project_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_column_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_url: Option<Url>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Project {
    pub owner_url: Url,
    pub url: Url,
    pub html_url: Url,
    pub columns_url: Url,
    pub id: ProjectId,
    pub node_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub creator: Author,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ProjectCardContentType {
    Issue,
    PullRequest,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectColumn {
    pub url: Url,
    pub project_url: Url,
    pub cards_url: Url,
    pub id: ProjectColumnId,
    pub node_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuePullRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Author {
    pub login: String,
    pub id: UserId,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
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
    pub name: Option<String>,
    pub patch_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// If a string is empty then deserialize it as none
fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // try to deserialize our input string
    let cast = String::deserialize(deserializer)?;
    // if this string is empty then return None
    if cast.is_empty() {
        Ok(None)
    } else {
        Ok(Some(cast))
    }
}

/// The full profile for a user
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserProfile {
    pub login: String,
    pub id: UserId,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
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
    pub name: Option<String>,
    pub company: Option<String>,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub blog: Option<String>,
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: u64,
    pub public_gists: u64,
    pub followers: u64,
    pub following: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data for updating a user profile,
/// e.g. name, email, company, location, bio, blog, twitter, hireable
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateUserProfile {
    pub name: Option<String>,
    pub email: Option<String>,
    pub blog: Option<String>,
    pub twitter_username: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
}

/// The simple profile for a GitHub user
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SimpleUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub login: String,
    pub id: UserId,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
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
    pub starred_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ItemGitUser {
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}

/// A user that is following another user
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Follower {
    pub login: String,
    pub id: UserId,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
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
    pub patch_url: Option<String>,
}

/// A user that is being followed by another user
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Followee {
    pub login: String,
    pub id: UserId,
    pub node_id: String,
    pub avatar_url: Url,
    pub gravatar_id: String,
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
    pub patch_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AuthorAssociation {
    Collaborator,
    Contributor,
    FirstTimer,
    FirstTimeContributor,
    Mannequin,
    Member,
    None,
    Owner,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Collaborator {
    #[serde(flatten)]
    pub author: Author,
    pub permissions: Permissions,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Contributor {
    #[serde(flatten)]
    pub author: Author,
    pub contributions: u32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StarGazer {
    pub starred_at: Option<DateTime<Utc>>,
    pub user: Option<Author>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Label {
    pub id: LabelId,
    pub node_id: String,
    pub url: Url,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Milestone {
    pub url: Url,
    pub html_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<Url>,
    pub id: MilestoneId,
    pub node_id: String,
    pub number: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_issues: Option<i64>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SquashMergeCommitTitle {
    PrTitle,
    CommitOrPrTitle,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SquashMergeCommitMessage {
    PrBody,
    CommitMessages,
    Blank,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repository {
    pub id: RepositoryId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blobs_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborators_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forks_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commits_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_refs_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_tags_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_comment_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_events_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merges_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulls_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stargazers_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trees_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svn_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forks_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stargazers_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchers_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_issues_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "date_serde::deserialize_opt"
    )]
    pub pushed_at: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "date_serde::deserialize_opt"
    )]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "date_serde::deserialize_opt"
    )]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Box<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title: Option<SquashMergeCommitTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message: Option<SquashMergeCommitMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Repository>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositoryFile {
    pub name: Option<String>,
    pub key: Option<String>,
    pub url: Option<Url>,
    pub html_url: Option<Url>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositoryMetrics {
    pub health_percentage: u64,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub files: HashMap<String, Option<RepositoryFile>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub content_reports_enabled: Option<bool>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    pub html_url: Option<Url>,
    pub description: Option<String>,
    pub implementation: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
    pub limitations: Option<Vec<String>>,
    pub body: Option<String>,
    pub featured: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Code {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: Url,
    pub git_url: Url,
    pub html_url: Url,
    pub repository: Repository,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Permissions {
    #[serde(default)]
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
    #[serde(default)]
    pub triage: bool,
    #[serde(default)]
    pub maintain: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRuns {
    pub total_count: i32,
    pub check_runs: Vec<CheckRun>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CheckRunId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CheckStatus {
    Queued,
    Completed,
    InProgress,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CombinedStatus {
    pub state: StatusState,
    pub sha: String,
    pub total_count: i64,
    pub statuses: Vec<Status>,
    #[serde(skip_serializing)]
    pub repository: Option<Repository>,
    #[serde(skip_serializing)]
    pub commit_url: Option<Url>,
    #[serde(skip_serializing)]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<StatusId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub state: StatusState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StatusState {
    Failure,
    Pending,
    Success,
    Error,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationRepositories {
    pub total_count: i64,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct Installation {
    pub id: InstallationId,
    pub account: Author,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_tokens_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<AppId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<UserOrOrgId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    pub permissions: InstallationPermissions,
    /// List of events in the installation.
    ///
    /// Note that for Webhook events, the list of events in the
    /// list is guaranteed to match variants from
    /// [WebhookEventType](webhook_events::WebhookEventType)
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "date_serde::deserialize_opt"
    )]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "date_serde::deserialize_opt"
    )]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct InstallationPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct InstallationToken {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub permissions: InstallationPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<Repository>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct PublicKey {
    pub key_id: String,
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimit {
    pub resources: Resources,
    pub rate: Rate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resources {
    pub core: Rate,
    pub search: Rate,
    pub graphql: Option<Rate>,
    pub integration_manifest: Option<Rate>,
    pub scim: Option<Rate>,
    pub source_import: Option<Rate>,
    pub code_scanning_upload: Option<Rate>,
    pub actions_runner_registration: Option<Rate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rate {
    pub limit: usize,
    pub used: usize,
    pub remaining: usize,
    pub reset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEmailInfo {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
    pub visibility: Option<EmailVisibilityState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedEmailInfo {
    pub email: String,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubKeyInfo {
    pub id: u64,
    pub primary_key_id: u64,
    pub key_id: String,
    pub public_key: String,
    pub emails: Vec<VerifiedEmailInfo>,
    pub subkeys: Option<Vec<SubKeyInfo>>,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_key: Option<String>,
    pub revoked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpgKey {
    pub id: u64,
    pub name: String,
    pub primary_key_id: u64,
    pub key_id: String,
    pub public_key: String,
    pub emails: Vec<VerifiedEmailInfo>,
    pub subkeys: Vec<SubKeyInfo>,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: DateTime<Utc>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "date_serde::deserialize_opt"
    )]
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSshKey {
    pub key: String,
    pub id: u64,
    pub url: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub verified: bool,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAccount {
    pub provider: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshSigningKey {
    pub key: String,
    pub id: u64,
    pub title: String,
    pub created_at: DateTime<Utc>,
}
