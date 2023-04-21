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
use util::ResultExt;

pub trait Drawable<V: View>: 'static {
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

    fn into_element(self) -> Element<V>
    where
        Self: 'static + Sized,
    {
        Element {
            drawable: Box::new(Lifecycle::Init { element: self }),
            view_type: PhantomData,
            name: None,
        }
    }

    fn into_named_element(self, name: impl Into<Cow<'static, str>>) -> Element<V>
    where
        Self: 'static + Sized,
    {
        Element {
            drawable: Box::new(Lifecycle::Init { element: self }),
            view_type: PhantomData,
            name: Some(name.into()),
        }
    }

    fn into_root_element(self, cx: &ViewContext<V>) -> RootElement<V>
    where
        Self: 'static + Sized,
    {
        RootElement {
            element: self.into_element(),
            view: cx.handle().downgrade(),
        }
    }

    fn constrained(self) -> ConstrainedBox<V>
    where
        Self: 'static + Sized,
    {
        ConstrainedBox::new(self.into_element())
    }

    fn aligned(self) -> Align<V>
    where
        Self: 'static + Sized,
    {
        Align::new(self.into_element())
    }

    fn clipped(self) -> Clipped<V>
    where
        Self: 'static + Sized,
    {
        Clipped::new(self.into_element())
    }

    fn contained(self) -> Container<V>
    where
        Self: 'static + Sized,
    {
        Container::new(self.into_element())
    }

    fn expanded(self) -> Expanded<V>
    where
        Self: 'static + Sized,
    {
        Expanded::new(self.into_element())
    }

    fn flex(self, flex: f32, expanded: bool) -> FlexItem<V>
    where
        Self: 'static + Sized,
    {
        FlexItem::new(self.into_element()).flex(flex, expanded)
    }

    fn flex_float(self) -> FlexItem<V>
    where
        Self: 'static + Sized,
    {
        FlexItem::new(self.into_element()).float()
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
        Tooltip::new::<Tag, V>(id, text, action, style, self.into_element(), cx)
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
            self.into_element(),
            element_id,
            side,
            handle_size,
            initial_size,
            cx,
        )
    }
}

