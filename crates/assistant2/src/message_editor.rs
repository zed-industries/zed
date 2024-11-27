use editor::{Editor, EditorElement, EditorStyle};
use gpui::{AppContext, FocusableView, Model, TextStyle, View};
use language_model::{LanguageModelRegistry, LanguageModelRequestTool};
use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, ButtonLike, CheckboxWithLabel, ElevationIndex, KeyBinding};

use crate::thread::{RequestKind, Thread};
use crate::Chat;

pub struct MessageEditor {
    thread: Model<Thread>,
    editor: View<Editor>,
    use_tools: bool,
}

impl MessageEditor {
    pub fn new(thread: Model<Thread>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            thread,
            editor: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(80, cx);
                editor.set_placeholder_text("Ask anything…", cx);

                editor
            }),
            use_tools: false,
        }
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

        self.thread.update(cx, |thread, cx| {
            thread.insert_user_message(user_message);
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
}

impl FocusableView for MessageEditor {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let font_size = TextSize::Default.rems(cx);
        let line_height = font_size.to_pixels(cx.rem_size()) * 1.3;
        let focus_handle = self.editor.focus_handle(cx);

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .size_full()
            .gap_2()
            .p_2()
            .bg(cx.theme().colors().editor_background)
            .child({
                let settings = ThemeSettings::get_global(cx);
                let text_style = TextStyle {
                    color: cx.theme().colors().editor_foreground,
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
                        background: cx.theme().colors().editor_background,
                        local_player: cx.theme().players().local(),
                        text: text_style,
                        ..Default::default()
                    },
                )
            })
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .child(
                                Button::new("add-context", "Add Context")
                                    .style(ButtonStyle::Filled)
                                    .icon(IconName::Plus)
                                    .icon_position(IconPosition::Start),
                            )
                            .child(CheckboxWithLabel::new(
                                "use-tools",
                                Label::new("Tools"),
                                self.use_tools.into(),
                                cx.listener(|this, selection, _cx| {
                                    this.use_tools = match selection {
                                        Selection::Selected => true,
                                        Selection::Unselected | Selection::Indeterminate => false,
                                    };
                                }),
                            )),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Button::new("codebase", "Codebase").style(ButtonStyle::Filled))
                            .child(Label::new("or"))
                            .child(
                                ButtonLike::new("chat")
                                    .style(ButtonStyle::Filled)
                                    .layer(ElevationIndex::ModalSurface)
                                    .child(Label::new("Chat"))
                                    .children(
                                        KeyBinding::for_action_in(&Chat, &focus_handle, cx)
                                            .map(|binding| binding.into_any_element()),
                                    )
                                    .on_click(move |_event, cx| {
                                        focus_handle.dispatch_action(&Chat, cx);
                                    }),
                            ),
                    ),
            )
    }
}
