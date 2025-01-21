use crate::assistant_model_selector::AssistantModelSelector;
use crate::buffer_codegen::BufferCodegen;
use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::terminal_codegen::TerminalCodegen;
use crate::thread_store::ThreadStore;
use crate::{CycleNextInlineAssist, CyclePreviousInlineAssist};
use crate::{RemoveAllContext, ToggleContextPicker, ToggleModelSelector};
use client::ErrorExt;
use collections::VecDeque;
use editor::{
    actions::{MoveDown, MoveUp},
    Editor, EditorElement, EditorEvent, EditorMode, EditorStyle, GutterDimensions, MultiBuffer,
};
use feature_flags::{FeatureFlagAppExt as _, ZedPro};
use fs::Fs;
use gpui::{
    anchored, deferred, point, AnyElement, AppContext, ClickEvent, CursorStyle, EventEmitter,
    FocusHandle, FocusableView, FontWeight, Model, Subscription, TextStyle, View, ViewContext,
    WeakModel, WeakView, WindowContext,
};
use language_model::{LanguageModel, LanguageModelRegistry};
use language_model_selector::LanguageModelSelector;
use parking_lot::Mutex;
use settings::Settings;
use std::cmp;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::utils::WithRemSize;
use ui::{
    prelude::*, CheckboxWithLabel, IconButtonShape, KeyBinding, Popover, PopoverMenuHandle, Tooltip,
};
use util::ResultExt;
use workspace::Workspace;

pub struct PromptEditor<T> {
    pub editor: View<Editor>,
    mode: PromptEditorMode,
    context_store: Model<ContextStore>,
    context_strip: View<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    model_selector: View<AssistantModelSelector>,
    model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    edited_since_done: bool,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
    _context_strip_subscription: Subscription,
    show_rate_limit_notice: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static> EventEmitter<PromptEditorEvent> for PromptEditor<T> {}

impl<T: 'static> Render for PromptEditor<T> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
        let mut buttons = Vec::new();

        let left_gutter_width = match &self.mode {
            PromptEditorMode::Buffer {
                id: _,
                codegen,
                gutter_dimensions,
            } => {
                let codegen = codegen.read(cx);

                if codegen.alternative_count(cx) > 1 {
                    buttons.push(self.render_cycle_controls(&codegen, cx));
                }

                let gutter_dimensions = gutter_dimensions.lock();

                gutter_dimensions.full_width() + (gutter_dimensions.margin / 2.0)
            }
            PromptEditorMode::Terminal { .. } => {
                // Give the equivalent of the same left-padding that we're using on the right
                Pixels::from(40.0)
            }
        };

        let bottom_padding = match &self.mode {
            PromptEditorMode::Buffer { .. } => Pixels::from(0.),
            PromptEditorMode::Terminal { .. } => Pixels::from(8.0),
        };

        buttons.extend(self.render_buttons(cx));

