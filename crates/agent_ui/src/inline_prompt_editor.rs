use agent::ThreadStore;
use collections::{HashMap, VecDeque};
use editor::actions::Paste;
use editor::code_context_menus::CodeContextMenu;
use editor::display_map::{CreaseId, EditorMargins};
use editor::{AnchorRangeExt as _, MultiBufferOffset, ToOffset as _};
use editor::{
    ContextMenuOptions, Editor, EditorElement, EditorEvent, EditorMode, EditorStyle, MultiBuffer,
    actions::{MoveDown, MoveUp},
};
use fs::Fs;
use gpui::{
    AnyElement, App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, TextStyle, TextStyleRefinement, WeakEntity, Window, actions,
};
use language_model::{LanguageModel, LanguageModelRegistry};
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use parking_lot::Mutex;
use project::Project;
use prompt_store::PromptStore;
use settings::Settings;
use std::cell::RefCell;
use std::cmp;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::utils::WithRemSize;
use ui::{IconButtonShape, KeyBinding, PopoverMenuHandle, Tooltip, prelude::*};
use uuid::Uuid;
use workspace::notifications::NotificationId;
use workspace::{Toast, Workspace};
use zed_actions::agent::ToggleModelSelector;

use crate::agent_model_selector::AgentModelSelector;
use crate::buffer_codegen::{BufferCodegen, CodegenAlternative};
use crate::completion_provider::{
    PromptCompletionProvider, PromptCompletionProviderDelegate, PromptContextType,
};
use crate::mention_set::paste_images_as_context;
use crate::mention_set::{MentionSet, crease_for_mention};
use crate::terminal_codegen::TerminalCodegen;
use crate::{
    CycleFavoriteModels, CycleNextInlineAssist, CyclePreviousInlineAssist, ModelUsageContext,
};

actions!(inline_assistant, [ThumbsUpResult, ThumbsDownResult]);

enum CompletionState {
    Pending,
    Generated { completion_text: Option<String> },
    Rated,
}

struct SessionState {
    session_id: Uuid,
    completion: CompletionState,
}

pub struct PromptEditor<T> {
    pub editor: Entity<Editor>,
    mode: PromptEditorMode,
    mention_set: Entity<MentionSet>,
    thread_store: Entity<ThreadStore>,
    prompt_store: Option<Entity<PromptStore>>,
    workspace: WeakEntity<Workspace>,
    model_selector: Entity<AgentModelSelector>,
    edited_since_done: bool,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
    show_rate_limit_notice: bool,
    session_state: SessionState,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static> EventEmitter<PromptEditorEvent> for PromptEditor<T> {}

impl<T: 'static> Render for PromptEditor<T> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let mut buttons = Vec::new();

        const RIGHT_PADDING: Pixels = px(9.);

        let (left_gutter_width, right_padding, explanation) = match &self.mode {
            PromptEditorMode::Buffer {
                id: _,
                codegen,
                editor_margins,
            } => {
                let codegen = codegen.read(cx);

                if codegen.alternative_count(cx) > 1 {
                    buttons.push(self.render_cycle_controls(codegen, cx));
                }

                let editor_margins = editor_margins.lock();
                let gutter = editor_margins.gutter;

                let left_gutter_width = gutter.full_width() + (gutter.margin / 2.0);
                let right_padding = editor_margins.right + RIGHT_PADDING;

                let active_alternative = codegen.active_alternative().read(cx);
                let explanation = active_alternative
                    .description
                    .clone()
                    .or_else(|| active_alternative.failure.clone());

                (left_gutter_width, right_padding, explanation)
            }
            PromptEditorMode::Terminal { .. } => {
                // Give the equivalent of the same left-padding that we're using on the right
                (Pixels::from(40.0), Pixels::from(24.), None)
            }
        };

        let bottom_padding = match &self.mode {
            PromptEditorMode::Buffer { .. } => rems_from_px(2.0),
            PromptEditorMode::Terminal { .. } => rems_from_px(4.0),
        };

        buttons.extend(self.render_buttons(window, cx));

