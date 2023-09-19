use super::{Element, Handle, Layout, LayoutId, Result, SharedString, ViewContext};
use std::marker::PhantomData;

pub fn field<S>(editor: Handle<Editor>) -> EditorElement<S> {
    EditorElement {
        editor,
        field: true,
        placeholder_text: None,
        parent_state: PhantomData,
    }
}

pub struct EditorElement<S> {
    editor: Handle<Editor>,
    field: bool,
    placeholder_text: Option<SharedString>,
    parent_state: PhantomData<S>,
}

impl<S> EditorElement<S> {
    pub fn field(mut self) -> Self {
        self.field = true;
        self
    }

    pub fn placeholder_text(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder_text = Some(text.into());
        self
    }
}

impl<S: 'static> Element for EditorElement<S> {
    type State = S;
    type FrameState = ();

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.editor.update(cx, |_editor, _cx| todo!())
    }

    fn paint(
        &mut self,
        _layout: Layout,
        _state: &mut Self::State,
        _frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.editor.update(cx, |_editor, _cx| todo!())
    }
}

pub struct Editor {}

impl Editor {
    pub fn new(_: &mut ViewContext<Self>) -> Self {
        Editor {}
    }
}
