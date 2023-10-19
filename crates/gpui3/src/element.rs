use crate::{
    BorrowWindow, Bounds, DispatchPhase, ElementId, FocusHandle, FocusListeners, LayoutId,
    MouseDownEvent, Pixels, Point, Style, StyleRefinement, ViewContext, WindowContext,
};
use derive_more::{Deref, DerefMut};
use refineable::Refineable;
pub(crate) use smallvec::SmallVec;
use std::{marker::PhantomData, mem};

pub trait Element: 'static + Send + Sync + IntoAnyElement<Self::ViewState> {
    type ViewState: 'static + Send + Sync;
    type ElementState: 'static + Send + Sync;

    fn id(&self) -> Option<ElementId>;

    fn initialize(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState;

    fn layout(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> LayoutId;

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    );
}

#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GlobalElementId(SmallVec<[ElementId; 32]>);

pub trait ElementInteractivity<V: 'static + Send + Sync>: 'static + Send + Sync {
    fn as_stateful(&self) -> Option<&StatefulInteractivity<V>>;

    fn initialize<R>(
        &self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(Option<GlobalElementId>, &mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(identified) = self.as_stateful() {
            cx.with_element_id(identified.id.clone(), |global_id, cx| {
                f(Some(global_id), cx)
            })
        } else {
            f(None, cx)
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct StatefulInteractivity<V: 'static + Send + Sync> {
    pub id: ElementId,
    #[deref]
    #[deref_mut]
    common: StatelessInteractivity<V>,
}

impl<V> ElementInteractivity<V> for StatefulInteractivity<V>
where
    V: 'static + Send + Sync,
{
    fn as_stateful(&self) -> Option<&StatefulInteractivity<V>> {
        Some(self)
    }
}

impl<V> From<ElementId> for StatefulInteractivity<V>
where
    V: 'static + Send + Sync,
{
    fn from(id: ElementId) -> Self {
        Self {
            id,
            common: StatelessInteractivity::default(),
        }
    }
}

pub struct StatelessInteractivity<V>(PhantomData<V>);

impl<V> Default for StatelessInteractivity<V> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<V> ElementInteractivity<V> for StatelessInteractivity<V>
where
    V: 'static + Send + Sync,
{
    fn as_stateful(&self) -> Option<&StatefulInteractivity<V>> {
        None
    }
}

pub trait ElementFocusability<V: 'static + Send + Sync>: 'static + Send + Sync {
    fn as_focusable(&self) -> Option<&Focusable<V>>;

    fn initialize<R>(
        &self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(focusable) = self.as_focusable() {
            for listener in focusable.focus_listeners.iter().cloned() {
                cx.on_focus_changed(move |view, event, cx| listener(view, event, cx));
            }
            cx.with_focus(focusable.focus_handle.clone(), |cx| f(cx))
        } else {
            f(cx)
        }
    }

    fn refine_style(&self, style: &mut Style, cx: &WindowContext) {
        if let Some(focusable) = self.as_focusable() {
            if focusable.focus_handle.contains_focused(cx) {
                style.refine(&focusable.focus_in_style);
            }

            if focusable.focus_handle.within_focused(cx) {
                style.refine(&focusable.in_focus_style);
            }

            if focusable.focus_handle.is_focused(cx) {
                style.refine(&focusable.focus_style);
            }
        }
    }

    fn paint(&self, bounds: Bounds<Pixels>, cx: &mut WindowContext) {
        if let Some(focusable) = self.as_focusable() {
            let focus_handle = focusable.focus_handle.clone();
            cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    if !cx.default_prevented() {
                        cx.focus(&focus_handle);
                        cx.prevent_default();
                    }
                }
            })
        }
    }
}

pub struct Focusable<V: 'static + Send + Sync> {
    pub focus_handle: FocusHandle,
    pub focus_listeners: FocusListeners<V>,
    pub focus_style: StyleRefinement,
    pub focus_in_style: StyleRefinement,
    pub in_focus_style: StyleRefinement,
}

impl<V> ElementFocusability<V> for Focusable<V>
where
    V: 'static + Send + Sync,
{
    fn as_focusable(&self) -> Option<&Focusable<V>> {
        Some(self)
    }
}

impl<V> From<FocusHandle> for Focusable<V>
where
    V: 'static + Send + Sync,
{
    fn from(value: FocusHandle) -> Self {
        Self {
            focus_handle: value,
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }
}

pub struct NonFocusable;

impl<V> ElementFocusability<V> for NonFocusable
where
    V: 'static + Send + Sync,
{
    fn as_focusable(&self) -> Option<&Focusable<V>> {
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

trait ElementObject<V>: 'static + Send + Sync {
    fn initialize(&mut self, view_state: &mut V, cx: &mut ViewContext<V>);
    fn layout(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> LayoutId;
    fn paint(&mut self, view_state: &mut V, offset: Option<Point<Pixels>>, cx: &mut ViewContext<V>);
}

struct RenderedElement<E: Element> {
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
impl<E: Element> RenderedElement<E> {
    fn new(element: E) -> Self {
        RenderedElement {
            element,
            phase: ElementRenderPhase::Start,
        }
    }
}

impl<E> ElementObject<E::ViewState> for RenderedElement<E>
where
    E: Element,
{
    fn initialize(&mut self, view_state: &mut E::ViewState, cx: &mut ViewContext<E::ViewState>) {
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

    fn layout(&mut self, state: &mut E::ViewState, cx: &mut ViewContext<E::ViewState>) -> LayoutId {
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

    fn paint(
        &mut self,
        view_state: &mut E::ViewState,
        offset: Option<Point<Pixels>>,
        cx: &mut ViewContext<E::ViewState>,
    ) {
        self.phase = match mem::take(&mut self.phase) {
            ElementRenderPhase::LayoutRequested {
                layout_id,
                mut frame_state,
            } => {
                let mut bounds = cx.layout_bounds(layout_id);
                offset.map(|offset| bounds.origin += offset);
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

impl<V: 'static + Send + Sync> AnyElement<V> {
    pub fn new<E: Element<ViewState = V>>(element: E) -> Self {
        AnyElement(Box::new(RenderedElement::new(element)))
    }

    pub fn initialize(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) {
        self.0.initialize(view_state, cx);
    }

    pub fn layout(&mut self, view_state: &mut V, cx: &mut ViewContext<V>) -> LayoutId {
        self.0.layout(view_state, cx)
    }

    pub fn paint(
        &mut self,
        view_state: &mut V,
        offset: Option<Point<Pixels>>,
        cx: &mut ViewContext<V>,
    ) {
        self.0.paint(view_state, offset, cx)
    }
}

pub trait IntoAnyElement<V> {
    fn into_any(self) -> AnyElement<V>;
}

impl<V> IntoAnyElement<V> for AnyElement<V> {
    fn into_any(self) -> AnyElement<V> {
        self
    }
}
