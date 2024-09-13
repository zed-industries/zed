use std::{any::Any, sync::Arc};

use any_vec::AnyVec;
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
}

pub trait SearchableItem: Item + EventEmitter<SearchEvent> {
    type Match: Any + Sync + Send + Clone;

    fn supported_options() -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
            replacement: true,
            selection: true,
        }
    }

    fn search_bar_visibility_changed(&mut self, _visible: bool, _cx: &mut ViewContext<Self>) {}

    fn has_filtered_search_ranges(&mut self) -> bool {
        Self::supported_options().selection
    }

    fn toggle_filtered_search_ranges(&mut self, _enabled: bool, _cx: &mut ViewContext<Self>) {}

    fn get_matches(&self, _: &mut WindowContext) -> Vec<Self::Match> {
        Vec::new()
    }
    fn clear_matches(&mut self, cx: &mut ViewContext<Self>);
    fn update_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>);
    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String;
    fn activate_match(&mut self, index: usize, matches: &[Self::Match], cx: &mut ViewContext<Self>);
    fn select_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>);
    fn replace(&mut self, _: &Self::Match, _: &SearchQuery, _: &mut ViewContext<Self>);
    fn replace_all(
        &mut self,
        matches: &mut dyn Iterator<Item = &Self::Match>,
        query: &SearchQuery,
        cx: &mut ViewContext<Self>,
    ) {
        for item in matches {
            self.replace(item, query, cx);
        }
    }
    fn match_index_for_direction(
        &mut self,
        matches: &[Self::Match],
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
        matches: &[Self::Match],
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
    fn update_matches(&self, matches: &AnyVec<dyn Send>, cx: &mut WindowContext);
    fn query_suggestion(&self, cx: &mut WindowContext) -> String;
    fn activate_match(&self, index: usize, matches: &AnyVec<dyn Send>, cx: &mut WindowContext);
    fn select_matches(&self, matches: &AnyVec<dyn Send>, cx: &mut WindowContext);
    fn replace(
        &self,
        _: any_vec::element::ElementRef<'_, dyn Send>,
        _: &SearchQuery,
        _: &mut WindowContext,
    );
    fn replace_all(
        &self,
        matches: &mut dyn Iterator<Item = any_vec::element::ElementRef<'_, dyn Send>>,
        query: &SearchQuery,
        cx: &mut WindowContext,
    );
    fn match_index_for_direction(
        &self,
        matches: &AnyVec<dyn Send>,
        current_index: usize,
        direction: Direction,
        count: usize,
        cx: &mut WindowContext,
    ) -> usize;
    fn find_matches(
        &self,
        query: Arc<SearchQuery>,
        cx: &mut WindowContext,
    ) -> Task<AnyVec<dyn Send>>;
    fn active_match_index(
        &self,
        matches: &AnyVec<dyn Send>,
        cx: &mut WindowContext,
    ) -> Option<usize>;
    fn search_bar_visibility_changed(&self, visible: bool, cx: &mut WindowContext);

    fn toggle_filtered_search_ranges(&mut self, enabled: bool, cx: &mut WindowContext);
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
    fn update_matches(&self, matches: &AnyVec<dyn Send>, cx: &mut WindowContext) {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| this.update_matches(matches.as_slice(), cx));
    }
    fn query_suggestion(&self, cx: &mut WindowContext) -> String {
        self.update(cx, |this, cx| this.query_suggestion(cx))
    }
    fn activate_match(&self, index: usize, matches: &AnyVec<dyn Send>, cx: &mut WindowContext) {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| {
            this.activate_match(index, matches.as_slice(), cx)
        });
    }

    fn select_matches(&self, matches: &AnyVec<dyn Send>, cx: &mut WindowContext) {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| this.select_matches(matches.as_slice(), cx));
    }

    fn match_index_for_direction(
        &self,
        matches: &AnyVec<dyn Send>,
        current_index: usize,
        direction: Direction,
        count: usize,
        cx: &mut WindowContext,
    ) -> usize {
        let matches = matches.downcast_ref().unwrap();
        self.update(cx, |this, cx| {
            this.match_index_for_direction(matches.as_slice(), current_index, direction, count, cx)
        })
    }
    fn find_matches(
        &self,
        query: Arc<SearchQuery>,
        cx: &mut WindowContext,
    ) -> Task<AnyVec<dyn Send>> {
        let matches = self.update(cx, |this, cx| this.find_matches(query, cx));
        cx.spawn(|_| async {
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
        matches: &AnyVec<dyn Send>,
        cx: &mut WindowContext,
    ) -> Option<usize> {
        let matches = matches.downcast_ref()?;
        self.update(cx, |this, cx| {
            this.active_match_index(matches.as_slice(), cx)
        })
    }

    fn replace(
        &self,
        mat: any_vec::element::ElementRef<'_, dyn Send>,
        query: &SearchQuery,
        cx: &mut WindowContext,
    ) {
        let mat = mat.downcast_ref().unwrap();
        self.update(cx, |this, cx| this.replace(mat, query, cx))
    }

    fn replace_all(
        &self,
        matches: &mut dyn Iterator<Item = any_vec::element::ElementRef<'_, dyn Send>>,
        query: &SearchQuery,
        cx: &mut WindowContext,
    ) {
        self.update(cx, |this, cx| {
            this.replace_all(&mut matches.map(|m| m.downcast_ref().unwrap()), query, cx);
        })
    }

    fn search_bar_visibility_changed(&self, visible: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| {
            this.search_bar_visibility_changed(visible, cx)
        });
    }

    fn toggle_filtered_search_ranges(&mut self, enabled: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| {
            this.toggle_filtered_search_ranges(enabled, cx)
        });
    }
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