        v_flex()
            .key_context("PromptEditor")
            .bg(cx.theme().colors().editor_background)
            .block_mouse_down()
            .gap_0p5()
            .border_y_1()
            .border_color(cx.theme().status().info_border)
            .size_full()
            .pt_0p5()
            .pb(bottom_padding)
            .pr_6()
            .child(
                h_flex()
                    .items_start()
                    .cursor(CursorStyle::Arrow)
                    .on_action(cx.listener(Self::toggle_context_picker))
                    .on_action(cx.listener(Self::toggle_model_selector))
                    .on_action(cx.listener(Self::confirm))
                    .on_action(cx.listener(Self::cancel))
                    .on_action(cx.listener(Self::move_up))
                    .on_action(cx.listener(Self::move_down))
                    .on_action(cx.listener(Self::remove_all_context))
                    .capture_action(cx.listener(Self::cycle_prev))
                    .capture_action(cx.listener(Self::cycle_next))
                    .child(
                        WithRemSize::new(ui_font_size)
                            .flex()
                            .flex_row()
                            .flex_shrink_0()
                            .items_center()
                            .h_full()
                            .w(left_gutter_width)
                            .justify_center()
                            .gap_2()
                            .child(self.render_close_button(cx))
                            .map(|el| {
                                let CodegenStatus::Error(error) = self.codegen_status(cx) else {
                                    return el;
                                };

                                let error_message = SharedString::from(error.to_string());
                                if error.error_code() == proto::ErrorCode::RateLimitExceeded
                                    && cx.has_flag::<ZedPro>()
                                {
                                    el.child(
                                        v_flex()
                                            .child(
                                                IconButton::new(
                                                    "rate-limit-error",
                                                    IconName::XCircle,
                                                )
                                                .toggle_state(self.show_rate_limit_notice)
                                                .shape(IconButtonShape::Square)
                                                .icon_size(IconSize::Small)
                                                .on_click(
                                                    cx.listener(Self::toggle_rate_limit_notice),
                                                ),
                                            )
                                            .children(self.show_rate_limit_notice.then(|| {
                                                deferred(
                                                    anchored()
                                                        .position_mode(
                                                            gpui::AnchoredPositionMode::Local,
                                                        )
                                                        .position(point(px(0.), px(24.)))
                                                        .anchor(gpui::Corner::TopLeft)
                                                        .child(self.render_rate_limit_notice(cx)),
                                                )
                                            })),
                                    )
                                } else {
                                    el.child(
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
                                }
                            }),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .child(div().flex_1().child(self.render_editor(cx)))
                            .child(
                                WithRemSize::new(ui_font_size)
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .children(buttons),
                            ),
                    ),
            )
            .child(
                WithRemSize::new(ui_font_size)
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(h_flex().flex_shrink_0().w(left_gutter_width))
                    .child(
                        h_flex()
                            .w_full()
                            .pl_1()
                            .items_start()
                            .justify_between()
                            .child(self.context_strip.clone())
                            .child(self.model_selector.clone()),
                    ),
            )
    }
}

impl<T: 'static> FocusableView for PromptEditor<T> {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl<T: 'static> PromptEditor<T> {
    const MAX_LINES: u8 = 8;

