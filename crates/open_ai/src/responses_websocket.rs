//! WebSocket mode for the Responses API.
//!
//! In WebSocket mode a persistent connection runs each turn by sending a
//! `response.create` event and continues a conversation by sending only the
//! new input items plus `previous_response_id`, instead of re-sending the
//! whole conversation. This module owns the transport-agnostic session
//! machinery: preparing a request for incremental sending, caching completed
//! connections ("chains") keyed by what the server already has, and draining
//! response events. Callers supply how to connect and how to frame a turn
//! message, so the same machinery drives both direct `api.openai.com`
//! connections and proxied transports that tunnel the same payloads.

use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, anyhow};
use futures::channel::mpsc;
use futures::future::{self, BoxFuture};
use futures::stream::BoxStream;
use futures::{Future, StreamExt as _};
use parking_lot::Mutex;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use websocket_client::{Timer, WebSocketConnection, WebSocketMessage};

use crate::responses::{Request, ResponseOutputItem, StreamEvent};

const MAX_WEBSOCKET_CHAINS: usize = 8;
const WEBSOCKET_CHAIN_IDLE_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const REUSED_CONNECTION_CONFIRMATION_TIMEOUT: Duration = Duration::from_secs(30);

/// Completed WebSocket connections cached for continuation, shared by all
/// models of a provider. Chains for different models never match because the
/// request configuration participates in the input hashes.
pub type SharedWebSocketChains = Arc<Mutex<WebSocketChains>>;

#[derive(Default)]
pub struct WebSocketChains {
    chains: Vec<WebSocketChain>,
}

impl WebSocketChains {
    pub fn new_shared() -> SharedWebSocketChains {
        Arc::default()
    }

    /// Drops all cached connections, e.g. when credentials change.
    pub fn clear(&mut self) {
        self.chains.clear();
    }
}

struct WebSocketChain {
    /// The [`PreparedWebSocketRequest::input_hash`] of the request that
    /// produced the cached response.
    input_hash: [u8; 32],
    previous_response_id: String,
    response_output_fingerprints: Vec<ResponseItemFingerprint>,
    connection: Box<dyn WebSocketConnection>,
    last_used_at: Instant,
}

/// Identifies a response output item by the fields that survive the round
/// trip through the conversation and back into the next request's input.
///
/// Byte-level comparison against the raw output items would not work: the
/// replay restructures them (reasoning items move to the front of the
/// assistant message, output messages lose their server-assigned ids and
/// statuses), so only content that is preserved verbatim is compared.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ResponseItemFingerprint {
    FunctionCall {
        call_id: String,
        name: String,
        arguments: String,
    },
    CustomToolCall {
        call_id: String,
        name: String,
        input: String,
    },
    Reasoning {
        id: Option<String>,
        encrypted_content: Option<String>,
    },
    AssistantMessage {
        text: String,
    },
    Compaction {
        id: Option<String>,
        encrypted_content: String,
    },
}

struct PreparedWebSocketRequest {
    input: Vec<Value>,
    /// Cumulative hashes seeded with the hash of the request configuration
    /// (the payload minus the input): entry `n` commits to the configuration
    /// plus the first `n` input items, so equal hashes imply an equal
    /// configuration as well as an equal input prefix.
    input_prefix_hashes: Vec<[u8; 32]>,
    /// The last entry of `input_prefix_hashes`: the configuration plus the
    /// entire input.
    input_hash: [u8; 32],
    payload: Map<String, Value>,
}

/// Frames a turn payload as a direct `response.create` event for
/// `api.openai.com`. Transports that tunnel the payload in their own
/// envelope pass their own callback to [`stream_websocket_response`]
/// instead.
pub fn response_create_envelope(mut turn_payload: Map<String, Value>) -> Result<String> {
    turn_payload.insert("type".into(), Value::String("response.create".into()));
    Ok(Value::Object(turn_payload).to_string())
}

