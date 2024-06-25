use anyhow::{Context as _, Result};
use client::telemetry::Telemetry;
use collections::{HashMap, VecDeque};
use editor::actions::MoveDown;
use editor::actions::MoveUp;
use editor::actions::SelectAll;
use editor::Editor;
use editor::EditorElement;
use editor::EditorEvent;
use editor::EditorMode;
use editor::EditorStyle;
use editor::MultiBuffer;
use fs::Fs;
use futures::channel::mpsc;
use futures::SinkExt;
use futures::StreamExt;
use gpui::Context;
use gpui::ModelContext;
use gpui::{
    AppContext, EventEmitter, FocusHandle, FocusableView, FontStyle, FontWeight, Global, Model,
    Subscription, Task, TextStyle, UpdateGlobal, View, WeakView, WhiteSpace,
};
use language::Buffer;
use settings::update_settings_file;
use settings::Settings;
use std::cmp;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use terminal::Terminal;
use terminal_view::TerminalView;
use theme::ThemeSettings;
use ui::prelude::*;
use ui::ContextMenu;
use ui::PopoverMenu;
use ui::Tooltip;
use util::ResultExt;
use workspace::Workspace;

use crate::assistant_settings::AssistantSettings;
use crate::humanize_token_count;
use crate::AssistantPanel;
use crate::AssistantPanelEvent;
use crate::CompletionProvider;
use crate::LanguageModelRequest;
use crate::LanguageModelRequestMessage;
use crate::Role;

pub fn init(fs: Arc<dyn Fs>, telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    cx.set_global(TerminalInlineAssistant::new(fs, telemetry));
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
    assists_by_editor: HashMap<WeakView<TerminalView>, TerminalInlineAssistId>,
    prompt_history: VecDeque<String>,
    telemetry: Option<Arc<Telemetry>>,
    fs: Arc<dyn Fs>,
}

impl Global for TerminalInlineAssistant {}

impl TerminalInlineAssistant {
    pub fn new(fs: Arc<dyn Fs>, telemetry: Arc<Telemetry>) -> Self {
        Self {
            next_assist_id: TerminalInlineAssistId::default(),
            assists: HashMap::default(),
            assists_by_editor: HashMap::default(),
            prompt_history: VecDeque::default(),
            telemetry: Some(telemetry),
            fs,
        }
    }

    pub fn assist(
        &mut self,
        terminal_view: &View<TerminalView>,
        workspace: Option<WeakView<Workspace>>,
        assistant_panel: Option<&View<AssistantPanel>>,
        cx: &mut WindowContext,
    ) {
        let terminal = terminal_view.read(cx).terminal().clone();
        let assist_id = self.next_assist_id.post_inc();
        let prompt_buffer = cx.new_model(|cx| Buffer::local("", cx));
        let prompt_buffer = cx.new_model(|cx| MultiBuffer::singleton(prompt_buffer, cx));
        let codegen = cx.new_model(|_| Codegen::new(terminal, self.telemetry.clone()));

        let prompt_editor = cx.new_view(|cx| {
            PromptEditor::new(
                assist_id,
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                codegen,
                assistant_panel,
                workspace.clone(),
                self.fs.clone(),
                cx,
            )
        });
        let prompt_editor_render = prompt_editor.clone();
        let block = terminal_view::BlockProperties {
            height: 1,
            render: Box::new(move |_| prompt_editor_render.clone().into_any_element()),
        };
        terminal_view.update(cx, |terminal_view, cx| {
            terminal_view.set_prompt(block, cx);
        });

        let terminal_assistant = TerminalInlineAssist::new(
            assist_id,
            terminal_view,
            prompt_editor,
            workspace.clone(),
            cx,
        );

        self.assists.insert(assist_id, terminal_assistant);

        self.focus_assist(assist_id, cx);
    }

    fn focus_assist(&mut self, assist_id: TerminalInlineAssistId, cx: &mut WindowContext) {
        let assist = &self.assists[&assist_id];
        assist.prompt_editor.update(cx, |this, cx| {
            this.editor.update(cx, |editor, cx| {
                editor.focus(cx);
                editor.select_all(&SelectAll, cx);
            });
        })
    }

    fn handle_prompt_editor_event(
        &mut self,
        prompt_editor: View<PromptEditor>,
        event: &PromptEditorEvent,
        cx: &mut WindowContext,
    ) {
        let assist_id = prompt_editor.read(cx).id;
        match event {
            PromptEditorEvent::StartRequested => {
                self.start_assist(assist_id, cx);
            }
            PromptEditorEvent::StopRequested => {
                self.stop_assist(assist_id, cx);
            }
            PromptEditorEvent::ConfirmRequested => {
                self.finish_assist(assist_id, false, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, cx);
            }
            PromptEditorEvent::DismissRequested => {
                self.dismiss_assist(assist_id, cx);
            }
            PromptEditorEvent::Resized { height_in_lines } => {
                self.insert_prompt_editor_into_terminal(assist_id, *height_in_lines, cx);
            }
        }
    }

