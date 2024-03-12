use std::sync::Arc;
use std::{ops::Range, path::PathBuf};

use editor::{Editor, EditorEvent};
use gpui::{
    list, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    IntoElement, ListState, ParentElement, Render, Styled, View, ViewContext, WeakView,
};
use language::LanguageRegistry;
use ui::prelude::*;
use workspace::item::{Item, ItemHandle};
use workspace::Workspace;

use crate::{
    markdown_elements::ParsedMarkdown,
    markdown_parser::parse_markdown,
    markdown_renderer::{render_markdown_block, RenderContext},
    OpenPreview,
};

pub struct MarkdownPreviewView {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    contents: Option<ParsedMarkdown>,
    selected_block: usize,
    list_state: ListState,
    tab_description: String,
}

impl MarkdownPreviewView {
    pub fn register(workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, cx| {
            if workspace.has_active_modal(cx) {
                cx.propagate();
                return;
            }

            if let Some(editor) = workspace.active_item_as::<Editor>(cx) {
                let language_registry = workspace.project().read(cx).languages().clone();
                let workspace_handle = workspace.weak_handle();
                let tab_description = editor.tab_description(0, cx);
                let view: View<MarkdownPreviewView> = MarkdownPreviewView::new(
                    editor,
                    workspace_handle,
                    tab_description,
                    language_registry,
                    cx,
                );
                workspace.split_item(workspace::SplitDirection::Right, Box::new(view.clone()), cx);
                cx.notify();
            }
        });
    }

    pub fn new(
        active_editor: View<Editor>,
        workspace: WeakView<Workspace>,
        tab_description: Option<SharedString>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Workspace>,
    ) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let view = cx.view().downgrade();
            let editor = active_editor.read(cx);
            let file_location = MarkdownPreviewView::get_folder_for_active_editor(editor, cx);
            let contents = editor.buffer().read(cx).snapshot(cx).text();

            let language_registry_copy = language_registry.clone();
            cx.spawn(|view, mut cx| async move {
                let contents =
                    parse_markdown(&contents, file_location, Some(language_registry_copy)).await;

                view.update(&mut cx, |view, cx| {
                    let markdown_blocks_count = contents.children.len();
                    view.contents = Some(contents);
                    view.list_state.reset(markdown_blocks_count);
                    cx.notify();
                })
            })
            .detach();

            cx.subscribe(
                &active_editor,
                move |this, editor, event: &EditorEvent, cx| {
                    match event {
                        EditorEvent::Edited => {
                            let editor = editor.read(cx);
                            let contents = editor.buffer().read(cx).snapshot(cx).text();
                            let file_location =
                                MarkdownPreviewView::get_folder_for_active_editor(editor, cx);
                            let language_registry = language_registry.clone();
                            cx.spawn(move |view, mut cx| async move {
                                let contents = parse_markdown(
                                    &contents,
                                    file_location,
                                    Some(language_registry.clone()),
                                )
                                .await;
                                view.update(&mut cx, move |view, cx| {
                                    let markdown_blocks_count = contents.children.len();
                                    view.contents = Some(contents);

                                    let scroll_top = view.list_state.logical_scroll_top();
                                    view.list_state.reset(markdown_blocks_count);
                                    view.list_state.scroll_to(scroll_top);
                                    cx.notify();
                                })
                            })
                            .detach();
                        }
                        EditorEvent::SelectionsChanged { .. } => {
                            let editor = editor.read(cx);
                            let selection_range = editor.selections.last::<usize>(cx).range();
                            this.selected_block =
                                this.get_block_index_under_cursor(selection_range);
                            this.list_state.scroll_to_reveal_item(this.selected_block);
                            cx.notify();
                        }
                        _ => {}
                    };
                },
            )
            .detach();

            let list_state =
                ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |view, cx| {
                            let Some(contents) = &view.contents else {
                                return div().into_any();
                            };
                            let mut render_cx =
                                RenderContext::new(Some(view.workspace.clone()), cx);
                            let block = contents.children.get(ix).unwrap();
                            let block = render_markdown_block(block, &mut render_cx);
                            let block = div().child(block).pl_4().pb_3();

                            if ix == view.selected_block {
                                let indicator = div()
                                    .h_full()
                                    .w(px(4.0))
                                    .bg(cx.theme().colors().border)
                                    .rounded_sm();

                                return div()
                                    .relative()
                                    .child(block)
                                    .child(indicator.absolute().left_0().top_0())
                                    .into_any();
                            }

                            block.into_any()
                        })
                    } else {
                        div().into_any()
                    }
                });

            let tab_description = tab_description
                .map(|tab_description| format!("Preview {}", tab_description))
                .unwrap_or("Markdown preview".to_string());

            Self {
                selected_block: 0,
                focus_handle: cx.focus_handle(),
                workspace,
                contents: None,
                list_state,
                tab_description,
            }
        })
    }

    /// The absolute path of the file that is currently being previewed.
    fn get_folder_for_active_editor(
        editor: &Editor,
        cx: &ViewContext<MarkdownPreviewView>,
    ) -> Option<PathBuf> {
        if let Some(file) = editor.file_at(0, cx) {
            if let Some(file) = file.as_local() {
                file.abs_path(cx).parent().map(|p| p.to_path_buf())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_block_index_under_cursor(&self, selection_range: Range<usize>) -> usize {
        let mut block_index = None;
        let cursor = selection_range.start;

        let mut last_end = 0;
        if let Some(content) = &self.contents {
            for (i, block) in content.children.iter().enumerate() {
                let Range { start, end } = block.source_range();

                // Check if the cursor is between the last block and the current block
                if last_end > cursor && cursor < start {
                    block_index = Some(i.saturating_sub(1));
                    break;
                }

                if start <= cursor && end >= cursor {
                    block_index = Some(i);
                    break;
                }
                last_end = end;
            }

            if block_index.is_none() && last_end < cursor {
                block_index = Some(content.children.len().saturating_sub(1));
            }
        }

        block_index.unwrap_or_default()
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
            .child(
                Label::new(self.tab_description.to_string()).color(if selected {
                    Color::Default
                } else {
                    Color::Muted
                }),
            )
            .into_any()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("markdown preview")
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(workspace::item::ItemEvent)) {}
}

impl Render for MarkdownPreviewView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("MarkdownPreview")
            .key_context("MarkdownPreview")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .p_4()
            .child(
                div()
                    .flex_grow()
                    .map(|this| this.child(list(self.list_state.clone()).size_full())),
            )
    }
}
