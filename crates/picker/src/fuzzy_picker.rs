use crate::{Picker, PickerDelegate};
use fuzzy::StringMatchCandidate;
use gpui::{IntoElement, SharedString, ViewContext};
use std::marker::PhantomData;

impl<I, R, E> Picker<FuzzyPickerDelegate<I, R, E>>
where
    I: 'static + Clone,
    R: 'static
        + Fn(usize, bool, &mut ViewContext<'_, Picker<FuzzyPickerDelegate<I, R, E>>>) -> Option<E>,
    E: 'static + IntoElement,
{
    pub fn fuzzy(
        items: Vec<FuzzyPickerItem<I>>,
        cx: &mut ViewContext<Picker<FuzzyPickerDelegate<I, R, E>>>,
        render_match: R,
    ) -> Self {
        Self::new(
            FuzzyPickerDelegate {
                items: items.clone(),
                matches: items,
                render_item: render_match,
                item_element_type: PhantomData,
                selected_index: 0,
                placeholder_text: None,
            },
            cx,
        )
    }

    pub fn placeholder_text(mut self, text: impl Into<SharedString>) -> Self {
        self.delegate.placeholder_text = Some(text.into());
        self
    }
}

pub struct FuzzyPickerDelegate<I: Clone, R, E> {
    items: Vec<FuzzyPickerItem<I>>,
    matches: Vec<FuzzyPickerItem<I>>,
    render_item: R,
    item_element_type: PhantomData<E>,
    selected_index: usize,
    placeholder_text: Option<SharedString>,
}

#[derive(Clone)]
pub struct FuzzyPickerItem<I: Clone> {
    name: SharedString,
    id: I,
}

impl<I, R, E> PickerDelegate for FuzzyPickerDelegate<I, R, E>
where
    I: 'static + Clone,
    R: 'static + Fn(usize, bool, &mut ViewContext<Picker<Self>>) -> Option<E>,
    E: 'static + IntoElement,
{
    type ListItem = E;

    fn match_count(&self) -> usize {
        self.items.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self) -> SharedString {
        self.placeholder_text.clone().unwrap_or_default()
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let items = self.items.clone();
        cx.spawn(move |this, mut cx| async move {
            let query = query.trim_start();
            let smart_case = query.chars().any(|c| c.is_uppercase());
            let candidates = items
                .iter()
                .enumerate()
                .map(|(id, item)| StringMatchCandidate::new(id, item.name.to_string()))
                .collect::<Vec<_>>();

            let mut fuzzy_matches = fuzzy::match_strings(
                candidates.as_slice(),
                query,
                smart_case,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            this.update(&mut cx, |this, _cx| {
                fuzzy_matches.sort_unstable_by_key(|m| m.candidate_id);
                let max_score = 0.;
                this.delegate.matches = fuzzy_matches
                    .into_iter()
                    .enumerate()
                    .map(|(ix, m)| {
                        if m.score > max_score {
                            this.delegate.selected_index = ix;
                        }
                        items[m.candidate_id].clone()
                    })
                    .collect();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        todo!()
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        todo!()
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        (self.render_item)(ix, selected, cx)
    }
}
