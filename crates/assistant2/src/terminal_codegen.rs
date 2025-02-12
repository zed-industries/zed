use crate::inline_prompt_editor::CodegenStatus;
use futures::{channel::mpsc, SinkExt, StreamExt};
use gpui::{App, Context, Entity, EventEmitter, Task};
use language_model::{LanguageModelRegistry, LanguageModelRequest};
use std::time::Instant;
use terminal::Terminal;

pub struct TerminalCodegen {
    pub status: CodegenStatus,
    terminal: Entity<Terminal>,
    generation: Task<()>,
    pub message_id: Option<String>,
    transaction: Option<TerminalTransaction>,
}

impl EventEmitter<CodegenEvent> for TerminalCodegen {}

impl TerminalCodegen {
    pub fn new(terminal: Entity<Terminal>) -> Self {
        Self {
            terminal,
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
            message_id: None,
            transaction: None,
        }
    }

    pub fn start(&mut self, prompt: LanguageModelRequest, cx: &mut Context<Self>) {
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };

        self.status = CodegenStatus::Pending;
        self.transaction = Some(TerminalTransaction::start(self.terminal.clone()));
        self.generation = cx.spawn(|this, mut cx| async move {
            let response = model.stream_completion_text(prompt, &cx).await;
            let generate = async {
                let message_id = response
                    .as_ref()
                    .ok()
                    .and_then(|response| response.message_id.clone());

                let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);

                let task = cx.background_executor().spawn({
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

                        result?;
                        anyhow::Ok(())
                    }
                });

                this.update(&mut cx, |this, _| {
                    this.message_id = message_id;
                })?;

                while let Some(hunk) = hunks_rx.next().await {
                    this.update(&mut cx, |this, cx| {
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

            this.update(&mut cx, |this, cx| {
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

pub const CLEAR_INPUT: &str = "\x15";
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

    fn sanitize_input(input: String) -> String {
        input.replace(['\r', '\n'], "")
    }
}
