#![allow(clippy::needless_update)]

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};

use crate::is_default;

/// Represents all possible webhook events that can trigger a workflow
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows
#[derive(Default, Debug, Clone, Deserialize, Serialize, Merge, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_protection_rule: Option<BranchProtectionRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_run: Option<CheckRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_suite: Option<CheckSuite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<Create>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Delete>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<DeploymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion: Option<Discussion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_comment: Option<DiscussionComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gollum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_comment: Option<IssueComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Issues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_group: Option<MergeGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_review: Option<PullRequestReview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_review_comment: Option<PullRequestReviewComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_target: Option<PullRequestTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<Push>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_package: Option<RegistryPackage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<Release>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_dispatch: Option<RepositoryDispatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Vec<Schedule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<Watch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_call: Option<WorkflowCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_dispatch: Option<WorkflowDispatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run: Option<WorkflowRun>,
}

impl Event {
    pub fn add_schedule(mut self, schedule: impl Into<Schedule>) -> Self {
        if let Some(list) = self.schedule.as_mut() {
            list.push(schedule.into());
        } else {
            self.schedule = Some(vec![schedule.into()])
        }

        self
    }

    pub fn add_cron_schedule(self, cron: impl ToString) -> Self {
        self.add_schedule(Schedule::new(cron))
    }
}

/// Types of branch protection rule events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#branch_protection_rule
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionRuleType {
    /// A branch protection rule was created
    Created,
    /// A branch protection rule was edited
    Edited,
    /// A branch protection rule was deleted
    Deleted,
}

/// Configuration for branch protection rule events
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct BranchProtectionRule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<BranchProtectionRuleType>,
}

