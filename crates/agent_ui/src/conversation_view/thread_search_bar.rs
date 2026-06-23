use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

use acp_thread::{
    AcpThread, AcpThreadEvent, AgentThreadEntry, AssistantMessageChunk, ContentBlock,
    ToolCallContent,
};
use collections::HashMap;
use editor::{
    Editor, EditorElement, EditorEvent, EditorStyle, HighlightKey, SelectionEffects,
    scroll::Autoscroll,
};
use gpui::{
    Action, Entity, EntityId, EventEmitter, FocusHandle, Focusable, KeyContext, Subscription, Task,
    TextStyle, WeakEntity, actions, prelude::*,
};
use markdown::Markdown;
use multi_buffer::{Anchor, MultiBufferOffset, MultiBufferSnapshot};
use project::search::SearchQuery;
use search::{SearchOption, SearchOptions, SearchSource};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{IconButtonShape, Tooltip, prelude::*};
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

/// Debounce for streaming thread updates, which can fire once per streamed
/// chunk. Query edits are handled immediately instead (see the query editor
/// subscription in `ThreadSearchBar::new`).
pub(super) const SEARCH_UPDATE_DEBOUNCE: Duration = Duration::from_millis(150);

/// Search hits can be painted on either markdown or past-message editors.
#[derive(Clone)]
enum MatchTarget {
    Markdown {
        markdown: WeakEntity<Markdown>,
        markdown_match_ix: usize,
    },
    Editor {
        editor: WeakEntity<Editor>,
        anchor_range: Range<Anchor>,
        editor_match_ix: usize,
    },
}

impl MatchTarget {
    fn entity_id(&self) -> EntityId {
        match self {
            MatchTarget::Markdown { markdown, .. } => markdown.entity_id(),
            MatchTarget::Editor { editor, .. } => editor.entity_id(),
        }
    }

    /// Index of this hit within its painted entity (markdown- or editor-local).
    fn match_ix(&self) -> usize {
        match self {
            MatchTarget::Markdown {
                markdown_match_ix, ..
            } => *markdown_match_ix,
            MatchTarget::Editor {
                editor_match_ix, ..
            } => *editor_match_ix,
        }
    }
}

struct ThreadMatch {
    entry_ix: usize,
    target: MatchTarget,
    source_range: Range<usize>,
}

impl ThreadMatch {
    /// Stable identity used to re-locate the active match across a rescan.
    fn key(&self) -> MatchKey {
        MatchKey {
            entry_ix: self.entry_ix,
            entity_id: self.target.entity_id(),
            source_range: self.source_range.clone(),
        }
    }
}

#[derive(PartialEq)]
struct MatchKey {
    entry_ix: usize,
    entity_id: EntityId,
    source_range: Range<usize>,
}

enum SearchTarget {
    Editor {
        entry_ix: usize,
        editor: Entity<Editor>,
        snapshot: MultiBufferSnapshot,
    },
    Markdown {
        entry_ix: usize,
        markdown: Entity<Markdown>,
        source: SharedString,
    },
}

enum ScannedTarget {
    Editor {
        entry_ix: usize,
        editor: Entity<Editor>,
        ranges: Vec<Range<usize>>,
        anchor_ranges: Vec<Range<Anchor>>,
    },
    Markdown {
        entry_ix: usize,
        markdown: Entity<Markdown>,
        ranges: Vec<Range<usize>>,
    },
}

pub struct ThreadSearchBar {
    pub(super) query_editor: Entity<Editor>,
    options: SearchOptions,
    matches: Vec<ThreadMatch>,
    active_match: Option<usize>,
    query_error: bool,
    query_error_message: Option<SharedString>,
    highlighted_markdowns: Vec<WeakEntity<Markdown>>,
    highlighted_editors: Vec<WeakEntity<Editor>>,
    thread: Entity<AcpThread>,
    entry_view_state: Entity<EntryViewState>,
    on_activate_match: Arc<dyn Fn(usize, &mut Window, &mut App)>,
    is_active: bool,
    _update_matches_task: Option<Task<()>>,
    _search_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

pub enum ThreadSearchBarEvent {
    Dismissed,
}

impl EventEmitter<ThreadSearchBarEvent> for ThreadSearchBar {}

impl Focusable for ThreadSearchBar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.query_editor.focus_handle(cx)
    }
}

