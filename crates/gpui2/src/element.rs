use crate::{
    AvailableSpace, BorrowWindow, Bounds, ElementId, LayoutId, Pixels, Point, Size, WindowContext,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{any::Any, fmt::Debug, mem};

pub trait Element {
    type ElementState: 'static;

    fn element_id(&self) -> Option<ElementId>;

    fn layout(
        &mut self,
        element_state: Option<Self::ElementState>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::ElementState);

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        element_state: &mut Self::ElementState,
        cx: &mut WindowContext,
    );

    fn draw<T, R>(
        self,
        origin: Point<Pixels>,
        available_space: Size<T>,
        cx: &mut WindowContext,
        f: impl FnOnce(&Self::ElementState, &mut WindowContext) -> R,
    ) -> R
    where
        Self: Sized,
        T: Clone + Default + Debug + Into<AvailableSpace>,
    {
        let mut element = RenderedElement {
            element: self,
            phase: ElementRenderPhase::Start,
        };
        element.draw(origin, available_space.map(Into::into), cx);
        if let ElementRenderPhase::Painted { frame_state } = &element.phase {
            if let Some(frame_state) = frame_state.as_ref() {
                f(&frame_state, cx)
            } else {
                let element_id = element
                    .element
                    .element_id()
                    .expect("we either have some frame_state or some element_id");
                cx.with_element_state(element_id, |element_state, cx| {
                    let element_state = element_state.unwrap();
                    let result = f(&element_state, cx);
                    (result, element_state)
                })
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GlobalElementId(SmallVec<[ElementId; 32]>);

pub trait ParentComponent {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]>;

    fn child(mut self, child: impl Component) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child.render());
        self
    }

    fn children(mut self, iter: impl IntoIterator<Item = impl Component>) -> Self
    where
        Self: Sized,
    {
        self.children_mut()
            .extend(iter.into_iter().map(|item| item.render()));
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

struct RenderedElement<E: Element> {
    element: E,
    phase: ElementRenderPhase<E::ElementState>,
}

#[derive(Default)]
enum ElementRenderPhase<V> {
    #[default]
    Start,
    LayoutRequested {
        layout_id: LayoutId,
        frame_state: Option<V>,
    },
    LayoutComputed {
        layout_id: LayoutId,
        available_space: Size<AvailableSpace>,
        frame_state: Option<V>,
    },
    Painted {
        frame_state: Option<V>,
    },
}

/// Internal struct that wraps an element to store Layout and ElementState after the element is rendered.
/// It's allocated as a trait object to erase the element type and wrapped in AnyElement<E::State> for
/// improved usability.
impl<E: Element> RenderedElement<E> {
    fn new(element: E) -> Self {
        RenderedElement {
            element,
            phase: ElementRenderPhase::Start,
        }
    }
}

impl<E> ElementObject for RenderedElement<E>
where
    E: Element,
    E::ElementState: 'static,
{
    fn element_id(&self) -> Option<ElementId> {
        self.element.element_id()
    }

    fn layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        let (layout_id, frame_state) = match mem::take(&mut self.phase) {
            ElementRenderPhase::Start => {
                if let Some(id) = self.element.element_id() {
                    let layout_id = cx.with_element_state(id, |element_state, cx| {
                        self.element.layout(element_state, cx)
                    });
                    (layout_id, None)
                } else {
                    let (layout_id, frame_state) = self.element.layout(None, cx);
                    (layout_id, Some(frame_state))
                }
            }
            ElementRenderPhase::LayoutRequested { .. }
            | ElementRenderPhase::LayoutComputed { .. }
            | ElementRenderPhase::Painted { .. } => {
                panic!("element rendered twice")
            }
        };

        self.phase = ElementRenderPhase::LayoutRequested {
            layout_id,
            frame_state,
        };
        layout_id
    }

    fn paint(&mut self, cx: &mut WindowContext) {
        self.phase = match mem::take(&mut self.phase) {
            ElementRenderPhase::LayoutRequested {
                layout_id,
                mut frame_state,
            }
            | ElementRenderPhase::LayoutComputed {
                layout_id,
                mut frame_state,
                ..
            } => {
                let bounds = cx.layout_bounds(layout_id);
                if let Some(id) = self.element.element_id() {
                    cx.with_element_state(id, |element_state, cx| {
                        let mut element_state = element_state.unwrap();
                        self.element.paint(bounds, &mut element_state, cx);
                        ((), element_state)
                    });
                } else {
                    self.element
                        .paint(bounds, frame_state.as_mut().unwrap(), cx);
                }
                ElementRenderPhase::Painted { frame_state }
            }

            _ => panic!("must call layout before paint"),
        };
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementRenderPhase::Start) {
            self.layout(cx);
        }

        let layout_id = match &mut self.phase {
            ElementRenderPhase::LayoutRequested {
                layout_id,
                frame_state,
            } => {
                cx.compute_layout(*layout_id, available_space);
                let layout_id = *layout_id;
                self.phase = ElementRenderPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    frame_state: frame_state.take(),
                };
                layout_id
            }
            ElementRenderPhase::LayoutComputed {
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
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) {
        self.measure(available_space, cx);
        cx.with_absolute_element_offset(origin, |cx| self.paint(cx))
    }
}

pub struct AnyElement(Box<dyn ElementObject>);

impl AnyElement {
    pub fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::ElementState: Any,
    {
        AnyElement(Box::new(RenderedElement::new(element)))
    }

    pub fn element_id(&self) -> Option<ElementId> {
        self.0.element_id()
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
}

pub trait Component {
    fn render(self) -> AnyElement;

    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
        U: Component,
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

impl Component for AnyElement {
    fn render(self) -> AnyElement {
        self
    }
}

impl<E, F> Element for Option<F>
where
    E: 'static + Component,
    F: FnOnce(&mut WindowContext) -> E + 'static,
{
    type ElementState = AnyElement;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn layout(
        &mut self,
        _: Option<Self::ElementState>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::ElementState) {
        let render = self.take().unwrap();
        let mut rendered_element = (render)(cx).render();
        let layout_id = rendered_element.layout(cx);
        (layout_id, rendered_element)
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        rendered_element: &mut Self::ElementState,
        cx: &mut WindowContext,
    ) {
        rendered_element.paint(cx)
    }
}

impl<E, F> Component for Option<F>
where
    E: 'static + Component,
    F: FnOnce(&mut WindowContext) -> E + 'static,
{
    fn render(self) -> AnyElement {
        AnyElement::new(self)
    }
}

impl<E, F> Component for F
where
    E: 'static + Component,
    F: FnOnce(&mut WindowContext) -> E + 'static,
{
    fn render(self) -> AnyElement {
        AnyElement::new(Some(self))
    }
}