/// Runs one completion turn over a WebSocket session.
///
/// Reuses a cached connection whose complete server-side state covers a
/// strict prefix of the request's input, sending only the remaining items
/// plus `previous_response_id`; otherwise awaits `connect` and sends the
/// full input. `envelope_turn` frames the turn payload (the serialized request
/// with the input slice, minus `stream`) into a wire message. `spawn` is
/// called with the background future that drains response events into the
/// returned stream; the caller must run it to completion (typically
/// detached on an executor).
pub async fn stream_websocket_response(
    request: &Request,
    connection_scope: &[u8],
    websocket_chains: SharedWebSocketChains,
    connect: impl Future<Output = Result<Box<dyn WebSocketConnection>>>,
    timer: Timer,
    envelope_turn: impl Fn(Map<String, Value>) -> Result<String>,
    spawn: impl FnOnce(BoxFuture<'static, ()>),
) -> Result<BoxStream<'static, Result<StreamEvent>>> {
    let prepared = prepare_websocket_request(request, connection_scope)?;
    let now = Instant::now();
    let continuation = {
        let mut chains = websocket_chains.lock();
        chains
            .chains
            .retain(|chain| now.duration_since(chain.last_used_at) < WEBSOCKET_CHAIN_IDLE_TIMEOUT);
        best_continuation_chain(&chains.chains, &prepared).map(|(index, covered_input_count)| {
            (chains.chains.swap_remove(index), covered_input_count)
        })
    };

    let mut confirmed_continuation = None;
    if let Some((chain, covered_input_count)) = continuation {
        log::debug!(
            "OpenAI Responses transport: WebSocket continuation; reused_items={}, new_items={}",
            covered_input_count,
            prepared.input.len().saturating_sub(covered_input_count)
        );
        let message = envelope_turn(turn_payload(
            &prepared,
            Some(&chain.previous_response_id),
            covered_input_count,
        ))?;
        match send_and_confirm_reused_connection(
            chain.connection,
            message,
            (timer)(REUSED_CONNECTION_CONFIRMATION_TIMEOUT),
        )
        .await
        {
            Ok(connection_and_first_event) => {
                confirmed_continuation = Some(connection_and_first_event);
            }
            Err(error) => log::debug!(
                "OpenAI Responses transport: cached WebSocket connection is stale ({error:#}); reconnecting"
            ),
        }
    }

    let (connection, first_event_text) = match confirmed_continuation {
        Some((connection, first_event_text)) => (connection, Some(first_event_text)),
        None => {
            log::debug!(
                "OpenAI Responses transport: WebSocket new connection; input_items={}",
                prepared.input.len()
            );
            let mut connection = connect.await?;
            connection
                .send(WebSocketMessage::Text(envelope_turn(turn_payload(
                    &prepared, None, 0,
                ))?))
                .await
                .context("failed to send OpenAI response.create")?;
            (connection, None)
        }
    };

    let (event_sender, event_receiver) = mpsc::unbounded();
    spawn(Box::pin(receive_websocket_response(
        connection,
        first_event_text,
        prepared.input_hash,
        websocket_chains,
        event_sender,
    )));
    Ok(event_receiver.boxed())
}

/// The turn payload: the request configuration plus the input items the
/// server does not already have, and the response to continue from.
fn turn_payload(
    prepared: &PreparedWebSocketRequest,
    previous_response_id: Option<&str>,
    reused_input_count: usize,
) -> Map<String, Value> {
    let mut payload = prepared.payload.clone();
    let input = prepared
        .input
        .get(reused_input_count..)
        .map_or_else(Vec::new, |input| input.to_vec());
    payload.insert("input".into(), Value::Array(input));
    if let Some(previous_response_id) = previous_response_id {
        payload.insert(
            "previous_response_id".into(),
            Value::String(previous_response_id.to_string()),
        );
    }
    payload
}

/// Sends a turn message over a cached connection and waits for the first
/// response event before committing to it. A connection the server closed
/// while the chain sat idle often accepts the send and only fails on the
/// next read, so waiting here lets the caller detect a stale connection and
/// fall back to a fresh one instead of failing the completion. The same
/// treatment applies when the server explicitly rejects the continuation
/// (see [`continuation_rejected_error_code`]).
async fn send_and_confirm_reused_connection(
    mut connection: Box<dyn WebSocketConnection>,
    message: String,
    deadline: BoxFuture<'static, ()>,
) -> Result<(Box<dyn WebSocketConnection>, String)> {
    let confirm = async move {
        connection
            .send(WebSocketMessage::Text(message))
            .await
            .context("failed to send OpenAI response.create")?;
        loop {
            match connection.receive().await {
                Some(Ok(WebSocketMessage::Text(text))) => {
                    if let Some(code) = continuation_rejected_error_code(&text) {
                        return Err(anyhow!("server rejected the continuation: {code}"));
                    }
                    return Ok((connection, text));
                }
                Some(Ok(WebSocketMessage::Ping(_))) | Some(Ok(WebSocketMessage::Pong(_))) => {}
                Some(Ok(WebSocketMessage::Close(_))) | None => {
                    return Err(anyhow!("connection closed before the first response event"));
                }
                Some(Ok(WebSocketMessage::Binary(_))) => {
                    return Err(anyhow!("unexpected binary OpenAI WebSocket event"));
                }
                Some(Err(error)) => return Err(error),
            }
        }
    };
    match future::select(Box::pin(confirm), deadline).await {
        future::Either::Left((result, _deadline)) => result,
        future::Either::Right(((), _confirm)) => Err(anyhow!(
            "timed out waiting for the first event on a reused WebSocket connection"
        )),
    }
}

/// Recognizes error events that invalidate a continuation without dooming a
/// retry: the referenced previous response is gone from the connection-local
/// cache (with `store=false` there is no persisted fallback, and the server
/// also evicts it when a turn fails), or the connection exceeded its
/// lifetime limit. Both are resolved by resending the full input on a fresh
/// connection, whereas other errors would just repeat there.
fn continuation_rejected_error_code(event_text: &str) -> Option<String> {
    let code = match serde_json::from_str::<StreamEvent>(event_text).ok()? {
        StreamEvent::Error { error } => error.code,
        StreamEvent::GenericError { error } => error.into_response_error().code,
        _ => None,
    }?;
    matches!(
        code.as_str(),
        "previous_response_not_found" | "websocket_connection_limit_reached"
    )
    .then_some(code)
}

