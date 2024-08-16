use std::sync::Arc;
use std::time::Duration;
use std::{ops::Range, path::PathBuf};

use anyhow::Result;
use editor::scroll::{Autoscroll, AutoscrollStrategy};
use editor::{Editor, EditorEvent};
use gpui::{
    list, AppContext, ClickEvent, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    IntoElement, ListState, ParentElement, Render, Styled, Subscription, Task, View, ViewContext,
    WeakView,
};
use language::LanguageRegistry;
use ui::prelude::*;
use workspace::item::{Item, ItemHandle};
use workspace::{Pane, Workspace};

use crate::markdown_elements::ParsedMarkdownElement;
use crate::OpenPreviewToTheSide;
use crate::{
    markdown_elements::ParsedMarkdown,
    markdown_parser::parse_markdown,
    markdown_renderer::{render_markdown_block, RenderContext},
    OpenPreview,
};

const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub struct MarkdownPreviewView {
    workspace: WeakView<Workspace>,
    active_editor: Option<EditorState>,
    focus_handle: FocusHandle,
    contents: Option<ParsedMarkdown>,
    selected_block: usize,
    list_state: ListState,
    tab_description: Option<String>,
    fallback_tab_description: SharedString,
    language_registry: Arc<LanguageRegistry>,
    parsing_markdown_task: Option<Task<Result<()>>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MarkdownPreviewMode {
    /// The preview will always show the contents of the provided editor.
    Default,
    /// The preview will "follow" the currently active editor.
    Follow,
}

struct EditorState {
    editor: View<Editor>,
    _subscription: Subscription,
}

impl MarkdownPreviewView {
    pub fn register(workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                let view = Self::create_markdown_view(workspace, editor, cx);
                workspace.active_pane().update(cx, |pane, cx| {
                    if let Some(existing_view_idx) = Self::find_existing_preview_item_idx(pane) {
                        pane.activate_item(existing_view_idx, true, true, cx);
                    } else {
                        pane.add_item(Box::new(view.clone()), true, true, None, cx)
                    }
                });
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                let view = Self::create_markdown_view(workspace, editor.clone(), cx);
                let pane = workspace
                    .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                    .unwrap_or_else(|| {
                        workspace.split_pane(
                            workspace.active_pane().clone(),
                            workspace::SplitDirection::Right,
                            cx,
                        )
                    });
                pane.update(cx, |pane, cx| {
                    if let Some(existing_view_idx) = Self::find_existing_preview_item_idx(pane) {
                        pane.activate_item(existing_view_idx, true, true, cx);
                    } else {
                        pane.add_item(Box::new(view.clone()), false, false, None, cx)
                    }
                });
                editor.focus_handle(cx).focus(cx);
                cx.notify();
            }
        });
    }

    fn find_existing_preview_item_idx(pane: &Pane) -> Option<usize> {
        pane.items_of_type::<MarkdownPreviewView>()
            .nth(0)
            .and_then(|view| pane.index_for_item(&view))
    }

    pub fn resolve_active_item_as_markdown_editor(
        workspace: &Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<View<Editor>> {
        if let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            if Self::is_markdown_file(&editor, cx) {
                return Some(editor);
            }
        }
        None
    }

    fn create_markdown_view(
        workspace: &mut Workspace,
        editor: View<Editor>,
        cx: &mut ViewContext<Workspace>,
    ) -> View<MarkdownPreviewView> {
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        MarkdownPreviewView::new(
            MarkdownPreviewMode::Follow,
            editor,
            workspace_handle,
            language_registry,
            None,
            cx,
        )
    }

    pub fn new(
        mode: MarkdownPreviewMode,
        active_editor: View<Editor>,
        workspace: WeakView<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        fallback_description: Option<SharedString>,
        cx: &mut ViewContext<Workspace>,
    ) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let view = cx.view().downgrade();

            let list_state =
                ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |this, cx| {
                            let Some(contents) = &this.contents else {
                                return div().into_any();
                            };

                            let mut render_cx =
                                RenderContext::new(Some(this.workspace.clone()), cx)
                                    .with_checkbox_clicked_callback({
                                        let view = view.clone();
                                        move |checked, source_range, cx| {
                                            view.update(cx, |view, cx| {
                                                if let Some(editor) = view
                                                    .active_editor
                                                    .as_ref()
                                                    .map(|s| s.editor.clone())
                                                {
                                                    editor.update(cx, |editor, cx| {
                                                        let task_marker =
                                                            if checked { "[x]" } else { "[ ]" };

                                                        editor.edit(
                                                            vec![(source_range, task_marker)],
                                                            cx,
                                                        );
                                                    });
                                                    view.parse_markdown_from_active_editor(
                                                        false, cx,
                                                    );
                                                    cx.notify();
                                                }
                                            })
                                        }
                                    });
                            let block = contents.children.get(ix).unwrap();
                            let rendered_block = render_markdown_block(block, &mut render_cx);

                            let should_apply_padding = Self::should_apply_padding_between(
                                block,
                                contents.children.get(ix + 1),
                            );

                            div()
                                .id(ix)
                                .when(should_apply_padding, |this| this.pb_3())
                                .group("markdown-block")
                                .on_click(cx.listener(move |this, event: &ClickEvent, cx| {
                                    if event.down.click_count == 2 {
                                        if let Some(block) =
                                            this.contents.as_ref().and_then(|c| c.children.get(ix))
                                        {
                                            let start = block.source_range().start;
                                            this.move_cursor_to_block(cx, start..start);
                                        }
                                    }
                                }))
                                .map(move |container| {
                                    let indicator = div()
                                        .h_full()
                                        .w(px(4.0))
                                        .when(ix == this.selected_block, |this| {
                                            this.bg(cx.theme().colors().border)
                                        })
                                        .group_hover("markdown-block", |s| {
                                            if ix == this.selected_block {
                                                s
                                            } else {
                                                s.bg(cx.theme().colors().border_variant)
                                            }
                                        })
                                        .rounded_sm();

                                    container.child(
                                        div()
                                            .relative()
                                            .child(div().pl_4().child(rendered_block))
                                            .child(indicator.absolute().left_0().top_0()),
                                    )
                                })
                                .into_any()
                        })
                    } else {
                        div().into_any()
                    }
                });

            let mut this = Self {
                selected_block: 0,
                active_editor: None,
                focus_handle: cx.focus_handle(),
                workspace: workspace.clone(),
                contents: None,
                list_state,
                tab_description: None,
                language_registry,
                fallback_tab_description: fallback_description
                    .unwrap_or_else(|| "Markdown Preview".into()),
                parsing_markdown_task: None,
            };

            this.set_editor(active_editor, cx);

            if mode == MarkdownPreviewMode::Follow {
                if let Some(workspace) = &workspace.upgrade() {
                    cx.observe(workspace, |this, workspace, cx| {
                        let item = workspace.read(cx).active_item(cx);
                        this.workspace_updated(item, cx);
                    })
                    .detach();
                } else {
                    log::error!("Failed to listen to workspace updates");
                }
            }

            this
        })
    }

    fn workspace_updated(
        &mut self,
        active_item: Option<Box<dyn ItemHandle>>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(item) = active_item {
            if item.item_id() != cx.entity_id() {
                if let Some(editor) = item.act_as::<Editor>(cx) {
                    if Self::is_markdown_file(&editor, cx) {
                        self.set_editor(editor, cx);
                    }
                }
            }
        }
    }

    pub fn is_markdown_file<V>(editor: &View<Editor>, cx: &mut ViewContext<V>) -> bool {
        let buffer = editor.read(cx).buffer().read(cx);
        if let Some(buffer) = buffer.as_singleton() {
            if let Some(language) = buffer.read(cx).language() {
                return language.name().as_ref() == "Markdown";
            }
        }
        false
    }

    fn set_editor(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        if let Some(active) = &self.active_editor {
            if active.editor == editor {
                return;
            }
        }

        let subscription = cx.subscribe(&editor, |this, editor, event: &EditorEvent, cx| {
            match event {
                EditorEvent::Edited { .. } => {
                    this.parse_markdown_from_active_editor(true, cx);
                }
                EditorEvent::SelectionsChanged { .. } => {
                    let editor = editor.read(cx);
                    let selection_range = editor.selections.last::<usize>(cx).range();
                    this.selected_block = this.get_block_index_under_cursor(selection_range);
                    this.list_state.scroll_to_reveal_item(this.selected_block);
                    cx.notify();
                }
                _ => {}
            };
        });

        self.tab_description = editor
            .read(cx)
            .tab_description(0, cx)
            .map(|tab_description| format!("Preview {}", tab_description));

        self.active_editor = Some(EditorState {
            editor,
            _subscription: subscription,
        });

        self.parse_markdown_from_active_editor(false, cx);
    }

    fn parse_markdown_from_active_editor(
        &mut self,
        wait_for_debounce: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(state) = &self.active_editor {
            self.parsing_markdown_task = Some(self.parse_markdown_in_background(
                wait_for_debounce,
                state.editor.clone(),
                cx,
            ));
        }
    }

    fn parse_markdown_in_background(
        &mut self,
        wait_for_debounce: bool,
        editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let language_registry = self.language_registry.clone();

        cx.spawn(move |view, mut cx| async move {
            if wait_for_debounce {
                // Wait for the user to stop typing
                cx.background_executor().timer(REPARSE_DEBOUNCE).await;
            }

            let (contents, file_location) = view.update(&mut cx, |_, cx| {
                let editor = editor.read(cx);
                let contents = editor.buffer().read(cx).snapshot(cx).text();
                let file_location = MarkdownPreviewView::get_folder_for_active_editor(editor, cx);
                (contents, file_location)
            })?;

            let parsing_task = cx.background_executor().spawn(async move {
                parse_markdown(&contents, file_location, Some(language_registry)).await
            });
            let contents = parsing_task.await;
            view.update(&mut cx, move |view, cx| {
                let markdown_blocks_count = contents.children.len();
                view.contents = Some(contents);
                let scroll_top = view.list_state.logical_scroll_top();
                view.list_state.reset(markdown_blocks_count);
                view.list_state.scroll_to(scroll_top);
                cx.notify();
            })
        })
    }

    fn move_cursor_to_block(&self, cx: &mut ViewContext<Self>, selection: Range<usize>) {
        if let Some(state) = &self.active_editor {
            state.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Center)),
                    cx,
                    |selections| selections.select_ranges(vec![selection]),
                );
                editor.focus(cx);
            });
        }
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
                if last_end <= cursor && cursor < start {
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

    fn should_apply_padding_between(
        current_block: &ParsedMarkdownElement,
        next_block: Option<&ParsedMarkdownElement>,
    ) -> bool {
        !(current_block.is_list_item() && next_block.map(|b| b.is_list_item()).unwrap_or(false))
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

    fn tab_icon(&self, _cx: &WindowContext) -> Option<Icon> {
        Some(Icon::new(IconName::FileDoc))
    }

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some(if let Some(description) = &self.tab_description {
            description.clone().into()
        } else {
            self.fallback_tab_description.clone()
        })
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
