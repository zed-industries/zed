use std::sync::Arc;

use gpui::{App, Entity, FocusHandle, Focusable, prelude::*};
use ui::prelude::*;
use ui_input::{ErasedEditor, ErasedEditorEvent};

/// The head of a [`Picker`](crate::Picker).
pub(crate) enum Head {
    /// Picker has an editor that allows the user to filter the list.
    Editor(Arc<dyn ErasedEditor>),

    /// Picker has no head, it's just a list of items.
    Empty(Entity<EmptyHead>),
}

impl Head {
    pub fn editor<V: 'static>(
        placeholder_text: Arc<str>,
        mut edit_handler: impl FnMut(&mut V, &ErasedEditorEvent, &mut Window, &mut Context<V>) + 'static,
        window: &mut Window,
        cx: &mut Context<V>,
    ) -> Self {
        let editor = (ui_input::ERASED_EDITOR_FACTORY.get().unwrap())(window, cx);

        editor.set_placeholder_text(placeholder_text.as_ref(), window, cx);
        let this = cx.weak_entity();
        editor
            .subscribe(
                Box::new(move |event, window, cx| {
                    this.update(cx, |this, cx| (edit_handler)(this, &event, window, cx))
                        .ok();
                }),
                window,
                cx,
            )
            .detach();
        // cx.subscribe_in(&editor, window, |v, _, event, window, cx| {
        //     edit_handler(v, event, window, cx);
        // })
        // .detach();
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
