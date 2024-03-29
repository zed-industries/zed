use std::sync::Arc;

use editor::{Editor, EditorEvent};
use gpui::{prelude::*, AppContext, FocusHandle, FocusableView, View};
use ui::prelude::*;

/// The head of a [`Picker`](crate::Picker).
pub(crate) enum Head {
    /// Picker has an editor that allows the user to filter the list.
    Editor(View<Editor>),

    /// Picker has no head, it's just a list of items.
    Empty(View<EmptyHead>),
}

impl Head {
    pub fn editor<V: 'static>(
        placeholder_text: Arc<str>,
        edit_handler: impl FnMut(&mut V, View<Editor>, &EditorEvent, &mut ViewContext<'_, V>) + 'static,
        cx: &mut ViewContext<V>,
    ) -> Self {
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text(placeholder_text, cx);
            editor
        });
        cx.subscribe(&editor, edit_handler).detach();
        Self::Editor(editor)
    }

    pub fn empty<V: 'static>(
        blur_handler: impl FnMut(&mut V, &mut ViewContext<'_, V>) + 'static,
        cx: &mut ViewContext<V>,
    ) -> Self {
        let head = cx.new_view(|cx| EmptyHead::new(cx));
        cx.on_blur(&head.focus_handle(cx), blur_handler).detach();
        Self::Empty(head)
    }
}

/// An invisible element that can hold focus.
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
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div().track_focus(&self.focus_handle)
    }
}

impl FocusableView for EmptyHead {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
