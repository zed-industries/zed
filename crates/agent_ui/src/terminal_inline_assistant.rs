use crate::{
    context::load_context,
    inline_prompt_editor::{
        CodegenStatus, PromptEditor, PromptEditorEvent, TerminalInlineAssistId,
    },
    terminal_codegen::{CLEAR_INPUT, CodegenEvent, TerminalCodegen},
};
use agent::ThreadStore;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result};

use cloud_llm_client::CompletionIntent;
use collections::{HashMap, VecDeque};
use editor::{MultiBuffer, actions::SelectAll};
use fs::Fs;
use gpui::{App, Entity, Focusable, Global, Subscription, Task, UpdateGlobal, WeakEntity};
use language::Buffer;
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    Role, report_anthropic_event,
};
use project::Project;
use prompt_store::{PromptBuilder, PromptStore};
use std::sync::Arc;
use terminal_view::TerminalView;
use ui::prelude::*;
use util::ResultExt;
use uuid::Uuid;
use workspace::{Toast, Workspace, notifications::NotificationId};

pub fn init(fs: Arc<dyn Fs>, prompt_builder: Arc<PromptBuilder>, cx: &mut App) {
    cx.set_global(TerminalInlineAssistant::new(fs, prompt_builder));
}

const DEFAULT_CONTEXT_LINES: usize = 50;
const PROMPT_HISTORY_MAX_LEN: usize = 20;

pub struct TerminalInlineAssistant {
    next_assist_id: TerminalInlineAssistId,
    assists: HashMap<TerminalInlineAssistId, TerminalInlineAssist>,
    prompt_history: VecDeque<String>,
    fs: Arc<dyn Fs>,
    prompt_builder: Arc<PromptBuilder>,
}

impl Global for TerminalInlineAssistant {}

impl TerminalInlineAssistant {
    pub fn new(fs: Arc<dyn Fs>, prompt_builder: Arc<PromptBuilder>) -> Self {
        Self {
            next_assist_id: TerminalInlineAssistId::default(),
            assists: HashMap::default(),
            prompt_history: VecDeque::default(),
            fs,
            prompt_builder,
        }
    }

    pub fn assist(
        &mut self,
        terminal_view: &Entity<TerminalView>,
        workspace: WeakEntity<Workspace>,
        project: WeakEntity<Project>,
        thread_store: Entity<ThreadStore>,
        prompt_store: Option<Entity<PromptStore>>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let terminal = terminal_view.read(cx).terminal().clone();
        let assist_id = self.next_assist_id.post_inc();
        let session_id = Uuid::new_v4();
        let prompt_buffer = cx.new(|cx| {
            MultiBuffer::singleton(
                cx.new(|cx| Buffer::local(initial_prompt.unwrap_or_default(), cx)),
                cx,
            )
        });
        let codegen = cx.new(|_| TerminalCodegen::new(terminal, session_id));

        let prompt_editor = cx.new(|cx| {
            PromptEditor::new_terminal(
                assist_id,
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                codegen,
                session_id,
                self.fs.clone(),
                thread_store.clone(),
                prompt_store.clone(),
                project.clone(),
                workspace.clone(),
                window,
                cx,
            )
        });
        let prompt_editor_render = prompt_editor.clone();
        let block = terminal_view::BlockProperties {
            height: 4,
            render: Box::new(move |_| prompt_editor_render.clone().into_any_element()),
        };
        terminal_view.update(cx, |terminal_view, cx| {
            terminal_view.set_block_below_cursor(block, window, cx);
        });

        let terminal_assistant = TerminalInlineAssist::new(
            assist_id,
            terminal_view,
            prompt_editor,
            workspace.clone(),
            window,
            cx,
        );

        self.assists.insert(assist_id, terminal_assistant);

        self.focus_assist(assist_id, window, cx);
    }

