//! Serde mappings from GitHub's Webhook payloads to structs.
//!
//! Github Applications can be written as simple servers (using e.g. Axum), and then
//! deserialize the webhook payloads it receives thanks to the models included in
//! this module.
//!
//! The main entry point is to read the "event type" from the HTTP request header sent by github (at
//! time of writing, the header is named
//! [`X-GitHub-Event`](https://docs.github.com/en/webhooks-and-events/webhooks/webhook-events-and-payloads#delivery-headers))
//! and then pass the value, along with the payload, to [`WebhookEvent::try_from_header_and_body`]
//! which will validate the payload and return a valid [`WebhookEvent`].
//!
//! ```
//! use octocrab::models::{AppId, webhook_events::{WebhookEvent, WebhookEventPayload, WebhookEventType}};
//!
//! let json = r#"{
//!   "zen": "Design for failure.",
//!   "hook_id": 423885699,
//!   "hook": {
//!     "type": "App",
//!     "id": 423885699,
//!     "name": "web",
//!     "active": true,
//!     "events": [
//!       "issues",
//!       "issue_comment",
//!       "meta",
//!       "pull_request",
//!       "pull_request_review",
//!       "pull_request_review_comment",
//!       "pull_request_review_thread",
//!       "repository"
//!     ],
//!     "config": {
//!       "content_type": "json",
//!       "insecure_ssl": "0",
//!       "secret": "********",
//!       "url": "https://smee.io/R"
//!     },
//!     "updated_at": "2023-07-13T09:30:45Z",
//!     "created_at": "2023-07-13T09:30:45Z",
//!     "app_id": 360617,
//!     "deliveries_url": "https://api.github.com/app/hook/deliveries"
//!   }
//! }"#;
//!
//! // Value picked from the `X-GitHub-Event` header
//! let event_name = "ping";
//!
//! let event = WebhookEvent::try_from_header_and_body(event_name, json)?;
//! assert_eq!(event.kind, WebhookEventType::Ping);
//! let WebhookEventPayload::Ping(ping_event) = event.specific else { panic!("checked for event.kind type before unwrapping") };
//! assert_eq!(ping_event.hook.unwrap().app_id.unwrap(), AppId(360617));
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! ### Stability
//!
//! The GitHub API for webhooks is not stable, nor specified [as far as we
//! know](https://github.com/orgs/community/discussions/61453). Therefore, any contribution to
//! provide more precise structures for payloads, with some actual example values to test, is very
//! welcome.
//!
//! This also means you should consider octocrab's support for webhooks in beta state: it is usable,
//! but the API needs to change according to actual usage in order to refine the different payloads
//! received.

pub mod payload;

use super::{orgs::Organization, Author, Installation, InstallationId, Repository, RepositoryId};
use serde::{Deserialize, Serialize};

pub use payload::WebhookEventPayload;

/// A GitHub webhook event.
///
/// The structure is separated in common fields and specific fields, so you can
/// always access the common values without needing to match the exact variant.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WebhookEvent {
    pub sender: Option<Author>,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub installation: Option<EventInstallation>,
    #[serde(skip)]
    pub kind: WebhookEventType,
    #[serde(flatten)]
    pub specific: WebhookEventPayload,
}

impl WebhookEvent {
    /// Deserialize the body of a webhook event according to the category in the header of the request.
    pub fn try_from_header_and_body<B>(header: &str, body: &B) -> Result<Self, serde_json::Error>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        // NOTE: this is inefficient code to simply reuse the code from "derived" serde::Deserialize instead
        // of writing specific deserialization code for the enum.
        let kind = if header.starts_with('"') {
            serde_json::from_str::<WebhookEventType>(header)?
        } else {
            serde_json::from_str::<WebhookEventType>(&format!("\"{header}\""))?
        };

        // Intermediate structure allows to separate the common fields from
        // the event specific one.
        #[derive(Deserialize)]
        struct Intermediate {
            sender: Option<Author>,
            repository: Option<Repository>,
            organization: Option<Organization>,
            installation: Option<EventInstallation>,
            #[serde(flatten)]
            specific: serde_json::Value,
        }

        let Intermediate {
            sender,
            repository,
            organization,
            installation,
            specific,
        } = serde_json::from_slice::<Intermediate>(body.as_ref())?;

        let specific = kind.parse_specific_payload(specific)?;

        Ok(Self {
            sender,
            repository,
            organization,
            installation,
            kind,
            specific,
        })
    }
}

