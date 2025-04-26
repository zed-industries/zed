use feature_flags::{DebuggerFeatureFlag, FeatureFlagAppExt as _};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, BackgroundExecutor, Entity, Focusable, FontWeight, ListSizingBehavior,
    ScrollStrategy, SharedString, Size, StrikethroughStyle, StyledText, UniformListScrollHandle,
    div, px, uniform_list,
};
use language::Buffer;
use language::CodeLabel;
use markdown::{Markdown, MarkdownElement};
use multi_buffer::{Anchor, ExcerptId};
use ordered_float::OrderedFloat;
use project::CompletionSource;
use project::lsp_store::CompletionDocumentation;
use project::{CodeAction, Completion, TaskSourceKind};

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

pub enum CodeContextMenu {
    Completions(CompletionsMenu),
    CodeActions(CodeActionsMenu),
}

impl CodeContextMenu {
    pub fn select_first(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_first(provider, cx),
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
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_prev(provider, cx),
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
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_next(provider, cx),
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
        cx: &mut Context<Editor>,
    ) -> bool {
        if self.visible() {
            match self {
                CodeContextMenu::Completions(menu) => menu.select_last(provider, cx),
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
        editor: &Editor,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        match self {
            CodeContextMenu::Completions(menu) => menu.render_aside(editor, max_size, window, cx),
            CodeContextMenu::CodeActions(_) => None,
        }
    }

    pub fn focused(&self, window: &mut Window, cx: &mut Context<Editor>) -> bool {
        match self {
            CodeContextMenu::Completions(completions_menu) => completions_menu
                .markdown_element
                .as_ref()
                .is_some_and(|markdown| markdown.focus_handle(cx).contains_focused(window, cx)),
            CodeContextMenu::CodeActions(_) => false,
        }
    }
}

pub enum ContextMenuOrigin {
    Cursor,
    GutterIndicator(DisplayRow),
}

#[derive(Clone, Debug)]
pub struct CompletionsMenu {
    pub id: CompletionId,
    sort_completions: bool,
    pub initial_position: Anchor,
    pub buffer: Entity<Buffer>,
    pub completions: Rc<RefCell<Box<[Completion]>>>,
    match_candidates: Rc<[StringMatchCandidate]>,
    pub entries: Rc<RefCell<Vec<StringMatch>>>,
    pub selected_item: usize,
    scroll_handle: UniformListScrollHandle,
    resolve_completions: bool,
    show_completion_documentation: bool,
    pub(super) ignore_completion_provider: bool,
    last_rendered_range: Rc<RefCell<Option<Range<usize>>>>,
    markdown_element: Option<Entity<Markdown>>,
    snippet_sort_order: SnippetSortOrder,
}

impl CompletionsMenu {
    pub fn new(
        id: CompletionId,
        sort_completions: bool,
        show_completion_documentation: bool,
        ignore_completion_provider: bool,
        initial_position: Anchor,
        buffer: Entity<Buffer>,
        completions: Box<[Completion]>,
        snippet_sort_order: SnippetSortOrder,
    ) -> Self {
        let match_candidates = completions
            .iter()
            .enumerate()
            .map(|(id, completion)| StringMatchCandidate::new(id, &completion.label.filter_text()))
            .collect();

        Self {
            id,
            sort_completions,
            initial_position,
            buffer,
            show_completion_documentation,
            ignore_completion_provider,
            completions: RefCell::new(completions).into(),
            match_candidates,
            entries: RefCell::new(Vec::new()).into(),
            selected_item: 0,
            scroll_handle: UniformListScrollHandle::new(),
            resolve_completions: true,
            last_rendered_range: RefCell::new(None).into(),
            markdown_element: None,
            snippet_sort_order,
        }
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
            .map(|(id, completion)| StringMatchCandidate::new(id, &completion))
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
            .collect::<Vec<_>>();
        Self {
            id,
            sort_completions,
            initial_position: selection.start,
            buffer,
            completions: RefCell::new(completions).into(),
            match_candidates,
            entries: RefCell::new(entries).into(),
            selected_item: 0,
            scroll_handle: UniformListScrollHandle::new(),
            resolve_completions: false,
            show_completion_documentation: false,
            ignore_completion_provider: false,
            last_rendered_range: RefCell::new(None).into(),
            markdown_element: None,
            snippet_sort_order,
        }
    }

    fn select_first(
        &mut self,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut Context<Editor>,
    ) {
        let index = if self.scroll_handle.y_flipped() {
            self.entries.borrow().len() - 1
        } else {
            0
        };
        self.update_selection_index(index, provider, cx);
    }

    fn select_last(&mut self, provider: Option<&dyn CompletionProvider>, cx: &mut Context<Editor>) {
        let index = if self.scroll_handle.y_flipped() {
            0
        } else {
            self.entries.borrow().len() - 1
        };
        self.update_selection_index(index, provider, cx);
    }

    fn select_prev(&mut self, provider: Option<&dyn CompletionProvider>, cx: &mut Context<Editor>) {
        let index = if self.scroll_handle.y_flipped() {
            self.next_match_index()
        } else {
            self.prev_match_index()
        };
        self.update_selection_index(index, provider, cx);
    }

    fn select_next(&mut self, provider: Option<&dyn CompletionProvider>, cx: &mut Context<Editor>) {
        let index = if self.scroll_handle.y_flipped() {
            self.prev_match_index()
        } else {
            self.next_match_index()
        };
        self.update_selection_index(index, provider, cx);
    }

    fn update_selection_index(
        &mut self,
        match_index: usize,
        provider: Option<&dyn CompletionProvider>,
        cx: &mut Context<Editor>,
    ) {
        if self.selected_item != match_index {
            self.selected_item = match_index;
            self.scroll_handle
                .scroll_to_item(self.selected_item, ScrollStrategy::Top);
            self.resolve_visible_completions(provider, cx);
            cx.notify();
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
        let entries = self.entries.borrow();
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
        const EXTRA_TO_RESOLVE: usize = 4;
        let entry_indices = util::iterate_expanded_and_wrapped_usize_range(
            entry_range.clone(),
            EXTRA_TO_RESOLVE,
            EXTRA_TO_RESOLVE,
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

        cx.spawn(async move |editor, cx| {
            if let Some(true) = resolve_task.await.log_err() {
                editor.update(cx, |_, cx| cx.notify()).ok();
            }
        })
        .detach();
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
        let completions = self.completions.borrow_mut();
        let show_completion_documentation = self.show_completion_documentation;
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

        let selected_item = self.selected_item;
        let completions = self.completions.clone();
        let entries = self.entries.clone();
        let last_rendered_range = self.last_rendered_range.clone();
        let style = style.clone();
        let list = uniform_list(
            cx.entity().clone(),
            "completions",
            self.entries.borrow().len(),
            move |_editor, range, _window, cx| {
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
                        let documentation_label = if let Some(
                            CompletionDocumentation::SingleLine(text),
                        ) = documentation
                        {
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
                        } else {
                            None
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
            },
        )
        .occlude()
        .max_h(max_height_in_lines as f32 * window.line_height())
        .track_scroll(self.scroll_handle.clone())
        .with_width_from_item(widest_completion_ix)
        .with_sizing_behavior(ListSizingBehavior::Infer);

        Popover::new().child(list).into_any_element()
    }

    fn render_aside(
        &mut self,
        editor: &Editor,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        if !self.show_completion_documentation {
            return None;
        }

        let mat = &self.entries.borrow()[self.selected_item];
        let multiline_docs = match self.completions.borrow_mut()[mat.candidate_id]
            .documentation
            .as_ref()?
        {
            CompletionDocumentation::MultiLinePlainText(text) => div().child(text.clone()),
            CompletionDocumentation::MultiLineMarkdown(parsed) if !parsed.is_empty() => {
                let markdown = self.markdown_element.get_or_insert_with(|| {
                    cx.new(|cx| {
                        let languages = editor
                            .workspace
                            .as_ref()
                            .and_then(|(workspace, _)| workspace.upgrade())
                            .map(|workspace| workspace.read(cx).app_state().languages.clone());
                        let language = editor
                            .language_at(self.initial_position, cx)
                            .map(|l| l.name().to_proto());
                        Markdown::new(SharedString::default(), languages, language, cx)
                    })
                });
                markdown.update(cx, |markdown, cx| {
                    markdown.reset(parsed.clone(), cx);
                });
                div().child(
                    MarkdownElement::new(markdown.clone(), hover_markdown_style(window, cx))
                        .code_block_renderer(markdown::CodeBlockRenderer::Default {
                            copy_button: false,
                            border: false,
                        })
                        .on_url_click(open_markdown_url),
                )
            }
            CompletionDocumentation::MultiLineMarkdown(_) => return None,
            CompletionDocumentation::SingleLine(_) => return None,
            CompletionDocumentation::Undocumented => return None,
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
                        .occlude(),
                )
                .into_any_element(),
        )
    }

    pub fn sort_matches(
        matches: &mut Vec<SortableMatch<'_>>,
        query: Option<&str>,
        snippet_sort_order: SnippetSortOrder,
    ) {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum MatchTier<'a> {
            WordStartMatch {
                sort_prefix: Reverse<usize>,
                sort_snippet: Reverse<i32>,
                sort_text: Option<&'a str>,
                sort_score: Reverse<OrderedFloat<f64>>,
                sort_key: (usize, &'a str),
            },
            OtherMatch {
                sort_score: Reverse<OrderedFloat<f64>>,
            },
        }

        // Our goal here is to intelligently sort completion suggestions. We want to
        // balance the raw fuzzy match score with hints from the language server

        let query_start_lower = query
            .and_then(|q| q.chars().next())
            .and_then(|c| c.to_lowercase().next());

        matches.sort_unstable_by_key(|mat| {
            let score = mat.string_match.score;
            let sort_score = Reverse(OrderedFloat(score));

            let query_start_doesnt_match_split_words = query_start_lower
                .map(|query_char| {
                    !split_words(&mat.string_match.string).any(|word| {
                        word.chars()
                            .next()
                            .and_then(|c| c.to_lowercase().next())
                            .map_or(false, |word_char| word_char == query_char)
                    })
                })
                .unwrap_or(false);

            if query_start_doesnt_match_split_words {
                MatchTier::OtherMatch { sort_score }
            } else {
                let sort_snippet = match snippet_sort_order {
                    SnippetSortOrder::Top => Reverse(if mat.is_snippet { 1 } else { 0 }),
                    SnippetSortOrder::Bottom => Reverse(if mat.is_snippet { 0 } else { 1 }),
                    SnippetSortOrder::Inline => Reverse(0),
                };
                let mixed_case_prefix_length = Reverse(
                    query
                        .map(|q| {
                            q.chars()
                                .zip(mat.string_match.string.chars())
                                .enumerate()
                                .take_while(|(i, (q_char, match_char))| {
                                    if *i == 0 {
                                        // Ccase-sensitive comparison for first character
                                        q_char == match_char
                                    } else {
                                        // Case-insensitive comparison for other characters
                                        q_char.to_lowercase().eq(match_char.to_lowercase())
                                    }
                                })
                                .count()
                        })
                        .unwrap_or(0),
                );
                MatchTier::WordStartMatch {
                    sort_prefix: mixed_case_prefix_length,
                    sort_snippet,
                    sort_text: mat.sort_text,
                    sort_score,
                    sort_key: mat.sort_key,
                }
            }
        });
    }

    pub async fn filter(&mut self, query: Option<&str>, executor: BackgroundExecutor) {
        let mut matches = if let Some(query) = query {
            fuzzy::match_strings(
                &self.match_candidates,
                query,
                query.chars().any(|c| c.is_uppercase()),
                100,
                &Default::default(),
                executor,
            )
            .await
        } else {
            self.match_candidates
                .iter()
                .enumerate()
                .map(|(candidate_id, candidate)| StringMatch {
                    candidate_id,
                    score: Default::default(),
                    positions: Default::default(),
                    string: candidate.string.clone(),
                })
                .collect()
        };

        if self.sort_completions {
            let completions = self.completions.borrow();

            let mut sortable_items: Vec<SortableMatch<'_>> = matches
                .into_iter()
                .map(|string_match| {
                    let completion = &completions[string_match.candidate_id];

                    let is_snippet = matches!(
                        &completion.source,
                        CompletionSource::Lsp { lsp_completion, .. }
                        if lsp_completion.kind == Some(CompletionItemKind::SNIPPET)
                    );

                    let sort_text =
                        if let CompletionSource::Lsp { lsp_completion, .. } = &completion.source {
                            lsp_completion.sort_text.as_deref()
                        } else {
                            None
                        };

                    let sort_key = completion.sort_key();

                    SortableMatch {
                        string_match,
                        is_snippet,
                        sort_text,
                        sort_key,
                    }
                })
                .collect();

            Self::sort_matches(&mut sortable_items, query, self.snippet_sort_order);

            matches = sortable_items
                .into_iter()
                .map(|sortable| sortable.string_match)
                .collect();
        }

        *self.entries.borrow_mut() = matches;
        self.selected_item = 0;
        // This keeps the display consistent when y_flipped.
        self.scroll_handle.scroll_to_item(0, ScrollStrategy::Top);
    }
}

#[derive(Debug)]
pub struct SortableMatch<'a> {
    pub string_match: StringMatch,
    pub is_snippet: bool,
    pub sort_text: Option<&'a str>,
    pub sort_key: (usize, &'a str),
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
}

impl CodeActionContents {
    pub fn new(
        mut tasks: Option<ResolvedTasks>,
        actions: Option<Rc<[AvailableCodeAction]>>,
        cx: &App,
    ) -> Self {
        if !cx.has_flag::<DebuggerFeatureFlag>() {
            if let Some(tasks) = &mut tasks {
                tasks
                    .templates
                    .retain(|(_, task)| !matches!(task.task_type(), task::TaskType::Debug(_)));
            }
        }

        Self {
            tasks: tasks.map(Rc::new),
            actions,
        }
    }

    pub fn tasks(&self) -> Option<&ResolvedTasks> {
        self.tasks.as_deref()
    }

    fn len(&self) -> usize {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => actions.len() + tasks.templates.len(),
            (Some(tasks), None) => tasks.templates.len(),
            (None, Some(actions)) => actions.len(),
            (None, None) => 0,
        }
    }

    fn is_empty(&self) -> bool {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => actions.is_empty() && tasks.templates.is_empty(),
            (Some(tasks), None) => tasks.templates.is_empty(),
            (None, Some(actions)) => actions.is_empty(),
            (None, None) => true,
        }
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
    }

    pub fn get(&self, index: usize) -> Option<CodeActionsItem> {
        match (&self.tasks, &self.actions) {
            (Some(tasks), Some(actions)) => {
                if index < tasks.templates.len() {
                    tasks
                        .templates
                        .get(index)
                        .cloned()
                        .map(|(kind, task)| CodeActionsItem::Task(kind, task))
                } else {
                    actions.get(index - tasks.templates.len()).map(|available| {
                        CodeActionsItem::CodeAction {
                            excerpt_id: available.excerpt_id,
                            action: available.action.clone(),
                            provider: available.provider.clone(),
                        }
                    })
                }
            }
            (Some(tasks), None) => tasks
                .templates
                .get(index)
                .cloned()
                .map(|(kind, task)| CodeActionsItem::Task(kind, task)),
            (None, Some(actions)) => {
                actions
                    .get(index)
                    .map(|available| CodeActionsItem::CodeAction {
                        excerpt_id: available.excerpt_id,
                        action: available.action.clone(),
                        provider: available.provider.clone(),
                    })
            }
            (None, None) => None,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum CodeActionsItem {
    Task(TaskSourceKind, ResolvedTask),
    CodeAction {
        excerpt_id: ExcerptId,
        action: CodeAction,
        provider: Rc<dyn CodeActionProvider>,
    },
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

    pub fn label(&self) -> String {
        match self {
            Self::CodeAction { action, .. } => action.lsp_action.title().to_owned(),
            Self::Task(_, task) => task.resolved_label.clone(),
        }
    }
}

pub struct CodeActionsMenu {
    pub actions: CodeActionContents,
    pub buffer: Entity<Buffer>,
    pub selected_item: usize,
    pub scroll_handle: UniformListScrollHandle,
    pub deployed_from_indicator: Option<DisplayRow>,
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

    fn visible(&self) -> bool {
        !self.actions.is_empty()
    }

    fn origin(&self) -> ContextMenuOrigin {
        if let Some(row) = self.deployed_from_indicator {
            ContextMenuOrigin::GutterIndicator(row)
        } else {
            ContextMenuOrigin::Cursor
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
            cx.entity().clone(),
            "code_actions_menu",
            self.actions.len(),
            move |_this, range, _, cx| {
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
                                    this.on_click(cx.listener(move |editor, _, window, cx| {
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
                                    }))
                                    .child(
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
                                    this.on_click(cx.listener(move |editor, _, window, cx| {
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
                                    }))
                                    .child(
                                        h_flex()
                                            .overflow_hidden()
                                            .child(task.resolved_label.replace("\n", ""))
                                            .when(selected, |this| {
                                                this.text_color(colors.text_accent)
                                            }),
                                    )
                                }),
                        )
                    })
                    .collect()
            },
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
                })
                .map(|(ix, _)| ix),
        )
        .with_sizing_behavior(ListSizingBehavior::Infer);

        Popover::new().child(list).into_any_element()
    }
}
