#![allow(missing_docs)]

use gpui::AnyElement;
use smallvec::SmallVec;

use crate::{prelude::*, v_flex, Label, ListHeader};

#[derive(IntoElement)]
pub struct List {
    /// Message to display when the list is empty
    /// Defaults to "No items"
    empty_message: SharedString,
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
            empty_message: "No items".into(),
            header: None,
            toggle: None,
            children: SmallVec::new(),
        }
    }

    pub fn empty_message(mut self, empty_message: impl Into<SharedString>) -> Self {
        self.empty_message = empty_message.into();
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

impl RenderOnce for List {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .w_full()
            .py(Spacing::Small.rems(cx))
            .children(self.header)
            .map(|this| match (self.children.is_empty(), self.toggle) {
                (false, _) => this.children(self.children),
                (true, Some(false)) => this,
                (true, _) => this.child(Label::new(self.empty_message.clone()).color(Color::Muted)),
            })
    }
}
