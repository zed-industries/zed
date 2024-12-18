use crate::context::attach_context_to_message;
use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;
use crate::context_strip::ContextStrip;
use crate::inline_prompt_editor::{CodegenStatus, PromptEditorEvent, PromptMode};
use crate::prompts::PromptBuilder;
use crate::thread_store::ThreadStore;
use crate::ToggleContextPicker;
use crate::{assistant_settings::AssistantSettings, inline_prompt_editor::render_cancel_button};
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
    AppContext, Context, EventEmitter, FocusHandle, FocusableView, Global, Model, ModelContext,
    Subscription, Task, TextStyle, UpdateGlobal, View, WeakModel, WeakView,
};
use language::Buffer;
use language_model::{
    LanguageModelRegistry, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use language_model_selector::{LanguageModelSelector, LanguageModelSelectorPopoverMenu};
use language_models::report_assistant_event;
use settings::{update_settings_file, Settings};
use std::{cmp, sync::Arc, time::Instant};
use telemetry_events::{AssistantEvent, AssistantKind, AssistantPhase};
use terminal::Terminal;
use terminal_view::TerminalView;
use theme::ThemeSettings;
use ui::{prelude::*, text_for_action, IconButtonShape, PopoverMenuHandle, Tooltip};
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

const DEFAULT_CONTEXT_LINES: usize = 50;
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
        terminal_view: &View<TerminalView>,
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        cx: &mut WindowContext,
    ) {
        let terminal = terminal_view.read(cx).terminal().clone();
        let assist_id = self.next_assist_id.post_inc();
        let prompt_buffer = cx.new_model(|cx| {
            MultiBuffer::singleton(cx.new_model(|cx| Buffer::local(String::new(), cx)), cx)
        });
        let context_store = cx.new_model(|_cx| ContextStore::new());
        let codegen = cx.new_model(|_| Codegen::new(terminal, self.telemetry.clone()));

        let prompt_editor = cx.new_view(|cx| {
            PromptEditor::new(
                assist_id,
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                codegen,
                self.fs.clone(),
                context_store.clone(),
                workspace.clone(),
                thread_store.clone(),
                cx,
            )
        });
        let prompt_editor_render = prompt_editor.clone();
        let block = terminal_view::BlockProperties {
            height: 2,
            render: Box::new(move |_| prompt_editor_render.clone().into_any_element()),
        };
        terminal_view.update(cx, |terminal_view, cx| {
            terminal_view.set_block_below_cursor(block, cx);
        });

        let terminal_assistant = TerminalInlineAssist::new(
            assist_id,
            terminal_view,
            prompt_editor,
            workspace.clone(),
            context_store,
            cx,
        );

        self.assists.insert(assist_id, terminal_assistant);

        self.focus_assist(assist_id, cx);
    }

    fn focus_assist(&mut self, assist_id: TerminalInlineAssistId, cx: &mut WindowContext) {
        let assist = &self.assists[&assist_id];
        if let Some(prompt_editor) = assist.prompt_editor.as_ref() {
            prompt_editor.update(cx, |this, cx| {
                this.editor.update(cx, |editor, cx| {
                    editor.focus(cx);
                    editor.select_all(&SelectAll, cx);
                });
            });
        }
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
            PromptEditorEvent::ConfirmRequested { execute } => {
                self.finish_assist(assist_id, false, *execute, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, false, cx);
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

        let shell = std::env::var("SHELL").ok();
        let (latest_output, working_directory) = assist
            .terminal
            .update(cx, |terminal, cx| {
                let terminal = terminal.model().read(cx);
                let latest_output = terminal.last_n_non_empty_lines(DEFAULT_CONTEXT_LINES);
                let working_directory = terminal
                    .working_directory()
                    .map(|path| path.to_string_lossy().to_string());
                (latest_output, working_directory)
            })
            .ok()
            .unwrap_or_default();

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

        let mut request_message = LanguageModelRequestMessage {
            role: Role::User,
            content: vec![],
            cache: false,
        };

        let context = assist
            .context_store
            .update(cx, |this, _cx| this.context().clone());
        attach_context_to_message(&mut request_message, context);

        request_message.content.push(prompt.into());

        Ok(LanguageModelRequest {
            messages: vec![request_message],
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
        cx: &mut WindowContext,
    ) {
        self.dismiss_assist(assist_id, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            assist
                .terminal
                .update(cx, |this, cx| {
                    this.clear_block_below_cursor(cx);
                    this.focus_handle(cx).focus(cx);
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
        cx: &mut WindowContext,
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
                this.focus_handle(cx).focus(cx);
            })
            .is_ok()
    }

    fn insert_prompt_editor_into_terminal(
        &mut self,
        assist_id: TerminalInlineAssistId,
        height: u8,
        cx: &mut WindowContext,
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
                        terminal.set_block_below_cursor(block, cx);
                    })
                    .log_err();
            }
        }
    }
}

struct TerminalInlineAssist {
    terminal: WeakView<TerminalView>,
    prompt_editor: Option<View<PromptEditor>>,
    codegen: Model<Codegen>,
    workspace: WeakView<Workspace>,
    context_store: Model<ContextStore>,
    _subscriptions: Vec<Subscription>,
}

