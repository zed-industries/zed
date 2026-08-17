mod branch_protection_rule;
mod check_run;
mod check_suite;
mod code_scanning_alert;
mod commit_comment;
mod create;
mod delete;
mod dependabot_alert;
mod deploy_key;
mod deployment;
mod deployment_protection_rule;
mod deployment_status;
mod discussion;
mod discussion_comment;
mod fork;
mod github_app_authorization;
mod gollum;
mod installation;
mod installation_repositories;
mod installation_target;
mod issue_comment;
mod issues;
mod label;
mod marketplace_purchase;
mod member;
mod membership;
mod merge_group;
mod meta;
mod milestone;
mod org_block;
mod organization;
mod package;
mod page_build;
mod personal_access_token_request;
mod ping;
mod project;
mod project_card;
mod project_column;
mod projects_v2;
mod projects_v2_item;
mod public;
mod pull_request;
mod pull_request_review;
mod pull_request_review_comment;
mod pull_request_review_thread;
mod push;
mod registry_package;
mod release;
mod repository;
mod repository_advisory;
mod repository_dispatch;
mod repository_import;
mod repository_vulnerability_alert;
mod schedule;
mod secret_scanning_alert;
mod secret_scanning_alert_location;
mod security_advisory;
mod security_and_analysis;
mod sponsorship;
mod star;
mod status;
mod team;
mod team_add;
mod watch;
mod workflow_dispatch;
mod workflow_job;
mod workflow_run;

pub use self::{
    branch_protection_rule::*, check_run::*, check_suite::*, code_scanning_alert::*,
    commit_comment::*, create::*, delete::*, dependabot_alert::*, deploy_key::*, deployment::*,
    deployment_protection_rule::*, deployment_status::*, discussion::*, discussion_comment::*,
    fork::*, github_app_authorization::*, gollum::*, installation::*, installation_repositories::*,
    installation_target::*, issue_comment::*, issues::*, label::*, marketplace_purchase::*,
    member::*, membership::*, merge_group::*, meta::*, milestone::*, org_block::*, organization::*,
    package::*, page_build::*, personal_access_token_request::*, ping::*, project::*,
    project_card::*, project_column::*, projects_v2::*, projects_v2_item::*, public::*,
    pull_request::*, pull_request_review::*, pull_request_review_comment::*,
    pull_request_review_thread::*, push::*, registry_package::*, release::*, repository::*,
    repository_advisory::*, repository_dispatch::*, repository_import::*,
    repository_vulnerability_alert::*, schedule::*, secret_scanning_alert::*,
    secret_scanning_alert_location::*, security_advisory::*, security_and_analysis::*,
    sponsorship::*, star::*, status::*, team::*, team_add::*, watch::*, workflow_dispatch::*,
    workflow_job::*, workflow_run::*,
};

use serde::{Deserialize, Serialize};