impl BranchProtectionRule {
    /// Adds a branch protection rule event type to filter on
    pub fn add_type(mut self, type_: BranchProtectionRuleType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Types of check run events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#check_run
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunType {
    /// A check run was created
    Created,
    /// A check run was requested to be re-run
    Rerequested,
    /// A check run was completed
    Completed,
    /// A user requested an action from the check run
    RequestedAction,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct CheckRun {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<CheckRunType>,
}

impl CheckRun {
    /// Adds a check run event type to filter on
    pub fn add_type(mut self, type_: CheckRunType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Types of check suite events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#check_suite
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteType {
    /// A check suite has completed
    Completed,
}

/// Configuration for check suite events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct CheckSuite {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<CheckSuiteType>,
}

impl CheckSuite {
    /// Adds a check suite event type to filter on
    pub fn add_type(mut self, type_: CheckSuiteType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Configuration for create events (branch or tag creation)
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#create
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Create {
    /// Filter on specific branch names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    /// Filter on specific tag names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Create {
    /// Adds a branch name to filter on
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    /// Adds a tag name to filter on
    pub fn add_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// Configuration for delete events (branch or tag deletion)
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#delete
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Delete {
    /// Filter on specific branch names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    /// Filter on specific tag names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Delete {
    /// Adds a branch name to filter on
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    /// Adds a tag name to filter on
    pub fn add_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// Types of deployment events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#deployment

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Deployment {
    /// Filter on specific branch names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl Deployment {
    /// Adds a branch name to filter on
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }
}

/// Types of deployment status events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#deployment_status

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct DeploymentStatus {
    /// Filter on specific deployment states
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<String>,
}

impl DeploymentStatus {
    /// Adds a deployment state to filter on
    pub fn add_state<S: Into<String>>(mut self, state: S) -> Self {
        self.states.push(state.into());
        self
    }
}

/// Types of discussion events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#discussion
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionType {
    /// A discussion was created
    Created,
    /// A discussion was edited
    Edited,
    /// A discussion was deleted
    Deleted,
    /// A discussion was transferred between repositories
    Transferred,
    /// A discussion was pinned
    Pinned,
    /// A discussion was unpinned
    Unpinned,
    /// A discussion was labeled
    Labeled,
    /// A discussion was unlabeled
    Unlabeled,
    /// A discussion was locked
    Locked,
    /// A discussion was unlocked
    Unlocked,
    /// A discussion's category was changed
    CategoryChanged,
    /// A discussion was marked as answered
    Answered,
    /// A discussion was unmarked as answered
    Unanswered,
}

/// Configuration for discussion events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
pub struct Discussion {
    /// Filter on specific discussion event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<DiscussionType>,
}

impl Discussion {
    /// Adds a discussion event type to filter on
    pub fn add_type(mut self, type_: DiscussionType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Types of discussion comment events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#discussion_comment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionCommentType {
    /// A discussion comment was created
    Created,
    /// A discussion comment was edited
    Edited,
    /// A discussion comment was deleted
    Deleted,
}

/// Configuration for discussion comment events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
pub struct DiscussionComment {
    /// Filter on specific discussion comment event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<DiscussionCommentType>,
}

impl DiscussionComment {
    /// Adds a discussion comment event type to filter on
    pub fn add_type(mut self, type_: DiscussionCommentType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Configuration for issue comment events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#issue_comment
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
pub struct IssueComment {
    /// Filter on specific issue comment event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<IssueCommentType>,
}

/// Types of issue comment events
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentType {
    /// An issue comment was created
    Created,
    /// An issue comment was edited
    Edited,
    /// An issue comment was deleted
    Deleted,
}

impl IssueComment {
    /// Adds an issue comment event type to filter on
    pub fn add_type(mut self, type_: IssueCommentType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Types of issue events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuesType {
    /// An issue was opened
    Opened,
    /// An issue was edited
    Edited,
    /// An issue was deleted
    Deleted,
    /// An issue was transferred between repositories
    Transferred,
    /// An issue was pinned
    Pinned,
    /// An issue was unpinned
    Unpinned,
    /// An issue was closed
    Closed,
    /// A closed issue was reopened
    Reopened,
    /// An issue was assigned to a user
    Assigned,
    /// An issue was unassigned from a user
    Unassigned,
    /// A label was added to an issue
    Labeled,
    /// A label was removed from an issue
    Unlabeled,
    /// An issue was locked
    Locked,
    /// An issue was unlocked
    Unlocked,
    /// An issue was added to a milestone
    Milestoned,
    /// An issue was removed from a milestone
    Demilestoned,
}

/// Configuration for issue events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Issues {
    /// Filter on specific issue event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<IssuesType>,
}

impl Issues {
    /// Adds an issue event type to filter on
    pub fn add_type(mut self, type_: IssuesType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Types of label events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#label
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelType {
    /// A label was created
    Created,
    /// A label was edited
    Edited,
    /// A label was deleted
    Deleted,
}

/// Configuration for label events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Label {
    /// Filter on specific label event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<LabelType>,
}

impl Label {
    /// Adds a label event type to filter on
    pub fn add_type(mut self, type_: LabelType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeGroupType {
    ChecksRequested,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct MergeGroup {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<MergeGroupType>,
}

impl MergeGroup {
    pub fn add_type(mut self, type_: MergeGroupType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneType {
    Created,
    Closed,
    Opened,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Milestone {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<MilestoneType>,
}

impl Milestone {
    pub fn add_type(mut self, type_: MilestoneType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestType {
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Opened,
    Edited,
    Closed,
    Reopened,
    Synchronize,
    ReadyForReview,
    Locked,
    Unlocked,
    ReviewRequested,
    ReviewRequestRemoved,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct PullRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
    /// Ignore specific file paths
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths_ignore: Vec<String>,
}

impl PullRequest {
    pub fn add_type(mut self, type_: PullRequestType) -> Self {
        self.types.push(type_);
        self
    }

    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    pub fn add_path<S: Into<String>>(mut self, path: S) -> Self {
        self.paths.push(path.into());
        self
    }

    pub fn add_ignored_path<S: Into<String>>(mut self, path: S) -> Self {
        self.paths_ignore.push(path.into());
        self
    }
}

/// Types of pull request review events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#pull_request_review
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewType {
    /// A review was submitted for a pull request
    Submitted,
    /// A review was edited
    Edited,
    /// A review was dismissed
    Dismissed,
}

/// Configuration for pull request review events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct PullRequestReview {
    /// Filter on specific pull request review event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestReviewType>,
}

impl PullRequestReview {
    /// Adds a pull request review event type to filter on
    pub fn add_type(mut self, type_: PullRequestReviewType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Types of pull request review comment events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#pull_request_review_comment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewCommentType {
    /// A review comment was created
    Created,
    /// A review comment was edited
    Edited,
    /// A review comment was deleted
    Deleted,
}

/// Configuration for pull request review comment events
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct PullRequestReviewComment {
    /// Filter on specific pull request review comment event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestReviewCommentType>,
}

impl PullRequestReviewComment {
    /// Adds a pull request review comment event type to filter on
    pub fn add_type(mut self, type_: PullRequestReviewCommentType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Configuration for pull request target events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#pull_request_target
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct PullRequestTarget {
    /// Filter on specific pull request event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<PullRequestType>,
    /// Filter on specific branch names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl PullRequestTarget {
    /// Adds a pull request event type to filter on
    pub fn add_type(mut self, type_: PullRequestType) -> Self {
        self.types.push(type_);
        self
    }

    /// Adds a branch name to filter on
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }
}

/// Configuration for push events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#push
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Push {
    /// Filter on specific branch names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
    /// Filter on specific file paths
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
    /// Ignore specific file paths
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths_ignore: Vec<String>,
    /// Filter on specific tags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Push {
    /// Adds a branch name to filter on
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }

    /// Adds a file path to filter on
    pub fn add_path<S: Into<String>>(mut self, path: S) -> Self {
        self.paths.push(path.into());
        self
    }

    /// Adds a file path to not trigger on
    pub fn add_ignored_path<S: Into<String>>(mut self, path: S) -> Self {
        self.paths_ignore.push(path.into());
        self
    }

    /// Adds a tag name to filter on
    pub fn add_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// Types of registry package events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#registry_package
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryPackageType {
    /// A package was published
    Published,
    /// A package was updated
    Updated,
}

/// Configuration for registry package events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct RegistryPackage {
    /// Filter on specific registry package event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<RegistryPackageType>,
}

impl RegistryPackage {
    /// Adds a registry package event type to filter on
    pub fn add_type(mut self, type_: RegistryPackageType) -> Self {
        self.types.push(type_);
        self
    }
}

/// Types of release events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#release
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseType {
    /// A release was published
    Published,
    /// A release was unpublished
    Unpublished,
    /// A release was created
    Created,
    /// A release was edited
    Edited,
    /// A release was deleted
    Deleted,
    /// A release was marked as a pre-release
    Prereleased,
    /// A release was released
    Released,
}

/// Configuration for release events

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Release {
    /// Filter on specific release event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<ReleaseType>,
}

impl Release {
    /// Adds a release event type to filter on
    pub fn add_type(mut self, type_: ReleaseType) -> Self {
        self.types.push(type_);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct RepositoryDispatch {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
}

impl RepositoryDispatch {
    pub fn add_type<S: Into<String>>(mut self, type_: S) -> Self {
        self.types.push(type_.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Schedule {
    pub cron: String,
}

impl Schedule {
    pub fn new(cron: impl ToString) -> Self {
        Self { cron: cron.to_string() }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct Watch {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
}

impl Watch {
    pub fn add_type<S: Into<String>>(mut self, type_: S) -> Self {
        self.types.push(type_.into());
        self
    }
}

/// Configuration for workflow call events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#workflow_call
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct WorkflowCall {
    /// Inputs for the workflow call
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub inputs: IndexMap<String, WorkflowCallInput>,
    /// Outputs from the workflow call
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub outputs: IndexMap<String, WorkflowCallOutput>,
    /// Secrets for the workflow call
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub secrets: IndexMap<String, WorkflowCallSecret>,
}

impl WorkflowCall {
    pub fn add_input(mut self, name: impl Into<String>, input: WorkflowCallInput) -> Self {
        self.inputs.insert(name.into(), input);
        self
    }
}

/// Configuration for workflow call input
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Setters, Eq)]
#[setters(strip_option, into)]
pub struct WorkflowCallInput {
    /// Description of the input
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Indicates if the input is required
    #[serde(skip_serializing_if = "is_default")]
    pub required: bool,
    /// Type of the input
    #[serde(rename = "type")]
    pub input_type: String,
    /// Default value for the input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

/// Configuration for workflow call output
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Setters, Eq)]
#[setters(strip_option, into)]
pub struct WorkflowCallOutput {
    /// Description of the output
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Value of the output
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
}

/// Configuration for workflow call secret
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Setters, Eq)]
#[setters(strip_option, into)]
pub struct WorkflowCallSecret {
    /// Description of the secret
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Indicates if the secret is required
    #[serde(skip_serializing_if = "is_default")]
    pub required: bool,
}

/// Configuration for workflow dispatch events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#workflow_dispatch

#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct WorkflowDispatch {
    /// Inputs for the workflow dispatch
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub inputs: IndexMap<String, WorkflowDispatchInput>,
}

impl WorkflowDispatch {
    pub fn add_input<I: ToString>(mut self, input_id: I, input: WorkflowDispatchInput) -> Self {
        self.inputs.insert(input_id.to_string(), input);
        self
    }
}

/// Configuration for workflow dispatch input
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct WorkflowDispatchInput {
    /// Description of the input
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Indicates if the input is required
    #[serde(skip_serializing_if = "is_default")]
    pub required: bool,
    /// Type of the input
    #[serde(rename = "type")]
    pub input_type: String,
    /// Default value for the input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

/// Types of workflow run events
/// See: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#workflow_run
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunType {
    /// A workflow run was completed
    Completed,
    /// A workflow run was requested
    Requested,
    /// A workflow run is in progress
    InProgress,
}

/// Configuration for workflow run events
#[derive(Debug, Clone, Default, Deserialize, Serialize, Setters, PartialEq, Eq)]
#[setters(strip_option, into)]
pub struct WorkflowRun {
    /// Filter on specific workflow run event types
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<WorkflowRunType>,
    /// Filter on specific workflow names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflows: Vec<String>,
    /// Filter on specific branch names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<String>,
}

impl WorkflowRun {
    /// Adds a workflow run event type to filter on
    pub fn add_type(mut self, type_: WorkflowRunType) -> Self {
        self.types.push(type_);
        self
    }

    /// Adds a workflow name to filter on
    pub fn add_workflow<S: Into<String>>(mut self, workflow: S) -> Self {
        self.workflows.push(workflow.into());
        self
    }

    /// Adds a branch name to filter on
    pub fn add_branch<S: Into<String>>(mut self, branch: S) -> Self {
        self.branches.push(branch.into());
        self
    }
}
