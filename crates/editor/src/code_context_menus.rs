use crate::scroll::ScrollAmount;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, Entity, Focusable, FontWeight, ListSizingBehavior, ScrollHandle, ScrollStrategy,
    SharedString, Size, StrikethroughStyle, StyledText, Task, UniformListScrollHandle, div, px,
    uniform_list,
};
use itertools::Itertools;
use language::CodeLabel;
use language::{Buffer, LanguageName, LanguageRegistry};
use markdown::{Markdown, MarkdownElement};
use multi_buffer::{Anchor, ExcerptId};
use ordered_float::OrderedFloat;
use project::lsp_store::CompletionDocumentation;
use project::{CodeAction, Completion, TaskSourceKind};
use project::{CompletionDisplayOptions, CompletionSource};
use task::DebugScenario;
use task::TaskContext;

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    cell::RefCell,
    cmp::{Reverse, min},
    iter,
    ops::Range,
    rc::Rc,
};
use task::ResolvedTask;
use ui::{Color, IntoElement, ListItem, Pixels, Popover, Styled, prelude::*};
use util::ResultExt;

use crate::CodeActionSource;
use crate::editor_settings::SnippetSortOrder;
use crate::hover_popover::{hover_markdown_style, open_markdown_url};
use crate::{
    CodeActionProvider, CompletionId, CompletionItemKind, CompletionProvider, DisplayRow, Editor,
    EditorStyle, ResolvedTasks,
    actions::{ConfirmCodeAction, ConfirmCompletion},
    split_words, styled_runs_for_code_label,
};

pub const MENU_GAP: Pixels = px(4.);
pub const MENU_ASIDE_X_PADDING: Pixels = px(16.);
pub const MENU_ASIDE_MIN_WIDTH: Pixels = px(260.);
pub const MENU_ASIDE_MAX_WIDTH: Pixels = px(500.);

// Constants for the markdown cache. The purpose of this cache is to reduce flickering due to
// documentation not yet being parsed.
//
// The size of the cache is set to 16, which is roughly 3 times more than the number of items
// fetched around the current selection. This way documentation is more often ready for render when
// revisiting previous entries, such as when pressing backspace.
const MARKDOWN_CACHE_MAX_SIZE: usize = 16;
const MARKDOWN_CACHE_BEFORE_ITEMS: usize = 2;
const MARKDOWN_CACHE_AFTER_ITEMS: usize = 2;

// Number of items beyond the visible items to resolve documentation.
const RESOLVE_BEFORE_ITEMS: usize = 4;
const RESOLVE_AFTER_ITEMS: usize = 4;

pub enum CodeContextMenu {
    Completions(CompletionsMenu),
    CodeActions(CodeActionsMenu),
}

impl CodeContextMenu {
    pub fn select_first(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_first(provider, window, cx),
                CodeContextMenu::CodeActions(menu) => menu.select_first(cx),
            }
            true
        } else {
            false
        }
    }

    pub fn select_prev(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_prev(provider, window, cx),
                CodeContextMenu::CodeActions(menu) => menu.select_prev(cx),
            }
            true
        } else {
            false
        }
    }

    pub fn select_next(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_next(provider, window, cx),
                CodeContextMenu::CodeActions(menu) => menu.select_next(cx),
            }
            true
        } else {
            false
        }
    }

    pub fn select_last(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_last(provider, window, cx),
                CodeContextMenu::CodeActions(menu) => menu.select_last(cx),
            }
            true
        } else {
            false
        }
    }

    pub fn visible(&self) -> bool {
        match self {
            CodeContextMenu::Completions(menu) => menu.visible(),
            CodeContextMenu::CodeActions(menu) => menu.visible(),
        }
    }

    pub fn origin(&self) -> ContextMenuOrigin {
        match self {
            CodeContextMenu::Completions(menu) => menu.origin(),
            CodeContextMenu::CodeActions(menu) => menu.origin(),
        }
    }

    pub fn render(
        &self,
        style: &EditorStyle,
        max_height_in_lines: u32,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
        match self {
            CodeContextMenu::Completions(menu) => {
                menu.render(style, max_height_in_lines, window, cx)
            }
            CodeContextMenu::CodeActions(menu) => {
                menu.render(style, max_height_in_lines, window, cx)
            }
        }
    }

    pub fn render_aside(
        &mut self,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        match self {
            CodeContextMenu::Completions(menu) => menu.render_aside(max_size, window, cx),
            CodeContextMenu::CodeActions(_) => None,
        }
    }

    pub fn focused(&self, window: &mut Window, cx: &mut Context<Editor>) -> bool {
        match self {
            CodeContextMenu::Completions(completions_menu) => completions_menu
                .get_or_create_entry_markdown(completions_menu.selected_item, cx)
                .as_ref()
                .is_some_and(|markdown| markdown.focus_handle(cx).contains_focused(window, cx)),
            CodeContextMenu::CodeActions(_) => false,
        }
    }

    pub fn scroll_aside(
        &mut self,
        scroll_amount: ScrollAmount,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        match self {
            CodeContextMenu::Completions(completions_menu) => {
                completions_menu.scroll_aside(scroll_amount, window, cx)
            }
            CodeContextMenu::CodeActions(_) => (),
        }
    }
}

pub enum ContextMenuOrigin {
    Cursor,
    GutterIndicator(DisplayRow),
    QuickActionBar,
}

