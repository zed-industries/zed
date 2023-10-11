use std::sync::Arc;

use crate::{
    BorrowWindow, Bounds, Clickable, ElementGroup, ElementId, LayoutId, MouseDownEvent,
    MouseUpEvent, Pixels, Point, SharedString, ViewContext,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;

pub trait Element: 'static + Send + Sync {
    type ViewState: 'static + Send + Sync;
    type ElementState: 'static + Send + Sync;

    fn element_id(&self) -> Option<ElementId>;

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState);

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    );

    fn group(self, name: impl Into<SharedString>) -> ElementGroup<Self>
    where
        Self: Sized,
    {
        ElementGroup::new(name.into(), self)
    }
}

pub trait IdentifiedElement: Element {
    fn element_id(&self) -> ElementId {
        Element::element_id(self).unwrap()
    }

    fn on_click(
        self,
        listener: impl Fn(
                &mut Self::ViewState,
                (&MouseDownEvent, &MouseUpEvent),
                &mut ViewContext<Self::ViewState>,
            ) + Send
            + Sync
            + 'static,
    ) -> Clickable<Self>
    where
        Self: Sized,
    {
        Clickable::new(self, Arc::from(listener))
    }
}

#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct GlobalElementId(SmallVec<[ElementId; 8]>);

pub trait ParentElement {
    type State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]>;

    fn child(mut self, child: impl IntoAnyElement<Self::State>) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child.into_any());
        self
    }

    fn children(mut self, iter: impl IntoIterator<Item = impl IntoAnyElement<Self::State>>) -> Self
    where
        Self: Sized,
    {
        self.children_mut()
            .extend(iter.into_iter().map(|item| item.into_any()));
        self
    }
}

trait ElementObject<S>: 'static + Send + Sync {
    fn layout(&mut self, state: &mut S, cx: &mut ViewContext<S>) -> LayoutId;
    fn paint(&mut self, state: &mut S, offset: Option<Point<Pixels>>, cx: &mut ViewContext<S>);
}

struct RenderedElement<E: Element> {
    element: E,
    phase: ElementRenderPhase<E::ElementState>,
}

#[derive(Default)]
enum ElementRenderPhase<S> {
    #[default]
    Rendered,
    LayoutRequested {
        layout_id: LayoutId,
        frame_state: Option<S>,
    },
    Painted {
        bounds: Bounds<Pixels>,
        frame_state: Option<S>,
    },
}

/// Internal struct that wraps an element to store Layout and ElementState after the element is rendered.
/// It's allocated as a trait object to erase the element type and wrapped in AnyElement<E::State> for
/// improved usability.
impl<E: Element> RenderedElement<E> {
    fn new(element: E) -> Self {
        RenderedElement {
            element,
            phase: ElementRenderPhase::Rendered,
        }
    }

    fn paint_with_element_state(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut E::ViewState,
        frame_state: &mut Option<E::ElementState>,
        cx: &mut ViewContext<E::ViewState>,
    ) {
        if let Some(id) = self.element.element_id() {
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
    }
}

impl<E, S> ElementObject<E::ViewState> for RenderedElement<E>
where
    E: Element<ElementState = S>,
    S: 'static + Send + Sync,
{
    fn layout(&mut self, state: &mut E::ViewState, cx: &mut ViewContext<E::ViewState>) -> LayoutId {
        let (layout_id, frame_state) = if let Some(id) = self.element.element_id() {
            let layout_id = cx.with_element_state(id, |element_state, cx| {
                self.element.layout(state, element_state, cx)
            });
            (layout_id, None)
        } else {
            let (layout_id, frame_state) = self.element.layout(state, None, cx);
            (layout_id, Some(frame_state))
        };

        self.phase = ElementRenderPhase::LayoutRequested {
            layout_id,
            frame_state,
        };

        layout_id
    }

    fn paint(
        &mut self,
        view_state: &mut E::ViewState,
        offset: Option<Point<Pixels>>,
        cx: &mut ViewContext<E::ViewState>,
    ) {
        self.phase = match std::mem::take(&mut self.phase) {
            ElementRenderPhase::Rendered => panic!("must call layout before paint"),

            ElementRenderPhase::LayoutRequested {
                layout_id,
                mut frame_state,
            } => {
                let mut bounds = cx.layout_bounds(layout_id);
                offset.map(|offset| bounds.origin += offset);
                self.paint_with_element_state(bounds, view_state, &mut frame_state, cx);
                ElementRenderPhase::Painted {
                    bounds,
                    frame_state,
                }
            }

            ElementRenderPhase::Painted {
                bounds,
                mut frame_state,
            } => {
                self.paint_with_element_state(bounds, view_state, &mut frame_state, cx);
                ElementRenderPhase::Painted {
                    bounds,
                    frame_state,
                }
            }
        };
    }
}

pub struct AnyElement<S>(Box<dyn ElementObject<S>>);

impl<S: 'static + Send + Sync> AnyElement<S> {
    pub fn layout(&mut self, state: &mut S, cx: &mut ViewContext<S>) -> LayoutId {
        self.0.layout(state, cx)
    }

    pub fn paint(&mut self, state: &mut S, offset: Option<Point<Pixels>>, cx: &mut ViewContext<S>) {
        self.0.paint(state, offset, cx)
    }
}

pub trait IntoAnyElement<S> {
    fn into_any(self) -> AnyElement<S>;
}

impl<E: Element> IntoAnyElement<E::ViewState> for E {
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement(Box::new(RenderedElement::new(self)))
    }
}

impl<S> IntoAnyElement<S> for AnyElement<S> {
    fn into_any(self) -> AnyElement<S> {
        self
    }
}