        let menu_visible = self.is_completions_menu_visible(cx);
        let add_context_button = IconButton::new("add-context", IconName::AtSign)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .when(!menu_visible, |this| {
                this.tooltip(move |_window, cx| {
                    Tooltip::with_meta("Add Context", None, "Or type @ to include context", cx)
                })
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.trigger_completion_menu(window, cx);
            }));

        let markdown = window.use_state(cx, |_, cx| Markdown::new("".into(), None, None, cx));

        if let Some(explanation) = &explanation {
            markdown.update(cx, |markdown, cx| {
                markdown.reset(SharedString::from(explanation), cx);
            });
        }

        let explanation_label = self
            .render_markdown(markdown, markdown_style(window, cx))
            .into_any_element();

        v_flex()
            .key_context("InlineAssistant")
            .capture_action(cx.listener(Self::paste))
            .block_mouse_except_scroll()
            .size_full()
            .pt_0p5()
            .pb(bottom_padding)
            .pr(right_padding)
            .gap_0p5()
            .justify_center()
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .on_action(cx.listener(Self::confirm))
                    .on_action(cx.listener(Self::cancel))
                    .on_action(cx.listener(Self::move_up))
                    .on_action(cx.listener(Self::move_down))
                    .on_action(cx.listener(Self::thumbs_up))
                    .on_action(cx.listener(Self::thumbs_down))
                    .capture_action(cx.listener(Self::cycle_prev))
                    .capture_action(cx.listener(Self::cycle_next))
                    .on_action(cx.listener(|this, _: &ToggleModelSelector, window, cx| {
                        this.model_selector
                            .update(cx, |model_selector, cx| model_selector.toggle(window, cx));
                    }))
                    .on_action(cx.listener(|this, _: &CycleFavoriteModels, window, cx| {
                        this.model_selector.update(cx, |model_selector, cx| {
                            model_selector.cycle_favorite_models(window, cx);
                        });
                    }))
                    .child(
                        WithRemSize::new(ui_font_size)
                            .h_full()
                            .w(left_gutter_width)
                            .flex()
                            .flex_row()
                            .flex_shrink_0()
                            .items_center()
                            .justify_center()
                            .gap_1()
                            .child(self.render_close_button(cx))
                            .map(|el| {
                                let CodegenStatus::Error(error) = self.codegen_status(cx) else {
                                    return el;
                                };

                                let error_message = SharedString::from(error.to_string());
                                el.child(
                                    div()
                                        .id("error")
                                        .tooltip(Tooltip::text(error_message))
                                        .child(
                                            Icon::new(IconName::XCircle)
                                                .size(IconSize::Small)
                                                .color(Color::Error),
                                        ),
                                )
                            }),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .child(div().flex_1().child(self.render_editor(window, cx)))
                            .child(
                                WithRemSize::new(ui_font_size)
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .child(add_context_button)
                                    .child(self.model_selector.clone())
                                    .children(buttons),
                            ),
                    ),
            )
            .when_some(explanation, |this, _| {
                this.child(
                    h_flex()
                        .size_full()
                        .justify_center()
                        .child(div().w(left_gutter_width + px(6.)))
                        .child(
                            div()
                                .size_full()
                                .min_w_0()
                                .pt(rems_from_px(3.))
                                .pl_0p5()
                                .flex_1()
                                .border_t_1()
                                .border_color(cx.theme().colors().border_variant)
                                .child(explanation_label),
                        ),
                )
            })
    }
}

fn markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();
    let mut text_style = window.text_style();

    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        color: Some(colors.text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        syntax: cx.theme().syntax().clone(),
        selection_background_color: colors.element_selection_background,
        heading_level_styles: Some(HeadingLevelStyles {
            h1: Some(TextStyleRefinement {
                font_size: Some(rems(1.15).into()),
                ..Default::default()
            }),
            h2: Some(TextStyleRefinement {
                font_size: Some(rems(1.1).into()),
                ..Default::default()
            }),
            h3: Some(TextStyleRefinement {
                font_size: Some(rems(1.05).into()),
                ..Default::default()
            }),
            h4: Some(TextStyleRefinement {
                font_size: Some(rems(1.).into()),
                ..Default::default()
            }),
            h5: Some(TextStyleRefinement {
                font_size: Some(rems(0.95).into()),
                ..Default::default()
            }),
            h6: Some(TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                ..Default::default()
            }),
        }),
        inline_code: TextStyleRefinement {
            font_family: Some(theme_settings.buffer_font.family.clone()),
            font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
            font_features: Some(theme_settings.buffer_font.features.clone()),
            background_color: Some(colors.editor_foreground.opacity(0.08)),
            ..Default::default()
        },
        ..Default::default()
    }
}

impl<T: 'static> Focusable for PromptEditor<T> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl<T: 'static> PromptEditor<T> {
    const MAX_LINES: u8 = 8;

