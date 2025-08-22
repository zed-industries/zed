use gpui::{EventEmitter, FocusHandle, Focusable};
use project::ProjectPath;
use ui::{App, Context, InteractiveElement, ParentElement, Render, SharedString, Window, div};
use workspace::Item;

pub struct InvalidBufferView {
    pub project_path: ProjectPath,
    error: SharedString,
    focus_handle: FocusHandle,
}

impl InvalidBufferView {
    pub fn new(
        project_path: ProjectPath,
        e: &anyhow::Error,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            project_path,
            error: format!("{e}").into(),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Item for InvalidBufferView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "TODO kb".into()
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
