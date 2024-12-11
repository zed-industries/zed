use crate::assistant_settings::AssistantSettings;
use crate::{
    humanize_token_count, prompts::PromptBuilder, AssistantPanel, AssistantPanelEvent, RequestType,
    DEFAULT_CONTEXT_LINES,
};
use anyhow::{Context as _, Result};
use client::telemetry::Telemetry;
use collections::{HashMap, VecDeque};
use editor::{
    actions::{MoveDown, MoveUp, SelectAll},
    Editor, EditorElement, EditorEvent, EditorMode, EditorStyle, MultiBuffer,
};
use fs::Fs;
use futures::{channel::mpsc, SinkExt, StreamExt};
use gpui::{
    AppContext, Context, EventEmitter, FocusHandle, FocusableView, Global, Model, Subscription,
    Task, TextStyle, UpdateGlobal, View, WeakView,
};
use language::Buffer;
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use language_model_selector::LanguageModelSelector;
use language_models::report_assistant_event;
use settings::{update_settings_file, Settings};
use std::{
    cmp,
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::{AssistantEvent, AssistantKind, AssistantPhase};
use terminal::Terminal;
use terminal_view::TerminalView;
use theme::ThemeSettings;
use ui::{prelude::*, text_for_action, IconButtonShape, Tooltip};
use util::ResultExt;
use workspace::{notifications::NotificationId, Toast, Workspace};

pub fn init(
    fs: Arc<dyn Fs>,
    prompt_builder: Arc<PromptBuilder>,
    telemetry: Arc<Telemetry>,
    cx: &mut AppContext,
) {
    cx.set_global(TerminalInlineAssistant::new(fs, prompt_builder, telemetry));
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct TerminalInlineAssistId(usize);

impl TerminalInlineAssistId {
    fn post_inc(&mut self) -> TerminalInlineAssistId {
        let id = *self;
        self.0 += 1;
        id
    }
}

pub struct TerminalInlineAssistant {
    next_assist_id: TerminalInlineAssistId,
    assists: HashMap<TerminalInlineAssistId, TerminalInlineAssist>,
    prompt_history: VecDeque<String>,
    telemetry: Option<Arc<Telemetry>>,
    fs: Arc<dyn Fs>,
    prompt_builder: Arc<PromptBuilder>,
}

impl Global for TerminalInlineAssistant {}

impl TerminalInlineAssistant {
    pub fn new(
        fs: Arc<dyn Fs>,
        prompt_builder: Arc<PromptBuilder>,
        telemetry: Arc<Telemetry>,
    ) -> Self {
        Self {
            next_assist_id: TerminalInlineAssistId::default(),
            assists: HashMap::default(),
            prompt_history: VecDeque::default(),
            telemetry: Some(telemetry),
            fs,
            prompt_builder,
        }
    }

    pub fn assist(
        &mut self,
        terminal_view: &Model<TerminalView>,
        workspace: Option<WeakModel<Workspace>>,
        assistant_panel: Option<&Model<AssistantPanel>>,
        initial_prompt: Option<String>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        let terminal = terminal_view.read(cx).terminal().clone();
        let assist_id = self.next_assist_id.post_inc();
        let prompt_buffer =
            cx.new_model(|model, cx| Buffer::local(initial_prompt.unwrap_or_default(), model, cx));
        let prompt_buffer =
            cx.new_model(|model, cx| MultiBuffer::singleton(prompt_buffer, model, cx));
        let codegen = cx.new_model(|_, _| Codegen::new(terminal, self.telemetry.clone()));

        let prompt_editor = cx.new_model(|model, cx| {
            PromptEditor::new(
                assist_id,
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                codegen,
                assistant_panel,
                workspace.clone(),
                self.fs.clone(),
                model,
                cx,
            )
        });
        let prompt_editor_render = prompt_editor.clone();
        let block = terminal_view::BlockProperties {
            height: 2,
            render: Box::new(move |_| prompt_editor_render.clone().into_any_element()),
        };
        terminal_view.update(cx, |terminal_view, model, cx| {
            terminal_view.set_block_below_cursor(block, model, cx);
        });

        let terminal_assistant = TerminalInlineAssist::new(
            assist_id,
            terminal_view,
            assistant_panel.is_some(),
            prompt_editor,
            workspace.clone(),
            model,
            cx,
        );

        self.assists.insert(assist_id, terminal_assistant);

        self.focus_assist(assist_id, model, cx);
    }

    fn focus_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        let assist = &self.assists[&assist_id];
        if let Some(prompt_editor) = assist.prompt_editor.as_ref() {
            prompt_editor.update(cx, |this, model, cx| {
                this.editor.update(cx, |editor, model, cx| {
                    editor.focus(window);
                    editor.select_all(&SelectAll, model, cx);
                });
            });
        }
    }

    fn handle_prompt_editor_event(
        &mut self,
        prompt_editor: Model<PromptEditor>,
        event: &PromptEditorEvent,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        let assist_id = prompt_editor.read(cx).id;
        match event {
            PromptEditorEvent::StartRequested => {
                self.start_assist(assist_id, model, cx);
            }
            PromptEditorEvent::StopRequested => {
                self.stop_assist(assist_id, model, cx);
            }
            PromptEditorEvent::ConfirmRequested { execute } => {
                self.finish_assist(assist_id, false, *execute, model, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, false, model, cx);
            }
            PromptEditorEvent::DismissRequested => {
                self.dismiss_assist(assist_id, model, cx);
            }
            PromptEditorEvent::Resized { height_in_lines } => {
                self.insert_prompt_editor_into_terminal(assist_id, *height_in_lines, model, cx);
            }
        }
    }

    fn start_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
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
        self.prompt_history.push_back(user_prompt.clone());
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        assist
            .terminal
            .update(cx, |terminal, model, cx| {
                terminal.terminal().update(cx, |terminal, model, _| {
                    terminal.input(CLEAR_INPUT.to_string())
                });
            })
            .log_err();

        let codegen = assist.codegen.clone();
        let Some(request) = self
            .request_for_inline_assist(assist_id, model, cx)
            .log_err()
        else {
            return;
        };

        codegen.update(cx, |codegen, model, cx| codegen.start(request, model, cx));
    }

    fn stop_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        assist
            .codegen
            .update(cx, |codegen, model, cx| codegen.stop(cx));
    }

    fn request_for_inline_assist(
        &self,
        assist_id: TerminalInlineAssistId,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Result<LanguageModelRequest> {
        let assist = self.assists.get(&assist_id).context("invalid assist")?;

        let shell = std::env::var("SHELL").ok();
        let (latest_output, working_directory) = assist
            .terminal
            .update(cx, |terminal, model, cx| {
                let terminal = terminal.model().read(cx);
                let latest_output = terminal.last_n_non_empty_lines(DEFAULT_CONTEXT_LINES);
                let working_directory = terminal
                    .working_directory()
                    .map(|path| path.to_string_lossy().to_string());
                (latest_output, working_directory)
            })
            .ok()
            .unwrap_or_default();

        let context_request = if assist.include_context {
            assist.workspace.as_ref().and_then(|workspace| {
                let workspace = workspace.upgrade()?.read(cx);
                let assistant_panel = workspace.panel::<AssistantPanel>(cx)?;
                Some(
                    assistant_panel
                        .read(cx)
                        .active_context(cx)?
                        .read(cx)
                        .to_completion_request(RequestType::Chat, cx),
                )
            })
        } else {
            None
        };

        let prompt = self.prompt_builder.generate_terminal_assistant_prompt(
            &assist
                .prompt_editor
                .clone()
                .context("invalid assist")?
                .read(cx)
                .prompt(cx),
            shell.as_deref(),
            working_directory.as_deref(),
            &latest_output,
        )?;

        let mut messages = Vec::new();
        if let Some(context_request) = context_request {
            messages = context_request.messages;
        }

        messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![prompt.into()],
            cache: false,
        });

        Ok(LanguageModelRequest {
            messages,
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
        })
    }

    fn finish_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        undo: bool,
        execute: bool,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        self.dismiss_assist(assist_id, model, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            assist
                .terminal
                .update(cx, |this, model, cx| {
                    this.clear_block_below_cursor(cx);
                    this.focus_handle(cx).focus(window);
                })
                .log_err();

            if let Some(model) = LanguageModelRegistry::read_global(cx).active_model() {
                let codegen = assist.codegen.read(cx);
                let executor = cx.background_executor().clone();
                report_assistant_event(
                    AssistantEvent {
                        conversation_id: None,
                        kind: AssistantKind::InlineTerminal,
                        message_id: codegen.message_id.clone(),
                        phase: if undo {
                            AssistantPhase::Rejected
                        } else {
                            AssistantPhase::Accepted
                        },
                        model: model.telemetry_id(),
                        model_provider: model.provider_id().to_string(),
                        response_latency: None,
                        error_message: None,
                        language_name: None,
                    },
                    codegen.telemetry.clone(),
                    cx.http_client(),
                    model.api_key(cx),
                    &executor,
                );
            }

            assist.codegen.update(cx, |codegen, model, cx| {
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
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
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
            .update(cx, |this, model, cx| {
                this.clear_block_below_cursor(cx);
                this.focus_handle(cx).focus(window);
            })
            .is_ok()
    }

    fn insert_prompt_editor_into_terminal(
        &mut self,
        assist_id: TerminalInlineAssistId,
        height: u8,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        if let Some(assist) = self.assists.get_mut(&assist_id) {
            if let Some(prompt_editor) = assist.prompt_editor.as_ref().cloned() {
                assist
                    .terminal
                    .update(cx, |terminal, model, cx| {
                        terminal.clear_block_below_cursor(cx);
                        let block = terminal_view::BlockProperties {
                            height,
                            render: Box::new(move |_| prompt_editor.clone().into_any_element()),
                        };
                        terminal.set_block_below_cursor(block, cx);
                    })
                    .log_err();
            }
        }
    }
}

struct TerminalInlineAssist {
    terminal: WeakModel<TerminalView>,
    prompt_editor: Option<Model<PromptEditor>>,
    codegen: Model<Codegen>,
    workspace: Option<WeakModel<Workspace>>,
    include_context: bool,
    _subscriptions: Vec<Subscription>,
}

impl TerminalInlineAssist {
    pub fn new(
        assist_id: TerminalInlineAssistId,
        terminal: &Model<TerminalView>,
        include_context: bool,
        prompt_editor: Model<PromptEditor>,
        workspace: Option<WeakModel<Workspace>>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Self {
        let codegen = prompt_editor.read(cx).codegen.clone();
        Self {
            terminal: terminal.downgrade(),
            prompt_editor: Some(prompt_editor.clone()),
            codegen: codegen.clone(),
            workspace: workspace.clone(),
            include_context,
            _subscriptions: vec![
                cx.subscribe(&prompt_editor, |prompt_editor, event, cx| {
                    TerminalInlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_event(prompt_editor, event, model, cx)
                    })
                }),
                cx.subscribe(&codegen, move |codegen, event, cx| {
                    TerminalInlineAssistant::update_global(cx, |this, cx| match event {
                        CodegenEvent::Finished => {
                            let assist = if let Some(assist) = this.assists.get(&assist_id) {
                                assist
                            } else {
                                return;
                            };

                            if let CodegenStatus::Error(error) = &codegen.read(cx).status {
                                if assist.prompt_editor.is_none() {
                                    if let Some(workspace) = assist
                                        .workspace
                                        .as_ref()
                                        .and_then(|workspace| workspace.upgrade())
                                    {
                                        let error =
                                            format!("Terminal inline assistant error: {}", error);
                                        workspace.update(cx, |workspace, model, cx| {
                                            struct InlineAssistantError;

                                            let id =
                                                NotificationId::composite::<InlineAssistantError>(
                                                    assist_id.0,
                                                );

                                            workspace.show_toast(Toast::new(id, error), cx);
                                        })
                                    }
                                }
                            }

                            if assist.prompt_editor.is_none() {
                                this.finish_assist(assist_id, false, false, model, cx);
                            }
                        }
                    })
                }),
            ],
        }
    }
}

