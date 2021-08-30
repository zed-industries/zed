mod align;
mod canvas;
mod constrained_box;
mod container;
mod empty;
mod event_handler;
mod flex;
mod label;
mod line_box;
mod list;
mod mouse_event_handler;
mod overlay;
mod stack;
mod svg;
mod text;
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
pub use list::*;
pub use mouse_event_handler::*;
pub use overlay::*;
pub use stack::*;
pub use svg::*;
pub use text::*;
pub use uniform_list::*;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json, DebugContext, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use core::panic;
use json::ToJson;
use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    mem,
    ops::{Deref, DerefMut},
    rc::Rc,
};

trait AnyElement {
    fn layout(&mut self, constraint: SizeConstraint, cx: &mut LayoutContext) -> Vector2F;
    fn paint(&mut self, origin: Vector2F, cx: &mut PaintContext);
    fn dispatch_event(&mut self, event: &Event, cx: &mut EventContext) -> bool;
    fn debug(&self, cx: &DebugContext) -> serde_json::Value;

    fn size(&self) -> Vector2F;
    fn metadata(&self) -> Option<&dyn Any>;
}

pub trait Element {
    type LayoutState;
    type PaintState;

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState);

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState;

    fn dispatch_event(
        &mut self,
        event: &Event,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        paint: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool;

    fn metadata(&self) -> Option<&dyn Any> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value;

    fn boxed(self) -> ElementBox
    where
        Self: 'static + Sized,
    {
        ElementBox(ElementRc {
            name: None,
            element: Rc::new(RefCell::new(Lifecycle::Init { element: self })),
        })
    }

    fn named(self, name: impl Into<Cow<'static, str>>) -> ElementBox
    where
        Self: 'static + Sized,
    {
        ElementBox(ElementRc {
            name: Some(name.into()),
            element: Rc::new(RefCell::new(Lifecycle::Init { element: self })),
        })
    }
}

pub enum Lifecycle<T: Element> {
    Empty,
    Init {
        element: T,
    },
    PostLayout {
        element: T,
        constraint: SizeConstraint,
        size: Vector2F,
        layout: T::LayoutState,
    },
    PostPaint {
        element: T,
        constraint: SizeConstraint,
        bounds: RectF,
        layout: T::LayoutState,
        paint: T::PaintState,
    },
}
pub struct ElementBox(ElementRc);

#[derive(Clone)]
pub struct ElementRc {
    name: Option<Cow<'static, str>>,
    element: Rc<RefCell<dyn AnyElement>>,
}

impl<T: Element> AnyElement for Lifecycle<T> {
    fn layout(&mut self, constraint: SizeConstraint, cx: &mut LayoutContext) -> Vector2F {
        let result;
        *self = match mem::take(self) {
            Lifecycle::Empty => unreachable!(),
            Lifecycle::Init { mut element }
            | Lifecycle::PostLayout { mut element, .. }
            | Lifecycle::PostPaint { mut element, .. } => {
                let (size, layout) = element.layout(constraint, cx);
                debug_assert!(size.x().is_finite());
                debug_assert!(size.y().is_finite());

                result = size;
                Lifecycle::PostLayout {
                    element,
                    constraint,
                    size,
                    layout,
                }
            }
        };
        result
    }

    fn paint(&mut self, origin: Vector2F, cx: &mut PaintContext) {
        *self = match mem::take(self) {
            Lifecycle::PostLayout {
                mut element,
                constraint,
                size,
                mut layout,
            } => {
                let bounds = RectF::new(origin, size);
                let paint = element.paint(bounds, &mut layout, cx);
                Lifecycle::PostPaint {
                    element,
                    constraint,
                    bounds,
                    layout,
                    paint,
                }
            }
            Lifecycle::PostPaint {
                mut element,
                constraint,
                bounds,
                mut layout,
                ..
            } => {
                let bounds = RectF::new(origin, bounds.size());
                let paint = element.paint(bounds, &mut layout, cx);
                Lifecycle::PostPaint {
                    element,
                    constraint,
                    bounds,
                    layout,
                    paint,
                }
            }
            _ => panic!("invalid element lifecycle state"),
        }
    }

    fn dispatch_event(&mut self, event: &Event, cx: &mut EventContext) -> bool {
        if let Lifecycle::PostPaint {
            element,
            bounds,
            layout,
            paint,
            ..
        } = self
        {
            element.dispatch_event(event, *bounds, layout, paint, cx)
        } else {
            panic!("invalid element lifecycle state");
        }
    }

    fn size(&self) -> Vector2F {
        match self {
            Lifecycle::Empty | Lifecycle::Init { .. } => panic!("invalid element lifecycle state"),
            Lifecycle::PostLayout { size, .. } => *size,
            Lifecycle::PostPaint { bounds, .. } => bounds.size(),
        }
    }

    fn metadata(&self) -> Option<&dyn Any> {
        match self {
            Lifecycle::Empty => unreachable!(),
            Lifecycle::Init { element }
            | Lifecycle::PostLayout { element, .. }
            | Lifecycle::PostPaint { element, .. } => element.metadata(),
        }
    }

    fn debug(&self, cx: &DebugContext) -> serde_json::Value {
        match self {
            Lifecycle::PostPaint {
                element,
                constraint,
                bounds,
                layout,
                paint,
            } => {
                let mut value = element.debug(*bounds, layout, paint, cx);
                if let json::Value::Object(map) = &mut value {
                    let mut new_map: crate::json::Map<String, serde_json::Value> =
                        Default::default();
                    if let Some(typ) = map.remove("type") {
                        new_map.insert("type".into(), typ);
                    }
                    new_map.insert("constraint".into(), constraint.to_json());
                    new_map.append(map);
                    json::Value::Object(new_map)
                } else {
                    value
                }
            }
            _ => panic!("invalid element lifecycle state"),
        }
    }
}

impl<T: Element> Default for Lifecycle<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl ElementBox {
    pub fn metadata(&self) -> Option<&dyn Any> {
        let element = unsafe { &*self.0.element.as_ptr() };
        element.metadata()
    }
}

impl Into<ElementRc> for ElementBox {
    fn into(self) -> ElementRc {
        self.0
    }
}

impl Deref for ElementBox {
    type Target = ElementRc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElementBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ElementRc {
    pub fn layout(&mut self, constraint: SizeConstraint, cx: &mut LayoutContext) -> Vector2F {
        self.element.borrow_mut().layout(constraint, cx)
    }

    pub fn paint(&mut self, origin: Vector2F, cx: &mut PaintContext) {
        self.element.borrow_mut().paint(origin, cx);
    }

    pub fn dispatch_event(&mut self, event: &Event, cx: &mut EventContext) -> bool {
        self.element.borrow_mut().dispatch_event(event, cx)
    }

    pub fn size(&self) -> Vector2F {
        self.element.borrow().size()
    }

    pub fn debug(&self, cx: &DebugContext) -> json::Value {
        let mut value = self.element.borrow().debug(cx);

        if let Some(name) = &self.name {
            if let json::Value::Object(map) = &mut value {
                let mut new_map: crate::json::Map<String, serde_json::Value> = Default::default();
                new_map.insert("name".into(), json::Value::String(name.to_string()));
                new_map.append(map);
                return json::Value::Object(new_map);
            }
        }

        value
    }
}

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
