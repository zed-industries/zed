use super::{Handle, Layout, LayoutId, Pixels, Point, ViewContext, WindowContext};
use anyhow::Result;
use std::{any::Any, cell::RefCell, marker::PhantomData, rc::Rc};

pub trait Element: 'static {
    type State;
    type FrameState;

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)>;

    fn paint(
        &mut self,
        layout: Layout,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()>;
}

pub trait ParentElement<S> {
    fn child(self, child: impl IntoAnyElement<S>) -> Self;
}

trait ElementObject<S> {
    fn layout(&mut self, state: &mut S, cx: &mut ViewContext<S>) -> Result<LayoutId>;
    fn paint(
        &mut self,
        parent_origin: super::Point<Pixels>,
        state: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<()>;
}

struct RenderedElement<E: Element> {
    element: E,
    phase: ElementRenderPhase<E::FrameState>,
}

#[derive(Default)]
enum ElementRenderPhase<S> {
    #[default]
    Rendered,
    LayoutRequested {
        layout_id: LayoutId,
        frame_state: S,
    },
    Painted {
        layout: Layout,
        frame_state: S,
    },
}

/// Internal struct that wraps an element to store Layout and FrameState after the element is rendered.
/// It's allocated as a trait object to erase the element type and wrapped in AnyElement<E::State> for
/// improved usability.
impl<E: Element> RenderedElement<E> {
    fn new(element: E) -> Self {
        RenderedElement {
            element,
            phase: ElementRenderPhase::Rendered,
        }
    }
}

impl<E: Element> ElementObject<E::State> for RenderedElement<E> {
    fn layout(&mut self, state: &mut E::State, cx: &mut ViewContext<E::State>) -> Result<LayoutId> {
        let (layout_id, frame_state) = self.element.layout(state, cx)?;
        self.phase = ElementRenderPhase::LayoutRequested {
            layout_id,
            frame_state,
        };
        Ok(layout_id)
    }

    fn paint(
        &mut self,
        parent_origin: Point<Pixels>,
        state: &mut E::State,
        cx: &mut ViewContext<E::State>,
    ) -> Result<()> {
        self.phase = match std::mem::take(&mut self.phase) {
            ElementRenderPhase::Rendered => panic!("must call layout before paint"),

            ElementRenderPhase::LayoutRequested {
                layout_id,
                mut frame_state,
            } => {
                let mut layout = cx.layout(layout_id)?;
                layout.bounds.origin += parent_origin;
                self.element
                    .paint(layout.clone(), state, &mut frame_state, cx)?;
                ElementRenderPhase::Painted {
                    layout,
                    frame_state,
                }
            }

            ElementRenderPhase::Painted {
                layout,
                mut frame_state,
            } => {
                self.element
                    .paint(layout.clone(), state, &mut frame_state, cx)?;
                ElementRenderPhase::Painted {
                    layout,
                    frame_state,
                }
            }
        };

        Ok(())
    }
}

pub struct AnyElement<S>(Box<dyn ElementObject<S>>);

impl<S> AnyElement<S> {
    pub fn layout(&mut self, state: &mut S, cx: &mut ViewContext<S>) -> Result<LayoutId> {
        self.0.layout(state, cx)
    }

    pub fn paint(
        &mut self,
        parent_origin: Point<Pixels>,
        state: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<()> {
        self.0.paint(parent_origin, state, cx)
    }
}

pub trait IntoAnyElement<S> {
    fn into_any(self) -> AnyElement<S>;
}

impl<E: Element> IntoAnyElement<E::State> for E {
    fn into_any(self) -> AnyElement<E::State> {
        AnyElement(Box::new(RenderedElement::new(self)))
    }
}

impl<S> IntoAnyElement<S> for AnyElement<S> {
    fn into_any(self) -> AnyElement<S> {
        self
    }
}

#[derive(Clone)]
pub struct View<S> {
    state: Handle<S>,
    render: Rc<dyn Fn(&mut S, &mut ViewContext<S>) -> AnyElement<S>>,
}

pub fn view<S: 'static, E: Element<State = S>>(
    state: Handle<S>,
    render: impl 'static + Fn(&mut S, &mut ViewContext<S>) -> E,
) -> View<S> {
    View {
        state,
        render: Rc::new(move |state, cx| render(state, cx).into_any()),
    }
}

impl<S: 'static> View<S> {
    pub fn into_any<ParentState>(self) -> AnyView<ParentState> {
        AnyView {
            view: Rc::new(RefCell::new(self)),
            parent_state_type: PhantomData,
        }
    }
}

impl<S: 'static> Element for View<S> {
    type State = ();
    type FrameState = AnyElement<S>;

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.state.update(cx, |state, cx| {
            let mut element = (self.render)(state, cx);
            let layout_id = element.layout(state, cx)?;
            Ok((layout_id, element))
        })
    }

    fn paint(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        element: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.state.update(cx, |state, cx| {
            element.paint(layout.bounds.origin, state, cx)
        })
    }
}

trait ViewObject {
    fn layout(&mut self, cx: &mut WindowContext) -> Result<(LayoutId, Box<dyn Any>)>;
    fn paint(
        &mut self,
        layout: Layout,
        element: &mut dyn Any,
        cx: &mut WindowContext,
    ) -> Result<()>;
}

impl<S: 'static> ViewObject for View<S> {
    fn layout(&mut self, cx: &mut WindowContext) -> Result<(LayoutId, Box<dyn Any>)> {
        self.state.update(cx, |state, cx| {
            let mut element = (self.render)(state, cx);
            let layout_id = element.layout(state, cx)?;
            let element = Box::new(element) as Box<dyn Any>;
            Ok((layout_id, element))
        })
    }

    fn paint(
        &mut self,
        layout: Layout,
        element: &mut dyn Any,
        cx: &mut WindowContext,
    ) -> Result<()> {
        self.state.update(cx, |state, cx| {
            element
                .downcast_mut::<AnyElement<S>>()
                .unwrap()
                .paint(layout.bounds.origin, state, cx)
        })
    }
}

pub struct AnyView<S> {
    view: Rc<RefCell<dyn ViewObject>>,
    parent_state_type: PhantomData<S>,
}

impl<S: 'static> Element for AnyView<S> {
    type State = S;
    type FrameState = Box<dyn Any>;

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.view.borrow_mut().layout(cx)
    }

    fn paint(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        element: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.view.borrow_mut().paint(layout, element, cx)
    }
}

impl<S> Clone for AnyView<S> {
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
            parent_state_type: PhantomData,
        }
    }
}