    fn codegen_status<'a>(&'a self, cx: &'a App) -> &'a CodegenStatus {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => codegen.read(cx).status(cx),
            PromptEditorMode::Terminal { codegen, .. } => &codegen.read(cx).status,
        }
    }

    fn subscribe_to_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions.push(cx.subscribe_in(
            &self.editor,
            window,
            Self::handle_prompt_editor_events,
        ));
    }

    fn assign_completion_provider(&mut self, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_completion_provider(Some(Rc::new(PromptCompletionProvider::new(
                PromptEditorCompletionProviderDelegate,
                cx.weak_entity(),
                self.mention_set.clone(),
                Some(self.thread_store.clone()),
                Rc::new(RefCell::new(None)),
                self.prompt_store.clone(),
                self.workspace.clone(),
            ))));
        });
    }

    pub fn set_show_cursor_when_unfocused(
        &mut self,
        show_cursor_when_unfocused: bool,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_show_cursor_when_unfocused(show_cursor_when_unfocused, cx)
        });
    }

    pub fn unlink(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let prompt = self.prompt(cx);
        let existing_creases = self.editor.update(cx, |editor, cx| {
            extract_message_creases(editor, &self.mention_set, window, cx)
        });
        let focus = self.editor.focus_handle(cx).contains_focused(window, cx);
        let mut creases = vec![];
        self.editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, Self::MAX_LINES as usize, window, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text("Add a prompt…", window, cx);
            editor.set_text(prompt, window, cx);
            creases = insert_message_creases(&mut editor, &existing_creases, window, cx);

            if focus {
                window.focus(&editor.focus_handle(cx), cx);
            }
            editor
        });

        self.mention_set.update(cx, |mention_set, _cx| {
            debug_assert_eq!(
                creases.len(),
                mention_set.creases().len(),
                "Missing creases"
            );

            let mentions = mention_set
                .clear()
                .zip(creases)
                .map(|((_, value), id)| (id, value))
                .collect::<HashMap<_, _>>();
            mention_set.set_mentions(mentions);
        });

        self.assign_completion_provider(cx);
        self.subscribe_to_editor(window, cx);
    }

    pub fn placeholder_text(mode: &PromptEditorMode, window: &mut Window, cx: &mut App) -> String {
        let action = match mode {
            PromptEditorMode::Buffer { codegen, .. } => {
                if codegen.read(cx).is_insertion {
                    "Generate"
                } else {
                    "Transform"
                }
            }
            PromptEditorMode::Terminal { .. } => "Generate",
        };

        let agent_panel_keybinding =
            ui::text_for_action(&zed_actions::assistant::ToggleFocus, window, cx)
                .map(|keybinding| format!("{keybinding} to chat"))
                .unwrap_or_default();

        format!("{action}… ({agent_panel_keybinding} ― ↓↑ for history — @ to include context)")
    }

    pub fn prompt(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if inline_assistant_model_supports_images(cx)
            && let Some(task) =
                paste_images_as_context(self.editor.clone(), self.mention_set.clone(), window, cx)
        {
            task.detach();
        }
    }

    fn handle_prompt_editor_events(
        &mut self,
        editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::Edited { .. } => {
                let snapshot = editor.update(cx, |editor, cx| editor.snapshot(window, cx));

                self.mention_set
                    .update(cx, |mention_set, _cx| mention_set.remove_invalid(&snapshot));

                if let Some(workspace) = window.root::<Workspace>().flatten() {
                    workspace.update(cx, |workspace, cx| {
                        let is_via_ssh = workspace.project().read(cx).is_via_remote_server();

                        workspace
                            .client()
                            .telemetry()
                            .log_edit_event("inline assist", is_via_ssh);
                    });
                }
                let prompt = snapshot.text();
                if self
                    .prompt_history_ix
                    .is_none_or(|ix| self.prompt_history[ix] != prompt)
                {
                    self.prompt_history_ix.take();
                    self.pending_prompt = prompt;
                }

                self.edited_since_done = true;
                self.session_state.completion = CompletionState::Pending;
                cx.notify();
            }
            EditorEvent::Blurred => {
                if self.show_rate_limit_notice {
                    self.show_rate_limit_notice = false;
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    pub fn is_completions_menu_visible(&self, cx: &App) -> bool {
        self.editor
            .read(cx)
            .context_menu()
            .borrow()
            .as_ref()
            .is_some_and(|menu| matches!(menu, CodeContextMenu::Completions(_)) && menu.visible())
    }

    pub fn trigger_completion_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let menu_is_open = editor.context_menu().borrow().as_ref().is_some_and(|menu| {
                matches!(menu, CodeContextMenu::Completions(_)) && menu.visible()
            });

            let has_at_sign = {
                let snapshot = editor.display_snapshot(cx);
                let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
                let offset = cursor.to_offset(&snapshot);
                if offset.0 > 0 {
                    snapshot
                        .buffer_snapshot()
                        .reversed_chars_at(offset)
                        .next()
                        .map(|sign| sign == '@')
                        .unwrap_or(false)
                } else {
                    false
                }
            };

            if menu_is_open && has_at_sign {
                return;
            }

            editor.insert("@", window, cx);
            editor.show_completions(&editor::actions::ShowCompletions, window, cx);
        });
    }

    fn cancel(
        &mut self,
        _: &editor::actions::Cancel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.codegen_status(cx) {
            CodegenStatus::Idle | CodegenStatus::Done | CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::CancelRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::StopRequested);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        match self.codegen_status(cx) {
            CodegenStatus::Idle => {
                self.fire_started_telemetry(cx);
                cx.emit(PromptEditorEvent::StartRequested);
            }
            CodegenStatus::Pending => {}
            CodegenStatus::Done => {
                if self.edited_since_done {
                    self.fire_started_telemetry(cx);
                    cx.emit(PromptEditorEvent::StartRequested);
                } else {
                    cx.emit(PromptEditorEvent::ConfirmRequested { execute: false });
                }
            }
            CodegenStatus::Error(_) => {
                self.fire_started_telemetry(cx);
                cx.emit(PromptEditorEvent::StartRequested);
            }
        }
    }

    fn fire_started_telemetry(&self, cx: &Context<Self>) {
        let Some(model) = LanguageModelRegistry::read_global(cx).inline_assistant_model() else {
            return;
        };

        let model_telemetry_id = model.model.telemetry_id();
        let model_provider_id = model.provider.id().to_string();

        let (kind, language_name) = match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => {
                let codegen = codegen.read(cx);
                (
                    "inline",
                    codegen.language_name(cx).map(|name| name.to_string()),
                )
            }
            PromptEditorMode::Terminal { .. } => ("inline_terminal", None),
        };

        telemetry::event!(
            "Assistant Started",
            session_id = self.session_state.session_id.to_string(),
            kind = kind,
            phase = "started",
            model = model_telemetry_id,
            model_provider = model_provider_id,
            language_name = language_name,
        );
    }

    fn thumbs_up(&mut self, _: &ThumbsUpResult, _window: &mut Window, cx: &mut Context<Self>) {
        match &self.session_state.completion {
            CompletionState::Pending => {
                self.toast("Can't rate, still generating...", None, cx);
                return;
            }
            CompletionState::Rated => {
                self.toast(
                    "Already rated this completion",
                    Some(self.session_state.session_id),
                    cx,
                );
                return;
            }
            CompletionState::Generated { completion_text } => {
                let model_info = self.model_selector.read(cx).active_model(cx);
                let (model_id, use_streaming_tools) = {
                    let Some(configured_model) = model_info else {
                        self.toast("No configured model", None, cx);
                        return;
                    };
                    (
                        configured_model.model.telemetry_id(),
                        CodegenAlternative::use_streaming_tools(
                            configured_model.model.as_ref(),
                            cx,
                        ),
                    )
                };

                let selected_text = match &self.mode {
                    PromptEditorMode::Buffer { codegen, .. } => {
                        codegen.read(cx).selected_text(cx).map(|s| s.to_string())
                    }
                    PromptEditorMode::Terminal { .. } => None,
                };

                let prompt = self.editor.read(cx).text(cx);

                let kind = match &self.mode {
                    PromptEditorMode::Buffer { .. } => "inline",
                    PromptEditorMode::Terminal { .. } => "inline_terminal",
                };

                telemetry::event!(
                    "Inline Assistant Rated",
                    rating = "positive",
                    session_id = self.session_state.session_id.to_string(),
                    kind = kind,
                    model = model_id,
                    prompt = prompt,
                    completion = completion_text,
                    selected_text = selected_text,
                    use_streaming_tools
                );

                self.session_state.completion = CompletionState::Rated;

                cx.notify();
            }
        }
    }

    fn thumbs_down(&mut self, _: &ThumbsDownResult, _window: &mut Window, cx: &mut Context<Self>) {
        match &self.session_state.completion {
            CompletionState::Pending => {
                self.toast("Can't rate, still generating...", None, cx);
                return;
            }
            CompletionState::Rated => {
                self.toast(
                    "Already rated this completion",
                    Some(self.session_state.session_id),
                    cx,
                );
                return;
            }
            CompletionState::Generated { completion_text } => {
                let model_info = self.model_selector.read(cx).active_model(cx);
                let (model_telemetry_id, use_streaming_tools) = {
                    let Some(configured_model) = model_info else {
                        self.toast("No configured model", None, cx);
                        return;
                    };
                    (
                        configured_model.model.telemetry_id(),
                        CodegenAlternative::use_streaming_tools(
                            configured_model.model.as_ref(),
                            cx,
                        ),
                    )
                };

                let selected_text = match &self.mode {
                    PromptEditorMode::Buffer { codegen, .. } => {
                        codegen.read(cx).selected_text(cx).map(|s| s.to_string())
                    }
                    PromptEditorMode::Terminal { .. } => None,
                };

                let prompt = self.editor.read(cx).text(cx);

                let kind = match &self.mode {
                    PromptEditorMode::Buffer { .. } => "inline",
                    PromptEditorMode::Terminal { .. } => "inline_terminal",
                };

                telemetry::event!(
                    "Inline Assistant Rated",
                    rating = "negative",
                    session_id = self.session_state.session_id.to_string(),
                    kind = kind,
                    model = model_telemetry_id,
                    prompt = prompt,
                    completion = completion_text,
                    selected_text = selected_text,
                    use_streaming_tools
                );

                self.session_state.completion = CompletionState::Rated;

                cx.notify();
            }
        }
    }

    fn toast(&mut self, msg: &str, uuid: Option<Uuid>, cx: &mut Context<'_, PromptEditor<T>>) {
        self.workspace
            .update(cx, |workspace, cx| {
                enum InlinePromptRating {}
                workspace.show_toast(
                    {
                        let mut toast = Toast::new(
                            NotificationId::unique::<InlinePromptRating>(),
                            msg.to_string(),
                        )
                        .autohide();

                        if let Some(uuid) = uuid {
                            toast = toast.on_click("Click to copy rating ID", move |_, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(uuid.to_string()));
                            });
                        };

                        toast
                    },
                    cx,
                );
            })
            .ok();
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

    fn render_buttons(&self, _window: &mut Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mode = match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => {
                let codegen = codegen.read(cx);
                if codegen.is_insertion {
                    GenerationMode::Generate
                } else {
                    GenerationMode::Transform
                }
            }
            PromptEditorMode::Terminal { .. } => GenerationMode::Generate,
        };

        let codegen_status = self.codegen_status(cx);

        match codegen_status {
            CodegenStatus::Idle => {
                vec![
                    Button::new("start", mode.start_label())
                        .label_size(LabelSize::Small)
                        .icon(IconName::Return)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::StartRequested)),
                        )
                        .into_any_element(),
                ]
            }
            CodegenStatus::Pending => vec![
                IconButton::new("stop", IconName::Stop)
                    .icon_color(Color::Error)
                    .shape(IconButtonShape::Square)
                    .tooltip(move |_window, cx| {
                        Tooltip::with_meta(
                            mode.tooltip_interrupt(),
                            Some(&menu::Cancel),
                            "Changes won't be discarded",
                            cx,
                        )
                    })
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::StopRequested)))
                    .into_any_element(),
            ],
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                let has_error = matches!(codegen_status, CodegenStatus::Error(_));
                if has_error || self.edited_since_done {
                    vec![
                        IconButton::new("restart", IconName::RotateCw)
                            .icon_color(Color::Info)
                            .shape(IconButtonShape::Square)
                            .tooltip(move |_window, cx| {
                                Tooltip::with_meta(
                                    mode.tooltip_restart(),
                                    Some(&menu::Confirm),
                                    "Changes will be discarded",
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(PromptEditorEvent::StartRequested);
                            }))
                            .into_any_element(),
                    ]
                } else {
                    let rated = matches!(self.session_state.completion, CompletionState::Rated);

                    let accept = IconButton::new("accept", IconName::Check)
                        .icon_color(Color::Info)
                        .shape(IconButtonShape::Square)
                        .tooltip(move |_window, cx| {
                            Tooltip::for_action(mode.tooltip_accept(), &menu::Confirm, cx)
                        })
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.emit(PromptEditorEvent::ConfirmRequested { execute: false });
                        }))
                        .into_any_element();

                    let mut buttons = Vec::new();

                    buttons.push(
                        h_flex()
                            .pl_1()
                            .gap_1()
                            .border_l_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                IconButton::new("thumbs-up", IconName::ThumbsUp)
                                    .shape(IconButtonShape::Square)
                                    .map(|this| {
                                        if rated {
                                            this.disabled(true).icon_color(Color::Disabled).tooltip(
                                                move |_, cx| {
                                                    Tooltip::with_meta(
                                                        "Good Result",
                                                        None,
                                                        "You already rated this result",
                                                        cx,
                                                    )
                                                },
                                            )
                                        } else {
                                            this.icon_color(Color::Muted).tooltip(move |_, cx| {
                                                Tooltip::for_action(
                                                    "Good Result",
                                                    &ThumbsUpResult,
                                                    cx,
                                                )
                                            })
                                        }
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.thumbs_up(&ThumbsUpResult, window, cx);
                                    })),
                            )
                            .child(
                                IconButton::new("thumbs-down", IconName::ThumbsDown)
                                    .shape(IconButtonShape::Square)
                                    .map(|this| {
                                        if rated {
                                            this.disabled(true).icon_color(Color::Disabled).tooltip(
                                                move |_, cx| {
                                                    Tooltip::with_meta(
                                                        "Bad Result",
                                                        None,
                                                        "You already rated this result",
                                                        cx,
                                                    )
                                                },
                                            )
                                        } else {
                                            this.icon_color(Color::Muted).tooltip(move |_, cx| {
                                                Tooltip::for_action(
                                                    "Bad Result",
                                                    &ThumbsDownResult,
                                                    cx,
                                                )
                                            })
                                        }
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.thumbs_down(&ThumbsDownResult, window, cx);
                                    })),
                            )
                            .into_any_element(),
                    );

                    buttons.push(accept);

                    match &self.mode {
                        PromptEditorMode::Terminal { .. } => {
                            buttons.push(
                                IconButton::new("confirm", IconName::PlayFilled)
                                    .icon_color(Color::Info)
                                    .shape(IconButtonShape::Square)
                                    .tooltip(|_window, cx| {
                                        Tooltip::for_action(
                                            "Execute Generated Command",
                                            &menu::SecondaryConfirm,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|_, _, _, cx| {
                                        cx.emit(PromptEditorEvent::ConfirmRequested {
                                            execute: true,
                                        });
                                    }))
                                    .into_any_element(),
                            );
                            buttons
                        }
                        PromptEditorMode::Buffer { .. } => buttons,
                    }
                }
            }
        }
    }

    fn cycle_prev(
        &mut self,
        _: &CyclePreviousInlineAssist,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => {
                codegen.update(cx, |codegen, cx| codegen.cycle_prev(cx));
            }
            PromptEditorMode::Terminal { .. } => {
                // no cycle buttons in terminal mode
            }
        }
    }

    fn cycle_next(&mut self, _: &CycleNextInlineAssist, _: &mut Window, cx: &mut Context<Self>) {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => {
                codegen.update(cx, |codegen, cx| codegen.cycle_next(cx));
            }
            PromptEditorMode::Terminal { .. } => {
                // no cycle buttons in terminal mode
            }
        }
    }

    fn render_close_button(&self, cx: &mut Context<Self>) -> AnyElement {
        let focus_handle = self.editor.focus_handle(cx);

        IconButton::new("cancel", IconName::Close)
            .icon_color(Color::Muted)
            .shape(IconButtonShape::Square)
            .tooltip({
                move |_window, cx| {
                    Tooltip::for_action_in(
                        "Close Assistant",
                        &editor::actions::Cancel,
                        &focus_handle,
                        cx,
                    )
                }
            })
            .on_click(cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::CancelRequested)))
            .into_any_element()
    }

    fn render_cycle_controls(&self, codegen: &BufferCodegen, cx: &Context<Self>) -> AnyElement {
        let disabled = matches!(codegen.status(cx), CodegenStatus::Idle);

        let model_registry = LanguageModelRegistry::read_global(cx);
        let default_model = model_registry.default_model().map(|default| default.model);
        let alternative_models = model_registry.inline_alternative_models();

        let get_model_name = |index: usize| -> String {
            let name = |model: &Arc<dyn LanguageModel>| model.name().0.to_string();

            match index {
                0 => default_model.as_ref().map_or_else(String::new, name),
                index if index <= alternative_models.len() => alternative_models
                    .get(index - 1)
                    .map_or_else(String::new, name),
                _ => String::new(),
            }
        };

        let total_models = alternative_models.len() + 1;

        if total_models <= 1 {
            return div().into_any_element();
        }

        let current_index = codegen.active_alternative;
        let prev_index = (current_index + total_models - 1) % total_models;
        let next_index = (current_index + 1) % total_models;

        let prev_model_name = get_model_name(prev_index);
        let next_model_name = get_model_name(next_index);

        h_flex()
            .child(
                IconButton::new("previous", IconName::ChevronLeft)
                    .icon_color(Color::Muted)
                    .disabled(disabled || current_index == 0)
                    .shape(IconButtonShape::Square)
                    .tooltip({
                        let focus_handle = self.editor.focus_handle(cx);
                        move |_window, cx| {
                            cx.new(|cx| {
                                let mut tooltip = Tooltip::new("Previous Alternative").key_binding(
                                    KeyBinding::for_action_in(
                                        &CyclePreviousInlineAssist,
                                        &focus_handle,
                                        cx,
                                    ),
                                );
                                if !disabled && current_index != 0 {
                                    tooltip = tooltip.meta(prev_model_name.clone());
                                }
                                tooltip
                            })
                            .into()
                        }
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.cycle_prev(&CyclePreviousInlineAssist, window, cx);
                    })),
            )
            .child(
                Label::new(format!(
                    "{}/{}",
                    codegen.active_alternative + 1,
                    codegen.alternative_count(cx)
                ))
                .size(LabelSize::Small)
                .color(if disabled {
                    Color::Disabled
                } else {
                    Color::Muted
                }),
            )
            .child(
                IconButton::new("next", IconName::ChevronRight)
                    .icon_color(Color::Muted)
                    .disabled(disabled || current_index == total_models - 1)
                    .shape(IconButtonShape::Square)
                    .tooltip({
                        let focus_handle = self.editor.focus_handle(cx);
                        move |_window, cx| {
                            cx.new(|cx| {
                                let mut tooltip = Tooltip::new("Next Alternative").key_binding(
                                    KeyBinding::for_action_in(
                                        &CycleNextInlineAssist,
                                        &focus_handle,
                                        cx,
                                    ),
                                );
                                if !disabled && current_index != total_models - 1 {
                                    tooltip = tooltip.meta(next_model_name.clone());
                                }
                                tooltip
                            })
                            .into()
                        }
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.cycle_next(&CycleNextInlineAssist, window, cx)
                    })),
            )
            .into_any_element()
    }

    fn render_editor(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let colors = cx.theme().colors();

        div()
            .size_full()
            .p_2()
            .pl_1()
            .bg(colors.editor_background)
            .child({
                let settings = ThemeSettings::get_global(cx);
                let font_size = settings.buffer_font_size(cx);
                let line_height = font_size * 1.2;

                let text_style = TextStyle {
                    color: colors.editor_foreground,
                    font_family: settings.buffer_font.family.clone(),
                    font_features: settings.buffer_font.features.clone(),
                    font_size: font_size.into(),
                    line_height: line_height.into(),
                    ..Default::default()
                };

                EditorElement::new(
                    &self.editor,
                    EditorStyle {
                        background: colors.editor_background,
                        local_player: cx.theme().players().local(),
                        syntax: cx.theme().syntax().clone(),
                        text: text_style,
                        ..Default::default()
                    },
                )
            })
            .into_any_element()
    }

    fn render_markdown(&self, markdown: Entity<Markdown>, style: MarkdownStyle) -> MarkdownElement {
        MarkdownElement::new(markdown, style)
    }
}

