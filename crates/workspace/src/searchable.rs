use std::any::Any;

use gpui::{
    AnyViewHandle, AnyWeakViewHandle, AppContext, Subscription, Task, ViewContext, ViewHandle,
    WeakViewHandle, WindowContext,
};
use project::search::SearchQuery;

use crate::{item::WeakItemHandle, Item, ItemHandle};

#[derive(Debug)]
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
}

pub trait SearchableItem: Item {
    type Match: Any + Sync + Send + Clone;

    fn supported_options() -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
        }
    }
    fn to_search_event(event: &Self::Event) -> Option<SearchEvent>;
    fn clear_matches(&mut self, cx: &mut ViewContext<Self>);
    fn update_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>);
    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String;
    fn activate_match(
        &mut self,
        index: usize,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    );
    fn match_index_for_direction(
        &mut self,
        matches: &Vec<Self::Match>,
        current_index: usize,
        direction: Direction,
        count: Option<usize>,
        _: &mut ViewContext<Self>,
    ) -> usize {
        match direction {
            Direction::Prev => {
                let count = count.unwrap_or(1) % matches.len();
                if current_index >= count {
                    current_index - count
                } else {
                    matches.len() - (count - current_index)
                }
            }
            Direction::Next => (current_index + count.unwrap_or(1)) % matches.len(),
        }
    }
    fn find_matches(
        &mut self,
        query: SearchQuery,
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
        handler: Box<dyn Fn(SearchEvent, &mut WindowContext)>,
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
    fn match_index_for_direction(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        current_index: usize,
        direction: Direction,
        count: Option<usize>,
        cx: &mut WindowContext,
    ) -> usize;
    fn find_matches(
        &self,
        query: SearchQuery,
        cx: &mut WindowContext,
    ) -> Task<Vec<Box<dyn Any + Send>>>;
    fn active_match_index(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut WindowContext,
    ) -> Option<usize>;
}

impl<T: SearchableItem> SearchableItemHandle for ViewHandle<T> {
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
        handler: Box<dyn Fn(SearchEvent, &mut WindowContext)>,
    ) -> Subscription {
        cx.subscribe(self, move |_, event, cx| {
            if let Some(search_event) = T::to_search_event(event) {
                handler(search_event, cx)
            }
        })
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
    fn match_index_for_direction(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        current_index: usize,
        direction: Direction,
        count: Option<usize>,
        cx: &mut WindowContext,
    ) -> usize {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| {
            this.match_index_for_direction(&matches, current_index, direction, count, cx)
        })
    }
    fn find_matches(
        &self,
        query: SearchQuery,
        cx: &mut WindowContext,
    ) -> Task<Vec<Box<dyn Any + Send>>> {
        let matches = self.update(cx, |this, cx| this.find_matches(query, cx));
        cx.foreground().spawn(async {
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

impl From<Box<dyn SearchableItemHandle>> for AnyViewHandle {
    fn from(this: Box<dyn SearchableItemHandle>) -> Self {
        this.as_any().clone()
    }
}

impl From<&Box<dyn SearchableItemHandle>> for AnyViewHandle {
    fn from(this: &Box<dyn SearchableItemHandle>) -> Self {
        this.as_any().clone()
    }
}

impl PartialEq for Box<dyn SearchableItemHandle> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.window_id() == other.window_id()
    }
}

impl Eq for Box<dyn SearchableItemHandle> {}

pub trait WeakSearchableItemHandle: WeakItemHandle {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>>;

    fn into_any(self) -> AnyWeakViewHandle;
}

impl<T: SearchableItem> WeakSearchableItemHandle for WeakViewHandle<T> {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.upgrade(cx)?))
    }

    fn into_any(self) -> AnyWeakViewHandle {
        self.into_any()
    }
}

impl PartialEq for Box<dyn WeakSearchableItemHandle> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.window_id() == other.window_id()
    }
}

impl Eq for Box<dyn WeakSearchableItemHandle> {}

impl std::hash::Hash for Box<dyn WeakSearchableItemHandle> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.id(), self.window_id()).hash(state)
    }
}
