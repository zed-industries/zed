use crate::inline_prompt_editor::CodegenStatus;
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Task};
use language_model::{ConfiguredModel, LanguageModelRegistry, LanguageModelRequest};
use std::time::Instant;
use terminal::Terminal;
use uuid::Uuid;

pub struct TerminalCodegen {
    pub status: CodegenStatus,
    terminal: Entity<Terminal>,
    generation: Task<()>,
    pub message_id: Option<String>,
    transaction: Option<TerminalTransaction>,
    session_id: Uuid,
}

impl EventEmitter<CodegenEvent> for TerminalCodegen {}

impl TerminalCodegen {
    pub fn new(terminal: Entity<Terminal>, session_id: Uuid) -> Self {
        Self {
            terminal,
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
            message_id: None,
            transaction: None,
            session_id,
        }
    }

    pub fn session_id(&self) -> Uuid {
        self.session_id
    }

    pub fn start(&mut self, prompt_task: Task<LanguageModelRequest>, cx: &mut Context<Self>) {
        let Some(ConfiguredModel { model, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        else {
            return;
        };

        let anthropic_reporter = language_model::AnthropicEventReporter::new(&model, cx);
        let session_id = self.session_id;
        let model_telemetry_id = model.telemetry_id();
        let model_provider_id = model.provider_id().to_string();

        self.status = CodegenStatus::Pending;
        self.transaction = Some(TerminalTransaction::start(self.terminal.clone()));
        self.generation = cx.spawn(async move |this, cx| {
            let prompt = prompt_task.await;
            let response = model.stream_completion_text(prompt, cx).await;
            let generate = async {
                let message_id = response
                    .as_ref()
                    .ok()
                    .and_then(|response| response.message_id.clone());

                let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);

                let task = cx.background_spawn({
                    let message_id = message_id.clone();
                    let anthropic_reporter = anthropic_reporter.clone();
                    async move {
                        let mut response_latency = None;
                        let request_start = Instant::now();
                        let task = async {
                            let mut chunks = response?.stream;
                            while let Some(chunk) = chunks.next().await {
                                if response_latency.is_none() {
                                    response_latency = Some(request_start.elapsed());
                                }
                                let chunk = chunk?;
                                hunks_tx.send(chunk).await?;
                            }

                            anyhow::Ok(())
                        };

                        let result = task.await;

                        let error_message = result.as_ref().err().map(|error| error.to_string());

                        telemetry::event!(
                            "Assistant Responded",
                            session_id = session_id.to_string(),
                            kind = "inline_terminal",
                            phase = "response",
                            model = model_telemetry_id,
                            model_provider = model_provider_id,
                            language_name = Option::<&str>::None,
                            message_id = message_id,
                            response_latency = response_latency,
                            error_message = error_message,
                        );

                        anthropic_reporter.report(language_model::AnthropicEventData {
                            completion_type: language_model::AnthropicCompletionType::Terminal,
                            event: language_model::AnthropicEventType::Response,
                            language_name: None,
                            message_id,
                        });

                        result?;
                        anyhow::Ok(())
                    }
                });

                this.update(cx, |this, _| {
                    this.message_id = message_id;
                })?;

                while let Some(hunk) = hunks_rx.next().await {
                    this.update(cx, |this, cx| {
                        if let Some(transaction) = &mut this.transaction {
                            transaction.push(hunk, cx);
                            cx.notify();
                        }
                    })?;
                }

                task.await?;
                anyhow::Ok(())
            };

            let result = generate.await;

            this.update(cx, |this, cx| {
                if let Err(error) = result {
                    this.status = CodegenStatus::Error(error);
                } else {
                    this.status = CodegenStatus::Done;
                }
                cx.emit(CodegenEvent::Finished);
                cx.notify();
            })
            .ok();
        });
        cx.notify();
    }

    pub fn completion(&self) -> Option<String> {
        self.transaction
            .as_ref()
            .map(|transaction| transaction.completion.clone())
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.status = CodegenStatus::Done;
        self.generation = Task::ready(());
        cx.emit(CodegenEvent::Finished);
        cx.notify();
    }

    pub fn complete(&mut self, cx: &mut Context<Self>) {
        if let Some(transaction) = self.transaction.take() {
            transaction.complete(cx);
        }
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        if let Some(transaction) = self.transaction.take() {
            transaction.undo(cx);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CodegenEvent {
    Finished,
}

#[cfg(not(target_os = "windows"))]
pub const CLEAR_INPUT: &str = "\x15";
#[cfg(target_os = "windows")]
pub const CLEAR_INPUT: &str = "\x03";
const CARRIAGE_RETURN: &str = "\x0d";

struct TerminalTransaction {
    completion: String,
    terminal: Entity<Terminal>,
}

impl TerminalTransaction {
    pub fn start(terminal: Entity<Terminal>) -> Self {
        Self {
            completion: String::new(),
            terminal,
        }
    }

    pub fn push(&mut self, hunk: String, cx: &mut App) {
        // Ensure that the assistant cannot accidentally execute commands that are streamed into the terminal
        let input = Self::sanitize_input(hunk);
        self.completion.push_str(&input);
        self.terminal
            .update(cx, |terminal, _| terminal.input(input.into_bytes()));
    }

    pub fn undo(self, cx: &mut App) {
        self.terminal
            .update(cx, |terminal, _| terminal.input(CLEAR_INPUT.as_bytes()));
    }

    pub fn complete(self, cx: &mut App) {
        self.terminal
            .update(cx, |terminal, _| terminal.input(CARRIAGE_RETURN.as_bytes()));
    }

    fn sanitize_input(mut input: String) -> String {
        input.retain(|c| c != '\r' && c != '\n');
        input
    }
}