pub enum PromptEditorMode {
    Buffer {
        id: InlineAssistId,
        codegen: Entity<BufferCodegen>,
        editor_margins: Arc<Mutex<EditorMargins>>,
    },
    Terminal {
        id: TerminalInlineAssistId,
        codegen: Entity<TerminalCodegen>,
        height_in_lines: u8,
    },
}

pub enum PromptEditorEvent {
    StartRequested,
    StopRequested,
    ConfirmRequested { execute: bool },
    CancelRequested,
    Resized { height_in_lines: u8 },
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct InlineAssistId(pub usize);

impl InlineAssistId {
    pub fn post_inc(&mut self) -> InlineAssistId {
        let id = *self;
        self.0 += 1;
        id
    }
}

struct PromptEditorCompletionProviderDelegate;

fn inline_assistant_model_supports_images(cx: &App) -> bool {
    LanguageModelRegistry::read_global(cx)
        .inline_assistant_model()
        .map_or(false, |m| m.model.supports_images())
}

impl PromptCompletionProviderDelegate for PromptEditorCompletionProviderDelegate {
    fn supported_modes(&self, _cx: &App) -> Vec<PromptContextType> {
        vec![
            PromptContextType::File,
            PromptContextType::Symbol,
            PromptContextType::Thread,
            PromptContextType::Fetch,
            PromptContextType::Rules,
        ]
    }