    fn codegen_status<'a>(&'a self, cx: &'a AppContext) -> &'a CodegenStatus {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => codegen.read(cx).status(cx),
            PromptEditorMode::Terminal { codegen, .. } => &codegen.read(cx).status,
        }
    }

    fn subscribe_to_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions
            .push(cx.subscribe(&self.editor, Self::handle_prompt_editor_events));
    }

    pub fn set_show_cursor_when_unfocused(
        &mut self,
        show_cursor_when_unfocused: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_show_cursor_when_unfocused(show_cursor_when_unfocused, cx)
        });
    }

    pub fn unlink(&mut self, cx: &mut ViewContext<Self>) {
        let prompt = self.prompt(cx);
        let focus = self.editor.focus_handle(cx).contains_focused(cx);
        self.editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(Self::MAX_LINES as usize, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text(Self::placeholder_text(&self.mode, cx), cx);
            editor.set_placeholder_text("Add a prompt…", cx);
            editor.set_text(prompt, cx);
            if focus {
                editor.focus(cx);
            }
            editor
        });
        self.subscribe_to_editor(cx);
    }

    pub fn placeholder_text(mode: &PromptEditorMode, cx: &WindowContext) -> String {
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

        let assistant_panel_keybinding = ui::text_for_action(&crate::ToggleFocus, cx)
            .map(|keybinding| format!("{keybinding} to chat ― "))
            .unwrap_or_default();

        format!("{action}… ({assistant_panel_keybinding}↓↑ for history)")
    }

    pub fn prompt(&self, cx: &AppContext) -> String {
        self.editor.read(cx).text(cx)
    }

    fn toggle_rate_limit_notice(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.show_rate_limit_notice = !self.show_rate_limit_notice;
        if self.show_rate_limit_notice {
            cx.focus_view(&self.editor);
        }
        cx.notify();
    }

    fn handle_prompt_editor_events(
        &mut self,
        _: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::Edited { .. } => {
                if let Some(workspace) = cx.window_handle().downcast::<Workspace>() {
                    workspace
                        .update(cx, |workspace, cx| {
                            let is_via_ssh = workspace
                                .project()
                                .update(cx, |project, _| project.is_via_ssh());

                            workspace
                                .client()
                                .telemetry()
                                .log_edit_event("inline assist", is_via_ssh);
                        })
                        .log_err();
                }
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
            EditorEvent::Blurred => {
                if self.show_rate_limit_notice {
                    self.show_rate_limit_notice = false;
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn toggle_context_picker(&mut self, _: &ToggleContextPicker, cx: &mut ViewContext<Self>) {
        self.context_picker_menu_handle.toggle(cx);
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_selector_menu_handle.toggle(cx);
    }

    pub fn remove_all_context(&mut self, _: &RemoveAllContext, cx: &mut ViewContext<Self>) {
        self.context_store.update(cx, |store, _cx| store.clear());
        cx.notify();
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        match self.codegen_status(cx) {
            CodegenStatus::Idle | CodegenStatus::Done | CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::CancelRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::StopRequested);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match self.codegen_status(cx) {
            CodegenStatus::Idle => {
                cx.emit(PromptEditorEvent::StartRequested);
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
        } else {
            cx.focus_view(&self.context_strip);
        }
    }

    fn render_buttons(&self, cx: &mut ViewContext<Self>) -> Vec<AnyElement> {
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
                vec![Button::new("start", mode.start_label())
                    .label_size(LabelSize::Small)
                    .icon(IconName::Return)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StartRequested)))
                    .into_any_element()]
            }
            CodegenStatus::Pending => vec![IconButton::new("stop", IconName::Stop)
                .icon_color(Color::Error)
                .shape(IconButtonShape::Square)
                .tooltip(move |cx| {
                    Tooltip::with_meta(
                        mode.tooltip_interrupt(),
                        Some(&menu::Cancel),
                        "Changes won't be discarded",
                        cx,
                    )
                })
                .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StopRequested)))
                .into_any_element()],
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                let has_error = matches!(codegen_status, CodegenStatus::Error(_));
                if has_error || self.edited_since_done {
                    vec![IconButton::new("restart", IconName::RotateCw)
                        .icon_color(Color::Info)
                        .shape(IconButtonShape::Square)
                        .tooltip(move |cx| {
                            Tooltip::with_meta(
                                mode.tooltip_restart(),
                                Some(&menu::Confirm),
                                "Changes will be discarded",
                                cx,
                            )
                        })
                        .on_click(cx.listener(|_, _, cx| {
                            cx.emit(PromptEditorEvent::StartRequested);
                        }))
                        .into_any_element()]
                } else {
                    let accept = IconButton::new("accept", IconName::Check)
                        .icon_color(Color::Info)
                        .shape(IconButtonShape::Square)
                        .tooltip(move |cx| {
                            Tooltip::for_action(mode.tooltip_accept(), &menu::Confirm, cx)
                        })
                        .on_click(cx.listener(|_, _, cx| {
                            cx.emit(PromptEditorEvent::ConfirmRequested { execute: false });
                        }))
                        .into_any_element();

                    match &self.mode {
                        PromptEditorMode::Terminal { .. } => vec![
                            accept,
                            IconButton::new("confirm", IconName::Play)
                                .icon_color(Color::Info)
                                .shape(IconButtonShape::Square)
                                .tooltip(|cx| {
                                    Tooltip::for_action(
                                        "Execute Generated Command",
                                        &menu::SecondaryConfirm,
                                        cx,
                                    )
                                })
                                .on_click(cx.listener(|_, _, cx| {
                                    cx.emit(PromptEditorEvent::ConfirmRequested { execute: true });
                                }))
                                .into_any_element(),
                        ],
                        PromptEditorMode::Buffer { .. } => vec![accept],
                    }
                }
            }
        }
    }

    fn cycle_prev(&mut self, _: &CyclePreviousInlineAssist, cx: &mut ViewContext<Self>) {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => {
                codegen.update(cx, |codegen, cx| codegen.cycle_prev(cx));
            }
            PromptEditorMode::Terminal { .. } => {
                // no cycle buttons in terminal mode
            }
        }
    }

    fn cycle_next(&mut self, _: &CycleNextInlineAssist, cx: &mut ViewContext<Self>) {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => {
                codegen.update(cx, |codegen, cx| codegen.cycle_next(cx));
            }
            PromptEditorMode::Terminal { .. } => {
                // no cycle buttons in terminal mode
            }
        }
    }

    fn render_close_button(&self, cx: &ViewContext<Self>) -> AnyElement {
        IconButton::new("cancel", IconName::Close)
            .icon_color(Color::Muted)
            .shape(IconButtonShape::Square)
            .tooltip(|cx| Tooltip::text("Close Assistant", cx))
            .on_click(cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)))
            .into_any_element()
    }

    fn render_cycle_controls(&self, codegen: &BufferCodegen, cx: &ViewContext<Self>) -> AnyElement {
        let disabled = matches!(codegen.status(cx), CodegenStatus::Idle);

        let model_registry = LanguageModelRegistry::read_global(cx);
        let default_model = model_registry.active_model();
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
                        move |cx| {
                            cx.new_view(|cx| {
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
                    .on_click(cx.listener(|this, _, cx| {
                        this.cycle_prev(&CyclePreviousInlineAssist, cx);
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
                        move |cx| {
                            cx.new_view(|cx| {
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
                    .on_click(
                        cx.listener(|this, _, cx| this.cycle_next(&CycleNextInlineAssist, cx)),
                    ),
            )
            .into_any_element()
    }

    fn render_rate_limit_notice(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Popover::new().child(
            v_flex()
                .occlude()
                .p_2()
                .child(
                    Label::new("Out of Tokens")
                        .size(LabelSize::Small)
                        .weight(FontWeight::BOLD),
                )
                .child(Label::new(
                    "Try Zed Pro for higher limits, a wider range of models, and more.",
                ))
                .child(
                    h_flex()
                        .justify_between()
                        .child(CheckboxWithLabel::new(
                            "dont-show-again",
                            Label::new("Don't show again"),
                            if dismissed_rate_limit_notice() {
                                ui::ToggleState::Selected
                            } else {
                                ui::ToggleState::Unselected
                            },
                            |selection, cx| {
                                let is_dismissed = match selection {
                                    ui::ToggleState::Unselected => false,
                                    ui::ToggleState::Indeterminate => return,
                                    ui::ToggleState::Selected => true,
                                };

                                set_rate_limit_notice_dismissed(is_dismissed, cx)
                            },
                        ))
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("dismiss", "Dismiss")
                                        .style(ButtonStyle::Transparent)
                                        .on_click(cx.listener(Self::toggle_rate_limit_notice)),
                                )
                                .child(Button::new("more-info", "More Info").on_click(
                                    |_event, cx| {
                                        cx.dispatch_action(Box::new(
                                            zed_actions::OpenAccountSettings,
                                        ))
                                    },
                                )),
                        ),
                ),
        )
    }

    fn render_editor(&mut self, cx: &mut ViewContext<Self>) -> AnyElement {
        let font_size = TextSize::Default.rems(cx);
        let line_height = font_size.to_pixels(cx.rem_size()) * 1.3;

        div()
            .key_context("InlineAssistEditor")
            .size_full()
            .p_2()
            .pl_1()
            .bg(cx.theme().colors().editor_background)
            .child({
                let settings = ThemeSettings::get_global(cx);
                let text_style = TextStyle {
                    color: cx.theme().colors().editor_foreground,
                    font_family: settings.buffer_font.family.clone(),
                    font_features: settings.buffer_font.features.clone(),
                    font_size: font_size.into(),
                    line_height: line_height.into(),
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
            })
            .into_any_element()
    }

    fn handle_context_strip_event(
        &mut self,
        _context_strip: View<ContextStrip>,
        event: &ContextStripEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ContextStripEvent::PickerDismissed
            | ContextStripEvent::BlurredEmpty
            | ContextStripEvent::BlurredUp => {
                let editor_focus_handle = self.editor.focus_handle(cx);
                cx.focus(&editor_focus_handle);
            }
            ContextStripEvent::BlurredDown => {}
        }
    }
}

pub enum PromptEditorMode {
    Buffer {
        id: InlineAssistId,
        codegen: Model<BufferCodegen>,
        gutter_dimensions: Arc<Mutex<GutterDimensions>>,
    },
    Terminal {
        id: TerminalInlineAssistId,
        codegen: Model<TerminalCodegen>,
        height_in_lines: u8,
    },
}

pub enum PromptEditorEvent {
    StartRequested,
    StopRequested,
    ConfirmRequested { execute: bool },
    CancelRequested,
    DismissRequested,
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

impl PromptEditor<BufferCodegen> {
    #[allow(clippy::too_many_arguments)]
    pub fn new_buffer(
        id: InlineAssistId,
        gutter_dimensions: Arc<Mutex<GutterDimensions>>,
        prompt_history: VecDeque<String>,
        prompt_buffer: Model<MultiBuffer>,
        codegen: Model<BufferCodegen>,
        fs: Arc<dyn Fs>,
        context_store: Model<ContextStore>,
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        cx: &mut ViewContext<PromptEditor<BufferCodegen>>,
    ) -> PromptEditor<BufferCodegen> {
        let codegen_subscription = cx.observe(&codegen, Self::handle_codegen_changed);
        let mode = PromptEditorMode::Buffer {
            id,
            codegen,
            gutter_dimensions,
        };

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
            // Since the prompt editors for all inline assistants are linked,
            // always show the cursor (even when it isn't focused) because
            // typing in one will make what you typed appear in all of them.
            editor.set_show_cursor_when_unfocused(true, cx);
            editor.set_placeholder_text(Self::placeholder_text(&mode, cx), cx);
            editor
        });
        let context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

        let context_strip = cx.new_view(|cx| {
            ContextStrip::new(
                context_store.clone(),
                workspace.clone(),
                prompt_editor.downgrade(),
                thread_store.clone(),
                context_picker_menu_handle.clone(),
                SuggestContextKind::Thread,
                cx,
            )
        });

        let context_strip_subscription =
            cx.subscribe(&context_strip, Self::handle_context_strip_event);

        let mut this: PromptEditor<BufferCodegen> = PromptEditor {
            editor: prompt_editor.clone(),
            context_store,
            context_strip,
            context_picker_menu_handle,
            model_selector: cx.new_view(|cx| {
                AssistantModelSelector::new(
                    fs,
                    model_selector_menu_handle.clone(),
                    prompt_editor.focus_handle(cx),
                    cx,
                )
            }),
            model_selector_menu_handle,
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: codegen_subscription,
            editor_subscriptions: Vec::new(),
            _context_strip_subscription: context_strip_subscription,
            show_rate_limit_notice: false,
            mode,
            _phantom: Default::default(),
        };

        this.subscribe_to_editor(cx);
        this
    }

    fn handle_codegen_changed(
        &mut self,
        _: Model<BufferCodegen>,
        cx: &mut ViewContext<PromptEditor<BufferCodegen>>,
    ) {
        match self.codegen_status(cx) {
            CodegenStatus::Idle => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Pending => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(true));
            }
            CodegenStatus::Done => {
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Error(error) => {
                if cx.has_flag::<ZedPro>()
                    && error.error_code() == proto::ErrorCode::RateLimitExceeded
                    && !dismissed_rate_limit_notice()
                {
                    self.show_rate_limit_notice = true;
                    cx.notify();
                }

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

    pub fn codegen(&self) -> &Model<BufferCodegen> {
        match &self.mode {
            PromptEditorMode::Buffer { codegen, .. } => codegen,
            PromptEditorMode::Terminal { .. } => unreachable!(),
        }
    }

    pub fn gutter_dimensions(&self) -> &Arc<Mutex<GutterDimensions>> {
        match &self.mode {
            PromptEditorMode::Buffer {
                gutter_dimensions, ..
            } => gutter_dimensions,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new_terminal(
        id: TerminalInlineAssistId,
        prompt_history: VecDeque<String>,
        prompt_buffer: Model<MultiBuffer>,
        codegen: Model<TerminalCodegen>,
        fs: Arc<dyn Fs>,
        context_store: Model<ContextStore>,
        workspace: WeakView<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let codegen_subscription = cx.observe(&codegen, Self::handle_codegen_changed);
        let mode = PromptEditorMode::Terminal {
            id,
            codegen,
            height_in_lines: 1,
        };

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
            editor.set_placeholder_text(Self::placeholder_text(&mode, cx), cx);
            editor
        });
        let context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

        let context_strip = cx.new_view(|cx| {
            ContextStrip::new(
                context_store.clone(),
                workspace.clone(),
                prompt_editor.downgrade(),
                thread_store.clone(),
                context_picker_menu_handle.clone(),
                SuggestContextKind::Thread,
                cx,
            )
        });

        let context_strip_subscription =
            cx.subscribe(&context_strip, Self::handle_context_strip_event);

        let mut this = Self {
            editor: prompt_editor.clone(),
            context_store,
            context_strip,
            context_picker_menu_handle,
            model_selector: cx.new_view(|cx| {
                AssistantModelSelector::new(
                    fs,
                    model_selector_menu_handle.clone(),
                    prompt_editor.focus_handle(cx),
                    cx,
                )
            }),
            model_selector_menu_handle,
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: codegen_subscription,
            editor_subscriptions: Vec::new(),
            _context_strip_subscription: context_strip_subscription,
            mode,
            show_rate_limit_notice: false,
            _phantom: Default::default(),
        };
        this.count_lines(cx);
        this.subscribe_to_editor(cx);
        this
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

    fn handle_codegen_changed(&mut self, _: Model<TerminalCodegen>, cx: &mut ViewContext<Self>) {
        match &self.codegen().read(cx).status {
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

    pub fn codegen(&self) -> &Model<TerminalCodegen> {
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

const DISMISSED_RATE_LIMIT_NOTICE_KEY: &str = "dismissed-rate-limit-notice";

fn dismissed_rate_limit_notice() -> bool {
    db::kvp::KEY_VALUE_STORE
        .read_kvp(DISMISSED_RATE_LIMIT_NOTICE_KEY)
        .log_err()
        .map_or(false, |s| s.is_some())
}

fn set_rate_limit_notice_dismissed(is_dismissed: bool, cx: &mut AppContext) {
    db::write_and_log(cx, move || async move {
        if is_dismissed {
            db::kvp::KEY_VALUE_STORE
                .write_kvp(DISMISSED_RATE_LIMIT_NOTICE_KEY.into(), "1".into())
                .await
        } else {
            db::kvp::KEY_VALUE_STORE
                .delete_kvp(DISMISSED_RATE_LIMIT_NOTICE_KEY.into())
                .await
        }
    })
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
            GenerationMode::Generate { .. } => "Generate",
            GenerationMode::Transform => "Transform",
        }
    }
    fn tooltip_interrupt(self) -> &'static str {
        match self {
            GenerationMode::Generate { .. } => "Interrupt Generation",
            GenerationMode::Transform => "Interrupt Transform",
        }
    }

    fn tooltip_restart(self) -> &'static str {
        match self {
            GenerationMode::Generate { .. } => "Restart Generation",
            GenerationMode::Transform => "Restart Transform",
        }
    }

    fn tooltip_accept(self) -> &'static str {
        match self {
            GenerationMode::Generate { .. } => "Accept Generation",
            GenerationMode::Transform => "Accept Transform",
        }
    }
}
