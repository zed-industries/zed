// The log shows v1 and v2 traffic. v1's ID type also covers the `null` ID of error
// responses, which v2 keeps even though v2 requests can't use it.
use agent_client_protocol::{Error, schema::v1::RequestId};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use util::ResultExt as _;

const MAX_DEBUG_BACKLOG_MESSAGES: usize = 2000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcpDebugMessageDirection {
    Incoming,
    Outgoing,
    Stderr,
}

#[derive(Clone)]
pub enum AcpDebugMessageContent {
    Request {
        id: RequestId,
        method: Arc<str>,
        params: Option<serde_json::Value>,
    },
    Response {
        id: RequestId,
        result: Result<Option<serde_json::Value>, Error>,
    },
    Notification {
        method: Arc<str>,
        params: Option<serde_json::Value>,
    },
    Stderr {
        line: Arc<str>,
    },
}

#[derive(Clone)]
pub struct AcpDebugMessage {
    pub direction: AcpDebugMessageDirection,
    pub message: AcpDebugMessageContent,
}

#[derive(Clone)]
struct ParsedDebugMessage {
    message: AcpDebugMessage,
    valid_envelope: bool,
}

impl AcpDebugMessage {
    fn parse_line(direction: AcpDebugMessageDirection, line: &str) -> Vec<ParsedDebugMessage> {
        if direction == AcpDebugMessageDirection::Stderr {
            return vec![ParsedDebugMessage {
                message: Self {
                    direction,
                    message: AcpDebugMessageContent::Stderr {
                        line: Arc::from(line),
                    },
                },
                valid_envelope: false,
            }];
        }

        let Ok(value) = serde_json::from_str(line) else {
            return Vec::new();
        };

        match value {
            serde_json::Value::Array(entries) => entries
                .into_iter()
                .filter_map(|entry| Self::parse_value(direction, entry))
                .collect(),
            value => Self::parse_value(direction, value).into_iter().collect(),
        }
    }

    fn parse_value(
        direction: AcpDebugMessageDirection,
        value: serde_json::Value,
    ) -> Option<ParsedDebugMessage> {
        let object = value.as_object()?;
        // Malformed frames are useful diagnostics, but cannot authorize protocol fallback.
        let valid_envelope = object.get("jsonrpc").and_then(serde_json::Value::as_str)
            == Some("2.0")
            && if object.contains_key("method") {
                object
                    .get("method")
                    .is_some_and(serde_json::Value::is_string)
                    && !object.contains_key("result")
                    && !object.contains_key("error")
            } else {
                object.contains_key("result") != object.contains_key("error")
            };

        let parsed_id = object
            .get("id")
            .map(|raw| serde_json::from_value::<RequestId>(raw.clone()));

        let message = if let Some(method) = object.get("method").and_then(|method| method.as_str())
        {
            match parsed_id {
                Some(Ok(id)) => AcpDebugMessageContent::Request {
                    id,
                    method: method.into(),
                    params: object.get("params").cloned(),
                },
                Some(Err(err)) => {
                    log::warn!("Skipping JSON-RPC message with unparsable id: {err}");
                    return None;
                }
                None => AcpDebugMessageContent::Notification {
                    method: method.into(),
                    params: object.get("params").cloned(),
                },
            }
        } else if let Some(parsed_id) = parsed_id {
            let id = match parsed_id {
                Ok(id) => id,
                Err(err) => {
                    log::warn!("Skipping JSON-RPC response with unparsable id: {err}");
                    return None;
                }
            };

            if let Some(error) = object.get("error") {
                let acp_error =
                    serde_json::from_value::<Error>(error.clone()).unwrap_or_else(|err| {
                        log::warn!("Failed to deserialize ACP error: {err}");
                        Error::internal_error().data(error.to_string())
                    });

                AcpDebugMessageContent::Response {
                    id,
                    result: Err(acp_error),
                }
            } else {
                AcpDebugMessageContent::Response {
                    id,
                    result: Ok(object.get("result").cloned()),
                }
            }
        } else {
            return None;
        };

        Some(ParsedDebugMessage {
            message: Self { direction, message },
            valid_envelope,
        })
    }
}

