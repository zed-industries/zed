use crate::prelude::*;
use crate::EditorPane;

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
        div().child("This is where Buffer Search goes.")
    }
}
