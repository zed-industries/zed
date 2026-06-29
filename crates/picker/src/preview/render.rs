use gpui::{Action, StyledText};
use language::HighlightedText;
use ui::prelude::*;

use crate::ToMultiBuffer;

use crate::preview;
use crate::preview::EditorPreview;

impl EditorPreview {
    pub(crate) fn render(&self, layout: preview::Layout, cx: &App) -> impl IntoElement {
        match layout {
            preview::Layout::Below => self.render_preview_below(cx).into_any_element(),
            preview::Layout::Right => self.render_preview_right(cx).into_any_element(),
            preview::Layout::Hidden => gpui::Empty.into_any_element(),
        }
    }
}

impl EditorPreview {
    pub(crate) fn render_preview_right(&self, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_t_md()
            .rounded_b_md()
            .child(self.render_message_or_editor(cx))
    }

    fn render_preview_below(&self, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_b_md()
            .child(self.render_message_or_editor(cx))
    }

    fn render_message_or_editor(&self, cx: &App) -> impl IntoElement {
        if let Some(message) = &self.message {
            self.render_message(message, cx).into_any_element()
        } else {
            div()
                .flex_1()
                .overflow_hidden()
                .child(self.editor_as_giant_button())
                .into_any_element()
        }
    }

    fn render_message(&self, message: &HighlightedText, cx: &App) -> impl IntoElement {
        // `with_highlights` inherits the container's text style (set below),
        // while keeping the message's own highlights (e.g. the file path in
        // the file finder's "Create new file" entry).
        let content = StyledText::new(message.text.clone())
            .with_highlights(message.highlights.iter().cloned())
            .into_any_element();
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
