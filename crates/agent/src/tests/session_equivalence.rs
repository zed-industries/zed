//! Checks that a `LanguageModelSession` builds the requests a `Thread` builds.
//!
//! One generated conversation drives both systems. Each has its own fake
//! provider, which streams them identical completion events. Thread runs its
//! tools itself; the session's host submits the same results in the order
//! Thread reported them. Every request either system sends must match,
//! apart from the differences `comparable` documents.
//!
//! Generation leaves out what the session deliberately doesn't do or does
//! differently:
//! - Thread-only behavior: compaction and the usage updates that trigger it,
//!   title and summary requests, retries after stream errors, refusal
//!   fallback, `Resume` messages, and changing the model, profile, tools, or
//!   settings mid-conversation. The session freezes its system prompt and
//!   tools, so the test copies them from Thread's first request.
//! - Results only Thread's tool runner knows: streaming tool inputs, JSON
//!   parse errors, and unknown tools.
//! - Thinking after a signed thinking block. The session starts a new block
//!   where Thread merges them.
//! - A later partial tool use without the `thought_signature` an earlier one
//!   carried. The session keeps the earlier signature where Thread drops it.

use super::*;
use gpui::proptest::prelude::*;
use language_model::session::{
    LanguageModelSession, RoundParameters, SessionConfig, SessionInput, UserContent,
};
use language_model::{
    LanguageModelToolResultContent, LanguageModelToolUseId, LanguageModelToolUseInput,
};
use pretty_assertions::assert_eq;