    fn supports_images(&self, cx: &App) -> bool {
        inline_assistant_model_supports_images(cx)
    }

    fn available_commands(&self, _cx: &App) -> Vec<crate::completion_provider::AvailableCommand> {
        Vec::new()
    }

    fn confirm_command(&self, _cx: &mut App) {}
}

impl PromptEditor<BufferCodegen> {
    pub fn new_buffer(
        id: InlineAssistId,
        editor_margins: Arc<Mutex<EditorMargins>>,
        prompt_history: VecDeque<String>,
        prompt_buffer: Entity<MultiBuffer>,
        codegen: Entity<BufferCodegen>,
        session_id: Uuid,
        fs: Arc<dyn Fs>,
        thread_store: Entity<ThreadStore>,
        prompt_store: Option<Entity<PromptStore>>,
        project: WeakEntity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<PromptEditor<BufferCodegen>>,
    ) -> PromptEditor<BufferCodegen> {
        let codegen_subscription = cx.observe(&codegen, Self::handle_codegen_changed);
        let mode = PromptEditorMode::Buffer {
            id,
            codegen,
            editor_margins,
        };

        let prompt_editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(Self::MAX_LINES as usize),
                },
                prompt_buffer,
                None,
                window,
                cx,
            );
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            // Since the prompt editors for all inline assistants are linked,
            // always show the cursor (even when it isn't focused) because
            // typing in one will make what you typed appear in all of them.
            editor.set_show_cursor_when_unfocused(true, cx);
            editor.set_placeholder_text(&Self::placeholder_text(&mode, window, cx), window, cx);
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: None,
            });

            editor
        });

        let mention_set = cx
            .new(|_cx| MentionSet::new(project, Some(thread_store.clone()), prompt_store.clone()));

        let model_selector_menu_handle = PopoverMenuHandle::default();

        let mut this: PromptEditor<BufferCodegen> = PromptEditor {
            editor: prompt_editor.clone(),
            mention_set,
            thread_store,
            prompt_store,
            workspace,
            model_selector: cx.new(|cx| {
                AgentModelSelector::new(
                    fs,
                    model_selector_menu_handle,
                    prompt_editor.focus_handle(cx),
                    ModelUsageContext::InlineAssistant,
                    window,
                    cx,
                )
            }),
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: codegen_subscription,
            editor_subscriptions: Vec::new(),
            show_rate_limit_notice: false,
            mode,
            session_state: SessionState {
                session_id,
                completion: CompletionState::Pending,
            },
            _phantom: Default::default(),
        };

        this.assign_completion_provider(cx);
        this.subscribe_to_editor(window, cx);
        this
    }

    fn handle_codegen_changed(
        &mut self,
        codegen: Entity<BufferCodegen>,
        cx: &mut Context<PromptEditor<BufferCodegen>>,
    ) {
        match self.codegen_status(cx) {
            CodegenStatus::Idle => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Pending => {
                self.session_state.completion = CompletionState::Pending;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(true));
            }
            CodegenStatus::Done => {
                let completion = codegen.read(cx).active_completion(cx);
                self.session_state.completion = CompletionState::Generated {
                    completion_text: completion,
                };
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Error(_error) => {
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
        }
    }

    pub fn id(&self) -> InlineAssistId {
        match &self.mode {
            PromptEditorMode::Buffer { id, .. } => *id,
            PromptEditorMode::Terminal { .. } => unreachable!(),
        }
    }

    pub fn codegen(&self) -> &Entity<BufferCodegen> {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => codegen,
            PromptEditorMode::Terminal { .. } => unreachable!(),
        }
    }

    pub fn mention_set(&self) -> &Entity<MentionSet> {
        &self.mention_set
    }

    pub fn editor_margins(&self) -> &Arc<Mutex<EditorMargins>> {
        match &self.mode {
            PromptEditorMode::Buffer { editor_margins, .. } => editor_margins,
            PromptEditorMode::Terminal { .. } => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct TerminalInlineAssistId(pub usize);

impl TerminalInlineAssistId {
    pub fn post_inc(&mut self) -> TerminalInlineAssistId {
        let id = *self;
        self.0 += 1;
        id
    }
}

impl PromptEditor<TerminalCodegen> {
    pub fn new_terminal(
        id: TerminalInlineAssistId,
        prompt_history: VecDeque<String>,
        prompt_buffer: Entity<MultiBuffer>,
        codegen: Entity<TerminalCodegen>,
        session_id: Uuid,
        fs: Arc<dyn Fs>,
        thread_store: Entity<ThreadStore>,
        prompt_store: Option<Entity<PromptStore>>,
        project: WeakEntity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let codegen_subscription = cx.observe(&codegen, Self::handle_codegen_changed);
        let mode = PromptEditorMode::Terminal {
            id,
            codegen,
            height_in_lines: 1,
        };

        let prompt_editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(Self::MAX_LINES as usize),
                },
                prompt_buffer,
                None,
                window,
                cx,
            );
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text(&Self::placeholder_text(&mode, window, cx), window, cx);
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: None,
            });
            editor
        });

        let mention_set = cx
            .new(|_cx| MentionSet::new(project, Some(thread_store.clone()), prompt_store.clone()));

        let model_selector_menu_handle = PopoverMenuHandle::default();

        let mut this = Self {
            editor: prompt_editor.clone(),
            mention_set,
            thread_store,
            prompt_store,
            workspace,
            model_selector: cx.new(|cx| {
                AgentModelSelector::new(
                    fs,
                    model_selector_menu_handle.clone(),
                    prompt_editor.focus_handle(cx),
                    ModelUsageContext::InlineAssistant,
                    window,
                    cx,
                )
            }),
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: codegen_subscription,
            editor_subscriptions: Vec::new(),
            mode,
            show_rate_limit_notice: false,
            session_state: SessionState {
                session_id,
                completion: CompletionState::Pending,
            },
            _phantom: Default::default(),
        };
        this.count_lines(cx);
        this.assign_completion_provider(cx);
        this.subscribe_to_editor(window, cx);
        this
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

        match &mut self.mode {
            PromptEditorMode::Terminal {
                height_in_lines: current_height,
                ..
            } => {
                if height_in_lines != *current_height {
                    *current_height = height_in_lines;
                    cx.emit(PromptEditorEvent::Resized { height_in_lines });
                }
            }
            PromptEditorMode::Buffer { .. } => unreachable!(),
        }
    }

    fn handle_codegen_changed(&mut self, codegen: Entity<TerminalCodegen>, cx: &mut Context<Self>) {
        match &self.codegen().read(cx).status {
            CodegenStatus::Idle => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Pending => {
                self.session_state.completion = CompletionState::Pending;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(true));
            }
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                self.session_state.completion = CompletionState::Generated {
                    completion_text: codegen.read(cx).completion(),
                };
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
        }
    }

    pub fn mention_set(&self) -> &Entity<MentionSet> {
        &self.mention_set
    }

    pub fn codegen(&self) -> &Entity<TerminalCodegen> {
        match &self.mode {
            PromptEditorMode::Buffer { .. } => unreachable!(),
            PromptEditorMode::Terminal { codegen, .. } => codegen,
        }
    }

    pub fn id(&self) -> TerminalInlineAssistId {
        match &self.mode {
            PromptEditorMode::Buffer { .. } => unreachable!(),
            PromptEditorMode::Terminal { id, .. } => *id,
        }
    }
}

