//! In-thread search bar for the agent panel.
//!
//! This is a small companion to `ThreadView` that lets the user grep the
//! currently-loaded thread without leaving the agent panel. It deliberately
//! does NOT use `BufferSearchBar`/`SearchableItem` because the agent panel
//! is not a workspace `Item`, so the toolbar/`ItemHandle` plumbing those
//! assume doesn't apply here. Instead it reuses the load-bearing primitives:
//!
//! - [`SearchQuery`] for query parsing (regex / case / whole-word toggles).
//! - [`search::SearchOption::as_button`] for the toggle button visuals — so
//!   the buttons render identically to the ones in `BufferSearchBar`.
//! - [`Markdown::set_search_highlights`] / `set_active_search_highlight` for
//!   inline highlight rendering, exactly as `MarkdownPreviewView` uses them.
//!
//! What gets searched: user message text, assistant message chunks
//! (`AssistantMessageChunk::Message` only), and tool-call labels. Results
//! are surfaced one match at a time via the next/prev controls.
//! Activating a match jumps the `ListState` to the entry that owns it
//! and asks the markdown view to auto-scroll to the source index.
//!
//! What is deliberately NOT searched, regardless of UI state:
//!
//! - `AssistantMessageChunk::Thought` blocks. The matcher filters these
//!   out in `collect_markdowns`.
//! - `ToolCall.content` — terminal command output, file content read by
//!   tools, diff editors, image previews. `collect_tool_call_markdowns`
//!   pushes only the tool-call label and ignores the content vector.
//!   **Expanding a tool call does NOT make its content searchable**;
//!   the matcher never walks the content list at all. Trade-off: the
//!   visible match count stays consistent with what's on screen, but
//!   users cannot grep tool output through this bar.
//!
//! Other out-of-scope items for this initial cut:
//!
//! - Searching tool-call raw input/output JSON, which is not a `Markdown`
//!   entity.
//! - Cross-thread / historic search.
use std::ops::Range;
use std::sync::Arc;

use acp_thread::{AcpThread, AgentThreadEntry, AssistantMessageChunk, ToolCall};
use collections::HashMap;
use editor::{Editor, EditorElement, EditorEvent, EditorStyle, HighlightKey};
use gpui::{
    Action, App, Context, Entity, EntityId, EventEmitter, FocusHandle, Focusable, Hsla,
    KeyContext, SharedString, Subscription, TextStyle, WeakEntity, Window, actions, relative,
    rems,
};
use markdown::Markdown;
use multi_buffer::{Anchor, MultiBufferOffset};
use project::search::SearchQuery;
use search::{SearchOption, SearchOptions, SearchSource};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{
    ActiveTheme, ButtonStyle, Color, IconButton, IconButtonShape, IconName, IntoElement, Label,
    LabelSize, Tooltip, div, h_flex, prelude::*, v_flex,
};
use util::paths::PathMatcher;

use crate::entry_view_state::EntryViewState;

actions!(
    agent,
    [
        /// Closes the thread search bar.
        DismissThreadSearch,
        /// Selects the next thread search match.
        SelectNextThreadMatch,
        /// Selects the previous thread search match.
        SelectPreviousThreadMatch,
    ]
);

/// Where a given match lives on screen. Past user messages are rendered
/// through an `Editor` (the `MessageEditor`'s inner editor), whereas
/// assistant messages and tool-call labels are rendered through `Markdown`
/// entities. The two have different highlight APIs, so we tag each match
/// with the entity it should be painted on.
#[derive(Clone)]
enum MatchTarget {
    Markdown {
        markdown: WeakEntity<Markdown>,
        /// Index of this match within `Markdown`'s highlight list.
        markdown_match_ix: usize,
    },
    Editor {
        editor: WeakEntity<Editor>,
        /// Anchor range in the editor's `MultiBuffer`, used both for
        /// `Editor::highlight_background` ranges and for autoscroll.
        anchor_range: Range<Anchor>,
        /// Index of this match within this editor's range list (passed to
        /// `highlight_background`'s color closure).
        editor_match_ix: usize,
    },
}

