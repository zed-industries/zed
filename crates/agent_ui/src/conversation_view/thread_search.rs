use std::ops::Range;
use std::sync::Arc;

use acp_thread::{AgentThreadEntry, AssistantMessageChunk, ContentBlock, ToolCallContent};
use agent_client_protocol::schema as acp;
use collections::{HashMap, HashSet};
use editor::display_map::HighlightKey;
use editor::{Anchor, Editor, MultiBufferOffset};
use gpui::{
    App, AppContext, Context, Entity, EntityId, EventEmitter, ListOffset, SharedString, Task,
    Window, px,
};
use markdown::Markdown;
use project::search::SearchQuery;
use workspace::item::{Item, ItemBufferKind, ItemEvent};
use workspace::searchable::{
    Direction, SearchEvent, SearchOptions as SearchableOptions, SearchToken, SearchableItem,
    SearchableItemHandle,
};

use crate::entry_view_state::EntryViewState;

use super::ThreadView;

/// Where inside the thread a match lives, so we can auto-expand disclosures
/// when the user has opted into searching hidden context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchOrigin {
    AlwaysVisible,
    ThinkingBlock { chunk_index: usize },
    ToolCallContent { id: acp::ToolCallId },
    UserMessage,
}

/// A match in the thread is either inside a rendered markdown entity (assistant
/// messages, thought blocks, tool-call content, plan entries, tool labels) or
/// inside a user-message `Editor`. User messages are rendered through a
/// `MessageEditor`, not the markdown entity that `UserMessage::content` holds,
/// so highlights for them must be routed through the editor directly.
#[derive(Debug, Clone)]
pub enum MatchTarget {
    Markdown(Entity<Markdown>),
    Editor(Entity<Editor>),
}

impl MatchTarget {
    fn entity_id(&self) -> EntityId {
        match self {
            Self::Markdown(markdown) => markdown.entity_id(),
            Self::Editor(editor) => editor.entity_id(),
        }
    }
}

