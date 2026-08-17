pub mod payload;

use crate::models::events::payload::EventInstallation;

use self::payload::{
    CommitCommentEventPayload, CreateEventPayload, DeleteEventPayload, EventPayload,
    ForkEventPayload, GollumEventPayload, IssueCommentEventPayload, IssuesEventPayload,
    PublicEventPayload, PullRequestEventPayload, PullRequestReviewCommentEventPayload,
    PullRequestReviewEventPayload, PushEventPayload, ReleaseEventPayload, WatchEventPayload,
    WorkflowRunEventPayload, WrappedEventPayload,
};
use super::{ActorId, OrgId, RepositoryId};
use chrono::{DateTime, Utc};
use payload::MemberEventPayload;
use serde::{de::Error, Deserialize, Serialize};
use url::Url;

/// A GitHub event.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct Event {
    pub id: String,
    pub r#type: EventType,
    pub actor: Actor,
    pub repo: Repository,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub payload: Option<WrappedEventPayload>,
    pub org: Option<Org>,
}

macro_rules! event_type {
    ( $( ($name:ident, $payload:ident)),+ $(,)? ) => {
        /// The type of an event.
        #[derive(Debug, Clone, PartialEq, Deserialize)]
        #[non_exhaustive]
        pub enum EventType {
            $($name),+,
            UnknownEvent(String),
        }

        impl Serialize for EventType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(EventType::$name => serializer.serialize_str(stringify!($name))),+,
                    EventType::UnknownEvent(typ) => serializer.serialize_str(typ),
                }
            }
        }

        fn deserialize_event_type(event_type: &str) -> EventType {
            match event_type {
                $(stringify!($name) => EventType::$name),+,
                unknown => EventType::UnknownEvent(unknown.to_owned()),
            }
        }

        fn deserialize_payload(
            event_type: &EventType,
            data: serde_json::Value,
        ) -> Result<Option<EventPayload>, serde_json::Error> {
            let maybe_payload = match event_type {
                $(EventType::$name=> {
                    serde_json::from_value::<Box<$payload>>(data).map(EventPayload::$name)?
                }),+,
                _ => EventPayload::UnknownEvent(Box::new(data)),
            };
            Ok(Some(maybe_payload))
        }
    };
}

event_type! {
    (PushEvent, PushEventPayload),
    (CreateEvent, CreateEventPayload),
    (DeleteEvent, DeleteEventPayload),
    (IssuesEvent, IssuesEventPayload),
    (IssueCommentEvent, IssueCommentEventPayload),
    (CommitCommentEvent, CommitCommentEventPayload),
    (ForkEvent, ForkEventPayload),
    (GollumEvent, GollumEventPayload),
    (MemberEvent, MemberEventPayload),
    (PublicEvent, PublicEventPayload),
    (PullRequestEvent, PullRequestEventPayload),
    (PullRequestReviewEvent, PullRequestReviewEventPayload),
    (PullRequestReviewCommentEvent, PullRequestReviewCommentEventPayload),
    (ReleaseEvent, ReleaseEventPayload),
    (WatchEvent, WatchEventPayload),
    (WorkflowRunEvent, WorkflowRunEventPayload)
}

/// The repository an [`Event`] belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Repository {
    pub id: RepositoryId,
    pub name: String,
    pub url: Url,
}

/// The organization an [`Event`] belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Org {
    pub id: OrgId,
    pub login: String,
    pub gravatar_id: String,
    pub url: Url,
    pub avatar_url: Url,
}

/// The actor that created this [`Event`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Actor {
    pub id: ActorId,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: Url,
    pub avatar_url: Url,
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Intermediate {
            id: String,
            #[serde(rename = "type")]
            typ: String,
            actor: Actor,
            repo: Repository,
            public: bool,
            created_at: DateTime<Utc>,
            org: Option<Org>,
            payload: Option<IntermediatePayload>,
        }
        #[derive(Deserialize)]
        struct IntermediatePayload {
            installation: Option<EventInstallation>,
            organization: Option<crate::models::orgs::Organization>,
            repository: Option<crate::models::Repository>,
            sender: Option<crate::models::Author>,
            #[serde(flatten)]
            specific: Option<serde_json::Value>,
        }
        let intermediate = Intermediate::deserialize(deserializer)?;
        let event_type = deserialize_event_type(intermediate.typ.as_ref());
        let payload = intermediate.payload.map_or(Ok(None), |data| {
            let specific = deserialize_payload(
                &event_type,
                data.specific.unwrap_or(serde_json::Value::Null),
            )
            .map_err(|e| Error::custom(e.to_string()))?;
            Ok(Some(WrappedEventPayload {
                installation: data.installation,
                organization: data.organization,
                repository: data.repository,
                sender: data.sender,
                specific,
            }))
        })?;
        let event = Event {
            id: intermediate.id,
            r#type: event_type,
            actor: intermediate.actor,
            repo: intermediate.repo,
            public: intermediate.public,
            created_at: intermediate.created_at,
            org: intermediate.org,
            payload,
        };
        Ok(event)
    }
}

#[cfg(test)]
mod test {
    use super::{Event, EventPayload, EventType};
    use pretty_assertions::assert_eq;
    use url::Url;

