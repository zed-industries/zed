use gpui::Action;
use ui::{
    ActiveTheme, App, Color, InteractiveElement, IntoElement, Label, LabelCommon, LabelSize,
    ParentElement, StatefulInteractiveElement, Styled, div, v_flex,
};

use crate::ToMultiBuffer;

use crate::preview::{EditorPreview, PreviewLayout};

impl EditorPreview {
    pub(crate) fn render(&self, layout: PreviewLayout, cx: &App) -> impl IntoElement {
        match layout {
            PreviewLayout::Below => self.render_preview_below(cx).into_any_element(),
            PreviewLayout::Right => self.render_preview_right(cx).into_any_element(),
            PreviewLayout::Hidden => gpui::Empty.into_any_element(),
        }
    }
}

impl EditorPreview {
    pub(crate) fn render_preview_right(&self, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_body(cx))
    }

    fn render_preview_below(&self, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_body(cx))
    }

    fn render_body(&self, cx: &App) -> impl IntoElement {
        if self.has_content(cx) {
            div()
                .flex_1()
                .overflow_hidden()
                .child(self.editor_as_giant_button())
                .into_any_element()
        } else {
            self.render_empty().into_any_element()
        }
    }

    fn render_empty(&self) -> impl IntoElement {
        v_flex().size_full().items_center().justify_center().child(
            Label::new("No results to preview")
                .size(LabelSize::Large)
                .color(Color::Muted),
        )
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
