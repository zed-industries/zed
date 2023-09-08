use std::{any::Any, marker::PhantomData};

use pathfinder_geometry::{rect::RectF, vector::Vector2F};

use crate::{AnyElement, Element, LayoutContext, PaintContext, SizeConstraint, ViewContext};

use super::Empty;

/// The core stateless component trait, simply rendering an element tree
pub trait Component {
    fn render<V: 'static>(self, cx: &mut ViewContext<V>) -> AnyElement<V>;

    fn element<V: 'static>(self) -> ComponentAdapter<V, Self>
    where
        Self: Sized,
    {
        ComponentAdapter::new(self)
    }

    fn stylable(self) -> StylableAdapter<Self>
    where
        Self: Sized,
    {
        StylableAdapter::new(self)
    }

    fn stateful<V: 'static>(self) -> StatefulAdapter<Self, V>
    where
        Self: Sized,
    {
        StatefulAdapter::new(self)
    }
}

/// Allows a a component's styles to be rebound in a simple way.
pub trait Stylable: Component {
    type Style: Clone;

    fn with_style(self, style: Self::Style) -> Self;
}

/// This trait models the typestate pattern for a component's style,
/// enforcing at compile time that a component is only usable after
/// it has been styled while still allowing for late binding of the
/// styling information
pub trait SafeStylable {
    type Style: Clone;
    type Output: Component;

    fn with_style(self, style: Self::Style) -> Self::Output;
}

/// All stylable components can trivially implement SafeStylable
impl<C: Stylable> SafeStylable for C {
    type Style = C::Style;

    type Output = C;

    fn with_style(self, style: Self::Style) -> Self::Output {
        self.with_style(style)
    }
}

/// Allows converting an unstylable component into a stylable one
/// by using `()` as the style type
pub struct StylableAdapter<C: Component> {
    component: C,
}

impl<C: Component> StylableAdapter<C> {
    pub fn new(component: C) -> Self {
        Self { component }
    }
}

impl<C: Component> SafeStylable for StylableAdapter<C> {
    type Style = ();

    type Output = C;

    fn with_style(self, _: Self::Style) -> Self::Output {
        self.component
    }
}

/// This is a secondary trait for components that can be styled
/// which rely on their view's state. This is useful for components that, for example,
/// want to take click handler callbacks Unfortunately, the generic bound on the
/// Component trait makes it incompatible with the stateless components above.
// So let's just replicate them for now
pub trait StatefulComponent<V: 'static> {
    fn render(self, v: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V>;

    fn element(self) -> ComponentAdapter<V, Self>
    where
        Self: Sized,
    {
        ComponentAdapter::new(self)
    }

    fn styleable(self) -> StatefulStylableAdapter<Self, V>
    where
        Self: Sized,
    {
        StatefulStylableAdapter::new(self)
    }

    fn stateless(self) -> StatelessElementAdapter
    where
        Self: Sized + 'static,
    {
        StatelessElementAdapter::new(self.element().into_any())
    }
}

/// It is trivial to convert stateless components to stateful components, so lets
/// do so en masse. Note that the reverse is impossible without a helper.
impl<V: 'static, C: Component> StatefulComponent<V> for C {
    fn render(self, _: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V> {
        self.render(cx)
    }
}

/// Same as stylable, but generic over a view type
pub trait StatefulStylable<V: 'static>: StatefulComponent<V> {
    type Style: Clone;

    fn with_style(self, style: Self::Style) -> Self;
}

/// Same as SafeStylable, but generic over a view type
pub trait StatefulSafeStylable<V: 'static> {
    type Style: Clone;
    type Output: StatefulComponent<V>;

    fn with_style(self, style: Self::Style) -> Self::Output;
}

/// Converting from stateless to stateful
impl<V: 'static, C: SafeStylable> StatefulSafeStylable<V> for C {
    type Style = C::Style;

    type Output = C::Output;

    fn with_style(self, style: Self::Style) -> Self::Output {
        self.with_style(style)
    }
}