    #[test]
    fn should_deserialize_push_event() {
        let json = include_str!("../../tests/resources/push_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::PushEvent);
    }

    #[test]
    fn should_deserialize_create_event() {
        let json = include_str!("../../tests/resources/create_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::CreateEvent);
    }

    #[test]
    fn should_deserialize_issues_event() {
        let json = include_str!("../../tests/resources/issues_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::IssuesEvent);
    }

    #[test]
    fn should_deserialize_issue_comment_event() {
        let json = include_str!("../../tests/resources/issue_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::IssueCommentEvent);
    }

    #[test]
    fn should_deserialize_pull_request_event() {
        let json = include_str!("../../tests/resources/pull_request_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::PullRequestEvent);
    }

    #[test]
    fn should_deserialize_pull_request_review_comment_event() {
        let json = include_str!("../../tests/resources/pull_request_review_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::PullRequestReviewCommentEvent);
    }

    #[test]
    fn should_deserialize_release_event() {
        let json = include_str!("../../tests/resources/release_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::ReleaseEvent);
    }

    #[test]
    fn should_deserialize_workflow_run_event() {
        let json = include_str!("../../tests/resources/workflow_run_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::WorkflowRunEvent);
        assert_eq!(
            event.payload.unwrap().installation.unwrap(),
            crate::models::events::payload::EventInstallation::Minimal(Box::new(
                crate::models::events::payload::EventInstallationId {
                    id: 18995746.into(),
                    node_id: "MDIzOkludGVncmF0aW9uSW5zdGFsbGF0aW9uMTg5OTU3NDY=".to_string()
                }
            ))
        )
    }

    #[test]
    fn should_deserialize_commit_comment_event() {
        let json = include_str!("../../tests/resources/commit_comment_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::CommitCommentEvent);
    }

    #[test]
    fn should_deserialize_delete_event() {
        let json = include_str!("../../tests/resources/delete_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::DeleteEvent);
    }

    #[test]
    fn should_deserialize_fork_event() {
        let json = include_str!("../../tests/resources/fork_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::ForkEvent);
    }

    #[test]
    fn should_deserialize_gollum_event() {
        let json = include_str!("../../tests/resources/gollum_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::GollumEvent);
    }

    #[test]
    fn should_deserialize_member_event() {
        let json = include_str!("../../tests/resources/member_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::MemberEvent);
    }

    #[test]
    fn should_deserialize_watch_event() {
        let json = include_str!("../../tests/resources/watch_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.r#type, EventType::WatchEvent);
    }

    #[test]
    fn should_deserialize_with_org_when_present() {
        let json = include_str!("../../tests/resources/create_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.org.is_some());
        let org = event.org.unwrap();
        assert_eq!(org.id.0, 1243215);
        assert_eq!(org.login, "hypothesis");
        assert_eq!(org.gravatar_id, "");
        assert_eq!(
            org.url,
            Url::parse("https://api.github.com/orgs/hypothesis").unwrap()
        );
        assert_eq!(
            org.avatar_url,
            Url::parse("https://avatars.githubusercontent.com/u/1243215?").unwrap()
        );
    }

    #[test]
    fn should_deserialize_unknown_event_payload() {
        let json = include_str!("../../tests/resources/unknown_event.json");
        let event: Event = serde_json::from_str(json).unwrap();
        assert!(event.payload.is_some());
        let payload = event.payload.unwrap();
        match payload.specific.unwrap() {
            EventPayload::UnknownEvent(json) => {
                assert!(json.is_object());
                let map = json.as_object().unwrap();
                assert_eq!(map.get("ref").unwrap(), "Core.GetText");
                assert_eq!(map.get("ref_type").unwrap(), "branch");
                assert_eq!(map.get("pusher_type").unwrap(), "user");
            }
            _ => panic!("unexpected event deserialized"),
        }
    }

    #[test]
    fn events_should_serialize_and_deserialize_correctly() {
        let event_types = [
            (
                "CreateEvent",
                include_str!("../../tests/resources/create_event.json"),
            ),
            (
                "PushEvent",
                include_str!("../../tests/resources/push_event.json"),
            ),
            (
                "ForkEvent",
                include_str!("../../tests/resources/fork_event.json"),
            ),
            (
                "DeleteEvent",
                include_str!("../../tests/resources/delete_event.json"),
            ),
            (
                "GollumEvent",
                include_str!("../../tests/resources/gollum_event.json"),
            ),
            (
                "IssuesEvent",
                include_str!("../../tests/resources/issues_event.json"),
            ),
            (
                "IssueCommentEvent",
                include_str!("../../tests/resources/issue_comment_event.json"),
            ),
            (
                "MemberEvent",
                include_str!("../../tests/resources/member_event.json"),
            ),
            (
                "PullRequestEvent",
                include_str!("../../tests/resources/pull_request_event.json"),
            ),
            (
                "PullRequestReviewCommentEvent",
                include_str!("../../tests/resources/pull_request_review_comment_event.json"),
            ),
            (
                "ReleaseEvent",
                include_str!("../../tests/resources/release_event.json"),
            ),
            (
                "CommitCommentEvent",
                include_str!("../../tests/resources/commit_comment_event.json"),
            ),
            (
                "WatchEvent",
                include_str!("../../tests/resources/watch_event.json"),
            ),
            (
                "WorkflowRunEvent",
                include_str!("../../tests/resources/workflow_run_event.json"),
            ),
            (
                "UnknownEvent",
                include_str!("../../tests/resources/unknown_event.json"),
            ),
        ];
        for (event_type, json) in event_types {
            serialize_and_deserialize(event_type, json);
        }
    }

    fn serialize_and_deserialize(event_type: &str, json: &str) {
        let event: Event = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&event).unwrap();
        // do it again so we can compare Events, otherwise we are comparing strings which may have
        // different whitespace characteristics.
        let deserialized = serde_json::from_str::<Event>(&serialized);
        assert!(
            deserialized.is_ok(),
            "expected deserialized result for {} to be ok, got error instead {:?}",
            event_type,
            deserialized
        );
        let deserialized = deserialized.unwrap();
        assert_eq!(
            deserialized, event,
            "unexpected event deserialized for {event_type}"
        );
    }
}
