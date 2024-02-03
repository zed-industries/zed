use std::ops::Range;

use editor::{Editor, EditorEvent};
use gpui::{
    canvas, list, AnyElement, AppContext, AvailableSpace, EventEmitter, FocusHandle, FocusableView,
    InteractiveElement, IntoElement, ListState, ParentElement, Render, Styled, View, ViewContext,
    WeakView,
};
use ui::prelude::*;
use workspace::item::Item;
use workspace::Workspace;

use crate::{
    markdown_elements::ParsedMarkdown,
    markdown_parser::parse_markdown,
    markdown_renderer::{render_markdown_block, render_parsed_markdown, RenderContext},
    OpenPreview,
};

pub struct MarkdownPreviewView {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    contents: ParsedMarkdown,
    list_state: ListState,
}

impl MarkdownPreviewView {
    pub fn register(workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, cx| {
            if workspace.has_active_modal(cx) {
                cx.propagate();
                return;
            }

            if let Some(editor) = workspace.active_item_as::<Editor>(cx) {
                let workspace_handle = workspace.weak_handle();
                let view: View<MarkdownPreviewView> =
                    MarkdownPreviewView::new(editor, workspace_handle, cx);
                workspace.split_item(workspace::SplitDirection::Right, Box::new(view.clone()), cx);
                cx.notify();
            }
        });
    }

    pub fn new(
        active_editor: View<Editor>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Workspace>,
    ) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let view = cx.view().downgrade();

            let editor = active_editor.read(cx);
            let contents = editor.buffer().read(cx).snapshot(cx).text();
            let contents = parse_markdown(&contents);

            cx.subscribe(&active_editor, |this, editor, event: &EditorEvent, cx| {
                match event {
                    EditorEvent::Edited => {
                        let editor = editor.read(cx);
                        let contents = editor.buffer().read(cx).snapshot(cx).text();
                        this.contents = parse_markdown(&contents);
                        this.list_state.reset(this.contents.children.len());
                        cx.notify();
                    }
                    EditorEvent::SelectionsChanged { .. } => {
                        let editor = editor.read(cx);
                        let selection_range = editor.selections.last::<usize>(cx).range();
                        let selected_block = this.get_block_index_under_cursor(selection_range);
                        this.list_state.scroll_to_reveal_item(selected_block);
                        cx.notify();
                    }
                    _ => {}
                };
            })
            .detach();

            let list_state = ListState::new(
                contents.children.len(),
                gpui::ListAlignment::Top,
                px(1000.),
                move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |view, cx| {
                            let mut render_cx =
                                RenderContext::new(Some(view.workspace.clone()), cx);
                            let block = view.contents.children.get(ix).unwrap();
                            render_markdown_block(block, &mut render_cx)
                        })
                    } else {
                        div().into_any()
                    }
                },
            );

            Self {
                focus_handle: cx.focus_handle(),
                workspace,
                contents,
                list_state,
            }
        })
    }

    fn get_block_index_under_cursor(&self, selection_range: Range<usize>) -> usize {
        let mut block_index = 0;
        let cursor = selection_range.start;

        for (i, block) in self.contents.children.iter().enumerate() {
            let Range { start, end } = block.source_range();
            if start <= cursor && end >= cursor {
                block_index = i;
                break;
            }
        }

        return block_index;
    }
}

impl FocusableView for MarkdownPreviewView {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewEvent {}

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

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(workspace::item::ItemEvent)) {}
}

impl Render for MarkdownPreviewView {
    // TODO: Block quote not rendering as expected
    // TODO: List items will overflow
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            // .items_start() // TODO: Need?
            // .justify_start()
            .id("MarkdownPreview")
            .key_context("MarkdownPreview")
            .track_focus(&self.focus_handle)
            .full()
            .bg(cx.theme().colors().editor_background)
            .p_4()
            .child(
                div()
                    .flex_grow()
                    // .px_2()
                    // .pt_1()
                    .map(|this| this.child(list(self.list_state.clone()).full())),
            )
    }
}
