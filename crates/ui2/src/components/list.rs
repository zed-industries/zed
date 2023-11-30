mod list_header;
mod list_item;
mod list_separator;
mod list_sub_header;

use gpui::{AnyElement, Div};
use smallvec::SmallVec;

use crate::prelude::*;
use crate::{v_stack, Label};

pub use list_header::*;
pub use list_item::*;
pub use list_separator::*;
pub use list_sub_header::*;

#[derive(IntoElement)]
pub struct List {
    /// Message to display when the list is empty
    /// Defaults to "No items"
    empty_message: SharedString,
    header: Option<ListHeader>,
    toggle: Option<bool>,
    children: SmallVec<[AnyElement; 2]>,
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

    pub fn header(mut self, header: ListHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }
}

impl ParentElement for List {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

impl RenderOnce for List {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        v_stack()
            .w_full()
            .py_1()
            .children(self.header.map(|header| header))
            .map(|this| match (self.children.is_empty(), self.toggle) {
                (false, _) => this.children(self.children),
                (true, Some(false)) => this,
                (true, _) => this.child(Label::new(self.empty_message.clone()).color(Color::Muted)),
            })
    }
}
