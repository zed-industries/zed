use editor::{Editor, EditorElement, EditorStyle};
use gpui::{AppContext, FocusableView, Model, TextStyle, View};
use language_model::{LanguageModelRegistry, LanguageModelRequestTool};
use language_model_selector::LanguageModelSelector;
use picker::Picker;
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    prelude::*, ButtonLike, CheckboxWithLabel, ElevationIndex, IconButtonShape, KeyBinding,
    PopoverMenuHandle, Tooltip,
};

use crate::context::{Context, ContextKind};
use crate::context_picker::{ContextPicker, ContextPickerDelegate};
use crate::thread::{RequestKind, Thread};
use crate::ui::ContextPill;
use crate::{Chat, ToggleModelSelector};

pub struct MessageEditor {
    thread: Model<Thread>,
    editor: View<Editor>,
    context: Vec<Context>,
    pub(crate) context_picker_handle: PopoverMenuHandle<Picker<ContextPickerDelegate>>,
    use_tools: bool,
}

impl MessageEditor {
    pub fn new(thread: Model<Thread>, cx: &mut ViewContext<Self>) -> Self {
        let mocked_context = vec![Context {
            name: "shape.rs".into(),
            kind: ContextKind::File,
            text: "```rs\npub enum Shape {\n    Circle,\n    Square,\n    Triangle,\n}".into(),
        }];

        Self {
            thread,
            editor: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(80, cx);
                editor.set_placeholder_text("Ask anything or type @ to add context", cx);

                editor
            }),
            context: mocked_context,
            context_picker_handle: PopoverMenuHandle::default(),
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
        let context = self.context.drain(..).collect::<Vec<_>>();

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

    fn render_language_model_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_provider = LanguageModelRegistry::read_global(cx).active_provider();
        let active_model = LanguageModelRegistry::read_global(cx).active_model();

        LanguageModelSelector::new(
            |model, _cx| {
                println!("Selected {:?}", model.name());
            },
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .w_full()
                        .gap_0p5()
                        .child(
                            div()
                                .overflow_x_hidden()
                                .flex_grow()
                                .whitespace_nowrap()
                                .child(match (active_provider, active_model) {
                                    (Some(provider), Some(model)) => h_flex()
                                        .gap_1()
                                        .child(
                                            Icon::new(
                                                model.icon().unwrap_or_else(|| provider.icon()),
                                            )
                                            .color(Color::Muted)
                                            .size(IconSize::XSmall),
                                        )
                                        .child(
                                            Label::new(model.name().0)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .into_any_element(),
                                    _ => Label::new("No model selected")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element(),
                                }),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        ),
                )
                .tooltip(move |cx| Tooltip::for_action("Change Model", &ToggleModelSelector, cx)),
        )
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
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_2()
                    .child(ContextPicker::new(
                        cx.view().downgrade(),
                        IconButton::new("add-context", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small),
                    ))
                    .children(
                        self.context
                            .iter()
                            .map(|context| ContextPill::new(context.clone())),
                    )
                    .when(!self.context.is_empty(), |parent| {
                        parent.child(
                            IconButton::new("remove-all-context", IconName::Eraser)
                                .shape(IconButtonShape::Square)
                                .icon_size(IconSize::Small)
                                .tooltip(move |cx| Tooltip::text("Remove All Context", cx))
                                .on_click(cx.listener(|this, _event, cx| {
                                    this.context.clear();
                                    cx.notify();
                                })),
                        )
                    }),
            )
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
                    .child(h_flex().gap_2().child(CheckboxWithLabel::new(
                        "use-tools",
                        Label::new("Tools"),
                        self.use_tools.into(),
                        cx.listener(|this, selection, _cx| {
                            this.use_tools = match selection {
                                Selection::Selected => true,
                                Selection::Unselected | Selection::Indeterminate => false,
                            };
                        }),
                    )))
                    .child(
                        h_flex()
                            .gap_2()
                            .child(self.render_language_model_selector(cx))
                            .child(
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
            )
    }
}
