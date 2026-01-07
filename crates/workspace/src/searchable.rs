use std::{any::Any, sync::Arc};

use any_vec::AnyVec;
use gpui::{
    AnyView, AnyWeakEntity, App, Context, Entity, EventEmitter, Subscription, Task, WeakEntity,
    Window,
};
use project::search::SearchQuery;

use crate::{
    ItemHandle,
    item::{Item, WeakItemHandle},
};

#[derive(Clone, Debug)]
pub enum CollapseDirection {
    Collapsed,
    Expanded,
}

#[derive(Clone, Debug)]
pub enum SearchEvent {
    MatchesInvalidated,
    ActiveMatchChanged,
    ResultsCollapsedChanged(CollapseDirection),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Direction {
    Prev,
    #[default]
    Next,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Prev => Direction::Next,
            Direction::Next => Direction::Prev,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SearchOptions {
    pub case: bool,
    pub word: bool,
    pub regex: bool,
    /// Specifies whether the  supports search & replace.
    pub replacement: bool,
    pub selection: bool,
    pub find_in_results: bool,
}

// Whether to always select the current selection (even if empty)
// or to use the default (restoring the previous search ranges if some,
// otherwise using the whole file).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FilteredSearchRange {
    Selection,
    #[default]
    Default,
}

pub trait SearchableItem: Item + EventEmitter<SearchEvent> {
    type Match: Any + Sync + Send + Clone;

