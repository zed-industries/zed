use crate::{Editor, EditorStyle};
use gpui::{
    div, AnyElement, InteractiveElement, IntoElement, MouseButton, ParentElement, Pixels, Size,
    StatefulInteractiveElement, Styled, AppContext, WeakView,
};
use language::ParsedMarkdown;
use ui::StyledExt;
use workspace::Workspace;

#[derive(Clone, Debug)]
pub struct SignatureHelpPopover {
    pub parsed_content: ParsedMarkdown,
}

impl PartialEq for SignatureHelpPopover {
    fn eq(&self, other: &Self) -> bool {
        let str_equality = self.parsed_content.text.as_str() == other.parsed_content.text.as_str();
        let highlight_equality = self.parsed_content.highlights == other.parsed_content.highlights;
        str_equality && highlight_equality
    }
}

impl SignatureHelpPopover {
    pub fn render(
        &mut self,
        style: &EditorStyle,
        max_size: Size<Pixels>,
        workspace: Option<WeakModel<Workspace>>,
        model: &Model<Editor>, cx: &mut AppContext,
    ) -> AnyElement {
        div()
            .id("signature_help_popover")
            .elevation_2(cx)
            .overflow_y_scroll()
            .max_w(max_size.width)
            .max_h(max_size.height)
            .on_mouse_move(|_, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
            .child(div().p_2().child(crate::render_parsed_markdown(
                "signature_help_popover_content",
                &self.parsed_content,
                style,
                workspace,
                cx,
            )))
            .into_any_element()
    }
}
