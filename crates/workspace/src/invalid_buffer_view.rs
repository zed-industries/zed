use std::path::PathBuf;

use gpui::{EventEmitter, FocusHandle, Focusable};
use ui::{App, Context, InteractiveElement, ParentElement, Render, SharedString, Window, div};

use crate::Item;

/// A view to display when a certain buffer fails to open.
pub struct InvalidBufferView {
    /// Which path was attempted to open.
    pub abs_path: PathBuf,
    /// An error message, happened when opening the buffer.
    pub error: SharedString,
    focus_handle: FocusHandle,
}

impl InvalidBufferView {
    pub fn new(abs_path: PathBuf, e: &anyhow::Error, _: &mut Window, cx: &mut App) -> Self {
        Self {
            abs_path,
            error: format!("{e}").into(),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Item for InvalidBufferView {
    type Event = ();

    fn tab_content_text(&self, mut detail: usize, _: &App) -> SharedString {
        // Ensure we always render at least the filename.
        detail += 1;

        let path = self.abs_path.as_path();

        let mut prefix = path;
        while detail > 0 {
            if let Some(parent) = prefix.parent() {
                prefix = parent;
                detail -= 1;
            } else {
                break;
            }
        }

        let path = if detail > 0 {
            path
        } else {
            path.strip_prefix(prefix).unwrap_or(path)
        };

        SharedString::new(path.to_string_lossy())
    }
}

impl EventEmitter<()> for InvalidBufferView {}

impl Focusable for InvalidBufferView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InvalidBufferView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl gpui::IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("InvalidBuffer")
            .child("so bad, TODO kb")
    }
}