async fn receive_websocket_response(
    mut connection: Box<dyn WebSocketConnection>,
    first_event_text: Option<String>,
    input_hash: [u8; 32],
    websocket_chains: SharedWebSocketChains,
    event_sender: mpsc::UnboundedSender<Result<StreamEvent>>,
) {
    let mut response_id = None;
    let mut response_output_fingerprints = Vec::new();
    let mut completed = false;
    let mut next_message = first_event_text.map(|text| Ok(WebSocketMessage::Text(text)));
    loop {
        // The loop exits as soon as a terminal event is observed, so any
        // close or end-of-stream seen here happened before the turn finished
        // and means the server aborted it (e.g. a policy rejection from a
        // tunneling transport). Surface that as an error instead of silently
        // ending the stream with no output, which the consumer could not
        // tell apart from a successful turn.
        let message = match next_message.take() {
            Some(message) => message,
            None => match connection.receive().await {
                Some(message) => message,
                None => Err(anyhow!("connection closed before the response completed")),
            },
        };
        let event = match message {
            Ok(WebSocketMessage::Text(text)) => serde_json::from_str::<StreamEvent>(&text)
                .context("failed to parse OpenAI WebSocket event"),
            Ok(WebSocketMessage::Ping(_)) | Ok(WebSocketMessage::Pong(_)) => continue,
            Ok(WebSocketMessage::Close(frame)) => Err(match frame {
                Some(frame) if !frame.reason.is_empty() => anyhow!(
                    "server closed the connection ({}): {}",
                    frame.code,
                    frame.reason
                ),
                Some(frame) => anyhow!(
                    "server closed the connection ({}) before the response completed",
                    frame.code
                ),
                None => anyhow!("server closed the connection before the response completed"),
            }),
            Ok(WebSocketMessage::Binary(_)) => {
                Err(anyhow!("unexpected binary OpenAI WebSocket event"))
            }
            Err(error) => Err(error),
        };
        match &event {
            Ok(StreamEvent::Created { response }) => {
                response_id.clone_from(&response.id);
            }
            Ok(StreamEvent::Completed { response }) => {
                response_id.clone_from(&response.id);
                response_output_fingerprints = response
                    .output
                    .iter()
                    .filter_map(output_item_fingerprint)
                    .collect();
                completed = true;
            }
            Ok(StreamEvent::Incomplete { .. })
            | Ok(StreamEvent::Failed { .. })
            | Ok(StreamEvent::Error { .. })
            | Ok(StreamEvent::GenericError { .. })
            | Err(_) => {
                completed = false;
            }
            _ => {}
        }
        let terminal = matches!(
            event,
            Ok(StreamEvent::Completed { .. })
                | Ok(StreamEvent::Incomplete { .. })
                | Ok(StreamEvent::Failed { .. })
                | Ok(StreamEvent::Error { .. })
                | Ok(StreamEvent::GenericError { .. })
                | Err(_)
        );
        if event_sender.unbounded_send(event).is_err() || terminal {
            break;
        }
    }

    if completed && let Some(previous_response_id) = response_id {
        let mut chains = websocket_chains.lock();
        chains.chains.push(WebSocketChain {
            input_hash,
            previous_response_id,
            response_output_fingerprints,
            connection,
            last_used_at: Instant::now(),
        });
        chains
            .chains
            .sort_by_key(|chain| std::cmp::Reverse(chain.last_used_at));
        chains.chains.truncate(MAX_WEBSOCKET_CHAINS);
        log::debug!(
            "OpenAI Responses transport: WebSocket response completed; chain cached; cached_chains={}",
            chains.chains.len()
        );
    }
}

fn prepare_websocket_request(
    request: &Request,
    connection_scope: &[u8],
) -> Result<PreparedWebSocketRequest> {
    let Value::Object(mut payload) = serde_json::to_value(request)? else {
        return Err(anyhow!(
            "OpenAI response request did not serialize to an object"
        ));
    };
    payload.remove("stream");
    let input = payload
        .remove("input")
        .and_then(|input| input.as_array().cloned())
        .unwrap_or_default();
    let configuration_hash = hash_json(&Value::Object(payload.clone()))?;
    let mut scope_hasher = Sha256::new();
    scope_hasher.update(connection_scope);
    let mut prefix_hash = hash_pair(scope_hasher.finalize().into(), configuration_hash);
    let mut input_prefix_hashes = Vec::with_capacity(input.len() + 1);
    input_prefix_hashes.push(prefix_hash);
    for item in &input {
        prefix_hash = hash_pair(prefix_hash, hash_json(item)?);
        input_prefix_hashes.push(prefix_hash);
    }
    Ok(PreparedWebSocketRequest {
        input,
        input_prefix_hashes,
        input_hash: prefix_hash,
        payload,
    })
}

/// Verifies that the new request continues a chain's complete server-side
/// state: the chain's request input (already matched by hash) followed by a
/// replay of every output item of its previous response, followed by at
/// least one new item.
///
/// A continuation with `previous_response_id` must send only items the
/// server's connection-local state does not already contain, and that state
/// includes the previous response's own output. The next request replays
/// that output (function calls, reasoning, assistant text) as input items
/// right after the previous request's input; replayed items are matched
/// against the completed response's output fingerprints, in any order
/// because the replay reorders items. Returns the number of input items the
/// server already has. Returns `None` when the request does not extend the
/// complete state: an unconsumed fingerprint means the request truncates or
/// branches off the previous response, and an empty remaining suffix means
/// it regenerates the response; both must be resent in full on a fresh
/// connection.
fn validate_continuation(
    input: &[Value],
    matched_input_count: usize,
    response_output_fingerprints: &[ResponseItemFingerprint],
) -> Option<usize> {
    let mut unmatched_fingerprints = response_output_fingerprints.to_vec();
    let mut covered_input_count = matched_input_count;
    for item in input.get(matched_input_count..)? {
        if unmatched_fingerprints.is_empty() {
            break;
        }
        let position = input_item_fingerprint(item).and_then(|fingerprint| {
            unmatched_fingerprints
                .iter()
                .position(|candidate| *candidate == fingerprint)
        })?;
        unmatched_fingerprints.swap_remove(position);
        covered_input_count += 1;
    }
    (unmatched_fingerprints.is_empty() && covered_input_count < input.len())
        .then_some(covered_input_count)
}

