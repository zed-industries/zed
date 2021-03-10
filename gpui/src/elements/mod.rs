mod align;
mod constrained_box;
mod container;
mod empty;
mod event_handler;
mod flex;
mod label;
mod line_box;
mod stack;
mod svg;
mod uniform_list;

pub use align::*;
pub use constrained_box::*;
pub use container::*;
pub use empty::*;
pub use event_handler::*;
pub use flex::*;
pub use label::*;
pub use line_box::*;
pub use stack::*;
pub use svg::*;
pub use uniform_list::*;

use crate::{
    AfterLayoutContext, AppContext, Event, EventContext, LayoutContext, MutableAppContext,
    PaintContext, SizeConstraint,
};
use pathfinder_geometry::{rect::RectF, vector::Vector2F};
use std::any::Any;

pub trait Element {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F;

    fn after_layout(&mut self, _: &mut AfterLayoutContext, _: &mut MutableAppContext) {}

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext);

    fn size(&self) -> Option<Vector2F>;

    fn parent_data(&self) -> Option<&dyn Any> {
        None
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool;

    fn boxed(self) -> Box<dyn Element> {
        Box::new(self)
    }
}

pub trait ParentElement<'a>: Extend<Box<dyn Element>> + Sized {
    fn add_children(&mut self, children: impl IntoIterator<Item = Box<dyn Element>>) {
        self.extend(children);
    }

    fn add_child(&mut self, child: Box<dyn Element>) {
        self.add_childen(Some(child));
    }

    fn with_children(mut self, children: impl IntoIterator<Item = Box<dyn Element>>) -> Self {
        self.add_children(children);
        self
    }

    fn with_child(self, child: Box<dyn Element>) -> Self {
        self.with_children(Some(child))
    }
}

impl<'a, T> ParentElement<'a> for T where T: Extend<Box<dyn Element>> {}

pub fn try_rect(origin: Option<Vector2F>, size: Option<Vector2F>) -> Option<RectF> {
    origin.and_then(|origin| size.map(|size| RectF::new(origin, size)))
}