enum PromptEditorEvent {
    StartRequested,
    StopRequested,
    ConfirmRequested { execute: bool },
    CancelRequested,
    DismissRequested,
    Resized { height_in_lines: u8 },
}

struct PromptEditor {
    id: TerminalInlineAssistId,
    fs: Arc<dyn Fs>,
    height_in_lines: u8,
    editor: Model<Editor>,
    edited_since_done: bool,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    codegen: Model<Codegen>,
    _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
    pending_token_count: Task<Result<()>>,
    token_count: Option<usize>,
    _token_count_subscriptions: Vec<Subscription>,
    workspace: Option<WeakModel<Workspace>>,
}

impl EventEmitter<PromptEditorEvent> for PromptEditor {}

impl Render for PromptEditor {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let status = &self.codegen.read(cx).status;
        let buttons =
            match status {
                CodegenStatus::Idle => {
                    vec![
                        IconButton::new("cancel", IconName::Close)
                            .icon_color(Color::Muted)
                            .shape(IconButtonShape::Square)
                            .tooltip(|window, cx| {
                                Tooltip::for_action("Cancel Assist", &menu::Cancel, model, cx)
                            })
                            .on_click(cx.listener(|_, _, cx| {
                                model.emit(PromptEditorEvent::CancelRequested, cx)
                            })),
                        IconButton::new("start", IconName::SparkleAlt)
                            .icon_color(Color::Muted)
                            .shape(IconButtonShape::Square)
                            .tooltip(|window, cx| {
                                Tooltip::for_action("Generate", &menu::Confirm, model, cx)
                            })
                            .on_click(cx.listener(|_, _, cx| {
                                model.emit(PromptEditorEvent::StartRequested, cx)
                            })),
                    ]
                }
                CodegenStatus::Pending => {
                    vec![
                        IconButton::new("cancel", IconName::Close)
                            .icon_color(Color::Muted)
                            .shape(IconButtonShape::Square)
                            .tooltip(|window, cx| Tooltip::text("Cancel Assist", cx))
                            .on_click(cx.listener(|_, _, cx| {
                                model.emit(PromptEditorEvent::CancelRequested, cx)
                            })),
                        IconButton::new("stop", IconName::Stop)
                            .icon_color(Color::Error)
                            .shape(IconButtonShape::Square)
                            .tooltip(|window, cx| {
                                Tooltip::with_meta(
                                    "Interrupt Generation",
                                    Some(&menu::Cancel),
                                    "Changes won't be discarded",
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|_, _, cx| {
                                model.emit(PromptEditorEvent::StopRequested, cx)
                            })),
                    ]
                }
                CodegenStatus::Error(_) | CodegenStatus::Done => {
                    let cancel = IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Cancel Assist", &menu::Cancel, model, cx)
                        })
                        .on_click(cx.listener(|_, _, cx| {
                            model.emit(PromptEditorEvent::CancelRequested, cx)
                        }));

                    let has_error = matches!(status, CodegenStatus::Error(_));
                    if has_error || self.edited_since_done {
                        vec![
                            cancel,
                            IconButton::new("restart", IconName::RotateCw)
                                .icon_color(Color::Info)
                                .shape(IconButtonShape::Square)
                                .tooltip(|window, cx| {
                                    Tooltip::with_meta(
                                        "Restart Generation",
                                        Some(&menu::Confirm),
                                        "Changes will be discarded",
                                        model,
                                        cx,
                                    )
                                })
                                .on_click(cx.listener(|_, _, cx| {
                                    model.emit(PromptEditorEvent::StartRequested, cx);
                                })),
                        ]
                    } else {
                        vec![
                            cancel,
                            IconButton::new("accept", IconName::Check)
                                .icon_color(Color::Info)
                                .shape(IconButtonShape::Square)
                                .tooltip(|window, cx| {
                                    Tooltip::for_action(
                                        "Accept Generated Command",
                                        &menu::Confirm,
                                        model,
                                        cx,
                                    )
                                })
                                .on_click(cx.listener(|_, _, cx| {
                                    model.emit(
                                        cx,
                                        PromptEditorEvent::ConfirmRequested { execute: false },
                                    );
                                })),
                            IconButton::new("confirm", IconName::Play)
                                .icon_color(Color::Info)
                                .shape(IconButtonShape::Square)
                                .tooltip(|window, cx| {
                                    Tooltip::for_action(
                                        "Execute Generated Command",
                                        &menu::SecondaryConfirm,
                                        model,
                                        cx,
                                    )
                                })
                                .on_click(cx.listener(|_, _, cx| {
                                    model.emit(
                                        cx,
                                        PromptEditorEvent::ConfirmRequested { execute: true },
                                    );
                                })),
                        ]
                    }
                }
            };

        h_flex()
            .bg(cx.theme().colors().editor_background)
            .border_y_1()
            .border_color(cx.theme().status().info_border)
            .py_2()
            .h_full()
            .w_full()
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::secondary_confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .child(
                h_flex()
                    .w_12()
                    .justify_center()
                    .gap_2()
                    .child(LanguageModelSelector::new(
                        {
                            let fs = self.fs.clone();
                            move |model, cx| {
                                update_settings_file::<AssistantSettings>(
                                    fs.clone(),
                                    cx,
                                    move |settings, _| settings.set_model(model.clone()),
                                );
                            }
                        },
                        IconButton::new("context", IconName::SettingsAlt)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(move |window, cx| {
                                Tooltip::with_meta(
                                    format!(
                                        "Using {}",
                                        LanguageModelRegistry::read_global(cx)
                                            .active_model()
                                            .map(|model| model.name().0)
                                            .unwrap_or_else(|| "No model selected".into()),
                                    ),
                                    None,
                                    "Change Model",
                                    model,
                                    cx,
                                )
                            }),
                    ))
                    .children(
                        if let CodegenStatus::Error(error) = &self.codegen.read(cx).status {
                            let error_message = SharedString::from(error.to_string());
                            Some(
                                div()
                                    .id("error")
                                    .tooltip(move |window, cx| {
                                        Tooltip::text(error_message.clone(), cx)
                                    })
                                    .child(
                                        Icon::new(IconName::XCircle)
                                            .size(IconSize::Small)
                                            .color(Color::Error),
                                    ),
                            )
                        } else {
                            None
                        },
                    ),
            )
            .child(div().flex_1().child(self.render_prompt_editor(cx)))
            .child(
                h_flex()
                    .gap_1()
                    .pr_4()
                    .children(self.render_token_count(cx))
                    .children(buttons),
            )
    }
}

