use editor::{Editor, EditorEvent};
use gpui::{
    div, AppContext, FocusHandle, FocusableView, IntoElement, ParentElement, Render, View,
    VisualContext,
};
use std::sync::Arc;
use ui::{prelude::*, Divider, ViewContext};

pub(crate) enum Head {
    /// Picker has an editor that allows the user to query list elements
    QueryLine(View<Editor>),

    /// Picker has no head, it's just a list of items
    Empty(View<EmptyHead>),
}

impl Head {
    pub fn query_line<V: 'static>(
        placeholder_text: Arc<str>,
        cx: &mut ViewContext<V>,
        edit_handler: impl FnMut(&mut V, View<Editor>, &EditorEvent, &mut ViewContext<'_, V>) + 'static,
    ) -> Self {
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text(placeholder_text, cx);
            editor
        });
        cx.subscribe(&editor, edit_handler).detach();
        Head::QueryLine(editor)
    }

    pub fn empty(cx: &mut WindowContext) -> Self {
        Self::Empty(cx.new_view(|cx| EmptyHead::new(cx)))
    }

    pub fn draw(&self) -> impl IntoElement {
        match self {
            Head::QueryLine(editor) => v_flex()
                .child(
                    h_flex()
                        .overflow_hidden()
                        .flex_none()
                        .h_9()
                        .px_4()
                        .child(editor.clone()),
                )
                .child(Divider::horizontal()),
            Head::Empty(empty_head) => div().child(empty_head.clone()),
        }
    }
}

impl FocusableView for Head {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        match &self {
            Head::QueryLine(editor) => editor.focus_handle(cx),
            Head::Empty(head) => head.focus_handle(cx),
        }
    }
}

impl Render for Head {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        self.draw()
    }
}

/// An invisible element that can hold focus
pub(crate) struct EmptyHead {
    focus_handle: FocusHandle,
}

impl EmptyHead {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for EmptyHead {
    fn render(&mut self, _: &mut ui::prelude::ViewContext<Self>) -> impl IntoElement {
        div().track_focus(&self.focus_handle)
    }
}

impl FocusableView for EmptyHead {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
