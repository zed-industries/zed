use crate::{ui::ProjectIndexButton, AssistantChat, CompletionProvider};
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{AnyElement, FontStyle, FontWeight, TextStyle, View, WeakView, WhiteSpace};
use settings::Settings;
use theme::ThemeSettings;
use ui::{popover_menu, prelude::*, ButtonLike, ContextMenu, Tooltip};

#[derive(IntoElement)]
pub struct Composer {
    editor: View<Editor>,
    project_index_button: Option<View<ProjectIndexButton>>,
    model_selector: AnyElement,
}

impl Composer {
    pub fn new(
        editor: View<Editor>,
        project_index_button: Option<View<ProjectIndexButton>>,
        model_selector: AnyElement,
    ) -> Self {
        Self {
            editor,
            project_index_button,
            model_selector,
        }
    }

    fn render_tools(&mut self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex().children(
            self.project_index_button
                .clone()
                .map(|view| view.into_any_element()),
        )
    }
}

impl RenderOnce for Composer {
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
        let font_size = rems(0.875);
        let line_height = font_size.to_pixels(cx.rem_size()) * 1.3;

        h_flex().w_full().items_start().mt_2().child(
            v_flex().size_full().gap_1().child(
                v_flex()
                    .w_full()
                    .p_3()
                    .bg(cx.theme().colors().editor_background)
                    .rounded_lg()
                    .child(
                        v_flex()
                            .justify_between()
                            .w_full()
                            .gap_2()
                            .child({
                                let settings = ThemeSettings::get_global(cx);
                                let text_style = TextStyle {
                                    color: cx.theme().colors().editor_foreground,
                                    font_family: settings.buffer_font.family.clone(),
                                    font_features: settings.buffer_font.features.clone(),
                                    font_size: font_size.into(),
                                    font_weight: FontWeight::NORMAL,
                                    font_style: FontStyle::Normal,
                                    line_height: line_height.into(),
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
                            })
                            .child(
                                h_flex()
                                    .flex_none()
                                    .gap_2()
                                    .justify_between()
                                    .w_full()
                                    .child(h_flex().gap_1().child(self.render_tools(cx)))
                                    .child(h_flex().gap_1().child(self.model_selector)),
                            ),
                    ),
            ),
        )
    }
}

#[derive(IntoElement)]
pub struct ModelSelector {
    assistant_chat: WeakView<AssistantChat>,
    model: String,
}

impl ModelSelector {
    pub fn new(assistant_chat: WeakView<AssistantChat>, model: String) -> Self {
        Self {
            assistant_chat,
            model,
        }
    }
}

impl RenderOnce for ModelSelector {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        popover_menu("model-switcher")
            .menu(move |cx| {
                ContextMenu::build(cx, |mut menu, cx| {
                    for model in CompletionProvider::get(cx).available_models() {
                        menu = menu.custom_entry(
                            {
                                let model = model.clone();
                                move |_| Label::new(model.clone()).into_any_element()
                            },
                            {
                                let assistant_chat = self.assistant_chat.clone();
                                move |cx| {
                                    _ = assistant_chat.update(cx, |assistant_chat, cx| {
                                        assistant_chat.model = model.clone();
                                        cx.notify();
                                    });
                                }
                            },
                        );
                    }
                    menu
                })
                .into()
            })
            .trigger(
                ButtonLike::new("active-model")
                    .child(
                        h_flex()
                            .w_full()
                            .gap_0p5()
                            .child(
                                div()
                                    .overflow_x_hidden()
                                    .flex_grow()
                                    .whitespace_nowrap()
                                    .child(
                                        Label::new(self.model)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(
                                div().child(
                                    Icon::new(IconName::ChevronDown)
                                        .color(Color::Muted)
                                        .size(IconSize::XSmall),
                                ),
                            ),
                    )
                    .style(ButtonStyle::Subtle)
                    .tooltip(move |cx| Tooltip::text("Change Model", cx)),
            )
            .anchor(gpui::AnchorCorner::BottomRight)
    }
}
