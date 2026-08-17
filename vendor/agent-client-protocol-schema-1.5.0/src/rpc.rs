//! JSON-RPC envelope types shared by ACP clients and agents.
//!
//! These types model the JSON-RPC 2.0 request, response, notification, and
//! batch envelopes that wrap ACP method-specific payloads.

use std::sync::Arc;

use derive_more::{Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// JSON RPC Request Id
///
/// An identifier established by the Client that MUST contain a String, Number, or NULL value if included. If it is not included it is assumed to be a notification. The value SHOULD normally not be Null \[1\] and Numbers SHOULD NOT contain fractional parts \[2\]
///
/// The Server MUST reply with the same value in the Response object if included. This member is used to correlate the context between the two objects.
///
/// \[1\] The use of Null as a value for the id member in a Request object is discouraged, because this specification uses a value of Null for Responses with an unknown id. Also, because JSON-RPC 1.0 uses an id value of Null for Notifications this could cause confusion in handling.
///
/// \[2\] Fractional parts may be problematic, since many decimal fractions cannot be represented exactly as binary fractions.
#[derive(
    Debug,
    PartialEq,
    Clone,
    Hash,
    Eq,
    Deserialize,
    Serialize,
    PartialOrd,
    Ord,
    Display,
    JsonSchema,
    From,
)]
#[serde(untagged)]
#[allow(
    clippy::exhaustive_enums,
    reason = "This comes from the JSON-RPC specification itself"
)]
#[from(String, i64)]
pub enum RequestId {
    /// The JSON-RPC `null` request id.
    #[display("null")]
    Null,
    /// A numeric JSON-RPC request id.
    Number(i64),
    /// A string JSON-RPC request id.
    Str(String),
}

/// A JSON-RPC request object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[allow(
    clippy::exhaustive_structs,
    reason = "This comes from the JSON-RPC specification itself"
)]
#[schemars(rename = "{Params}", extend("x-docs-ignore" = true))]
#[skip_serializing_none]
pub struct Request<Params> {
    /// The request id used to correlate the matching response.
    pub id: RequestId,
    /// The method name to invoke.
    pub method: Arc<str>,
    /// Method-specific request parameters.
    pub params: Option<Params>,
}

/// A JSON-RPC response object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[allow(
    clippy::exhaustive_enums,
    reason = "This comes from the JSON-RPC specification itself"
)]
#[serde(untagged)]
#[schemars(rename = "{Result}", extend("x-docs-ignore" = true))]
pub enum Response<Result, Error> {
    /// A successful JSON-RPC response.
    Result {
        /// The id of the request this response answers.
        id: RequestId,
        /// Method-specific response data.
        result: Result,
    },
    /// A failed JSON-RPC response.
    Error {
        /// The id of the request this response answers.
        id: RequestId,
        /// Method-specific error data.
        error: Error,
    },
}

impl<R, E> Response<R, E> {
    /// Creates a JSON-RPC response from a Rust [`Result`].
    #[must_use]
    pub fn new(id: impl Into<RequestId>, result: std::result::Result<R, E>) -> Self {
        match result {
            Ok(result) => Self::Result {
                id: id.into(),
                result,
            },
            Err(error) => Self::Error {
                id: id.into(),
                error,
            },
        }
    }
}

/// A JSON-RPC notification object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[allow(
    clippy::exhaustive_structs,
    reason = "This comes from the JSON-RPC specification itself"
)]
#[schemars(rename = "{Params}", extend("x-docs-ignore" = true))]
#[skip_serializing_none]
pub struct Notification<Params> {
    /// The notification method name.
    pub method: Arc<str>,
    /// Method-specific notification parameters.
    pub params: Option<Params>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(inline)]
enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2,
}

/// A message (request, response, or notification) with `"jsonrpc": "2.0"` specified as
/// [required by JSON-RPC 2.0 Specification][1].
///
/// [1]: https://www.jsonrpc.org/specification#compatibility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(inline)]
pub struct JsonRpcMessage<M> {
    jsonrpc: JsonRpcVersion,
    #[serde(flatten)]
    message: M,
}

impl<M> JsonRpcMessage<M> {
    /// Wraps the provided message into a versioned [`JsonRpcMessage`].
    #[must_use]
    pub fn wrap(message: M) -> Self {
        Self {
            jsonrpc: JsonRpcVersion::V2,
            message,
        }
    }

    /// Returns the contained message.
    #[must_use]
    pub fn inner(&self) -> &M {
        &self.message
    }

    /// Unwraps the contained message.
    #[must_use]
    pub fn into_inner(self) -> M {
        self.message
    }
}

/// Error returned when constructing an empty JSON-RPC batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[display("JSON-RPC batch must contain at least one message")]
#[non_exhaustive]
pub struct EmptyJsonRpcBatch;

impl std::error::Error for EmptyJsonRpcBatch {}

