use gpui::Action;
use ui::TextSize;
use ui::prelude::*;

use crate::ToMultiBuffer;

use crate::preview;
use crate::preview::EditorPreview;

impl EditorPreview {
    pub(crate) fn render(
        &self,
        layout: preview::Layout,
        window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        match layout {
            preview::Layout::Below => self.render_preview_below(window, cx).into_any_element(),
            preview::Layout::Right => self.render_preview_right(window, cx).into_any_element(),
            preview::Layout::Hidden => gpui::Empty.into_any_element(),
        }
    }
}

impl EditorPreview {
    pub(crate) fn render_preview_right(&self, window: &mut Window, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_t_md()
            .rounded_b_md()
            .child(self.render_body(window, cx))
    }

    fn render_preview_below(&self, window: &mut Window, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_b_md()
            .child(self.render_body(window, cx))
    }

    fn render_body(&self, window: &mut Window, cx: &App) -> impl IntoElement {
        if self.has_content(cx) {
            div()
                .flex_1()
                .overflow_hidden()
                .child(self.editor_as_giant_button())
                .into_any_element()
        } else {
            self.render_empty(window, cx).into_any_element()
        }
    }

    fn render_empty(&self, window: &mut Window, cx: &App) -> impl IntoElement {
        let content = match self.message() {
            Some(message) => {
                let mut text_style = window.text_style();
                text_style.color = Color::Muted.color(cx);
                text_style.font_size = TextSize::Large.rems(cx).into();
                message.to_styled_text(&text_style).into_any_element()
            }
            None => Label::new("No results to preview")
                .size(LabelSize::Large)
                .color(Color::Muted)
                .into_any_element(),
        };
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(content)
    }

    fn editor_as_giant_button(&self) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(self.preview_editor.clone())
            .child(
                div()
                    .id("picker-preview-editor")
                    .absolute()
                    .inset_0()
                    .occlude()
                    .on_click(|_, window, cx| {
                        window.dispatch_action(ToMultiBuffer.boxed_clone(), cx);
                    }),
            )
    }
}