trait AnyDrawable<V: View> {
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

enum Lifecycle<V: View, E: Drawable<V>> {
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

impl<V: View, E: Drawable<V>> AnyDrawable<V> for Lifecycle<V, E> {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Vector2F {
        let result;
        *self = match mem::take(self) {
            Lifecycle::Empty => unreachable!(),
            Lifecycle::Init { mut element }
            | Lifecycle::PostLayout { mut element, .. }
            | Lifecycle::PostPaint { mut element, .. } => {
                let (size, layout) = element.layout(constraint, view, cx);
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

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        *self = match mem::take(self) {
            Lifecycle::PostLayout {
                mut element,
                constraint,
                size,
                mut layout,
            } => {
                let bounds = RectF::new(origin, size);
                let paint = element.paint(scene, bounds, visible_bounds, &mut layout, view, cx);
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
                let paint = element.paint(scene, bounds, visible_bounds, &mut layout, view, cx);
                Lifecycle::PostPaint {
                    element,
                    constraint,
                    bounds,
                    visible_bounds,
                    layout,
                    paint,
                }
            }
            Lifecycle::Empty => panic!("invalid element lifecycle state"),
            Lifecycle::Init { .. } => {
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
        if let Lifecycle::PostPaint {
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

    fn debug(&self, view: &V, cx: &ViewContext<V>) -> serde_json::Value {
        match self {
            Lifecycle::PostPaint {
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

impl<V: View, E: Drawable<V>> Default for Lifecycle<V, E> {
    fn default() -> Self {
        Self::Empty
    }
}

pub struct Element<V: View> {
    drawable: Box<dyn AnyDrawable<V>>,
    view_type: PhantomData<V>,
    name: Option<Cow<'static, str>>,
}

impl<V: View> Element<V> {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn metadata<T: 'static>(&self) -> Option<&T> {
        self.drawable
            .metadata()
            .and_then(|data| data.downcast_ref::<T>())
    }

    pub fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Vector2F {
        self.drawable.layout(constraint, view, cx)
    }

    pub fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        origin: Vector2F,
        visible_bounds: RectF,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        self.drawable.paint(scene, origin, visible_bounds, view, cx);
    }

    pub fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.drawable.rect_for_text_range(range_utf16, view, cx)
    }

    pub fn size(&self) -> Vector2F {
        self.drawable.size()
    }

    pub fn debug(&self, view: &V, cx: &ViewContext<V>) -> json::Value {
        let mut value = self.drawable.debug(view, cx);

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
        f(self.drawable.metadata().and_then(|m| m.downcast_ref()))
    }
}

impl<V: View> Drawable<V> for Element<V> {
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

    fn into_element(self) -> Element<V>
    where
        Self: Sized,
    {
        self
    }
}

pub struct RootElement<V: View> {
    element: Element<V>,
    view: WeakViewHandle<V>,
}

impl<V: View> RootElement<V> {
    pub fn new(element: Element<V>, view: WeakViewHandle<V>) -> Self {
        Self { element, view }
    }
}

pub trait Component<V: View>: 'static {
    fn render(&self, view: &mut V, cx: &mut ViewContext<V>) -> Element<V>;
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

impl<V: View, C: Component<V>> Drawable<V> for ComponentHost<V, C> {
    type LayoutState = Element<V>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Element<V>) {
        let mut element = self.component.render(view, cx);
        let size = element.layout(constraint, view, cx);
        (size, element)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        element: &mut Element<V>,
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
        element: &Element<V>,
        _: &(),
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        element.rect_for_text_range(range_utf16, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        element: &Element<V>,
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
        Ok(self.element.debug(view, &view_context))
    }

    fn name(&self) -> Option<&str> {
        self.element.name()
    }
}

impl<V: View, R: View> Drawable<V> for RootElement<R> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, ()) {
        let size = AnyRootElement::layout(self, constraint, cx)
            .log_err()
            .unwrap_or_else(|| Vector2F::zero());
        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _layout: &mut Self::LayoutState,
        _view: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        AnyRootElement::paint(self, scene, bounds.origin(), visible_bounds, cx).log_err();
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _bounds: RectF,
        _visible_bounds: RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        _view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        AnyRootElement::rect_for_text_range(self, range_utf16, cx)
            .log_err()
            .flatten()
    }

    fn debug(
        &self,
        _bounds: RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        _view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        AnyRootElement::debug(self, cx)
            .log_err()
            .unwrap_or_default()
    }
}

pub trait ParentElement<'a, V: View>: Extend<Element<V>> + Sized {
    fn add_children<D: Drawable<V>>(&mut self, children: impl IntoIterator<Item = D>) {
        self.extend(children.into_iter().map(|child| child.into_element()));
    }

    fn add_child<D: Drawable<V>>(&mut self, child: D) {
        self.extend(Some(child.into_element()));
    }

    fn with_children<D: Drawable<V>>(mut self, children: impl IntoIterator<Item = D>) -> Self {
        self.extend(children.into_iter().map(|child| child.into_element()));
        self
    }

    fn with_child<D: Drawable<V>>(mut self, child: D) -> Self {
        self.extend(Some(child.into_element()));
        self
    }
}

impl<'a, V: View, T> ParentElement<'a, V> for T where T: Extend<Element<V>> {}

pub fn constrain_size_preserving_aspect_ratio(max_size: Vector2F, size: Vector2F) -> Vector2F {
    if max_size.x().is_infinite() && max_size.y().is_infinite() {
        size
    } else if max_size.x().is_infinite() || max_size.x() / max_size.y() > size.x() / size.y() {
        vec2f(size.x() * max_size.y() / size.y(), max_size.y())
    } else {
        vec2f(max_size.x(), size.y() * max_size.x() / size.x())
    }
}
