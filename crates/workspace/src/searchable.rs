use std::any::Any;

use gpui::{
    AnyViewHandle, AnyWeakViewHandle, AppContext, MutableAppContext, Subscription, Task,
    ViewContext, ViewHandle, WeakViewHandle,
};
use project::search::SearchQuery;

use crate::{Item, ItemHandle, WeakItemHandle};

#[derive(Debug)]
pub enum SearchEvent {
    MatchesInvalidated,
    ActiveMatchChanged,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

pub trait SearchableItem: Item {
    type Match: Any + Sync + Send + Clone;

    fn to_search_event(event: &Self::Event) -> Option<SearchEvent>;
    fn clear_matches(&mut self, cx: &mut ViewContext<Self>);
    fn update_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>);
    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String;
    fn activate_next_match(
        &mut self,
        index: usize,
        direction: Direction,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    );
    fn activate_match_at_index(
        &mut self,
        index: usize,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    );
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
    fn subscribe(
        &self,
        cx: &mut MutableAppContext,
        handler: Box<dyn Fn(SearchEvent, &mut MutableAppContext)>,
    ) -> Subscription;
    fn clear_highlights(&self, cx: &mut MutableAppContext);
    fn highlight_matches(&self, matches: &Vec<Box<dyn Any + Send>>, cx: &mut MutableAppContext);
    fn query_suggestion(&self, cx: &mut MutableAppContext) -> String;
    fn select_next_match_in_direction(
        &self,
        index: usize,
        direction: Direction,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut MutableAppContext,
    );
    fn select_match_by_index(
        &self,
        index: usize,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut MutableAppContext,
    );
    fn matches(
        &self,
        query: SearchQuery,
        cx: &mut MutableAppContext,
    ) -> Task<Vec<Box<dyn Any + Send>>>;
    fn active_match_index(
        &self,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut MutableAppContext,
    ) -> Option<usize>;
}

impl<T: SearchableItem> SearchableItemHandle for ViewHandle<T> {
    fn downgrade(&self) -> Box<dyn WeakSearchableItemHandle> {
        Box::new(self.downgrade())
    }

    fn boxed_clone(&self) -> Box<dyn SearchableItemHandle> {
        Box::new(self.clone())
    }

    fn subscribe(
        &self,
        cx: &mut MutableAppContext,
        handler: Box<dyn Fn(SearchEvent, &mut MutableAppContext)>,
    ) -> Subscription {
        cx.subscribe(self, move |_, event, cx| {
            if let Some(search_event) = T::to_search_event(event) {
                handler(search_event, cx)
            }
        })
    }

    fn clear_highlights(&self, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| this.clear_matches(cx));
    }
    fn highlight_matches(&self, matches: &Vec<Box<dyn Any + Send>>, cx: &mut MutableAppContext) {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| this.update_matches(matches, cx));
    }
    fn query_suggestion(&self, cx: &mut MutableAppContext) -> String {
        self.update(cx, |this, cx| this.query_suggestion(cx))
    }
    fn select_next_match_in_direction(
        &self,
        index: usize,
        direction: Direction,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut MutableAppContext,
    ) {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| {
            this.activate_next_match(index, direction, matches, cx)
        });
    }
    fn select_match_by_index(
        &self,
        index: usize,
        matches: &Vec<Box<dyn Any + Send>>,
        cx: &mut MutableAppContext,
    ) {
        let matches = downcast_matches(matches);
        self.update(cx, |this, cx| {
            this.activate_match_at_index(index, matches, cx)
        });
    }
    fn matches(
        &self,
        query: SearchQuery,
        cx: &mut MutableAppContext,
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
        cx: &mut MutableAppContext,
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
        this.to_any()
    }
}

impl From<&Box<dyn SearchableItemHandle>> for AnyViewHandle {
    fn from(this: &Box<dyn SearchableItemHandle>) -> Self {
        this.to_any()
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

    fn to_any(self) -> AnyWeakViewHandle;
}

impl<T: SearchableItem> WeakSearchableItemHandle for WeakViewHandle<T> {
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.upgrade(cx)?))
    }

    fn to_any(self) -> AnyWeakViewHandle {
        self.into()
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
