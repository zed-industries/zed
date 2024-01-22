use std::{any::Any, sync::Arc};

use gpui::{
    AnyView, AnyWeakView, AppContext, EventEmitter, Subscription, Task, View, ViewContext,
    WeakView, WindowContext,
};
use project::search::SearchQuery;

use crate::{
    item::{Item, WeakItemHandle},
    ItemHandle,
};

#[derive(Clone, Debug)]
pub enum SearchEvent {
    MatchesInvalidated,
    ActiveMatchChanged,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Prev,
    Next,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SearchOptions {
    pub case: bool,
    pub word: bool,
    pub regex: bool,
    /// Specifies whether the item supports search & replace.
    pub replacement: bool,
}

pub trait SearchableItem: Item + EventEmitter<SearchEvent> {
    type Match: Any + Sync + Send + Clone;

    fn supported_options() -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
            replacement: true,
        }
    }

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>);
    fn update_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>);
    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String;
    fn activate_match(
        &mut self,
        index: usize,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    );
    fn select_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>);
    fn replace(&mut self, _: &Self::Match, _: &SearchQuery, _: &mut ViewContext<Self>);
    fn match_index_for_direction(
        &mut self,
        matches: &Vec<Self::Match>,
        current_index: usize,
        direction: Direction,
        count: usize,
        _: &mut ViewContext<Self>,
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
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>>;
    fn active_match_index(
        &mut self,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    ) -> Option<usize>;
}

pub trait SearchableItemHandle: ItemHandle {
    fn downgrade(&self) -> Box<dyn WeakSearchableItemHandle>;
    fn boxed_clone(&self) -> Box<dyn SearchableItemHandle>;
    fn supported_options(&self) -> SearchOptions;
    fn subscribe_to_search_events(
        &self,
        cx: &mut WindowContext,
        handler: Box<dyn Fn(&SearchEvent, &mut WindowContext) + Send>,
    ) -> Subscription;
    fn clear_matches(&self, cx: &mut WindowContext);
    fn update_matches(&self, matches: &Vec<Box<dyn Any + Send>>, cx: &mut WindowContext);
    fn query_suggestion(&self, cx: &mut WindowContext) -> String;
    fn activate_match(
        &self,
        index: usize,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut WindowContext,
    );
    fn select_matches(&self, matches: &Vec<Box<dyn Any + Send>>, cx: &mut WindowContext);
    fn replace(&self, _: &Box<dyn Any + Send>, _: &SearchQuery, _: &mut WindowContext);
    fn match_index_for_direction(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        current_index: usize,
        direction: Direction,
        count: usize,
        cx: &mut WindowContext,
    ) -> usize;
    fn find_matches(
        &self,
        query: Arc<SearchQuery>,
        cx: &mut WindowContext,
    ) -> Task<Vec<Box<dyn Any + Send>>>;
    fn active_match_index(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut WindowContext,
    ) -> Option<usize>;
}

impl<T: SearchableItem> SearchableItemHandle for View<T> {
    fn downgrade(&self) -> Box<dyn WeakSearchableItemHandle> {
        Box::new(self.downgrade())
    }

    fn boxed_clone(&self) -> Box<dyn SearchableItemHandle> {
        Box::new(self.clone())
    }

    fn supported_options(&self) -> SearchOptions {
        T::supported_options()
    }

    fn subscribe_to_search_events(
        &self,
        cx: &mut WindowContext,
        handler: Box<dyn Fn(&SearchEvent, &mut WindowContext) + Send>,
    ) -> Subscription {
        cx.subscribe(self, move |_, event: &SearchEvent, cx| handler(event, cx))
    }

    fn clear_matches(&self, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.clear_matches(cx));
    }
    fn update_matches(&self, matches: &Vec<Box<dyn Any + Send>>, cx: &mut WindowContext) {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| this.update_matches(matches, cx));
    }
    fn query_suggestion(&self, cx: &mut WindowContext) -> String {
        self.update(cx, |this, cx| this.query_suggestion(cx))
    }
    fn activate_match(
        &self,
        index: usize,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut WindowContext,
    ) {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| this.activate_match(index, matches, cx));
    }

    fn select_matches(&self, matches: &Vec<Box<dyn Any + Send>>, cx: &mut WindowContext) {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| this.select_matches(matches, cx));
    }

    fn match_index_for_direction(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        current_index: usize,
        direction: Direction,
        count: usize,
        cx: &mut WindowContext,
    ) -> usize {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| {
            this.match_index_for_direction(&matches, current_index, direction, count, cx)
        })
    }
    fn find_matches(
        &self,
        query: Arc<SearchQuery>,
        cx: &mut WindowContext,
    ) -> Task<Vec<Box<dyn Any + Send>>> {
        let matches = self.update(cx, |this, cx| this.find_matches(query, cx));
        cx.spawn(|_| async {
            let matches = matches.await;
            matches
                .into_iter()
                .map::<Box<dyn Any + Send>, _>(|range| Box::new(range))
                .collect()
        })
    }
    fn active_match_index(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut WindowContext,
    ) -> Option<usize> {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| this.active_match_index(matches, cx))
    }

    fn replace(&self, matches: &Box<dyn Any + Send>, query: &SearchQuery, cx: &mut WindowContext) {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| this.replace(matches, query, cx))
    }
}

fn downcast_matches<T: Any + Clone>(matches: &Vec<Box<dyn Any + Send>>) -> Vec<T> {
    matches
        .iter()
        .map(|range| range.downcast_ref::<T>().cloned())
        .collect::<Option<Vec<_>>>()
        .expect(
            "SearchableItemHandle function called with vec of matches of a different type than expected",
        )
}

impl From<Box<dyn SearchableItemHandle>> for AnyView {
    fn from(this: Box<dyn SearchableItemHandle>) -> Self {
        this.to_any().clone()
    }
}

impl From<&Box<dyn SearchableItemHandle>> for AnyView {
    fn from(this: &Box<dyn SearchableItemHandle>) -> Self {
        this.to_any().clone()
    }
}

impl PartialEq for Box<dyn SearchableItemHandle> {
    fn eq(&self, other: &Self) -> bool {
        self.item_id() == other.item_id()
    }
}

impl Eq for Box<dyn SearchableItemHandle> {}

pub trait WeakSearchableItemHandle: WeakItemHandle {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>>;

    fn into_any(self) -> AnyWeakView;
}

impl<T: SearchableItem> WeakSearchableItemHandle for WeakView<T> {
    fn upgrade(&self, _cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.upgrade()?))
    }

    fn into_any(self) -> AnyWeakView {
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
