use crate::prelude::*;
use crate::{
    theme, v_stack, Label, List, ListEntry, ListItem, ListItemVariant, ListSeparator, ListSubHeader,
};

#[derive(Clone)]
pub enum ContextMenuItem<S: 'static + Send + Sync + Clone> {
    Header(&'static str),
    Entry(Label<S>),
    Separator,
}

impl<S: 'static + Send + Sync + Clone> ContextMenuItem<S> {
    fn to_list_item(self) -> ListItem<S> {
        match self {
            ContextMenuItem::Header(label) => ListSubHeader::new(label).into(),
            ContextMenuItem::Entry(label) => {
                ListEntry::new(label).variant(ListItemVariant::Inset).into()
            }
            ContextMenuItem::Separator => ListSeparator::new().into(),
        }
    }

    pub fn header(label: &'static str) -> Self {
        Self::Header(label)
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn entry(label: Label<S>) -> Self {
        Self::Entry(label)
    }
}

#[derive(Element)]
pub struct ContextMenu<S: 'static + Send + Sync + Clone> {
    items: Vec<ContextMenuItem<S>>,
}

impl<S: 'static + Send + Sync + Clone> ContextMenu<S> {
    pub fn new(items: impl IntoIterator<Item = ContextMenuItem<S>>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        v_stack()
            .flex()
            .fill(theme.lowest.base.default.background)
            .border()
            .border_color(theme.lowest.base.default.border)
            .child(
                List::new(
                    self.items
                        .clone()
                        .into_iter()
                        .map(ContextMenuItem::to_list_item)
                        .collect(),
                )
                .set_toggle(ToggleState::Toggled),
            )
    }
}
