use std::marker::PhantomData;

use gpui::{IntoElement, SharedString, ViewContext};

use crate::{Picker, PickerDelegate};

impl<I, R, E> Picker<FuzzyPickerDelegate<I, R, E>>
where
    I: 'static,
    R: 'static + Fn(usize, bool, &mut ViewContext<Picker<Self>>) -> E,
    E: 'static + IntoElement,
{
    fn fuzzy(
        items: Vec<FuzzyPickerItem<I>>,
        cx: &mut ViewContext<Picker<FuzzyPickerDelegate<I, R, E>>>,
        render_match: R,
    ) -> Self {
        Self::new(
            FuzzyPickerDelegate {
                items,
                render_match,
                item_element_type: PhantomData,
            },
            cx,
        )
    }
}

pub struct FuzzyPickerDelegate<I, R, E> {
    items: Vec<FuzzyPickerItem<I>>,
    render_match: R,
    item_element_type: PhantomData<E>,
}

pub struct FuzzyPickerItem<I> {
    name: SharedString,
    id: I,
}

impl<I: 'static, R: 'static + Fn(), E: 'static + IntoElement> PickerDelegate
    for FuzzyPickerDelegate<I, R, E>
{
    type ListItem = E;

    fn match_count(&self) -> usize {
        todo!()
    }

    fn selected_index(&self) -> usize {
        todo!()
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        todo!()
    }

    fn placeholder_text(&self) -> std::sync::Arc<str> {
        todo!()
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        todo!()
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
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
        todo!()
    }
}
