use std::sync::Arc;

use editor::{Editor, EditorElement, EditorEvent, EditorStyle};
use fs::Fs;
use gpui::{
    AppContext, DismissEvent, FocusableView, Model, Subscription, TextStyle, View, WeakModel,
    WeakView,
};
use language_model::{LanguageModelRegistry, LanguageModelRequestTool};
use language_model_selector::LanguageModelSelector;
use rope::Point;
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    prelude::*, ButtonLike, ElevationIndex, KeyBinding, PopoverMenu, PopoverMenuHandle,
    SwitchWithLabel,
};
use workspace::Workspace;

use crate::assistant_model_selector::AssistantModelSelector;
use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;
use crate::context_strip::{ContextStrip, SuggestContextKind};
use crate::thread::{RequestKind, Thread};
use crate::thread_store::ThreadStore;
use crate::{Chat, ToggleContextPicker, ToggleModelSelector};

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
        let context_store = cx.new_model(|_cx| ContextStore::new());
        let context_picker_menu_handle = PopoverMenuHandle::default();
        let inline_context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(10, cx);
            editor.set_placeholder_text("Ask anything…", cx);
            editor.set_show_indent_guides(false, cx);

            editor
        });
        let inline_context_picker = cx.new_view(|cx| {
            ContextPicker::new(
                workspace.clone(),
                Some(thread_store.clone()),
                context_store.downgrade(),
                ConfirmBehavior::Close,
                cx,
            )
        });
        let subscriptions = vec![
            cx.subscribe(&editor, Self::handle_editor_event),
            cx.subscribe(
                &inline_context_picker,
                Self::handle_inline_context_picker_event,
            ),
        ];

        Self {
            thread,
            editor: editor.clone(),
            context_store: context_store.clone(),
            context_strip: cx.new_view(|cx| {
                ContextStrip::new(
                    context_store,
                    workspace.clone(),
                    Some(thread_store.clone()),
                    editor.focus_handle(cx),
                    context_picker_menu_handle.clone(),
                    SuggestContextKind::File,
                    cx,
                )
            }),
            context_picker_menu_handle,
            inline_context_picker,
            inline_context_picker_menu_handle,
            model_selector: cx.new_view(|cx| {
                AssistantModelSelector::new(fs, model_selector_menu_handle.clone(), cx)
            }),
            model_selector_menu_handle,
            use_tools: false,
            _subscriptions: subscriptions,
        }
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_selector_menu_handle.toggle(cx)
    }

    fn toggle_context_picker(&mut self, _: &ToggleContextPicker, cx: &mut ViewContext<Self>) {
        self.context_picker_menu_handle.toggle(cx);
    }

    fn chat(&mut self, _: &Chat, cx: &mut ViewContext<Self>) {
        self.send_to_model(RequestKind::Chat, cx);
    }

    fn send_to_model(
        &mut self,
        request_kind: RequestKind,
        cx: &mut ViewContext<Self>,
    ) -> Option<()> {
        let provider = LanguageModelRegistry::read_global(cx).active_provider();
        if provider
            .as_ref()
            .map_or(false, |provider| provider.must_accept_terms(cx))
        {
            cx.notify();
            return None;
        }

        let model_registry = LanguageModelRegistry::read_global(cx);
        let model = model_registry.active_model()?;

        let user_message = self.editor.update(cx, |editor, cx| {
            let text = editor.text(cx);
            editor.clear(cx);
            text
        });
        let context = self.context_store.update(cx, |this, _cx| this.drain());

        self.thread.update(cx, |thread, cx| {
            thread.insert_user_message(user_message, context, cx);
            let mut request = thread.to_completion_request(request_kind, cx);

            if self.use_tools {
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
        });

        None
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

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .on_action(cx.listener(Self::toggle_model_selector))
            .on_action(cx.listener(Self::toggle_context_picker))
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
                            .menu(move |_cx| Some(inline_context_picker.clone()))
                            .attach(gpui::Corner::TopLeft)
                            .anchor(gpui::Corner::BottomLeft)
                            .offset(gpui::Point {
                                x: px(0.0),
                                y: px(-16.0),
                            })
                            .with_handle(self.inline_context_picker_menu_handle.clone()),
                    )
                    .child(
                        h_flex()
                            .justify_between()
                            .child(SwitchWithLabel::new(
                                "use-tools",
                                Label::new("Tools"),
                                self.use_tools.into(),
                                cx.listener(|this, selection, _cx| {
                                    this.use_tools = match selection {
                                        ToggleState::Selected => true,
                                        ToggleState::Unselected | ToggleState::Indeterminate => {
                                            false
                                        }
                                    };
                                }),
                            ))
                            .child(
                                h_flex().gap_1().child(self.model_selector.clone()).child(
                                    ButtonLike::new("chat")
                                        .style(ButtonStyle::Filled)
                                        .layer(ElevationIndex::ModalSurface)
                                        .child(Label::new("Submit"))
                                        .children(
                                            KeyBinding::for_action_in(&Chat, &focus_handle, cx)
                                                .map(|binding| binding.into_any_element()),
                                        )
                                        .on_click(move |_event, cx| {
                                            focus_handle.dispatch_action(&Chat, cx);
                                        }),
                                ),
                            ),
                    ),
            )
    }
}
