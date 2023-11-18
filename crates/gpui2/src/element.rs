use crate::{
    AvailableSpace, BorrowWindow, Bounds, ElementId, LayoutId, Pixels, Point, Size, ViewContext,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{any::Any, fmt::Debug};

pub trait Element<V: 'static>: 'static + Sized {
    type State: 'static;

    fn element_id(&self) -> Option<ElementId>;

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State);

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        view_state: &mut V,
        element_state: &mut Self::State,
        cx: &mut ViewContext<V>,
    );

    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }

    fn draw<T, R>(
        self,
        origin: Point<Pixels>,
        available_space: Size<T>,
        view_state: &mut V,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut Self::State, &mut ViewContext<V>) -> R,
    ) -> R
    where
        T: Clone + Default + Debug + Into<AvailableSpace>,
    {
        let element_id = self.element_id();
        let element = DrawableElement {
            element: Some(self),
            phase: ElementDrawPhase::Start,
        };
        let frame_state = element.draw(origin, available_space.map(Into::into), view_state, cx);

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
}

#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GlobalElementId(SmallVec<[ElementId; 32]>);

pub trait ParentComponent<V: 'static> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]>;

    fn child(mut self, child: impl Component<V>) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child.render());
        self
    }

    fn children(mut self, iter: impl IntoIterator<Item = impl Component<V>>) -> Self
    where
        Self: Sized,
    {
        self.children_mut()
            .extend(iter.into_iter().map(|item| item.render()));
        self
    }
}

trait ElementObject<V> {
    fn layout(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> LayoutId;
    fn paint(&mut self, view_state: &mut V, cx: &mut ViewContext<V>);
    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Size<Pixels>;
    fn draw(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    );
}

pub struct DrawableElement<V: 'static, E: Element<V>> {
    element: Option<E>,
    phase: ElementDrawPhase<E::State>,
}

#[derive(Default)]
enum ElementDrawPhase<V> {
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
}

/// Internal struct that wraps an element to store Layout and ElementState after the element is rendered.
/// It's allocated as a trait object to erase the element type and wrapped in AnyElement<E::State> for
/// improved usability.
impl<V, E: Element<V>> DrawableElement<V, E> {
    fn new(element: E) -> Self {
        DrawableElement {
            element: Some(element),
            phase: ElementDrawPhase::Start,
        }
    }

    fn layout(&mut self, state: &mut V, cx: &mut ViewContext<V>) -> LayoutId {
        let (layout_id, frame_state) = if let Some(id) = self.element.as_ref().unwrap().element_id()
        {
            let layout_id = cx.with_element_state(id, |element_state, cx| {
                self.element
                    .as_mut()
                    .unwrap()
                    .layout(state, element_state, cx)
            });
            (layout_id, None)
        } else {
            let (layout_id, frame_state) = self.element.as_mut().unwrap().layout(state, None, cx);
            (layout_id, Some(frame_state))
        };

        self.phase = ElementDrawPhase::LayoutRequested {
            layout_id,
            frame_state,
        };
        layout_id
    }

    fn paint(mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> Option<E::State> {
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
                        .paint(bounds, view_state, &mut frame_state, cx);
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
                        self.element.take().unwrap().paint(
                            bounds,
                            view_state,
                            &mut element_state,
                            cx,
                        );
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
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.layout(view_state, cx);
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
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Option<E::State> {
        self.measure(available_space, view_state, cx);
        cx.with_absolute_element_offset(origin, |cx| self.paint(view_state, cx))
    }
}

impl<V, E> ElementObject<V> for Option<DrawableElement<V, E>>
where
    E: Element<V>,
    E::State: 'static,
{
    fn layout(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> LayoutId {
        DrawableElement::layout(self.as_mut().unwrap(), view_state, cx)
    }

    fn paint(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) {
        DrawableElement::paint(self.take().unwrap(), view_state, cx);
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Size<Pixels> {
        DrawableElement::measure(self.as_mut().unwrap(), available_space, view_state, cx)
    }

    fn draw(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        DrawableElement::draw(
            self.take().unwrap(),
            origin,
            available_space,
            view_state,
            cx,
        );
    }
}

pub struct AnyElement<V>(Box<dyn ElementObject<V>>);

impl<V> AnyElement<V> {
    pub fn new<E>(element: E) -> Self
    where
        V: 'static,
        E: 'static + Element<V>,
        E::State: Any,
    {
        AnyElement(Box::new(Some(DrawableElement::new(element))) as Box<dyn ElementObject<V>>)
    }

    pub fn layout(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> LayoutId {
        self.0.layout(view_state, cx)
    }

    pub fn paint(mut self, view_state: &mut V, cx: &mut ViewContext<V>) {
        self.0.paint(view_state, cx)
    }

    /// Initializes this element and performs layout within the given available space to determine its size.
    pub fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Size<Pixels> {
        self.0.measure(available_space, view_state, cx)
    }

    /// Initializes this element and performs layout in the available space, then paints it at the given origin.
    pub fn draw(
        mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        view_state: &mut V,
        cx: &mut ViewContext<V>,
    ) {
        self.0.draw(origin, available_space, view_state, cx)
    }
}

pub trait Component<V> {
    fn render(self) -> AnyElement<V>;

    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
        U: Component<V>,
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

impl<V> Component<V> for AnyElement<V> {
    fn render(self) -> AnyElement<V> {
        self
    }
}

impl<V, E, F> Element<V> for Option<F>
where
    V: 'static,
    E: 'static + Component<V>,
    F: FnOnce(&mut V, &mut ViewContext<'_, V>) -> E + 'static,
{
    type State = Option<AnyElement<V>>;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        _: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        let render = self.take().unwrap();
        let mut rendered_element = (render)(view_state, cx).render();
        let layout_id = rendered_element.layout(view_state, cx);
        (layout_id, Some(rendered_element))
    }

    fn paint(
        self,
        _bounds: Bounds<Pixels>,
        view_state: &mut V,
        rendered_element: &mut Self::State,
        cx: &mut ViewContext<V>,
    ) {
        rendered_element.take().unwrap().paint(view_state, cx);
    }
}

impl<V, E, F> Component<V> for Option<F>
where
    V: 'static,
    E: 'static + Component<V>,
    F: FnOnce(&mut V, &mut ViewContext<'_, V>) -> E + 'static,
{
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, E, F> Component<V> for F
where
    V: 'static,
    E: 'static + Component<V>,
    F: FnOnce(&mut V, &mut ViewContext<'_, V>) -> E + 'static,
{
    fn render(self) -> AnyElement<V> {
        AnyElement::new(Some(self))
    }
}
