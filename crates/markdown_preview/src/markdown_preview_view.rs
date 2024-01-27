use editor::Editor;
use gpui::{
    div, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled, View, ViewContext,
};
use language::LanguageRegistry;
use std::sync::Arc;
use ui::prelude::*;
use workspace::item::Item;
use workspace::Workspace;

use crate::OpenPreview;

pub struct MarkdownPreviewView {
    languages: Arc<LanguageRegistry>,
    active_editor: View<Editor>,
    focus_handle: FocusHandle,
}

impl MarkdownPreviewView {
    pub fn register(workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>) {
        let languages = workspace.app_state().languages.clone();

        workspace.register_action(move |workspace, _: &OpenPreview, cx| {
            if workspace.has_active_modal(cx) {
                cx.propagate();
                return;
            }
            let languages = languages.clone();
            let editor = workspace.active_item_as::<Editor>(cx).unwrap();
            MarkdownPreviewView::deploy_preview(workspace, languages, editor, cx);
            cx.notify();
        });
    }

    pub fn new(
        active_editor: View<Editor>,
        languages: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            languages,
            active_editor,
            focus_handle,
        }
    }

    // Re-active the most recently active preview, or create a new one if there isn't one.
    fn deploy_preview(
        workspace: &mut Workspace,
        languages: Arc<LanguageRegistry>,
        active_editor: View<Editor>,
        cx: &mut ViewContext<Workspace>,
    ) {
        let existing = workspace
            .active_pane()
            .read(cx)
            .items()
            .find_map(|item| item.downcast::<MarkdownPreviewView>());

        Self::existing_or_new_preview(workspace, languages, active_editor, existing, cx)
    }

    fn existing_or_new_preview(
        workspace: &mut Workspace,
        languages: Arc<LanguageRegistry>,
        active_editor: View<Editor>,
        existing: Option<View<MarkdownPreviewView>>,
        cx: &mut ViewContext<Workspace>,
    ) {
        if let Some(existing) = existing {
            workspace.activate_item(&existing, cx);
            existing
        } else {
            let view: View<MarkdownPreviewView> =
                cx.new_view(|cx| MarkdownPreviewView::new(active_editor, languages, cx));

            workspace.add_item(Box::new(view.clone()), cx);
            view
        };
    }
}

impl FocusableView for MarkdownPreviewView {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewEvent {
    Dismiss,
}

impl EventEmitter<PreviewEvent> for MarkdownPreviewView {}

impl Item for MarkdownPreviewView {
    type Event = PreviewEvent;

    fn tab_content(
        &self,
        _detail: Option<usize>,
        selected: bool,
        _cx: &WindowContext,
    ) -> AnyElement {
        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FileDoc).color(if selected {
                Color::Default
            } else {
                Color::Muted
            }))
            .child(Label::new("Markdown preview").color(if selected {
                Color::Default
            } else {
                Color::Muted
            }))
            .into_any()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("markdown preview")
    }

    fn to_item_events(event: &Self::Event, _f: impl FnMut(workspace::item::ItemEvent)) {
        // TODO: Not sure what I need here
        match event {
            PreviewEvent::Dismiss => {}
        }
    }
}

impl Render for MarkdownPreviewView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let container = div()
            .elevation_3(cx)
            .key_context("MarkdownPreview")
            .p_4()
            .gap_4();

        let editor = self.active_editor.read(cx);
        let contents = editor.buffer().read(cx).snapshot(cx).text();

        let mentions = vec![];
        let text = rich_text::render_markdown(contents, &mentions, &self.languages, None);

        let md_container = div()
            // TODO: Why do I need `.id` in order to use overflow?
            // TODO: This doesn't make the child `text` element scrollable
            // and I can't add `overflow_y_scroll` to `text` because it's
            // an `AnyElement`.
            .id("markdown_preview_container")
            .gap_2()
            .overflow_y_scroll()
            .child(text.element("body".into(), cx));

        container.child(md_container)
    }
}

// TODO: Testing