#[derive(Default)]
struct AcpDebugLogState {
    messages: VecDeque<AcpDebugMessage>,
    subscribers: Vec<async_channel::Sender<AcpDebugMessage>>,
    // Startup fallback must not depend on eviction from the diagnostic backlog.
    initialization: Option<InitializationExchange>,
}

struct InitializationExchange {
    request_id: RequestId,
    response: Option<ParsedDebugMessage>,
}

#[derive(Clone, Default)]
pub(super) struct AcpDebugLog {
    state: Arc<Mutex<AcpDebugLogState>>,
}

impl AcpDebugLog {
    pub(super) fn initialization_response(
        &self,
    ) -> Option<Result<Option<serde_json::Value>, Error>> {
        let state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let response = state.initialization.as_ref()?.response.as_ref()?;
        if !response.valid_envelope {
            return None;
        }
        match &response.message.message {
            AcpDebugMessageContent::Response { result, .. } => Some(result.clone()),
            _ => None,
        }
    }

    pub(super) fn subscribe(
        &self,
    ) -> (
        Vec<AcpDebugMessage>,
        async_channel::Receiver<AcpDebugMessage>,
    ) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let backlog = state.messages.iter().cloned().collect();
        let (sender, receiver) = async_channel::unbounded();
        state.subscribers.push(sender);
        (backlog, receiver)
    }

    pub(super) fn record_line(&self, direction: AcpDebugMessageDirection, line: &str) {
        let messages = AcpDebugMessage::parse_line(direction, line);
        if messages.is_empty() {
            return;
        }
        self.record_messages(messages);
    }

    fn record_messages(&self, messages: Vec<ParsedDebugMessage>) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        state.subscribers.retain(|sender| !sender.is_closed());
        for parsed in messages {
            let message = &parsed.message;
            match (message.direction, &message.message) {
                (
                    AcpDebugMessageDirection::Outgoing,
                    AcpDebugMessageContent::Request { id, method, .. },
                ) if parsed.valid_envelope
                    && method.as_ref() == "initialize"
                    && state.initialization.is_none() =>
                {
                    state.initialization = Some(InitializationExchange {
                        request_id: id.clone(),
                        response: None,
                    });
                }
                (
                    AcpDebugMessageDirection::Incoming,
                    AcpDebugMessageContent::Response { id, .. },
                ) => {
                    if let Some(initialization) = &mut state.initialization
                        && initialization.request_id == *id
                        && initialization.response.is_none()
                    {
                        initialization.response = Some(parsed.clone());
                    }
                }
                _ => {}
            }
            if state.messages.len() == MAX_DEBUG_BACKLOG_MESSAGES {
                state.messages.pop_front();
            }
            state.messages.push_back(message.clone());

            for sender in &state.subscribers {
                sender.try_send(message.clone()).log_err();
            }
        }
    }

    pub(super) fn trailing_stderr(&self) -> Option<String> {
        let state = self.state.lock().ok()?;
        let mut lines = state
            .messages
            .iter()
            .rev()
            // Client writes can race the agent's final stderr; only later
            // traffic from the agent makes those diagnostics stale.
            .filter(|message| message.direction != AcpDebugMessageDirection::Outgoing)
            .take_while(|message| matches!(&message.message, AcpDebugMessageContent::Stderr { .. }))
            .filter_map(|message| match &message.message {
                AcpDebugMessageContent::Stderr { line } if !line.is_empty() => Some(line.as_ref()),
                _ => None,
            })
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return None;
        }

        lines.reverse();
        Some(lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_initialization_envelopes_never_become_fallback_evidence() {
        for response in [
            r#"{"id":"initialize","result":{"protocolVersion":1}}"#,
            r#"{"jsonrpc":"1.0","id":"initialize","result":{"protocolVersion":1}}"#,
            r#"{"jsonrpc":"2.0","id":"initialize","result":{"protocolVersion":1},"error":{"code":-32022,"message":"Unsupported","data":{"requested":2,"supported":[1]}}}"#,
            r#"{"jsonrpc":"2.0","id":"initialize","method":null,"result":{"protocolVersion":1}}"#,
        ] {
            let log = AcpDebugLog::default();
            log.record_line(
                AcpDebugMessageDirection::Outgoing,
                r#"{"jsonrpc":"2.0","id":"initialize","method":"initialize","params":{"protocolVersion":2}}"#,
            );
            log.record_line(AcpDebugMessageDirection::Incoming, response);
            assert!(log.initialization_response().is_none(), "{response}");
            log.record_line(
                AcpDebugMessageDirection::Incoming,
                r#"{"jsonrpc":"2.0","id":"initialize","result":{"protocolVersion":1}}"#,
            );
            assert!(
                log.initialization_response().is_none(),
                "invalid first response must not be replaced"
            );
            assert_eq!(log.subscribe().0.len(), 3, "diagnostics remain visible");
        }
    }

    #[test]
    fn initialization_evidence_is_correlated_and_survives_backlog_eviction() {
        let log = AcpDebugLog::default();
        log.record_line(
            AcpDebugMessageDirection::Outgoing,
            r#"{"jsonrpc":"2.0","id":"initialize","method":"initialize","params":{"protocolVersion":2}}"#,
        );
        log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"{"jsonrpc":"2.0","id":"unrelated","result":{"protocolVersion":1}}"#,
        );
        assert!(log.initialization_response().is_none());
        log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"{"jsonrpc":"2.0","id":"initialize","error":{"code":-32022,"message":"Unsupported","data":{"requested":2,"supported":[1]}}}"#,
        );
        log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"{"jsonrpc":"2.0","id":"initialize","result":{"protocolVersion":2}}"#,
        );
        for _ in 0..MAX_DEBUG_BACKLOG_MESSAGES {
            log.record_line(
                AcpDebugMessageDirection::Incoming,
                r#"{"jsonrpc":"2.0","method":"_noise"}"#,
            );
        }
        let error = log
            .initialization_response()
            .expect("retained initialization")
            .expect_err("first response wins");
        assert_eq!(error.code, Error::new(-32022, "").code);
        assert_eq!(
            error.data,
            Some(serde_json::json!({"requested": 2, "supported": [1]}))
        );
        assert!(
            log.subscribe().0.iter().all(|message| matches!(
                message.message,
                AcpDebugMessageContent::Notification { .. }
            ))
        );
    }

    #[test]
    fn preserves_shared_request_ids_payloads_and_runtime_error_data() {
        let id = RequestId::Str("opaque/request:雪".into());
        let params = serde_json::json!({
            "sessionId": "session",
            "update": {"sessionUpdate": "_future", "payload": [null, 7]}
        });
        let request = serde_json::json!({
            "jsonrpc": "2.0", "id": id, "method": "session/update", "params": params
        });
        let requests =
            AcpDebugMessage::parse_line(AcpDebugMessageDirection::Outgoing, &request.to_string());
        let [request] = requests.as_slice() else {
            panic!("one request");
        };
        let AcpDebugMessageContent::Request {
            id: request_id,
            method,
            params: actual_params,
        } = &request.message.message
        else {
            panic!("request message");
        };
        assert_eq!(request_id, &id);
        assert_eq!(method.as_ref(), "session/update");
        assert_eq!(actual_params, &Some(params));

        let error = serde_json::json!({
            "code": -32999, "message": "Unknown runtime error",
            "data": {"retain": [null, false, {"nested": "雪"}]}
        });
        let response = serde_json::json!({"jsonrpc": "2.0", "id": id, "error": error});
        let responses =
            AcpDebugMessage::parse_line(AcpDebugMessageDirection::Incoming, &response.to_string());
        let [response] = responses.as_slice() else {
            panic!("one response");
        };
        let AcpDebugMessageContent::Response {
            id: response_id,
            result: Err(actual_error),
        } = &response.message.message
        else {
            panic!("error response");
        };
        assert_eq!(response_id, &id);
        assert_eq!(
            serde_json::to_value(actual_error).expect("runtime error"),
            error
        );
    }

    #[test]
    fn trailing_stderr_only_uses_final_stderr_block() {
        let debug_log = AcpDebugLog::default();
        debug_log.record_line(AcpDebugMessageDirection::Stderr, "stale stderr");
        debug_log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"{"method":"initialized"}"#,
        );

        assert_eq!(debug_log.trailing_stderr(), None);

        debug_log.record_line(AcpDebugMessageDirection::Stderr, "recent stderr");
        assert_eq!(
            debug_log.trailing_stderr().as_deref(),
            Some("recent stderr")
        );
    }

    #[test]
    fn trailing_stderr_is_not_hidden_by_client_traffic() {
        let debug_log = AcpDebugLog::default();
        debug_log.record_line(AcpDebugMessageDirection::Stderr, "startup error");
        debug_log.record_line(
            AcpDebugMessageDirection::Outgoing,
            r#"{"jsonrpc":"2.0","id":"startup","method":"initialize","params":{}}"#,
        );
        debug_log.record_line(AcpDebugMessageDirection::Stderr, "error detail");
        debug_log.record_line(
            AcpDebugMessageDirection::Outgoing,
            r#"{"jsonrpc":"2.0","method":"$/cancel_request","params":{"requestId":"startup"}}"#,
        );
        assert_eq!(
            debug_log.trailing_stderr().as_deref(),
            Some("startup error\nerror detail")
        );

        debug_log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"{"jsonrpc":"2.0","id":"startup","result":{}}"#,
        );
        debug_log.record_line(
            AcpDebugMessageDirection::Outgoing,
            r#"{"jsonrpc":"2.0","method":"session/cancel","params":{"sessionId":"session"}}"#,
        );
        assert_eq!(debug_log.trailing_stderr(), None);

        debug_log.record_line(AcpDebugMessageDirection::Stderr, "recent error");
        debug_log.record_line(
            AcpDebugMessageDirection::Outgoing,
            r#"{"jsonrpc":"2.0","id":"next","method":"session/new","params":{}}"#,
        );
        assert_eq!(debug_log.trailing_stderr().as_deref(), Some("recent error"));
    }

    #[test]
    fn debug_log_records_each_json_rpc_batch_entry() {
        let debug_log = AcpDebugLog::default();
        debug_log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"{"jsonrpc":"2.0","method":"legacy/update"}"#,
        );
        debug_log.record_line(
            AcpDebugMessageDirection::Incoming,
            r#"[
                {"jsonrpc":"2.0","method":"session/update","params":{"value":1}},
                null,
                [{"jsonrpc":"2.0","method":"nested/update"}],
                {"jsonrpc":"2.0","id":1,"method":"session/one","params":{"value":2}},
                {"jsonrpc":"2.0","id":{"invalid":true},"method":"invalid/id"}
            ]"#,
        );
        debug_log.record_line(
            AcpDebugMessageDirection::Outgoing,
            r#"[
                {"jsonrpc":"2.0","id":1,"result":{"accepted":true}},
                {"jsonrpc":"2.0","id":null,"error":{"code":-32600,"message":"Invalid Request"}}
            ]"#,
        );

        let (messages, _receiver) = debug_log.subscribe();
        let mut messages = messages.iter();

        assert!(matches!(
            messages.next(),
            Some(AcpDebugMessage {
                direction: AcpDebugMessageDirection::Incoming,
                message: AcpDebugMessageContent::Notification { method, .. },
            }) if method.as_ref() == "legacy/update"
        ));
        assert!(matches!(
            messages.next(),
            Some(AcpDebugMessage {
                direction: AcpDebugMessageDirection::Incoming,
                message: AcpDebugMessageContent::Notification { method, .. },
            }) if method.as_ref() == "session/update"
        ));
        assert!(matches!(
            messages.next(),
            Some(AcpDebugMessage {
                direction: AcpDebugMessageDirection::Incoming,
                message: AcpDebugMessageContent::Request { id, method, .. },
            }) if id == &RequestId::Number(1) && method.as_ref() == "session/one"
        ));
        assert!(matches!(
            messages.next(),
            Some(AcpDebugMessage {
                direction: AcpDebugMessageDirection::Outgoing,
                message: AcpDebugMessageContent::Response {
                    id,
                    result: Ok(Some(_)),
                },
            }) if id == &RequestId::Number(1)
        ));
        assert!(matches!(
            messages.next(),
            Some(AcpDebugMessage {
                direction: AcpDebugMessageDirection::Outgoing,
                message: AcpDebugMessageContent::Response {
                    id,
                    result: Err(_),
                },
            }) if id == &RequestId::Null
        ));
        assert!(messages.next().is_none());
    }
}
