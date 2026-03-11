use component::{Component, ComponentScope, example_group_with_title, single_example};
use gpui::{AnyElement, ElementId, Role};
use smallvec::SmallVec;

use crate::{Label, ListHeader, ListItem, prelude::*};

pub enum EmptyMessage {
    Text(SharedString),
    Element(AnyElement),
}

#[derive(IntoElement, RegisterComponent)]
pub struct List {
    id: Option<ElementId>,
    /// Message to display when the list is empty
    /// Defaults to "No items"
    empty_message: EmptyMessage,
    header: Option<ListHeader>,
    toggle: Option<bool>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            id: None,
            empty_message: EmptyMessage::Text("No items".into()),
            header: None,
            toggle: None,
            children: SmallVec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn empty_message(mut self, message: impl Into<EmptyMessage>) -> Self {
        self.empty_message = message.into();
        self
    }

    pub fn header(mut self, header: impl Into<Option<ListHeader>>) -> Self {
        self.header = header.into();
        self
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }
}

impl ParentElement for List {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl From<String> for EmptyMessage {
    fn from(s: String) -> Self {
        EmptyMessage::Text(SharedString::from(s))
    }
}

impl From<&str> for EmptyMessage {
    fn from(s: &str) -> Self {
        EmptyMessage::Text(SharedString::from(s.to_owned()))
    }
}

impl From<AnyElement> for EmptyMessage {
    fn from(e: AnyElement) -> Self {
        EmptyMessage::Element(e)
    }
}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let styled = v_flex()
            .w_full()
            .py(DynamicSpacing::Base04.rems(cx))
            .children(self.header)
            .map(|this| match (self.children.is_empty(), self.toggle) {
                (false, _) => this.children(self.children),
                (true, Some(false)) => this,
                (true, _) => match self.empty_message {
                    EmptyMessage::Text(text) => {
                        this.px_2().child(Label::new(text).color(Color::Muted))
                    }
                    EmptyMessage::Element(element) => this.child(element),
                },
            });

        if let Some(id) = self.id {
            styled.id(id).role(Role::List).into_any_element()
        } else {
            styled.into_any_element()
        }
    }
}

impl Component for List {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn description() -> Option<&'static str> {
        Some(
            "A container component for displaying a collection of list items with optional header and empty state.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group_with_title(
                    "Basic Lists",
                    vec![
                        single_example(
                            "Simple List",
                            List::new().id("simple-list")
                                .child(ListItem::new("item1").child(Label::new("Item 1")))
                                .child(ListItem::new("item2").child(Label::new("Item 2")))
                                .child(ListItem::new("item3").child(Label::new("Item 3")))
                                .into_any_element(),
                        ),
                        single_example(
                            "With Header",
                            List::new().id("header-list")
                                .header(ListHeader::new("Section Header"))
                                .child(ListItem::new("item1").child(Label::new("Item 1")))
                                .child(ListItem::new("item2").child(Label::new("Item 2")))
                                .into_any_element(),
                        ),
                        single_example(
                            "Empty List",
                            List::new().id("empty-list")
                                .empty_message("No items to display")
                                .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
