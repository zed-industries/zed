use std::cmp::min;
use std::sync::Arc;
use std::time::Duration;
use std::{ops::Range, path::PathBuf};

use anyhow::Result;
use editor::scroll::Autoscroll;
use editor::{Editor, EditorEvent, MultiBufferOffset, SelectionEffects};
use fs::normalize_path;
use gpui::{
    App, ClickEvent, Context, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    IntoElement, IsZero, ListState, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement, Render, RetainAllImageCache, Styled, Subscription, Task, WeakEntity, Window,
    list,
};
use language::CharClassifier;
use language::LanguageRegistry;
use settings::Settings;
use theme::ThemeSettings;
use ui::{WithScrollbar, prelude::*};
use workspace::item::{Item, ItemHandle};
use workspace::{Pane, Workspace};

use crate::markdown_elements::ParsedMarkdownElement;
use crate::markdown_renderer::{CheckboxClickedEvent, MermaidState};
use crate::{
    OpenFollowingPreview, OpenPreview, OpenPreviewToTheSide, ScrollPageDown, ScrollPageUp,
    markdown_elements::ParsedMarkdown,
    markdown_parser::parse_markdown,
    markdown_renderer::{RenderContext, render_markdown_block},
};
use crate::{ScrollDown, ScrollDownByItem, ScrollUp, ScrollUpByItem};

const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub struct MarkdownPreviewView {
    workspace: WeakEntity<Workspace>,
    image_cache: Entity<RetainAllImageCache>,
    active_editor: Option<EditorState>,
    focus_handle: FocusHandle,
    contents: Option<ParsedMarkdown>,
    selected_block: usize,
    list_state: ListState,
    language_registry: Arc<LanguageRegistry>,
    mermaid_state: MermaidState,
    parsing_markdown_task: Option<Task<Result<()>>>,
    mode: MarkdownPreviewMode,
    preview_text_index: Option<PreviewTextIndex>,
    preview_selection: Option<PreviewSelection>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MarkdownPreviewMode {
    /// The preview will always show the contents of the provided editor.
    Default,
    /// The preview will "follow" the currently active editor.
    Follow,
}

struct EditorState {
    editor: Entity<Editor>,
    _subscription: Subscription,
}

#[derive(Clone, Debug, Default)]
struct PreviewTextIndex {
    text: String,
    block_ranges: Vec<Range<usize>>,
    chunk_ranges: Vec<Range<usize>>,
}

#[derive(Clone, Debug)]
struct PreviewSelection {
    start: usize,
    end: usize,
    reversed: bool,
    dragging: bool,
    dragged: bool,
    mode: PreviewSelectMode,
}

#[derive(Clone, Debug)]
enum PreviewSelectMode {
    Character,
    Word(Range<usize>),
    Chunk(Range<usize>),
}

impl Default for PreviewSelection {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            reversed: false,
            dragging: false,
            dragged: false,
            mode: PreviewSelectMode::Character,
        }
    }
}

