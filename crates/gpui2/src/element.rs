use crate::{
    AvailableSpace, BorrowWindow, Bounds, ElementId, LayoutId, Pixels, Point, Size, ViewContext,
    WindowContext,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{any::Any, fmt::Debug};

pub trait Render: 'static + Sized {
    type Element: Element + 'static;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element;
}

pub trait RenderOnce: 'static {
    type Output: IntoElement;

    fn render_once(self, cx: &mut WindowContext) -> Self::Output;
}

pub trait RenderOnceStateful: 'static {
    type Output: IntoElement;
    type State: 'static;

    fn element_id(&self) -> ElementId;

    fn render_once(self, state: &mut Option<Self::State>, cx: &mut WindowContext) -> Self::Output;
}

pub trait IntoElement: Sized {
    type Element: Element + 'static;

    fn element_id(&self) -> Option<ElementId>;

    fn into_element(self) -> Self::Element;

    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }

    fn draw<T, R>(
        self,
        origin: Point<Pixels>,
        available_space: Size<T>,
        cx: &mut WindowContext,
        f: impl FnOnce(&mut <Self::Element as Element>::State, &mut WindowContext) -> R,
    ) -> R
    where
        T: Clone + Default + Debug + Into<AvailableSpace>,
    {
        let element = self.into_element();
        let element_id = element.element_id();
        let element = DrawableElement {
            element: Some(element),
            phase: ElementDrawPhase::Start,
        };

        let frame_state =
            DrawableElement::draw(element, origin, available_space.map(Into::into), cx);

        if let Some(mut frame_state) = frame_state {
            f(&mut frame_state, cx)
        } else {
            cx.with_element_state(element_id.unwrap(), |element_state, cx| {
                let mut element_state = element_state.unwrap();
                let result = f(&mut element_state, cx);
                (result, element_state)
            })
        }
    }

    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
        U: IntoElement,
    {
        f(self)
    }

    fn when(self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| if condition { then(this) } else { this })
    }

    fn when_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| {
            if let Some(value) = option {
                then(this, value)
            } else {
                this
            }
        })
    }
}

pub trait Element: 'static + IntoElement {
    type State: 'static;

    fn layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State);

    fn paint(self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext);

    fn into_any(self) -> AnyElement {
        AnyElement::new(self)
    }
}

pub struct Component<R>(Option<R>);

pub struct ComponentState<C: RenderOnce> {
    rendered_element: Option<<C::Output as IntoElement>::Element>,
    rendered_element_state: <<C::Output as IntoElement>::Element as Element>::State,
}

impl<R> Component<R> {
    pub fn new(renderable: R) -> Self {
        Component(Some(renderable))
    }
}

impl<R: RenderOnce> Element for Component<R> {
    type State = ComponentState<R>;

    fn layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let mut element = self.0.take().unwrap().render_once(cx).into_element();
        let (layout_id, state) = element.layout(state.map(|s| s.rendered_element_state), cx);
        let state = ComponentState {
            rendered_element: Some(element),
            rendered_element_state: state,
        };
        (layout_id, state)
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        state
            .rendered_element
            .take()
            .unwrap()
            .paint(bounds, &mut state.rendered_element_state, cx);
    }
}

impl<C: RenderOnce> IntoElement for Component<C> {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct StatefulComponent<R>(Option<R>);

pub struct StatefulComponentState<R: RenderOnceStateful> {
    rendered_element: Option<<R::Output as IntoElement>::Element>,
    rendered_element_state: <<R::Output as IntoElement>::Element as Element>::State,
    component_state: Option<R::State>,
}

impl<R: RenderOnceStateful> Element for StatefulComponent<R> {
    type State = StatefulComponentState<R>;

    fn layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        if let Some(StatefulComponentState {
            rendered_element_state,
            mut component_state,
            ..
        }) = state
        {
            let mut rendered_element = self
                .0
                .take()
                .unwrap()
                .render_once(&mut component_state, cx)
                .into_element();
            let (layout_id, rendered_element_state) =
                rendered_element.layout(Some(rendered_element_state), cx);

            let state = StatefulComponentState {
                rendered_element: Some(rendered_element),
                rendered_element_state,
                component_state,
            };
            (layout_id, state)
        } else {
            let mut component_state = None;
            let mut rendered_element = self
                .0
                .take()
                .unwrap()
                .render_once(&mut component_state, cx)
                .into_element();
            let (layout_id, rendered_element_state) = rendered_element.layout(None, cx);

            let state = StatefulComponentState {
                rendered_element: Some(rendered_element),
                rendered_element_state,
                component_state,
            };
            (layout_id, state)
        }
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        state
            .rendered_element
            .take()
            .unwrap()
            .paint(bounds, &mut state.rendered_element_state, cx)
    }
}

impl<R: RenderOnceStateful> IntoElement for StatefulComponent<R> {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.0.as_ref().unwrap().element_id())
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GlobalElementId(SmallVec<[ElementId; 32]>);

pub trait ParentElement {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]>;

    fn child(mut self, child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child.into_element().into_any());
        self
    }

    fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.children_mut()
            .extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

trait ElementObject {
    fn element_id(&self) -> Option<ElementId>;

    fn layout(&mut self, cx: &mut WindowContext) -> LayoutId;

    fn paint(&mut self, cx: &mut WindowContext);

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Size<Pixels>;

