use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::Deref,
};

use derive_more::{
    Deref,
    FromStr,
};
#[cfg(any(test, feature = "testing"))]
use proptest::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[cfg(any(test, feature = "testing"))]
use crate::testing::arb_json;
use crate::{
    Timestamp,
    UdfPath,
};

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize, Hash,
)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub struct QueryId(u32);

impl QueryId {
    pub fn new(id: u32) -> Self {
        QueryId(id)
    }

    pub fn get_id(&self) -> u32 {
        self.0
    }
}

impl Display for QueryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type QuerySetVersion = u32;
pub type IdentityVersion = u32;

/// This strategy only generates vectors of strings (not arbitrary JSON) but
/// it's good enough for our tests here.
#[cfg(any(test, feature = "testing"))]
fn string_json_args_strategy() -> impl proptest::strategy::Strategy<Value = Vec<JsonValue>> {
    prop::collection::vec(any::<String>(), 0..2)
        .prop_map(|v| v.iter().map(|s| JsonValue::String(s.into())).collect())
}

#[cfg(any(test, feature = "testing"))]
fn custom_claims_strategy() -> impl proptest::strategy::Strategy<Value = BTreeMap<String, String>> {
    prop::collection::btree_map(
        any::<String>(),
        arb_json().prop_map(|v| serde_json::to_string(&v).unwrap()),
        0..2,
    )
}

/// This strategy only generates a string (not arbitrary JSON) but
/// it's good enough for our tests here.
#[cfg(any(test, feature = "testing"))]
fn string_json_arg_strategy() -> impl proptest::strategy::Strategy<Value = JsonValue> {
    String::arbitrary().prop_map(JsonValue::String)
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub struct Query {
    pub query_id: QueryId,
    pub udf_path: UdfPath,
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "string_json_args_strategy()")
    )]
    pub args: Vec<JsonValue>,

    /// Query journals are only specified on reconnect. Also old clients
    /// (<=0.2.1) don't send them.
    pub journal: Option<SerializedQueryJournal>,

    /// For internal use by Convex dashboard. Only works with admin auth.
    /// Allows calling a query within a component directly.
    pub component_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub enum QuerySetModification {
    Add(Query),
    Remove { query_id: QueryId },
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub enum ClientMessage {
    Connect {
        session_id: SessionId,
        connection_count: u32,
        last_close_reason: String,
        max_observed_timestamp: Option<Timestamp>,
    },
    ModifyQuerySet {
        base_version: QuerySetVersion,
        new_version: QuerySetVersion,
        #[cfg_attr(
            any(test, feature = "testing"),
            proptest(strategy = "prop::collection::vec(any::<QuerySetModification>(), 0..2)")
        )]
        modifications: Vec<QuerySetModification>,
    },
    Mutation {
        request_id: SessionRequestSeqNumber,
        udf_path: UdfPath,
        #[cfg_attr(
            any(test, feature = "testing"),
            proptest(strategy = "string_json_args_strategy()")
        )]
        args: Vec<JsonValue>,
        /// For internal use by Convex dashboard. Only works with admin auth.
        /// Allows calling a mutation within a component directly.
        component_path: Option<String>,
    },
    Action {
        request_id: SessionRequestSeqNumber,
        udf_path: UdfPath,
        #[cfg_attr(
            any(test, feature = "testing"),
            proptest(strategy = "string_json_args_strategy()")
        )]
        args: Vec<JsonValue>,
        /// For internal use by Convex dashboard. Only works with admin auth.
        /// Allows calling an action within a component directly.
        component_path: Option<String>,
    },
    Authenticate {
        base_version: IdentityVersion,
        token: AuthenticationToken,
    },
    Event(ClientEvent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub struct ClientEvent {
    pub event_type: String,
    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "string_json_arg_strategy()")
    )]
    pub event: JsonValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
#[serde(transparent)]
pub struct UserIdentifier(pub String);
impl UserIdentifier {
    pub fn construct(issuer_name: &str, subject: &str) -> Self {
        Self(format!("{issuer_name}|{subject}"))
    }
}

impl Deref for UserIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Make issuer and subject not optional to match TypeScript
// type and runtime behavior. Requires all FunctionTesters
// to require them.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub struct UserIdentityAttributes {
    pub token_identifier: UserIdentifier,
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub nickname: Option<String>,
    pub preferred_username: Option<String>,
    pub profile_url: Option<String>,
    pub picture_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_verified: Option<bool>,
    pub address: Option<String>,
    /// Stored as RFC3339 string
    pub updated_at: Option<String>,

    #[cfg_attr(
        any(test, feature = "testing"),
        proptest(strategy = "custom_claims_strategy()")
    )]
    pub custom_claims: BTreeMap<String, String>,
}