/// Kind of webhook event.
///
/// For all content in quotes, like `see "...."` in the documentation strings, you can search for
/// this exact sentence in the [search bar of GitHub docs](https://docs.github.com/en/search) to go
/// for the specific documentation page. For example,
/// [this](https://docs.github.com/en/search?query=About+code+scanning+alerts) to look for "About
/// code scanning alerts".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    /// This event occurs when there is activity relating to branch protection rules. For more
    /// information, see "About protected branches."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Administration" repository permission.
    BranchProtectionRule,
    /// This event occurs when there is activity relating to a check run. For information about
    /// check runs, see "Getting started with the Checks
    /// API."
    ///
    /// For activity relating to check suites, see the [`CheckSuite`](WebhookEventType::CheckSuite) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Checks" repository permission. To receive the rerequested and requested_action event types,
    /// the app must have at least write-level access for the "Checks" permission. GitHub Apps with
    /// write-level access for the "Checks" permission are automatically subscribed to this webhook
    /// event.
    ///
    /// Repository and organization webhooks only receive payloads for the created and completed
    /// event types in repositories.
    ///
    /// **Note:** The API only looks for pushes in the repository where the check run was created.
    /// Pushes to a branch in a forked repository are not detected and return an empty pull_requests
    /// array and a null value for head_branch in the [payload](WebhookEventPayload::CheckRun).
    CheckRun,
    /// This event occurs when there is activity relating to a check suite. For information about
    /// check suites, see "Getting started with the Checks
    /// API."
    ///
    /// For activity relating to check runs, see the [`CheckRun`](WebhookEventType::CheckRun) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Checks" permission. To receive the requested and rerequested event types, the app must have
    /// at lease write-level access for the "Checks" permission. GitHub Apps with write-level access
    /// for the "Checks" permission are automatically subscribed to this webhook event.
    ///
    /// Repository and organization webhooks only receive payloads for the completed event types in
    /// repositories.
    ///
    /// **Note:** The API only looks for pushes in the repository where the check suite was created.
    /// Pushes to a branch in a forked repository are not detected and return an empty pull_requests
    /// array and a null value for head_branch.
    CheckSuite,
    /// This event occurs when there is activity relating to code scanning alerts in a repository.
    /// For more information, see "About code scanning" and "About code scanning alerts."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Code
    /// scanning alerts" repository permission.
    CodeScanningAlert,
    /// This event occurs when there is activity relating to commit comments. For more information
    /// about commit comments, see "Commenting on a pull request."
    ///
    /// For activity relating to comments on pull request reviews, see the
    /// [`PullRequestReviewComment`](WebhookEventType::PullRequestReviewComment) event. For activity
    /// relating to issue comments, see the [`IssueComment`](WebhookEventType::IssueComment) event.
    /// For activity relating to discussion comments, see the
    /// [`DiscussionComment`](WebhookEventType::DiscussionComment) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    CommitComment,
    /// This event occurs when a Git branch or tag is created.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    ///
    /// **Note:** This event will not occur when more than three tags are created at once.
    Create,
    /// This event occurs when a Git branch or tag is deleted.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    ///
    /// **Note:** This event will not occur when more than three tags are deleted at once.
    Delete,
    /// This event occurs when there is activity relating to Dependabot alerts.
    ///
    /// For more information about Dependabot alerts, see "About Dependabot alerts."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Dependabot alerts" repository permission.
    ///
    /// **Note:** Webhook events for Dependabot alerts are currently in beta and subject to change.
    DependabotAlert,
    /// This event occurs when there is activity relating to deploy keys. For more information, see
    /// "Managing deploy keys."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Deployments" repository permission.
    DeployKey,
    /// This event occurs when there is activity relating to deployments. For more information, see
    /// "About deployments."
    ///
    /// For activity relating to deployment status, see the
    /// [`DeploymentStatus`](WebhookEventType::DeploymentStatus) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Deployments" repository permission.
    Deployment,
    /// This event occurs when there is activity relating to deployment protection rules. For more
    /// information, see "Using environments for deployment."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Deployments" repository permission.
    DeploymentProtectionRule,
    /// This event occurs when there is activity relating to deployment statuses. For more
    /// information, see "About deployments."
    ///
    /// For activity relating to deployment creation, see the
    /// [`Deployment`](WebhookEventType::Deployment) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Deployments" repository permission.
    DeploymentStatus,
    /// This event occurs when there is activity relating to a discussion. For more information
    /// about discussions, see "GitHub Discussions."
    ///
    /// For activity relating to a comment on a discussion, see the
    /// [`DiscussionComment`](WebhookEventType::DiscussionComment) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Discussions" repository permission.
    ///
    /// **Note:** Webhook events for GitHub Discussions are currently in beta and subject to change.
    Discussion,
    /// This event occurs when there is activity relating to a comment on a discussion. For more
    /// information about discussions, see "GitHub Discussions."
    ///
    /// For activity relating to a discussion as opposed to comments on a discussion, use the
    /// [`Discussion`](WebhookEventType::Discussion) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Discussions" repository permission.
    ///
    /// **Note:** Webhook events for GitHub Discussions are currently in beta and subject to change.
    DiscussionComment,
    /// This event occurs when someone forks a repository. For more information, see "Fork a repo."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    Fork,
    /// This event occurs when a user revokes their authorization of a GitHub App. For more
    /// information, see "About apps."
    ///
    /// A GitHub App receives this webhook by default and cannot unsubscribe from this event.
    ///
    /// Anyone can revoke their authorization of a GitHub App from their GitHub account settings
    /// page. Revoking the authorization of a GitHub App does not uninstall the GitHub App. You
    /// should program your GitHub App so that when it receives this webhook, it stops calling the
    /// API on behalf of the person who revoked the token. If your GitHub App continues to use a
    /// revoked access token, it will receive the 401 Bad Credentials error. For details about
    /// requests with a user access token, which require GitHub App authorization, see
    /// "Authenticating with a GitHub App on behalf of a user."
    GithubAppAuthorization,
    /// This event occurs when someone creates or updates a wiki page. For more information, see
    /// "About wikis."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    Gollum,
    /// This event occurs when there is activity relating to a GitHub App installation. All GitHub
    /// Apps receive this event by default. You cannot manually subscribe to this event.
    ///
    /// For more information about GitHub Apps, see "About apps."
    Installation,
    /// This event occurs when there is activity relating to which repositories a GitHub App
    /// installation can access. All GitHub Apps receive this event by default. You cannot manually
    /// subscribe to this event.
    ///
    /// For more information about GitHub Apps, see "About apps."
    InstallationRepositories,
    /// This event occurs when there is activity relating to the user or organization account that a
    /// GitHub App is installed on. For more information, see "About apps."
    InstallationTarget,
    /// This event occurs when there is activity relating to a comment on an issue or pull request.
    /// For more information about issues and pull requests, see "About issues" and "About pull
    /// requests."
    ///
    /// For activity relating to an issue as opposed to comments on an issue, see the
    /// [`Issues`](WebhookEventType::Issues) event. For activity related to pull request reviews or
    /// pull request review comments, use the
    /// [`PullRequestReview`](WebhookEventType::PullRequestReview) or
    /// [`PullRequestReviewComment`](WebhookEventType::PullRequestReviewComment) events. For more
    /// information about the different types of pull request comments, see "Working with comments."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Issues" repository permission.
    IssueComment,
    /// This event occurs when there is activity relating to an issue. For more information about
    /// issues, see "About issues."
    ///
    /// For activity relating to a comment on an issue, see the
    /// [`IssueComment`](WebhookEventType::IssueComment) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Issues" repository permission.
    Issues,
    /// This event occurs when there is activity relating to labels. For more information, see
    /// "Managing labels."
    ///
    /// If you want to receive an event when a label is added to or removed from an issue, pull
    /// request, or discussion, use the labeled or unlabeled action type for the
    /// [`Issues`](WebhookEventType::Issues) [`PullRequest`](WebhookEventType::PullRequest) or
    /// [`Discussion`](WebhookEventType::Discussion) events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Metadata" repository permission.
    Label,
    /// This event occurs when there is activity relating to a GitHub Marketplace purchase. For more
    /// information, see "GitHub Marketplace."
    MarketplacePurchase,
    /// This event occurs when there is activity relating to collaborators in a repository. For more
    /// information, see "Adding outside collaborators to repositories in your organization." For
    /// more information about the API to manage repository collaborators, see the GraphQL API
    /// documentation or "Collaborators" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Members" organization permission.
    Member,
    /// This event occurs when there is activity relating to team membership. For more information,
    /// see "About teams." For more information about the APIs to manage team memberships, see the
    /// GraphQL API documentation or "Team members" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Members" organization permission.
    Membership,
    /// This event occurs when there is activity relating to a merge group in a merge queue. For
    /// more information, see "Managing a merge queue."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Merge
    /// queues" repository permission.
    MergeGroup,
    /// This event occurs when there is activity relating to a webhook itself.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Meta"
    /// app permission.
    Meta,
    /// This event occurs when there is activity relating to milestones. For more information, see
    /// "About milestones."
    ///
    /// If you want to receive an event when an issue or pull request is added to or removed from a
    /// milestone, use the milestoned or demilestoned action type for the
    /// [`Issues`](WebhookEventType::Issues) or [`PullRequest`](WebhookEventType::PullRequest) events
    /// instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Issues" or "Pull requests" repository permissions.
    Milestone,
    /// This event occurs when organization owners or moderators block or unblock a non-member from
    /// collaborating on the organization's repositories. For more information, see "Blocking a user
    /// from your organization."
    ///
    /// If you want to receive an event when members are added or removed from an organization, use
    /// the [`Organization`](WebhookEventType::Organization) event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Administration" organization permission.
    OrgBlock,
    /// This event occurs when there is activity relating to an organization and its members. For
    /// more information, see "About organizations."
    ///
    /// If you want to receive an event when a non-member is blocked or unblocked from an
    /// organization, see the [`OrgBlock`](WebhookEventType::OrgBlock) event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Members" organization permission.
    Organization,
    /// This event occurs when there is activity relating to GitHub Packages. For more information,
    /// see "Introduction to GitHub Packages."
    ///
    /// To install this event on a GitHub App, the app must have at least read-level access for the
    /// "Packages" repository permission.
    Package,
    /// This event occurs when there is an attempted build of a GitHub Pages site. This event occurs
    /// regardless of whether the build is successful. For more information, see "Configuring a
    /// publishing source for your GitHub Pages site."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Pages" repository permission.
    PageBuild,
    /// This event occurs when there is activity relating to a request for a fine-grained personal
    /// access token to access resources that belong to a resource owner that requires approval for
    /// token access. For more information, see "Creating a personal access token."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Personal access token requests" organization permission.
    ///
    /// **Note:** Fine-grained PATs are in public beta. Related APIs, events, and functionality are subject to change.
    PersonalAccessTokenRequest,
    /// This event occurs when you create a new webhook. The ping event is a confirmation from
    /// GitHub that you configured the webhook correctly.
    Ping,
    /// This event occurs when there is activity relating to a card on a classic project. For more
    /// information, see "About projects (classic)."
    ///
    /// For activity relating to a project or a column on a project, see the
    /// [`Project`](WebhookEventType::Project) and [`ProjectColumn`](WebhookEventType::ProjectColumn)
    /// event. For activity relating to Projects instead of Projects (classic), use the
    /// [`ProjectsV2`](WebhookEventType::ProjectsV2) event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Projects" repository or organization permission.
    ProjectCard,
    /// This event occurs when there is activity relating to a classic project. For more
    /// information, see "About projects (classic)."
    ///
    /// For activity relating to a card or column on a project, see the
    /// [`ProjectCard`](WebhookEventType::ProjectCard) and
    /// [`ProjectColumn`](WebhookEventType::ProjectColumn) event. For activity relating to Projects
    /// instead of Projects (classic), use the [`ProjectsV2`](WebhookEventType::ProjectsV2) event
    /// instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Projects" repository or organization permission.
    Project,
    /// This event occurs when there is activity relating to a column on a classic project. For more
    /// information, see "About projects (classic)."
    ///
    /// For activity relating to a project or a card on a project, see the
    /// [`Project`](WebhookEventType::Project) and [`ProjectCard`](WebhookEventType::ProjectCard)
    /// event. For activity relating to Projects instead of Projects (classic), see the
    /// [`ProjectsV2`](WebhookEventType::ProjectsV2) event instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Projects" repository or organization permission.
    ProjectColumn,
    /// This event occurs when there is activity relating to an organization-level project. For more
    /// information, see "About Projects."
    ///
    /// For activity relating to a item on a project, see the
    /// [`ProjectsV2Item`](WebhookEventType::ProjectsV2Item) event. For activity relating to Projects
    /// (classic), see the [`Project`](WebhookEventType::Project),
    /// [`ProjectCard`](WebhookEventType::ProjectCard), and
    /// [`ProjectColumn`](WebhookEventType::ProjectColumn) events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Projects" organization permission.
    ///
    /// **Note:** Webhook events for projects are currently in beta and subject to change. To share
    /// feedback about projects webhooks with GitHub, see the Projects webhook feedback discussion.
    ProjectsV2,
    /// This event occurs when there is activity relating to an item on an organization-level
    /// project. For more information, see "About Projects."
    ///
    /// For activity relating to a project (instead of an item on a project), see the
    /// [`ProjectsV2`](WebhookEventType::ProjectsV2) event. For activity relating to Projects
    /// (classic), see the [`Project`](WebhookEventType::Project),
    /// [`ProjectCard`](WebhookEventType::ProjectCard) and
    /// [`ProjectColumn`](WebhookEventType::ProjectColumn) events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Projects" organization permission.
    ///
    /// **Note:** Webhook events for projects are currently in beta and subject to change. To share
    /// feedback about projects webhooks with GitHub, see the Projects webhook feedback discussion.
    ProjectsV2Item,
    /// This event occurs when repository visibility changes from private to public. For more
    /// information, see "Setting repository visibility."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Metadata" repository permission.
    Public,
    /// This event occurs when there is activity on a pull request. For more information, see "About
    /// pull requests."
    ///
    /// For activity related to pull request reviews, pull request review comments, pull request
    /// comments, or pull request review threads, see the
    /// [`PullRequestReview`](WebhookEventType::PullRequestReview),
    /// [`PullRequestReviewComment`](WebhookEventType::PullRequestReviewComment),
    /// [`IssueComment`](WebhookEventType::IssueComment), or
    /// [`PullRequestReviewThread`](WebhookEventType::PullRequestReviewThread) events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull
    /// requests" repository permission.
    PullRequest,
    /// This event occurs when there is activity relating to a pull request review. A pull request
    /// review is a group of pull request review comments in addition to a body comment and a state.
    /// For more information, see "About pull request reviews."
    ///
    /// For activity related to pull request review comments, pull request comments, or pull request
    /// review threads, see the
    /// [`PullRequestReviewComment`](WebhookEventType::PullRequestReviewComment),
    /// [`IssueComment`](WebhookEventType::IssueComment), or
    /// [`PullRequestReviewThread`](WebhookEventType::PullRequestReviewThread) events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull
    /// requests" repository permission.
    PullRequestReview,
    /// This event occurs when there is activity relating to a pull request review comment. A pull
    /// request review comment is a comment on a pull request's diff. For more information, see
    /// "Commenting on a pull request."
    ///
    /// For activity related to pull request reviews, pull request comments, or pull request review
    /// threads, see the [`PullRequestReview`](WebhookEventType::PullRequestReview),
    /// [`IssueComment`](WebhookEventType::IssueComment), or
    /// [`PullRequestReviewThread`](WebhookEventType::PullRequestReviewThread), events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull
    /// requests" repository permission.
    PullRequestReviewComment,
    /// This event occurs when there is activity relating to a comment thread on a pull request. For
    /// more information, see "About pull request reviews."
    ///
    /// For activity related to pull request review comments, pull request comments, or pull request
    /// reviews, see the [`PullRequestReviewComment`](WebhookEventType::PullRequestReviewComment),
    /// [`IssueComment`](WebhookEventType::IssueComment), or
    /// [`PullRequestReview`](WebhookEventType::PullRequestReview) events instead.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the "Pull
    /// requests" repository permission.
    PullRequestReviewThread,
    /// This event occurs when a commit or tag is pushed.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    ///
    /// **Note:** An event will not be created when more than three tags are pushed at once.
    Push,
    /// This event occurs when there is activity relating to GitHub Packages. For more information,
    /// see "Introduction to GitHub Packages."
    ///
    /// To install this event on a GitHub App, the app must have at least read-level access for the
    /// "Packages" repository permission.
    ///
    /// **Note:** GitHub recommends that you use the newer package event instead.
    RegistryPackage,
    /// This event occurs when there is activity relating to releases. For more information, see
    /// "About releases."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    Release,
    /// This event occurs when there is activity relating to a repository security advisory. For
    /// more information about repository security advisories, see "About GitHub Security Advisories
    /// for repositories."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Repository security advisories" permission.
    RepositoryAdvisory,
    /// This event occurs when there is activity relating to repositories. For more information, see
    /// "About repositories."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Metadata" repository permission.
    Repository,
    /// This event occurs when a GitHub App sends a POST request to
    /// /repos/{owner}/{repo}/dispatches. For more information, see the REST API documentation for
    /// creating a repository dispatch event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    RepositoryDispatch,
    /// This event occurs when a repository is imported to GitHub. For more information, see
    /// "Importing a repository with GitHub Importer." For more information about the API to manage
    /// imports, see the REST API documentation.
    RepositoryImport,
    /// This event occurs when there is activity relating to a security vulnerability alert in a
    /// repository.
    ///
    /// **Note:** This event is deprecated. see the
    /// [`DependabotAlert`](WebhookEventType::DependabotAlert) event instead.
    RepositoryVulnerabilityAlert,
    /// The schedule event allows you to trigger a workflow at a scheduled time.
    ///
    /// You can schedule a workflow to run at specific UTC times using POSIX cron syntax. Scheduled
    /// workflows run on the latest commit on the default or base branch. The shortest interval you
    /// can run scheduled workflows is once every 5 minutes.
    Schedule,
    /// This event occurs when there is activity relating to a secret scanning alert. For more
    /// information about secret scanning, see "About secret scanning."
    ///
    /// For activity relating to secret scanning alert locations, use the
    /// [`SecretScanningAlertLocation`](WebhookEventType::SecretScanningAlertLocation) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Secret scanning alerts" repository permission.
    SecretScanningAlert,
    /// This event occurs when there is activity relating to the locations of a secret in a secret
    /// scanning alert.
    ///
    /// For more information about secret scanning, see "About secret scanning."
    ///
    /// For activity relating to secret scanning alerts, see the
    /// [`SecretScanningAlert`](WebhookEventType::SecretScanningAlert) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Secret scanning alerts" repository permission.
    SecretScanningAlertLocation,
    /// This event occurs when there is activity relating to a security advisory that was reviewed
    /// by GitHub. A GitHub-reviewed security advisory provides information about security-related
    /// vulnerabilities in software on GitHub. For more information about security advisories, see
    /// "About GitHub Security Advisories for repositories."
    ///
    /// GitHub Dependabot alerts are also powered by the security advisory dataset. For more
    /// information, see "About Dependabot alerts."
    SecurityAdvisory,
    /// This event occurs when code security and analysis features are enabled or disabled for a
    /// repository. For more information, see "GitHub security features."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Administration" repository permission.
    SecurityAndAnalysis,
    /// This event occurs when there is activity relating to a sponsorship listing. For more
    /// information, see "About GitHub Sponsors."
    ///
    /// You can only create a sponsorship webhook on GitHub.com. For more information, see
    /// "Configuring webhooks for events in your sponsored account."
    Sponsorship,
    /// This event occurs when there is activity relating to repository stars. For more information
    /// about stars, see "Saving repositories with stars."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Metadata" repository permission.
    Star,
    /// This event occurs when the status of a Git commit changes. For example, commits can be
    /// marked as error, failure, pending, or success. For more information, see "About status
    /// checks."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Commit statuses" repository permission.
    Status,
    /// This event occurs when a team is added to a repository. For more information, see "Managing
    /// teams and people with access to your repository."
    ///
    /// For activity relating to teams, see the teams event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Members" organization permission.
    TeamAdd,
    /// This event occurs when there is activity relating to teams in an organization. For more
    /// information, see "About teams."
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Members" organization permission.
    Team,
    /// This event occurs when there is activity relating to watching, or subscribing to, a
    /// repository. For more information about watching, see "Managing your subscriptions." For
    /// information about the APIs to manage watching, see "Watching" in the REST API documentation.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Metadata" repository permission.
    Watch,
    /// This event occurs when a GitHub Actions workflow is manually triggered. For more
    /// information, see "Manually running a workflow."
    ///
    /// For activity relating to workflow runs, see the
    /// [`WorkflowRun`](WebhookEventType::WorkflowRun) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Contents" repository permission.
    WorkflowDispatch,
    /// This event occurs when there is activity relating to a job in a GitHub Actions workflow. For
    /// more information, see "Using jobs in a workflow."
    ///
    /// For activity relating to a workflow run instead of a job in a workflow run, use the
    /// [`WorkflowRun`](WebhookEventType::WorkflowRun) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Actions" repository permission.
    WorkflowJob,
    /// This event occurs when there is activity relating to a run of a GitHub Actions workflow. For
    /// more information, see "About workflows."
    ///
    /// For activity relating to a job in a workflow run, see the
    /// [`WorkflowJob`](WebhookEventType::WorkflowJob) event.
    ///
    /// To subscribe to this event, a GitHub App must have at least read-level access for the
    /// "Actions" repository permission.
    WorkflowRun,
    /// A webhook event that is currently unsupported by octocrab.
    ///
    /// When a [`WebhookEvent`] has this [kind](WebhookEvent::kind), then the
    /// [specific](WebhookEvent::specific) payload will only be a generic `serde_json::Value`.
    #[serde(untagged)]
    Unknown(String),
}