    fn focus_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let assist = &self.assists[&assist_id];
        if let Some(prompt_editor) = assist.prompt_editor.as_ref() {
            prompt_editor.update(cx, |this, cx| {
                this.editor.update(cx, |editor, cx| {
                    window.focus(&editor.focus_handle(cx), cx);
                    editor.select_all(&SelectAll, window, cx);
                });
            });
        }
    }

    fn handle_prompt_editor_event(
        &mut self,
        prompt_editor: Entity<PromptEditor<TerminalCodegen>>,
        event: &PromptEditorEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let assist_id = prompt_editor.read(cx).id();
        match event {
            PromptEditorEvent::StartRequested => {
                self.start_assist(assist_id, cx);
            }
            PromptEditorEvent::StopRequested => {
                self.stop_assist(assist_id, cx);
            }
            PromptEditorEvent::ConfirmRequested { execute } => {
                self.finish_assist(assist_id, false, *execute, window, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, false, window, cx);
            }
            PromptEditorEvent::Resized { height_in_lines } => {
                self.insert_prompt_editor_into_terminal(assist_id, *height_in_lines, window, cx);
            }
        }
    }

    fn start_assist(&mut self, assist_id: TerminalInlineAssistId, cx: &mut App) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        let Some(user_prompt) = assist
            .prompt_editor
            .as_ref()
            .map(|editor| editor.read(cx).prompt(cx))
        else {
            return;
        };

        self.prompt_history.retain(|prompt| *prompt != user_prompt);
        self.prompt_history.push_back(user_prompt);
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        assist
            .terminal
            .update(cx, |terminal, cx| {
                terminal
                    .terminal()
                    .update(cx, |terminal, _| terminal.input(CLEAR_INPUT.as_bytes()));
            })
            .log_err();

        let codegen = assist.codegen.clone();
        let Some(request_task) = self.request_for_inline_assist(assist_id, cx).log_err() else {
            return;
        };

        codegen.update(cx, |codegen, cx| codegen.start(request_task, cx));
    }

    fn stop_assist(&mut self, assist_id: TerminalInlineAssistId, cx: &mut App) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        assist.codegen.update(cx, |codegen, cx| codegen.stop(cx));
    }

    fn request_for_inline_assist(
        &self,
        assist_id: TerminalInlineAssistId,
        cx: &mut App,
    ) -> Result<Task<LanguageModelRequest>> {
        let ConfiguredModel { model, .. } = LanguageModelRegistry::read_global(cx)
            .inline_assistant_model()
            .context("No inline assistant model")?;

        let assist = self.assists.get(&assist_id).context("invalid assist")?;

        let shell = std::env::var("SHELL").ok();
        let (latest_output, working_directory) = assist
            .terminal
            .update(cx, |terminal, cx| {
                let terminal = terminal.entity().read(cx);
                let latest_output = terminal.last_n_non_empty_lines(DEFAULT_CONTEXT_LINES);
                let working_directory = terminal
                    .working_directory()
                    .map(|path| path.to_string_lossy().into_owned());
                (latest_output, working_directory)
            })
            .ok()
            .unwrap_or_default();

        let prompt_editor = assist.prompt_editor.clone().context("invalid assist")?;

        let prompt = self.prompt_builder.generate_terminal_assistant_prompt(
            &prompt_editor.read(cx).prompt(cx),
            shell.as_deref(),
            working_directory.as_deref(),
            &latest_output,
        )?;

        let temperature = AgentSettings::temperature_for_model(&model, cx);

        let mention_set = prompt_editor.read(cx).mention_set().clone();
        let load_context_task = load_context(&mention_set, cx);

        Ok(cx.background_spawn(async move {
            let mut request_message = LanguageModelRequestMessage {
                role: Role::User,
                content: vec![],
                cache: false,
                reasoning_details: None,
            };

            if let Some(context) = load_context_task.await {
                context.add_to_request_message(&mut request_message);
            }

            request_message.content.push(prompt.into());

            LanguageModelRequest {
                thread_id: None,
                prompt_id: None,
                mode: None,
                intent: Some(CompletionIntent::TerminalInlineAssist),
                messages: vec![request_message],
                tools: Vec::new(),
                tool_choice: None,
                stop: Vec::new(),
                temperature,
                thinking_allowed: false,
            }
        }))
    }

    fn finish_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        undo: bool,
        execute: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.dismiss_assist(assist_id, window, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            assist
                .terminal
                .update(cx, |this, cx| {
                    this.clear_block_below_cursor(cx);
                    this.focus_handle(cx).focus(window, cx);
                })
                .log_err();

            if let Some(ConfiguredModel { model, .. }) =
                LanguageModelRegistry::read_global(cx).inline_assistant_model()
            {
                let codegen = assist.codegen.read(cx);
                let session_id = codegen.session_id();
                let message_id = codegen.message_id.clone();
                let model_telemetry_id = model.telemetry_id();
                let model_provider_id = model.provider_id().to_string();

                let (phase, event_type, anthropic_event_type) = if undo {
                    (
                        "rejected",
                        "Assistant Response Rejected",
                        language_model::AnthropicEventType::Reject,
                    )
                } else {
                    (
                        "accepted",
                        "Assistant Response Accepted",
                        language_model::AnthropicEventType::Accept,
                    )
                };

                // Fire Zed telemetry
                telemetry::event!(
                    event_type,
                    kind = "inline_terminal",
                    phase = phase,
                    model = model_telemetry_id,
                    model_provider = model_provider_id,
                    message_id = message_id,
                    session_id = session_id,
                );

                report_anthropic_event(
                    &model,
                    language_model::AnthropicEventData {
                        completion_type: language_model::AnthropicCompletionType::Terminal,
                        event: anthropic_event_type,
                        language_name: None,
                        message_id,
                    },
                    cx,
                );
            }

            assist.codegen.update(cx, |codegen, cx| {
                if undo {
                    codegen.undo(cx);
                } else if execute {
                    codegen.complete(cx);
                }
            });
        }
    }

    fn dismiss_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return false;
        };
        if assist.prompt_editor.is_none() {
            return false;
        }
        assist.prompt_editor = None;
        assist
            .terminal
            .update(cx, |this, cx| {
                this.clear_block_below_cursor(cx);
                this.focus_handle(cx).focus(window, cx);
            })
            .is_ok()
    }

    fn insert_prompt_editor_into_terminal(
        &mut self,
        assist_id: TerminalInlineAssistId,
        height: u8,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(assist) = self.assists.get_mut(&assist_id)
            && let Some(prompt_editor) = assist.prompt_editor.as_ref().cloned()
        {
            assist
                .terminal
                .update(cx, |terminal, cx| {
                    terminal.clear_block_below_cursor(cx);
                    let block = terminal_view::BlockProperties {
                        height,
                        render: Box::new(move |_| prompt_editor.clone().into_any_element()),
                    };
                    terminal.set_block_below_cursor(block, window, cx);
                })
                .log_err();
        }
    }
}

