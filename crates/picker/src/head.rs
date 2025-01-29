use std::sync::Arc;

use editor::{Editor, EditorEvent};
use gpui::{prelude::*, App, Entity, FocusHandle, Focusable};
use ui::prelude::*;

/// The head of a [`Picker`](crate::Picker).
pub(crate) enum Head {
    /// Picker has an editor that allows the user to filter the list.
    Editor(Entity<Editor>),

    /// Picker has no head, it's just a list of items.
    Empty(Entity<EmptyHead>),
}

impl Head {
    pub fn editor<V: 'static>(
        placeholder_text: Arc<str>,
        edit_handler: impl FnMut(&mut V, &Entity<Editor>, &EditorEvent, &mut Window, &mut Context<V>)
            + 'static,
        window: &mut Window,
        cx: &mut Context<V>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(placeholder_text, cx);
            editor
        });
        cx.subscribe_in(&editor, window, edit_handler).detach();
        Self::Editor(editor)
    }

    pub fn empty<V: 'static>(
        blur_handler: impl FnMut(&mut V, &mut Window, &mut Context<V>) + 'static,
        window: &mut Window,
        cx: &mut Context<V>,
    ) -> Self {
        let head = cx.new(EmptyHead::new);
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
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for EmptyHead {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().track_focus(&self.focus_handle(cx))
    }
}

impl Focusable for EmptyHead {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