impl WebhookEventType {
    /// Parse (and verify) the payload for the specific event kind.
    pub fn parse_specific_payload(
        &self,
        data: serde_json::Value,
    ) -> Result<WebhookEventPayload, serde_json::Error> {
        match self {
            WebhookEventType::BranchProtectionRule => Ok(
                WebhookEventPayload::BranchProtectionRule(Box::new(serde_json::from_value(data)?)),
            ),
            WebhookEventType::CheckRun => Ok(WebhookEventPayload::CheckRun(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::CheckSuite => Ok(WebhookEventPayload::CheckSuite(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::CodeScanningAlert => Ok(WebhookEventPayload::CodeScanningAlert(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::CommitComment => Ok(WebhookEventPayload::CommitComment(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Create => Ok(WebhookEventPayload::Create(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Delete => Ok(WebhookEventPayload::Delete(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::DependabotAlert => Ok(WebhookEventPayload::DependabotAlert(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::DeployKey => Ok(WebhookEventPayload::DeployKey(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Deployment => Ok(WebhookEventPayload::Deployment(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::DeploymentProtectionRule => {
                Ok(WebhookEventPayload::DeploymentProtectionRule(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::DeploymentStatus => Ok(WebhookEventPayload::DeploymentStatus(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Discussion => Ok(WebhookEventPayload::Discussion(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::DiscussionComment => Ok(WebhookEventPayload::DiscussionComment(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Fork => Ok(WebhookEventPayload::Fork(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::GithubAppAuthorization => {
                Ok(WebhookEventPayload::GithubAppAuthorization(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::Gollum => Ok(WebhookEventPayload::Gollum(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Installation => Ok(WebhookEventPayload::Installation(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::InstallationRepositories => {
                Ok(WebhookEventPayload::InstallationRepositories(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::InstallationTarget => Ok(WebhookEventPayload::InstallationTarget(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::IssueComment => Ok(WebhookEventPayload::IssueComment(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Issues => Ok(WebhookEventPayload::Issues(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Label => Ok(WebhookEventPayload::Label(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::MarketplacePurchase => Ok(WebhookEventPayload::MarketplacePurchase(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Member => Ok(WebhookEventPayload::Member(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Membership => Ok(WebhookEventPayload::Membership(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::MergeGroup => Ok(WebhookEventPayload::MergeGroup(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Meta => Ok(WebhookEventPayload::Meta(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Milestone => Ok(WebhookEventPayload::Milestone(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::OrgBlock => Ok(WebhookEventPayload::OrgBlock(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Organization => Ok(WebhookEventPayload::Organization(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Package => Ok(WebhookEventPayload::Package(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::PageBuild => Ok(WebhookEventPayload::PageBuild(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::PersonalAccessTokenRequest => {
                Ok(WebhookEventPayload::PersonalAccessTokenRequest(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::Ping => Ok(WebhookEventPayload::Ping(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::ProjectCard => Ok(WebhookEventPayload::ProjectCard(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Project => Ok(WebhookEventPayload::Project(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::ProjectColumn => Ok(WebhookEventPayload::ProjectColumn(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::ProjectsV2 => Ok(WebhookEventPayload::ProjectsV2(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::ProjectsV2Item => Ok(WebhookEventPayload::ProjectsV2Item(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Public => Ok(WebhookEventPayload::Public(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::PullRequest => Ok(WebhookEventPayload::PullRequest(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::PullRequestReview => Ok(WebhookEventPayload::PullRequestReview(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::PullRequestReviewComment => {
                Ok(WebhookEventPayload::PullRequestReviewComment(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::PullRequestReviewThread => {
                Ok(WebhookEventPayload::PullRequestReviewThread(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::Push => Ok(WebhookEventPayload::Push(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::RegistryPackage => Ok(WebhookEventPayload::RegistryPackage(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Release => Ok(WebhookEventPayload::Release(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::RepositoryAdvisory => Ok(WebhookEventPayload::RepositoryAdvisory(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Repository => Ok(WebhookEventPayload::Repository(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::RepositoryDispatch => Ok(WebhookEventPayload::RepositoryDispatch(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::RepositoryImport => Ok(WebhookEventPayload::RepositoryImport(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::RepositoryVulnerabilityAlert => {
                Ok(WebhookEventPayload::RepositoryVulnerabilityAlert(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::Schedule => Ok(WebhookEventPayload::Schedule(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::SecretScanningAlert => Ok(WebhookEventPayload::SecretScanningAlert(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::SecretScanningAlertLocation => {
                Ok(WebhookEventPayload::SecretScanningAlertLocation(Box::new(
                    serde_json::from_value(data)?,
                )))
            }
            WebhookEventType::SecurityAdvisory => Ok(WebhookEventPayload::SecurityAdvisory(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::SecurityAndAnalysis => Ok(WebhookEventPayload::SecurityAndAnalysis(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::Sponsorship => Ok(WebhookEventPayload::Sponsorship(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Star => Ok(WebhookEventPayload::Star(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Status => Ok(WebhookEventPayload::Status(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::TeamAdd => Ok(WebhookEventPayload::TeamAdd(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Team => Ok(WebhookEventPayload::Team(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Watch => Ok(WebhookEventPayload::Watch(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::WorkflowDispatch => Ok(WebhookEventPayload::WorkflowDispatch(
                Box::new(serde_json::from_value(data)?),
            )),
            WebhookEventType::WorkflowJob => Ok(WebhookEventPayload::WorkflowJob(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::WorkflowRun => Ok(WebhookEventPayload::WorkflowRun(Box::new(
                serde_json::from_value(data)?,
            ))),
            WebhookEventType::Unknown(_) => Ok(WebhookEventPayload::Unknown(Box::new(data))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventInstallation {
    /// A full installation object which is present for `Installation*` related webhook events.
    Full(Box<Installation>),
    /// The minimal installation object is present for all other event types.
    Minimal(Box<EventInstallationId>),
}

impl EventInstallation {
    pub fn id(&self) -> InstallationId {
        match self {
            EventInstallation::Full(installation) => installation.id,
            EventInstallation::Minimal(installation) => installation.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventInstallationId {
    pub id: InstallationId,
    pub node_id: String,
}

/// A repository in installation related webhook events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallationEventRepository {
    pub id: RepositoryId,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::models::AuthorAssociation;

    use super::payload::*;
    use super::*;

    #[test]
    fn deserialize_commit_comment_created() {
        let json = include_str!("../../tests/resources/commit_comment_created_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("commit_comment", json).unwrap();
        let WebhookEventPayload::CommitComment(commit_comment_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            commit_comment_event.action,
            CommitCommentWebhookEventAction::Created
        );
        assert_eq!(
            commit_comment_event.comment.author_association,
            AuthorAssociation::Owner
        );
        assert_eq!(
            commit_comment_event.comment.body.as_deref(),
            Some("@gagbo-test-app[bot] compare-tag v0.1")
        );
    }

    #[test]
    fn deserialize_installation_created() {
        let json = include_str!("../../tests/resources/installation_created_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("installation", json).unwrap();
        let WebhookEventPayload::Installation(install_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            install_event.action,
            InstallationWebhookEventAction::Created
        );
        assert_eq!(install_event.repositories.as_ref().unwrap().len(), 3);
        assert_eq!(install_event.repositories.unwrap()[2].name, "octocrab");
    }

    #[test]
    fn deserialize_installation_deleted() {
        let json = include_str!("../../tests/resources/installation_deleted_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("installation", json).unwrap();
        let WebhookEventPayload::Installation(install_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            install_event.action,
            InstallationWebhookEventAction::Deleted
        );
        assert_eq!(install_event.repositories.as_ref().unwrap().len(), 3);
        assert_eq!(install_event.repositories.unwrap()[2].name, "octocrab");
    }

    #[test]
    fn deserialize_installation_new_permissions_accepted() {
        let json = include_str!(
            "../../tests/resources/installation_new_permissions_accepted_webhook_event.json"
        );
        let event = WebhookEvent::try_from_header_and_body("installation", json).unwrap();
        let WebhookEventPayload::Installation(ref install_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        let Some(EventInstallation::Full(installation)) = event.installation else {
            panic!(
                "event is missing a fully described installation object {:?}",
                event
            )
        };
        assert_eq!(
            install_event.action,
            InstallationWebhookEventAction::NewPermissionsAccepted
        );
        assert_eq!(
            installation.updated_at.unwrap(),
            chrono::Utc
                .with_ymd_and_hms(2023, 8, 18, 13, 28, 4)
                .unwrap()
        );
        assert_eq!(
            installation.created_at.unwrap(),
            chrono::Utc
                .with_ymd_and_hms(2023, 7, 13, 9, 35, 31)
                .unwrap()
        );
        assert_eq!(installation.events.len(), 12);
    }

    #[test]
    fn deserialize_installation_repositories_removed() {
        let json = include_str!(
            "../../tests/resources/installation_repositories_removed_webhook_event.json"
        );
        let event =
            WebhookEvent::try_from_header_and_body("installation_repositories", json).unwrap();
        let WebhookEventPayload::InstallationRepositories(install_repositories_event) =
            event.specific
        else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            install_repositories_event.action,
            InstallationRepositoriesWebhookEventAction::Removed
        );
        assert_eq!(install_repositories_event.repositories_removed.len(), 1);
        assert_eq!(
            install_repositories_event.repository_selection,
            InstallationRepositoriesWebhookEventSelection::All
        );
    }

    #[test]
    fn deserialize_issue_comment_created() {
        let json = include_str!("../../tests/resources/issue_comment_created_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("issue_comment", json).unwrap();
        let WebhookEventPayload::IssueComment(issue_comment_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            issue_comment_event.action,
            IssueCommentWebhookEventAction::Created
        );
        assert_eq!(*issue_comment_event.comment.id, 1633968123);
    }

    #[test]
    fn deserialize_issue_comment_deleted() {
        let json = include_str!("../../tests/resources/issue_comment_deleted_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("issue_comment", json).unwrap();
        let WebhookEventPayload::IssueComment(issue_comment_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            issue_comment_event.action,
            IssueCommentWebhookEventAction::Deleted
        );
    }

    #[test]
    fn deserialize_issue_comment_edited() {
        let json = include_str!("../../tests/resources/issue_comment_edited_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("issue_comment", json).unwrap();
        let WebhookEventPayload::IssueComment(issue_comment_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            issue_comment_event.action,
            IssueCommentWebhookEventAction::Edited
        );
        assert_eq!(issue_comment_event.changes.unwrap().body.from, "Old Body");
    }

    #[test]
    fn deserialize_issues_labeled() {
        let json = include_str!("../../tests/resources/issues_labeled_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("issues", json).unwrap();
        let WebhookEventPayload::Issues(issues_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(issues_event.action, IssuesWebhookEventAction::Labeled);
    }

    #[test]
    fn deserialize_issues_opened() {
        let json = include_str!("../../tests/resources/issues_opened_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("issues", json).unwrap();
        let WebhookEventPayload::Issues(issues_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(issues_event.action, IssuesWebhookEventAction::Opened);
    }

    #[test]
    fn deserialize_ping() {
        let json = include_str!("../../tests/resources/ping_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("ping", json).unwrap();
        let WebhookEventPayload::Ping(ping_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(ping_event.hook.unwrap().id, 423885699);
    }

    #[test]
    fn deserialize_pull_request_closed() {
        let json = include_str!("../../tests/resources/pull_request_closed_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("pull_request", json).unwrap();
        let WebhookEventPayload::PullRequest(pull_request_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            pull_request_event.action,
            PullRequestWebhookEventAction::Closed
        );
        assert_eq!(pull_request_event.pull_request.number, 2);
    }

    #[test]
    fn deserialize_pull_request_opened() {
        let json = include_str!("../../tests/resources/pull_request_opened_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("pull_request", json).unwrap();
        let WebhookEventPayload::PullRequest(pull_request_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            pull_request_event.action,
            PullRequestWebhookEventAction::Opened
        );
    }

    #[test]
    fn deserialize_pull_request_synchronize() {
        let json =
            include_str!("../../tests/resources/pull_request_synchronize_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("pull_request", json).unwrap();
        let WebhookEventPayload::PullRequest(pull_request_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            pull_request_event.action,
            PullRequestWebhookEventAction::Synchronize
        );
    }

    #[test]
    fn deserialize_repository_deleted() {
        let json = include_str!("../../tests/resources/repository_deleted_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("repository", json).unwrap();
        let WebhookEventPayload::Repository(repository_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert_eq!(
            repository_event.action,
            RepositoryWebhookEventAction::Deleted
        );
    }

    #[test]
    fn deserialize_push() {
        let json = include_str!("../../tests/resources/push_webhook_event.json");
        let event = WebhookEvent::try_from_header_and_body("push", json).unwrap();
        let WebhookEventPayload::Push(push_event) = event.specific else {
            panic!(" event is of the wrong type {:?}", event)
        };
        assert!(push_event.created);
    }
}