impl FocusableView for PromptEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl PromptEditor {
    const MAX_LINES: u8 = 8;

    #[allow(clippy::too_many_arguments)]
    fn new(
        id: TerminalInlineAssistId,
        prompt_history: VecDeque<String>,
        prompt_buffer: Model<MultiBuffer>,
        codegen: Model<Codegen>,
        assistant_panel: Option<&Model<AssistantPanel>>,
        workspace: Option<WeakModel<Workspace>>,
        fs: Arc<dyn Fs>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> Self {
        let prompt_editor = cx.new_model(|model, cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    max_lines: Self::MAX_LINES as usize,
                },
                prompt_buffer,
                None,
                false,
                model,
                cx,
            );
            editor.set_soft_wrap_mode(
                language::language_settings::SoftWrap::EditorWidth,
                model,
                model,
                model,
                cx,
            );
            editor.set_placeholder_text(Self::placeholder_text(cx), model, cx);
            editor
        });

        let mut token_count_subscriptions = Vec::new();
        if let Some(assistant_panel) = assistant_panel {
            token_count_subscriptions
                .push(cx.subscribe(assistant_panel, Self::handle_assistant_panel_event));
        }

        let mut this = Self {
            id,
            height_in_lines: 1,
            editor: prompt_editor,
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: cx.observe(&codegen, Self::handle_codegen_changed),
            editor_subscriptions: Vec::new(),
            codegen,
            fs,
            pending_token_count: Task::ready(Ok(())),
            token_count: None,
            _token_count_subscriptions: token_count_subscriptions,
            workspace,
        };
        this.count_lines(cx);
        this.count_tokens(cx);
        this.subscribe_to_editor(cx);
        this
    }

    fn placeholder_text(window: &Window, cx: &AppContext) -> String {
        let context_keybinding = text_for_action(&crate::ToggleFocus, model, cx)
            .map(|keybinding| format!(" • {keybinding} for context"))
            .unwrap_or_default();

        format!("Generate…{context_keybinding} • ↓↑ for history")
    }

    fn subscribe_to_editor(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions
            .push(cx.observe(&self.editor, Self::handle_prompt_editor_changed));
        self.editor_subscriptions
            .push(cx.subscribe(&self.editor, Self::handle_prompt_editor_events));
    }

    fn prompt(&self, cx: &AppContext) -> String {
        self.editor.read(cx).text(cx)
    }

    fn count_lines(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        let height_in_lines = cmp::max(
            2, // Make the editor at least two lines tall, to account for padding and buttons.
            cmp::min(
                self.editor
                    .update(cx, |editor, model, cx| editor.max_point(cx).row().0 + 1),
                Self::MAX_LINES as u32,
            ),
        ) as u8;

        if height_in_lines != self.height_in_lines {
            self.height_in_lines = height_in_lines;
            model.emit(PromptEditorEvent::Resized { height_in_lines }, cx);
        }
    }

    fn handle_assistant_panel_event(
        &mut self,
        _: Model<AssistantPanel>,
        event: &AssistantPanelEvent,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let AssistantPanelEvent::ContextEdited { .. } = event;
        self.count_tokens(cx);
    }

    fn count_tokens(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        let assist_id = self.id;
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };
        self.pending_token_count = model.spawn(cx, |this, mut cx| async move {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let request =
                cx.update_global(|inline_assistant: &mut TerminalInlineAssistant, cx| {
                    inline_assistant.request_for_inline_assist(assist_id, model, cx)
                })??;

            let token_count = cx.update(|cx| model.count_tokens(request, cx))?.await?;
            this.update(&mut cx, |this, model, cx| {
                this.token_count = Some(token_count);
                model.notify(cx);
            })
        })
    }

    fn handle_prompt_editor_changed(
        &mut self,
        _: Model<Editor>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        self.count_lines(cx);
    }

    fn handle_prompt_editor_events(
        &mut self,
        _: Model<Editor>,
        event: &EditorEvent,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        match event {
            EditorEvent::Edited { .. } => {
                let prompt = self.editor.read(cx).text(cx);
                if self
                    .prompt_history_ix
                    .map_or(true, |ix| self.prompt_history[ix] != prompt)
                {
                    self.prompt_history_ix.take();
                    self.pending_prompt = prompt;
                }

                self.edited_since_done = true;
                model.notify(cx);
            }
            EditorEvent::BufferEdited => {
                self.count_tokens(cx);
            }
            _ => {}
        }
    }

    fn handle_codegen_changed(
        &mut self,
        _: Model<Codegen>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                self.editor
                    .update(cx, |editor, model, _| editor.set_read_only(false));
            }
            CodegenStatus::Pending => {
                self.editor
                    .update(cx, |editor, model, _| editor.set_read_only(true));
            }
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, model, _| editor.set_read_only(false));
            }
        }
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, model: &Model<Self>, cx: &mut AppContext) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle | CodegenStatus::Done | CodegenStatus::Error(_) => {
                model.emit(PromptEditorEvent::CancelRequested, cx);
            }
            CodegenStatus::Pending => {
                model.emit(PromptEditorEvent::StopRequested, cx);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, model: &Model<Self>, cx: &mut AppContext) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                if !self.editor.read(cx).text(cx).trim().is_empty() {
                    model.emit(PromptEditorEvent::StartRequested, cx);
                }
            }
            CodegenStatus::Pending => {
                model.emit(PromptEditorEvent::DismissRequested, cx);
            }
            CodegenStatus::Done => {
                if self.edited_since_done {
                    model.emit(PromptEditorEvent::StartRequested, cx);
                } else {
                    model.emit(PromptEditorEvent::ConfirmRequested { execute: false }, cx);
                }
            }
            CodegenStatus::Error(_) => {
                model.emit(PromptEditorEvent::StartRequested, cx);
            }
        }
    }

    fn secondary_confirm(
        &mut self,
        _: &menu::SecondaryConfirm,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        if matches!(self.codegen.read(cx).status, CodegenStatus::Done) {
            model.emit(PromptEditorEvent::ConfirmRequested { execute: true }, cx);
        }
    }

    fn move_up(&mut self, _: &MoveUp, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(ix) = self.prompt_history_ix {
            if ix > 0 {
                self.prompt_history_ix = Some(ix - 1);
                let prompt = self.prompt_history[ix - 1].as_str();
                self.editor.update(cx, |editor, model, cx| {
                    editor.set_text(prompt, model, cx);
                    editor.move_to_beginning(&Default::default(), model, cx);
                });
            }
        } else if !self.prompt_history.is_empty() {
            self.prompt_history_ix = Some(self.prompt_history.len() - 1);
            let prompt = self.prompt_history[self.prompt_history.len() - 1].as_str();
            self.editor.update(cx, |editor, model, cx| {
                editor.set_text(prompt, model, cx);
                editor.move_to_beginning(&Default::default(), model, cx);
            });
        }
    }

    fn move_down(&mut self, _: &MoveDown, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(ix) = self.prompt_history_ix {
            if ix < self.prompt_history.len() - 1 {
                self.prompt_history_ix = Some(ix + 1);
                let prompt = self.prompt_history[ix + 1].as_str();
                self.editor.update(cx, |editor, model, cx| {
                    editor.set_text(prompt, model, cx);
                    editor.move_to_end(&Default::default(), model, cx)
                });
            } else {
                self.prompt_history_ix = None;
                let prompt = self.pending_prompt.as_str();
                self.editor.update(cx, |editor, model, cx| {
                    editor.set_text(prompt, model, cx);
                    editor.move_to_end(&Default::default(), model, cx)
                });
            }
        }
    }

    fn render_token_count(
        &self,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> Option<impl IntoElement> {
        let model = LanguageModelRegistry::read_global(cx).active_model()?;
        let token_count = self.token_count?;
        let max_token_count = model.max_token_count();

        let remaining_tokens = max_token_count as isize - token_count as isize;
        let token_count_color = if remaining_tokens <= 0 {
            Color::Error
        } else if token_count as f32 / max_token_count as f32 >= 0.8 {
            Color::Warning
        } else {
            Color::Muted
        };

        let mut token_count = h_flex()
            .id("token_count")
            .gap_0p5()
            .child(
                Label::new(humanize_token_count(token_count))
                    .size(LabelSize::Small)
                    .color(token_count_color),
            )
            .child(Label::new("/").size(LabelSize::Small).color(Color::Muted))
            .child(
                Label::new(humanize_token_count(max_token_count))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        if let Some(workspace) = self.workspace.clone() {
            token_count = token_count
                .tooltip(|window, cx| {
                    Tooltip::with_meta(
                        "Tokens Used by Inline Assistant",
                        None,
                        "Click to Open Assistant Panel",
                        model,
                        cx,
                    )
                })
                .cursor_pointer()
                .on_mouse_down(gpui::MouseButton::Left, |_, cx| cx.stop_propagation())
                .on_click(move |_, cx| {
                    cx.stop_propagation();
                    workspace
                        .update(cx, |workspace, model, cx| {
                            workspace.focus_panel::<AssistantPanel>(cx)
                        })
                        .ok();
                });
        } else {
            token_count = token_count
                .cursor_default()
                .tooltip(|window, cx| Tooltip::text("Tokens Used by Inline Assistant", cx));
        }

        Some(token_count)
    }

    fn render_prompt_editor(&self, model: &Model<Self>, cx: &mut AppContext) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_size: settings.buffer_font_size.into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(settings.buffer_line_height.value()),
            ..Default::default()
        };
        EditorElement::new(
            &self.editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug)]
