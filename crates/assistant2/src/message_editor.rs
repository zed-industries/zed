use std::sync::Arc;

use editor::actions::MoveUp;
use editor::{Editor, EditorElement, EditorEvent, EditorStyle};
use fs::Fs;
use gpui::{
    pulsating_between, Animation, AnimationExt, AppContext, DismissEvent, FocusableView, Model,
    Subscription, TextStyle, View, WeakModel, WeakView,
};
use language_model::{LanguageModelRegistry, LanguageModelRequestTool};
use language_model_selector::LanguageModelSelector;
use rope::Point;
use settings::Settings;
use std::time::Duration;
use theme::ThemeSettings;
use ui::{prelude::*, ButtonLike, KeyBinding, PopoverMenu, PopoverMenuHandle, Switch, TintColor};
use workspace::Workspace;

use crate::assistant_model_selector::AssistantModelSelector;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::{refresh_context_store_text, ContextStore};
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::thread::{RequestKind, Thread};
use crate::thread_store::ThreadStore;
use crate::{Chat, ChatMode, RemoveAllContext, ToggleContextPicker, ToggleModelSelector};

pub struct MessageEditor {
    thread: Model<Thread>,
    editor: View<Editor>,
    context_store: Model<ContextStore>,
    context_strip: View<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    inline_context_picker: View<ContextPicker>,
    inline_context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    model_selector: View<AssistantModelSelector>,
    model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    use_tools: bool,
    _subscriptions: Vec<Subscription>,
}

impl MessageEditor {
    pub fn new(
        fs: Arc<dyn Fs>,
        workspace: WeakView<Workspace>,
        thread_store: WeakModel<ThreadStore>,
        thread: Model<Thread>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let context_store = cx.new_model(|_cx| ContextStore::new(workspace.clone()));
        let context_picker_menu_handle = PopoverMenuHandle::default();
        let inline_context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(10, cx);
            editor.set_placeholder_text("Ask anything, @ to mention, ↑ to select", cx);
            editor.set_show_indent_guides(false, cx);

            editor
        });

        let inline_context_picker = cx.new_view(|cx| {
            ContextPicker::new(
                workspace.clone(),
                Some(thread_store.clone()),
                context_store.downgrade(),
                editor.downgrade(),
                ConfirmBehavior::Close,
                cx,
            )
        });

        let context_strip = cx.new_view(|cx| {
            ContextStrip::new(
                context_store.clone(),
                workspace.clone(),
                editor.downgrade(),
                Some(thread_store.clone()),
                context_picker_menu_handle.clone(),
                SuggestContextKind::File,
                cx,
            )
        });

        let subscriptions = vec![
            cx.subscribe(&editor, Self::handle_editor_event),
            cx.subscribe(
                &inline_context_picker,
                Self::handle_inline_context_picker_event,
            ),
            cx.subscribe(&context_strip, Self::handle_context_strip_event),
        ];

        Self {
            thread,
            editor: editor.clone(),
            context_store,
            context_strip,
            context_picker_menu_handle,
            inline_context_picker,
            inline_context_picker_menu_handle,
            model_selector: cx.new_view(|cx| {
                AssistantModelSelector::new(
                    fs,
                    model_selector_menu_handle.clone(),
                    editor.focus_handle(cx),
                    cx,
                )
            }),
            model_selector_menu_handle,
            use_tools: false,
            _subscriptions: subscriptions,
        }
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_selector_menu_handle.toggle(cx)
    }

    fn toggle_chat_mode(&mut self, _: &ChatMode, cx: &mut ViewContext<Self>) {
        self.use_tools = !self.use_tools;
        cx.notify();
    }

    fn toggle_context_picker(&mut self, _: &ToggleContextPicker, cx: &mut ViewContext<Self>) {
        self.context_picker_menu_handle.toggle(cx);
    }

    pub fn remove_all_context(&mut self, _: &RemoveAllContext, cx: &mut ViewContext<Self>) {
        self.context_store.update(cx, |store, _cx| store.clear());
        cx.notify();
    }

    fn chat(&mut self, _: &Chat, cx: &mut ViewContext<Self>) {
        self.send_to_model(RequestKind::Chat, cx);
    }

    fn send_to_model(&mut self, request_kind: RequestKind, cx: &mut ViewContext<Self>) {
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
            editor.clear(cx);
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
        editor: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::SelectionsChanged { .. } => {
                editor.update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let newest_cursor = editor.selections.newest::<Point>(cx).head();
                    if newest_cursor.column > 0 {
                        let behind_cursor = Point::new(newest_cursor.row, newest_cursor.column - 1);
                        let char_behind_cursor = snapshot.chars_at(behind_cursor).next();
                        if char_behind_cursor == Some('@') {
                            self.inline_context_picker_menu_handle.show(cx);
                        }
                    }
                });
            }
            _ => {}
        }
    }

    fn handle_inline_context_picker_event(
        &mut self,
        _inline_context_picker: View<ContextPicker>,
        _event: &DismissEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let editor_focus_handle = self.editor.focus_handle(cx);
        cx.focus(&editor_focus_handle);
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
            | ContextStripEvent::BlurredDown => {
                let editor_focus_handle = self.editor.focus_handle(cx);
                cx.focus(&editor_focus_handle);
            }
            ContextStripEvent::BlurredUp => {}
        }
    }

    fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if self.context_picker_menu_handle.is_deployed() {
            cx.propagate();
        } else {
            cx.focus_view(&self.context_strip);
        }
    }
}