    fn supported_options(&self) -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
            replacement: true,
            selection: true,
            find_in_results: false,
        }
    }

    fn search_bar_visibility_changed(
        &mut self,
        _visible: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn has_filtered_search_ranges(&mut self) -> bool {
        self.supported_options().selection
    }

    fn toggle_filtered_search_ranges(
        &mut self,
        _enabled: Option<FilteredSearchRange>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn get_matches(&self, _window: &mut Window, _: &mut App) -> Vec<Self::Match> {
        Vec::new()
    }
    fn clear_matches(&mut self, window: &mut Window, cx: &mut Context<Self>);
    fn update_matches(
        &mut self,
        matches: &[Self::Match],
        active_match_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    );
    fn query_suggestion(&mut self, window: &mut Window, cx: &mut Context<Self>) -> String;
    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    );
    fn select_matches(
        &mut self,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    );
    fn replace(
        &mut self,
        _: &Self::Match,
        _: &SearchQuery,
        _window: &mut Window,
        _: &mut Context<Self>,
    );
    fn replace_all(
        &mut self,
        matches: &mut dyn Iterator<Item = &Self::Match>,
        query: &SearchQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for item in matches {
            self.replace(item, query, window, cx);
        }
    }
    fn match_index_for_direction(
        &mut self,
        matches: &[Self::Match],
        current_index: usize,
        direction: Direction,
        count: usize,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> usize {
        match direction {
            Direction::Prev => {
                let count = count % matches.len();
                if current_index >= count {
                    current_index - count
                } else {
                    matches.len() - (count - current_index)
                }
            }
            Direction::Next => (current_index + count) % matches.len(),
        }
    }
    fn find_matches(
        &mut self,
        query: Arc<SearchQuery>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Self::Match>>;
    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize>;
    fn set_search_is_case_sensitive(&mut self, _: Option<bool>, _: &mut Context<Self>) {}
}

pub trait SearchableItemHandle: ItemHandle {
    fn downgrade(&self) -> Box<dyn WeakSearchableItemHandle>;
    fn boxed_clone(&self) -> Box<dyn SearchableItemHandle>;
    fn supported_options(&self, cx: &App) -> SearchOptions;
    fn subscribe_to_search_events(
        &self,
        window: &mut Window,
        cx: &mut App,
        handler: Box<dyn Fn(&SearchEvent, &mut Window, &mut App) + Send>,
    ) -> Subscription;
    fn clear_matches(&self, window: &mut Window, cx: &mut App);
    fn update_matches(
        &self,
        matches: &AnyVec<dyn Send>,
        active_match_index: Option<usize>,
        window: &mut Window,
        cx: &mut App,
    );
    fn query_suggestion(&self, window: &mut Window, cx: &mut App) -> String;
    fn activate_match(
        &self,
        index: usize,
        matches: &AnyVec<dyn Send>,
        window: &mut Window,
        cx: &mut App,
    );
    fn select_matches(&self, matches: &AnyVec<dyn Send>, window: &mut Window, cx: &mut App);
    fn replace(
        &self,
        _: any_vec::element::ElementRef<'_, dyn Send>,
        _: &SearchQuery,
        _window: &mut Window,
        _: &mut App,
    );
    fn replace_all(
        &self,
        matches: &mut dyn Iterator<Item = any_vec::element::ElementRef<'_, dyn Send>>,
        query: &SearchQuery,
        window: &mut Window,
        cx: &mut App,
    );
    fn match_index_for_direction(
        &self,
        matches: &AnyVec<dyn Send>,
        current_index: usize,
        direction: Direction,
        count: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> usize;
    fn find_matches(
        &self,
        query: Arc<SearchQuery>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<AnyVec<dyn Send>>;
    fn active_match_index(
        &self,
        direction: Direction,
        matches: &AnyVec<dyn Send>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<usize>;
    fn search_bar_visibility_changed(&self, visible: bool, window: &mut Window, cx: &mut App);

    fn toggle_filtered_search_ranges(
        &mut self,
        enabled: Option<FilteredSearchRange>,
        window: &mut Window,
        cx: &mut App,
    );

    fn set_search_is_case_sensitive(&self, is_case_sensitive: Option<bool>, cx: &mut App);
}

impl<T: SearchableItem> SearchableItemHandle for Entity<T> {
    fn downgrade(&self) -> Box<dyn WeakSearchableItemHandle> {
        Box::new(self.downgrade())
    }

    fn boxed_clone(&self) -> Box<dyn SearchableItemHandle> {
        Box::new(self.clone())
    }

    fn supported_options(&self, cx: &App) -> SearchOptions {
        self.read(cx).supported_options()
    }

    fn subscribe_to_search_events(
        &self,
        window: &mut Window,
        cx: &mut App,
        handler: Box<dyn Fn(&SearchEvent, &mut Window, &mut App) + Send>,
    ) -> Subscription {
        window.subscribe(self, cx, move |_, event: &SearchEvent, window, cx| {
            handler(event, window, cx)
        })
    }

    fn clear_matches(&self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.clear_matches(window, cx));
    }
    fn update_matches(
        &self,
        matches: &AnyVec<dyn Send>,
        active_match_index: Option<usize>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| {
            this.update_matches(matches.as_slice(), active_match_index, window, cx)
        });
    }
    fn query_suggestion(&self, window: &mut Window, cx: &mut App) -> String {
        self.update(cx, |this, cx| this.query_suggestion(window, cx))
    }
    fn activate_match(
        &self,
        index: usize,
        matches: &AnyVec<dyn Send>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| {
            this.activate_match(index, matches.as_slice(), window, cx)
        });
    }

    fn select_matches(&self, matches: &AnyVec<dyn Send>, window: &mut Window, cx: &mut App) {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| {
            this.select_matches(matches.as_slice(), window, cx)
        });
    }

    fn match_index_for_direction(
        &self,
        matches: &AnyVec<dyn Send>,
        current_index: usize,
        direction: Direction,
        count: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> usize {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| {
            this.match_index_for_direction(
                matches.as_slice(),
                current_index,
                direction,
                count,
                window,
                cx,
            )
        })
    }
    fn find_matches(
        &self,
        query: Arc<SearchQuery>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<AnyVec<dyn Send>> {
        let matches = self.update(cx, |this, cx| this.find_matches(query, window, cx));
        window.spawn(cx, async |_| {
            let matches = matches.await;
            let mut any_matches = AnyVec::with_capacity::<T::Match>(matches.len());
            {
                let mut any_matches = any_matches.downcast_mut::<T::Match>().unwrap();
                for mat in matches {
                    any_matches.push(mat);
                }
            }
            any_matches
        })
    }
    fn active_match_index(
        &self,
        direction: Direction,
        matches: &AnyVec<dyn Send>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<usize> {
        let matches = matches.downcast_ref()?;
        self.update(cx, |this, cx| {
            this.active_match_index(direction, matches.as_slice(), window, cx)
        })
    }

    fn replace(
        &self,
        mat: any_vec::element::ElementRef<'_, dyn Send>,
        query: &SearchQuery,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mat = mat.downcast_ref().unwrap();
        self.update(cx, |this, cx| this.replace(mat, query, window, cx))
    }

    fn replace_all(
        &self,
        matches: &mut dyn Iterator<Item = any_vec::element::ElementRef<'_, dyn Send>>,
        query: &SearchQuery,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.replace_all(
                &mut matches.map(|m| m.downcast_ref().unwrap()),
                query,
                window,
                cx,
            );
        })
    }

    fn search_bar_visibility_changed(&self, visible: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.search_bar_visibility_changed(visible, window, cx)
        });
    }

    fn toggle_filtered_search_ranges(
        &mut self,
        enabled: Option<FilteredSearchRange>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.toggle_filtered_search_ranges(enabled, window, cx)
        });
    }
    fn set_search_is_case_sensitive(&self, enabled: Option<bool>, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.set_search_is_case_sensitive(enabled, cx)
        });
    }
}

impl From<Box<dyn SearchableItemHandle>> for AnyView {
    fn from(this: Box<dyn SearchableItemHandle>) -> Self {
        this.to_any_view()
    }
}

impl From<&Box<dyn SearchableItemHandle>> for AnyView {
    fn from(this: &Box<dyn SearchableItemHandle>) -> Self {
        this.to_any_view()
    }
}

impl PartialEq for Box<dyn SearchableItemHandle> {
    fn eq(&self, other: &Self) -> bool {
        self.item_id() == other.item_id()
    }
}

impl Eq for Box<dyn SearchableItemHandle> {}

pub trait WeakSearchableItemHandle: WeakItemHandle {
    fn upgrade(&self, cx: &App) -> Option<Box<dyn SearchableItemHandle>>;

    fn into_any(self) -> AnyWeakEntity;
}

impl<T: SearchableItem> WeakSearchableItemHandle for WeakEntity<T> {
    fn upgrade(&self, _cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.upgrade()?))
    }

    fn into_any(self) -> AnyWeakEntity {
        self.into()
    }
}

impl PartialEq for Box<dyn WeakSearchableItemHandle> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Box<dyn WeakSearchableItemHandle> {}

impl std::hash::Hash for Box<dyn WeakSearchableItemHandle> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}
