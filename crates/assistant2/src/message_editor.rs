use std::sync::Arc;

use editor::{Editor, EditorElement, EditorStyle};
use fs::Fs;
use gpui::{AppContext, FocusableView, Model, TextStyle, View, WeakModel, WeakView};
use language_model::{LanguageModelRegistry, LanguageModelRequestTool};
use language_model_selector::{LanguageModelSelector, LanguageModelSelectorPopoverMenu};
use settings::{update_settings_file, Settings};
use theme::ThemeSettings;
use ui::{
    prelude::*, ButtonLike, CheckboxWithLabel, ElevationIndex, KeyBinding, PopoverMenuHandle,
    Tooltip,
};
use workspace::Workspace;

use crate::assistant_settings::AssistantSettings;
use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;
use crate::context_strip::ContextStrip;
use crate::thread::{RequestKind, Thread};
use crate::thread_store::ThreadStore;
use crate::{Chat, ToggleContextPicker, ToggleModelSelector};

pub struct MessageEditor {
    thread: Model<Thread>,
    editor: View<Editor>,
    context_store: Model<ContextStore>,
    context_strip: View<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    language_model_selector: View<LanguageModelSelector>,
    language_model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    use_tools: bool,
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

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(80, cx);
            editor.set_placeholder_text("Ask anything, @ to add context", cx);
            editor.set_show_indent_guides(false, cx);

            editor
        });

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
                            move |settings, _cx| settings.set_model(model.clone()),
                        );
                    },
                    cx,
                )
            }),
            language_model_selector_menu_handle: PopoverMenuHandle::default(),
            use_tools: false,
        }
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.language_model_selector_menu_handle.toggle(cx);
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

    fn render_language_model_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_model = LanguageModelRegistry::read_global(cx).active_model();
        let focus_handle = self.language_model_selector.focus_handle(cx).clone();

        LanguageModelSelectorPopoverMenu::new(
            self.language_model_selector.clone(),
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
                                .child(match active_model {
                                    Some(model) => h_flex()
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
                .tooltip(move |cx| {
                    Tooltip::for_action_in("Change Model", &ToggleModelSelector, &focus_handle, cx)
                }),
        )
        .with_handle(self.language_model_selector_menu_handle.clone())
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
            .child(div().id("thread_editor").overflow_y_scroll().h_12().child({
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
                        background: bg_color,
                        local_player: cx.theme().players().local(),
                        text: text_style,
                        ..Default::default()
                    },
                )
            }))
            .child(
                h_flex()
                    .justify_between()
                    .child(CheckboxWithLabel::new(
                        "use-tools",
                        Label::new("Tools"),
                        self.use_tools.into(),
                        cx.listener(|this, selection, _cx| {
                            this.use_tools = match selection {
                                ToggleState::Selected => true,
                                ToggleState::Unselected | ToggleState::Indeterminate => false,
                            };
                        }),
                    ))
                    .child(
                        h_flex()
                            .gap_1()
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