impl FocusableView for MessageEditor {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let font_size = TextSize::Default.rems(cx);
        let line_height = font_size.to_pixels(cx.rem_size()) * 1.5;
        let focus_handle = self.editor.focus_handle(cx);
        let inline_context_picker = self.inline_context_picker.clone();
        let bg_color = cx.theme().colors().editor_background;
        let is_streaming_completion = self.thread.read(cx).is_streaming();
        let button_width = px(64.);

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
                            .menu(move |cx| {
                                inline_context_picker.update(cx, |this, cx| {
                                    this.init(cx);
                                });

                                Some(inline_context_picker.clone())
                            })
                            .attach(gpui::Corner::TopLeft)
                            .anchor(gpui::Corner::BottomLeft)
                            .offset(gpui::Point {
                                x: px(0.0),
                                y: px(-ThemeSettings::clamp_font_size(
                                    ThemeSettings::get_global(cx).ui_font_size,
                                )
                                .0 * 2.0)
                                    - px(4.0),
                            })
                            .with_handle(self.inline_context_picker_menu_handle.clone()),
                    )
                    .child(
                        h_flex()
                            .justify_between()
                            .child(
                                Switch::new("use-tools", self.use_tools.into())
                                    .label("Tools")
                                    .on_click(cx.listener(|this, selection, _cx| {
                                        this.use_tools = match selection {
                                            ToggleState::Selected => true,
                                            ToggleState::Unselected
                                            | ToggleState::Indeterminate => false,
                                        };
                                    }))
                                    .key_binding(KeyBinding::for_action_in(
                                        &ChatMode,
                                        &focus_handle,
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
                                                        cx,
                                                    )
                                                    .map(|binding| binding.into_any_element()),
                                                ),
                                        )
                                        .on_click(move |_event, cx| {
                                            focus_handle
                                                .dispatch_action(&editor::actions::Cancel, cx);
                                        })
                                } else {
                                    ButtonLike::new("submit-message")
                                        .width(button_width.into())
                                        .style(ButtonStyle::Filled)
                                        .child(
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .child(Label::new("Submit").size(LabelSize::Small))
                                                .children(
                                                    KeyBinding::for_action_in(
                                                        &Chat,
                                                        &focus_handle,
                                                        cx,
                                                    )
                                                    .map(|binding| binding.into_any_element()),
                                                ),
                                        )
                                        .on_click(move |_event, cx| {
                                            focus_handle.dispatch_action(&Chat, cx);
                                        })
                                },
                            )),
                    ),
            )
    }
}