#[gpui::property_test(config = ProptestConfig {
    cases: 64,
    ..Default::default()
})]
async fn session_requests_match_thread_requests(
    #[strategy = conversation()] turns: Vec<Turn>,
    cx: &mut TestAppContext,
) {
    let ThreadTest {
        model: thread_model,
        fake: thread_fake,
        thread,
        ..
    } = setup(cx, TestModel::Fake).await;
    // Thread may have been created before the settings file loaded, so it
    // can't rely on picking up the default profile.
    cx.run_until_parked();
    // Otherwise Thread's tool runner replaces successful image results before
    // request construction, whereas the session's host supplies the results.
    thread_fake.update_model("fake", |model| model.supports_images = true);
    thread.update(cx, |thread, cx| {
        thread.refresh_model(&thread_model.provider_id, cx);
        thread.tools.insert(
            ScriptedResultTool::NAME.into(),
            Arc::new(ScriptedResultTool),
        );
        thread.set_profile(AgentProfileId("test-profile".into()), cx);
    });
    let session_fake = Arc::new(FakeLanguageModelProvider::default());
    let session_model = session_fake.update_model("fake", |model| model.supports_images = true);
    let mut session = None;
    let mut next_tool_use_id = 0;

    for (turn_index, turn) in turns.iter().enumerate() {
        let mut thread_events = thread
            .update(cx, |thread, cx| {
                thread.send(ClientUserMessageId::new(), turn.thread_content(), cx)
            })
            .unwrap();
        cx.run_until_parked();

        for (round_index, round) in turn.rounds.iter().enumerate() {
            let thread_request = only_open_request(&thread_fake, &thread_model, "Thread");
            let session = session
                .get_or_insert_with(|| {
                    let config = frozen_config(&thread_request);
                    let client = session_fake.clone();
                    let model = session_model.clone();
                    cx.new(|_| LanguageModelSession::new(client, model, config))
                })
                .clone();
            if round_index == 0 {
                session.update(cx, |session, _| session.append(turn.session_input()));
            }
            let session_stream = session
                .update(cx, |session, cx| {
                    session.complete(round_parameters(&thread_request), cx)
                })
                .unwrap();
            cx.run_until_parked();
            let session_request = only_open_request(&session_fake, &session_model, "the session");
            assert_eq!(
                comparable(session_request),
                comparable(thread_request),
                "turn {turn_index}, round {round_index}"
            );

            let (events, tool_uses) = round.events(&mut next_tool_use_id);
            let sent_events = match round.ending {
                RoundEnding::Cancelled { sent_events } => &events[..sent_events.min(events.len())],
                _ => &events[..],
            };
            for event in sent_events {
                thread_fake.send_last_event(&thread_model, event.clone());
                session_fake.send_last_event(&session_model, event.clone());
            }
            cx.run_until_parked();
            if let RoundEnding::Cancelled { .. } = round.ending {
                thread.update(cx, |thread, cx| thread.cancel(cx)).await;
            } else {
                thread_fake.end_last(&thread_model);
                session_fake.end_last(&session_model);
            }
            drop(session_stream);
            cx.run_until_parked();

            let started_tool_uses: Vec<&ScriptedToolUse> = tool_uses
                .iter()
                .filter(|tool_use| {
                    sent_events.iter().any(|event| {
                        matches!(event, LanguageModelCompletionEvent::ToolUse(sent) if sent.id == tool_use.id)
                    })
                })
                .collect();
            let finished_ids = finished_tool_use_ids(&mut thread_events, &started_tool_uses);
            let expected_finished_ids: HashSet<&LanguageModelToolUseId> = match round.ending {
                RoundEnding::MaxTokens => HashSet::default(),
                RoundEnding::Completed { .. } | RoundEnding::Cancelled { .. } => started_tool_uses
                    .iter()
                    .filter(|tool_use| {
                        sent_events.iter().any(|event| {
                            matches!(
                                event,
                                LanguageModelCompletionEvent::ToolUse(sent)
                                    if sent.id == tool_use.id && sent.is_input_complete
                            )
                        })
                    })
                    .map(|tool_use| &tool_use.id)
                    .collect(),
            };
            assert_eq!(
                finished_ids.iter().collect::<HashSet<_>>(),
                expected_finished_ids,
                "tools Thread ran in turn {turn_index}, round {round_index}"
            );

            session.update(cx, |session, _| {
                for id in &finished_ids {
                    let tool_use = started_tool_uses
                        .iter()
                        .find(|tool_use| &tool_use.id == id)
                        .expect("Thread finished a tool use it never started");
                    session.submit_tool_result(tool_use.result()).unwrap();
                }
                for tool_use in &started_tool_uses {
                    if !finished_ids.contains(&tool_use.id) {
                        session
                            .submit_tool_result(tool_use.canceled_result())
                            .unwrap();
                    }
                }
            });

            let continues = round.continues();
            assert_eq!(
                thread.read_with(cx, |thread, _| thread.is_turn_complete()),
                !continues,
                "whether Thread's turn {turn_index} ended after round {round_index}"
            );
        }
    }

    // The last round's output only appears in a later request.
    thread
        .update(cx, |thread, cx| {
            thread.send(ClientUserMessageId::new(), ["Anything else?"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    let thread_request = only_open_request(&thread_fake, &thread_model, "Thread");
    let Some(session) = session else {
        return;
    };
    let _session_stream = session.update(cx, |session, cx| {
        session.append(SessionInput::UserMessage(vec![UserContent::Text(
            "Anything else?".into(),
        )]));
        session
            .complete(round_parameters(&thread_request), cx)
            .unwrap()
    });
    cx.run_until_parked();
    let session_request = only_open_request(&session_fake, &session_model, "the session");
    assert_eq!(
        comparable(session_request),
        comparable(thread_request),
        "final request"
    );
}

/// The one request still streaming to `model`, failing if the system sent
/// none or more than one. Cancelled requests stay pending with their
/// receivers dropped, so only open ones count.
fn only_open_request(
    fake: &FakeLanguageModelProvider,
    model: &LanguageModel,
    description: &str,
) -> LanguageModelRequest {
    let mut open: Vec<LanguageModelRequest> = fake
        .pending_completions_for(model)
        .into_iter()
        .filter(|request| !fake.is_stream_closed(model, request))
        .collect();
    assert_eq!(open.len(), 1, "{description}: exactly one open request");
    open.remove(0)
}
/// A user message and the model rounds it starts.
///
/// Every round but the last runs a tool, since Thread continues the turn
/// exactly when that happens.
#[derive(Clone, Debug)]
struct Turn {
    user_content: Vec<UserPart>,
    rounds: Vec<Round>,
}

#[derive(Clone, Debug)]
enum UserPart {
    Text(String),
    Image(String),
}

#[derive(Clone, Debug)]
struct Round {
    segments: Vec<Segment>,
    ending: RoundEnding,
}

#[derive(Clone, Debug)]
enum RoundEnding {
    /// The stream ends, after an optional non-error stop reason.
    Completed { stop: Option<StopReason> },
    /// The stream ends after `Stop(MaxTokens)`, which ends Thread's turn
    /// without running tools.
    MaxTokens,
    /// The host cancels after this many events.
    Cancelled { sent_events: usize },
}

/// A run of completion events that build one piece of content.
#[derive(Clone, Debug)]
enum Segment {
    StartMessage,
    Text(Vec<String>),
    Thinking {
        chunks: Vec<String>,
        /// Sent with the last chunk.
        signature: Option<String>,
    },
    RedactedThinking(String),
    ToolUse {
        partial_count: usize,
        is_complete: bool,
        result: Vec<ScriptedResultPart>,
        is_error: bool,
        /// Sent with every partial and the complete tool use.
        thought_signature: Option<String>,
    },
    ReasoningDetails(serde_json::Value),
}

struct ScriptedToolUse {
    id: LanguageModelToolUseId,
    result: Vec<ScriptedResultPart>,
    is_error: bool,
}

impl Turn {
    fn thread_content(&self) -> Vec<UserMessageContent> {
        self.user_content
            .iter()
            .map(|part| match part {
                UserPart::Text(text) => UserMessageContent::Text(text.clone()),
                UserPart::Image(source) => UserMessageContent::Image(image(source)),
            })
            .collect()
    }

    fn session_input(&self) -> SessionInput {
        SessionInput::UserMessage(
            self.user_content
                .iter()
                .map(|part| match part {
                    UserPart::Text(text) => UserContent::Text(text.clone()),
                    UserPart::Image(source) => UserContent::Image(image(source)),
                })
                .collect(),
        )
    }
}

impl Round {
    fn continues(&self) -> bool {
        matches!(self.ending, RoundEnding::Completed { .. })
            && self.segments.iter().any(|segment| {
                matches!(
                    segment,
                    Segment::ToolUse {
                        is_complete: true,
                        ..
                    }
                )
            })
    }

    fn events(
        &self,
        next_tool_use_id: &mut usize,
    ) -> (Vec<LanguageModelCompletionEvent>, Vec<ScriptedToolUse>) {
        let mut events = Vec::new();
        let mut tool_uses = Vec::new();
        for segment in &self.segments {
            match segment {
                Segment::StartMessage => events.push(LanguageModelCompletionEvent::StartMessage {
                    message_id: format!("message-{}", events.len()),
                }),
                Segment::Text(chunks) => events.extend(
                    chunks
                        .iter()
                        .map(|chunk| LanguageModelCompletionEvent::Text(chunk.clone())),
                ),
                Segment::Thinking { chunks, signature } => {
                    let last_index = chunks.len().saturating_sub(1);
                    events.extend(chunks.iter().enumerate().map(|(index, chunk)| {
                        LanguageModelCompletionEvent::Thinking {
                            text: chunk.clone(),
                            signature: signature.clone().filter(|_| index == last_index),
                        }
                    }));
                }
                Segment::RedactedThinking(data) => events
                    .push(LanguageModelCompletionEvent::RedactedThinking { data: data.clone() }),
                Segment::ToolUse {
                    partial_count,
                    is_complete,
                    result,
                    is_error,
                    thought_signature,
                } => {
                    let id = LanguageModelToolUseId::from(format!("tool-{next_tool_use_id}"));
                    *next_tool_use_id += 1;
                    let tool_use = |input: serde_json::Value, is_input_complete| {
                        LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                            id: id.clone(),
                            name: ScriptedResultTool::NAME.into(),
                            raw_input: input.to_string(),
                            input: LanguageModelToolUseInput::Json(input),
                            is_input_complete,
                            thought_signature: thought_signature.clone(),
                        })
                    };
                    for partial_index in 0..*partial_count {
                        let prefix = &result[..partial_index.min(result.len())];
                        events.push(tool_use(json!({ "result": prefix }), false));
                    }
                    if *is_complete {
                        events.push(tool_use(
                            json!({ "result": result, "is_error": is_error }),
                            true,
                        ));
                    }
                    tool_uses.push(ScriptedToolUse {
                        id,
                        result: result.clone(),
                        is_error: *is_error,
                    });
                }
                Segment::ReasoningDetails(details) => events.push(
                    LanguageModelCompletionEvent::ReasoningDetails(details.clone()),
                ),
            }
        }
        match self.ending {
            RoundEnding::Completed { stop: Some(stop) } => {
                events.push(LanguageModelCompletionEvent::Stop(stop))
            }
            RoundEnding::MaxTokens => {
                events.push(LanguageModelCompletionEvent::Stop(StopReason::MaxTokens))
            }
            RoundEnding::Completed { stop: None } | RoundEnding::Cancelled { .. } => {}
        }
        (events, tool_uses)
    }
}

impl ScriptedToolUse {
    /// The result `ScriptedResultTool` returns for this tool use.
    fn result(&self) -> LanguageModelToolResult {
        LanguageModelToolResult {
            tool_use_id: self.id.clone(),
            tool_name: ScriptedResultTool::NAME.into(),
            is_error: self.is_error,
            content: self
                .result
                .iter()
                .map(|part| match part {
                    ScriptedResultPart::Text(text) => {
                        LanguageModelToolResultContent::Text(text.as_str().into())
                    }
                    ScriptedResultPart::Image(source) => {
                        LanguageModelToolResultContent::Image(image(source))
                    }
                })
                .collect(),
            output: None,
        }
    }

    /// The result Thread records for a tool use that never finished.
    fn canceled_result(&self) -> LanguageModelToolResult {
        LanguageModelToolResult {
            tool_use_id: self.id.clone(),
            tool_name: ScriptedResultTool::NAME.into(),
            is_error: true,
            content: vec![LanguageModelToolResultContent::Text(
                TOOL_CANCELED_MESSAGE.into(),
            )],
            output: None,
        }
    }
}

fn conversation() -> impl Strategy<Value = Vec<Turn>> {
    prop::collection::vec(turn(), 1..4)
}

fn turn() -> impl Strategy<Value = Turn> {
    (
        prop::collection::vec(user_part(), 0..3),
        prop::collection::vec(round(), 1..4),
    )
        .prop_map(|(user_content, rounds)| Turn {
            user_content,
            rounds: rounds_thread_would_run(rounds),
        })
}

/// Cuts `rounds` after the first one that ends Thread's turn, and ends it
/// with a text-only round if every round runs a tool.
fn rounds_thread_would_run(rounds: Vec<Round>) -> Vec<Round> {
    let mut kept = Vec::new();
    for round in rounds {
        let continues = round.continues();
        kept.push(round);
        if !continues {
            return kept;
        }
    }
    kept.push(Round {
        segments: vec![Segment::Text(vec!["Done.".into()])],
        ending: RoundEnding::Completed {
            stop: Some(StopReason::EndTurn),
        },
    });
    kept
}

fn user_part() -> impl Strategy<Value = UserPart> {
    prop_oneof![
        3 => short_text().prop_map(UserPart::Text),
        1 => "[a-z]{1,4}".prop_map(UserPart::Image),
    ]
}

fn round() -> impl Strategy<Value = Round> {
    (prop::collection::vec(segment(), 0..6), round_ending()).prop_map(|(segments, ending)| Round {
        segments: without_excluded_shapes(segments),
        ending,
    })
}

/// Drops the segments the module docs exclude: thinking that would directly
/// follow a signed thinking block.
fn without_excluded_shapes(segments: Vec<Segment>) -> Vec<Segment> {
    let mut kept = Vec::new();
    let mut last_content_is_signed_thinking = false;
    for segment in segments {
        match &segment {
            Segment::Thinking { signature, .. } => {
                if last_content_is_signed_thinking {
                    continue;
                }
                last_content_is_signed_thinking = signature.is_some();
            }
            Segment::ReasoningDetails(_) => {}
            Segment::StartMessage
            | Segment::ToolUse { .. }
            | Segment::Text(_)
            | Segment::RedactedThinking(_) => last_content_is_signed_thinking = false,
        }
        kept.push(segment);
    }
    kept
}

fn round_ending() -> impl Strategy<Value = RoundEnding> {
    prop_oneof![
        4 => prop::option::of(prop_oneof![
            Just(StopReason::EndTurn),
            Just(StopReason::ToolUse),
        ])
        .prop_map(|stop| RoundEnding::Completed { stop }),
        1 => Just(RoundEnding::MaxTokens),
        1 => (0usize..8).prop_map(|sent_events| RoundEnding::Cancelled { sent_events }),
    ]
}

fn segment() -> impl Strategy<Value = Segment> {
    prop_oneof![
        1 => Just(Segment::StartMessage),
        3 => prop::collection::vec(short_text(), 1..3).prop_map(Segment::Text),
        2 => (
            prop::collection::vec(short_text(), 1..3),
            prop::option::of(signature()),
        )
            .prop_map(|(chunks, signature)| Segment::Thinking { chunks, signature }),
        1 => "[a-z]{1,4}".prop_map(Segment::RedactedThinking),
        3 => (
            0usize..3,
            prop::bool::weighted(0.8),
            tool_result(),
            prop::bool::weighted(0.2),
            prop::option::of(signature()),
        )
            .prop_map(
                |(partial_count, is_complete, result, is_error, thought_signature)| {
                    Segment::ToolUse {
                        // An incomplete tool use needs a partial to exist.
                        partial_count: if is_complete {
                            partial_count
                        } else {
                            partial_count.max(1)
                        },
                        is_complete,
                        result,
                        is_error,
                        thought_signature,
                    }
                }
            ),
        1 => reasoning_details().prop_map(Segment::ReasoningDetails),
    ]
}

fn tool_result() -> impl Strategy<Value = Vec<ScriptedResultPart>> {
    prop_oneof![
        1 => Just(Vec::new()),
        1 => (1usize..=3).prop_map(|count| vec![ScriptedResultPart::Text(String::new()); count]),
        6 => prop::collection::vec(
            prop_oneof![
                3 => short_text().prop_map(ScriptedResultPart::Text),
                1 => "[a-z]{1,4}".prop_map(ScriptedResultPart::Image),
            ],
            1..=3,
        ),
    ]
}

fn reasoning_details() -> impl Strategy<Value = serde_json::Value> {
    prop_oneof![
        Just(json!([])),
        "[a-z]{1,4}".prop_map(|data| json!([{ "type": "encrypted", "data": data }])),
        "[a-z]{1,4}".prop_map(|summary| json!({ "summary": summary })),
    ]
}

fn short_text() -> impl Strategy<Value = String> {
    "[a-z ]{0,6}"
}

fn signature() -> impl Strategy<Value = String> {
    "signature-[a-z]{1,3}"
}

/// The session configuration that reproduces Thread's first request.
fn frozen_config(request: &LanguageModelRequest) -> SessionConfig {
    let system_prompt = match request.messages.first() {
        Some(LanguageModelRequestMessage {
            role: Role::System,
            content,
            ..
        }) => match content.as_slice() {
            [MessageContent::Text(system_prompt)] => system_prompt.clone(),
            content => panic!("expected a text system prompt, got {content:?}"),
        },
        message => panic!("expected a system message first, got {message:?}"),
    };
    assert!(
        !request.tools.is_empty(),
        "Thread should offer ScriptedResultTool"
    );
    SessionConfig {
        thread_id: request.thread_id.clone(),
        prompt_cache_key: request.prompt_cache_key.clone(),
        system_prompt: Some(system_prompt),
        tools: request.tools.clone(),
    }
}

/// The per-round parameters Thread chose for `request`.
fn round_parameters(request: &LanguageModelRequest) -> RoundParameters {
    RoundParameters {
        intent: request.intent,
        prompt_id: request.prompt_id.clone(),
        temperature: request.temperature,
        thinking_allowed: request.thinking_allowed,
        thinking_effort: request.thinking_effort.clone(),
        speed: request.speed,
    }
}

/// `request` without the parts the two systems intentionally differ on.
///
/// The session places its own cache marks, and it clears each tool result's
/// `output`, which no provider reads.
fn comparable(mut request: LanguageModelRequest) -> LanguageModelRequest {
    for message in &mut request.messages {
        message.cache = false;
        for content in &mut message.content {
            if let MessageContent::ToolResult(result) = content {
                result.output = None;
            }
        }
    }
    request
}

/// The tool uses Thread finished since the last call, in the order it
/// reported them.
fn finished_tool_use_ids(
    thread_events: &mut UnboundedReceiver<Result<ThreadEvent>>,
    started_tool_uses: &[&ScriptedToolUse],
) -> Vec<LanguageModelToolUseId> {
    let mut finished = Vec::new();
    while let Some(Some(event)) = thread_events.next().now_or_never() {
        let Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(update))) =
            event
        else {
            continue;
        };
        if !matches!(
            update.fields.status,
            Some(acp::ToolCallStatus::Completed | acp::ToolCallStatus::Failed)
        ) {
            continue;
        }
        let tool_call_id = update.tool_call_id.to_string();
        let tool_use = started_tool_uses
            .iter()
            .find(|tool_use| {
                tool_call_id
                    .split_once(':')
                    .is_some_and(|(_, id)| id == tool_use.id.to_string())
            })
            .expect("Thread finished a tool use it never started");
        finished.push(tool_use.id.clone());
    }
    finished
}
