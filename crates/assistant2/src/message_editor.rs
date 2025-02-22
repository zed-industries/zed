use std::sync::Arc;

use editor::actions::MoveUp;
use editor::{Editor, EditorElement, EditorEvent, EditorStyle};
use fs::Fs;
use gpui::{
    pulsating_between, Animation, AnimationExt, App, DismissEvent, Entity, Focusable, Subscription,
    TextStyle, WeakEntity,
};
use language_model::{LanguageModelRegistry, LanguageModelRequestTool};
use language_model_selector::LanguageModelSelector;
use rope::Point;
use settings::Settings;
use std::time::Duration;
use text::Bias;
use theme::{get_ui_font_size, ThemeSettings};
use ui::{
    prelude::*, ButtonLike, KeyBinding, PopoverMenu, PopoverMenuHandle, Switch, TintColor, Tooltip,
};
use workspace::Workspace;

use crate::assistant_model_selector::AssistantModelSelector;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::{refresh_context_store_text, ContextStore};
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::thread::{RequestKind, Thread};
use crate::thread_store::ThreadStore;
use crate::{Chat, ChatMode, RemoveAllContext, ToggleContextPicker, ToggleModelSelector};

pub struct MessageEditor {
    thread: Entity<Thread>,
    editor: Entity<Editor>,
    context_store: Entity<ContextStore>,
    context_strip: Entity<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    inline_context_picker: Entity<ContextPicker>,
    inline_context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    model_selector: Entity<AssistantModelSelector>,
    model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    use_tools: bool,
    _subscriptions: Vec<Subscription>,
}