impl PartialEq for MatchTarget {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Markdown(a), Self::Markdown(b)) => a == b,
            (Self::Editor(a), Self::Editor(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for MatchTarget {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadSearchMatch {
    pub entry_index: usize,
    pub target: MatchTarget,
    pub byte_range: Range<usize>,
    pub origin: MatchOrigin,
}

impl EventEmitter<SearchEvent> for ThreadView {}

impl Item for ThreadView {
    type Event = super::AcpThreadViewEvent;

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.thread
            .read(cx)
            .title()
            .unwrap_or_else(|| SharedString::new_static(crate::DEFAULT_THREAD_TITLE))
    }

    fn buffer_kind(&self, _cx: &App) -> ItemBufferKind {
        ItemBufferKind::Singleton
    }

    fn to_item_events(_event: &Self::Event, _f: &mut dyn FnMut(ItemEvent)) {}

    fn as_searchable(
        &self,
        handle: &Entity<Self>,
        _cx: &App,
    ) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }
}

impl SearchableItem for ThreadView {
    type Match = ThreadSearchMatch;

    fn supported_options(&self) -> SearchableOptions {
        SearchableOptions {
            case: true,
            word: true,
            regex: true,
            replacement: false,
            selection: false,
            select_all: false,
            find_in_results: false,
            include_hidden: true,
        }
    }

    fn clear_matches(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let had_matches = !self.search_matches.is_empty();
        self.search_matches.clear();
        self.active_search_match_index = None;
        self.active_search_position = None;
        for markdown in std::mem::take(&mut self.highlighted_markdowns) {
            markdown.update(cx, |md, cx| md.clear_search_highlights(cx));
        }
        for editor in std::mem::take(&mut self.highlighted_user_message_editors) {
            editor.update(cx, |editor, cx| {
                editor.clear_background_highlights(HighlightKey::BufferSearchHighlights, cx);
            });
        }
        self.collapse_search_auto_expanded_sections(cx);
        if had_matches {
            cx.emit(SearchEvent::MatchesInvalidated);
        }
        cx.notify();
    }

    fn update_matches(
        &mut self,
        matches: &[Self::Match],
        active_match_index: Option<usize>,
        _token: SearchToken,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let changed = self.search_matches.as_slice() != matches;

        let per_target = group_matches_by_target(matches, active_match_index);

        let new_ids: HashSet<EntityId> = per_target
            .iter()
            .map(|(target, _, _)| target.entity_id())
            .collect();
        for markdown in std::mem::take(&mut self.highlighted_markdowns) {
            if !new_ids.contains(&markdown.entity_id()) {
                markdown.update(cx, |md, cx| md.clear_search_highlights(cx));
            }
        }
        for editor in std::mem::take(&mut self.highlighted_user_message_editors) {
            if !new_ids.contains(&editor.entity_id()) {
                editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights(HighlightKey::BufferSearchHighlights, cx);
                });
            }
        }

        let mut new_highlighted_markdowns = Vec::new();
        let mut new_highlighted_editors = Vec::new();
        for (target, ranges, active_local) in per_target {
            match target {
                MatchTarget::Markdown(markdown) => {
                    markdown.update(cx, |md, cx| {
                        md.set_search_highlights(ranges, active_local, cx);
                    });
                    new_highlighted_markdowns.push(markdown);
                }
                MatchTarget::Editor(editor) => {
                    apply_editor_highlights(&editor, &ranges, active_local, cx);
                    new_highlighted_editors.push(editor);
                }
            }
        }
        self.highlighted_markdowns = new_highlighted_markdowns;
        self.highlighted_user_message_editors = new_highlighted_editors;

        self.search_matches = matches.to_vec();
        self.active_search_match_index = active_match_index;
        self.active_search_position = active_match_index
            .and_then(|i| matches.get(i))
            .map(|m| (m.entry_index, m.byte_range.start));
        if changed {
            cx.emit(SearchEvent::MatchesInvalidated);
        }
        cx.notify();
    }

    fn query_suggestion(
        &mut self,
        _ignore_settings: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> String {
        String::new()
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        _token: SearchToken,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(m) = matches.get(index) else {
            return;
        };
        self.active_search_match_index = Some(index);
        self.active_search_position = Some((m.entry_index, m.byte_range.start));

        self.ensure_origin_expanded(m.entry_index, &m.origin, cx);

        let target_id = m.target.entity_id();
        let local_index = matches[..=index]
            .iter()
            .filter(|other| other.target.entity_id() == target_id)
            .count()
            - 1;

        for other in self.highlighted_markdowns.clone() {
            if other.entity_id() != target_id {
                other.update(cx, |md, cx| md.set_active_search_highlight(None, cx));
            }
        }
        // Editor highlights capture the active index inside a color closure at
        // insertion time, so whenever the active index changes we have to
        // re-apply highlights for every editor — otherwise a previously-active
        // editor would keep showing the orange "active" color on a stale match.
        let per_target = group_matches_by_target(matches, Some(index));
        for (target, ranges, active_local) in per_target {
            if let MatchTarget::Editor(editor) = target {
                apply_editor_highlights(&editor, &ranges, active_local, cx);
            }
        }
        match &m.target {
            MatchTarget::Markdown(markdown) => {
                markdown.update(cx, |md, cx| {
                    md.set_active_search_highlight(Some(local_index), cx);
                    md.request_autoscroll_to_source_index(m.byte_range.start, cx);
                });
            }
            MatchTarget::Editor(active_editor) => {
                autoscroll_editor_to_range(active_editor, m.byte_range.clone(), cx);
            }
        }

        self.list_state.scroll_to(ListOffset {
            item_ix: m.entry_index,
            offset_in_item: px(0.0),
        });
        cx.emit(SearchEvent::ActiveMatchChanged);
        cx.notify();
    }

    fn select_matches(
        &mut self,
        _matches: &[Self::Match],
        _token: SearchToken,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn replace(
        &mut self,
        _: &Self::Match,
        _: &SearchQuery,
        _token: SearchToken,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    fn find_matches(
        &mut self,
        query: Arc<SearchQuery>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Self::Match>> {
        let include_hidden = query.include_hidden();

        if !include_hidden {
            self.collapse_search_auto_expanded_sections(cx);
        }

        let expanded_tool_calls = self.expanded_tool_calls.clone();
        let expanded_thinking_blocks = self.expanded_thinking_blocks.clone();
        let entry_view_state = self.entry_view_state.read(cx);

        let mut entries_data: Vec<(usize, Vec<(MatchTarget, String, MatchOrigin)>)> = Vec::new();
        for (entry_index, entry) in self.thread.read(cx).entries().iter().enumerate() {
            let mut targets = Vec::new();
            collect_searchable_targets(
                entry,
                cx,
                include_hidden,
                &expanded_tool_calls,
                entry_index,
                &expanded_thinking_blocks,
                entry_view_state,
                &mut targets,
            );
            if !targets.is_empty() {
                entries_data.push((entry_index, targets));
            }
        }
        cx.background_spawn(async move {
            let mut matches = Vec::new();
            for (entry_index, targets) in entries_data {
                for (target, source, origin) in targets {
                    for byte_range in query.search_str(&source) {
                        matches.push(ThreadSearchMatch {
                            entry_index,
                            target: target.clone(),
                            byte_range,
                            origin: origin.clone(),
                        });
                    }
                }
            }
            matches
        })
    }

    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Self::Match],
        _token: SearchToken,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        if matches.is_empty() {
            return None;
        }

        let anchor = self.active_search_position.unwrap_or((0, 0));

        match direction {
            Direction::Next => matches
                .iter()
                .position(|m| (m.entry_index, m.byte_range.start) >= anchor)
                .or(Some(0)),
            Direction::Prev => matches
                .iter()
                .rposition(|m| (m.entry_index, m.byte_range.start) <= anchor)
                .or(Some(matches.len().saturating_sub(1))),
        }
    }
}

impl ThreadView {
    fn ensure_origin_expanded(
        &mut self,
        entry_index: usize,
        origin: &MatchOrigin,
        cx: &mut Context<Self>,
    ) {
        match origin {
            MatchOrigin::AlwaysVisible | MatchOrigin::UserMessage => {}
            MatchOrigin::ThinkingBlock { chunk_index } => {
                let key = (entry_index, *chunk_index);
                if !self.expanded_thinking_blocks.contains(&key) {
                    self.expanded_thinking_blocks.insert(key);
                    self.search_auto_expanded_thinking_blocks.insert(key);
                    cx.notify();
                }
            }
            MatchOrigin::ToolCallContent { id } => {
                if !self.expanded_tool_calls.contains(id) {
                    self.expanded_tool_calls.insert(id.clone());
                    self.search_auto_expanded_tool_calls.insert(id.clone());
                    cx.notify();
                }
            }
        }
    }

    pub(crate) fn collapse_search_auto_expanded_sections(&mut self, cx: &mut Context<Self>) {
        if self.search_auto_expanded_tool_calls.is_empty()
            && self.search_auto_expanded_thinking_blocks.is_empty()
        {
            return;
        }

        for id in std::mem::take(&mut self.search_auto_expanded_tool_calls) {
            self.expanded_tool_calls.remove(&id);
        }
        for key in std::mem::take(&mut self.search_auto_expanded_thinking_blocks) {
            if !self.user_toggled_thinking_blocks.contains(&key) {
                self.expanded_thinking_blocks.remove(&key);
            }
        }
        cx.notify();
    }
}

fn collect_searchable_targets(
    entry: &AgentThreadEntry,
    cx: &App,
    include_hidden: bool,
    expanded_tool_calls: &HashSet<acp::ToolCallId>,
    entry_index: usize,
    expanded_thinking_blocks: &HashSet<(usize, usize)>,
    entry_view_state: &EntryViewState,
    out: &mut Vec<(MatchTarget, String, MatchOrigin)>,
) {
    match entry {
        AgentThreadEntry::UserMessage(_message) => {
            // User messages are rendered through a `MessageEditor`, so search
            // the editor's buffer text directly rather than the (unrendered)
            // markdown entity on `UserMessage::content`. The two texts diverge
            // for image chunks — the markdown uses ``Image`` while the editor
            // uses a mention link — so the editor text is the source of truth.
            if let Some(editor) = entry_view_state
                .entry(entry_index)
                .and_then(|entry| entry.message_editor())
                .map(|message_editor| message_editor.read(cx).editor().clone())
            {
                let text = editor.read(cx).text(cx);
                out.push((MatchTarget::Editor(editor), text, MatchOrigin::UserMessage));
            }
        }
        AgentThreadEntry::AssistantMessage(message) => {
            for (chunk_index, chunk) in message.chunks.iter().enumerate() {
                match chunk {
                    AssistantMessageChunk::Message { block } => {
                        push_content_block(block, cx, MatchOrigin::AlwaysVisible, out);
                    }
                    AssistantMessageChunk::Thought { block } => {
                        let key = (entry_index, chunk_index);
                        if include_hidden || expanded_thinking_blocks.contains(&key) {
                            push_content_block(
                                block,
                                cx,
                                MatchOrigin::ThinkingBlock { chunk_index },
                                out,
                            );
                        }
                    }
                }
            }
        }
        AgentThreadEntry::ToolCall(call) => {
            // For Execute tool calls, `call.label` carries the full command,
            // but the visible rendering is a code-fenced markdown: once a
            // terminal has been created, `terminal.command()`; otherwise (in
            // the pre-approval "Run Command" preview, or for rejected
            // commands) `call.command_markdown`. Indexing those fenced
            // entities — rather than the plain label — means highlights end
            // up on the markdown the user actually sees.
            let mut has_terminal = false;
            for terminal in call.terminals() {
                has_terminal = true;
                push_markdown(
                    terminal.read(cx).command(),
                    cx,
                    MatchOrigin::AlwaysVisible,
                    out,
                );
            }
            if !has_terminal {
                if let Some(command_markdown) = &call.command_markdown {
                    push_markdown(command_markdown, cx, MatchOrigin::AlwaysVisible, out);
                } else {
                    push_markdown(&call.label, cx, MatchOrigin::AlwaysVisible, out);
                }
            }
            let content_visible = include_hidden || expanded_tool_calls.contains(&call.id);
            if content_visible {
                for content in &call.content {
                    if let ToolCallContent::ContentBlock(block) = content {
                        push_content_block(
                            block,
                            cx,
                            MatchOrigin::ToolCallContent {
                                id: call.id.clone(),
                            },
                            out,
                        );
                    }
                }
            }
        }
        AgentThreadEntry::CompletedPlan(plan_entries) => {
            for plan_entry in plan_entries {
                push_markdown(&plan_entry.content, cx, MatchOrigin::AlwaysVisible, out);
            }
        }
    }
}

fn push_content_block(
    block: &ContentBlock,
    cx: &App,
    origin: MatchOrigin,
    out: &mut Vec<(MatchTarget, String, MatchOrigin)>,
) {
    if let Some(markdown) = block.markdown() {
        push_markdown(markdown, cx, origin, out);
    }
}

fn push_markdown(
    markdown: &Entity<Markdown>,
    cx: &App,
    origin: MatchOrigin,
    out: &mut Vec<(MatchTarget, String, MatchOrigin)>,
) {
    out.push((
        MatchTarget::Markdown(markdown.clone()),
        markdown.read(cx).source().to_string(),
        origin,
    ));
}

fn group_matches_by_target(
    matches: &[ThreadSearchMatch],
    active_match_index: Option<usize>,
) -> Vec<(MatchTarget, Vec<Range<usize>>, Option<usize>)> {
    let mut per_target: Vec<(MatchTarget, Vec<Range<usize>>, Option<usize>)> = Vec::new();
    let mut index_map: HashMap<EntityId, usize> = HashMap::default();
    for (global_idx, m) in matches.iter().enumerate() {
        let group_idx = *index_map.entry(m.target.entity_id()).or_insert_with(|| {
            per_target.push((m.target.clone(), Vec::new(), None));
            per_target.len() - 1
        });
        let (_, ranges, active_local) = &mut per_target[group_idx];
        ranges.push(m.byte_range.clone());
        if Some(global_idx) == active_match_index {
            *active_local = Some(ranges.len() - 1);
        }
    }
    per_target
}

fn apply_editor_highlights(
    editor: &Entity<Editor>,
    ranges: &[Range<usize>],
    active_local: Option<usize>,
    cx: &mut Context<ThreadView>,
) {
    let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
    let anchor_ranges: Vec<Range<Anchor>> = ranges
        .iter()
        .map(|range| {
            let start = snapshot.anchor_after(MultiBufferOffset(range.start));
            let end = snapshot.anchor_before(MultiBufferOffset(range.end));
            start..end
        })
        .collect();
    editor.update(cx, |editor, cx| {
        editor.highlight_background(
            HighlightKey::BufferSearchHighlights,
            &anchor_ranges,
            move |index, theme| {
                if active_local == Some(*index) {
                    theme.colors().search_active_match_background
                } else {
                    theme.colors().search_match_background
                }
            },
            cx,
        );
    });
}

fn autoscroll_editor_to_range(
    editor: &Entity<Editor>,
    byte_range: Range<usize>,
    cx: &mut Context<ThreadView>,
) {
    let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
    let anchor = snapshot.anchor_before(MultiBufferOffset(byte_range.start));
    editor.update(cx, |editor, cx| {
        editor.request_autoscroll(
            editor::scroll::Autoscroll::Strategy(
                editor::scroll::AutoscrollStrategy::Center,
                Some(anchor),
            ),
            cx,
        );
    });
}