/// A single match: which entry it belongs to, where it lives, and the
/// byte offset inside the source string (used to autoscroll markdowns).
struct ThreadMatch {
    entry_ix: usize,
    target: MatchTarget,
    source_range: Range<usize>,
}

pub struct ThreadSearchBar {
    pub(super) query_editor: Entity<Editor>,
    options: SearchOptions,
    /// `None` means "not yet searched / empty query". `Some(...)` is the
    /// flat list of matches in thread order.
    matches: Vec<ThreadMatch>,
    /// Index into `matches` of the currently-active highlight, if any.
    active_match: Option<usize>,
    /// Set to true if the query is non-empty but failed to parse (e.g. bad
    /// regex). Used to color the input red, mirroring `BufferSearchBar`.
    query_error: bool,
    /// Human-readable error from the last failed query (e.g. regex parse
    /// error). `None` when the query is valid or empty.
    query_error_message: Option<SharedString>,
    /// The most-recently-used set of markdown entities. We hold weak refs
    /// so we can clear their highlights when the query changes or the bar
    /// is dismissed without leaking them.
    highlighted_markdowns: Vec<WeakEntity<Markdown>>,
    /// Same purpose as `highlighted_markdowns`, but for editor-backed
    /// matches (past user messages). Highlights here go through
    /// `Editor::highlight_background(HighlightKey::BufferSearchHighlights, …)`,
    /// which is what `Editor`'s own `SearchableItem` impl uses.
    highlighted_editors: Vec<WeakEntity<Editor>>,
    thread: Entity<AcpThread>,
    entry_view_state: Entity<EntryViewState>,
    on_activate_match: Arc<dyn Fn(usize, usize, &mut Window, &mut App)>,
    _subscriptions: Vec<Subscription>,
}

pub enum ThreadSearchBarEvent {
    /// Emitted when the user wants to focus the parent thread (e.g. pressed
    /// Escape). The parent restores focus to its own focus handle.
    Dismissed,
}

impl EventEmitter<ThreadSearchBarEvent> for ThreadSearchBar {}

impl Focusable for ThreadSearchBar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.query_editor.focus_handle(cx)
    }
}

