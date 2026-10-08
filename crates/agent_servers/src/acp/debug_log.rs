use agent_client_protocol::schema::v1 as acp;
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
        id: acp::RequestId,
        method: Arc<str>,
        params: Option<serde_json::Value>,
    },
    Response {
        id: acp::RequestId,
        result: Result<Option<serde_json::Value>, acp::Error>,
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

impl AcpDebugMessage {
    fn parse_line(direction: AcpDebugMessageDirection, line: &str) -> Vec<Self> {
        if direction == AcpDebugMessageDirection::Stderr {
            return vec![Self {
                direction,
                message: AcpDebugMessageContent::Stderr {
                    line: Arc::from(line),
                },
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

    fn parse_value(direction: AcpDebugMessageDirection, value: serde_json::Value) -> Option<Self> {
        let object = value.as_object()?;

        let parsed_id = object
            .get("id")
            .map(|raw| serde_json::from_value::<acp::RequestId>(raw.clone()));

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
                    serde_json::from_value::<acp::Error>(error.clone()).unwrap_or_else(|err| {
                        log::warn!("Failed to deserialize ACP error: {err}");
                        acp::Error::internal_error().data(error.to_string())
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

        Some(Self { direction, message })
    }
}

#[derive(Default)]
struct AcpDebugLogState {
    messages: VecDeque<AcpDebugMessage>,
    subscribers: Vec<async_channel::Sender<AcpDebugMessage>>,
}

#[derive(Clone, Default)]
pub(super) struct AcpDebugLog {
    state: Arc<Mutex<AcpDebugLogState>>,
}

impl AcpDebugLog {
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

    fn record_messages(&self, messages: Vec<AcpDebugMessage>) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        state.subscribers.retain(|sender| !sender.is_closed());
        for message in messages {
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
            }) if id == &acp::RequestId::Number(1) && method.as_ref() == "session/one"
        ));
        assert!(matches!(
            messages.next(),
            Some(AcpDebugMessage {
                direction: AcpDebugMessageDirection::Outgoing,
                message: AcpDebugMessageContent::Response {
                    id,
                    result: Ok(Some(_)),
                },
            }) if id == &acp::RequestId::Number(1)
        ));
        assert!(matches!(
            messages.next(),
            Some(AcpDebugMessage {
                direction: AcpDebugMessageDirection::Outgoing,
                message: AcpDebugMessageContent::Response {
                    id,
                    result: Err(_),
                },
            }) if id == &acp::RequestId::Null
        ));
        assert!(messages.next().is_none());
    }
}
