use std::ops::Range;
use std::sync::Arc;

use acp_thread::{AgentThreadEntry, AssistantMessageChunk, ContentBlock, ToolCallContent};
use agent_client_protocol::schema as acp;
use collections::{HashMap, HashSet};
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

use super::ThreadView;

/// Where inside the thread a match lives, so we can auto-expand disclosures
/// when the user has opted into searching hidden context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchOrigin {
    AlwaysVisible,
    ThinkingBlock { chunk_index: usize },
    ToolCallContent { id: acp::ToolCallId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadSearchMatch {
    pub entry_index: usize,
    pub markdown: Entity<Markdown>,
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

        let mut per_markdown: Vec<(Entity<Markdown>, Vec<Range<usize>>, Option<usize>)> =
            Vec::new();
        let mut index_map: HashMap<EntityId, usize> = HashMap::default();
        for (global_idx, m) in matches.iter().enumerate() {
            let group_idx = *index_map.entry(m.markdown.entity_id()).or_insert_with(|| {
                per_markdown.push((m.markdown.clone(), Vec::new(), None));
                per_markdown.len() - 1
            });
            let (_, ranges, active_local) = &mut per_markdown[group_idx];
            ranges.push(m.byte_range.clone());
            if Some(global_idx) == active_match_index {
                *active_local = Some(ranges.len() - 1);
            }
        }

        let new_ids: HashSet<EntityId> = per_markdown
            .iter()
            .map(|(markdown, _, _)| markdown.entity_id())
            .collect();
        for markdown in std::mem::take(&mut self.highlighted_markdowns) {
            if !new_ids.contains(&markdown.entity_id()) {
                markdown.update(cx, |md, cx| md.clear_search_highlights(cx));
            }
        }

        for (markdown, ranges, active_local) in &per_markdown {
            let ranges = ranges.clone();
            let active_local = *active_local;
            markdown.update(cx, |md, cx| {
                md.set_search_highlights(ranges, active_local, cx);
            });
        }
        self.highlighted_markdowns = per_markdown
            .into_iter()
            .map(|(markdown, _, _)| markdown)
            .collect();

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

        let target_id = m.markdown.entity_id();
        let local_index = matches[..=index]
            .iter()
            .filter(|other| other.markdown.entity_id() == target_id)
            .count()
            - 1;

        for other in self.highlighted_markdowns.clone() {
            if other.entity_id() != target_id {
                other.update(cx, |md, cx| md.set_active_search_highlight(None, cx));
            }
        }
        m.markdown.update(cx, |md, cx| {
            md.set_active_search_highlight(Some(local_index), cx);
            md.request_autoscroll_to_source_index(m.byte_range.start, cx);
        });

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

        let mut entries_data: Vec<(usize, Vec<(Entity<Markdown>, String, MatchOrigin)>)> =
            Vec::new();
        for (entry_index, entry) in self.thread.read(cx).entries().iter().enumerate() {
            let mut markdowns = Vec::new();
            collect_searchable_markdowns(
                entry,
                cx,
                include_hidden,
                &expanded_tool_calls,
                entry_index,
                &expanded_thinking_blocks,
                &mut markdowns,
            );
            if !markdowns.is_empty() {
                entries_data.push((entry_index, markdowns));
            }
        }
        cx.background_spawn(async move {
            let mut matches = Vec::new();
            for (entry_index, markdowns) in entries_data {
                for (markdown, source, origin) in markdowns {
                    for byte_range in query.search_str(&source) {
                        matches.push(ThreadSearchMatch {
                            entry_index,
                            markdown: markdown.clone(),
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
            MatchOrigin::AlwaysVisible => {}
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

fn collect_searchable_markdowns(
    entry: &AgentThreadEntry,
    cx: &App,
    include_hidden: bool,
    expanded_tool_calls: &HashSet<acp::ToolCallId>,
    entry_index: usize,
    expanded_thinking_blocks: &HashSet<(usize, usize)>,
    out: &mut Vec<(Entity<Markdown>, String, MatchOrigin)>,
) {
    match entry {
        AgentThreadEntry::UserMessage(message) => {
            push_content_block(&message.content, cx, MatchOrigin::AlwaysVisible, out);
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
            push_markdown(&call.label, cx, MatchOrigin::AlwaysVisible, out);
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
    out: &mut Vec<(Entity<Markdown>, String, MatchOrigin)>,
) {
    if let Some(markdown) = block.markdown() {
        push_markdown(markdown, cx, origin, out);
    }
}

fn push_markdown(
    markdown: &Entity<Markdown>,
    cx: &App,
    origin: MatchOrigin,
    out: &mut Vec<(Entity<Markdown>, String, MatchOrigin)>,
) {
    out.push((
        markdown.clone(),
        markdown.read(cx).source().to_string(),
        origin,
    ));
}