pub enum CodegenEvent {
    Finished,
}

impl EventEmitter<CodegenEvent> for Codegen {}

const CLEAR_INPUT: &str = "\x15";
const CARRIAGE_RETURN: &str = "\x0d";

struct TerminalTransaction {
    terminal: Model<Terminal>,
}

impl TerminalTransaction {
    pub fn start(terminal: Model<Terminal>) -> Self {
        Self { terminal }
    }

    pub fn push(&mut self, hunk: String, cx: &mut AppContext) {
        // Ensure that the assistant cannot accidentally execute commands that are streamed into the terminal
        let input = Self::sanitize_input(hunk);
        self.terminal
            .update(cx, |terminal, model, _| terminal.input(input));
    }

    pub fn undo(&self, cx: &mut AppContext) {
        self.terminal.update(cx, |terminal, model, _| {
            terminal.input(CLEAR_INPUT.to_string())
        });
    }

    pub fn complete(&self, cx: &mut AppContext) {
        self.terminal.update(cx, |terminal, model, _| {
            terminal.input(CARRIAGE_RETURN.to_string())
        });
    }

    fn sanitize_input(input: String) -> String {
        input.replace(['\r', '\n'], "")
    }
}

pub struct Codegen {
    status: CodegenStatus,
    telemetry: Option<Arc<Telemetry>>,
    terminal: Model<Terminal>,
    generation: Task<()>,
    message_id: Option<String>,
    transaction: Option<TerminalTransaction>,
}