impl ThreadSearchBar {
    /// Build a new search bar bound to `thread`. `on_activate_match` is
    /// invoked when the user navigates to a match; the args are
    /// `(entry_ix, source_index_within_markdown)`. The thread view uses
    /// this to scroll the `ListState` to the entry. Intra-entry scrolling
    /// is handled by the `Markdown` autoscroll request which we set
    /// internally before invoking the callback.
    pub fn new(
        thread: Entity<AcpThread>,
        entry_view_state: Entity<EntryViewState>,
        on_activate_match: Arc<dyn Fn(usize, usize, &mut Window, &mut App)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let query_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search this thread…", window, cx);
            editor
        });
        let editor_subscription = cx.subscribe_in(
            &query_editor,
            window,
            |this, _editor, event: &EditorEvent, window, cx| {
                if matches!(event, EditorEvent::Edited { .. } | EditorEvent::BufferEdited) {
                    this.update_matches(window, cx);
                }
            },
        );
        Self {
            query_editor,
            options: SearchOptions::NONE,
            matches: Vec::new(),
            active_match: None,
            query_error: false,
            query_error_message: None,
            highlighted_markdowns: Vec::new(),
            highlighted_editors: Vec::new(),
            thread,
            entry_view_state,
            on_activate_match,
            _subscriptions: vec![editor_subscription],
        }
    }

    /// Called by `ThreadView` after the user pressed `agent::ToggleSearch`
    /// or after a new thread becomes active. Focuses the query input and
    /// re-runs the existing query (if any) against the new thread state.
    pub fn focus_and_refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let focus_handle = self.query_editor.focus_handle(cx);
        focus_handle.focus(window, cx);
        self.query_editor.update(cx, |editor, cx| {
            editor.select_all(&editor::actions::SelectAll, window, cx);
        });
        self.update_matches(window, cx);
    }

    /// Test-only accessor for the total match count. The `matches` vec
    /// stores `ThreadMatch` which is private; exposing this scalar keeps
    /// the type sealed while letting `conversation_view::tests` observe
    /// the search result.
    #[cfg(test)]
    pub(super) fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Test-only accessor for the active match index.
    #[cfg(test)]
    pub(super) fn active_match_index(&self) -> Option<usize> {
        self.active_match
    }

    pub fn active_match_text(&self, cx: &App) -> Option<String> {
        if self.query_editor.read(cx).text(cx).is_empty() {
            return None;
        }
        match self.active_match {
            Some(ix) => Some(format!("{}/{}", ix + 1, self.matches.len())),
            None => Some(format!("0/{}", self.matches.len())),
        }
    }

    fn current_query(&self, cx: &App) -> String {
        self.query_editor.read(cx).text(cx)
    }

    /// Returns the `(query, error_message)` pair. On parse failure the
    /// query is `None` and the error message describes what went wrong
    /// (useful for surfacing regex parse errors to the user).
    fn build_query(&self, cx: &App) -> (Option<Arc<SearchQuery>>, Option<SharedString>) {
        let text = self.current_query(cx);
        if text.is_empty() {
            return (None, None);
        }
        let whole_word = self.options.contains(SearchOptions::WHOLE_WORD);
        let case_sensitive = self.options.contains(SearchOptions::CASE_SENSITIVE);
        let result = if self.options.contains(SearchOptions::REGEX) {
            SearchQuery::regex(
                text,
                whole_word,
                case_sensitive,
                false,
                false,
                PathMatcher::default(),
                PathMatcher::default(),
                false,
                None,
            )
        } else {
            SearchQuery::text(
                text,
                whole_word,
                case_sensitive,
                false,
                PathMatcher::default(),
                PathMatcher::default(),
                false,
                None,
            )
        };
        match result {
            Ok(q) => (Some(Arc::new(q)), None),
            Err(err) => (None, Some(SharedString::from(err.to_string()))),
        }
    }

    pub(super) fn update_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (query, err_msg) = self.build_query(cx);
        self.query_error = !self.current_query(cx).is_empty() && query.is_none();
        self.query_error_message = err_msg;
        // Always clear stale highlights from the previous query.
        for weak in self.highlighted_markdowns.drain(..) {
            if let Some(md) = weak.upgrade() {
                md.update(cx, |md, cx| md.clear_search_highlights(cx));
            }
        }
        for weak in self.highlighted_editors.drain(..) {
            if let Some(editor) = weak.upgrade() {
                editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights(
                        HighlightKey::BufferSearchHighlights,
                        cx,
                    );
                });
            }
        }
        self.matches.clear();
        self.active_match = None;

        let Some(query) = query else {
            cx.notify();
            return;
        };

        // For each entry, dispatch on type:
        //
        // * `UserMessage` is rendered through a `MessageEditor` (an `Editor`),
        //   not the markdown attached to the entry. Searching the markdown
        //   would count hits but paint nothing visible. Look up the editor
        //   from `EntryViewState`, search its buffer text, and paint via
        //   `Editor::highlight_background` — same path `Editor`'s own
        //   `SearchableItem` impl uses.
        // * Everything else (`AssistantMessage`, `ToolCall` label,
        //   `CompletedPlan`) keeps the markdown highlight path.
        let entry_count = self.thread.read(cx).entries().len();
        for entry_ix in 0..entry_count {
            let is_user_message = self
                .thread
                .read(cx)
                .entries()
                .get(entry_ix)
                .map(|entry| matches!(entry, AgentThreadEntry::UserMessage(_)))
                .unwrap_or(false);

            if is_user_message {
                let editor_entity = self
                    .entry_view_state
                    .read(cx)
                    .entry(entry_ix)
                    .and_then(|entry| entry.message_editor())
                    .map(|message_editor| message_editor.read(cx).editor().clone());
                let Some(editor_entity) = editor_entity else {
                    continue;
                };

                let snapshot = editor_entity.read(cx).buffer().read(cx).snapshot(cx);
                let text = snapshot.text();
                let ranges = query.search_str(&text);
                if ranges.is_empty() {
                    continue;
                }
                let anchor_ranges: Vec<Range<Anchor>> = ranges
                    .iter()
                    .map(|range| {
                        snapshot.anchor_before(MultiBufferOffset(range.start))
                            ..snapshot.anchor_after(MultiBufferOffset(range.end))
                    })
                    .collect();

                let weak_editor = editor_entity.downgrade();
                for (ix, range) in ranges.iter().enumerate() {
                    self.matches.push(ThreadMatch {
                        entry_ix,
                        target: MatchTarget::Editor {
                            editor: weak_editor.clone(),
                            anchor_range: anchor_ranges[ix].clone(),
                            editor_match_ix: ix,
                        },
                        source_range: range.clone(),
                    });
                }
                self.highlighted_editors.push(weak_editor);

                editor_entity.update(cx, |editor, cx| {
                    editor.highlight_background(
                        HighlightKey::BufferSearchHighlights,
                        &anchor_ranges,
                        |_index, theme| theme.colors().search_match_background,
                        cx,
                    );
                });
            } else {
                let markdowns = self
                    .thread
                    .read(cx)
                    .entries()
                    .get(entry_ix)
                    .map(collect_markdowns)
                    .unwrap_or_default();
                for markdown in markdowns {
                    let source = markdown.read(cx).source().to_string();
                    let ranges = query.search_str(&source);
                    if ranges.is_empty() {
                        continue;
                    }
                    let weak = markdown.downgrade();
                    for (ix, range) in ranges.iter().enumerate() {
                        self.matches.push(ThreadMatch {
                            entry_ix,
                            target: MatchTarget::Markdown {
                                markdown: weak.clone(),
                                markdown_match_ix: ix,
                            },
                            source_range: range.clone(),
                        });
                    }
                    self.highlighted_markdowns.push(weak);
                    markdown.update(cx, |md, cx| {
                        md.set_search_highlights(ranges, None, cx);
                    });
                }
            }
        }

        if !self.matches.is_empty() {
            self.activate_match(0, window, cx);
        } else {
            cx.notify();
        }
    }

    fn activate_match(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(m) = self.matches.get(ix) else {
            return;
        };
        let entry_ix = m.entry_ix;
        let source_index = m.source_range.start;
        let target = m.target.clone();

        // Walk all highlighted markdowns and update which one (if any) has
        // an active highlight set. We can't store the markdown_id inside
        // `Markdown` because per-entry markdowns share the same type and
        // don't know their "place" in our match list; instead we set
        // active only on the markdown that owns the current match, and
        // clear it on everything else.
        let (target_markdown_id, target_markdown_match_ix) = match &target {
            MatchTarget::Markdown {
                markdown,
                markdown_match_ix,
            } => (Some(markdown.entity_id()), Some(*markdown_match_ix)),
            MatchTarget::Editor { .. } => (None, None),
        };
        for weak in &self.highlighted_markdowns {
            if let Some(md) = weak.upgrade() {
                let is_target = Some(weak.entity_id()) == target_markdown_id;
                md.update(cx, |md, cx| {
                    if is_target {
                        md.set_active_search_highlight(target_markdown_match_ix, cx);
                        md.request_autoscroll_to_source_index(source_index, cx);
                    } else {
                        md.set_active_search_highlight(None, cx);
                    }
                });
            }
        }

        // For editor-backed matches we have to re-apply `highlight_background`
        // because the active-vs-inactive distinction lives inside the color
        // closure (per `Editor::SearchableItem::update_matches`). Compute,
        // per editor, the ordered range list and which of its matches (if
        // any) is the active one, then re-paint.
        let target_editor_id = match &target {
            MatchTarget::Editor { editor, .. } => Some(editor.entity_id()),
            MatchTarget::Markdown { .. } => None,
        };
        let target_editor_match_ix = match &target {
            MatchTarget::Editor {
                editor_match_ix, ..
            } => Some(*editor_match_ix),
            MatchTarget::Markdown { .. } => None,
        };
        let mut per_editor: HashMap<EntityId, (WeakEntity<Editor>, Vec<Range<Anchor>>)> =
            HashMap::default();
        for m in &self.matches {
            if let MatchTarget::Editor {
                editor,
                anchor_range,
                ..
            } = &m.target
            {
                let entry = per_editor
                    .entry(editor.entity_id())
                    .or_insert_with(|| (editor.clone(), Vec::new()));
                entry.1.push(anchor_range.clone());
            }
        }
        for (editor_id, (weak_editor, ranges)) in per_editor {
            let Some(editor) = weak_editor.upgrade() else {
                continue;
            };
            let active_ix = if Some(editor_id) == target_editor_id {
                target_editor_match_ix
            } else {
                None
            };
            editor.update(cx, |editor, cx| {
                editor.highlight_background(
                    HighlightKey::BufferSearchHighlights,
                    &ranges,
                    move |index, theme| {
                        if active_ix == Some(*index) {
                            theme.colors().search_active_match_background
                        } else {
                            theme.colors().search_match_background
                        }
                    },
                    cx,
                );
            });
        }

        self.active_match = Some(ix);
        (self.on_activate_match)(entry_ix, source_index, window, cx);
        cx.notify();
    }

    pub(super) fn select_next_match(
        &mut self,
        _: &SelectNextThreadMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.matches.is_empty() {
            return;
        }
        let next = match self.active_match {
            Some(ix) => (ix + 1) % self.matches.len(),
            None => 0,
        };
        self.activate_match(next, window, cx);
    }

    pub(super) fn select_prev_match(
        &mut self,
        _: &SelectPreviousThreadMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.matches.is_empty() {
            return;
        }
        let prev = match self.active_match {
            Some(ix) => {
                if ix == 0 {
                    self.matches.len() - 1
                } else {
                    ix - 1
                }
            }
            None => self.matches.len() - 1,
        };
        self.activate_match(prev, window, cx);
    }

    fn dismiss(&mut self, _: &DismissThreadSearch, _window: &mut Window, cx: &mut Context<Self>) {
        self.clear_highlights(cx);
        cx.emit(ThreadSearchBarEvent::Dismissed);
    }

    /// Called when the bar is being torn down or hidden. Clears every
    /// markdown entity we touched so we don't leave stale yellow highlights
    /// when the user toggles search off.
    pub fn clear_highlights(&mut self, cx: &mut Context<Self>) {
        for weak in self.highlighted_markdowns.drain(..) {
            if let Some(md) = weak.upgrade() {
                md.update(cx, |md, cx| md.clear_search_highlights(cx));
            }
        }
        for weak in self.highlighted_editors.drain(..) {
            if let Some(editor) = weak.upgrade() {
                editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights(
                        HighlightKey::BufferSearchHighlights,
                        cx,
                    );
                });
            }
        }
        self.matches.clear();
        self.active_match = None;
        cx.notify();
    }

    pub(super) fn toggle_case_sensitive(
        &mut self,
        _: &search::ToggleCaseSensitive,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.options.toggle(SearchOptions::CASE_SENSITIVE);
        self.update_matches(window, cx);
    }

    pub(super) fn toggle_whole_word(
        &mut self,
        _: &search::ToggleWholeWord,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.options.toggle(SearchOptions::WHOLE_WORD);
        self.update_matches(window, cx);
    }

    pub(super) fn toggle_regex(
        &mut self,
        _: &search::ToggleRegex,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.options.toggle(SearchOptions::REGEX);
        self.update_matches(window, cx);
    }

    /// Handler for `search::FocusSearch` (bound to Cmd/Ctrl+F inside the bar's
    /// context). Mirrors `BufferSearchBar`'s behavior: focus the query input
    /// and select all its text so the next keystroke replaces the query.
    pub(super) fn focus_search(
        &mut self,
        _: &search::FocusSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let focus_handle = self.query_editor.focus_handle(cx);
        focus_handle.focus(window, cx);
        self.query_editor.update(cx, |editor, cx| {
            editor.select_all(&editor::actions::SelectAll, window, cx);
        });
    }
}

