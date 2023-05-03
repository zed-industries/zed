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
pub use crate::window::ChildView;

use self::{clipped::Clipped, expanded::Expanded};
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json, Action, SceneBuilder, SizeConstraint, View, ViewContext, WeakViewHandle, WindowContext,
};
use anyhow::{anyhow, Result};
use core::panic;
use json::ToJson;
use std::{
    any::Any,
    borrow::Cow,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut, Range},
};

pub trait Element<V: View>: 'static {
    type LayoutState;
    type PaintState;

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState);

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState;

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF>;

    fn metadata(&self) -> Option<&dyn Any> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value;

    fn into_any(self) -> AnyElement<V>
    where
        Self: 'static + Sized,
    {
        AnyElement {
            state: Box::new(ElementState::Init { element: self }),
            name: None,
        }
    }

    fn into_any_named(self, name: impl Into<Cow<'static, str>>) -> AnyElement<V>
    where
        Self: 'static + Sized,
    {
        AnyElement {
            state: Box::new(ElementState::Init { element: self }),
            name: Some(name.into()),
        }
    }

    fn into_root_element(self, cx: &ViewContext<V>) -> RootElement<V>
    where
        Self: 'static + Sized,
    {
        RootElement {
            element: self.into_any(),
            view: cx.handle().downgrade(),
        }
    }

    fn constrained(self) -> ConstrainedBox<V>
    where
        Self: 'static + Sized,
    {
        ConstrainedBox::new(self.into_any())
    }

    fn aligned(self) -> Align<V>
    where
        Self: 'static + Sized,
    {
        Align::new(self.into_any())
    }

    fn clipped(self) -> Clipped<V>
    where
        Self: 'static + Sized,
    {
        Clipped::new(self.into_any())
    }

    fn contained(self) -> Container<V>
    where
        Self: 'static + Sized,
    {
        Container::new(self.into_any())
    }

    fn expanded(self) -> Expanded<V>
    where
        Self: 'static + Sized,
    {
        Expanded::new(self.into_any())
    }

    fn flex(self, flex: f32, expanded: bool) -> FlexItem<V>
    where
        Self: 'static + Sized,
    {
        FlexItem::new(self.into_any()).flex(flex, expanded)
    }

    fn flex_float(self) -> FlexItem<V>
    where
        Self: 'static + Sized,
    {
        FlexItem::new(self.into_any()).float()
    }

    fn with_tooltip<Tag: 'static>(
        self,
        id: usize,
        text: String,
        action: Option<Box<dyn Action>>,
        style: TooltipStyle,
        cx: &mut ViewContext<V>,
    ) -> Tooltip<V>
    where
        Self: 'static + Sized,
    {
        Tooltip::new::<Tag, V>(id, text, action, style, self.into_any(), cx)
    }

    fn with_resize_handle<Tag: 'static>(
        self,
        element_id: usize,
        side: Side,
        handle_size: f32,
        initial_size: f32,
        cx: &mut ViewContext<V>,
    ) -> Resizable<V>
    where
        Self: 'static + Sized,
    {
        Resizable::new::<Tag, V>(
            self.into_any(),
            element_id,
            side,
            handle_size,
            initial_size,
            cx,
        )
    }
}

trait AnyElementState<V: View> {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Vector2F;

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        view: &mut V,
        cx: &mut ViewContext<V>,
    );

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF>;

    fn debug(&self, view: &V, cx: &ViewContext<V>) -> serde_json::Value;

    fn size(&self) -> Vector2F;

    fn metadata(&self) -> Option<&dyn Any>;
}

enum ElementState<V: View, E: Element<V>> {
    Empty,
    Init {
        element: E,
    },
    PostLayout {
        element: E,
        constraint: SizeConstraint,
        size: Vector2F,
        layout: E::LayoutState,
    },
    PostPaint {
        element: E,
        constraint: SizeConstraint,
        bounds: RectF,
        visible_bounds: RectF,
        layout: E::LayoutState,
        paint: E::PaintState,
    },
}