impl ThreadSearchBar {
    pub fn new(
        thread: Entity<AcpThread>,
        entry_view_state: Entity<EntryViewState>,
        on_activate_match: Arc<dyn Fn(usize, &mut Window, &mut App)>,
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
                if matches!(
                    event,
                    EditorEvent::Edited { .. } | EditorEvent::BufferEdited
                ) {
                    // Re-scan immediately so typing feels responsive
                    this._update_matches_task = None;
                    this.update_matches(window, cx);
                }
            },
        );
        let thread_subscription = cx.subscribe_in(
            &thread,
            window,
            |this, _thread, event: &AcpThreadEvent, window, cx| {
                if this.is_active
                    && matches!(
                        event,
                        AcpThreadEvent::NewEntry
                            | AcpThreadEvent::EntryUpdated(_)
                            | AcpThreadEvent::EntriesRemoved(_)
                    )
                {
                    this.schedule_update_matches(window, cx);
                }
            },
        );
        cx.on_release(|this, cx| {
            this.clear_highlights_impl(cx);
        })
        .detach();
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
            is_active: false,
            _update_matches_task: None,
            _search_task: None,
            _subscriptions: vec![editor_subscription, thread_subscription],
        }
    }

    pub fn focus_and_refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_active = true;
        self.focus_query_and_select_all(window, cx);
        self.update_matches(window, cx);
    }

    fn focus_query_and_select_all(&self, window: &mut Window, cx: &mut Context<Self>) {
        let focus_handle = self.query_editor.focus_handle(cx);
        focus_handle.focus(window, cx);
        self.query_editor.update(cx, |editor, cx| {
            editor.select_all(&editor::actions::SelectAll, window, cx);
        });
    }

    fn schedule_update_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self._update_matches_task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(SEARCH_UPDATE_DEBOUNCE).await;
            this.update_in(cx, |this, window, cx| this.update_matches(window, cx))
                .ok();
        }));
    }

    #[cfg(test)]
    pub(super) fn match_count(&self) -> usize {
        self.matches.len()
    }

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
        let previous_active_match_ix = self.active_match;
        let previous_active_key = previous_active_match_ix
            .and_then(|ix| self.matches.get(ix))
            .map(ThreadMatch::key);

        let (query, err_msg) = self.build_query(cx);
        self.query_error = !self.current_query(cx).is_empty() && query.is_none();
        self.query_error_message = err_msg;

        let Some(query) = query else {
            self.clear_results(cx);
            cx.notify();
            return;
        };

        let mut targets: Vec<SearchTarget> = Vec::new();
        let thread = self.thread.read(cx);
        let entry_view_state = self.entry_view_state.read(cx);
        for (entry_ix, entry) in thread.entries().iter().enumerate() {
            match entry {
                // Past user messages render through `MessageEditor`, not markdown.
                AgentThreadEntry::UserMessage(_) => {
                    let editor = entry_view_state
                        .entry(entry_ix)
                        .and_then(|view_entry| view_entry.message_editor())
                        .map(|message_editor| message_editor.read(cx).editor().clone());
                    let Some(editor) = editor else {
                        continue;
                    };
                    let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
                    targets.push(SearchTarget::Editor {
                        entry_ix,
                        editor,
                        snapshot,
                    });
                }
                _ => {
                    for markdown in collect_markdowns(entry_ix, entry, &entry_view_state, cx) {
                        let source = markdown.read(cx).source().clone();
                        targets.push(SearchTarget::Markdown {
                            entry_ix,
                            markdown,
                            source,
                        });
                    }
                }
            }
        }

        if targets.is_empty() {
            self.clear_results(cx);
            cx.notify();
            return;
        }

        self._search_task = Some(cx.spawn_in(window, async move |this, cx| {
            let scanned = cx
                .background_spawn(async move {
                    targets
                        .into_iter()
                        .filter_map(|target| match target {
                            SearchTarget::Editor {
                                entry_ix,
                                editor,
                                snapshot,
                            } => {
                                let ranges = query.search_str(&snapshot.text());
                                if ranges.is_empty() {
                                    return None;
                                }
                                let anchor_ranges = ranges
                                    .iter()
                                    .map(|range| {
                                        snapshot.anchor_before(MultiBufferOffset(range.start))
                                            ..snapshot.anchor_after(MultiBufferOffset(range.end))
                                    })
                                    .collect();
                                Some(ScannedTarget::Editor {
                                    entry_ix,
                                    editor,
                                    ranges,
                                    anchor_ranges,
                                })
                            }
                            SearchTarget::Markdown {
                                entry_ix,
                                markdown,
                                source,
                            } => {
                                let ranges = query.search_str(&source);
                                if ranges.is_empty() {
                                    return None;
                                }
                                Some(ScannedTarget::Markdown {
                                    entry_ix,
                                    markdown,
                                    ranges,
                                })
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                this.apply_search_results(
                    scanned,
                    previous_active_key,
                    previous_active_match_ix,
                    window,
                    cx,
                );
            })
            .ok();
        }));
    }

    fn apply_search_results(
        &mut self,
        scanned: Vec<ScannedTarget>,
        previous_active_key: Option<MatchKey>,
        previous_active_match_ix: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear_match_highlights(cx);
        self.matches.clear();
        self.active_match = None;

        for target in scanned {
            match target {
                ScannedTarget::Editor {
                    entry_ix,
                    editor,
                    ranges,
                    anchor_ranges,
                } => {
                    let weak_editor = editor.downgrade();
                    for (ix, (range, anchor_range)) in ranges.iter().zip(&anchor_ranges).enumerate()
                    {
                        self.matches.push(ThreadMatch {
                            entry_ix,
                            target: MatchTarget::Editor {
                                editor: weak_editor.clone(),
                                anchor_range: anchor_range.clone(),
                                editor_match_ix: ix,
                            },
                            source_range: range.clone(),
                        });
                    }
                    self.highlighted_editors.push(weak_editor);
                }
                ScannedTarget::Markdown {
                    entry_ix,
                    markdown,
                    ranges,
                } => {
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
            let preserved_ix = previous_active_key
                .as_ref()
                .and_then(|key| self.matches.iter().position(|m| &m.key() == key));
            let active_match_ix = preserved_ix
                .or_else(|| previous_active_match_ix.filter(|ix| *ix < self.matches.len()))
                .unwrap_or(0);
            let scroll_to_match = preserved_ix.is_none();
            self.activate_match(active_match_ix, scroll_to_match, window, cx);
        } else {
            cx.notify();
        }
    }

    fn activate_match(
        &mut self,
        ix: usize,
        scroll_to_match: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(m) = self.matches.get(ix) else {
            return;
        };
        let entry_ix = m.entry_ix;
        let source_index = m.source_range.start;
        let target = m.target.clone();
        // Markdown and editor entity ids never collide, so the active hit is
        // simply the painted entity whose id equals the target's.
        let target_entity_id = target.entity_id();
        let target_match_ix = target.match_ix();

        for weak in &self.highlighted_markdowns {
            if let Some(markdown) = weak.upgrade() {
                let active = (weak.entity_id() == target_entity_id).then_some(target_match_ix);
                markdown.update(cx, |markdown, cx| {
                    markdown.set_active_search_highlight(active, cx);
                    if active.is_some() && scroll_to_match {
                        markdown.request_autoscroll_to_source_index(source_index, cx);
                    }
                });
            }
        }

        // Editor highlight colors are computed by index, so repaint on navigation.
        let mut per_editor: HashMap<EntityId, (WeakEntity<Editor>, Vec<Range<Anchor>>)> =
            HashMap::default();
        for mat in &self.matches {
            if let MatchTarget::Editor {
                editor,
                anchor_range,
                ..
            } = &mat.target
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
            let active_ix = (editor_id == target_entity_id).then_some(target_match_ix);
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

        if scroll_to_match
            && let MatchTarget::Editor {
                editor,
                anchor_range,
                ..
            } = &target
            && let Some(editor) = editor.upgrade()
        {
            let anchor_range = anchor_range.clone();
            editor.update(cx, |editor, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::fit()).from_search(true),
                    window,
                    cx,
                    |selections| selections.select_anchor_ranges([anchor_range]),
                );
            });
        }

        self.active_match = Some(ix);
        if scroll_to_match {
            (self.on_activate_match)(entry_ix, window, cx);
        }
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
        self.activate_match(next, true, window, cx);
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
        self.activate_match(prev, true, window, cx);
    }

    fn dismiss(&mut self, _: &DismissThreadSearch, _window: &mut Window, cx: &mut Context<Self>) {
        self.clear_highlights(cx);
        cx.emit(ThreadSearchBarEvent::Dismissed);
    }

    pub fn clear_highlights(&mut self, cx: &mut Context<Self>) {
        self.clear_highlights_impl(cx);
        cx.notify();
    }

    fn clear_highlights_impl(&mut self, cx: &mut App) {
        self.clear_results(cx);
        self.is_active = false;
        self._update_matches_task = None;
    }

    /// Drops all painted highlights, recorded matches, and any in-flight scan.
    fn clear_results(&mut self, cx: &mut App) {
        self.clear_match_highlights(cx);
        self.matches.clear();
        self.active_match = None;
        self._search_task = None;
    }

    fn clear_match_highlights(&mut self, cx: &mut App) {
        for weak in self.highlighted_markdowns.drain(..) {
            if let Some(md) = weak.upgrade() {
                md.update(cx, |md, cx| md.clear_search_highlights(cx));
            }
        }
        for weak in self.highlighted_editors.drain(..) {
            if let Some(editor) = weak.upgrade() {
                editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights(HighlightKey::BufferSearchHighlights, cx);
                });
            }
        }
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

    pub(super) fn focus_search(
        &mut self,
        _: &search::FocusSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_query_and_select_all(window, cx);
    }
}