impl Codegen {
    pub fn new(terminal: Model<Terminal>, telemetry: Option<Arc<Telemetry>>) -> Self {
        Self {
            terminal,
            telemetry,
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
            message_id: None,
            transaction: None,
        }
    }

    pub fn start(
        &mut self,
        prompt: LanguageModelRequest,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };

        let model_api_key = model.api_key(cx);
        let http_client = cx.http_client();
        let telemetry = self.telemetry.clone();
        self.status = CodegenStatus::Pending;
        self.transaction = Some(TerminalTransaction::start(self.terminal.clone()));
        self.generation = model.spawn(cx, |this, mut cx| async move {
            let model_telemetry_id = model.telemetry_id();
            let model_provider_id = model.provider_id();
            let response = model.stream_completion_text(prompt, &cx).await;
            let generate = async {
                let message_id = response
                    .as_ref()
                    .ok()
                    .and_then(|response| response.message_id.clone());

                let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);

                let task = cx.background_executor().spawn({
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
                            AssistantEvent {
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

                this.update(&mut cx, |this, _, _| {
                    this.message_id = message_id;
                })?;

                while let Some(hunk) = hunks_rx.next().await {
                    this.update(&mut cx, |this, model, cx| {
                        if let Some(transaction) = &mut this.transaction {
                            transaction.push(hunk, cx);
                            model.notify(cx);
                        }
                    })?;
                }

                task.await?;
                anyhow::Ok(())
            };

            let result = generate.await;

            this.update(&mut cx, |this, model, cx| {
                if let Err(error) = result {
                    this.status = CodegenStatus::Error(error);
                } else {
                    this.status = CodegenStatus::Done;
                }
                model.emit(CodegenEvent::Finished, cx);
                model.notify(cx);
            })
            .ok();
        });
        model.notify(cx);
    }

    pub fn stop(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        self.status = CodegenStatus::Done;
        self.generation = Task::ready(());
        model.emit(CodegenEvent::Finished, cx);
        model.notify(cx);
    }

    pub fn complete(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(transaction) = self.transaction.take() {
            transaction.complete(cx);
        }
    }

    pub fn undo(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(transaction) = self.transaction.take() {
            transaction.undo(cx);
        }
    }
}

enum CodegenStatus {
    Idle,
    Pending,
    Done,
    Error(anyhow::Error),
}
