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

use acp_thread::{
    AcpThread, AgentThreadEntry, AssistantMessageChunk, ToolCall, ToolCallContent,
};
use editor::{Editor, EditorElement, EditorEvent, EditorStyle};
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Hsla, KeyContext,
    Subscription, TextStyle, WeakEntity, Window, actions, relative, rems,
};
use markdown::Markdown;
use project::search::SearchQuery;
use search::{SearchOption, SearchOptions, SearchSource};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{
    ActiveTheme, ButtonStyle, Color, IconButton, IconButtonShape, IconName, IntoElement, Label,
    LabelSize, Tooltip, div, h_flex, prelude::*,
};
use util::{ResultExt as _, paths::PathMatcher};

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

/// A single match: the entry index in the thread, the `Markdown` entity
/// that owns it, the index of the match within that markdown's highlight
/// list, and the source-offset range inside the markdown's source string.
struct ThreadMatch {
    entry_ix: usize,
    markdown: WeakEntity<Markdown>,
    markdown_match_ix: usize,
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
    /// The most-recently-used set of markdown entities. We hold weak refs
    /// so we can clear their highlights when the query changes or the bar
    /// is dismissed without leaking them.
    highlighted: Vec<WeakEntity<Markdown>>,
    thread: Entity<AcpThread>,
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
        on_activate_match: Arc<dyn Fn(usize, usize, &mut Window, &mut App)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let query_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search thread…", window, cx);
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
            highlighted: Vec::new(),
            thread,
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

    fn build_query(&self, cx: &App) -> Option<Arc<SearchQuery>> {
        let text = self.current_query(cx);
        if text.is_empty() {
            return None;
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
        result.log_err().map(Arc::new)
    }

    pub(super) fn update_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let query = self.build_query(cx);
        self.query_error = !self.current_query(cx).is_empty() && query.is_none();
        // Always clear stale highlights from the previous query.
        for weak in self.highlighted.drain(..) {
            if let Some(md) = weak.upgrade() {
                md.update(cx, |md, cx| md.clear_search_highlights(cx));
            }
        }
        self.matches.clear();
        self.active_match = None;

        let Some(query) = query else {
            cx.notify();
            return;
        };

        // For each entry, walk its markdown entities, run the query, and
        // record matches. Push the matched ranges to the markdown via
        // `set_search_highlights` so they're painted inline (yellow bg,
        // active match is emphasized when we call `set_active`).
        let entry_markdowns: Vec<(usize, Vec<Entity<Markdown>>)> = self
            .thread
            .read(cx)
            .entries()
            .iter()
            .enumerate()
            .map(|(ix, entry)| (ix, collect_markdowns(entry)))
            .collect();

        for (entry_ix, markdowns) in entry_markdowns {
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
                        markdown: weak.clone(),
                        markdown_match_ix: ix,
                        source_range: range.clone(),
                    });
                }
                self.highlighted.push(weak);
                markdown.update(cx, |md, cx| {
                    md.set_search_highlights(ranges, None, cx);
                });
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
        let target_markdown = m.markdown.clone();
        let target_markdown_match_ix = m.markdown_match_ix;

        // Walk all highlighted markdowns and update which one (if any) has
        // an active highlight set. We can't store the markdown_id inside
        // `Markdown` because per-entry markdowns share the same type and
        // don't know their "place" in our match list; instead we set
        // active only on the markdown that owns the current match, and
        // clear it on everything else.
        for weak in &self.highlighted {
            if let Some(md) = weak.upgrade() {
                let is_target = weak.entity_id() == target_markdown.entity_id();
                md.update(cx, |md, cx| {
                    if is_target {
                        md.set_active_search_highlight(Some(target_markdown_match_ix), cx);
                        md.request_autoscroll_to_source_index(source_index, cx);
                    } else {
                        md.set_active_search_highlight(None, cx);
                    }
                });
            }
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
        for weak in self.highlighted.drain(..) {
            if let Some(md) = weak.upgrade() {
                md.update(cx, |md, cx| md.clear_search_highlights(cx));
            }
        }
        self.matches.clear();
        self.active_match = None;
        cx.notify();
    }

    fn toggle_case_sensitive(
        &mut self,
        _: &search::ToggleCaseSensitive,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.options.toggle(SearchOptions::CASE_SENSITIVE);
        self.update_matches(window, cx);
    }

    fn toggle_whole_word(
        &mut self,
        _: &search::ToggleWholeWord,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.options.toggle(SearchOptions::WHOLE_WORD);
        self.update_matches(window, cx);
    }

    fn toggle_regex(
        &mut self,
        _: &search::ToggleRegex,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.options.toggle(SearchOptions::REGEX);
        self.update_matches(window, cx);
    }
}

impl Render for ThreadSearchBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.query_editor.focus_handle(cx);
        let theme = cx.theme().colors();
        let border_color = if self.query_error {
            ui::Color::Error.color(cx)
        } else {
            theme.border
        };

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AcpThreadSearchBar");

        let has_matches = !self.matches.is_empty();
        let counter_text = self.active_match_text(cx).unwrap_or_default();
        let counter_color = if has_matches {
            Color::Default
        } else if self.query_editor.read(cx).text(cx).is_empty() {
            Color::Muted
        } else {
            Color::Error
        };

        h_flex()
            .key_context(key_context)
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::select_next_match))
            .on_action(cx.listener(Self::select_prev_match))
            .on_action(cx.listener(Self::toggle_case_sensitive))
            .on_action(cx.listener(Self::toggle_whole_word))
            .on_action(cx.listener(Self::toggle_regex))
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
                            .child(render_query_input(&self.query_editor, self.query_error, cx)),
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
            )
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
        AgentThreadEntry::UserMessage(message) => {
            if let Some(md) = message.content.markdown() {
                out.push(md.clone());
            }
        }
        AgentThreadEntry::AssistantMessage(message) => {
            for chunk in &message.chunks {
                let block = match chunk {
                    AssistantMessageChunk::Message { block } => block,
                    AssistantMessageChunk::Thought { block } => block,
                };
                if let Some(md) = block.markdown() {
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
    out.push(tool_call.label.clone());
    for content in &tool_call.content {
        if let ToolCallContent::ContentBlock(block) = content
            && let Some(md) = block.markdown()
        {
            out.push(md.clone());
        }
    }
}


