use crate::{BorrowWindow, Bounds, ElementId, LayoutId, Pixels, ViewContext};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{any::Any, mem};

pub trait Element<V: 'static> {
    type ElementState: 'static;

    fn id(&self) -> Option<ElementId>;

    /// Called to initialize this element for the current frame. If this
    /// element had state in a previous frame, it will be passed in for the 3rd argument.
    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState;

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId;

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    );
}

#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GlobalElementId(SmallVec<[ElementId; 32]>);

pub trait ParentElement<V: 'static> {
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
    fn initialize(&mut self, view_state: &mut V, cx: &mut ViewContext<V>);
    fn layout(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> LayoutId;
    fn paint(&mut self, view_state: &mut V, cx: &mut ViewContext<V>);
}

struct RenderedElement<V: 'static, E: Element<V>> {
    element: E,
    phase: ElementRenderPhase<E::ElementState>,
}

#[derive(Default)]
enum ElementRenderPhase<V> {
    #[default]
    Start,
    Initialized {
        frame_state: Option<V>,
    },
    LayoutRequested {
        layout_id: LayoutId,
        frame_state: Option<V>,
    },
    Painted,
}

/// Internal struct that wraps an element to store Layout and ElementState after the element is rendered.
/// It's allocated as a trait object to erase the element type and wrapped in AnyElement<E::State> for
/// improved usability.
impl<V, E: Element<V>> RenderedElement<V, E> {
    fn new(element: E) -> Self {
        RenderedElement {
            element,
            phase: ElementRenderPhase::Start,
        }
    }
}

impl<V, E> ElementObject<V> for RenderedElement<V, E>
where
    E: Element<V>,
    E::ElementState: 'static,
{
    fn initialize(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) {
        let frame_state = if let Some(id) = self.element.id() {
            cx.with_element_state(id, |element_state, cx| {
                let element_state = self.element.initialize(view_state, element_state, cx);
                ((), element_state)
            });
            None
        } else {
            let frame_state = self.element.initialize(view_state, None, cx);
            Some(frame_state)
        };

        self.phase = ElementRenderPhase::Initialized { frame_state };
    }

    fn layout(&mut self, state: &mut V, cx: &mut ViewContext<V>) -> LayoutId {
        let layout_id;
        let mut frame_state;
        match mem::take(&mut self.phase) {
            ElementRenderPhase::Initialized {
                frame_state: initial_frame_state,
            } => {
                frame_state = initial_frame_state;
                if let Some(id) = self.element.id() {
                    layout_id = cx.with_element_state(id, |element_state, cx| {
                        let mut element_state = element_state.unwrap();
                        let layout_id = self.element.layout(state, &mut element_state, cx);
                        (layout_id, element_state)
                    });
                } else {
                    layout_id = self
                        .element
                        .layout(state, frame_state.as_mut().unwrap(), cx);
                }
            }
            _ => panic!("must call initialize before layout"),
        };

        self.phase = ElementRenderPhase::LayoutRequested {
            layout_id,
            frame_state,
        };
        layout_id
    }

    fn paint(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) {
        self.phase = match mem::take(&mut self.phase) {
            ElementRenderPhase::LayoutRequested {
                layout_id,
                mut frame_state,
            } => {
                let bounds = cx.layout_bounds(layout_id);
                if let Some(id) = self.element.id() {
                    cx.with_element_state(id, |element_state, cx| {
                        let mut element_state = element_state.unwrap();
                        self.element
                            .paint(bounds, view_state, &mut element_state, cx);
                        ((), element_state)
                    });
                } else {
                    self.element
                        .paint(bounds, view_state, frame_state.as_mut().unwrap(), cx);
                }
                ElementRenderPhase::Painted
            }

            _ => panic!("must call layout before paint"),
        };
    }
}

pub struct AnyElement<V>(Box<dyn ElementObject<V>>);

impl<V> AnyElement<V> {
    pub fn new<E>(element: E) -> Self
    where
        V: 'static,
        E: 'static + Element<V>,
        E::ElementState: Any,
    {
        AnyElement(Box::new(RenderedElement::new(element)))
    }

    pub fn initialize(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) {
        self.0.initialize(view_state, cx);
    }

    pub fn layout(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> LayoutId {
        self.0.layout(view_state, cx)
    }

    pub fn paint(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) {
        self.0.paint(view_state, cx)
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

    fn when(mut self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        if condition {
            self = then(self);
        }
        self
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
    type ElementState = AnyElement<V>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        _rendered_element: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        let render = self.take().unwrap();
        let mut rendered_element = (render)(view_state, cx).render();
        rendered_element.initialize(view_state, cx);
        rendered_element
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
        rendered_element.layout(view_state, cx)
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        view_state: &mut V,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        rendered_element.paint(view_state, cx)
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
