use crate::{AssistantPanel, AssistantPanelEvent, DEFAULT_CONTEXT_LINES};
use anyhow::{Context as _, Result};
use assistant_context_editor::{RequestType, humanize_token_count};
use assistant_settings::AssistantSettings;
use client::telemetry::Telemetry;
use collections::{HashMap, VecDeque};
use editor::{
    ContextMenuOptions, Editor, EditorElement, EditorEvent, EditorMode, EditorStyle, MultiBuffer,
    actions::{MoveDown, MoveUp, SelectAll},
};
use fs::Fs;
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, Global, Subscription, Task,
    TextStyle, UpdateGlobal, WeakEntity,
};
use language::Buffer;
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage,
    Role, report_assistant_event,
};
use language_model_selector::{LanguageModelSelector, LanguageModelSelectorPopoverMenu};
use prompt_store::PromptBuilder;
use settings::{Settings, update_settings_file};
use std::{
    cmp,
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};
use terminal::Terminal;
use terminal_view::TerminalView;
use theme::ThemeSettings;
use ui::{IconButtonShape, Tooltip, prelude::*, text_for_action};
use util::ResultExt;
use workspace::{Toast, Workspace, notifications::NotificationId};

pub fn init(
    fs: Arc<dyn Fs>,
    prompt_builder: Arc<PromptBuilder>,
    telemetry: Arc<Telemetry>,
    cx: &mut App,
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
        terminal_view: &Entity<TerminalView>,
        workspace: Option<WeakEntity<Workspace>>,
        assistant_panel: Option<&Entity<AssistantPanel>>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let terminal = terminal_view.read(cx).terminal().clone();
        let assist_id = self.next_assist_id.post_inc();
        let prompt_buffer = cx.new(|cx| Buffer::local(initial_prompt.unwrap_or_default(), cx));
        let prompt_buffer = cx.new(|cx| MultiBuffer::singleton(prompt_buffer, cx));
        let codegen = cx.new(|_| Codegen::new(terminal, self.telemetry.clone()));

        let prompt_editor = cx.new(|cx| {
            PromptEditor::new(
                assist_id,
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                codegen,
                assistant_panel,
                workspace.clone(),
                self.fs.clone(),
                window,
                cx,
            )
        });
        let prompt_editor_render = prompt_editor.clone();
        let block = terminal_view::BlockProperties {
            height: 2,
            render: Box::new(move |_| prompt_editor_render.clone().into_any_element()),
        };
        terminal_view.update(cx, |terminal_view, cx| {
            terminal_view.set_block_below_cursor(block, window, cx);
        });

        let terminal_assistant = TerminalInlineAssist::new(
            assist_id,
            terminal_view,
            assistant_panel.is_some(),
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
                    window.focus(&editor.focus_handle(cx));
                    editor.select_all(&SelectAll, window, cx);
                });
            });
        }
    }

    fn handle_prompt_editor_event(
        &mut self,
        prompt_editor: Entity<PromptEditor>,
        event: &PromptEditorEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let assist_id = prompt_editor.read(cx).id;
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
            PromptEditorEvent::DismissRequested => {
                self.dismiss_assist(assist_id, window, cx);
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
        self.prompt_history.push_back(user_prompt.clone());
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        assist
            .terminal
            .update(cx, |terminal, cx| {
                terminal
                    .terminal()
                    .update(cx, |terminal, _| terminal.input(CLEAR_INPUT.to_string()));
            })
            .log_err();

        let codegen = assist.codegen.clone();
        let Some(request) = self.request_for_inline_assist(assist_id, cx).log_err() else {
            return;
        };

        codegen.update(cx, |codegen, cx| codegen.start(request, cx));
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
    ) -> Result<LanguageModelRequest> {
        let assist = self.assists.get(&assist_id).context("invalid assist")?;

        let shell = std::env::var("SHELL").ok();
        let (latest_output, working_directory) = assist
            .terminal
            .update(cx, |terminal, cx| {
                let terminal = terminal.entity().read(cx);
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
                        .to_completion_request(None, RequestType::Chat, cx),
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
            thread_id: None,
            prompt_id: None,
            mode: None,
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
        window: &mut Window,
        cx: &mut App,
    ) {
        self.dismiss_assist(assist_id, window, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            assist
                .terminal
                .update(cx, |this, cx| {
                    this.clear_block_below_cursor(cx);
                    this.focus_handle(cx).focus(window);
                })
                .log_err();

            if let Some(ConfiguredModel { model, .. }) =
                LanguageModelRegistry::read_global(cx).inline_assistant_model()
            {
                let codegen = assist.codegen.read(cx);
                let executor = cx.background_executor().clone();
                report_assistant_event(
                    AssistantEventData {
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
                this.focus_handle(cx).focus(window);
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
        if let Some(assist) = self.assists.get_mut(&assist_id) {
            if let Some(prompt_editor) = assist.prompt_editor.as_ref().cloned() {
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
}

struct TerminalInlineAssist {
    terminal: WeakEntity<TerminalView>,
    prompt_editor: Option<Entity<PromptEditor>>,
    codegen: Entity<Codegen>,
    workspace: Option<WeakEntity<Workspace>>,
    include_context: bool,
    _subscriptions: Vec<Subscription>,
}

impl TerminalInlineAssist {
    pub fn new(
        assist_id: TerminalInlineAssistId,
        terminal: &Entity<TerminalView>,
        include_context: bool,
        prompt_editor: Entity<PromptEditor>,
        workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let codegen = prompt_editor.read(cx).codegen.clone();
        Self {
            terminal: terminal.downgrade(),
            prompt_editor: Some(prompt_editor.clone()),
            codegen: codegen.clone(),
            workspace: workspace.clone(),
            include_context,
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

                            if let CodegenStatus::Error(error) = &codegen.read(cx).status {
                                if assist.prompt_editor.is_none() {
                                    if let Some(workspace) = assist
                                        .workspace
                                        .as_ref()
                                        .and_then(|workspace| workspace.upgrade())
                                    {
                                        let error =
                                            format!("Terminal inline assistant error: {}", error);
                                        workspace.update(cx, |workspace, cx| {
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
                                this.finish_assist(assist_id, false, false, window, cx);
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
    height_in_lines: u8,
    editor: Entity<Editor>,
    language_model_selector: Entity<LanguageModelSelector>,
    edited_since_done: bool,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    codegen: Entity<Codegen>,
    _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
    pending_token_count: Task<Result<()>>,
    token_count: Option<usize>,
    _token_count_subscriptions: Vec<Subscription>,
    workspace: Option<WeakEntity<Workspace>>,
}

impl EventEmitter<PromptEditorEvent> for PromptEditor {}

impl Render for PromptEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = &self.codegen.read(cx).status;
        let buttons = match status {
            CodegenStatus::Idle => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Cancel Assist", &menu::Cancel, window, cx)
                        })
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    IconButton::new("start", IconName::SparkleAlt)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Generate", &menu::Confirm, window, cx)
                        })
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::StartRequested)),
                        ),
                ]
            }
            CodegenStatus::Pending => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(Tooltip::text("Cancel Assist"))
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    IconButton::new("stop", IconName::Stop)
                        .icon_color(Color::Error)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::with_meta(
                                "Interrupt Generation",
                                Some(&menu::Cancel),
                                "Changes won't be discarded",
                                window,
                                cx,
                            )
                        })
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::StopRequested)),
                        ),
                ]
            }
            CodegenStatus::Error(_) | CodegenStatus::Done => {
                let cancel = IconButton::new("cancel", IconName::Close)
                    .icon_color(Color::Muted)
                    .shape(IconButtonShape::Square)
                    .tooltip(|window, cx| {
                        Tooltip::for_action("Cancel Assist", &menu::Cancel, window, cx)
                    })
                    .on_click(
                        cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                    );

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
                                    window,
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(PromptEditorEvent::StartRequested);
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
                                    window,
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(PromptEditorEvent::ConfirmRequested { execute: false });
                            })),
                        IconButton::new("confirm", IconName::Play)
                            .icon_color(Color::Info)
                            .shape(IconButtonShape::Square)
                            .tooltip(|window, cx| {
                                Tooltip::for_action(
                                    "Execute Generated Command",
                                    &menu::SecondaryConfirm,
                                    window,
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(PromptEditorEvent::ConfirmRequested { execute: true });
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
                    .child(LanguageModelSelectorPopoverMenu::new(
                        self.language_model_selector.clone(),
                        IconButton::new("change-model", IconName::SettingsAlt)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted),
                        move |window, cx| {
                            Tooltip::with_meta(
                                format!(
                                    "Using {}",
                                    LanguageModelRegistry::read_global(cx)
                                        .inline_assistant_model()
                                        .map(|inline_assistant| inline_assistant.model.name().0)
                                        .unwrap_or_else(|| "No model selected".into()),
                                ),
                                None,
                                "Change Model",
                                window,
                                cx,
                            )
                        },
                        gpui::Corner::TopRight,
                    ))
                    .children(
                        if let CodegenStatus::Error(error) = &self.codegen.read(cx).status {
                            let error_message = SharedString::from(error.to_string());
                            Some(
                                div()
                                    .id("error")
                                    .tooltip(Tooltip::text(error_message))
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

impl Focusable for PromptEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl PromptEditor {
    const MAX_LINES: u8 = 8;

    fn new(
        id: TerminalInlineAssistId,
        prompt_history: VecDeque<String>,
        prompt_buffer: Entity<MultiBuffer>,
        codegen: Entity<Codegen>,
        assistant_panel: Option<&Entity<AssistantPanel>>,
        workspace: Option<WeakEntity<Workspace>>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let prompt_editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    max_lines: Self::MAX_LINES as usize,
                },
                prompt_buffer,
                None,
                window,
                cx,
            );
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text(Self::placeholder_text(window, cx), cx);
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: None,
            });
            editor
        });

        let mut token_count_subscriptions = Vec::new();
        if let Some(assistant_panel) = assistant_panel {
            token_count_subscriptions.push(cx.subscribe_in(
                assistant_panel,
                window,
                Self::handle_assistant_panel_event,
            ));
        }

        let mut this = Self {
            id,
            height_in_lines: 1,
            editor: prompt_editor,
            language_model_selector: cx.new(|cx| {
                let fs = fs.clone();
                LanguageModelSelector::new(
                    |cx| LanguageModelRegistry::read_global(cx).default_model(),
                    move |model, cx| {
                        update_settings_file::<AssistantSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _| settings.set_model(model.clone()),
                        );
                    },
                    window,
                    cx,
                )
            }),
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: cx.observe_in(&codegen, window, Self::handle_codegen_changed),
            editor_subscriptions: Vec::new(),
            codegen,
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

    fn placeholder_text(window: &Window, cx: &App) -> String {
        let context_keybinding = text_for_action(&zed_actions::assistant::ToggleFocus, window, cx)
            .map(|keybinding| format!(" • {keybinding} for context"))
            .unwrap_or_default();

        format!("Generate…{context_keybinding} • ↓↑ for history")
    }

    fn subscribe_to_editor(&mut self, cx: &mut Context<Self>) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions
            .push(cx.observe(&self.editor, Self::handle_prompt_editor_changed));
        self.editor_subscriptions
            .push(cx.subscribe(&self.editor, Self::handle_prompt_editor_events));
    }

    fn prompt(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
    }

    fn count_lines(&mut self, cx: &mut Context<Self>) {
        let height_in_lines = cmp::max(
            2, // Make the editor at least two lines tall, to account for padding and buttons.
            cmp::min(
                self.editor
                    .update(cx, |editor, cx| editor.max_point(cx).row().0 + 1),
                Self::MAX_LINES as u32,
            ),
        ) as u8;

        if height_in_lines != self.height_in_lines {
            self.height_in_lines = height_in_lines;
            cx.emit(PromptEditorEvent::Resized { height_in_lines });
        }
    }

    fn handle_assistant_panel_event(
        &mut self,
        _: &Entity<AssistantPanel>,
        event: &AssistantPanelEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let AssistantPanelEvent::ContextEdited { .. } = event;
        self.count_tokens(cx);
    }

    fn count_tokens(&mut self, cx: &mut Context<Self>) {
        let assist_id = self.id;
        let Some(ConfiguredModel { model, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        else {
            return;
        };
        self.pending_token_count = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let request =
                cx.update_global(|inline_assistant: &mut TerminalInlineAssistant, cx| {
                    inline_assistant.request_for_inline_assist(assist_id, cx)
                })??;

            let token_count = cx.update(|cx| model.count_tokens(request, cx))?.await?;
            this.update(cx, |this, cx| {
                this.token_count = Some(token_count);
                cx.notify();
            })
        })
    }

    fn handle_prompt_editor_changed(&mut self, _: Entity<Editor>, cx: &mut Context<Self>) {
        self.count_lines(cx);
    }

    fn handle_prompt_editor_events(
        &mut self,
        _: Entity<Editor>,
        event: &EditorEvent,
        cx: &mut Context<Self>,
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
                cx.notify();
            }
            EditorEvent::BufferEdited => {
                self.count_tokens(cx);
            }
            _ => {}
        }
    }

    fn handle_codegen_changed(
        &mut self,
        _: Entity<Codegen>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Pending => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(true));
            }
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
        }
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle | CodegenStatus::Done | CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::CancelRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::StopRequested);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                if !self.editor.read(cx).text(cx).trim().is_empty() {
                    cx.emit(PromptEditorEvent::StartRequested);
                }
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::DismissRequested);
            }
            CodegenStatus::Done => {
                if self.edited_since_done {
                    cx.emit(PromptEditorEvent::StartRequested);
                } else {
                    cx.emit(PromptEditorEvent::ConfirmRequested { execute: false });
                }
            }
            CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::StartRequested);
            }
        }
    }

    fn secondary_confirm(
        &mut self,
        _: &menu::SecondaryConfirm,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.codegen.read(cx).status, CodegenStatus::Done) {
            cx.emit(PromptEditorEvent::ConfirmRequested { execute: true });
        }
    }

    fn move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix > 0 {
                self.prompt_history_ix = Some(ix - 1);
                let prompt = self.prompt_history[ix - 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, window, cx);
                    editor.move_to_beginning(&Default::default(), window, cx);
                });
            }
        } else if !self.prompt_history.is_empty() {
            self.prompt_history_ix = Some(self.prompt_history.len() - 1);
            let prompt = self.prompt_history[self.prompt_history.len() - 1].as_str();
            self.editor.update(cx, |editor, cx| {
                editor.set_text(prompt, window, cx);
                editor.move_to_beginning(&Default::default(), window, cx);
            });
        }
    }

    fn move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix < self.prompt_history.len() - 1 {
                self.prompt_history_ix = Some(ix + 1);
                let prompt = self.prompt_history[ix + 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, window, cx);
                    editor.move_to_end(&Default::default(), window, cx)
                });
            } else {
                self.prompt_history_ix = None;
                let prompt = self.pending_prompt.as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, window, cx);
                    editor.move_to_end(&Default::default(), window, cx)
                });
            }
        }
    }

    fn render_token_count(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let model = LanguageModelRegistry::read_global(cx)
            .inline_assistant_model()?
            .model;
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
                        window,
                        cx,
                    )
                })
                .cursor_pointer()
                .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_click(move |_, window, cx| {
                    cx.stop_propagation();
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.focus_panel::<AssistantPanel>(window, cx)
                        })
                        .ok();
                });
        } else {
            token_count = token_count
                .cursor_default()
                .tooltip(Tooltip::text("Tokens Used by Inline Assistant"));
        }

        Some(token_count)
    }

    fn render_prompt_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_size: settings.buffer_font_size(cx).into(),
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

#[cfg(not(target_os = "windows"))]
const CLEAR_INPUT: &str = "\x15";
#[cfg(target_os = "windows")]
const CLEAR_INPUT: &str = "\x03";
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

pub struct Codegen {
    status: CodegenStatus,
    telemetry: Option<Arc<Telemetry>>,
    terminal: Entity<Terminal>,
    generation: Task<()>,
    message_id: Option<String>,
    transaction: Option<TerminalTransaction>,
}

impl Codegen {
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

    pub fn start(&mut self, prompt: LanguageModelRequest, cx: &mut Context<Self>) {
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

enum CodegenStatus {
    Idle,
    Pending,
    Done,
    Error(anyhow::Error),
}
