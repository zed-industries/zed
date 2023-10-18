use crate::{BorrowWindow, Bounds, ElementId, FocusHandle, LayoutId, Pixels, Point, ViewContext};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;

pub trait Element: 'static + Send + Sync + IntoAnyElement<Self::ViewState> {
    type ViewState: 'static + Send + Sync;
    type ElementState: 'static + Send + Sync;

    fn id(&self) -> Option<ElementId>;

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
}

#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct GlobalElementId(SmallVec<[ElementId; 8]>);

pub trait ElementIdentity: 'static + Send + Sync {
    fn id(&self) -> Option<ElementId>;
}

pub struct Identified(pub(crate) ElementId);

impl ElementIdentity for Identified {
    fn id(&self) -> Option<ElementId> {
        Some(self.0.clone())
    }
}

pub struct Anonymous;

impl ElementIdentity for Anonymous {
    fn id(&self) -> Option<ElementId> {
        None
    }
}

pub trait ElementFocusability: 'static + Send + Sync {
    fn focus_handle(&self) -> Option<&FocusHandle>;
}

pub struct Focusable(FocusHandle);

impl ElementFocusability for Focusable {
    fn focus_handle(&self) -> Option<&FocusHandle> {
        Some(&self.0)
    }
}

impl From<FocusHandle> for Focusable {
    fn from(value: FocusHandle) -> Self {
        Self(value)
    }
}

pub struct NonFocusable;

impl ElementFocusability for NonFocusable {
    fn focus_handle(&self) -> Option<&FocusHandle> {
        None
    }
}

pub trait ParentElement: Element {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::ViewState>; 2]>;

    fn child(mut self, child: impl IntoAnyElement<Self::ViewState>) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child.into_any());
        self
    }

    fn children(
        mut self,
        iter: impl IntoIterator<Item = impl IntoAnyElement<Self::ViewState>>,
    ) -> Self
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
    }
}

impl<E, S> ElementObject<E::ViewState> for RenderedElement<E>
where
    E: Element<ElementState = S>,
    S: 'static + Send + Sync,
{
    fn layout(&mut self, state: &mut E::ViewState, cx: &mut ViewContext<E::ViewState>) -> LayoutId {
        let (layout_id, frame_state) = if let Some(id) = self.element.id() {
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
    pub fn new<E: Element<ViewState = S>>(element: E) -> Self {
        AnyElement(Box::new(RenderedElement::new(element)))
    }
}

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

impl<S> IntoAnyElement<S> for AnyElement<S> {
    fn into_any(self) -> AnyElement<S> {
        self
    }
}
