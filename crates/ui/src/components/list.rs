use crate::theme::theme;
use crate::tokens::token;
use crate::{icon, label, prelude::*, v_stack, IconAsset, LabelColor, ListItem, ListSectionHeader};
use gpui2::style::StyleHelpers;
use gpui2::IntoElement;
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct List {
    header: Option<ListSectionHeader>,
    items: Vec<ListItem>,
    empty_message: &'static str,
    toggle: Option<ToggleState>,
    // footer: Option<ListSectionFooter>,
}

pub fn list(items: Vec<ListItem>) -> List {
    List {
        header: None,
        items,
        empty_message: "No items",
        toggle: None,
    }
}

impl List {
    pub fn header(mut self, header: ListSectionHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn empty_message(mut self, empty_message: &'static str) -> Self {
        self.empty_message = empty_message;
        self
    }

    pub fn set_toggle(mut self, toggle: ToggleState) -> Self {
        self.toggle = Some(toggle);
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();

        let disclosure_control = match self.toggle {
            Some(ToggleState::NotToggled) => Some(icon(IconAsset::ChevronRight)),
            Some(ToggleState::Toggled) => Some(icon(IconAsset::ChevronDown)),
            None => None,
        };

        v_stack()
            .py_1()
            .children(self.header.map(|h| h))
            .children(
                self.items
                    .is_empty()
                    .then(|| label(self.empty_message).color(LabelColor::Muted)),
            )
            .children(self.items.iter().cloned())
    }
}