impl Render for ThreadSearchBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.query_editor.focus_handle(cx);
        let theme = cx.theme().colors();
        let has_matches = !self.matches.is_empty();
        let query_empty = self.query_editor.read(cx).text(cx).is_empty();
        // The query text turns red when the query is non-empty but produces
        // no matches (or fails to parse as regex). Border stays at the
        // default color: feedback comes from text + counter, not from a
        // surrounding box — matches the convention the user prefers.
        let in_error_state = self.query_error || (!query_empty && !has_matches);
        let border_color = theme.border;

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AcpThreadSearchBar");

        let counter_text = self.active_match_text(cx).unwrap_or_default();
        // Counter stays muted on "no matches" rather than red, mirroring
        // `BufferSearchBar` / Markdown Preview Search. The red signal comes from
        // the query text turning red (via `in_error_state`); doubling it on the
        // counter was too noisy.
        let counter_color = if has_matches {
            Color::Default
        } else {
            Color::Muted
        };

        let bar_row = h_flex()
            // Tie this element to the query editor's focus handle so the
            // `AcpThreadSearchBar` key context lands in the editor's
            // dispatch chain when typing in the query. The handlers below
            // are belt-and-suspenders — `ThreadView` also forwards these
            // actions from its own (outer) element so they fire reliably
            // when focus is somewhere else (e.g. the message editor) while
            // the bar is open.
            .track_focus(&focus_handle)
            .key_context(key_context)
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::select_next_match))
            .on_action(cx.listener(Self::select_prev_match))
            .on_action(cx.listener(Self::toggle_case_sensitive))
            .on_action(cx.listener(Self::toggle_whole_word))
            .on_action(cx.listener(Self::toggle_regex))
            .on_action(cx.listener(Self::focus_search))
            .w_full()
            .gap_2()
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(theme.border)
            .bg(theme.toolbar_background)
            .child(
                input_box(border_color)
                    .flex_1()
                    .min_w_32()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .py_1()
                            .child(render_query_input(&self.query_editor, in_error_state, cx)),
                    )
                    .child(
                        h_flex()
                            .flex_none()
                            .gap_1()
                            .child(SearchOption::CaseSensitive.as_button(
                                self.options,
                                SearchSource::Buffer,
                                focus_handle.clone(),
                            ))
                            .child(SearchOption::WholeWord.as_button(
                                self.options,
                                SearchSource::Buffer,
                                focus_handle.clone(),
                            ))
                            .child(SearchOption::Regex.as_button(
                                self.options,
                                SearchSource::Buffer,
                                focus_handle.clone(),
                            )),
                    ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .gap_1()
                    .child(nav_button(
                        "thread-search-prev",
                        IconName::ChevronLeft,
                        !has_matches,
                        "Previous Match",
                        &SelectPreviousThreadMatch,
                        focus_handle.clone(),
                    ))
                    .child(nav_button(
                        "thread-search-next",
                        IconName::ChevronRight,
                        !has_matches,
                        "Next Match",
                        &SelectNextThreadMatch,
                        focus_handle.clone(),
                    ))
                    .child(
                        div()
                            .ml_1()
                            .min_w(rems(2.5))
                            .child(Label::new(counter_text).size(LabelSize::Small).color(counter_color)),
                    )
                    .child(nav_button(
                        "thread-search-dismiss",
                        IconName::Close,
                        false,
                        "Close Search",
                        &DismissThreadSearch,
                        focus_handle,
                    )),
            );

        // Stack the bar above an error message row so a bad regex etc.
        // gets a textual explanation, matching the `MarkdownPreview`
        // search behavior the user pointed at as the reference UX.
        let error_row = self.query_error_message.clone().map(|msg| {
            div()
                .w_full()
                .px_2()
                .py_0p5()
                .border_b_1()
                .border_color(theme.border)
                .bg(theme.toolbar_background)
                .child(Label::new(msg).size(LabelSize::Small).color(Color::Error))
        });

        v_flex().w_full().child(bar_row).children(error_row)
    }
}

