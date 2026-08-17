mod commit_comment;
mod create;
mod delete;
mod fork;
mod gollum;
mod installation;
mod installation_repositories;
mod installation_target;
mod issue_comment;
mod issues;
mod member;
mod public;
mod pull_request;
mod pull_request_review;
mod pull_request_review_comment;
mod push;
mod release;
mod watch;
mod workflow_run;

pub use commit_comment::*;
pub use create::*;
pub use delete::*;
pub use fork::*;
pub use gollum::*;
pub use installation::*;
pub use installation_repositories::*;
pub use installation_target::*;
pub use issue_comment::*;
pub use issues::*;
pub use member::*;
pub use public::*;
pub use pull_request::*;
pub use pull_request_review::*;
pub use pull_request_review_comment::*;
pub use push::*;
pub use release::*;
pub use watch::*;
pub use workflow_run::*;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{
    orgs::Organization, repos::CommitAuthor, Author, Installation, InstallationId, Repository,
    RepositoryId,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventInstallation {
    /// A full installation object which is present for `Installation*` related webhook events.
    Full(Box<Installation>),
    /// The minimal installation object is present for all other event types.
    Minimal(Box<EventInstallationId>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventInstallationId {
    pub id: InstallationId,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WrappedEventPayload {
    pub installation: Option<EventInstallation>,
    pub organization: Option<Organization>,
    pub repository: Option<Repository>,
    pub sender: Option<Author>,
    #[serde(flatten)]
    pub specific: Option<EventPayload>,
}

/// The payload in an event type.
///
/// Different event types have different payloads. Any event type not specifically part
/// of this enum will be captured in the variant `UnknownEvent` with a value of
/// [`serde_json::Value`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum EventPayload {
    PushEvent(Box<PushEventPayload>),
    CreateEvent(Box<CreateEventPayload>),
    DeleteEvent(Box<DeleteEventPayload>),
    InstallationEvent(Box<InstallationEventPayload>),
    InstallationRepositoriesEvent(Box<InstallationRepositoriesEventPayload>),
    InstallationTargetEvent(Box<InstallationTargetEventPayload>),
    IssuesEvent(Box<IssuesEventPayload>),
    IssueCommentEvent(Box<IssueCommentEventPayload>),
    CommitCommentEvent(Box<CommitCommentEventPayload>),
    ForkEvent(Box<ForkEventPayload>),
    GollumEvent(Box<GollumEventPayload>),
    MemberEvent(Box<MemberEventPayload>),
    PublicEvent(Box<PublicEventPayload>),
    PullRequestEvent(Box<PullRequestEventPayload>),
    PullRequestReviewEvent(Box<PullRequestReviewEventPayload>),
    PullRequestReviewCommentEvent(Box<PullRequestReviewCommentEventPayload>),
    ReleaseEvent(Box<ReleaseEventPayload>),
    WatchEvent(Box<WatchEventPayload>),
    WorkflowRunEvent(Box<WorkflowRunEventPayload>),
    UnknownEvent(Box<serde_json::Value>),
}

/// A git commit in specific payload types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Commit {
    pub sha: String,
    pub author: CommitAuthor,
    pub message: String,
    pub distinct: bool,
    pub url: Url,
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
    use super::*;

    #[test]
    fn should_deserialize_installation_event() {
        // The payload has been extracted as the `payload` key from a webhook installation event.
        let json = include_str!("../../../tests/resources/installation_event.json");
        let event: WrappedEventPayload = serde_json::from_str(json).unwrap();

        let installation = event.installation.unwrap();
        let specific = event.specific.unwrap();

        match installation {
            EventInstallation::Full(install) => {
                assert_eq!(install.id, 7777777.into());
                assert_eq!(install.repository_selection.unwrap(), "all");
            }
            EventInstallation::Minimal(_) => {
                panic!("expected a Full installation payload for the event.")
            }
        };

        match specific {
            EventPayload::InstallationEvent(install) => {
                let repos = install.repositories;
                assert_eq!(repos.len(), 3);
                assert!(
                    repos.iter().any(|repo| repo.name == "ViscoElRebound"),
                    "ViscoElRebound should be in the list of repositories"
                );
                assert!(
                    repos.iter().any(|repo| repo.name == "OSSU"),
                    "OSSU should be in the list of repositories"
                );
                assert!(
                    repos.iter().any(|repo| repo.name == "octocrab"),
                    "octocrab should be in the list of repositories"
                );
            }
            EventPayload::PushEvent(_)
            | EventPayload::CreateEvent(_)
            | EventPayload::DeleteEvent(_)
            | EventPayload::InstallationRepositoriesEvent(_)
            | EventPayload::InstallationTargetEvent(_)
            | EventPayload::IssuesEvent(_)
            | EventPayload::IssueCommentEvent(_)
            | EventPayload::CommitCommentEvent(_)
            | EventPayload::ForkEvent(_)
            | EventPayload::GollumEvent(_)
            | EventPayload::MemberEvent(_)
            | EventPayload::PublicEvent(_)
            | EventPayload::PullRequestEvent(_)
            | EventPayload::PullRequestReviewEvent(_)
            | EventPayload::PullRequestReviewCommentEvent(_)
            | EventPayload::ReleaseEvent(_)
            | EventPayload::WorkflowRunEvent(_)
            | EventPayload::WatchEvent(_)
            | EventPayload::UnknownEvent(_) => {
                panic!("Expected an installation event, got {:?}", specific)
            }
        }
    }
}