impl<V: View, E: Element<V>> AnyElementState<V> for ElementState<V, E> {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Vector2F {
        let result;
        *self = match mem::take(self) {
            ElementState::Empty => unreachable!(),
            ElementState::Init { mut element }
            | ElementState::PostLayout { mut element, .. }
            | ElementState::PostPaint { mut element, .. } => {
                let (size, layout) = element.layout(constraint, view, cx);
                debug_assert!(size.x().is_finite());
                debug_assert!(size.y().is_finite());

                result = size;
                ElementState::PostLayout {
                    element,
                    constraint,
                    size,
                    layout,
                }
            }
        };
        result
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        *self = match mem::take(self) {
            ElementState::PostLayout {
                mut element,
                constraint,
                size,
                mut layout,
            } => {
                let bounds = RectF::new(origin, size);
                let paint = element.paint(scene, bounds, visible_bounds, &mut layout, view, cx);
                ElementState::PostPaint {
                    element,
                    constraint,
                    bounds,
                    visible_bounds,
                    layout,
                    paint,
                }
            }
            ElementState::PostPaint {
                mut element,
                constraint,
                bounds,
                mut layout,
                ..
            } => {
                let bounds = RectF::new(origin, bounds.size());
                let paint = element.paint(scene, bounds, visible_bounds, &mut layout, view, cx);
                ElementState::PostPaint {
                    element,
                    constraint,
                    bounds,
                    visible_bounds,
                    layout,
                    paint,
                }
            }
            ElementState::Empty => panic!("invalid element lifecycle state"),
            ElementState::Init { .. } => {
                panic!("invalid element lifecycle state, paint called before layout")
            }
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        if let ElementState::PostPaint {
            element,
            bounds,
            visible_bounds,
            layout,
            paint,
            ..
        } = self
        {
            element.rect_for_text_range(
                range_utf16,
                *bounds,
                *visible_bounds,
                layout,
                paint,
                view,
                cx,
            )
        } else {
            None
        }
    }

    fn size(&self) -> Vector2F {
        match self {
            ElementState::Empty | ElementState::Init { .. } => {
                panic!("invalid element lifecycle state")
            }
            ElementState::PostLayout { size, .. } => *size,
            ElementState::PostPaint { bounds, .. } => bounds.size(),
        }
    }

    fn metadata(&self) -> Option<&dyn Any> {
        match self {
            ElementState::Empty => unreachable!(),
            ElementState::Init { element }
            | ElementState::PostLayout { element, .. }
            | ElementState::PostPaint { element, .. } => element.metadata(),
        }
    }

    fn debug(&self, view: &V, cx: &ViewContext<V>) -> serde_json::Value {
        match self {
            ElementState::PostPaint {
                element,
                constraint,
                bounds,
                visible_bounds,
                layout,
                paint,
            } => {
                let mut value = element.debug(*bounds, layout, paint, view, cx);
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

impl<V: View, E: Element<V>> Default for ElementState<V, E> {
    fn default() -> Self {
        Self::Empty
    }
}

pub struct AnyElement<V: View> {
    state: Box<dyn AnyElementState<V>>,
    name: Option<Cow<'static, str>>,
}

impl<V: View> AnyElement<V> {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn metadata<T: 'static>(&self) -> Option<&T> {
        self.state
            .metadata()
            .and_then(|data| data.downcast_ref::<T>())
    }

    pub fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Vector2F {
        self.state.layout(constraint, view, cx)
    }

    pub fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        self.state.paint(scene, origin, visible_bounds, view, cx);
    }

    pub fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.state.rect_for_text_range(range_utf16, view, cx)
    }

    pub fn size(&self) -> Vector2F {
        self.state.size()
    }

    pub fn debug(&self, view: &V, cx: &ViewContext<V>) -> json::Value {
        let mut value = self.state.debug(view, cx);

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
        f(self.state.metadata().and_then(|m| m.downcast_ref()))
    }
}

impl<V: View> Element<V> for AnyElement<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.layout(constraint, view, cx);
        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        self.paint(scene, bounds.origin(), visible_bounds, view, cx);
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        self.debug(view, cx)
    }

    fn into_any(self) -> AnyElement<V>
    where
        Self: Sized,
    {
        self
    }
}

pub struct RootElement<V: View> {
    element: AnyElement<V>,
    view: WeakViewHandle<V>,
}

impl<V: View> RootElement<V> {
    pub fn new(element: AnyElement<V>, view: WeakViewHandle<V>) -> Self {
        Self { element, view }
    }
}

pub trait Component<V: View>: 'static {
    fn render(&self, view: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V>;
}