/// A non-empty JSON-RPC 2.0 batch message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
#[schemars(inline)]
#[serde(transparent)]
#[allow(
    clippy::exhaustive_structs,
    reason = "This comes from the JSON-RPC specification itself"
)]
pub struct JsonRpcBatch<M>(#[schemars(length(min = 1))] Vec<JsonRpcMessage<M>>);

impl<M> JsonRpcBatch<M> {
    /// Creates a non-empty JSON-RPC batch.
    ///
    /// Returns an error if `messages` is empty, because JSON-RPC 2.0 treats an
    /// empty batch array as an invalid request.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyJsonRpcBatch`] when `messages` is empty.
    pub fn new(messages: Vec<JsonRpcMessage<M>>) -> Result<Self, EmptyJsonRpcBatch> {
        if messages.is_empty() {
            Err(EmptyJsonRpcBatch)
        } else {
            Ok(Self(messages))
        }
    }

    /// Returns the messages in this batch.
    #[must_use]
    pub fn as_slice(&self) -> &[JsonRpcMessage<M>] {
        &self.0
    }

    /// Consumes this batch and returns its messages.
    #[must_use]
    pub fn into_vec(self) -> Vec<JsonRpcMessage<M>> {
        self.0
    }
}

impl<M> TryFrom<Vec<JsonRpcMessage<M>>> for JsonRpcBatch<M> {
    type Error = EmptyJsonRpcBatch;

    fn try_from(messages: Vec<JsonRpcMessage<M>>) -> Result<Self, Self::Error> {
        Self::new(messages)
    }
}

impl<'de, M> Deserialize<'de> for JsonRpcBatch<M>
where
    M: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let messages = Vec::<JsonRpcMessage<M>>::deserialize(deserializer)?;
        Self::new(messages).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::v1::{
        AgentNotification, CancelNotification, ClientNotification, ContentBlock, ContentChunk,
        SessionId, SessionNotification, SessionUpdate, TextContent,
    };
    use serde_json::{Number, Value, json};

    #[test]
    fn id_deserialization() {
        let id = serde_json::from_value::<RequestId>(Value::Null).unwrap();
        assert_eq!(id, RequestId::Null);

        let id = serde_json::from_value::<RequestId>(Value::Number(Number::from_u128(1).unwrap()))
            .unwrap();
        assert_eq!(id, RequestId::Number(1));

        let id = serde_json::from_value::<RequestId>(Value::Number(Number::from_i128(-1).unwrap()))
            .unwrap();
        assert_eq!(id, RequestId::Number(-1));

        let id = serde_json::from_value::<RequestId>(Value::String("id".to_owned())).unwrap();
        assert_eq!(id, RequestId::Str("id".to_owned()));
    }

    #[test]
    fn id_serialization() {
        let id = serde_json::to_value(RequestId::Null).unwrap();
        assert_eq!(id, Value::Null);

        let id = serde_json::to_value(RequestId::Number(1)).unwrap();
        assert_eq!(id, Value::Number(Number::from_u128(1).unwrap()));

        let id = serde_json::to_value(RequestId::Number(-1)).unwrap();
        assert_eq!(id, Value::Number(Number::from_i128(-1).unwrap()));

        let id = serde_json::to_value(RequestId::Str("id".to_owned())).unwrap();
        assert_eq!(id, Value::String("id".to_owned()));
    }

    #[test]
    fn id_display() {
        let id = RequestId::Null;
        assert_eq!(id.to_string(), "null");

        let id = RequestId::Number(1);
        assert_eq!(id.to_string(), "1");

        let id = RequestId::Number(-1);
        assert_eq!(id.to_string(), "-1");

        let id = RequestId::Str("id".to_owned());
        assert_eq!(id.to_string(), "id");
    }

    #[test]
    fn batch_deserialization_requires_at_least_one_message() {
        let err = serde_json::from_value::<JsonRpcBatch<Notification<ClientNotification>>>(
            Value::Array(Vec::new()),
        )
        .unwrap_err();
        assert!(err.to_string().contains("at least one message"));
    }

    #[test]
    fn batch_serialization_round_trips_non_empty_messages() {
        let notification = JsonRpcMessage::wrap(Notification {
            method: "cancel".into(),
            params: Some(ClientNotification::CancelNotification(CancelNotification {
                session_id: SessionId("test-123".into()),
                meta: None,
            })),
        });

        let batch = JsonRpcBatch::new(vec![notification]).unwrap();
        let serialized = serde_json::to_value(&batch).unwrap();
        assert_eq!(
            serialized,
            json!([{
                "jsonrpc": "2.0",
                "method": "cancel",
                "params": {
                    "sessionId": "test-123"
                },
            }])
        );

        let deserialized =
            serde_json::from_value::<JsonRpcBatch<Notification<ClientNotification>>>(serialized)
                .unwrap();
        assert_eq!(deserialized.as_slice().len(), 1);
        assert_eq!(deserialized.as_slice()[0].inner().method.as_ref(), "cancel");
    }

    #[test]
    fn notification_wire_format() {
        // Test client -> agent notification wire format
        let outgoing_msg = JsonRpcMessage::wrap(Notification {
            method: "cancel".into(),
            params: Some(ClientNotification::CancelNotification(CancelNotification {
                session_id: SessionId("test-123".into()),
                meta: None,
            })),
        });

        let serialized: Value = serde_json::to_value(&outgoing_msg).unwrap();
        assert_eq!(
            serialized,
            json!({
                "jsonrpc": "2.0",
                "method": "cancel",
                "params": {
                    "sessionId": "test-123"
                },
            })
        );

        // Test agent -> client notification wire format
        let outgoing_msg = JsonRpcMessage::wrap(Notification {
            method: "sessionUpdate".into(),
            params: Some(AgentNotification::SessionNotification(
                SessionNotification {
                    session_id: SessionId("test-456".into()),
                    update: SessionUpdate::AgentMessageChunk(ContentChunk {
                        content: ContentBlock::Text(TextContent {
                            annotations: None,
                            text: "Hello".to_string(),
                            meta: None,
                        }),
                        message_id: None,
                        meta: None,
                    }),
                    meta: None,
                },
            )),
        });

        let serialized: Value = serde_json::to_value(&outgoing_msg).unwrap();
        assert_eq!(
            serialized,
            json!({
                "jsonrpc": "2.0",
                "method": "sessionUpdate",
                "params": {
                    "sessionId": "test-456",
                    "update": {
                        "sessionUpdate": "agent_message_chunk",
                        "content": {
                            "type": "text",
                            "text": "Hello"
                        }
                    }
                }
            })
        );
    }
}