pub struct CompletionsMenu {
    pub id: CompletionId,
    pub source: CompletionsMenuSource,
    sort_completions: bool,
    pub initial_position: Anchor,
    pub initial_query: Option<Arc<String>>,
    pub is_incomplete: bool,
    pub buffer: Entity<Buffer>,
    pub completions: Rc<RefCell<Box<[Completion]>>>,
    match_candidates: Arc<[StringMatchCandidate]>,
    pub entries: Rc<RefCell<Box<[StringMatch]>>>,
    pub selected_item: usize,
    filter_task: Task<()>,
    cancel_filter: Arc<AtomicBool>,
    scroll_handle: UniformListScrollHandle,
    // The `ScrollHandle` used on the Markdown documentation rendered on the
    // side of the completions menu.
    pub scroll_handle_aside: ScrollHandle,
    resolve_completions: bool,
    show_completion_documentation: bool,
    last_rendered_range: Rc<RefCell<Option<Range<usize>>>>,
    markdown_cache: Rc<RefCell<VecDeque<(MarkdownCacheKey, Entity<Markdown>)>>>,
    language_registry: Option<Arc<LanguageRegistry>>,
    language: Option<LanguageName>,
    display_options: CompletionDisplayOptions,
    snippet_sort_order: SnippetSortOrder,
}