pub struct ComponentHost<V: View, C: Component<V>> {
    component: C,
    view_type: PhantomData<V>,
}

impl<V: View, C: Component<V>> Deref for ComponentHost<V, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<V: View, C: Component<V>> DerefMut for ComponentHost<V, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}

impl<V: View, C: Component<V>> Element<V> for ComponentHost<V, C> {
    type LayoutState = AnyElement<V>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, AnyElement<V>) {
        let mut element = self.component.render(view, cx);
        let size = element.layout(constraint, view, cx);
        (size, element)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        element: &mut AnyElement<V>,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        element.paint(scene, bounds.origin(), visible_bounds, view, cx);
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        element: &AnyElement<V>,
        _: &(),
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        element.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        element: &AnyElement<V>,
        _: &(),
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        element.debug(view, cx)
    }
}

pub trait AnyRootElement {
    fn layout(&mut self, constraint: SizeConstraint, cx: &mut WindowContext) -> Result<Vector2F>;
    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        cx: &mut WindowContext,
    ) -> Result<()>;
    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        cx: &WindowContext,
    ) -> Result<Option<RectF>>;
    fn debug(&self, cx: &WindowContext) -> Result<serde_json::Value>;
    fn name(&self) -> Option<&str>;
}

impl<V: View> AnyRootElement for RootElement<V> {
    fn layout(&mut self, constraint: SizeConstraint, cx: &mut WindowContext) -> Result<Vector2F> {
        let view = self
            .view
            .upgrade(cx)
            .ok_or_else(|| anyhow!("layout called on a root element for a dropped view"))?;
        view.update(cx, |view, cx| Ok(self.element.layout(constraint, view, cx)))
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        cx: &mut WindowContext,
    ) -> Result<()> {
        let view = self
            .view
            .upgrade(cx)
            .ok_or_else(|| anyhow!("paint called on a root element for a dropped view"))?;

        view.update(cx, |view, cx| {
            self.element.paint(scene, origin, visible_bounds, view, cx);
            Ok(())
        })
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        cx: &WindowContext,
    ) -> Result<Option<RectF>> {
        let view = self.view.upgrade(cx).ok_or_else(|| {
            anyhow!("rect_for_text_range called on a root element for a dropped view")
        })?;
        let view = view.read(cx);
        let view_context = ViewContext::immutable(cx, self.view.id());
        Ok(self
            .element
            .rect_for_text_range(range_utf16, view, &view_context))
    }

    fn debug(&self, cx: &WindowContext) -> Result<serde_json::Value> {
        let view = self
            .view
            .upgrade(cx)
            .ok_or_else(|| anyhow!("debug called on a root element for a dropped view"))?;
        let view = view.read(cx);
        let view_context = ViewContext::immutable(cx, self.view.id());
        Ok(serde_json::json!({
            "view_id": self.view.id(),
            "view_name": V::ui_name(),
            "view": view.debug_json(cx),
            "element": self.element.debug(view, &view_context)
        }))
    }

    fn name(&self) -> Option<&str> {
        self.element.name()
    }
}

pub trait ParentElement<'a, V: View>: Extend<AnyElement<V>> + Sized {
    fn add_children<E: Element<V>>(&mut self, children: impl IntoIterator<Item = E>) {
        self.extend(children.into_iter().map(|child| child.into_any()));
    }

    fn add_child<D: Element<V>>(&mut self, child: D) {
        self.extend(Some(child.into_any()));
    }

    fn with_children<D: Element<V>>(mut self, children: impl IntoIterator<Item = D>) -> Self {
        self.extend(children.into_iter().map(|child| child.into_any()));
        self
    }

    fn with_child<D: Element<V>>(mut self, child: D) -> Self {
        self.extend(Some(child.into_any()));
        self
    }
}

impl<'a, V: View, T> ParentElement<'a, V> for T where T: Extend<AnyElement<V>> {}

pub fn constrain_size_preserving_aspect_ratio(max_size: Vector2F, size: Vector2F) -> Vector2F {
    if max_size.x().is_infinite() && max_size.y().is_infinite() {
        size
    } else if max_size.x().is_infinite() || max_size.x() / max_size.y() > size.x() / size.y() {
        vec2f(size.x() * max_size.y() / size.y(), max_size.y())
    } else {
        vec2f(max_size.x(), size.y() * max_size.x() / size.x())
    }
}