    fn start_assist(&mut self, assist_id: TerminalInlineAssistId, cx: &mut WindowContext) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        let user_prompt = assist.prompt_editor.read(cx).prompt(cx);

        self.prompt_history.retain(|prompt| *prompt != user_prompt);
        self.prompt_history.push_back(user_prompt.clone());
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        assist.codegen.update(cx, |codegen, cx| codegen.undo(cx));
        let codegen = assist.codegen.clone();
        let Some(request) = self.request_for_inline_assist(assist_id, cx).log_err() else {
            return;
        };

        codegen.update(cx, |codegen, cx| codegen.start(request, cx));
    }

    fn stop_assist(&mut self, assist_id: TerminalInlineAssistId, cx: &mut WindowContext) {
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
        cx: &mut WindowContext,
    ) -> Result<LanguageModelRequest> {
        let assist = self.assists.get(&assist_id).context("invalid assist")?;

        let model = CompletionProvider::global(cx).model();
        let messages = vec![LanguageModelRequestMessage {
            role: Role::User,
            content: assist.prompt_editor.read(cx).prompt(cx),
        }];

        Ok(LanguageModelRequest {
            model,
            messages,
            stop: Vec::new(),
            temperature: 1.0,
        })
    }

    fn finish_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        undo: bool,
        cx: &mut WindowContext,
    ) {
        self.dismiss_assist(assist_id, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            assist
                .terminal
                .update(cx, |this, cx| {
                    this.clear_prompt(cx);
                    this.focus_handle(cx).focus(cx);
                })
                .log_err();

            // if undo {
            //     assist.codegen.update(cx, |codegen, cx| codegen.undo(cx));
            // }
        }
    }

    fn dismiss_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        cx: &mut WindowContext,
    ) -> bool {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return false;
        };
        let Some(terminal) = assist.terminal.upgrade() else {
            return false;
        };

        true
    }

    fn insert_prompt_editor_into_terminal(
        &mut self,
        assist_id: TerminalInlineAssistId,
        height: u8,
        cx: &mut WindowContext,
    ) {
        if let Some(assist) = self.assists.get_mut(&assist_id) {
            let prompt_editor = assist.prompt_editor.clone();
            assist
                .terminal
                .update(cx, |terminal, cx| {
                    terminal.clear_prompt(cx);
                    let block = terminal_view::BlockProperties {
                        height,
                        render: Box::new(move |_| prompt_editor.clone().into_any_element()),
                    };
                    terminal.set_prompt(block, cx);
                })
                .log_err();
        }
    }
}

struct TerminalInlineAssist {
    terminal: WeakView<TerminalView>,
    prompt_editor: View<PromptEditor>,
    codegen: Model<Codegen>,
    workspace: Option<WeakView<Workspace>>,
    _subscriptions: Vec<Subscription>,
}

impl TerminalInlineAssist {
    pub fn new(
        assist_id: TerminalInlineAssistId,
        terminal: &View<TerminalView>,
        prompt_editor: View<PromptEditor>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Self {
        Self {
            terminal: terminal.downgrade(),
            prompt_editor: prompt_editor.clone(),
            codegen: prompt_editor.read(cx).codegen.clone(),
            workspace: workspace.clone(),
            _subscriptions: vec![cx.subscribe(&prompt_editor, |prompt_editor, event, cx| {
                TerminalInlineAssistant::update_global(cx, |this, cx| {
                    this.handle_prompt_editor_event(prompt_editor, event, cx)
                })
            })],
        }
    }
}

enum PromptEditorEvent {
    StartRequested,
    StopRequested,
    ConfirmRequested,
    CancelRequested,
    DismissRequested,
    Resized { height_in_lines: u8 },
}

struct PromptEditor {
    id: TerminalInlineAssistId,
    fs: Arc<dyn Fs>,
    height_in_lines: u8,
    editor: View<Editor>,
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
    workspace: Option<WeakView<Workspace>>,
}

impl EventEmitter<PromptEditorEvent> for PromptEditor {}

impl Render for PromptEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let fs = self.fs.clone();

