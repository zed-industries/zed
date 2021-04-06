mod align;
mod canvas;
mod constrained_box;
mod container;
mod empty;
mod event_handler;
mod flex;
mod label;
mod line_box;
mod new;
mod stack;
mod svg;
mod uniform_list;

pub use crate::presenter::ChildView;
pub use align::*;
pub use canvas::*;
pub use constrained_box::*;
pub use container::*;
pub use empty::*;
pub use event_handler::*;
pub use flex::*;
pub use label::*;
pub use line_box::*;
pub use new::*;
pub use stack::*;
pub use svg::*;
pub use uniform_list::*;

use crate::{
    AfterLayoutContext, AppContext, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub trait ParentElement<'a>: Extend<ElementBox> + Sized {
    fn add_children(&mut self, children: impl IntoIterator<Item = ElementBox>) {
        self.extend(children);
    }

    fn add_child(&mut self, child: ElementBox) {
        self.add_children(Some(child));
    }

    fn with_children(mut self, children: impl IntoIterator<Item = ElementBox>) -> Self {
        self.add_children(children);
        self
    }

    fn with_child(self, child: ElementBox) -> Self {
        self.with_children(Some(child))
    }
}

impl<'a, T> ParentElement<'a> for T where T: Extend<ElementBox> {}
