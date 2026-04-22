use std::sync::Arc;

use gpui::{App, AppContext, Context, Entity, EventEmitter, ListOffset, SharedString, Task, Window, px};
use project::search::SearchQuery;
use workspace::item::{Item, ItemBufferKind, ItemEvent};
use workspace::searchable::{
    Direction, SearchEvent, SearchOptions, SearchToken, SearchableItem, SearchableItemHandle,
};

use super::ThreadView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadSearchMatch {
    pub entry_index: usize,
    pub byte_range: std::ops::Range<usize>,
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
        if changed {
            self.search_matches = matches.to_vec();
            cx.emit(SearchEvent::MatchesInvalidated);
        }
        self.active_search_match_index = active_match_index;
        if let Some(idx) = active_match_index
            && let Some(m) = matches.get(idx)
        {
            self.active_search_position = Some((m.entry_index, m.byte_range.start));
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
        if let Some(m) = matches.get(index) {
            self.active_search_match_index = Some(index);
            self.active_search_position = Some((m.entry_index, m.byte_range.start));
            self.list_state.scroll_to(ListOffset {
                item_ix: m.entry_index,
                offset_in_item: px(0.0),
            });
            cx.emit(SearchEvent::ActiveMatchChanged);
            cx.notify();
        }
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
        let entries: Vec<(usize, String)> = self
            .thread
            .read(cx)
            .entries()
            .iter()
            .enumerate()
            .map(|(idx, entry)| (idx, entry.to_markdown(cx)))
            .collect();
        cx.background_spawn(async move {
            let mut matches = Vec::new();
            for (entry_index, text) in entries {
                for byte_range in query.search_str(&text) {
                    matches.push(ThreadSearchMatch {
                        entry_index,
                        byte_range,
                    });
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