        let buttons = match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    IconButton::new("start", IconName::Sparkle)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .icon_size(IconSize::XSmall)
                        .tooltip(|cx| Tooltip::for_action("Generate", &menu::Confirm, cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StartRequested)),
                        ),
                ]
            }
            CodegenStatus::Pending => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::text("Cancel Assist", cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    IconButton::new("stop", IconName::Stop)
                        .icon_color(Color::Error)
                        .size(ButtonSize::None)
                        .icon_size(IconSize::XSmall)
                        .tooltip(|cx| {
                            Tooltip::with_meta(
                                "Interrupt Generation",
                                Some(&menu::Cancel),
                                "Changes won't be discarded",
                                cx,
                            )
                        })
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StopRequested)),
                        ),
                ]
            }
            CodegenStatus::Error(_) | CodegenStatus::Done => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    if self.edited_since_done {
                        IconButton::new("restart", IconName::RotateCw)
                            .icon_color(Color::Info)
                            .icon_size(IconSize::XSmall)
                            .size(ButtonSize::None)
                            .tooltip(|cx| {
                                Tooltip::with_meta(
                                    "Restart Generation",
                                    Some(&menu::Confirm),
                                    "Changes will be discarded",
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|_, _, cx| {
                                cx.emit(PromptEditorEvent::StartRequested);
                            }))
                    } else {
                        IconButton::new("confirm", IconName::Play)
                            .icon_color(Color::Info)
                            .size(ButtonSize::None)
                            .tooltip(|cx| {
                                Tooltip::for_action("Execute generated command", &menu::Confirm, cx)
                            })
                            .on_click(cx.listener(|_, _, cx| {
                                cx.emit(PromptEditorEvent::ConfirmRequested);
                            }))
                    },
                ]
            }
        };

        h_flex()
            .bg(cx.theme().colors().editor_background)
            .border_y_1()
            .border_color(cx.theme().status().info_border)
            .py_1p5()
            .h_full()
            .w_full()
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .child(
                h_flex()
                    .w_10()
                    .justify_center()
                    .gap_2()
                    .child(
                        PopoverMenu::new("model-switcher")
                            .menu(move |cx| {
                                ContextMenu::build(cx, |mut menu, cx| {
                                    for model in CompletionProvider::global(cx).available_models() {
                                        menu = menu.custom_entry(
                                            {
                                                let model = model.clone();
                                                move |_| {
                                                    Label::new(model.display_name())
                                                        .into_any_element()
                                                }
                                            },
                                            {
                                                let fs = fs.clone();
                                                let model = model.clone();
                                                move |cx| {
                                                    let model = model.clone();
                                                    update_settings_file::<AssistantSettings>(
                                                        fs.clone(),
                                                        cx,
                                                        move |settings| settings.set_model(model),
                                                    );
                                                }
                                            },
                                        );
                                    }
                                    menu
                                })
                                .into()
                            })
                            .trigger(
                                IconButton::new("context", IconName::Settings)
                                    .size(ButtonSize::None)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .tooltip(move |cx| {
                                        Tooltip::with_meta(
                                            format!(
                                                "Using {}",
                                                CompletionProvider::global(cx)
                                                    .model()
                                                    .display_name()
                                            ),
                                            None,
                                            "Click to Change Model",
                                            cx,
                                        )
                                    }),
                            )
                            .anchor(gpui::AnchorCorner::BottomRight),
                    )
                    .children(
                        if let CodegenStatus::Error(error) = &self.codegen.read(cx).status {
                            let error_message = SharedString::from(error.to_string());
                            Some(
                                div()
                                    .id("error")
                                    .tooltip(move |cx| Tooltip::text(error_message.clone(), cx))
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
                    .gap_2()
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
        assistant_panel: Option<&View<AssistantPanel>>,
        workspace: Option<WeakView<Workspace>>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let prompt_editor = cx.new_view(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    max_lines: Self::MAX_LINES as usize,
                },
                prompt_buffer,
                None,
                false,
                cx,
            );
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text("Add a prompt…", cx);
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

    fn subscribe_to_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions
            .push(cx.observe(&self.editor, Self::handle_prompt_editor_changed));
        self.editor_subscriptions
            .push(cx.subscribe(&self.editor, Self::handle_prompt_editor_events));
    }

    fn prompt(&self, cx: &AppContext) -> String {
        self.editor.read(cx).text(cx)
    }

    fn count_lines(&mut self, cx: &mut ViewContext<Self>) {
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
        _: View<AssistantPanel>,
        event: &AssistantPanelEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let AssistantPanelEvent::ContextEdited { .. } = event;
        self.count_tokens(cx);
    }

    fn count_tokens(&mut self, cx: &mut ViewContext<Self>) {
        let assist_id = self.id;
        self.pending_token_count = cx.spawn(|this, mut cx| async move {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let request =
                cx.update_global(|inline_assistant: &mut TerminalInlineAssistant, cx| {
                    inline_assistant.request_for_inline_assist(assist_id, cx)
                })??;

            let token_count = cx
                .update(|cx| CompletionProvider::global(cx).count_tokens(request, cx))?
                .await?;
            this.update(&mut cx, |this, cx| {
                this.token_count = Some(token_count);
                cx.notify();
            })
        })
    }

    fn handle_prompt_editor_changed(&mut self, _: View<Editor>, cx: &mut ViewContext<Self>) {
        self.count_lines(cx);
    }

    fn handle_prompt_editor_events(
        &mut self,
        _: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
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

    fn handle_codegen_changed(&mut self, _: Model<Codegen>, cx: &mut ViewContext<Self>) {
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

    fn cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle | CodegenStatus::Done | CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::CancelRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::StopRequested);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                cx.emit(PromptEditorEvent::StartRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::DismissRequested);
            }
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                if self.edited_since_done {
                    cx.emit(PromptEditorEvent::StartRequested);
                } else {
                    cx.emit(PromptEditorEvent::ConfirmRequested);
                }
            }
        }
    }

    fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix > 0 {
                self.prompt_history_ix = Some(ix - 1);
                let prompt = self.prompt_history[ix - 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, cx);
                    editor.move_to_beginning(&Default::default(), cx);
                });
            }
        } else if !self.prompt_history.is_empty() {
            self.prompt_history_ix = Some(self.prompt_history.len() - 1);
            let prompt = self.prompt_history[self.prompt_history.len() - 1].as_str();
            self.editor.update(cx, |editor, cx| {
                editor.set_text(prompt, cx);
                editor.move_to_beginning(&Default::default(), cx);
            });
        }
    }

    fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix < self.prompt_history.len() - 1 {
                self.prompt_history_ix = Some(ix + 1);
                let prompt = self.prompt_history[ix + 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, cx);
                    editor.move_to_end(&Default::default(), cx)
                });
            } else {
                self.prompt_history_ix = None;
                let prompt = self.pending_prompt.as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, cx);
                    editor.move_to_end(&Default::default(), cx)
                });
            }
        }
    }

    fn render_token_count(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let model = CompletionProvider::global(cx).model();
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
                .tooltip(|cx| {
                    Tooltip::with_meta(
                        "Tokens Used by Inline Assistant",
                        None,
                        "Click to Open Assistant Panel",
                        cx,
                    )
                })
                .cursor_pointer()
                .on_mouse_down(gpui::MouseButton::Left, |_, cx| cx.stop_propagation())
                .on_click(move |_, cx| {
                    cx.stop_propagation();
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.focus_panel::<AssistantPanel>(cx)
                        })
                        .ok();
                });
        } else {
            token_count = token_count
                .cursor_default()
                .tooltip(|cx| Tooltip::text("Tokens Used by Inline Assistant", cx));
        }

        Some(token_count)
    }

    fn render_prompt_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
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
    Undone,
}

