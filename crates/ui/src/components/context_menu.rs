use crate::theme::theme;
use crate::{
    prelude::*, v_stack, Label, List, ListEntry, ListItem, ListItemVariant, ListSeparator,
    ListSubHeader,
};

#[derive(Clone)]
pub enum ContextMenuItem {
    Header(&'static str),
    Entry(Label),
    Separator,
}

impl ContextMenuItem {
    fn to_list_item(self) -> ListItem {
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
    pub fn entry(label: Label) -> Self {
        Self::Entry(label)
    }
}

#[derive(Element)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
}

impl ContextMenu {
    pub fn new(items: impl IntoIterator<Item = ContextMenuItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
    fn render<V: 'static>(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
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
        //div().p_1().children(self.items.clone())
    }
}
