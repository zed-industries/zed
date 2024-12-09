use std::sync::Arc;

use editor::{Editor, EditorEvent};
use gpui::{prelude::*, AppContext, FocusHandle, FocusableView, View};
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
        edit_handler: impl FnMut(&mut V, Model<Editor>, &EditorEvent,&Model<'>,  &mut AppContext) + 'static,
        model: &Model<V>, cx: &mut AppContext,
    ) -> Self {
        let editor = cx.new_model(|model, cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text(placeholder_text, cx);
            editor
        });
        cx.subscribe(&editor, edit_handler).detach();
        Self::Editor(editor)
    }

    pub fn empty<V: 'static>(
        blur_handler: impl FnMut(&mut V,&Model<'>,  &mut AppContext) + 'static,
        model: &Model<V>, cx: &mut AppContext,
    ) -> Self {
        let head = cx.new_model(EmptyHead::new);
        cx.on_blur(&head.focus_handle(cx), blur_handler).detach();
        Self::Empty(head)
    }
}

/// An invisible element that can hold focus.
pub(crate) struct EmptyHead {
    focus_handle: FocusHandle,
}

impl EmptyHead {
    fn new(model: &Model<Self>, cx: &mut AppContext) -> Self {
        Self {
            focus_handle: window.focus_handle(),
        }
    }
}

impl Render for EmptyHead {
    fn render(&mut self, model: &Model<Self>, window: &mut gpui::Window, cx: &mut AppContext) -> impl IntoElement {
        div().track_focus(&self.focus_handle(cx))
    }
}

impl FocusableView for EmptyHead {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