impl MarkdownPreviewView {
    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                let view = Self::create_markdown_view(workspace, editor.clone(), window, cx);
                workspace.active_pane().update(cx, |pane, cx| {
                    if let Some(existing_view_idx) =
                        Self::find_existing_independent_preview_item_idx(pane, &editor, cx)
                    {
                        pane.activate_item(existing_view_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view.clone()), true, true, None, window, cx)
                    }
                });
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                let view = Self::create_markdown_view(workspace, editor.clone(), window, cx);
                let pane = workspace
                    .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                    .unwrap_or_else(|| {
                        workspace.split_pane(
                            workspace.active_pane().clone(),
                            workspace::SplitDirection::Right,
                            window,
                            cx,
                        )
                    });
                pane.update(cx, |pane, cx| {
                    if let Some(existing_view_idx) =
                        Self::find_existing_independent_preview_item_idx(pane, &editor, cx)
                    {
                        pane.activate_item(existing_view_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view.clone()), false, false, None, window, cx)
                    }
                });
                editor.focus_handle(cx).focus(window, cx);
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenFollowingPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                // Check if there's already a following preview
                let existing_follow_view_idx = {
                    let active_pane = workspace.active_pane().read(cx);
                    active_pane
                        .items_of_type::<MarkdownPreviewView>()
                        .find(|view| view.read(cx).mode == MarkdownPreviewMode::Follow)
                        .and_then(|view| active_pane.index_for_item(&view))
                };

                if let Some(existing_follow_view_idx) = existing_follow_view_idx {
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.activate_item(existing_follow_view_idx, true, true, window, cx);
                    });
                } else {
                    let view = Self::create_following_markdown_view(workspace, editor, window, cx);
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.add_item(Box::new(view.clone()), true, true, None, window, cx)
                    });
                }
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &crate::CopyRichSelection, window, cx| {
            if let Some(view) = workspace.active_item_as::<MarkdownPreviewView>(cx) {
                view.update(cx, |this, cx| {
                    this.copy_rich(window, cx);
                });
            }
        });
    }

    fn copy_rich(&self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(contents) = &self.contents
            && let Some(selection) = self.current_selection_range()
            && let Some(exported) = crate::markdown_to_html::export_selection(contents, selection)
        {
            cx.write_to_clipboard(gpui::ClipboardItem::new_rich_text(
                exported.plain_text,
                exported.html,
            ));
        }
    }

    fn find_existing_independent_preview_item_idx(
        pane: &Pane,
        editor: &Entity<Editor>,
        cx: &App,
    ) -> Option<usize> {
        pane.items_of_type::<MarkdownPreviewView>()
            .find(|view| {
                let view_read = view.read(cx);
                // Only look for independent (Default mode) previews, not Follow previews
                view_read.mode == MarkdownPreviewMode::Default
                    && view_read
                        .active_editor
                        .as_ref()
                        .is_some_and(|active_editor| active_editor.editor == *editor)
            })
            .and_then(|view| pane.index_for_item(&view))
    }

    pub fn resolve_active_item_as_markdown_editor(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<Editor>> {
        if let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            && Self::is_markdown_file(&editor, cx)
        {
            return Some(editor);
        }
        None
    }

    fn create_markdown_view(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<MarkdownPreviewView> {
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        MarkdownPreviewView::new(
            MarkdownPreviewMode::Default,
            editor,
            workspace_handle,
            language_registry,
            window,
            cx,
        )
    }

    fn create_following_markdown_view(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<MarkdownPreviewView> {
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        MarkdownPreviewView::new(
            MarkdownPreviewMode::Follow,
            editor,
            workspace_handle,
            language_registry,
            window,
            cx,
        )
    }

    pub fn new(
        mode: MarkdownPreviewMode,
        active_editor: Entity<Editor>,
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let list_state = ListState::new(0, gpui::ListAlignment::Top, px(1000.));

            let mut this = Self {
                selected_block: 0,
                active_editor: None,
                focus_handle: cx.focus_handle(),
                workspace: workspace.clone(),
                contents: None,
                list_state,
                language_registry,
                mermaid_state: Default::default(),
                parsing_markdown_task: None,
                image_cache: RetainAllImageCache::new(cx),
                mode,
                preview_text_index: None,
                preview_selection: None,
            };

            this.set_editor(active_editor, window, cx);

            if mode == MarkdownPreviewMode::Follow {
                if let Some(workspace) = &workspace.upgrade() {
                    cx.observe_in(workspace, window, |this, workspace, window, cx| {
                        let item = workspace.read(cx).active_item(cx);
                        this.workspace_updated(item, window, cx);
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(item) = active_item
            && item.item_id() != cx.entity_id()
            && let Some(editor) = item.act_as::<Editor>(cx)
            && Self::is_markdown_file(&editor, cx)
        {
            self.set_editor(editor, window, cx);
        }
    }

    pub fn is_markdown_file<V>(editor: &Entity<Editor>, cx: &mut Context<V>) -> bool {
        let buffer = editor.read(cx).buffer().read(cx);
        if let Some(buffer) = buffer.as_singleton()
            && let Some(language) = buffer.read(cx).language()
        {
            return language.name() == "Markdown";
        }
        false
    }

    fn set_editor(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active) = &self.active_editor
            && active.editor == editor
        {
            return;
        }

        let subscription = cx.subscribe_in(
            &editor,
            window,
            |this, editor, event: &EditorEvent, window, cx| {
                match event {
                    EditorEvent::Edited { .. }
                    | EditorEvent::DirtyChanged
                    | EditorEvent::ExcerptsEdited { .. } => {
                        this.parse_markdown_from_active_editor(true, window, cx);
                    }
                    EditorEvent::SelectionsChanged { .. } => {
                        let selection_range = editor.update(cx, |editor, cx| {
                            editor
                                .selections
                                .last::<MultiBufferOffset>(&editor.display_snapshot(cx))
                                .range()
                        });
                        this.selected_block = this.get_block_index_under_cursor(selection_range);
                        this.list_state.scroll_to_reveal_item(this.selected_block);
                        cx.notify();
                    }
                    _ => {}
                };
            },
        );

        self.active_editor = Some(EditorState {
            editor,
            _subscription: subscription,
        });

        self.parse_markdown_from_active_editor(false, window, cx);
    }

    fn parse_markdown_from_active_editor(
        &mut self,
        wait_for_debounce: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(state) = &self.active_editor {
            // if there is already a task to update the ui and the current task is also debounced (not high priority), do nothing
            if wait_for_debounce && self.parsing_markdown_task.is_some() {
                return;
            }
            self.parsing_markdown_task = Some(self.parse_markdown_in_background(
                wait_for_debounce,
                state.editor.clone(),
                window,
                cx,
            ));
        }
    }

    fn parse_markdown_in_background(
        &mut self,
        wait_for_debounce: bool,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let language_registry = self.language_registry.clone();

        cx.spawn_in(window, async move |view, cx| {
            if wait_for_debounce {
                // Wait for the user to stop typing
                cx.background_executor().timer(REPARSE_DEBOUNCE).await;
            }

            let (contents, file_location) = view.update(cx, |_, cx| {
                let editor = editor.read(cx);
                let contents = editor.buffer().read(cx).snapshot(cx).text();
                let file_location = MarkdownPreviewView::get_folder_for_active_editor(editor, cx);
                (contents, file_location)
            })?;

            let parsing_task = cx.background_spawn(async move {
                parse_markdown(&contents, file_location, Some(language_registry)).await
            });
            let contents = parsing_task.await;

            view.update(cx, move |view, cx| {
                view.mermaid_state.update(&contents, cx);
                let markdown_blocks_count = contents.children.len();
                view.preview_text_index = Some(PreviewTextIndex::from_markdown(&contents));
                view.preview_selection = None;
                view.contents = Some(contents);
                let scroll_top = view.list_state.logical_scroll_top();
                view.list_state.reset(markdown_blocks_count);
                view.list_state.scroll_to(scroll_top);
                view.parsing_markdown_task = None;
                cx.notify();
            })
        })
    }

    fn move_cursor_to_block(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        selection: Range<MultiBufferOffset>,
    ) {
        if let Some(state) = &self.active_editor {
            state.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::center()),
                    window,
                    cx,
                    |selections| selections.select_ranges(vec![selection]),
                );
                window.focus(&editor.focus_handle(cx), cx);
            });
        }
    }

    /// The absolute path of the file that is currently being previewed.
    fn get_folder_for_active_editor(editor: &Editor, cx: &App) -> Option<PathBuf> {
        if let Some(file) = editor.file_at(MultiBufferOffset(0), cx) {
            if let Some(file) = file.as_local() {
                file.abs_path(cx).parent().map(|p| p.to_path_buf())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_block_index_under_cursor(&self, selection_range: Range<MultiBufferOffset>) -> usize {
        let mut block_index = None;
        let cursor = selection_range.start.0;

        let mut last_end = 0;
        if let Some(content) = &self.contents {
            for (i, block) in content.children.iter().enumerate() {
                let Some(Range { start, end }) = block.source_range() else {
                    continue;
                };

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

    fn scroll_page_up(&mut self, _: &ScrollPageUp, _window: &mut Window, cx: &mut Context<Self>) {
        let viewport_height = self.list_state.viewport_bounds().size.height;
        if viewport_height.is_zero() {
            return;
        }

        self.list_state.scroll_by(-viewport_height);
        cx.notify();
    }

    fn scroll_page_down(
        &mut self,
        _: &ScrollPageDown,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let viewport_height = self.list_state.viewport_bounds().size.height;
        if viewport_height.is_zero() {
            return;
        }

        self.list_state.scroll_by(viewport_height);
        cx.notify();
    }

    fn scroll_up(&mut self, _: &ScrollUp, window: &mut Window, cx: &mut Context<Self>) {
        let scroll_top = self.list_state.logical_scroll_top();
        if let Some(bounds) = self.list_state.bounds_for_item(scroll_top.item_ix) {
            let item_height = bounds.size.height;
            // Scroll no more than the rough equivalent of a large headline
            let max_height = window.rem_size() * 2;
            let scroll_height = min(item_height, max_height);
            self.list_state.scroll_by(-scroll_height);
        }
        cx.notify();
    }

    fn scroll_down(&mut self, _: &ScrollDown, window: &mut Window, cx: &mut Context<Self>) {
        let scroll_top = self.list_state.logical_scroll_top();
        if let Some(bounds) = self.list_state.bounds_for_item(scroll_top.item_ix) {
            let item_height = bounds.size.height;
            // Scroll no more than the rough equivalent of a large headline
            let max_height = window.rem_size() * 2;
            let scroll_height = min(item_height, max_height);
            self.list_state.scroll_by(scroll_height);
        }
        cx.notify();
    }

    fn scroll_up_by_item(
        &mut self,
        _: &ScrollUpByItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let scroll_top = self.list_state.logical_scroll_top();
        if let Some(bounds) = self.list_state.bounds_for_item(scroll_top.item_ix) {
            self.list_state.scroll_by(-bounds.size.height);
        }
        cx.notify();
    }

    fn scroll_down_by_item(
        &mut self,
        _: &ScrollDownByItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let scroll_top = self.list_state.logical_scroll_top();
        if let Some(bounds) = self.list_state.bounds_for_item(scroll_top.item_ix) {
            self.list_state.scroll_by(bounds.size.height);
        }
        cx.notify();
    }

    fn current_selection_range(&self) -> Option<Range<usize>> {
        self.preview_selection.as_ref().and_then(|selection| {
            (selection.end > selection.start).then_some(selection.start..selection.end)
        })
    }

    fn should_follow_text_clicks(&self) -> bool {
        self.preview_selection
            .as_ref()
            .is_none_or(|selection| !selection.dragged)
    }

    fn begin_text_selection(
        &mut self,
        index: usize,
        chunk_range: Range<usize>,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_handle(cx).focus(window, cx);

        let mut selection = PreviewSelection {
            start: index,
            end: index,
            reversed: false,
            dragging: event.button == MouseButton::Left,
            dragged: false,
            mode: PreviewSelectMode::Character,
        };

        match event.click_count {
            2 => {
                if let Some(text_index) = &self.preview_text_index {
                    let word = surrounding_word_range(&text_index.text, index);
                    selection.start = word.start;
                    selection.end = word.end;
                    selection.mode = PreviewSelectMode::Word(word);
                }
            }
            3.. => {
                selection.start = chunk_range.start;
                selection.end = chunk_range.end;
                selection.mode = PreviewSelectMode::Chunk(chunk_range);
            }
            _ => {}
        }

        self.preview_selection = Some(selection);
        cx.notify();
    }

    fn hover_text_selection(
        &mut self,
        index: Option<usize>,
        _event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = index else {
            return;
        };
        let Some(selection) = self.preview_selection.as_mut() else {
            return;
        };
        if !selection.dragging {
            return;
        }

        let previous = selection.start..selection.end;
        selection.dragged |= selection.start != index && selection.end != index;
        selection.set_head(index, self.preview_text_index.as_ref());
        if previous != (selection.start..selection.end) {
            cx.notify();
        }
    }

    fn finish_text_selection(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(selection) = self.preview_selection.as_mut()
            && selection.dragging
        {
            selection.dragging = false;
            cx.notify();
        }
    }
}

impl PreviewSelection {
    fn tail(&self) -> usize {
        if self.reversed { self.end } else { self.start }
    }

    fn set_head(&mut self, head: usize, index: Option<&PreviewTextIndex>) {
        match &self.mode {
            PreviewSelectMode::Character => {
                if head < self.tail() {
                    if !self.reversed {
                        self.end = self.start;
                        self.reversed = true;
                    }
                    self.start = head;
                } else {
                    if self.reversed {
                        self.start = self.end;
                        self.reversed = false;
                    }
                    self.end = head;
                }
            }
            PreviewSelectMode::Word(original_range) => {
                let Some(index) = index else {
                    return;
                };
                let head_range = surrounding_word_range(&index.text, head);
                if head < original_range.start {
                    self.start = head_range.start;
                    self.end = original_range.end;
                    self.reversed = true;
                } else if head >= original_range.end {
                    self.start = original_range.start;
                    self.end = head_range.end;
                    self.reversed = false;
                } else {
                    self.start = original_range.start;
                    self.end = original_range.end;
                    self.reversed = false;
                }
            }
            PreviewSelectMode::Chunk(original_range) => {
                let Some(index) = index else {
                    return;
                };
                let head_range = index.chunk_range_containing(head).unwrap_or(head..head);
                if head < original_range.start {
                    self.start = head_range.start;
                    self.end = original_range.end;
                    self.reversed = true;
                } else if head >= original_range.end {
                    self.start = original_range.start;
                    self.end = head_range.end;
                    self.reversed = false;
                } else {
                    self.start = original_range.start;
                    self.end = original_range.end;
                    self.reversed = false;
                }
            }
        }
    }
}

impl PreviewTextIndex {
    fn from_markdown(markdown: &ParsedMarkdown) -> Self {
        let mut this = Self::default();
        let mut cursor = 0;

        for block in &markdown.children {
            let start = cursor;
            collect_selectable_text(block, &mut cursor, &mut this.text, &mut this.chunk_ranges);
            this.block_ranges.push(start..cursor);
        }

        this
    }

    fn chunk_range_containing(&self, index: usize) -> Option<Range<usize>> {
        self.chunk_ranges
            .iter()
            .find(|range| {
                range.contains(&index)
                    || (index == range.end
                        && self
                            .chunk_ranges
                            .last()
                            .is_some_and(|last_range| last_range == *range))
            })
            .cloned()
    }
}

fn collect_selectable_text(
    block: &ParsedMarkdownElement,
    cursor: &mut usize,
    text: &mut String,
    chunk_ranges: &mut Vec<Range<usize>>,
) {
    use crate::markdown_elements::{MarkdownParagraphChunk, ParsedMarkdownElement};

    let mut push_chunk = |contents: &str| {
        if contents.is_empty() {
            return;
        }
        let range = *cursor..*cursor + contents.len();
        *cursor = range.end;
        text.push_str(contents);
        chunk_ranges.push(range);
    };

    match block {
        ParsedMarkdownElement::Paragraph(chunks) => {
            for chunk in chunks {
                if let MarkdownParagraphChunk::Text(parsed) = chunk {
                    push_chunk(parsed.contents.as_ref());
                }
            }
        }
        ParsedMarkdownElement::Heading(heading) => {
            for chunk in &heading.contents {
                if let MarkdownParagraphChunk::Text(parsed) = chunk {
                    push_chunk(parsed.contents.as_ref());
                }
            }
        }
        ParsedMarkdownElement::ListItem(item) => {
            for block in &item.content {
                collect_selectable_text(block, cursor, text, chunk_ranges);
            }
        }
        ParsedMarkdownElement::Table(table) => {
            if let Some(caption) = &table.caption {
                for chunk in caption {
                    if let MarkdownParagraphChunk::Text(parsed) = chunk {
                        push_chunk(parsed.contents.as_ref());
                    }
                }
            }
            for row in table.header.iter().chain(table.body.iter()) {
                for column in &row.columns {
                    for chunk in &column.children {
                        if let MarkdownParagraphChunk::Text(parsed) = chunk {
                            push_chunk(parsed.contents.as_ref());
                        }
                    }
                }
            }
        }
        ParsedMarkdownElement::BlockQuote(block_quote) => {
            for block in &block_quote.children {
                collect_selectable_text(block, cursor, text, chunk_ranges);
            }
        }
        ParsedMarkdownElement::CodeBlock(code_block) => push_chunk(code_block.contents.as_ref()),
        ParsedMarkdownElement::HorizontalRule(_) | ParsedMarkdownElement::Image(_) => {}
    }
}

fn surrounding_word_range(text: &str, index: usize) -> Range<usize> {
    if text.is_empty() {
        return 0..0;
    }

    let index = index.min(text.len().saturating_sub(1));
    let classifier = CharClassifier::new(None);

    let mut prev_chars = text[..index].chars().rev().peekable();
    let mut next_chars = text[index..].chars().peekable();
    let word_kind = std::cmp::max(
        prev_chars.peek().map(|&ch| classifier.kind(ch)),
        next_chars.peek().map(|&ch| classifier.kind(ch)),
    );

    let mut start = index;
    for ch in prev_chars {
        if Some(classifier.kind(ch)) == word_kind {
            start -= ch.len_utf8();
        } else {
            break;
        }
    }

    let mut end = index;
    for ch in next_chars {
        if Some(classifier.kind(ch)) == word_kind {
            end += ch.len_utf8();
        } else {
            break;
        }
    }

    start..end
}

impl Focusable for MarkdownPreviewView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for MarkdownPreviewView {}

impl Item for MarkdownPreviewView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileDoc))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.active_editor
            .as_ref()
            .map(|editor_state| {
                let buffer = editor_state.editor.read(cx).buffer().read(cx);
                let title = buffer.title(cx);
                format!("Preview {}", title).into()
            })
            .unwrap_or_else(|| SharedString::from("Markdown Preview"))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Markdown Preview Opened")
    }

    fn to_item_events(_event: &Self::Event, _f: &mut dyn FnMut(workspace::item::ItemEvent)) {}
}

impl Render for MarkdownPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let buffer_size = ThemeSettings::get_global(cx).buffer_font_size(cx);
        let buffer_line_height = ThemeSettings::get_global(cx).buffer_line_height;

        v_flex()
            .image_cache(self.image_cache.clone())
            .id("MarkdownPreview")
            .key_context("MarkdownPreview")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(MarkdownPreviewView::scroll_page_up))
            .on_action(cx.listener(MarkdownPreviewView::scroll_page_down))
            .on_action(cx.listener(MarkdownPreviewView::scroll_up))
            .on_action(cx.listener(MarkdownPreviewView::scroll_down))
            .on_action(cx.listener(MarkdownPreviewView::scroll_up_by_item))
            .on_action(cx.listener(MarkdownPreviewView::scroll_down_by_item))
            .capture_any_mouse_up(cx.listener(MarkdownPreviewView::finish_text_selection))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .p_4()
            .text_size(buffer_size)
            .line_height(relative(buffer_line_height.value()))
            .child(div().flex_grow().map(|this| {
                this.child(
                    list(
                        self.list_state.clone(),
                        cx.processor(|this, ix, window, cx| {
                            let Some(contents) = &this.contents else {
                                return div().into_any();
                            };

                            let weak_view = cx.weak_entity();
                            let mut render_cx = RenderContext::new(
                                Some(this.workspace.clone()),
                                &this.mermaid_state,
                                window,
                                cx,
                            )
                            .with_text_selection(
                                this.current_selection_range(),
                                this.preview_text_index
                                    .as_ref()
                                    .and_then(|index| index.block_ranges.get(ix))
                                    .map(|range: &Range<usize>| range.start)
                                    .unwrap_or_default(),
                                {
                                    let weak_view = weak_view.clone();
                                    move |index, chunk_range: Range<usize>, event: &MouseDownEvent, window, cx| {
                                        weak_view
                                            .update(cx, |this, cx| {
                                                this.begin_text_selection(index, chunk_range, event, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                                {
                                    let weak_view = weak_view.clone();
                                    move |index: Option<usize>, event: &MouseMoveEvent, window, cx| {
                                        weak_view
                                            .update(cx, |this, cx| {
                                                this.hover_text_selection(index, event, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                                {
                                    let weak_view = weak_view.clone();
                                    move |_index, _chunk_range: Range<usize>, event: &MouseUpEvent, window, cx| {
                                        weak_view
                                            .update(cx, |this, cx| {
                                                this.finish_text_selection(event, window, cx);
                                            })
                                            .ok();
                                    }
                                },
                                move |link: &crate::markdown_elements::Link, window, cx| {
                                    weak_view
                                        .update(cx, |this, cx| {
                                            if !this.should_follow_text_clicks() {
                                                return;
                                            }

                                            match link {
                                                crate::markdown_elements::Link::Web { url } => cx.open_url(url),
                                                crate::markdown_elements::Link::Path { path, .. } => {
                                                    if let Some(workspace) = this.workspace.upgrade() {
                                                        _ = workspace.update(cx, |workspace, cx| {
                                                            workspace
                                                                .open_abs_path(
                                                                    normalize_path(path.as_path()),
                                                                    workspace::OpenOptions {
                                                                        visible: Some(workspace::OpenVisible::None),
                                                                        ..Default::default()
                                                                    },
                                                                    window,
                                                                    cx,
                                                                )
                                                                .detach();
                                                        });
                                                    }
                                                }
                                            }
                                        })
                                        .ok();
                                },
                            )
                            .with_checkbox_clicked_callback(cx.listener(
                                move |this, e: &CheckboxClickedEvent, window, cx| {
                                    if let Some(editor) =
                                        this.active_editor.as_ref().map(|s| s.editor.clone())
                                    {
                                        editor.update(cx, |editor, cx| {
                                            let task_marker =
                                                if e.checked() { "[x]" } else { "[ ]" };

                                            editor.edit(
                                                [(
                                                    MultiBufferOffset(e.source_range().start)
                                                        ..MultiBufferOffset(e.source_range().end),
                                                    task_marker,
                                                )],
                                                cx,
                                            );
                                        });
                                        this.parse_markdown_from_active_editor(false, window, cx);
                                        cx.notify();
                                    }
                                },
                            ));

                            let block = contents.children.get(ix).unwrap();
                            let rendered_block = render_markdown_block(block, &mut render_cx);

                            let should_apply_padding = Self::should_apply_padding_between(
                                block,
                                contents.children.get(ix + 1),
                            );

                            let selected_block = this.selected_block;
                            let scaled_rems = render_cx.scaled_rems(1.0);
                            div()
                                .id(ix)
                                .when(should_apply_padding, |this| {
                                    this.pb(render_cx.scaled_rems(0.75))
                                })
                                .group("markdown-block")
                                .on_click(cx.listener(
                                    move |this, event: &ClickEvent, window, cx| {
                                        if event.click_count() == 2
                                            && this.current_selection_range().is_none()
                                            && let Some(source_range) = this
                                                .contents
                                                .as_ref()
                                                .and_then(|c| c.children.get(ix))
                                                .and_then(|block: &ParsedMarkdownElement| {
                                                    block.source_range()
                                                })
                                        {
                                            this.move_cursor_to_block(
                                                window,
                                                cx,
                                                MultiBufferOffset(source_range.start)
                                                    ..MultiBufferOffset(source_range.start),
                                            );
                                        }
                                    },
                                ))
                                .map(move |container| {
                                    let indicator = div()
                                        .h_full()
                                        .w(px(4.0))
                                        .when(ix == selected_block, |this| {
                                            this.bg(cx.theme().colors().border)
                                        })
                                        .group_hover("markdown-block", |s| {
                                            if ix == selected_block {
                                                s
                                            } else {
                                                s.bg(cx.theme().colors().border_variant)
                                            }
                                        })
                                        .rounded_xs();

                                    container.child(
                                        div()
                                            .relative()
                                            .child(div().pl(scaled_rems).child(rendered_block))
                                            .child(indicator.absolute().left_0().top_0()),
                                    )
                                })
                                .into_any()
                        }),
                    )
                    .size_full(),
                )
            }))
            .vertical_scrollbar_for(&self.list_state, window, cx)
    }
}
