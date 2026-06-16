use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

use acp_thread::{
    AcpThread, AcpThreadEvent, AgentThreadEntry, AssistantMessageChunk, ContentBlock,
    ToolCallContent,
};
use collections::HashMap;
use editor::{Editor, EditorElement, EditorEvent, EditorStyle, HighlightKey, SelectionEffects};
use gpui::{
    Action, App, Context, Entity, EntityId, EventEmitter, FocusHandle, Focusable, Hsla, KeyContext,
    SharedString, Subscription, Task, TextStyle, WeakEntity, Window, actions, relative, rems,
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

/// Debounce for query edits and streaming thread updates.
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

struct ThreadMatch {
    entry_ix: usize,
    target: MatchTarget,
    source_range: Range<usize>,
}

struct MatchPosition {
    entry_ix: usize,
    target: MatchPositionTarget,
    source_range: Range<usize>,
}

enum MatchPositionTarget {
    Markdown(EntityId),
    Editor(EntityId),
}

impl MatchPosition {
    fn new(m: &ThreadMatch) -> Self {
        Self {
            entry_ix: m.entry_ix,
            target: match &m.target {
                MatchTarget::Markdown { markdown, .. } => {
                    MatchPositionTarget::Markdown(markdown.entity_id())
                }
                MatchTarget::Editor { editor, .. } => {
                    MatchPositionTarget::Editor(editor.entity_id())
                }
            },
            source_range: m.source_range.clone(),
        }
    }

    fn matches(&self, m: &ThreadMatch) -> bool {
        self.entry_ix == m.entry_ix
            && self.source_range == m.source_range
            && match (&self.target, &m.target) {
                (
                    MatchPositionTarget::Markdown(entity_id),
                    MatchTarget::Markdown { markdown, .. },
                ) => *entity_id == markdown.entity_id(),
                (MatchPositionTarget::Editor(entity_id), MatchTarget::Editor { editor, .. }) => {
                    *entity_id == editor.entity_id()
                }
                _ => false,
            }
    }
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
                    this.schedule_update_matches(window, cx);
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
        let previous_active_match = previous_active_match_ix
            .and_then(|ix| self.matches.get(ix))
            .map(MatchPosition::new);

        let (query, err_msg) = self.build_query(cx);
        self.query_error = !self.current_query(cx).is_empty() && query.is_none();
        self.query_error_message = err_msg;
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
        self.matches.clear();
        self.active_match = None;

        let Some(query) = query else {
            cx.notify();
            return;
        };

        // Past user messages render through `MessageEditor`, not markdown.
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
                    .map(|entry| {
                        let entry_view_state = self.entry_view_state.read(cx);
                        collect_markdowns(entry_ix, entry, &entry_view_state, cx)
                    })
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
            let active_match_ix = previous_active_match
                .and_then(|position| self.matches.iter().position(|m| position.matches(m)))
                .or_else(|| previous_active_match_ix.filter(|ix| *ix < self.matches.len()))
                .unwrap_or(0);
            self.activate_match(active_match_ix, window, cx);
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

        // Editor highlight colors are computed by index, so repaint on navigation.
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

        if let MatchTarget::Editor {
            editor,
            anchor_range,
            ..
        } = &target
            && let Some(editor) = editor.upgrade()
        {
            let anchor_range = anchor_range.clone();
            editor.update(cx, |editor, cx| {
                editor.change_selections(
                    SelectionEffects::no_scroll().from_search(true),
                    window,
                    cx,
                    |selections| selections.select_anchor_ranges([anchor_range]),
                );
            });
        }

        self.active_match = Some(ix);
        (self.on_activate_match)(entry_ix, window, cx);
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

    pub fn clear_highlights(&mut self, cx: &mut Context<Self>) {
        self.clear_highlights_impl(cx);
        cx.notify();
    }

    fn clear_highlights_impl(&mut self, cx: &mut App) {
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
        self.matches.clear();
        self.active_match = None;
        self.is_active = false;
        self._update_matches_task = None;
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
        let border_color = theme.border;

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AcpThreadSearchBar");

        let counter_text = self.active_match_text(cx).unwrap_or_default();
        let counter_color = if has_matches {
            Color::Default
        } else {
            Color::Muted
        };

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
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(theme.border)
            .bg(theme.toolbar_background)
            .child(
                input_box(border_color)
                    .flex_1()
                    .min_w_32()
                    .child(div().flex_1().min_w_0().py_1().child(render_query_input(
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
                                .color(counter_color),
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

fn render_query_input(editor: &Entity<Editor>, has_error: bool, app: &App) -> impl IntoElement {
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