/// The specific part of the payload in a webhook event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebhookEventPayload {
    BranchProtectionRule(Box<BranchProtectionRuleWebhookEventPayload>),
    CheckRun(Box<CheckRunWebhookEventPayload>),
    CheckSuite(Box<CheckSuiteWebhookEventPayload>),
    CodeScanningAlert(Box<CodeScanningAlertWebhookEventPayload>),
    CommitComment(Box<CommitCommentWebhookEventPayload>),
    Create(Box<CreateWebhookEventPayload>),
    Delete(Box<DeleteWebhookEventPayload>),
    DependabotAlert(Box<DependabotAlertWebhookEventPayload>),
    DeployKey(Box<DeployKeyWebhookEventPayload>),
    Deployment(Box<DeploymentWebhookEventPayload>),
    DeploymentProtectionRule(Box<DeploymentProtectionRuleWebhookEventPayload>),
    DeploymentStatus(Box<DeploymentStatusWebhookEventPayload>),
    Discussion(Box<DiscussionWebhookEventPayload>),
    DiscussionComment(Box<DiscussionCommentWebhookEventPayload>),
    Fork(Box<ForkWebhookEventPayload>),
    GithubAppAuthorization(Box<GithubAppAuthorizationWebhookEventPayload>),
    Gollum(Box<GollumWebhookEventPayload>),
    Installation(Box<InstallationWebhookEventPayload>),
    InstallationRepositories(Box<InstallationRepositoriesWebhookEventPayload>),
    InstallationTarget(Box<InstallationTargetWebhookEventPayload>),
    IssueComment(Box<IssueCommentWebhookEventPayload>),
    Issues(Box<IssuesWebhookEventPayload>),
    Label(Box<LabelWebhookEventPayload>),
    MarketplacePurchase(Box<MarketplacePurchaseWebhookEventPayload>),
    Member(Box<MemberWebhookEventPayload>),
    Membership(Box<MembershipWebhookEventPayload>),
    MergeGroup(Box<MergeGroupWebhookEventPayload>),
    Meta(Box<MetaWebhookEventPayload>),
    Milestone(Box<MilestoneWebhookEventPayload>),
    OrgBlock(Box<OrgBlockWebhookEventPayload>),
    Organization(Box<OrganizationWebhookEventPayload>),
    Package(Box<PackageWebhookEventPayload>),
    PageBuild(Box<PageBuildWebhookEventPayload>),
    PersonalAccessTokenRequest(Box<PersonalAccessTokenRequestWebhookEventPayload>),
    Ping(Box<PingWebhookEventPayload>),
    ProjectCard(Box<ProjectCardWebhookEventPayload>),
    Project(Box<ProjectWebhookEventPayload>),
    ProjectColumn(Box<ProjectColumnWebhookEventPayload>),
    ProjectsV2(Box<ProjectsV2WebhookEventPayload>),
    ProjectsV2Item(Box<ProjectsV2ItemWebhookEventPayload>),
    Public(Box<PublicWebhookEventPayload>),
    PullRequest(Box<PullRequestWebhookEventPayload>),
    PullRequestReview(Box<PullRequestReviewWebhookEventPayload>),
    PullRequestReviewComment(Box<PullRequestReviewCommentWebhookEventPayload>),
    PullRequestReviewThread(Box<PullRequestReviewThreadWebhookEventPayload>),
    Push(Box<PushWebhookEventPayload>),
    RegistryPackage(Box<RegistryPackageWebhookEventPayload>),
    Release(Box<ReleaseWebhookEventPayload>),
    RepositoryAdvisory(Box<RepositoryAdvisoryWebhookEventPayload>),
    Repository(Box<RepositoryWebhookEventPayload>),
    RepositoryDispatch(Box<RepositoryDispatchWebhookEventPayload>),
    RepositoryImport(Box<RepositoryImportWebhookEventPayload>),
    RepositoryVulnerabilityAlert(Box<RepositoryVulnerabilityAlertWebhookEventPayload>),
    Schedule(Box<ScheduleWebhookEventPayload>),
    SecretScanningAlert(Box<SecretScanningAlertWebhookEventPayload>),
    SecretScanningAlertLocation(Box<SecretScanningAlertLocationWebhookEventPayload>),
    SecurityAdvisory(Box<SecurityAdvisoryWebhookEventPayload>),
    SecurityAndAnalysis(Box<SecurityAndAnalysisWebhookEventPayload>),
    Sponsorship(Box<SponsorshipWebhookEventPayload>),
    Star(Box<StarWebhookEventPayload>),
    Status(Box<StatusWebhookEventPayload>),
    TeamAdd(Box<TeamAddWebhookEventPayload>),
    Team(Box<TeamWebhookEventPayload>),
    Watch(Box<WatchWebhookEventPayload>),
    WorkflowDispatch(Box<WorkflowDispatchWebhookEventPayload>),
    WorkflowJob(Box<WorkflowJobWebhookEventPayload>),
    WorkflowRun(Box<WorkflowRunWebhookEventPayload>),
    Unknown(Box<serde_json::Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RefType {
    Tag,
    Branch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PusherType {
    User,
    #[serde(untagged)]
    DeployKey(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MembershipScope {
    Team,
    Organization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OldValue<T>
where
    T: Serialize,
    T: std::fmt::Debug + Clone + PartialEq,
{
    /// Old value, when the webhook payload is a change
    pub from: T,
}