impl Default for UserIdentityAttributes {
    fn default() -> Self {
        UserIdentityAttributes {
            token_identifier: UserIdentifier::construct("convex", "fake_user"),
            subject: None,
            issuer: None,
            name: None,
            email: None,
            given_name: None,
            family_name: None,
            nickname: None,
            preferred_username: None,
            profile_url: None,
            picture_url: None,
            website_url: None,
            email_verified: None,
            gender: None,
            birthday: None,
            timezone: None,
            language: None,
            phone_number: None,
            phone_number_verified: None,
            address: None,
            updated_at: None,
            custom_claims: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub enum AuthenticationToken {
    /// Admin key issued by a KeyBroker, potentially acting as a user.
    Admin(String, Option<UserIdentityAttributes>),
    /// OpenID Connect JWT
    User(String),
    #[default]
    /// Logged out.
    None,
}

/// The serialized representation of the query journal for pagination.
pub type SerializedQueryJournal = Option<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub enum StateModification<V> {
    QueryUpdated {
        query_id: QueryId,
        value: V,
        log_lines: LogLinesMessage,
        journal: SerializedQueryJournal,
    },
    QueryFailed {
        query_id: QueryId,
        error_message: String,
        log_lines: LogLinesMessage,
        journal: SerializedQueryJournal,
        error_data: Option<V>,
    },
    QueryRemoved {
        query_id: QueryId,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub struct StateVersion {
    pub query_set: QuerySetVersion,
    pub identity: IdentityVersion,
    pub ts: Timestamp,
}

impl StateVersion {
    pub fn initial() -> Self {
        Self {
            query_set: 0,
            identity: 0,
            ts: Timestamp::MIN,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub enum ServerMessage<V: 'static> {
    Transition {
        start_version: StateVersion,
        end_version: StateVersion,
        #[cfg_attr(
            test,
            proptest(strategy = "prop::collection::vec(any::<StateModification<V>>(), 0..8)")
        )]
        modifications: Vec<StateModification<V>>,
    },
    MutationResponse {
        request_id: SessionRequestSeqNumber,
        result: Result<V, ErrorPayload<V>>,
        ts: Option<Timestamp>,
        log_lines: LogLinesMessage,
    },
    ActionResponse {
        request_id: SessionRequestSeqNumber,
        result: Result<V, ErrorPayload<V>>,
        log_lines: LogLinesMessage,
    },
    AuthError {
        error_message: String,
        base_version: Option<IdentityVersion>,
        // We want to differentiate between "updating auth starting at version `base_version`
        // failed" and "auth at version `base_version` is invalid" (e.g. it expired)
        auth_update_attempted: Option<bool>,
    },
    FatalError {
        error_message: String,
    },
    Ping,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub enum ErrorPayload<V: 'static> {
    /// From any error, redacted from prod deployments.
    Message(String),
    /// From ConvexError, never redacted.
    /// `message` is generic, partly for backwards compatibility.
    ErrorData { message: String, data: V },
}

impl<V: 'static> ErrorPayload<V> {
    pub fn get_message(&self) -> &str {
        match self {
            ErrorPayload::Message(message) => message,
            ErrorPayload::ErrorData { message, .. } => message,
        }
    }

    pub fn get_data(&self) -> Option<&V> {
        match self {
            ErrorPayload::Message(..) => None,
            ErrorPayload::ErrorData { message: _, data } => Some(data),
        }
    }
}

/// List of log lines from a Convex function execution.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "testing"), derive(proptest_derive::Arbitrary))]
pub struct LogLinesMessage(pub Vec<String>);

#[derive(Copy, Clone, Debug, Deref, Eq, FromStr, PartialEq)]
pub struct SessionId(Uuid);

impl SessionId {
    #[cfg(any(test, feature = "testing"))]
    pub fn nil() -> Self {
        Self(Uuid::nil())
    }

    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<SessionId> for Uuid {
    fn from(id: SessionId) -> Self {
        id.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for SessionId {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        "[a-f0-9]{32}"
            .prop_map(|s| s.parse().expect("Invalid Uuid"))
            .boxed()
    }
}

// The seq number of a request with a session. Uniquely identifies a
// modification request within a session.
pub type SessionRequestSeqNumber = u32;
