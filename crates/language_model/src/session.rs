//! A GPUI entity that streams completions from a [`SessionLog`].
//!
//! All conversation logic lives in [`SessionLog`]; [`LanguageModelSession`]
//! only dispatches the rendered request, folds the streamed output into the
//! log before the host sees it, and cancels rounds the log's mutations
//! would invalidate.

pub use language_model_core::session::*;

use std::sync::Arc;

use futures::{
    FutureExt as _, StreamExt as _,
    channel::{mpsc, oneshot},
    select_biased,
};
use gpui::{Context, Task};

use crate::{
    LanguageModel, LanguageModelClient, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelCompletionStream, LanguageModelToolResult,
};

/// An append-only conversation with one model.
pub struct LanguageModelSession {
    client: Arc<dyn LanguageModelClient>,
    model: LanguageModel,
    log: SessionLog,
    active_round: Option<ActiveRound>,
}

struct ActiveRound {
    /// Closed once the host drops the round's stream.
    events:
        mpsc::UnboundedSender<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
    _task: Task<()>,
}

impl LanguageModelSession {
    /// Starts an empty session with `model`, served by `client`.
    pub fn new(
        client: Arc<dyn LanguageModelClient>,
        model: LanguageModel,
        config: SessionConfig,
    ) -> Self {
        Self::resume(client, model, SessionLog::new(config))
    }

    /// Continues a log previously returned by [`Self::export`].
    pub fn resume(
        client: Arc<dyn LanguageModelClient>,
        model: LanguageModel,
        log: SessionLog,
    ) -> Self {
        Self {
            client,
            model,
            log,
            active_round: None,
        }
    }

    /// The model every round is sent to.
    pub fn model(&self) -> &LanguageModel {
        &self.model
    }

    /// The conversation so far, including the active round's output.
    pub fn log(&self) -> &SessionLog {
        &self.log
    }

    /// A copy of the log, for persisting and [`Self::resume`].
    ///
    /// During a round, the copy includes the output recorded so far.
    pub fn export(&self) -> SessionLog {
        self.log.clone()
    }

    /// Appends `input`, cancelling the active round first.
    pub fn append(&mut self, input: SessionInput) -> SessionAnchor {
        self.cancel_round();
        self.log.append(input)
    }

    /// Appends a tool result without cancelling the active round, so results
    /// can be submitted while the model is still streaming.
    pub fn submit_tool_result(
        &mut self,
        result: LanguageModelToolResult,
    ) -> Result<(), SessionError> {
        self.log.submit_tool_result(result)
    }

    /// Removes the anchored input and everything after it, cancelling the
    /// active round first.
    pub fn truncate(&mut self, anchor: SessionAnchor) -> Result<(), SessionError> {
        self.cancel_round();
        self.log.truncate(anchor)
    }

    /// Streams the model's response to the log's rendered request.
    ///
    /// Each event is recorded in the log before the returned stream yields
    /// it. A failure to connect arrives as the stream's first item. Dropping
    /// the stream ends the round, keeping the output recorded so far, and
    /// cancels the connection if it hasn't been established yet.
    ///
    /// # Errors
    ///
    /// [`SessionError::RoundInProgress`] while a previous round's stream is
    /// still held, or [`SessionError::NoPendingInput`] if the log doesn't end
    /// in an input or tool result.
    pub fn complete(
        &mut self,
        parameters: RoundParameters,
        cx: &mut Context<Self>,
    ) -> Result<LanguageModelCompletionStream, SessionError> {
        if let Some(round) = &self.active_round {
            if !round.events.is_closed() {
                return Err(SessionError::RoundInProgress);
            }
            self.cancel_round();
        }

        let request = self.log.render_request(&parameters);
        self.log
            .begin_round(self.model.provider_id.clone(), self.model.id.clone())?;
        let response = self
            .client
            .stream_completion(&self.model, request, &cx.to_async());

        let (events, host_events) = mpsc::unbounded();
        let (stream_dropped_sender, stream_dropped) = oneshot::channel::<()>();
        let task = cx.spawn({
            let events = events.clone();
            async move |this, cx| {
                let mut stream_dropped = stream_dropped.fuse();
                // Scoped so that a cancelled connection is dropped right away.
                let connection = {
                    let mut connecting = response.fuse();
                    select_biased! {
                        _ = stream_dropped => None,
                        connection = connecting => Some(connection),
                    }
                };
                match connection {
                    None => {}
                    Some(Err(error)) => {
                        events.unbounded_send(Err(error)).ok();
                    }
                    Some(Ok(response)) => {
                        let mut response = response.fuse();
                        loop {
                            select_biased! {
                                _ = stream_dropped => break,
                                event = response.next() => match event {
                                    Some(Ok(event)) => {
                                        let recorded = this.update(cx, |this, _| {
                                            this.log.record_output(&event)
                                        });
                                        if recorded.is_err()
                                            || events.unbounded_send(Ok(event)).is_err()
                                        {
                                            break;
                                        }
                                    }
                                    Some(Err(error)) => {
                                        events.unbounded_send(Err(error)).ok();
                                        break;
                                    }
                                    None => break,
                                },
                            }
                        }
                    }
                }
                this.update(cx, |this, cx| {
                    this.log.end_round();
                    this.active_round = None;
                    cx.notify();
                })
                .ok();
            }
        });
        self.active_round = Some(ActiveRound {
            events,
            _task: task,
        });

        // The stream owns the sender, so dropping it resolves `stream_dropped`.
        Ok(host_events
            .map(move |event| {
                let _stream_dropped_sender = &stream_dropped_sender;
                event
            })
            .boxed())
    }