struct TerminalInlineAssist {
    terminal: WeakEntity<TerminalView>,
    prompt_editor: Option<Entity<PromptEditor<TerminalCodegen>>>,
    codegen: Entity<TerminalCodegen>,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl TerminalInlineAssist {
    pub fn new(
        assist_id: TerminalInlineAssistId,
        terminal: &Entity<TerminalView>,
        prompt_editor: Entity<PromptEditor<TerminalCodegen>>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let codegen = prompt_editor.read(cx).codegen().clone();
        Self {
            terminal: terminal.downgrade(),
            prompt_editor: Some(prompt_editor.clone()),
            codegen: codegen.clone(),
            workspace,
            _subscriptions: vec![
                window.subscribe(&prompt_editor, cx, |prompt_editor, event, window, cx| {
                    TerminalInlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_event(prompt_editor, event, window, cx)
                    })
                }),
                window.subscribe(&codegen, cx, move |codegen, event, window, cx| {
                    TerminalInlineAssistant::update_global(cx, |this, cx| match event {
                        CodegenEvent::Finished => {
                            let assist = if let Some(assist) = this.assists.get(&assist_id) {
                                assist
                            } else {
                                return;
                            };

                            if let CodegenStatus::Error(error) = &codegen.read(cx).status
                                && assist.prompt_editor.is_none()
                                && let Some(workspace) = assist.workspace.upgrade()
                            {
                                let error = format!("Terminal inline assistant error: {}", error);
                                workspace.update(cx, |workspace, cx| {
                                    struct InlineAssistantError;

                                    let id = NotificationId::composite::<InlineAssistantError>(
                                        assist_id.0,
                                    );

                                    workspace.show_toast(Toast::new(id, error), cx);
                                })
                            }

                            if assist.prompt_editor.is_none() {
                                this.finish_assist(assist_id, false, false, window, cx);
                            }
                        }
                    })
                }),
            ],
        }
    }
}