    fn draw(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    );
}

pub struct DrawableElement<E: Element> {
    element: Option<E>,
    phase: ElementDrawPhase<E::State>,
}

#[derive(Default)]
enum ElementDrawPhase<S> {
    #[default]
    Start,
    LayoutRequested {
        layout_id: LayoutId,
        frame_state: Option<S>,
    },
    LayoutComputed {
        layout_id: LayoutId,
        available_space: Size<AvailableSpace>,
        frame_state: Option<S>,
    },
}

/// A wrapper around an implementer of [Element] that allows it to be drawn in a window.
impl<E: Element> DrawableElement<E> {
    fn new(element: E) -> Self {
        DrawableElement {
            element: Some(element),
            phase: ElementDrawPhase::Start,
        }
    }

    fn element_id(&self) -> Option<ElementId> {
        self.element.as_ref()?.element_id()
    }

    fn layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        let (layout_id, frame_state) = if let Some(id) = self.element.as_ref().unwrap().element_id()
        {
            let layout_id = cx.with_element_state(id, |element_state, cx| {
                self.element.as_mut().unwrap().layout(element_state, cx)
            });
            (layout_id, None)
        } else {
            let (layout_id, frame_state) = self.element.as_mut().unwrap().layout(None, cx);
            (layout_id, Some(frame_state))
        };

        self.phase = ElementDrawPhase::LayoutRequested {
            layout_id,
            frame_state,
        };
        layout_id
    }

    fn paint(mut self, cx: &mut WindowContext) -> Option<E::State> {
        match self.phase {
            ElementDrawPhase::LayoutRequested {
                layout_id,
                frame_state,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                frame_state,
                ..
            } => {
                let bounds = cx.layout_bounds(layout_id);

                if let Some(mut frame_state) = frame_state {
                    self.element
                        .take()
                        .unwrap()
                        .paint(bounds, &mut frame_state, cx);
                    Some(frame_state)
                } else {
                    let element_id = self
                        .element
                        .as_ref()
                        .unwrap()
                        .element_id()
                        .expect("if we don't have frame state, we should have element state");
                    cx.with_element_state(element_id, |element_state, cx| {
                        let mut element_state = element_state.unwrap();
                        self.element
                            .take()
                            .unwrap()
                            .paint(bounds, &mut element_state, cx);
                        ((), element_state)
                    });
                    None
                }
            }

            _ => panic!("must call layout before paint"),
        }
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.layout(cx);
        }

        let layout_id = match &mut self.phase {
            ElementDrawPhase::LayoutRequested {
                layout_id,
                frame_state,
            } => {
                cx.compute_layout(*layout_id, available_space);
                let layout_id = *layout_id;
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    frame_state: frame_state.take(),
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                available_space: prev_available_space,
                ..
            } => {
                if available_space != *prev_available_space {
                    cx.compute_layout(*layout_id, available_space);
                    *prev_available_space = available_space;
                }
                *layout_id
            }
            _ => panic!("cannot measure after painting"),
        };

        cx.layout_bounds(layout_id).size
    }

    fn draw(
        mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Option<E::State> {
        self.measure(available_space, cx);
        cx.with_absolute_element_offset(origin, |cx| self.paint(cx))
    }
}

impl<E> ElementObject for Option<DrawableElement<E>>
where
    E: Element,
    E::State: 'static,
{
    fn element_id(&self) -> Option<ElementId> {
        self.as_ref().unwrap().element_id()
    }

    fn layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        DrawableElement::layout(self.as_mut().unwrap(), cx)
    }

    fn paint(&mut self, cx: &mut WindowContext) {
        DrawableElement::paint(self.take().unwrap(), cx);
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Size<Pixels> {
        DrawableElement::measure(self.as_mut().unwrap(), available_space, cx)
    }

    fn draw(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) {
        DrawableElement::draw(self.take().unwrap(), origin, available_space, cx);
    }
}

pub struct AnyElement(Box<dyn ElementObject>);

impl AnyElement {
    pub fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::State: Any,
    {
        AnyElement(Box::new(Some(DrawableElement::new(element))) as Box<dyn ElementObject>)
    }

    pub fn element_id(&self) -> Option<ElementId> {
        self.0.element_id()
    }

    pub fn layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        self.0.layout(cx)
    }

    pub fn paint(mut self, cx: &mut WindowContext) {
        self.0.paint(cx)
    }

    /// Initializes this element and performs layout within the given available space to determine its size.
    pub fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Size<Pixels> {
        self.0.measure(available_space, cx)
    }

    /// Initializes this element and performs layout in the available space, then paints it at the given origin.
    pub fn draw(
        mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) {
        self.0.draw(origin, available_space, cx)
    }

    /// Converts this `AnyElement` into a trait object that can be stored and manipulated.
    pub fn into_any(self) -> AnyElement {
        AnyElement::new(self)
    }
}

impl Element for AnyElement {
    type State = ();

    fn layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let layout_id = self.layout(cx);
        (layout_id, ())
    }

    fn paint(self, _: Bounds<Pixels>, _: &mut Self::State, cx: &mut WindowContext) {
        self.paint(cx);
    }
}

impl IntoElement for AnyElement {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        AnyElement::element_id(self)
    }

    fn into_element(self) -> Self::Element {
        self
    }
}