fn input_box(border_color: Hsla) -> gpui::Div {
    h_flex()
        .min_h_8()
        .pl_2()
        .pr_1()
        .border_1()
        .border_color(border_color)
        .rounded_md()
}

fn render_query_input(
    editor: &Entity<Editor>,
    has_error: bool,
    app: &App,
) -> impl IntoElement {
    let theme = app.theme().colors();
    let (color, use_syntax) = if has_error {
        (ui::Color::Error.color(app), false)
    } else {
        (theme.text, true)
    };
    let settings = ThemeSettings::get_global(app);
    let text_style = TextStyle {
        color,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: rems(0.875).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.3),
        ..TextStyle::default()
    };
    let mut style = EditorStyle {
        background: theme.toolbar_background,
        local_player: app.theme().players().local(),
        text: text_style,
        ..EditorStyle::default()
    };
    if use_syntax {
        style.syntax = app.theme().syntax().clone();
    }
    EditorElement::new(editor, style)
}

fn nav_button(
    id: &'static str,
    icon: IconName,
    disabled: bool,
    tooltip: &'static str,
    action: &'static dyn Action,
    focus_handle: FocusHandle,
) -> IconButton {
    let action_for_dispatch = action;
    IconButton::new(id, icon)
        .style(ButtonStyle::Subtle)
        .shape(IconButtonShape::Square)
        .disabled(disabled)
        .on_click({
            let focus_handle = focus_handle.clone();
            move |_, window, cx| {
                if !focus_handle.is_focused(window) {
                    window.focus(&focus_handle, cx);
                }
                window.dispatch_action(action_for_dispatch.boxed_clone(), cx);
            }
        })
        .tooltip(move |_window, cx| Tooltip::for_action_in(tooltip, action, &focus_handle, cx))
}

