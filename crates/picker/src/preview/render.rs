use gpui::{Action, StyledText};
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

    fn render_empty(&self, _window: &mut Window, cx: &App) -> impl IntoElement {
        let content = match self.message() {
            // `with_highlights` inherits the container's text style (set below),
            // while keeping the message's own highlights (e.g. the file path in
            // the file finder's "Create new file" entry).
            Some(message) => StyledText::new(message.text.clone())
                .with_highlights(message.highlights.iter().cloned())
                .into_any_element(),
            None => Label::new("No results to preview")
                .color(Color::Muted)
                .into_any_element(),
        };
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .font_ui(cx)
            .text_ui(cx)
            .text_color(Color::Muted.color(cx))
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
