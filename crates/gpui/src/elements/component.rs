use std::{any::Any, marker::PhantomData};

use pathfinder_geometry::{rect::RectF, vector::Vector2F};

use crate::{
    AnyElement, Element, LayoutContext, PaintContext, SceneBuilder, SizeConstraint, View,
    ViewContext,
};

use super::Empty;

pub trait Component {
    fn render<V: View>(self, v: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V>;

    fn element<V: View>(self) -> StatefulAdapter<V, Self>
    where
        Self: Sized,
    {
        StatefulAdapter::new(self)
    }

    fn stylable(self) -> StylableAdapter<Self>
    where
        Self: Sized,
    {
        StylableAdapter::new(self)
    }
}

pub struct StylableAdapter<C: Component> {
    component: C,
}

impl<C: Component> StylableAdapter<C> {
    pub fn new(component: C) -> Self {
        Self { component }
    }
}

impl<C: Component> StyleableComponent for StylableAdapter<C> {
    type Style = ();

    type Output = C;

    fn with_style(self, _: Self::Style) -> Self::Output {
        self.component
    }
}

pub trait StyleableComponent {
    type Style: Clone;
    type Output: Component;

    fn with_style(self, style: Self::Style) -> Self::Output;
}

impl Component for () {
    fn render<V: View>(self, _: &mut V, _: &mut ViewContext<V>) -> AnyElement<V> {
        Empty::new().into_any()
    }
}

impl StyleableComponent for () {
    type Style = ();
    type Output = ();

    fn with_style(self, _: Self::Style) -> Self::Output {
        ()
    }
}

pub trait StatefulStyleableComponent<V: View> {
    type Style: Clone;
    type Output: StatefulComponent<V>;

    fn stateful_with_style(self, style: Self::Style) -> Self::Output;
}

impl<V: View, C: StyleableComponent> StatefulStyleableComponent<V> for C {
    type Style = C::Style;

    type Output = C::Output;

    fn stateful_with_style(self, style: Self::Style) -> Self::Output {
        self.with_style(style)
    }
}

pub trait StatefulComponent<V: View> {
    fn render(self, v: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V>;

    fn stateful_element(self) -> StatefulAdapter<V, Self>
    where
        Self: Sized,
    {
        StatefulAdapter::new(self)
    }

    fn stateful_styleable(self) -> StatefulStylableAdapter<Self, V>
    where
        Self: Sized,
    {
        StatefulStylableAdapter::new(self)
    }
}

impl<V: View, C: Component> StatefulComponent<V> for C {
    fn render(self, v: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V> {
        self.render(v, cx)
    }
}

// StylableComponent -> Component
pub struct StatefulStylableAdapter<C: StatefulComponent<V>, V: View> {
    component: C,
    phantom: std::marker::PhantomData<V>,
}

impl<C: StatefulComponent<V>, V: View> StatefulStylableAdapter<C, V> {
    pub fn new(component: C) -> Self {
        Self {
            component,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<C: StatefulComponent<V>, V: View> StatefulStyleableComponent<V>
    for StatefulStylableAdapter<C, V>
{
    type Style = ();

    type Output = C;

    fn stateful_with_style(self, _: Self::Style) -> Self::Output {
        self.component
    }
}

// Element -> GeneralComponent

pub struct DynamicElementAdapter {
    element: Box<dyn Any>,
}

impl DynamicElementAdapter {
    pub fn new<V: View>(element: AnyElement<V>) -> Self {
        DynamicElementAdapter {
            element: Box::new(element) as Box<dyn Any>,
        }
    }
}

impl Component for DynamicElementAdapter {
    fn render<V: View>(self, _: &mut V, _: &mut ViewContext<V>) -> AnyElement<V> {
        let element = self
            .element
            .downcast::<AnyElement<V>>()
            .expect("Don't move elements out of their view :(");
        *element
    }
}

// Element -> Component
pub struct ElementAdapter<V: View> {
    element: AnyElement<V>,
    _phantom: std::marker::PhantomData<V>,
}

impl<V: View> ElementAdapter<V> {
    pub fn new(element: AnyElement<V>) -> Self {
        Self {
            element,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<V: View> StatefulComponent<V> for ElementAdapter<V> {
    fn render(self, _: &mut V, _: &mut ViewContext<V>) -> AnyElement<V> {
        self.element
    }
}

// Component -> Element
pub struct StatefulAdapter<V: View, E> {
    component: Option<E>,
    element: Option<AnyElement<V>>,
    phantom: PhantomData<V>,
}

impl<E, V: View> StatefulAdapter<V, E> {
    pub fn new(e: E) -> Self {
        Self {
            component: Some(e),
            element: None,
            phantom: PhantomData,
        }
    }
}

impl<V: View, C: StatefulComponent<V> + 'static> Element<V> for StatefulAdapter<V, C> {
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
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Self::PaintState {
        self.element
            .as_mut()
            .expect("Layout should always be called before paint")
            .paint(scene, bounds.origin(), visible_bounds, view, cx)
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
            "child": self.element.as_ref().map(|el| el.debug(view, cx)),
        })
    }
}