// A helper for converting stateless components into stateful ones
pub struct StatefulAdapter<C, V> {
    component: C,
    phantom: std::marker::PhantomData<V>,
}

impl<C: Component, V: 'static> StatefulAdapter<C, V> {
    pub fn new(component: C) -> Self {
        Self {
            component,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Component, V: 'static> StatefulComponent<V> for StatefulAdapter<C, V> {
    fn render(self, _: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V> {
        self.component.render(cx)
    }
}

// A helper for converting stateful but style-less components into stylable ones
// by using `()` as the style type
pub struct StatefulStylableAdapter<C: StatefulComponent<V>, V: 'static> {
    component: C,
    phantom: std::marker::PhantomData<V>,
}

impl<C: StatefulComponent<V>, V: 'static> StatefulStylableAdapter<C, V> {
    pub fn new(component: C) -> Self {
        Self {
            component,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<C: StatefulComponent<V>, V: 'static> StatefulSafeStylable<V>
    for StatefulStylableAdapter<C, V>
{
    type Style = ();

    type Output = C;

    fn with_style(self, _: Self::Style) -> Self::Output {
        self.component
    }
}

/// A way of erasing the view generic from an element, useful
/// for wrapping up an explicit element tree into stateless
/// components
pub struct StatelessElementAdapter {
    element: Box<dyn Any>,
}

impl StatelessElementAdapter {
    pub fn new<V: 'static>(element: AnyElement<V>) -> Self {
        StatelessElementAdapter {
            element: Box::new(element) as Box<dyn Any>,
        }
    }
}

impl Component for StatelessElementAdapter {
    fn render<V: 'static>(self, _: &mut ViewContext<V>) -> AnyElement<V> {
        *self
            .element
            .downcast::<AnyElement<V>>()
            .expect("Don't move elements out of their view :(")
    }
}

// For converting elements into stateful components
pub struct StatefulElementAdapter<V: 'static> {
    element: AnyElement<V>,
    _phantom: std::marker::PhantomData<V>,
}

impl<V: 'static> StatefulElementAdapter<V> {
    pub fn new(element: AnyElement<V>) -> Self {
        Self {
            element,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<V: 'static> StatefulComponent<V> for StatefulElementAdapter<V> {
    fn render(self, _: &mut V, _: &mut ViewContext<V>) -> AnyElement<V> {
        self.element
    }
}

/// A convenient shorthand for creating an empty component.
impl Component for () {
    fn render<V: 'static>(self, _: &mut ViewContext<V>) -> AnyElement<V> {
        Empty::new().into_any()
    }
}

impl Stylable for () {
    type Style = ();

    fn with_style(self, _: Self::Style) -> Self {
        ()
    }
}

// For converting components back into Elements
pub struct ComponentAdapter<V: 'static, E> {
    component: Option<E>,
    element: Option<AnyElement<V>>,
    phantom: PhantomData<V>,
}

impl<E, V: 'static> ComponentAdapter<V, E> {
    pub fn new(e: E) -> Self {
        Self {
            component: Some(e),
            element: None,
            phantom: PhantomData,
        }
    }
}

impl<V: 'static, C: StatefulComponent<V> + 'static> Element<V> for ComponentAdapter<V, C> {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        if self.element.is_none() {
            let element = self
                .component
                .take()
                .expect("Component can only be rendered once")
                .render(view, cx.view_context());
            self.element = Some(element);
        }
        let constraint = self.element.as_mut().unwrap().layout(constraint, view, cx);
        (constraint, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        self.element
            .as_mut()
            .expect("Layout should always be called before paint")
            .paint(bounds.origin(), visible_bounds, view, cx)
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.element
            .as_ref()
            .and_then(|el| el.rect_for_text_range(range_utf16, view, cx))
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        serde_json::json!({
            "type": "ComponentAdapter",
            "component": std::any::type_name::<C>(),
            "child": self.element.as_ref().map(|el| el.debug(view, cx)),
        })
    }
}