impl Render for ThreadSearchBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.query_editor.focus_handle(cx);
        let theme = cx.theme().colors();

        let has_matches = !self.matches.is_empty();
        let query_empty = self.query_editor.read(cx).text(cx).is_empty();
        let in_error_state = self.query_error || (!query_empty && !has_matches);

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AcpThreadSearchBar");

        let counter_text = self.active_match_text(cx).unwrap_or_default();

        let bar_row = h_flex()
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
            .child(
                h_flex()
                    .min_h_8()
                    .min_w_32()
                    .flex_1()
                    .px_1p5()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.editor_background)
                    .rounded_md()
                    .child(div().px_1().flex_1().child(render_query_input(
                        &self.query_editor,
                        in_error_state,
                        cx,
                    )))
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
                        div().ml_1().min_w(rems(2.5)).child(
                            Label::new(counter_text)
                                .size(LabelSize::Small)
                                .when(!has_matches, |this| this.color(Color::Muted)),
                        ),
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

        let error_row = self
            .query_error_message
            .clone()
            .map(|msg| Label::new(msg).size(LabelSize::Small).color(Color::Error));

        v_flex()
            .w_full()
            .p_1p5()
            .bg(theme.panel_background)
            .border_b_1()
            .border_color(theme.border.opacity(0.6))
            .child(bar_row)
            .children(error_row)
    }
}

