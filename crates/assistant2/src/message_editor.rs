use editor::{Editor, EditorElement, EditorStyle};
use gpui::{TextStyle, View};
use settings::Settings;
use theme::ThemeSettings;
use ui::prelude::*;

pub struct MessageEditor {
    editor: View<Editor>,
}

impl MessageEditor {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            editor: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(80, cx);
                editor.set_placeholder_text("Ask anything…", cx);

                editor
            }),
        }
    }
}

impl Render for MessageEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let font_size = TextSize::Default.rems(cx);
        let line_height = font_size.to_pixels(cx.rem_size()) * 1.3;

        v_flex()
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
                        h_flex().child(
                            Button::new("add-context", "Add Context")
                                .style(ButtonStyle::Filled)
                                .icon(IconName::Plus)
                                .icon_position(IconPosition::Start),
                        ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Button::new("codebase", "Codebase").style(ButtonStyle::Filled))
                            .child(Label::new("or"))
                            .child(Button::new("chat", "Chat").style(ButtonStyle::Filled)),
                    ),
            )
    }
}
