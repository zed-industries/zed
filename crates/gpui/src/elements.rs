mod align;
mod canvas;
mod clipped;
mod constrained_box;
mod container;
mod empty;
mod expanded;
mod flex;
mod hook;
mod image;
mod keystroke_label;
mod label;
mod list;
mod mouse_event_handler;
mod overlay;
mod resizable;
mod stack;
mod svg;
mod text;
mod tooltip;
mod uniform_list;

pub use self::{
    align::*, canvas::*, constrained_box::*, container::*, empty::*, flex::*, hook::*, image::*,
    keystroke_label::*, label::*, list::*, mouse_event_handler::*, overlay::*, resizable::*,
    stack::*, svg::*, text::*, tooltip::*, uniform_list::*,
};
use self::{clipped::Clipped, expanded::Expanded};
pub use crate::presenter::ChildView;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json,
    presenter::MeasurementContext,
    Action, DebugContext, EventContext, LayoutContext, PaintContext, RenderContext, SizeConstraint,
    View,
};
use core::panic;
use json::ToJson;
use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    mem,
    ops::{Deref, DerefMut, Range},
    rc::Rc,
};

trait AnyElement {
    fn layout(&mut self, constraint: SizeConstraint, cx: &mut LayoutContext) -> Vector2F;
    fn paint(&mut self, origin: Vector2F, visible_bounds: RectF, cx: &mut PaintContext);
    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        cx: &MeasurementContext,
    ) -> Option<RectF>;
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
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState;

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF>;

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

    fn constrained(self) -> ConstrainedBox
    where
        Self: 'static + Sized,
    {
        ConstrainedBox::new(self.boxed())
    }

    fn aligned(self) -> Align
    where
        Self: 'static + Sized,
    {
        Align::new(self.boxed())
    }

    fn clipped(self) -> Clipped
    where
        Self: 'static + Sized,
    {
        Clipped::new(self.boxed())
    }

    fn contained(self) -> Container
    where
        Self: 'static + Sized,
    {
        Container::new(self.boxed())
    }

    fn expanded(self) -> Expanded
    where
        Self: 'static + Sized,
    {
        Expanded::new(self.boxed())
    }

    fn flex(self, flex: f32, expanded: bool) -> FlexItem
    where
        Self: 'static + Sized,
    {
        FlexItem::new(self.boxed()).flex(flex, expanded)
    }

    fn flex_float(self) -> FlexItem
    where
        Self: 'static + Sized,
    {
        FlexItem::new(self.boxed()).float()
    }

    fn with_tooltip<Tag: 'static, T: View>(
        self,
        id: usize,
        text: String,
        action: Option<Box<dyn Action>>,
        style: TooltipStyle,
        cx: &mut RenderContext<T>,
    ) -> Tooltip
    where
        Self: 'static + Sized,
    {
        Tooltip::new::<Tag, T>(id, text, action, style, self.boxed(), cx)
    }

    fn with_resize_handle<Tag: 'static, T: View>(
        self,
        element_id: usize,
        side: Side,
        handle_size: f32,
        initial_size: f32,
        cx: &mut RenderContext<T>,
    ) -> Resizable
    where
        Self: 'static + Sized,
    {
        Resizable::new::<Tag, T>(
            self.boxed(),
            element_id,
            side,
            handle_size,
            initial_size,
            cx,
        )
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
        visible_bounds: RectF,
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

    fn paint(&mut self, origin: Vector2F, visible_bounds: RectF, cx: &mut PaintContext) {
        *self = match mem::take(self) {
            Lifecycle::PostLayout {
                mut element,
                constraint,
                size,
                mut layout,
            } => {
                let bounds = RectF::new(origin, size);
                let paint = element.paint(bounds, visible_bounds, &mut layout, cx);
                Lifecycle::PostPaint {
                    element,
                    constraint,
                    bounds,
                    visible_bounds,
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
                let paint = element.paint(bounds, visible_bounds, &mut layout, cx);
                Lifecycle::PostPaint {
                    element,
                    constraint,
                    bounds,
                    visible_bounds,
                    layout,
                    paint,
                }
            }
            _ => panic!("invalid element lifecycle state"),
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        if let Lifecycle::PostPaint {
            element,
            bounds,
            visible_bounds,
            layout,
            paint,
            ..
        } = self
        {
            element.rect_for_text_range(range_utf16, *bounds, *visible_bounds, layout, paint, cx)
        } else {
            None
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
                visible_bounds,
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
                    new_map.insert("bounds".into(), bounds.to_json());
                    new_map.insert("visible_bounds".into(), visible_bounds.to_json());
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
    pub fn name(&self) -> Option<&str> {
        self.0.name.as_deref()
    }

    pub fn metadata<T: 'static>(&self) -> Option<&T> {
        let element = unsafe { &*self.0.element.as_ptr() };
        element.metadata().and_then(|m| m.downcast_ref())
    }
}

impl From<ElementBox> for ElementRc {
    fn from(val: ElementBox) -> Self {
        val.0
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

    pub fn paint(&mut self, origin: Vector2F, visible_bounds: RectF, cx: &mut PaintContext) {
        self.element.borrow_mut().paint(origin, visible_bounds, cx);
    }

    pub fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.element.borrow().rect_for_text_range(range_utf16, cx)
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

    pub fn with_metadata<T, F, R>(&self, f: F) -> R
    where
        T: 'static,
        F: FnOnce(Option<&T>) -> R,
    {
        let element = self.element.borrow();
        f(element.metadata().and_then(|m| m.downcast_ref()))
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

pub fn constrain_size_preserving_aspect_ratio(max_size: Vector2F, size: Vector2F) -> Vector2F {
    if max_size.x().is_infinite() && max_size.y().is_infinite() {
        size
    } else if max_size.x().is_infinite() || max_size.x() / max_size.y() > size.x() / size.y() {
        vec2f(size.x() * max_size.y() / size.y(), max_size.y())
    } else {
        vec2f(max_size.x(), size.y() * max_size.x() / size.x())
    }
}