/// Collects every `Entity<Markdown>` reachable from a thread entry,
/// in display order. Used by the search bar to enumerate searchable
/// content. Returns an empty Vec for `CompletedPlan` entries (plan
/// entries contain markdown, but they're attached to other entries and
/// already covered there); not searching them avoids double-counting.
fn collect_markdowns(entry: &AgentThreadEntry) -> Vec<Entity<Markdown>> {
    let mut out = Vec::new();
    match entry {
        AgentThreadEntry::UserMessage(_) => {
            // User messages render through `MessageEditor`'s inner `Editor`,
            // not through `Markdown`. `ThreadSearchBar::update_matches` handles
            // them via `EntryViewState`-driven editor lookup + a separate
            // `Editor::highlight_background` paint path; we deliberately skip
            // them here so we don't double-count or paint invisible markdown
            // highlights.
        }
        AgentThreadEntry::AssistantMessage(message) => {
            // Only search the visible-by-default `Message` chunks. `Thought`
            // chunks are collapsed behind a "Thinking" disclosure by
            // default, and surfacing matches inside them produces invisible
            // navigation jumps (the user is scrolled to an entry that
            // appears to contain no match) — same failure mode as searching
            // collapsed tool-call content.
            for chunk in &message.chunks {
                if let AssistantMessageChunk::Message { block } = chunk
                    && let Some(md) = block.markdown()
                {
                    out.push(md.clone());
                }
            }
        }
        AgentThreadEntry::ToolCall(tool_call) => {
            collect_tool_call_markdowns(tool_call, &mut out);
        }
        AgentThreadEntry::CompletedPlan(_) => {}
    }
    out
}

fn collect_tool_call_markdowns(tool_call: &ToolCall, out: &mut Vec<Entity<Markdown>>) {
    // Only search the tool-call label, not its rendered content. Content
    // (e.g. command output, file contents inlined by a tool) is hidden
    // behind an expand toggle by default; matching inside collapsed blocks
    // produces high match counts with no visible highlights, which is
    // user-hostile. The label is always visible, so this keeps
    // "what I see is what is searched" coherent. Users who want to grep
    // expanded tool output can use a normal buffer search on the text
    // after expanding the block.
    out.push(tool_call.label.clone());
    let _ = tool_call.content;
}