fn render_query_input(editor: &Entity<Editor>, has_error: bool, app: &App) -> impl IntoElement {
    let theme = app.theme().colors();
    let (color, use_syntax) = if has_error {
        (Color::Error.color(app), false)
    } else {
        (theme.text, true)
    };

    let settings = ThemeSettings::get_global(app);

    let text_style = TextStyle {
        color,
        font_family: settings.ui_font.family.clone(),
        font_features: settings.ui_font.features.clone(),
        font_fallbacks: settings.ui_font.fallbacks.clone(),
        font_size: rems(0.875).into(),
        font_weight: settings.ui_font.weight,
        line_height: relative(1.3),
        ..TextStyle::default()
    };
    let mut style = EditorStyle {
        background: theme.editor_background,
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

fn collect_markdowns(
    entry_ix: usize,
    entry: &AgentThreadEntry,
    entry_view_state: &EntryViewState,
    cx: &App,
) -> Vec<Entity<Markdown>> {
    let mut out = Vec::new();
    match entry {
        AgentThreadEntry::UserMessage(_) => {}
        AgentThreadEntry::AssistantMessage(message) => {
            for (chunk_ix, chunk) in message.chunks.iter().enumerate() {
                match chunk {
                    AssistantMessageChunk::Message { block } => {
                        if let Some(md) = block.markdown() {
                            out.push(md.clone());
                        }
                    }
                    AssistantMessageChunk::Thought { block }
                        if entry_view_state
                            .thinking_block_state((entry_ix, chunk_ix), cx)
                            .0 =>
                    {
                        if let Some(md) = block.markdown() {
                            out.push(md.clone());
                        }
                    }
                    AssistantMessageChunk::Thought { .. } => {}
                }
            }
        }
        AgentThreadEntry::ToolCall(tool_call) => {
            out.push(tool_call.label.clone());
            if entry_view_state.is_tool_call_expanded(&tool_call.id) {
                out.extend(
                    tool_call
                        .content
                        .iter()
                        .filter_map(|content| match content {
                            ToolCallContent::ContentBlock(ContentBlock::Markdown { markdown }) => {
                                Some(markdown.clone())
                            }
                            ToolCallContent::ContentBlock(
                                ContentBlock::Empty
                                | ContentBlock::ResourceLink { .. }
                                | ContentBlock::Image { .. },
                            )
                            | ToolCallContent::Diff(_)
                            | ToolCallContent::Terminal(_) => None,
                        }),
                );
            }
        }
        AgentThreadEntry::CompletedPlan(entries) => {
            out.extend(entries.iter().map(|e| e.content.clone()))
        }
        AgentThreadEntry::ContextCompaction(compaction)
            if entry_view_state.is_compaction_expanded(entry_ix) =>
        {
            if let Some(summary) = &compaction.summary {
                out.push(summary.clone());
            }
        }
        AgentThreadEntry::ContextCompaction(_) => {}
    }
    out
}