pub enum CodegenStatus {
    Idle,
    Pending,
    Done,
    Error(anyhow::Error),
}

/// This is just CodegenStatus without the anyhow::Error, which causes a lifetime issue for rendering the Cancel button.
#[derive(Copy, Clone)]
pub enum CancelButtonState {
    Idle,
    Pending,
    Done,
    Error,
}

impl Into<CancelButtonState> for &CodegenStatus {
    fn into(self) -> CancelButtonState {
        match self {
            CodegenStatus::Idle => CancelButtonState::Idle,
            CodegenStatus::Pending => CancelButtonState::Pending,
            CodegenStatus::Done => CancelButtonState::Done,
            CodegenStatus::Error(_) => CancelButtonState::Error,
        }
    }
}

#[derive(Copy, Clone)]
pub enum GenerationMode {
    Generate,
    Transform,
}

impl GenerationMode {
    fn start_label(self) -> &'static str {
        match self {
            GenerationMode::Generate => "Generate",
            GenerationMode::Transform => "Transform",
        }
    }
    fn tooltip_interrupt(self) -> &'static str {
        match self {
            GenerationMode::Generate => "Interrupt Generation",
            GenerationMode::Transform => "Interrupt Transform",
        }
    }

    fn tooltip_restart(self) -> &'static str {
        match self {
            GenerationMode::Generate => "Restart Generation",
            GenerationMode::Transform => "Restart Transform",
        }
    }

    fn tooltip_accept(self) -> &'static str {
        match self {
            GenerationMode::Generate => "Accept Generation",
            GenerationMode::Transform => "Accept Transform",
        }
    }
}