fn output_item_fingerprint(item: &ResponseOutputItem) -> Option<ResponseItemFingerprint> {
    match item {
        ResponseOutputItem::FunctionCall(function_call) => {
            Some(ResponseItemFingerprint::FunctionCall {
                call_id: function_call
                    .call_id
                    .clone()
                    .or_else(|| function_call.id.clone())?,
                name: function_call.name.clone().unwrap_or_default(),
                arguments: function_call.arguments.clone(),
            })
        }
        ResponseOutputItem::CustomToolCall(custom_tool_call) => {
            Some(ResponseItemFingerprint::CustomToolCall {
                call_id: custom_tool_call
                    .call_id
                    .clone()
                    .or_else(|| custom_tool_call.id.clone())?,
                name: custom_tool_call.name.clone().unwrap_or_default(),
                input: custom_tool_call.input.clone(),
            })
        }
        ResponseOutputItem::Reasoning(reasoning) => Some(ResponseItemFingerprint::Reasoning {
            id: reasoning.id.clone(),
            encrypted_content: reasoning.encrypted_content.clone(),
        }),
        ResponseOutputItem::Message(message) => Some(ResponseItemFingerprint::AssistantMessage {
            text: message
                .content
                .iter()
                .filter_map(output_text_of_part)
                .collect(),
        }),
        ResponseOutputItem::Compaction(compaction) => Some(ResponseItemFingerprint::Compaction {
            id: compaction.id.as_deref().map(str::to_string),
            encrypted_content: compaction.encrypted_content.to_string(),
        }),
        ResponseOutputItem::Unknown => None,
    }
}

fn input_item_fingerprint(item: &Value) -> Option<ResponseItemFingerprint> {
    match item.get("type")?.as_str()? {
        "function_call" => Some(ResponseItemFingerprint::FunctionCall {
            call_id: string_field(item, "call_id")?,
            name: string_field(item, "name")?,
            arguments: string_field(item, "arguments")?,
        }),
        "custom_tool_call" => Some(ResponseItemFingerprint::CustomToolCall {
            call_id: string_field(item, "call_id")?,
            name: string_field(item, "name")?,
            input: string_field(item, "input")?,
        }),
        "reasoning" => Some(ResponseItemFingerprint::Reasoning {
            id: string_field(item, "id"),
            encrypted_content: string_field(item, "encrypted_content"),
        }),
        "compaction" => Some(ResponseItemFingerprint::Compaction {
            id: string_field(item, "id"),
            encrypted_content: string_field(item, "encrypted_content")?,
        }),
        "message" if string_field(item, "role").as_deref() == Some("assistant") => {
            Some(ResponseItemFingerprint::AssistantMessage {
                text: item
                    .get("content")?
                    .as_array()?
                    .iter()
                    .filter_map(output_text_of_part)
                    .collect(),
            })
        }
        _ => None,
    }
}

fn output_text_of_part(part: &Value) -> Option<&str> {
    (part.get("type")?.as_str()? == "output_text")
        .then(|| part.get("text")?.as_str())
        .flatten()
}

fn string_field(item: &Value, field: &str) -> Option<String> {
    Some(item.get(field)?.as_str()?.to_string())
}

/// Finds the chain whose complete server-side state covers the longest
/// prefix of the request's input, returning the chain's index and the
/// number of input items that state covers (the chain's request input plus
/// its replayed response output).
///
/// Prefix lookup and complete-state validation are one operation: a chain
/// matching the longest request-input prefix can still be unusable (see
/// [`validate_continuation`]) while a chain matching a shorter prefix is a
/// valid continuation, so every chain is validated before candidates are
/// compared.
fn best_continuation_chain(
    chains: &[WebSocketChain],
    request: &PreparedWebSocketRequest,
) -> Option<(usize, usize)> {
    chains
        .iter()
        .enumerate()
        .filter_map(|(index, chain)| {
            let matched_input_count = request
                .input_prefix_hashes
                .iter()
                .position(|prefix_hash| chain.input_hash == *prefix_hash)?;
            let covered_input_count = validate_continuation(
                &request.input,
                matched_input_count,
                &chain.response_output_fingerprints,
            )?;
            Some((index, covered_input_count))
        })
        .max_by_key(|&(_, covered_input_count)| covered_input_count)
}

fn hash_json(value: &Value) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_vec(value)?);
    Ok(hasher.finalize().into())
}

fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::{
        ResponseFunctionToolCall, ResponseInputContent, ResponseInputItem, ResponseMessageItem,
        ResponseOutputMessage, ResponseReasoningItem,
    };
    use futures::FutureExt as _;
    use futures::executor::block_on;
    use futures::future;
    use serde_json::json;
    use websocket_client::test_support::ScriptedWebSocketConnection;

    #[test]
    fn websocket_chain_lookup_prefers_the_longest_matching_prefix() {
        let request = prepared_request_with_input("gpt-test", &["one", "two", "three"]);
        let shorter = cached_chain(&prepared_request_with_input("gpt-test", &["one"]));
        let longer = cached_chain(&prepared_request_with_input("gpt-test", &["one", "two"]));

        assert_eq!(
            best_continuation_chain(&[shorter, longer], &request),
            Some((1, 2))
        );
    }

    #[test]
    fn websocket_chain_lookup_rejects_different_request_configuration() {
        let request = prepared_request_with_input("gpt-test", &["one", "two"]);
        let previous = cached_chain(&prepared_request_with_input("different-model", &["one"]));

        assert_eq!(best_continuation_chain(&[previous], &request), None);
    }

    #[test]
    fn websocket_chain_lookup_rejects_different_connection_scope() {
        let request = test_request_with_input("gpt-test", &["one"]);
        let previous =
            cached_chain(&prepare_websocket_request(&request, b"first-credential").unwrap());
        let request = prepare_websocket_request(&request, b"second-credential").unwrap();

        assert_eq!(best_continuation_chain(&[previous], &request), None);
    }

    #[test]
    fn websocket_chain_lookup_falls_back_to_a_shorter_valid_chain() {
        // The longer chain matches the request's full input, but the request
        // does not replay its response output: it is a regeneration of that
        // response and must not continue the chain. The shorter chain's
        // complete state is a strict prefix of the request, so it remains a
        // valid continuation.
        let request = prepared_request_with_input("gpt-test", &["one", "two", "three"]);
        let invalid_longer = WebSocketChain {
            response_output_fingerprints: vec![ResponseItemFingerprint::AssistantMessage {
                text: "not replayed in the request".to_string(),
            }],
            ..cached_chain(&prepared_request_with_input(
                "gpt-test",
                &["one", "two", "three"],
            ))
        };
        let valid_shorter = cached_chain(&prepared_request_with_input("gpt-test", &["one"]));

        assert_eq!(
            best_continuation_chain(&[invalid_longer, valid_shorter], &request),
            Some((1, 1))
        );
    }

    #[test]
    fn complete_continuation_reuses_the_chain_and_sends_only_the_new_suffix() {
        let user_message = json!({
            "type": "message",
            "role": "user",
            "content": [{"type": "input_text", "text": "question"}],
        });
        let assistant_reply = json!({
            "type": "message",
            "role": "assistant",
            "content": [{"type": "output_text", "text": "reply", "annotations": []}],
        });
        let tool_output = json!({
            "type": "function_call_output",
            "call_id": "call_1",
            "output": "data",
        });
        let chain = WebSocketChain {
            response_output_fingerprints: vec![ResponseItemFingerprint::AssistantMessage {
                text: "reply".to_string(),
            }],
            ..cached_chain(&prepared_request_with_raw_input(
                "gpt-test",
                vec![user_message.clone()],
            ))
        };
        let request = prepared_request_with_raw_input(
            "gpt-test",
            vec![user_message, assistant_reply, tool_output],
        );

        assert_eq!(best_continuation_chain(&[chain], &request), Some((0, 2)));

        let payload = turn_payload(&request, Some("resp_test"), 2);
        assert_eq!(payload["previous_response_id"], "resp_test");
        let input = payload["input"].as_array().unwrap();
        assert_eq!(input.len(), 1);
        assert_eq!(input[0]["type"], "function_call_output");
    }

    #[test]
    fn continuation_covers_the_previous_response_output_items() {
        // Fingerprints of the previous response's output, as recorded when
        // its `response.completed` event arrived. The message comes before
        // the reasoning item here, unlike in the replayed input below, to
        // exercise the order-independent matching.
        let fingerprints = [
            ResponseOutputItem::Message(ResponseOutputMessage {
                id: Some("msg_1".to_string()),
                content: vec![json!({
                    "type": "output_text",
                    "text": "Let me look.",
                    "annotations": [],
                })],
                role: Some("assistant".to_string()),
                status: None,
                phase: None,
            }),
            ResponseOutputItem::Reasoning(ResponseReasoningItem {
                id: Some("rs_1".to_string()),
                summary: Vec::new(),
                content: Vec::new(),
                encrypted_content: Some("encrypted".to_string()),
                status: None,
            }),
            ResponseOutputItem::FunctionCall(ResponseFunctionToolCall {
                id: Some("fc_1".to_string()),
                arguments: "{\"path\":\"main.rs\"}".to_string(),
                call_id: Some("call_1".to_string()),
                name: Some("read_file".to_string()),
                status: None,
            }),
        ]
        .iter()
        .filter_map(output_item_fingerprint)
        .collect::<Vec<_>>();
        assert_eq!(fingerprints.len(), 3);

        // The next request's input: the original user message, then the
        // previous response's output replayed as input items (restructured
        // the way a conversation replays them: reasoning first, message
        // without its server-assigned id, then the function call), and
        // finally the genuinely new tool output.
        let input = vec![
            json!({
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "Find fizz_buzz"}],
            }),
            json!({
                "type": "reasoning",
                "id": "rs_1",
                "summary": [],
                "encrypted_content": "encrypted",
            }),
            json!({
                "type": "message",
                "role": "assistant",
                "content": [{"type": "output_text", "text": "Let me look.", "annotations": []}],
            }),
            json!({
                "type": "function_call",
                "call_id": "call_1",
                "name": "read_file",
                "arguments": "{\"path\":\"main.rs\"}",
            }),
            json!({
                "type": "function_call_output",
                "call_id": "call_1",
                "output": "fn fizz_buzz() {}",
            }),
        ];

        let covered = validate_continuation(&input, 1, &fingerprints);
        assert_eq!(covered, Some(input.len() - 1));
        assert_eq!(input[covered.unwrap()]["type"], "function_call_output");
    }

    #[test]
    fn exact_resend_is_not_a_continuation() {
        let input = vec![json!({
            "type": "message",
            "role": "user",
            "content": [{"type": "input_text", "text": "hi"}],
        })];
        let fingerprints = vec![ResponseItemFingerprint::AssistantMessage {
            text: "previous reply".to_string(),
        }];

        // The request equals the input that produced the chain's response:
        // this regenerates the response instead of continuing it.
        assert_eq!(validate_continuation(&input, 1, &fingerprints), None);
        // Even when the response produced no fingerprintable output, a
        // request with no new items is still a regeneration.
        assert_eq!(validate_continuation(&input, 1, &[]), None);
    }

    #[test]
    fn partial_response_replay_is_not_a_continuation() {
        let fingerprints = [
            ResponseOutputItem::Reasoning(ResponseReasoningItem {
                id: Some("rs_1".to_string()),
                summary: Vec::new(),
                content: Vec::new(),
                encrypted_content: Some("encrypted".to_string()),
                status: None,
            }),
            ResponseOutputItem::Message(ResponseOutputMessage {
                id: Some("msg_1".to_string()),
                content: vec![json!({
                    "type": "output_text",
                    "text": "Let me look.",
                    "annotations": [],
                })],
                role: Some("assistant".to_string()),
                status: None,
                phase: None,
            }),
        ]
        .iter()
        .filter_map(output_item_fingerprint)
        .collect::<Vec<_>>();
        assert_eq!(fingerprints.len(), 2);

        // The request replays only the reasoning item and truncates the
        // assistant message; continuing from the response would resurrect
        // the truncated text on the server.
        let input = vec![
            json!({
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "hi"}],
            }),
            json!({
                "type": "reasoning",
                "id": "rs_1",
                "summary": [],
                "encrypted_content": "encrypted",
            }),
            json!({
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "try again"}],
            }),
        ];

        assert_eq!(validate_continuation(&input, 1, &fingerprints), None);
    }

    #[test]
    fn branched_response_replay_is_not_a_continuation() {
        let fingerprints = [ResponseOutputItem::FunctionCall(ResponseFunctionToolCall {
            id: None,
            arguments: "{}".to_string(),
            call_id: Some("call_1".to_string()),
            name: Some("read_file".to_string()),
            status: None,
        })]
        .iter()
        .filter_map(output_item_fingerprint)
        .collect::<Vec<_>>();

        // The item after the matched prefix is not part of the previous
        // response's output: the request branches off the chain's state.
        let input = vec![
            json!({
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "hi"}],
            }),
            json!({
                "type": "function_call",
                "call_id": "call_other",
                "name": "read_file",
                "arguments": "{}",
            }),
        ];

        assert_eq!(validate_continuation(&input, 1, &fingerprints), None);
    }

    #[test]
    fn turn_payload_slices_input_and_adds_previous_response_id() {
        let prepared = prepared_request_with_input("gpt-test", &["one", "two", "three"]);

        let payload = turn_payload(&prepared, Some("resp_1"), 2);
        assert_eq!(payload["previous_response_id"], "resp_1");
        assert_eq!(payload["input"].as_array().unwrap().len(), 1);
        assert!(payload.get("stream").is_none());

        let message = response_create_envelope(payload).unwrap();
        let message: Value = serde_json::from_str(&message).unwrap();
        assert_eq!(message["type"], "response.create");

        let payload = turn_payload(&prepared, None, 0);
        assert!(payload.get("previous_response_id").is_none());
        assert_eq!(payload["input"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn exact_resend_sends_the_full_input_on_a_new_connection() {
        // A chain produced from this exact input: resending it regenerates
        // the chain's response, which the chain's connection cannot do.
        let request = test_request_with_input("gpt-test", &["one", "two"]);
        let prepared = prepare_websocket_request(&request, b"test-scope").unwrap();
        let chains = WebSocketChains::new_shared();
        let cached_sent_messages = Arc::new(Mutex::new(Vec::new()));
        chains.lock().chains.push(WebSocketChain {
            response_output_fingerprints: vec![ResponseItemFingerprint::AssistantMessage {
                text: "previous reply".to_string(),
            }],
            connection: Box::new(ScriptedWebSocketConnection::with_sent_messages(
                cached_sent_messages.clone(),
                Vec::new(),
            )),
            ..cached_chain(&prepared)
        });

        let fresh_sent_messages = Arc::new(Mutex::new(Vec::new()));
        let fresh_connection = Box::new(ScriptedWebSocketConnection::with_sent_messages(
            fresh_sent_messages.clone(),
            vec![Ok(WebSocketMessage::Text(
                serde_json::to_string(&json!({
                    "type": "response.completed",
                    "response": {"id": "resp_regenerated", "output": []},
                }))
                .unwrap(),
            ))],
        ));

        let mut background = None;
        let stream = block_on(stream_websocket_response(
            &request,
            b"test-scope",
            chains.clone(),
            future::ready(Ok(fresh_connection as Box<dyn WebSocketConnection>)),
            Arc::new(|_| future::pending::<()>().boxed()),
            response_create_envelope,
            |future| background = Some(future),
        ))
        .unwrap();
        block_on(background.unwrap());

        // The cached connection must not receive anything: continuing it
        // would generate a response after the one being regenerated.
        assert!(cached_sent_messages.lock().is_empty());
        let sent = fresh_sent_messages.lock();
        assert_eq!(sent.len(), 1);
        let WebSocketMessage::Text(sent_text) = &sent[0] else {
            panic!("expected a text message, got {:?}", sent[0]);
        };
        let payload: Value = serde_json::from_str(sent_text).unwrap();
        assert!(payload.get("previous_response_id").is_none());
        assert_eq!(payload["input"].as_array().unwrap().len(), 2);

        let events = block_on(stream.collect::<Vec<_>>());
        assert!(matches!(
            events.last(),
            Some(Ok(StreamEvent::Completed { response })) if response.id.as_deref() == Some("resp_regenerated")
        ));
        // The unused chain stays cached alongside the regeneration's new
        // chain; it can still serve a later continuation of its own state.
        let chains = chains.lock();
        let mut cached_response_ids = chains
            .chains
            .iter()
            .map(|chain| chain.previous_response_id.as_str())
            .collect::<Vec<_>>();
        cached_response_ids.sort_unstable();
        assert_eq!(cached_response_ids, ["resp_regenerated", "resp_test"]);
    }

    #[test]
    fn reused_connection_is_confirmed_by_the_first_response_event() {
        let connection = Box::new(ScriptedWebSocketConnection::new(vec![
            Ok(WebSocketMessage::Ping(Vec::new())),
            Ok(WebSocketMessage::Text("{\"type\":\"stub\"}".to_string())),
        ]));

        let (_connection, first_event_text) = block_on(send_and_confirm_reused_connection(
            connection,
            "request".to_string(),
            future::pending().boxed(),
        ))
        .unwrap();

        assert_eq!(first_event_text, "{\"type\":\"stub\"}");
    }

    #[test]
    fn reused_connection_confirmation_times_out() {
        let result = block_on(send_and_confirm_reused_connection(
            Box::new(NeverRespondingWebSocketConnection),
            "request".to_string(),
            future::ready(()).boxed(),
        ));

        assert!(
            result
                .as_ref()
                .is_err_and(|error| error.to_string().contains("timed out"))
        );
    }

    #[test]
    fn reused_connection_closed_by_the_server_is_reported_as_stale() {
        let closed_connection = Box::new(ScriptedWebSocketConnection::new(vec![Ok(
            WebSocketMessage::Close(None),
        )]));
        assert!(
            block_on(send_and_confirm_reused_connection(
                closed_connection,
                "request".to_string(),
                future::pending().boxed(),
            ))
            .is_err()
        );

        let dropped_connection = Box::new(ScriptedWebSocketConnection::new(Vec::new()));
        assert!(
            block_on(send_and_confirm_reused_connection(
                dropped_connection,
                "request".to_string(),
                future::pending().boxed(),
            ))
            .is_err()
        );
    }

    #[test]
    fn continuation_rejection_errors_are_reported_as_stale() {
        let previous_response_not_found = r#"{"type":"error","status":400,"error":{"code":"previous_response_not_found","message":"Previous response with id 'resp_abc' not found.","param":"previous_response_id"}}"#;
        let connection_limit_reached = r#"{"type":"error","status":400,"error":{"type":"invalid_request_error","code":"websocket_connection_limit_reached","message":"Responses websocket connection limit reached (60 minutes). Create a new websocket connection to continue."}}"#;
        for rejection in [previous_response_not_found, connection_limit_reached] {
            let connection = Box::new(ScriptedWebSocketConnection::new(vec![Ok(
                WebSocketMessage::Text(rejection.to_string()),
            )]));
            assert!(
                block_on(send_and_confirm_reused_connection(
                    connection,
                    "request".to_string(),
                    future::pending().boxed(),
                ))
                .is_err()
            );
        }

        // Other errors are not stale-connection signals; they pass through
        // to the caller like any other first event.
        let other_error = r#"{"type":"error","status":429,"error":{"code":"rate_limit_exceeded","message":"Rate limit exceeded"}}"#;
        let connection = Box::new(ScriptedWebSocketConnection::new(vec![Ok(
            WebSocketMessage::Text(other_error.to_string()),
        )]));
        let (_connection, first_event_text) = block_on(send_and_confirm_reused_connection(
            connection,
            "request".to_string(),
            future::pending().boxed(),
        ))
        .unwrap();
        assert_eq!(first_event_text, other_error);
    }

    #[test]
    fn completed_response_caches_the_connection_as_a_chain() {
        let prepared = prepared_request_with_input("gpt-test", &["one"]);
        let chains = WebSocketChains::new_shared();
        let connection = Box::new(ScriptedWebSocketConnection::new(vec![Ok(
            WebSocketMessage::Text(
                serde_json::to_string(&json!({
                    "type": "response.completed",
                    "response": {"id": "resp_1", "output": []},
                }))
                .unwrap(),
            ),
        )]));
        let (event_sender, event_receiver) = mpsc::unbounded();

        block_on(receive_websocket_response(
            connection,
            None,
            prepared.input_hash,
            chains.clone(),
            event_sender,
        ));

        let events = block_on(event_receiver.collect::<Vec<_>>());
        assert_eq!(events.len(), 1);
        assert!(matches!(
            events[0],
            Ok(StreamEvent::Completed { ref response }) if response.id.as_deref() == Some("resp_1")
        ));
        let chains = chains.lock();
        assert_eq!(chains.chains.len(), 1);
        assert_eq!(chains.chains[0].previous_response_id, "resp_1");
        assert_eq!(chains.chains[0].input_hash, prepared.input_hash);
    }

    #[test]
    fn abnormal_close_before_the_terminal_event_surfaces_an_error() {
        let prepared = prepared_request_with_input("gpt-test", &["one"]);
        let chains = WebSocketChains::new_shared();
        let connection = Box::new(ScriptedWebSocketConnection::new(vec![Ok(
            WebSocketMessage::Close(Some(websocket_client::WebSocketCloseFrame {
                code: websocket_client::WebSocketCloseCode::Policy,
                reason: "usage limit reached".to_string(),
            })),
        )]));
        let (event_sender, event_receiver) = mpsc::unbounded();

        block_on(receive_websocket_response(
            connection,
            None,
            prepared.input_hash,
            chains.clone(),
            event_sender,
        ));

        let events = block_on(event_receiver.collect::<Vec<_>>());
        assert_eq!(events.len(), 1);
        let error = events[0].as_ref().unwrap_err();
        assert!(error.to_string().contains("usage limit reached"));
        assert!(chains.lock().chains.is_empty());
    }

    #[test]
    fn silent_close_before_the_terminal_event_surfaces_an_error() {
        // End-of-stream without a close frame, a close frame with no
        // payload, and a close frame with a code but an empty reason must
        // all be reported as errors; none of them is a successful end of
        // the turn.
        let cases: Vec<(Vec<Result<WebSocketMessage>>, &str)> = vec![
            (Vec::new(), "connection closed"),
            (vec![Ok(WebSocketMessage::Close(None))], "connection"),
            (
                vec![Ok(WebSocketMessage::Close(Some(
                    websocket_client::WebSocketCloseFrame {
                        code: websocket_client::WebSocketCloseCode::Error,
                        reason: String::new(),
                    },
                )))],
                "1011",
            ),
        ];
        for (incoming, expected_error_fragment) in cases {
            let prepared = prepared_request_with_input("gpt-test", &["one"]);
            let chains = WebSocketChains::new_shared();
            let connection = Box::new(ScriptedWebSocketConnection::new(incoming));
            let (event_sender, event_receiver) = mpsc::unbounded();

            block_on(receive_websocket_response(
                connection,
                None,
                prepared.input_hash,
                chains.clone(),
                event_sender,
            ));

            let events = block_on(event_receiver.collect::<Vec<_>>());
            assert_eq!(events.len(), 1);
            let error = events[0].as_ref().unwrap_err();
            assert!(
                error.to_string().contains(expected_error_fragment),
                "expected error containing {expected_error_fragment:?}, got: {error}"
            );
            assert!(chains.lock().chains.is_empty());
        }
    }

    struct NeverRespondingWebSocketConnection;

    impl WebSocketConnection for NeverRespondingWebSocketConnection {
        fn send(&mut self, _message: WebSocketMessage) -> BoxFuture<'_, Result<()>> {
            future::ready(Ok(())).boxed()
        }

        fn receive(&mut self) -> BoxFuture<'_, Option<Result<WebSocketMessage>>> {
            future::pending().boxed()
        }
    }

    fn cached_chain(prepared: &PreparedWebSocketRequest) -> WebSocketChain {
        WebSocketChain {
            input_hash: prepared.input_hash,
            previous_response_id: "resp_test".to_string(),
            response_output_fingerprints: Vec::new(),
            connection: Box::new(ScriptedWebSocketConnection::new(Vec::new())),
            last_used_at: Instant::now(),
        }
    }

    fn prepared_request_with_input(model: &str, input: &[&str]) -> PreparedWebSocketRequest {
        prepare_websocket_request(&test_request_with_input(model, input), b"test-scope").unwrap()
    }

    /// Like [`prepared_request_with_input`], but with raw input items, for
    /// requests that replay a previous response's output.
    fn prepared_request_with_raw_input(model: &str, input: Vec<Value>) -> PreparedWebSocketRequest {
        let mut request = test_request_with_input(model, &[]);
        request.input = crate::responses::ResponseInput::new(input, Vec::new());
        prepare_websocket_request(&request, b"test-scope").unwrap()
    }

    fn test_request_with_input(model: &str, input: &[&str]) -> Request {
        Request {
            model: model.to_string(),
            instructions: None,
            input: crate::responses::ResponseInput::new(
                Vec::new(),
                input
                    .iter()
                    .map(|text| {
                        ResponseInputItem::Message(ResponseMessageItem {
                            role: crate::Role::User,
                            content: vec![ResponseInputContent::Text {
                                text: text.to_string(),
                            }],
                            phase: None,
                        })
                    })
                    .collect(),
            ),
            include: Vec::new(),
            stream: true,
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: Vec::new(),
            prompt_cache_key: None,
            reasoning: None,
            store: Some(false),
            service_tier: None,
            context_management: None,
        }
    }
}
