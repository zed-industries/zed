use std::ops::Range;
use std::sync::Arc;

use acp_thread::{AgentThreadEntry, AssistantMessageChunk, ContentBlock, ToolCallContent};
use collections::{HashMap, HashSet};
use gpui::{
    App, AppContext, Context, Entity, EntityId, EventEmitter, ListOffset, SharedString, Task,
    Window, px,
};
use markdown::Markdown;
use project::search::SearchQuery;
use workspace::item::{Item, ItemBufferKind, ItemEvent};
use workspace::searchable::{
    Direction, SearchEvent, SearchOptions, SearchToken, SearchableItem, SearchableItemHandle,
};

use super::ThreadView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadSearchMatch {
    pub entry_index: usize,
    pub markdown: Entity<Markdown>,
    pub byte_range: Range<usize>,
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

    fn supported_options(&self) -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
            replacement: false,
            selection: false,
            select_all: false,
            find_in_results: false,
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

        // Group matches by their Markdown entity while preserving the global order.
        let mut per_markdown: Vec<(Entity<Markdown>, Vec<Range<usize>>, Option<usize>)> = Vec::new();
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

        // Clear highlights on previously-highlighted markdowns that no longer have matches.
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

        // Count how many matches sharing this markdown precede (and include) the
        // activated one — that's the active index within the per-markdown range list.
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

        // Place the entry at the viewport top as a coarse jump. The Markdown
        // autoscroll request above then propagates a `window.request_autoscroll`
        // during paint, which the List honors to bring the specific match into
        // view if it sits deeper in a tall entry.
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
        let mut entries_data: Vec<(usize, Vec<(Entity<Markdown>, String)>)> = Vec::new();
        for (entry_index, entry) in self.thread.read(cx).entries().iter().enumerate() {
            let mut markdowns = Vec::new();
            collect_searchable_markdowns(entry, cx, &mut markdowns);
            if !markdowns.is_empty() {
                entries_data.push((entry_index, markdowns));
            }
        }
        cx.background_spawn(async move {
            let mut matches = Vec::new();
            for (entry_index, markdowns) in entries_data {
                for (markdown, source) in markdowns {
                    for byte_range in query.search_str(&source) {
                        matches.push(ThreadSearchMatch {
                            entry_index,
                            markdown: markdown.clone(),
                            byte_range,
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

fn collect_searchable_markdowns(
    entry: &AgentThreadEntry,
    cx: &App,
    out: &mut Vec<(Entity<Markdown>, String)>,
) {
    match entry {
        AgentThreadEntry::UserMessage(message) => {
            push_content_block(&message.content, cx, out);
        }
        AgentThreadEntry::AssistantMessage(message) => {
            for chunk in &message.chunks {
                let block = match chunk {
                    AssistantMessageChunk::Message { block } => block,
                    AssistantMessageChunk::Thought { block } => block,
                };
                push_content_block(block, cx, out);
            }
        }
        AgentThreadEntry::ToolCall(call) => {
            push_markdown(&call.label, cx, out);
            for content in &call.content {
                if let ToolCallContent::ContentBlock(block) = content {
                    push_content_block(block, cx, out);
                }
            }
        }
        AgentThreadEntry::CompletedPlan(plan_entries) => {
            for plan_entry in plan_entries {
                push_markdown(&plan_entry.content, cx, out);
            }
        }
    }
}

fn push_content_block(
    block: &ContentBlock,
    cx: &App,
    out: &mut Vec<(Entity<Markdown>, String)>,
) {
    if let Some(markdown) = block.markdown() {
        push_markdown(markdown, cx, out);
    }
}

fn push_markdown(markdown: &Entity<Markdown>, cx: &App, out: &mut Vec<(Entity<Markdown>, String)>) {
    out.push((markdown.clone(), markdown.read(cx).source().to_string()));
}