impl MessageEditor {
    pub fn new(
        fs: Arc<dyn Fs>,
        workspace: WeakEntity<Workspace>,
        thread_store: WeakEntity<ThreadStore>,
        thread: Entity<Thread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let context_store = cx.new(|_cx| ContextStore::new(workspace.clone()));
        let context_picker_menu_handle = PopoverMenuHandle::default();
        let inline_context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(10, window, cx);
            editor.set_placeholder_text("Ask anything, @ to mention, ↑ to select", cx);
            editor.set_show_indent_guides(false, cx);

            editor
        });

        let inline_context_picker = cx.new(|cx| {
            ContextPicker::new(
                workspace.clone(),
                Some(thread_store.clone()),
                context_store.downgrade(),
                editor.downgrade(),
                ConfirmBehavior::Close,
                window,
                cx,
            )
        });

        let context_strip = cx.new(|cx| {
            ContextStrip::new(
                context_store.clone(),
                workspace.clone(),
                editor.downgrade(),
                Some(thread_store.clone()),
                context_picker_menu_handle.clone(),
                SuggestContextKind::File,
                window,
                cx,
            )
        });

        let subscriptions = vec![
            cx.subscribe_in(&editor, window, Self::handle_editor_event),
            cx.subscribe_in(
                &inline_context_picker,
                window,
                Self::handle_inline_context_picker_event,
            ),
            cx.subscribe_in(&context_strip, window, Self::handle_context_strip_event),
        ];

        Self {
            thread,
            editor: editor.clone(),
            context_store,
            context_strip,
            context_picker_menu_handle,
            inline_context_picker,
            inline_context_picker_menu_handle,
            model_selector: cx.new(|cx| {
                AssistantModelSelector::new(
                    fs,
                    model_selector_menu_handle.clone(),
                    editor.focus_handle(cx),
                    window,
                    cx,
                )
            }),
            model_selector_menu_handle,
            use_tools: false,
            _subscriptions: subscriptions,
        }
    }

    fn toggle_model_selector(
        &mut self,
        _: &ToggleModelSelector,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.model_selector_menu_handle.toggle(window, cx)
    }

    fn toggle_chat_mode(&mut self, _: &ChatMode, _window: &mut Window, cx: &mut Context<Self>) {
        self.use_tools = !self.use_tools;
        cx.notify();
    }

    fn toggle_context_picker(
        &mut self,
        _: &ToggleContextPicker,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_picker_menu_handle.toggle(window, cx);
    }

    pub fn remove_all_context(
        &mut self,
        _: &RemoveAllContext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_store.update(cx, |store, _cx| store.clear());
        cx.notify();
    }

    fn chat(&mut self, _: &Chat, window: &mut Window, cx: &mut Context<Self>) {
        self.send_to_model(RequestKind::Chat, window, cx);
    }

    fn is_editor_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).text(cx).is_empty()
    }

    fn is_model_selected(&self, cx: &App) -> bool {
        LanguageModelRegistry::read_global(cx)
            .active_model()
            .is_some()
    }

    fn send_to_model(
        &mut self,
        request_kind: RequestKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let provider = LanguageModelRegistry::read_global(cx).active_provider();
        if provider
            .as_ref()
            .map_or(false, |provider| provider.must_accept_terms(cx))
        {
            cx.notify();
            return;
        }

        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(model) = model_registry.active_model() else {
            return;
        };

        let user_message = self.editor.update(cx, |editor, cx| {
            let text = editor.text(cx);
            editor.clear(window, cx);
            text
        });

        let refresh_task = refresh_context_store_text(self.context_store.clone(), cx);

        let thread = self.thread.clone();
        let context_store = self.context_store.clone();
        let use_tools = self.use_tools;
        cx.spawn(move |_, mut cx| async move {
            refresh_task.await;
            thread
                .update(&mut cx, |thread, cx| {
                    let context = context_store.read(cx).snapshot(cx).collect::<Vec<_>>();
                    thread.insert_user_message(user_message, context, cx);
                    let mut request = thread.to_completion_request(request_kind, cx);

                    if use_tools {
                        request.tools = thread
                            .tools()
                            .tools(cx)
                            .into_iter()
                            .map(|tool| LanguageModelRequestTool {
                                name: tool.name(),
                                description: tool.description(),
                                input_schema: tool.input_schema(),
                            })
                            .collect();
                    }

                    thread.stream_completion(request, model, cx)
                })
                .ok();
        })
        .detach();
    }

    fn handle_editor_event(
        &mut self,
        editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::SelectionsChanged { .. } => {
                editor.update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let newest_cursor = editor.selections.newest::<Point>(cx).head();
                    if newest_cursor.column > 0 {
                        let behind_cursor = snapshot.clip_point(
                            Point::new(newest_cursor.row, newest_cursor.column - 1),
                            Bias::Left,
                        );
                        let char_behind_cursor = snapshot.chars_at(behind_cursor).next();
                        if char_behind_cursor == Some('@') {
                            self.inline_context_picker_menu_handle.show(window, cx);
                        }
                    }
                });
            }
            _ => {}
        }
    }

    fn handle_inline_context_picker_event(
        &mut self,
        _inline_context_picker: &Entity<ContextPicker>,
        _event: &DismissEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor_focus_handle = self.editor.focus_handle(cx);
        window.focus(&editor_focus_handle);
    }

    fn handle_context_strip_event(
        &mut self,
        _context_strip: &Entity<ContextStrip>,
        event: &ContextStripEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ContextStripEvent::PickerDismissed
            | ContextStripEvent::BlurredEmpty
            | ContextStripEvent::BlurredDown => {
                let editor_focus_handle = self.editor.focus_handle(cx);
                window.focus(&editor_focus_handle);
            }
            ContextStripEvent::BlurredUp => {}
        }
    }

    fn move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        if self.context_picker_menu_handle.is_deployed()
            || self.inline_context_picker_menu_handle.is_deployed()
        {
            cx.propagate();
        } else {
            self.context_strip.focus_handle(cx).focus(window);
        }
    }
}