#[derive(Clone, Debug, PartialEq)]
enum MarkdownCacheKey {
    ForCandidate {
        candidate_id: usize,
    },
    ForCompletionMatch {
        new_text: String,
        markdown_source: SharedString,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CompletionsMenuSource {
    Normal,
    SnippetChoices,
    Words,
}

// TODO: There should really be a wrapper around fuzzy match tasks that does this.
impl Drop for CompletionsMenu {
    fn drop(&mut self) {
        self.cancel_filter.store(true, Ordering::Relaxed);
    }
}

impl CompletionsMenu {
    pub fn new(
        id: CompletionId,
        source: CompletionsMenuSource,
        sort_completions: bool,
        show_completion_documentation: bool,
        initial_position: Anchor,
        initial_query: Option<Arc<String>>,
        is_incomplete: bool,
        buffer: Entity<Buffer>,
        completions: Box<[Completion]>,
        display_options: CompletionDisplayOptions,
        snippet_sort_order: SnippetSortOrder,
        language_registry: Option<Arc<LanguageRegistry>>,
        language: Option<LanguageName>,
        cx: &mut Context<Editor>,
    ) -> Self {
        let match_candidates = completions
            .iter()
            .enumerate()
            .map(|(id, completion)| StringMatchCandidate::new(id, completion.label.filter_text()))
            .collect();

        let completions_menu = Self {
            id,
            source,
            sort_completions,
            initial_position,
            initial_query,
            is_incomplete,
            buffer,
            show_completion_documentation,
            completions: RefCell::new(completions).into(),
            match_candidates,
            entries: Rc::new(RefCell::new(Box::new([]))),
            selected_item: 0,
            filter_task: Task::ready(()),
            cancel_filter: Arc::new(AtomicBool::new(false)),
            scroll_handle: UniformListScrollHandle::new(),
            scroll_handle_aside: ScrollHandle::new(),
            resolve_completions: true,
            last_rendered_range: RefCell::new(None).into(),
            markdown_cache: RefCell::new(VecDeque::new()).into(),
            language_registry,
            language,
            display_options,
            snippet_sort_order,
        };

        completions_menu.start_markdown_parse_for_nearby_entries(cx);

        completions_menu
    }

    pub fn new_snippet_choices(
        id: CompletionId,
        sort_completions: bool,
        choices: &Vec<String>,
        selection: Range<Anchor>,
        buffer: Entity<Buffer>,
        snippet_sort_order: SnippetSortOrder,
    ) -> Self {
        let completions = choices
            .iter()
            .map(|choice| Completion {
                replace_range: selection.start.text_anchor..selection.end.text_anchor,
                new_text: choice.to_string(),
                label: CodeLabel {
                    text: choice.to_string(),
                    runs: Default::default(),
                    filter_range: Default::default(),
                },
                icon_path: None,
                documentation: None,
                confirm: None,
                insert_text_mode: None,
                source: CompletionSource::Custom,
            })
            .collect();

        let match_candidates = choices
            .iter()
            .enumerate()
            .map(|(id, completion)| StringMatchCandidate::new(id, completion))
            .collect();
        let entries = choices
            .iter()
            .enumerate()
            .map(|(id, completion)| StringMatch {
                candidate_id: id,
                score: 1.,
                positions: vec![],
                string: completion.clone(),
            })
            .collect();
        Self {
            id,
            source: CompletionsMenuSource::SnippetChoices,
            sort_completions,
            initial_position: selection.start,
            initial_query: None,
            is_incomplete: false,
            buffer,
            completions: RefCell::new(completions).into(),
            match_candidates,
            entries: RefCell::new(entries).into(),
            selected_item: 0,
            filter_task: Task::ready(()),
            cancel_filter: Arc::new(AtomicBool::new(false)),
            scroll_handle: UniformListScrollHandle::new(),
            scroll_handle_aside: ScrollHandle::new(),
            resolve_completions: false,
            show_completion_documentation: false,
            last_rendered_range: RefCell::new(None).into(),
            markdown_cache: RefCell::new(VecDeque::new()).into(),
            language_registry: None,
            language: None,
            display_options: CompletionDisplayOptions::default(),
            snippet_sort_order,
        }
    }

    fn select_first(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let index = if self.scroll_handle.y_flipped() {
            self.entries.borrow().len() - 1
        } else {
            0
        };
        self.update_selection_index(index, provider, window, cx);
    }

    fn select_last(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let index = if self.scroll_handle.y_flipped() {
            0
        } else {
            self.entries.borrow().len() - 1
        };
        self.update_selection_index(index, provider, window, cx);
    }

    fn select_prev(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let index = if self.scroll_handle.y_flipped() {
            self.next_match_index()
        } else {
            self.prev_match_index()
        };
        self.update_selection_index(index, provider, window, cx);
    }

    fn select_next(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let index = if self.scroll_handle.y_flipped() {
            self.prev_match_index()
        } else {
            self.next_match_index()
        };
        self.update_selection_index(index, provider, window, cx);
    }

    fn update_selection_index(
        &mut self,
        match_index: usize,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if self.selected_item != match_index {
            self.selected_item = match_index;
            self.handle_selection_changed(provider, window, cx);
        }
    }

    fn prev_match_index(&self) -> usize {
        if self.selected_item > 0 {
            self.selected_item - 1
        } else {
            self.entries.borrow().len() - 1
        }
    }

    fn next_match_index(&self) -> usize {
        if self.selected_item + 1 < self.entries.borrow().len() {
            self.selected_item + 1
        } else {
            0
        }
    }

    fn handle_selection_changed(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        if let Some(provider) = provider {
            let entries = self.entries.borrow();
            let entry = if self.selected_item < entries.len() {
                Some(&entries[self.selected_item])
            } else {
                None
            };
            provider.selection_changed(entry, window, cx);
        }
        self.resolve_visible_completions(provider, cx);
        self.start_markdown_parse_for_nearby_entries(cx);
        cx.notify();
    }

    pub fn resolve_visible_completions(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut Context<Editor>,
    ) {
        if !self.resolve_completions {
            return;
        }
        let Some(provider) = provider else {
            return;
        };

        let entries = self.entries.borrow();
        if entries.is_empty() {
            return;
        }
        if self.selected_item >= entries.len() {
            log::error!(
                "bug: completion selected_item >= entries.len(): {} >= {}",
                self.selected_item,
                entries.len()
            );
            self.selected_item = entries.len() - 1;
        }

        // Attempt to resolve completions for every item that will be displayed. This matters
        // because single line documentation may be displayed inline with the completion.
        //
        // When navigating to the very beginning or end of completions, `last_rendered_range` may
        // have no overlap with the completions that will be displayed, so instead use a range based
        // on the last rendered count.
        const APPROXIMATE_VISIBLE_COUNT: usize = 12;
        let last_rendered_range = self.last_rendered_range.borrow().clone();
        let visible_count = last_rendered_range
            .clone()
            .map_or(APPROXIMATE_VISIBLE_COUNT, |range| range.count());
        let entry_range = if self.selected_item == 0 {
            0..min(visible_count, entries.len())
        } else if self.selected_item == entries.len() - 1 {
            entries.len().saturating_sub(visible_count)..entries.len()
        } else {
            last_rendered_range.map_or(0..0, |range| {
                min(range.start, entries.len())..min(range.end, entries.len())
            })
        };

        // Expand the range to resolve more completions than are predicted to be visible, to reduce
        // jank on navigation.
        let entry_indices = util::expanded_and_wrapped_usize_range(
            entry_range,
            RESOLVE_BEFORE_ITEMS,
            RESOLVE_AFTER_ITEMS,
            entries.len(),
        );

        // Avoid work by sometimes filtering out completions that already have documentation.
        // This filtering doesn't happen if the completions are currently being updated.
        let completions = self.completions.borrow();
        let candidate_ids = entry_indices
            .map(|i| entries[i].candidate_id)
            .filter(|i| completions[*i].documentation.is_none());

        // Current selection is always resolved even if it already has documentation, to handle
        // out-of-spec language servers that return more results later.
        let selected_candidate_id = entries[self.selected_item].candidate_id;
        let candidate_ids = iter::once(selected_candidate_id)
            .chain(candidate_ids.filter(|id| *id != selected_candidate_id))
            .collect::<Vec<usize>>();
        drop(entries);

        if candidate_ids.is_empty() {
            return;
        }

        let resolve_task = provider.resolve_completions(
            self.buffer.clone(),
            candidate_ids,
            self.completions.clone(),
            cx,
        );

        let completion_id = self.id;
        cx.spawn(async move |editor, cx| {
            if let Some(true) = resolve_task.await.log_err() {
                editor
                    .update(cx, |editor, cx| {
                        // `resolve_completions` modified state affecting display.
                        cx.notify();
                        editor.with_completions_menu_matching_id(completion_id, |menu| {
                            if let Some(menu) = menu {
                                menu.start_markdown_parse_for_nearby_entries(cx)
                            }
                        });
                    })
                    .ok();
            }
        })
        .detach();
    }

    fn start_markdown_parse_for_nearby_entries(&self, cx: &mut Context<Editor>) {
        // Enqueue parse tasks of nearer items first.
        //
        // TODO: This means that the nearer items will actually be further back in the cache, which
        // is not ideal. In practice this is fine because `get_or_create_markdown` moves the current
        // selection to the front (when `is_render = true`).
        let entry_indices = util::wrapped_usize_outward_from(
            self.selected_item,
            MARKDOWN_CACHE_BEFORE_ITEMS,
            MARKDOWN_CACHE_AFTER_ITEMS,
            self.entries.borrow().len(),
        );

        for index in entry_indices {
            self.get_or_create_entry_markdown(index, cx);
        }
    }

    fn get_or_create_entry_markdown(
        &self,
        index: usize,
        cx: &mut Context<Editor>,
    ) -> Option<Entity<Markdown>> {
        let entries = self.entries.borrow();
        if index >= entries.len() {
            return None;
        }
        let candidate_id = entries[index].candidate_id;
        let completions = self.completions.borrow();
        match &completions[candidate_id].documentation {
            Some(CompletionDocumentation::MultiLineMarkdown(source)) if !source.is_empty() => self
                .get_or_create_markdown(candidate_id, Some(source), false, &completions, cx)
                .map(|(_, markdown)| markdown),
            Some(_) => None,
            _ => None,
        }
    }

    fn get_or_create_markdown(
        &self,
        candidate_id: usize,
        source: Option<&SharedString>,
        is_render: bool,
        completions: &[Completion],
        cx: &mut Context<Editor>,
    ) -> Option<(bool, Entity<Markdown>)> {
        let mut markdown_cache = self.markdown_cache.borrow_mut();

        let mut has_completion_match_cache_entry = false;
        let mut matching_entry = markdown_cache.iter().find_position(|(key, _)| match key {
            MarkdownCacheKey::ForCandidate { candidate_id: id } => *id == candidate_id,
            MarkdownCacheKey::ForCompletionMatch { .. } => {
                has_completion_match_cache_entry = true;
                false
            }
        });

        if has_completion_match_cache_entry && matching_entry.is_none() {
            if let Some(source) = source {
                matching_entry = markdown_cache.iter().find_position(|(key, _)| {
                    matches!(key, MarkdownCacheKey::ForCompletionMatch { markdown_source, .. }
                                if markdown_source == source)
                });
            } else {
                // Heuristic guess that documentation can be reused when new_text matches. This is
                // to mitigate documentation flicker while typing. If this is wrong, then resolution
                // should cause the correct documentation to be displayed soon.
                let completion = &completions[candidate_id];
                matching_entry = markdown_cache.iter().find_position(|(key, _)| {
                    matches!(key, MarkdownCacheKey::ForCompletionMatch { new_text, .. }
                                if new_text == &completion.new_text)
                });
            }
        }

        if let Some((cache_index, (key, markdown))) = matching_entry {
            let markdown = markdown.clone();

            // Since the markdown source matches, the key can now be ForCandidate.
            if source.is_some() && matches!(key, MarkdownCacheKey::ForCompletionMatch { .. }) {
                markdown_cache[cache_index].0 = MarkdownCacheKey::ForCandidate { candidate_id };
            }

            if is_render && cache_index != 0 {
                // Move the current selection's cache entry to the front.
                markdown_cache.rotate_right(1);
                let cache_len = markdown_cache.len();
                markdown_cache.swap(0, (cache_index + 1) % cache_len);
            }

            let is_parsing = markdown.update(cx, |markdown, cx| {
                if let Some(source) = source {
                    // `reset` is called as it's possible for documentation to change due to resolve
                    // requests. It does nothing if `source` is unchanged.
                    markdown.reset(source.clone(), cx);
                }
                markdown.is_parsing()
            });
            return Some((is_parsing, markdown));
        }

        let Some(source) = source else {
            // Can't create markdown as there is no source.
            return None;
        };

        if markdown_cache.len() < MARKDOWN_CACHE_MAX_SIZE {
            let markdown = cx.new(|cx| {
                Markdown::new(
                    source.clone(),
                    self.language_registry.clone(),
                    self.language.clone(),
                    cx,
                )
            });
            // Handles redraw when the markdown is done parsing. The current render is for a
            // deferred draw, and so without this did not redraw when `markdown` notified.
            cx.observe(&markdown, |_, _, cx| cx.notify()).detach();
            markdown_cache.push_front((
                MarkdownCacheKey::ForCandidate { candidate_id },
                markdown.clone(),
            ));
            Some((true, markdown))
        } else {
            debug_assert_eq!(markdown_cache.capacity(), MARKDOWN_CACHE_MAX_SIZE);
            // Moves the last cache entry to the start. The ring buffer is full, so this does no
            // copying and just shifts indexes.
            markdown_cache.rotate_right(1);
            markdown_cache[0].0 = MarkdownCacheKey::ForCandidate { candidate_id };
            let markdown = &markdown_cache[0].1;
            markdown.update(cx, |markdown, cx| markdown.reset(source.clone(), cx));
            Some((true, markdown.clone()))
        }
    }

    pub fn visible(&self) -> bool {
        !self.entries.borrow().is_empty()
    }

    fn origin(&self) -> ContextMenuOrigin {
        ContextMenuOrigin::Cursor
    }

    fn render(
        &self,
        style: &EditorStyle,
        max_height_in_lines: u32,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
        let show_completion_documentation = self.show_completion_documentation;
        let widest_completion_ix = if self.display_options.dynamic_width {
            let completions = self.completions.borrow();
            let widest_completion_ix = self
                .entries
                .borrow()
                .iter()
                .enumerate()
                .max_by_key(|(_, mat)| {
                    let completion = &completions[mat.candidate_id];
                    let documentation = &completion.documentation;

                    let mut len = completion.label.text.chars().count();
                    if let Some(CompletionDocumentation::SingleLine(text)) = documentation {
                        if show_completion_documentation {
                            len += text.chars().count();
                        }
                    }

                    len
                })
                .map(|(ix, _)| ix);
            drop(completions);
            widest_completion_ix
        } else {
            None
        };

        let selected_item = self.selected_item;
        let completions = self.completions.clone();
        let entries = self.entries.clone();
        let last_rendered_range = self.last_rendered_range.clone();
        let style = style.clone();
        let list = uniform_list(
            "completions",
            self.entries.borrow().len(),
            cx.processor(move |_editor, range: Range<usize>, _window, cx| {
                last_rendered_range.borrow_mut().replace(range.clone());
                let start_ix = range.start;
                let completions_guard = completions.borrow_mut();

                entries.borrow()[range]
                    .iter()
                    .enumerate()
                    .map(|(ix, mat)| {
                        let item_ix = start_ix + ix;
                        let completion = &completions_guard[mat.candidate_id];
                        let documentation = if show_completion_documentation {
                            &completion.documentation
                        } else {
                            &None
                        };

                        let filter_start = completion.label.filter_range.start;
                        let highlights = gpui::combine_highlights(
                            mat.ranges().map(|range| {
                                (
                                    filter_start + range.start..filter_start + range.end,
                                    FontWeight::BOLD.into(),
                                )
                            }),
                            styled_runs_for_code_label(&completion.label, &style.syntax).map(
                                |(range, mut highlight)| {
                                    // Ignore font weight for syntax highlighting, as we'll use it
                                    // for fuzzy matches.
                                    highlight.font_weight = None;
                                    if completion
                                        .source
                                        .lsp_completion(false)
                                        .and_then(|lsp_completion| lsp_completion.deprecated)
                                        .unwrap_or(false)
                                    {
                                        highlight.strikethrough = Some(StrikethroughStyle {
                                            thickness: 1.0.into(),
                                            ..Default::default()
                                        });
                                        highlight.color = Some(cx.theme().colors().text_muted);
                                    }

                                    (range, highlight)
                                },
                            ),
                        );

                        let completion_label = StyledText::new(completion.label.text.clone())
                            .with_default_highlights(&style.text, highlights);

                        let documentation_label = match documentation {
                            Some(CompletionDocumentation::SingleLine(text))
                            | Some(CompletionDocumentation::SingleLineAndMultiLinePlainText {
                                single_line: text,
                                ..
                            }) => {
                                if text.trim().is_empty() {
                                    None
                                } else {
                                    Some(
                                        Label::new(text.clone())
                                            .ml_4()
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }
                            }
                            _ => None,
                        };

                        let start_slot = completion
                            .color()
                            .map(|color| {
                                div()
                                    .flex_shrink_0()
                                    .size_3p5()
                                    .rounded_xs()
                                    .bg(color)
                                    .into_any_element()
                            })
                            .or_else(|| {
                                completion.icon_path.as_ref().map(|path| {
                                    Icon::from_path(path)
                                        .size(IconSize::XSmall)
                                        .color(Color::Muted)
                                        .into_any_element()
                                })
                            });

                        div().min_w(px(280.)).max_w(px(540.)).child(
                            ListItem::new(mat.candidate_id)
                                .inset(true)
                                .toggle_state(item_ix == selected_item)
                                .on_click(cx.listener(move |editor, _event, window, cx| {
                                    cx.stop_propagation();
                                    if let Some(task) = editor.confirm_completion(
                                        &ConfirmCompletion {
                                            item_ix: Some(item_ix),
                                        },
                                        window,
                                        cx,
                                    ) {
                                        task.detach_and_log_err(cx)
                                    }
                                }))
                                .start_slot::<AnyElement>(start_slot)
                                .child(h_flex().overflow_hidden().child(completion_label))
                                .end_slot::<Label>(documentation_label),
                        )
                    })
                    .collect()
            }),
        )
        .occlude()
        .max_h(max_height_in_lines as f32 * window.line_height())
        .track_scroll(self.scroll_handle.clone())
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .map(|this| {
            if self.display_options.dynamic_width {
                this.with_width_from_item(widest_completion_ix)
            } else {
                this.w(rems(34.))
            }
        });

        Popover::new().child(list).into_any_element()
    }

    fn render_aside(
        &mut self,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        if !self.show_completion_documentation {
            return None;
        }

        let mat = &self.entries.borrow()[self.selected_item];
        let completions = self.completions.borrow_mut();
        let multiline_docs = match completions[mat.candidate_id].documentation.as_ref() {
            Some(CompletionDocumentation::MultiLinePlainText(text)) => div().child(text.clone()),
            Some(CompletionDocumentation::SingleLineAndMultiLinePlainText {
                plain_text: Some(text),
                ..
            }) => div().child(text.clone()),
            Some(CompletionDocumentation::MultiLineMarkdown(source)) if !source.is_empty() => {
                let Some((false, markdown)) = self.get_or_create_markdown(
                    mat.candidate_id,
                    Some(source),
                    true,
                    &completions,
                    cx,
                ) else {
                    return None;
                };
                Self::render_markdown(markdown, window, cx)
            }
            None => {
                // Handle the case where documentation hasn't yet been resolved but there's a
                // `new_text` match in the cache.
                //
                // TODO: It's inconsistent that documentation caching based on matching `new_text`
                // only works for markdown. Consider generally caching the results of resolving
                // completions.
                let Some((false, markdown)) =
                    self.get_or_create_markdown(mat.candidate_id, None, true, &completions, cx)
                else {
                    return None;
                };
                Self::render_markdown(markdown, window, cx)
            }
            Some(CompletionDocumentation::MultiLineMarkdown(_)) => return None,
            Some(CompletionDocumentation::SingleLine(_)) => return None,
            Some(CompletionDocumentation::Undocumented) => return None,
            Some(CompletionDocumentation::SingleLineAndMultiLinePlainText {
                plain_text: None,
                ..
            }) => {
                return None;
            }
        };

        Some(
            Popover::new()
                .child(
                    multiline_docs
                        .id("multiline_docs")
                        .px(MENU_ASIDE_X_PADDING / 2.)
                        .max_w(max_size.width)
                        .max_h(max_size.height)
                        .overflow_y_scroll()
                        .track_scroll(&self.scroll_handle_aside)
                        .occlude(),
                )
                .into_any_element(),
        )
    }

    fn render_markdown(
        markdown: Entity<Markdown>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Div {
        div().child(
            MarkdownElement::new(markdown, hover_markdown_style(window, cx))
                .code_block_renderer(markdown::CodeBlockRenderer::Default {
                    copy_button: false,
                    copy_button_on_hover: false,
                    border: false,
                })
                .on_url_click(open_markdown_url),
        )
    }

    pub fn filter(
        &mut self,
        query: Option<Arc<String>>,
        provider: Option<Rc<dyn CompletionProvider>>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.cancel_filter.store(true, Ordering::Relaxed);
        if let Some(query) = query {
            self.cancel_filter = Arc::new(AtomicBool::new(false));
            let matches = self.do_async_filtering(query, cx);
            let id = self.id;
            self.filter_task = cx.spawn_in(window, async move |editor, cx| {
                let matches = matches.await;
                editor
                    .update_in(cx, |editor, window, cx| {
                        editor.with_completions_menu_matching_id(id, |this| {
                            if let Some(this) = this {
                                this.set_filter_results(matches, provider, window, cx);
                            }
                        });
                    })
                    .ok();
            });
        } else {
            self.filter_task = Task::ready(());
            let matches = self.unfiltered_matches();
            self.set_filter_results(matches, provider, window, cx);
        }
    }

    pub fn do_async_filtering(
        &self,
        query: Arc<String>,
        cx: &Context<Editor>,
    ) -> Task<Vec<StringMatch>> {
        let matches_task = cx.background_spawn({
            let query = query.clone();
            let match_candidates = self.match_candidates.clone();
            let cancel_filter = self.cancel_filter.clone();
            let background_executor = cx.background_executor().clone();
            async move {
                fuzzy::match_strings(
                    &match_candidates,
                    &query,
                    query.chars().any(|c| c.is_uppercase()),
                    false,
                    1000,
                    &cancel_filter,
                    background_executor,
                )
                .await
            }
        });

        let completions = self.completions.clone();
        let sort_completions = self.sort_completions;
        let snippet_sort_order = self.snippet_sort_order;
        cx.foreground_executor().spawn(async move {
            let mut matches = matches_task.await;

            if sort_completions {
                matches = Self::sort_string_matches(
                    matches,
                    Some(&query),
                    snippet_sort_order,
                    completions.borrow().as_ref(),
                );
            }

            matches
        })
    }

    /// Like `do_async_filtering` but there is no filter query, so no need to spawn tasks.
    pub fn unfiltered_matches(&self) -> Vec<StringMatch> {
        let mut matches = self
            .match_candidates
            .iter()
            .enumerate()
            .map(|(candidate_id, candidate)| StringMatch {
                candidate_id,
                score: Default::default(),
                positions: Default::default(),
                string: candidate.string.clone(),
            })
            .collect();

        if self.sort_completions {
            matches = Self::sort_string_matches(
                matches,
                None,
                self.snippet_sort_order,
                self.completions.borrow().as_ref(),
            );
        }

        matches
    }

    pub fn set_filter_results(
        &mut self,
        matches: Vec<StringMatch>,
        provider: Option<Rc<dyn CompletionProvider>>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        *self.entries.borrow_mut() = matches.into_boxed_slice();
        self.selected_item = 0;
        self.handle_selection_changed(provider.as_deref(), window, cx);
    }

    pub fn sort_string_matches(
        matches: Vec<StringMatch>,
        query: Option<&str>,
        snippet_sort_order: SnippetSortOrder,
        completions: &[Completion],
    ) -> Vec<StringMatch> {
        let mut matches = matches;

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum MatchTier<'a> {
            WordStartMatch {
                sort_exact: Reverse<i32>,
                sort_snippet: Reverse<i32>,
                sort_score: Reverse<OrderedFloat<f64>>,
                sort_positions: Vec<usize>,
                sort_text: Option<&'a str>,
                sort_kind: usize,
                sort_label: &'a str,
            },
            OtherMatch {
                sort_score: Reverse<OrderedFloat<f64>>,
            },
        }

        let query_start_lower = query
            .as_ref()
            .and_then(|q| q.chars().next())
            .and_then(|c| c.to_lowercase().next());

        if snippet_sort_order == SnippetSortOrder::None {
            matches.retain(|string_match| {
                let completion = &completions[string_match.candidate_id];

                let is_snippet = matches!(
                    &completion.source,
                    CompletionSource::Lsp { lsp_completion, .. }
                    if lsp_completion.kind == Some(CompletionItemKind::SNIPPET)
                );

                !is_snippet
            });
        }

        matches.sort_unstable_by_key(|string_match| {
            let completion = &completions[string_match.candidate_id];

            let is_snippet = matches!(
                &completion.source,
                CompletionSource::Lsp { lsp_completion, .. }
                if lsp_completion.kind == Some(CompletionItemKind::SNIPPET)
            );

            let sort_text = match &completion.source {
                CompletionSource::Lsp { lsp_completion, .. } => lsp_completion.sort_text.as_deref(),
                CompletionSource::Dap { sort_text } => Some(sort_text.as_str()),
                _ => None,
            };

            let (sort_kind, sort_label) = completion.sort_key();

            let score = string_match.score;
            let sort_score = Reverse(OrderedFloat(score));

            let query_start_doesnt_match_split_words = query_start_lower
                .map(|query_char| {
                    !split_words(&string_match.string).any(|word| {
                        word.chars().next().and_then(|c| c.to_lowercase().next())
                            == Some(query_char)
                    })
                })
                .unwrap_or(false);

            if query_start_doesnt_match_split_words {
                MatchTier::OtherMatch { sort_score }
            } else {
                let sort_snippet = match snippet_sort_order {
                    SnippetSortOrder::Top => Reverse(if is_snippet { 1 } else { 0 }),
                    SnippetSortOrder::Bottom => Reverse(if is_snippet { 0 } else { 1 }),
                    SnippetSortOrder::Inline => Reverse(0),
                    SnippetSortOrder::None => Reverse(0),
                };
                let sort_positions = string_match.positions.clone();
                let sort_exact = Reverse(if Some(completion.label.filter_text()) == query {
                    1
                } else {
                    0
                });

                MatchTier::WordStartMatch {
                    sort_exact,
                    sort_snippet,
                    sort_score,
                    sort_positions,
                    sort_text,
                    sort_kind,
                    sort_label,
                }
            }
        });

        matches
    }

    pub fn preserve_markdown_cache(&mut self, prev_menu: CompletionsMenu) {
        self.markdown_cache = prev_menu.markdown_cache.clone();

        // Convert ForCandidate cache keys to ForCompletionMatch keys.
        let prev_completions = prev_menu.completions.borrow();
        self.markdown_cache
            .borrow_mut()
            .retain_mut(|(key, _markdown)| match key {
                MarkdownCacheKey::ForCompletionMatch { .. } => true,
                MarkdownCacheKey::ForCandidate { candidate_id } => {
                    if let Some(completion) = prev_completions.get(*candidate_id) {
                        match &completion.documentation {
                            Some(CompletionDocumentation::MultiLineMarkdown(source)) => {
                                *key = MarkdownCacheKey::ForCompletionMatch {
                                    new_text: completion.new_text.clone(),
                                    markdown_source: source.clone(),
                                };
                                true
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
            });
    }

    pub fn scroll_aside(
        &mut self,
        amount: ScrollAmount,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let mut offset = self.scroll_handle_aside.offset();

        offset.y -= amount.pixels(
            window.line_height(),
            self.scroll_handle_aside.bounds().size.height - px(16.),
        ) / 2.0;

        cx.notify();
        self.scroll_handle_aside.set_offset(offset);
    }
}

#[derive(Clone)]
pub struct AvailableCodeAction {
    pub excerpt_id: ExcerptId,
    pub action: CodeAction,
    pub provider: Rc<dyn CodeActionProvider>,
}

#[derive(Clone)]
pub struct CodeActionContents {
    tasks: Option<Rc<ResolvedTasks>>,
    actions: Option<Rc<[AvailableCodeAction]>>,
    debug_scenarios: Vec<DebugScenario>,
    pub(crate) context: TaskContext,
}

impl CodeActionContents {
    pub(crate) fn new(
        tasks: Option<ResolvedTasks>,
        actions: Option<Rc<[AvailableCodeAction]>>,
        debug_scenarios: Vec<DebugScenario>,
        context: TaskContext,
    ) -> Self {
        Self {
            tasks: tasks.map(Rc::new),
            actions,
            debug_scenarios,
            context,
        }
    }

    pub fn tasks(&self) -> Option<&ResolvedTasks> {
        self.tasks.as_deref()
    }

    fn len(&self) -> usize {
        let tasks_len = self.tasks.as_ref().map_or(0, |tasks| tasks.templates.len());
        let code_actions_len = self.actions.as_ref().map_or(0, |actions| actions.len());
        tasks_len + code_actions_len + self.debug_scenarios.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iter(&self) -> impl Iterator<Item = CodeActionsItem> + '_ {
        self.tasks
            .iter()
            .flat_map(|tasks| {
                tasks
                    .templates
                    .iter()
                    .map(|(kind, task)| CodeActionsItem::Task(kind.clone(), task.clone()))
            })
            .chain(self.actions.iter().flat_map(|actions| {
                actions.iter().map(|available| CodeActionsItem::CodeAction {
                    excerpt_id: available.excerpt_id,
                    action: available.action.clone(),
                    provider: available.provider.clone(),
                })
            }))
            .chain(
                self.debug_scenarios
                    .iter()
                    .cloned()
                    .map(CodeActionsItem::DebugScenario),
            )
    }

    pub fn get(&self, mut index: usize) -> Option<CodeActionsItem> {
        if let Some(tasks) = &self.tasks {
            if let Some((kind, task)) = tasks.templates.get(index) {
                return Some(CodeActionsItem::Task(kind.clone(), task.clone()));
            } else {
                index -= tasks.templates.len();
            }
        }
        if let Some(actions) = &self.actions {
            if let Some(available) = actions.get(index) {
                return Some(CodeActionsItem::CodeAction {
                    excerpt_id: available.excerpt_id,
                    action: available.action.clone(),
                    provider: available.provider.clone(),
                });
            } else {
                index -= actions.len();
            }
        }

        self.debug_scenarios
            .get(index)
            .cloned()
            .map(CodeActionsItem::DebugScenario)
    }
}

#[derive(Clone)]
pub enum CodeActionsItem {
    Task(TaskSourceKind, ResolvedTask),
    CodeAction {
        excerpt_id: ExcerptId,
        action: CodeAction,
        provider: Rc<dyn CodeActionProvider>,
    },
    DebugScenario(DebugScenario),
}

impl CodeActionsItem {
    fn as_task(&self) -> Option<&ResolvedTask> {
        let Self::Task(_, task) = self else {
            return None;
        };
        Some(task)
    }

    fn as_code_action(&self) -> Option<&CodeAction> {
        let Self::CodeAction { action, .. } = self else {
            return None;
        };
        Some(action)
    }
    fn as_debug_scenario(&self) -> Option<&DebugScenario> {
        let Self::DebugScenario(scenario) = self else {
            return None;
        };
        Some(scenario)
    }

    pub fn label(&self) -> String {
        match self {
            Self::CodeAction { action, .. } => action.lsp_action.title().to_owned(),
            Self::Task(_, task) => task.resolved_label.clone(),
            Self::DebugScenario(scenario) => scenario.label.to_string(),
        }
    }
}

pub struct CodeActionsMenu {
    pub actions: CodeActionContents,
    pub buffer: Entity<Buffer>,
    pub selected_item: usize,
    pub scroll_handle: UniformListScrollHandle,
    pub deployed_from: Option<CodeActionSource>,
}

impl CodeActionsMenu {
    fn select_first(&mut self, cx: &mut Context<Editor>) {
        self.selected_item = if self.scroll_handle.y_flipped() {
            self.actions.len() - 1
        } else {
            0
        };
        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify()
    }

    fn select_last(&mut self, cx: &mut Context<Editor>) {
        self.selected_item = if self.scroll_handle.y_flipped() {
            0
        } else {
            self.actions.len() - 1
        };
        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify()
    }

    fn select_prev(&mut self, cx: &mut Context<Editor>) {
        self.selected_item = if self.scroll_handle.y_flipped() {
            self.next_match_index()
        } else {
            self.prev_match_index()
        };
        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify();
    }

    fn select_next(&mut self, cx: &mut Context<Editor>) {
        self.selected_item = if self.scroll_handle.y_flipped() {
            self.prev_match_index()
        } else {
            self.next_match_index()
        };
        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify();
    }

    fn prev_match_index(&self) -> usize {
        if self.selected_item > 0 {
            self.selected_item - 1
        } else {
            self.actions.len() - 1
        }
    }

    fn next_match_index(&self) -> usize {
        if self.selected_item + 1 < self.actions.len() {
            self.selected_item + 1
        } else {
            0
        }
    }

    pub fn visible(&self) -> bool {
        !self.actions.is_empty()
    }

    fn origin(&self) -> ContextMenuOrigin {
        match &self.deployed_from {
            Some(CodeActionSource::Indicator(row)) | Some(CodeActionSource::RunMenu(row)) => {
                ContextMenuOrigin::GutterIndicator(*row)
            }
            Some(CodeActionSource::QuickActionBar) => ContextMenuOrigin::QuickActionBar,
            None => ContextMenuOrigin::Cursor,
        }
    }

    fn render(
        &self,
        _style: &EditorStyle,
        max_height_in_lines: u32,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
        let actions = self.actions.clone();
        let selected_item = self.selected_item;
        let list = uniform_list(
            "code_actions_menu",
            self.actions.len(),
            cx.processor(move |_this, range: Range<usize>, _, cx| {
                actions
                    .iter()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .enumerate()
                    .map(|(ix, action)| {
                        let item_ix = range.start + ix;
                        let selected = item_ix == selected_item;
                        let colors = cx.theme().colors();
                        div().min_w(px(220.)).max_w(px(540.)).child(
                            ListItem::new(item_ix)
                                .inset(true)
                                .toggle_state(selected)
                                .when_some(action.as_code_action(), |this, action| {
                                    this.child(
                                        h_flex()
                                            .overflow_hidden()
                                            .child(
                                                // TASK: It would be good to make lsp_action.title a SharedString to avoid allocating here.
                                                action.lsp_action.title().replace("\n", ""),
                                            )
                                            .when(selected, |this| {
                                                this.text_color(colors.text_accent)
                                            }),
                                    )
                                })
                                .when_some(action.as_task(), |this, task| {
                                    this.child(
                                        h_flex()
                                            .overflow_hidden()
                                            .child(task.resolved_label.replace("\n", ""))
                                            .when(selected, |this| {
                                                this.text_color(colors.text_accent)
                                            }),
                                    )
                                })
                                .when_some(action.as_debug_scenario(), |this, scenario| {
                                    this.child(
                                        h_flex()
                                            .overflow_hidden()
                                            .child("debug: ")
                                            .child(scenario.label.clone())
                                            .when(selected, |this| {
                                                this.text_color(colors.text_accent)
                                            }),
                                    )
                                })
                                .on_click(cx.listener(move |editor, _, window, cx| {
                                    cx.stop_propagation();
                                    if let Some(task) = editor.confirm_code_action(
                                        &ConfirmCodeAction {
                                            item_ix: Some(item_ix),
                                        },
                                        window,
                                        cx,
                                    ) {
                                        task.detach_and_log_err(cx)
                                    }
                                })),
                        )
                    })
                    .collect()
            }),
        )
        .occlude()
        .max_h(max_height_in_lines as f32 * window.line_height())
        .track_scroll(self.scroll_handle.clone())
        .with_width_from_item(
            self.actions
                .iter()
                .enumerate()
                .max_by_key(|(_, action)| match action {
                    CodeActionsItem::Task(_, task) => task.resolved_label.chars().count(),
                    CodeActionsItem::CodeAction { action, .. } => {
                        action.lsp_action.title().chars().count()
                    }
                    CodeActionsItem::DebugScenario(scenario) => {
                        format!("debug: {}", scenario.label).chars().count()
                    }
                })
                .map(|(ix, _)| ix),
        )
        .with_sizing_behavior(ListSizingBehavior::Infer);

        Popover::new().child(list).into_any_element()
    }
}
