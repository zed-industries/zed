use crate::inline_prompt_editor::CodegenStatus;
use client::telemetry::Telemetry;
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Task};
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, report_assistant_event,
};
use std::{sync::Arc, time::Instant};
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};
use terminal::Terminal;

pub struct TerminalCodegen {
    pub status: CodegenStatus,
    pub telemetry: Option<Arc<Telemetry>>,
    terminal: Entity<Terminal>,
    generation: Task<()>,
    pub message_id: Option<String>,
    transaction: Option<TerminalTransaction>,
}

impl EventEmitter<CodegenEvent> for TerminalCodegen {}

impl TerminalCodegen {
    pub fn new(terminal: Entity<Terminal>, telemetry: Option<Arc<Telemetry>>) -> Self {
        Self {
            terminal,
            telemetry,
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
            message_id: None,
            transaction: None,
        }
    }

    pub fn start(&mut self, prompt_task: Task<LanguageModelRequest>, cx: &mut Context<Self>) {
        let Some(ConfiguredModel { model, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        else {
            return;
        };

        let model_api_key = model.api_key(cx);
        let http_client = cx.http_client();
        let telemetry = self.telemetry.clone();
        self.status = CodegenStatus::Pending;
        self.transaction = Some(TerminalTransaction::start(self.terminal.clone()));
        self.generation = cx.spawn(async move |this, cx| {
            let prompt = prompt_task.await;
            let model_telemetry_id = model.telemetry_id();
            let model_provider_id = model.provider_id();
            let response = model.stream_completion_text(prompt, &cx).await;
            let generate = async {
                let message_id = response
                    .as_ref()
                    .ok()
                    .and_then(|response| response.message_id.clone());

                let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);

                let task = cx.background_spawn({
                    let message_id = message_id.clone();
                    let executor = cx.background_executor().clone();
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
                        report_assistant_event(
                            AssistantEventData {
                                conversation_id: None,
                                kind: AssistantKind::InlineTerminal,
                                message_id,
                                phase: AssistantPhase::Response,
                                model: model_telemetry_id,
                                model_provider: model_provider_id.to_string(),
                                response_latency,
                                error_message,
                                language_name: None,
                            },
                            telemetry,
                            http_client,
                            model_api_key,
                            &executor,
                        );

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
    terminal: Entity<Terminal>,
}

impl TerminalTransaction {
    pub fn start(terminal: Entity<Terminal>) -> Self {
        Self { terminal }
    }

    pub fn push(&mut self, hunk: String, cx: &mut App) {
        // Ensure that the assistant cannot accidentally execute commands that are streamed into the terminal
        let input = Self::sanitize_input(hunk);
        self.terminal
            .update(cx, |terminal, _| terminal.input(input));
    }

    pub fn undo(&self, cx: &mut App) {
        self.terminal
            .update(cx, |terminal, _| terminal.input(CLEAR_INPUT.to_string()));
    }

    pub fn complete(&self, cx: &mut App) {
        self.terminal.update(cx, |terminal, _| {
            terminal.input(CARRIAGE_RETURN.to_string())
        });
    }

    fn sanitize_input(mut input: String) -> String {
        input.retain(|c| c != '\r' && c != '\n');
        input
    }
}
