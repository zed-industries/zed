use gpui::{EventEmitter, FocusHandle, Focusable};
use project::ProjectPath;
use ui::{App, Context, InteractiveElement, ParentElement, Render, SharedString, Window, div};

use crate::Item;

/// A view to display when a certain buffer fails to open.
pub struct InvalidBufferView {
    /// Which path was attempted to open.
    pub project_path: ProjectPath,
    /// An error message, happened when opening the buffer.
    pub error: SharedString,
    focus_handle: FocusHandle,
}

impl InvalidBufferView {
    pub fn new(project_path: ProjectPath, e: &anyhow::Error, _: &mut Window, cx: &mut App) -> Self {
        Self {
            project_path,
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

        let path = self.project_path.path.as_ref();

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

// TODO kb also check other ways to open the file (e.g. by drag and drop) and ensure it's the same view that opens for them

impl Render for InvalidBufferView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl gpui::IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("InvalidBuffer")
            .child("so bad, TODO kb")
    }
}
