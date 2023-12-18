use crate::{
    frame_alloc, ArenaRef, AvailableSpace, BorrowWindow, Bounds, ElementId, LayoutId, Pixels,
    Point, Size, ViewContext, WindowContext,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{any::Any, fmt::Debug, mem};

pub trait Render: 'static + Sized {
    type Element: Element + 'static;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element;
}

pub trait IntoElement: Sized {
    type Element: Element + 'static;

    fn element_id(&self) -> Option<ElementId>;

    fn into_element(self) -> Self::Element;

    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }

    fn draw_and_update_state<T, R>(
        self,
        origin: Point<Pixels>,
        available_space: Size<T>,
        cx: &mut WindowContext,
        f: impl FnOnce(&mut <Self::Element as Element>::FrameState, &mut WindowContext) -> R,
    ) -> R
    where
        T: Clone + Default + Debug + Into<AvailableSpace>,
    {
        let element = self.into_element();
        let element = DrawableElement {
            element: Some(element),
            phase: ElementDrawPhase::Start,
        };

        let mut frame_state =
            DrawableElement::draw(element, origin, available_space.map(Into::into), cx);
        f(&mut frame_state, cx)
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
    type FrameState: 'static;

    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, Self::FrameState);

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::FrameState,
        cx: &mut WindowContext,
    );

    fn into_any(self) -> AnyElement {
        AnyElement::new(self)
    }
}

pub trait RenderOnce: 'static {
    type Rendered: IntoElement;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered;
}

pub struct Component<C> {
    component: Option<C>,
}

pub struct CompositeElementState<C: RenderOnce> {
    rendered_element: Option<<C::Rendered as IntoElement>::Element>,
    rendered_element_state: Option<<<C::Rendered as IntoElement>::Element as Element>::FrameState>,
}

impl<C> Component<C> {
    pub fn new(component: C) -> Self {
        Component {
            component: Some(component),
        }
    }
}

impl<C: RenderOnce> Element for Component<C> {
    type FrameState = CompositeElementState<C>;

    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, Self::FrameState) {
        let mut element = self.component.take().unwrap().render(cx).into_element();
        let (layout_id, state) = element.layout(cx);
        let state = CompositeElementState {
            rendered_element: Some(element),
            rendered_element_state: Some(state),
        };
        (layout_id, state)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::FrameState,
        cx: &mut WindowContext,
    ) {
        let mut element = state.rendered_element.take().unwrap();
        element.paint(
            bounds,
            &mut state.rendered_element_state.as_mut().unwrap(),
            cx,
        );
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
    phase: ElementDrawPhase<E::FrameState>,
}

#[derive(Default)]
enum ElementDrawPhase<S> {
    #[default]
    Start,
    LayoutRequested {
        layout_id: LayoutId,
        frame_state: S,
    },
    LayoutComputed {
        layout_id: LayoutId,
        available_space: Size<AvailableSpace>,
        frame_state: S,
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
        let (layout_id, frame_state) = self.element.as_mut().unwrap().layout(cx);
        self.phase = ElementDrawPhase::LayoutRequested {
            layout_id,
            frame_state,
        };
        layout_id
    }

    fn paint(mut self, cx: &mut WindowContext) -> E::FrameState {
        match self.phase {
            ElementDrawPhase::LayoutRequested {
                layout_id,
                mut frame_state,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                mut frame_state,
                ..
            } => {
                let bounds = cx.layout_bounds(layout_id);
                self.element
                    .take()
                    .unwrap()
                    .paint(bounds, &mut frame_state, cx);
                frame_state
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

        let layout_id = match mem::take(&mut self.phase) {
            ElementDrawPhase::LayoutRequested {
                layout_id,
                frame_state,
            } => {
                cx.compute_layout(layout_id, available_space);
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    frame_state,
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                available_space: prev_available_space,
                frame_state,
            } => {
                if available_space != prev_available_space {
                    cx.compute_layout(layout_id, available_space);
                }
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    frame_state,
                };

                layout_id
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
    ) -> E::FrameState {
        self.measure(available_space, cx);
        cx.with_absolute_element_offset(origin, |cx| self.paint(cx))
    }
}

impl<E> ElementObject for Option<DrawableElement<E>>
where
    E: Element,
    E::FrameState: 'static,
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

pub struct AnyElement(ArenaRef<dyn ElementObject>);

impl AnyElement {
    pub fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::FrameState: Any,
    {
        let element = frame_alloc(|| Some(DrawableElement::new(element)))
            .map(|element| element as &mut dyn ElementObject);
        AnyElement(element)
    }

    pub fn layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        self.0.layout(cx)
    }

    pub fn paint(&mut self, cx: &mut WindowContext) {
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
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) {
        self.0.draw(origin, available_space, cx)
    }

    pub fn inner_id(&self) -> Option<ElementId> {
        self.0.element_id()
    }
}

impl Element for AnyElement {
    type FrameState = ();

    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, Self::FrameState) {
        let layout_id = self.layout(cx);
        (layout_id, ())
    }

    fn paint(&mut self, _: Bounds<Pixels>, _: &mut Self::FrameState, cx: &mut WindowContext) {
        self.paint(cx)
    }
}

impl IntoElement for AnyElement {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }

    fn into_any_element(self) -> AnyElement {
        self
    }
}

/// The empty element, which renders nothing.
pub type Empty = ();

impl IntoElement for () {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for () {
    type FrameState = ();

    fn layout(&mut self, cx: &mut WindowContext) -> (LayoutId, Self::FrameState) {
        (cx.request_layout(&crate::Style::default(), None), ())
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::FrameState,
        _cx: &mut WindowContext,
    ) {
    }
}