/// Stored information that can be used to resurrect a context crease when creating an editor for a past message.
#[derive(Clone, Debug)]
struct MessageCrease {
    range: Range<MultiBufferOffset>,
    icon_path: SharedString,
    label: SharedString,
}

fn extract_message_creases(
    editor: &mut Editor,
    mention_set: &Entity<MentionSet>,
    window: &mut Window,
    cx: &mut Context<'_, Editor>,
) -> Vec<MessageCrease> {
    let creases = mention_set.read(cx).creases();
    let snapshot = editor.snapshot(window, cx);
    snapshot
        .crease_snapshot
        .creases()
        .filter(|(id, _)| creases.contains(id))
        .filter_map(|(_, crease)| {
            let metadata = crease.metadata()?.clone();
            Some(MessageCrease {
                range: crease.range().to_offset(snapshot.buffer()),
                label: metadata.label,
                icon_path: metadata.icon_path,
            })
        })
        .collect()
}

fn insert_message_creases(
    editor: &mut Editor,
    message_creases: &[MessageCrease],
    window: &mut Window,
    cx: &mut Context<'_, Editor>,
) -> Vec<CreaseId> {
    let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
    let creases = message_creases
        .iter()
        .map(|crease| {
            let start = buffer_snapshot.anchor_after(crease.range.start);
            let end = buffer_snapshot.anchor_before(crease.range.end);
            crease_for_mention(
                crease.label.clone(),
                crease.icon_path.clone(),
                start..end,
                cx.weak_entity(),
            )
        })
        .collect::<Vec<_>>();
    let ids = editor.insert_creases(creases.clone(), cx);
    editor.fold_creases(creases, false, window, cx);
    ids
}
