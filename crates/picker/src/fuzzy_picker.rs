use crate::{Picker, PickerDelegate};
use fuzzy::StringMatchCandidate;
use gpui::{IntoElement, SharedString, ViewContext, WindowContext};
use std::marker::PhantomData;

impl<I, E> Picker<FuzzyPickerDelegate<I, E>>
where
    I: 'static + Clone,
    E: 'static + IntoElement,
{
    pub fn fuzzy(
        items: Vec<FuzzyPickerItem<I>>,
        cx: &mut ViewContext<Picker<FuzzyPickerDelegate<I, E>>>,
        render_match: impl 'static + Fn(usize, bool, &mut ViewContext<Self>) -> E,
    ) -> Self {
        Self::new(
            FuzzyPickerDelegate {
                items: items.clone(),
                matches: items,
                render_match: Box::new(render_match),
                item_element_type: PhantomData,
                selected_index: 0,
                placeholder_text: None,
                confirm: None,
                dismiss: None,
            },
            cx,
        )
    }

    pub fn placeholder_text(mut self, text: impl Into<SharedString>) -> Self {
        self.delegate.placeholder_text = Some(text.into());
        self
    }

    pub fn on_confirm(
        mut self,
        confirm: impl 'static + FnOnce(I, bool, &mut WindowContext),
    ) -> Self {
        self.delegate.confirm = Some(Box::new(confirm));
        self
    }

    pub fn on_dismiss(mut self, dismiss: impl 'static + FnOnce(&mut WindowContext)) -> Self {
        self.delegate.dismiss = Some(Box::new(dismiss));
        self
    }
}

pub struct FuzzyPickerDelegate<I, E>
where
    I: 'static + Clone,
    E: 'static + IntoElement,
{
    items: Vec<FuzzyPickerItem<I>>,
    matches: Vec<FuzzyPickerItem<I>>,
    render_match:
        Box<dyn Fn(usize, bool, &mut ViewContext<'_, Picker<FuzzyPickerDelegate<I, E>>>) -> E>,
    item_element_type: PhantomData<E>,
    selected_index: usize,
    placeholder_text: Option<SharedString>,
    confirm: Option<Box<dyn FnOnce(I, bool, &mut WindowContext)>>,
    dismiss: Option<Box<dyn FnOnce(&mut WindowContext)>>,
}

#[derive(Clone)]
pub struct FuzzyPickerItem<I: Clone> {
    pub name: SharedString,
    pub id: I,
}

impl<I, E> PickerDelegate for FuzzyPickerDelegate<I, E>
where
    I: 'static + Clone,
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

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(confirm) = self.confirm.take() {
            let confirmed = self.items[self.selected_index].id.clone();
            confirm(confirmed, secondary, cx)
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(dismiss) = self.dismiss.take() {
            dismiss(cx);
        }
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some((self.render_match)(ix, selected, cx))
    }
}
