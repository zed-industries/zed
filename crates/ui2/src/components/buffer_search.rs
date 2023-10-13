use crate::prelude::*;
use crate::{h_stack, EditorPane, Icon, IconButton, Input};

#[derive(Element)]
#[element(view_state = "EditorPane")]
pub struct BufferSearch {}

impl BufferSearch {
    pub fn new() -> Self {
        Self {}
    }

    fn render(
        &mut self,
        _view: &mut EditorPane,
        cx: &mut ViewContext<EditorPane>,
    ) -> impl Element<ViewState = EditorPane> {
        let theme = theme(cx);

        h_stack()
            .fill(theme.highest.base.default.background)
            .p_2()
            .child(
                h_stack()
                    .child(Input::new("Search (↑/↓ for previous/next query)"))
                    .child(IconButton::new(Icon::Replace)),
            )
    }
}