impl EventEmitter<CodegenEvent> for Codegen {}

pub struct Codegen {
    status: CodegenStatus,
    telemetry: Option<Arc<Telemetry>>,
    terminal: Model<Terminal>,
    generation: Task<()>,
}

impl Codegen {
    pub fn new(terminal: Model<Terminal>, telemetry: Option<Arc<Telemetry>>) -> Self {
        Self {
            terminal,
            telemetry,
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
        }
    }

    pub fn start(&mut self, prompt: LanguageModelRequest, cx: &mut ModelContext<Self>) {
        self.status = CodegenStatus::Pending;

        let telemetry = self.telemetry.clone();
        let model_telemetry_id = prompt.model.telemetry_id();
        let response = CompletionProvider::global(cx).complete(prompt);

        self.generation = cx.spawn(|this, mut cx| async move {
            let generate = async {
                let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);

                let task = cx.background_executor().spawn(async move {
                    let mut response_latency = None;
                    let request_start = Instant::now();
                    let task = async {
                        let mut response = response.await?;
                        while let Some(chunk) = response.next().await {
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
                    if let Some(telemetry) = telemetry {
                        telemetry.report_assistant_event(
                            None,
                            telemetry_events::AssistantKind::Inline,
                            model_telemetry_id,
                            response_latency,
                            error_message,
                        );
                    }

                    result?;
                    anyhow::Ok(())
                });

                while let Some(hunk) = hunks_rx.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.terminal.update(cx, |terminal, _| terminal.input(hunk));
                        cx.notify();
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

    pub fn stop(&mut self, cx: &mut ModelContext<Self>) {
        self.status = CodegenStatus::Done;
        self.generation = Task::ready(());
        cx.emit(CodegenEvent::Finished);
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        //TODO
        // if let Some(transaction_id) = self.transaction_id.take() {
        //     self.buffer
        //         .update(cx, |buffer, cx| buffer.undo_transaction(transaction_id, cx));
        // }
    }
}

enum CodegenStatus {
    Idle,
    Pending,
    Done,
    Error(anyhow::Error),
}