impl TerminalInlineAssist {
    pub fn new(
        assist_id: TerminalInlineAssistId,
        terminal: &View<TerminalView>,
        prompt_editor: View<PromptEditor>,
        workspace: WeakView<Workspace>,
        context_store: Model<ContextStore>,
        cx: &mut WindowContext,
    ) -> Self {
        let codegen = prompt_editor.read(cx).codegen.clone();
        Self {
            terminal: terminal.downgrade(),
            prompt_editor: Some(prompt_editor.clone()),
            codegen: codegen.clone(),
            workspace: workspace.clone(),
            context_store,
            _subscriptions: vec![
                cx.subscribe(&prompt_editor, |prompt_editor, event, cx| {
                    TerminalInlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_event(prompt_editor, event, cx)
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
                                    if let Some(workspace) = assist.workspace.upgrade() {
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
                                this.finish_assist(assist_id, false, false, cx);
                            }
                        }
                    })
                }),
            ],
        }
    }
}

struct PromptEditor {
    id: TerminalInlineAssistId,
    height_in_lines: u8,
    editor: View<Editor>,
    context_strip: View<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    language_model_selector: View<LanguageModelSelector>,
    edited_since_done: bool,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    codegen: Model<Codegen>,
    _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
}

impl EventEmitter<PromptEditorEvent> for PromptEditor {}

impl Render for PromptEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut buttons = Vec::new();

        buttons.extend(render_cancel_button(
            (&self.codegen.read(cx).status).into(),
            self.edited_since_done,
            PromptMode::Generate {
                supports_execute: true,
            },
            cx,
        ));

        v_flex()
            .border_y_1()
            .border_color(cx.theme().status().info_border)
            .py_2()
            .size_full()
            .child(
                h_flex()
                    .key_context("PromptEditor")
                    .bg(cx.theme().colors().editor_background)
                    .on_action(cx.listener(Self::toggle_context_picker))
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
                                IconButton::new("context", IconName::SettingsAlt)
                                    .shape(IconButtonShape::Square)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .tooltip(move |cx| {
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
                                            .tooltip(move |cx| {
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
                    .child(h_flex().gap_1().pr_4().children(buttons)),
            )
            .child(h_flex().child(self.context_strip.clone()))
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
        fs: Arc<dyn Fs>,
        context_store: Model<ContextStore>,
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
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
            editor.set_placeholder_text(Self::placeholder_text(cx), cx);
            editor
        });
        let context_picker_menu_handle = PopoverMenuHandle::default();

        let mut this = Self {
            id,
            height_in_lines: 1,
            editor: prompt_editor.clone(),
            context_strip: cx.new_view(|cx| {
                ContextStrip::new(
                    context_store,
                    workspace.clone(),
                    thread_store.clone(),
                    prompt_editor.focus_handle(cx),
                    context_picker_menu_handle.clone(),
                    cx,
                )
            }),
            context_picker_menu_handle,
            language_model_selector: cx.new_view(|cx| {
                let fs = fs.clone();
                LanguageModelSelector::new(
                    move |model, cx| {
                        update_settings_file::<AssistantSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _| settings.set_model(model.clone()),
                        );
                    },
                    cx,
                )
            }),
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: cx.observe(&codegen, Self::handle_codegen_changed),
            editor_subscriptions: Vec::new(),
            codegen,
        };
        this.count_lines(cx);
        this.subscribe_to_editor(cx);
        this
    }

    fn placeholder_text(cx: &WindowContext) -> String {
        let context_keybinding = text_for_action(&crate::ToggleFocus, cx)
            .map(|keybinding| format!(" • {keybinding} for context"))
            .unwrap_or_default();

        format!("Generate…{context_keybinding} ↓↑ for history")
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

    fn toggle_context_picker(&mut self, _: &ToggleContextPicker, cx: &mut ViewContext<Self>) {
        self.context_picker_menu_handle.toggle(cx);
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

    fn secondary_confirm(&mut self, _: &menu::SecondaryConfirm, cx: &mut ViewContext<Self>) {
        if matches!(self.codegen.read(cx).status, CodegenStatus::Done) {
            cx.emit(PromptEditorEvent::ConfirmRequested { execute: true });
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

    fn render_prompt_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
            .update(cx, |terminal, _| terminal.input(input));
    }

    pub fn undo(&self, cx: &mut AppContext) {
        self.terminal
            .update(cx, |terminal, _| terminal.input(CLEAR_INPUT.to_string()));
    }

    pub fn complete(&self, cx: &mut AppContext) {
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

    pub fn start(&mut self, prompt: LanguageModelRequest, cx: &mut ModelContext<Self>) {
        let Some(model) = LanguageModelRegistry::read_global(cx).active_model() else {
            return;
        };

        let model_api_key = model.api_key(cx);
        let http_client = cx.http_client();
        let telemetry = self.telemetry.clone();
        self.status = CodegenStatus::Pending;
        self.transaction = Some(TerminalTransaction::start(self.terminal.clone()));
        self.generation = cx.spawn(|this, mut cx| async move {
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

    pub fn stop(&mut self, cx: &mut ModelContext<Self>) {
        self.status = CodegenStatus::Done;
        self.generation = Task::ready(());
        cx.emit(CodegenEvent::Finished);
        cx.notify();
    }

    pub fn complete(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(transaction) = self.transaction.take() {
            transaction.complete(cx);
        }
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(transaction) = self.transaction.take() {
            transaction.undo(cx);
        }
    }
}