    fn cancel_round(&mut self) {
        if self.active_round.take().is_some() {
            self.log.end_round();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_provider::FakeLanguageModelProvider;
    use crate::{
        CompletionIntent, LanguageModelRequest, LanguageModelToolResultContent,
        LanguageModelToolUse, LanguageModelToolUseInput, MessageContent, StopReason,
    };
    use futures::future::BoxFuture;
    use gpui::{AppContext as _, AsyncApp, Entity, TestAppContext};
    use parking_lot::Mutex;
    use serde_json::json;

    #[gpui::test]
    fn complete_dispatches_the_rendered_request(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let parameters = parameters();

        let _stream = session.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters.clone(), cx).unwrap()
        });

        let rendered =
            session.read_with(cx, |session, _| session.log().render_request(&parameters));
        assert_eq!(fake.pending_completions(), vec![rendered]);
    }

    #[gpui::test]
    fn second_round_extends_first_request_as_prefix(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let model = fake.model("fake");

        let _first_stream = session.update(cx, |session, cx| {
            session.append(user_text("Read the file"));
            session.complete(parameters(), cx).unwrap()
        });
        let first = only_pending(&fake);
        fake.send_last_event(
            &model,
            LanguageModelCompletionEvent::Text("Reading.".into()),
        );
        fake.send_last_event(
            &model,
            LanguageModelCompletionEvent::ToolUse(tool_use("tool-1")),
        );
        fake.send_last_event(
            &model,
            LanguageModelCompletionEvent::Stop(StopReason::ToolUse),
        );
        fake.end_last(&model);
        cx.run_until_parked();

        let _second_stream = session.update(cx, |session, cx| {
            session.submit_tool_result(tool_result("tool-1")).unwrap();
            session.complete(parameters(), cx).unwrap()
        });
        let second = only_pending(&fake);

        let first_messages = without_cache_marks(&first);
        let second_messages = without_cache_marks(&second);
        assert_eq!(second_messages.len(), first_messages.len() + 2);
        assert_eq!(second_messages[..first_messages.len()], first_messages[..]);
    }

    #[gpui::test]
    async fn events_are_committed_before_the_host_observes_them(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let model = fake.model("fake");
        let mut stream = session.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters(), cx).unwrap()
        });

        let observer = cx.spawn({
            let session = session.clone();
            async move |cx| {
                let mut observed = Vec::new();
                let mut host_text = String::new();
                while let Some(Ok(event)) = stream.next().await {
                    if let LanguageModelCompletionEvent::Text(chunk) = event {
                        host_text.push_str(&chunk);
                        let log_text =
                            session.read_with(&cx, |session, _| output_text(session.log()));
                        observed.push((host_text.clone(), log_text));
                    }
                }
                observed
            }
        });
        for chunk in ["Hel", "lo"] {
            fake.send_last_text(&model, chunk);
            cx.run_until_parked();
        }
        fake.end_last(&model);

        assert_eq!(
            observer.await,
            vec![
                ("Hel".to_string(), "Hel".to_string()),
                ("Hello".to_string(), "Hello".to_string()),
            ]
        );
    }

    #[gpui::test]
    fn dropping_the_stream_ends_the_round_and_keeps_partial_output(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let model = fake.model("fake");
        let stream = session.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters(), cx).unwrap()
        });
        let request = only_pending(&fake);
        fake.send_last_text(&model, "partial");
        cx.run_until_parked();

        drop(stream);
        cx.run_until_parked();

        assert!(fake.is_stream_closed(&model, &request));
        // Refused only because the log ends in the model's output, not
        // because the round is still in progress.
        let result = session.update(cx, |session, cx| {
            session.complete(parameters(), cx).map(drop)
        });
        assert_eq!(result, Err(SessionError::NoPendingInput));
        let output = session.read_with(cx, |session, _| last_output(session.log()));
        assert_eq!(
            output.messages[0].content,
            vec![MessageContent::Text("partial".into())]
        );
        assert_eq!(output.stop_reason, None);
    }

    #[gpui::test]
    fn append_cancels_the_active_round(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let model = fake.model("fake");
        let mut stream = session.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters(), cx).unwrap()
        });
        let request = only_pending(&fake);
        fake.send_last_text(&model, "partial");
        cx.run_until_parked();

        session.update(cx, |session, _| session.append(user_text("Actually, wait")));
        cx.run_until_parked();

        assert_eq!(
            ready_events(&mut stream),
            (vec![text("partial")], StreamState::Ended)
        );
        assert!(fake.is_stream_closed(&model, &request));
        let entries = session.read_with(cx, |session, _| session.log().entries().to_vec());
        assert!(
            matches!(
                entries.as_slice(),
                [
                    SessionLogEntry::Input { .. },
                    SessionLogEntry::Output(_),
                    SessionLogEntry::Input { .. },
                ]
            ),
            "{entries:?}"
        );
    }

    #[gpui::test]
    fn truncate_cancels_the_active_round(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let model = fake.model("fake");
        let (anchor, mut stream) = session.update(cx, |session, cx| {
            let anchor = session.append(user_text("Hello"));
            (anchor, session.complete(parameters(), cx).unwrap())
        });
        let request = only_pending(&fake);
        fake.send_last_text(&model, "partial");
        cx.run_until_parked();

        session
            .update(cx, |session, _| session.truncate(anchor))
            .unwrap();
        cx.run_until_parked();

        assert_eq!(
            ready_events(&mut stream),
            (vec![text("partial")], StreamState::Ended)
        );
        assert!(fake.is_stream_closed(&model, &request));
        assert!(session.read_with(cx, |session, _| session.log().entries().is_empty()));
    }

    #[gpui::test]
    fn submit_tool_result_does_not_cancel_the_active_round(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let model = fake.model("fake");
        let mut stream = session.update(cx, |session, cx| {
            session.append(user_text("Read the file"));
            session.complete(parameters(), cx).unwrap()
        });
        let request = only_pending(&fake);
        fake.send_last_event(
            &model,
            LanguageModelCompletionEvent::ToolUse(tool_use("tool-1")),
        );
        cx.run_until_parked();

        session
            .update(cx, |session, _| {
                session.submit_tool_result(tool_result("tool-1"))
            })
            .unwrap();
        fake.send_last_text(&model, "More.");
        cx.run_until_parked();

        assert_eq!(
            ready_events(&mut stream),
            (
                vec![
                    LanguageModelCompletionEvent::ToolUse(tool_use("tool-1")),
                    text("More."),
                ],
                StreamState::Open
            )
        );
        assert!(!fake.is_stream_closed(&model, &request));
        let output = session.read_with(cx, |session, _| last_output(session.log()));
        assert_eq!(
            output.messages[0].content,
            vec![
                MessageContent::ToolUse(tool_use("tool-1")),
                MessageContent::Text("More.".into()),
            ]
        );
    }

    #[gpui::test]
    fn complete_during_a_held_round_errors(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let stream = session.update(cx, |session, cx| {
            session.append(user_text("Read the file"));
            session.complete(parameters(), cx).unwrap()
        });
        fake.send_last_event(
            &fake.model("fake"),
            LanguageModelCompletionEvent::ToolUse(tool_use("tool-1")),
        );
        cx.run_until_parked();

        let result = session.update(cx, |session, cx| {
            // The log alone would allow another round after a tool result.
            session.submit_tool_result(tool_result("tool-1")).unwrap();
            session.complete(parameters(), cx).map(drop)
        });

        assert_eq!(result, Err(SessionError::RoundInProgress));
        assert_eq!(fake.completion_count(), 1);

        // Once the host drops the stream, the next round can start right away.
        drop(stream);
        let result = session.update(cx, |session, cx| {
            session.complete(parameters(), cx).map(drop)
        });
        assert_eq!(result, Ok(()));
        assert_eq!(fake.completion_count(), 2);
    }

    #[gpui::test]
    fn connect_error_is_the_first_stream_item_and_ends_the_round(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        fake.forbid_requests();
        let mut stream = session.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters(), cx).unwrap()
        });
        cx.run_until_parked();

        assert!(matches!(stream.next().now_or_never(), Some(Some(Err(_)))));
        assert!(matches!(stream.next().now_or_never(), Some(None)));
        let entries = session.read_with(cx, |session, _| session.log().entries().to_vec());
        assert!(
            matches!(entries.as_slice(), [SessionLogEntry::Input { .. }]),
            "{entries:?}"
        );

        fake.allow_requests();
        let retry = session.update(cx, |session, cx| {
            session.complete(parameters(), cx).map(drop)
        });
        assert_eq!(retry, Ok(()));
    }

    #[gpui::test]
    fn dropping_the_stream_while_connecting_cancels_the_connection(cx: &mut TestAppContext) {
        let (connection_alive, connection) = oneshot::channel();
        let client = Arc::new(PendingConnectionClient {
            connection: Mutex::new(Some(connection)),
        });
        let session = cx.new(|_| {
            LanguageModelSession::new(
                client,
                FakeLanguageModelProvider::default().model("fake"),
                SessionConfig {
                    thread_id: None,
                    prompt_cache_key: None,
                    system_prompt: None,
                    tools: Vec::new(),
                },
            )
        });
        let stream = session.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters(), cx).unwrap()
        });
        cx.run_until_parked();
        assert!(!connection_alive.is_canceled());

        drop(stream);
        cx.run_until_parked();

        assert!(connection_alive.is_canceled());
        let (round_active, entries) = session.read_with(cx, |session, _| {
            (
                session.active_round.is_some(),
                session.log().entries().to_vec(),
            )
        });
        assert!(!round_active);
        assert!(
            matches!(entries.as_slice(), [SessionLogEntry::Input { .. }]),
            "{entries:?}"
        );
    }

    #[gpui::test]
    fn mid_stream_error_ends_the_round_without_an_empty_output(cx: &mut TestAppContext) {
        let (fake, session) = setup(cx);
        let model = fake.model("fake");
        let mut stream = session.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters(), cx).unwrap()
        });
        fake.send_last_event(&model, LanguageModelCompletionEvent::Started);
        fake.send_last_error(
            &model,
            LanguageModelCompletionError::Other(anyhow::anyhow!("overloaded")),
        );
        cx.run_until_parked();

        assert!(matches!(
            stream.next().now_or_never(),
            Some(Some(Ok(LanguageModelCompletionEvent::Started)))
        ));
        assert!(matches!(stream.next().now_or_never(), Some(Some(Err(_)))));
        assert!(matches!(stream.next().now_or_never(), Some(None)));
        let entries = session.read_with(cx, |session, _| session.log().entries().to_vec());
        assert!(
            matches!(entries.as_slice(), [SessionLogEntry::Input { .. }]),
            "{entries:?}"
        );
        let retry = session.update(cx, |session, cx| {
            session.complete(parameters(), cx).map(drop)
        });
        assert_eq!(retry, Ok(()));
    }

    #[gpui::test]
    fn resume_from_export_sends_the_same_request(cx: &mut TestAppContext) {
        let (fake, original) = setup(cx);
        let model = fake.model("fake");
        let _first_stream = original.update(cx, |session, cx| {
            session.append(user_text("Hello"));
            session.complete(parameters(), cx).unwrap()
        });
        fake.send_last_event(
            &model,
            LanguageModelCompletionEvent::Thinking {
                text: "Greeting.".into(),
                signature: Some("signature".into()),
            },
        );
        fake.send_last_text(&model, "Hi!");
        fake.send_last_event(
            &model,
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn),
        );
        fake.end_last(&model);
        cx.run_until_parked();
        original.update(cx, |session, _| session.append(user_text("How are you?")));

        let exported = original.read_with(cx, |session, _| session.export());
        let resumed =
            cx.new(|_| LanguageModelSession::resume(fake.clone(), model.clone(), exported));
        let _original_stream = original.update(cx, |session, cx| {
            session.complete(parameters(), cx).unwrap()
        });
        let _resumed_stream = resumed.update(cx, |session, cx| {
            session.complete(parameters(), cx).unwrap()
        });

        let [from_original, from_resumed] = fake.pending_completions().try_into().unwrap();
        assert_eq!(from_resumed, from_original);
        assert_eq!(from_original.messages.len(), 4);
    }

    /// Connects forever, holding `connection` so tests can observe when the
    /// connection future is dropped.
    struct PendingConnectionClient {
        connection: Mutex<Option<oneshot::Receiver<()>>>,
    }

    impl LanguageModelClient for PendingConnectionClient {
        fn stream_completion(
            &self,
            _model: &LanguageModel,
            _request: LanguageModelRequest,
            _cx: &AsyncApp,
        ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
        {
            let connection = self.connection.lock().take();
            async move {
                let _connection = connection;
                futures::future::pending().await
            }
            .boxed()
        }
    }

    #[derive(Debug, PartialEq)]
    enum StreamState {
        Open,
        Ended,
    }

    fn setup(
        cx: &mut TestAppContext,
    ) -> (Arc<FakeLanguageModelProvider>, Entity<LanguageModelSession>) {
        let fake = Arc::new(FakeLanguageModelProvider::default());
        let model = fake.model("fake");
        let session = cx.new(|_| {
            LanguageModelSession::new(
                fake.clone(),
                model,
                SessionConfig {
                    thread_id: Some("thread".into()),
                    prompt_cache_key: None,
                    system_prompt: Some("You are helpful.".into()),
                    tools: Vec::new(),
                },
            )
        });
        (fake, session)
    }

    fn parameters() -> RoundParameters {
        RoundParameters {
            intent: Some(CompletionIntent::UserPrompt),
            thinking_allowed: true,
            ..RoundParameters::default()
        }
    }

    /// Events the stream has ready, and whether it has ended.
    fn ready_events(
        stream: &mut LanguageModelCompletionStream,
    ) -> (Vec<LanguageModelCompletionEvent>, StreamState) {
        let mut events = Vec::new();
        loop {
            match stream.next().now_or_never() {
                Some(Some(event)) => events.push(event.unwrap()),
                Some(None) => return (events, StreamState::Ended),
                None => return (events, StreamState::Open),
            }
        }
    }

    fn only_pending(fake: &FakeLanguageModelProvider) -> LanguageModelRequest {
        let [request] = fake.pending_completions().try_into().unwrap();
        request
    }

    fn without_cache_marks(
        request: &LanguageModelRequest,
    ) -> Vec<crate::LanguageModelRequestMessage> {
        request
            .messages
            .iter()
            .cloned()
            .map(|message| crate::LanguageModelRequestMessage {
                cache: false,
                ..message
            })
            .collect()
    }

    fn last_output(log: &SessionLog) -> AssistantOutput {
        log.entries()
            .iter()
            .rev()
            .find_map(|entry| match entry {
                SessionLogEntry::Output(output) => Some(output.clone()),
                _ => None,
            })
            .unwrap()
    }

    fn output_text(log: &SessionLog) -> String {
        last_output(log)
            .messages
            .iter()
            .flat_map(|message| &message.content)
            .filter_map(|content| match content {
                MessageContent::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    fn user_text(text: &str) -> SessionInput {
        SessionInput::UserMessage(vec![UserContent::Text(text.into())])
    }

    fn text(text: &str) -> LanguageModelCompletionEvent {
        LanguageModelCompletionEvent::Text(text.into())
    }

    fn tool_use(id: &str) -> LanguageModelToolUse {
        LanguageModelToolUse {
            id: id.into(),
            name: "read".into(),
            raw_input: "{}".into(),
            input: LanguageModelToolUseInput::Json(json!({})),
            is_input_complete: true,
            thought_signature: None,
        }
    }

    fn tool_result(id: &str) -> LanguageModelToolResult {
        LanguageModelToolResult {
            tool_use_id: id.into(),
            tool_name: "read".into(),
            is_error: false,
            content: vec![LanguageModelToolResultContent::Text("contents".into())],
            output: None,
        }
    }
}
