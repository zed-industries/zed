use std::ops::Range;

use crate::Editor;
use gpui::{
    div, AnyElement, Context, HighlightStyle, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Pixels, Size, StatefulInteractiveElement, Styled, StyledText, TextStyle,
};
use ui::{SharedString, StyledExt};

#[derive(Clone, Debug, PartialEq)]
pub struct SignatureHelpPopover {
    pub label: SharedString,
    pub style: TextStyle,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
}

impl SignatureHelpPopover {
    pub fn render(&mut self, max_size: Size<Pixels>, cx: &mut Context<Editor>) -> AnyElement {
        div()
            .id("signature_help_popover")
            .elevation_2(cx)
            .overflow_y_scroll()
            .max_w(max_size.width)
            .max_h(max_size.height)
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div().px_4().pb_1().child(
                    StyledText::new(self.label.clone())
                        .with_highlights(&self.style, self.highlights.iter().cloned()),
                ),
            )
            .into_any_element()
    }
}