impl Focusable for MessageEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let font_size = TextSize::Default.rems(cx);
        let line_height = font_size.to_pixels(window.rem_size()) * 1.5;
        let focus_handle = self.editor.focus_handle(cx);
        let inline_context_picker = self.inline_context_picker.clone();
        let bg_color = cx.theme().colors().editor_background;
        let is_streaming_completion = self.thread.read(cx).is_streaming();
        let button_width = px(64.);
        let is_model_selected = self.is_model_selected(cx);
        let is_editor_empty = self.is_editor_empty(cx);
        let submit_label_color = if is_editor_empty {
            Color::Muted
        } else {
            Color::Default
        };

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .on_action(cx.listener(Self::toggle_model_selector))
            .on_action(cx.listener(Self::toggle_context_picker))
            .on_action(cx.listener(Self::remove_all_context))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::toggle_chat_mode))
            .size_full()
            .gap_2()
            .p_2()
            .bg(bg_color)
            .child(self.context_strip.clone())
            .child(
                v_flex()
                    .gap_4()
                    .child({
                        let settings = ThemeSettings::get_global(cx);
                        let text_style = TextStyle {
                            color: cx.theme().colors().text,
                            font_family: settings.ui_font.family.clone(),
                            font_features: settings.ui_font.features.clone(),
                            font_size: font_size.into(),
                            font_weight: settings.ui_font.weight,
                            line_height: line_height.into(),
                            ..Default::default()
                        };

                        EditorElement::new(
                            &self.editor,
                            EditorStyle {
                                background: bg_color,
                                local_player: cx.theme().players().local(),
                                text: text_style,
                                ..Default::default()
                            },
                        )
                    })
                    .child(
                        PopoverMenu::new("inline-context-picker")
                            .menu(move |window, cx| {
                                inline_context_picker.update(cx, |this, cx| {
                                    this.init(window, cx);
                                });

                                Some(inline_context_picker.clone())
                            })
                            .attach(gpui::Corner::TopLeft)
                            .anchor(gpui::Corner::BottomLeft)
                            .offset(gpui::Point {
                                x: px(0.0),
                                y: (-get_ui_font_size(cx) * 2) - px(4.0),
                            })
                            .with_handle(self.inline_context_picker_menu_handle.clone()),
                    )
                    .child(
                        h_flex()
                            .justify_between()
                            .child(
                                Switch::new("use-tools", self.use_tools.into())
                                    .label("Tools")
                                    .on_click(cx.listener(|this, selection, _window, _cx| {
                                        this.use_tools = match selection {
                                            ToggleState::Selected => true,
                                            ToggleState::Unselected
                                            | ToggleState::Indeterminate => false,
                                        };
                                    }))
                                    .key_binding(KeyBinding::for_action_in(
                                        &ChatMode,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )),
                            )
                            .child(h_flex().gap_1().child(self.model_selector.clone()).child(
                                if is_streaming_completion {
                                    ButtonLike::new("cancel-generation")
                                        .width(button_width.into())
                                        .style(ButtonStyle::Tinted(TintColor::Accent))
                                        .child(
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .child(
                                                    Label::new("Cancel")
                                                        .size(LabelSize::Small)
                                                        .with_animation(
                                                            "pulsating-label",
                                                            Animation::new(Duration::from_secs(2))
                                                                .repeat()
                                                                .with_easing(pulsating_between(
                                                                    0.4, 0.8,
                                                                )),
                                                            |label, delta| label.alpha(delta),
                                                        ),
                                                )
                                                .children(
                                                    KeyBinding::for_action_in(
                                                        &editor::actions::Cancel,
                                                        &focus_handle,
                                                        window,
                                                        cx,
                                                    )
                                                    .map(|binding| binding.into_any_element()),
                                                ),
                                        )
                                        .on_click(move |_event, window, cx| {
                                            focus_handle.dispatch_action(
                                                &editor::actions::Cancel,
                                                window,
                                                cx,
                                            );
                                        })
                                } else {
                                    ButtonLike::new("submit-message")
                                        .width(button_width.into())
                                        .style(ButtonStyle::Filled)
                                        .disabled(is_editor_empty || !is_model_selected)
                                        .child(
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .child(
                                                    Label::new("Submit")
                                                        .size(LabelSize::Small)
                                                        .color(submit_label_color),
                                                )
                                                .children(
                                                    KeyBinding::for_action_in(
                                                        &Chat,
                                                        &focus_handle,
                                                        window,
                                                        cx,
                                                    )
                                                    .map(|binding| binding.into_any_element()),
                                                ),
                                        )
                                        .on_click(move |_event, window, cx| {
                                            focus_handle.dispatch_action(&Chat, window, cx);
                                        })
                                        .when(is_editor_empty, |button| {
                                            button
                                                .tooltip(Tooltip::text("Type a message to submit"))
                                        })
                                        .when(!is_model_selected, |button| {
                                            button.tooltip(Tooltip::text(
                                                "Select a model to continue",
                                            ))
                                        })
                                },
                            )),
                    ),
            )
    }
}
