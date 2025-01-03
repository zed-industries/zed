use std::sync::Arc;

use editor::{Editor, EditorEvent};
use gpui::{prelude::*, AppContext, FocusHandle, FocusableView, Model};
use ui::prelude::*;

/// The head of a [`Picker`](crate::Picker).
pub(crate) enum Head {
    /// Picker has an editor that allows the user to filter the list.
    Editor(Model<Editor>),

    /// Picker has no head, it's just a list of items.
    Empty(Model<EmptyHead>),
}

impl Head {
    pub fn editor<V: 'static>(
        placeholder_text: Arc<str>,
        edit_handler: impl FnMut(&mut V, &Model<Editor>, &EditorEvent, &mut Window, &mut ModelContext<V>)
            + 'static,
        window: &mut Window,
        cx: &mut ModelContext<V>,
    ) -> Self {
        let editor = window.new_view(cx, |window, cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(placeholder_text, window, cx);
            editor
        });
        cx.subscribe_in(&editor, window, edit_handler).detach();
        Self::Editor(editor)
    }

    pub fn empty<V: 'static>(
        blur_handler: impl FnMut(&mut V, &mut Window, &mut ModelContext<V>) + 'static,
        window: &mut Window,
        cx: &mut ModelContext<V>,
    ) -> Self {
        let head = window.new_view(cx, EmptyHead::new);
        cx.on_blur(&head.focus_handle(cx), window, blur_handler)
            .detach();
        Self::Empty(head)
    }
}

/// An invisible element that can hold focus.
pub(crate) struct EmptyHead {
    focus_handle: FocusHandle,
}

impl EmptyHead {
    fn new(window: &mut Window, cx: &mut ModelContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for EmptyHead {
    fn render(&mut self, window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
        div().track_focus(&self.focus_handle(cx))
    }
}

impl FocusableView for EmptyHead {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
