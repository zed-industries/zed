use crate::{Picker, PickerDelegate};
use fuzzy::StringMatchCandidate;
use gpui::{IntoElement, SharedString, ViewContext, WindowContext};

pub trait FuzzyPickerItem: 'static + Clone {
    fn match_text(&self) -> SharedString;
}

impl<I, E> Picker<FuzzyPickerDelegate<I, E>>
where
    I: FuzzyPickerItem,
    E: 'static + IntoElement,
{
    pub fn fuzzy(
        items: Vec<I>,
        cx: &mut ViewContext<Picker<FuzzyPickerDelegate<I, E>>>,
        render_match: impl 'static + Fn(&I, bool, &mut WindowContext) -> E,
    ) -> Self {
        Self::new(
            FuzzyPickerDelegate {
                items: items.clone(),
                matches: items,
                selected_index: 0,
                placeholder_text: None,
                render_match: Box::new(render_match),
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

pub struct FuzzyPickerDelegate<I: FuzzyPickerItem, E> {
    items: Vec<I>,
    matches: Vec<I>,
    selected_index: usize,
    placeholder_text: Option<SharedString>,
    render_match: Box<dyn Fn(&I, bool, &mut WindowContext) -> E>,
    confirm: Option<Box<dyn FnOnce(I, bool, &mut WindowContext)>>,
    dismiss: Option<Box<dyn FnOnce(&mut WindowContext)>>,
}

impl<I, E> PickerDelegate for FuzzyPickerDelegate<I, E>
where
    I: FuzzyPickerItem,
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
                .map(|(id, item)| StringMatchCandidate::new(id, item.match_text().into()))
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
            let confirmed = self.matches[self.selected_index].clone();
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
        Some((self.render_match)(&self.matches[ix], selected, cx))
    }
}
