use crate::{
    group_bounds, AnyElement, BorrowWindow, DispatchPhase, Element, ElementId, IdentifiedElement,
    IntoAnyElement, MouseDownEvent, MouseMoveEvent, MouseUpEvent, SharedString, Style,
    StyleCascade, StyleRefinement, ViewContext,
};
use parking_lot::Mutex;
use refineable::{CascadeSlot, Refineable};
use smallvec::SmallVec;
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc,
};

trait LayoutNode<V: 'static + Send + Sync, K: ElementKind> {
    fn state(&mut self) -> &mut LayoutNodeElement<V, K>;

    fn child(mut self, child: impl IntoAnyElement<V>) -> Self
    where
        Self: Sized,
    {
        self.state().children.push(child.into_any());
        self
    }

    fn children<C, E>(mut self, children: C) -> Self
    where
        C: IntoIterator<Item = E>,
        E: IntoAnyElement<V>,
        Self: Sized,
    {
        for child in children {
            self.state().children.push(child.into_any());
        }
        self
    }
}

pub trait ElementKind: 'static + Send + Sync {
    fn id(&self) -> Option<ElementId>;
}

pub struct Identified(ElementId);
pub struct Anonymous;

impl ElementKind for Identified {
    fn id(&self) -> Option<ElementId> {
        Some(self.0.clone())
    }
}

impl ElementKind for Anonymous {
    fn id(&self) -> Option<ElementId> {
        None
    }
}

struct LayoutNodeElement<V: 'static + Send + Sync, K: ElementKind> {
    style_cascade: StyleCascade,
    computed_style: Option<Style>,
    children: SmallVec<[AnyElement<V>; 2]>,
    kind: K,
}

impl<V: 'static + Send + Sync> LayoutNodeElement<V, Anonymous> {
    pub fn identify(self, id: impl Into<ElementId>) -> LayoutNodeElement<V, Identified> {
        LayoutNodeElement {
            style_cascade: self.style_cascade,
            computed_style: self.computed_style,
            children: self.children,
            kind: Identified(id.into()),
        }
    }
}

impl<V: 'static + Send + Sync, E: ElementKind> LayoutNodeElement<V, E> {
    fn with_element_id<R>(
        &mut self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut Self, &mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(id) = self.id() {
            cx.with_element_id(id, |cx| f(self, cx))
        } else {
            f(self, cx)
        }
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Styled for LayoutNodeElement<V, K> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        &mut self.style_cascade
    }

    fn computed_style(&mut self) -> &Style {
        self.computed_style
            .get_or_insert_with(|| Style::default().refined(self.style_cascade.merged()))
    }
}

impl<V: 'static + Send + Sync> IdentifiedElement for LayoutNodeElement<V, Identified> {
    fn element_id(&self) -> ElementId {
        self.kind.0.clone()
    }
}

impl<V, K> IntoAnyElement<V> for LayoutNodeElement<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Element for LayoutNodeElement<V, K> {
    type ViewState = V;
    type ElementState = ();

    fn id(&self) -> Option<ElementId> {
        self.kind.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        self.with_element_id(cx, |this, cx| {
            let layout_ids = this
                .children
                .iter_mut()
                .map(|child| child.layout(state, cx))
                .collect::<Vec<_>>();

            let style = this.computed_style().clone();
            let layout_id = cx.request_layout(style, layout_ids);
            (layout_id, ())
        })
    }

    fn paint(
        &mut self,
        _: crate::Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        _: &mut Self::ElementState,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) {
        self.with_element_id(cx, |this, cx| {
            for child in &mut this.children {
                child.paint(state, None, cx);
            }
        })
    }
}

pub trait Styled {
    fn style_cascade(&mut self) -> &mut StyleCascade;
    fn computed_style(&mut self) -> &Style;
}

pub trait Hoverable {
    fn hover_style(&mut self) -> &mut StyleRefinement;

    fn hover(mut self, f: impl FnOnce(&mut StyleRefinement) -> &mut StyleRefinement) -> Self
    where
        Self: Sized,
    {
        f(self.hover_style());
        self
    }
}

struct HoverableElement<E> {
    hover_style: StyleRefinement,
    group: Option<SharedString>,
    cascade_slot: CascadeSlot,
    hovered: Arc<AtomicBool>,
    child: E,
}

impl<E: Styled + Element> HoverableElement<E> {
    pub fn replace_child<E2: Element<ViewState = E::ViewState>>(
        self,
        replace: impl FnOnce(E) -> E2,
    ) -> HoverableElement<E2> {
        HoverableElement {
            hover_style: self.hover_style,
            group: self.group,
            cascade_slot: self.cascade_slot,
            hovered: self.hovered,
            child: replace(self.child),
        }
    }

    fn hover_style(&mut self) -> &mut StyleRefinement {
        &mut self.hover_style
    }
}

impl<E> IntoAnyElement<E::ViewState> for HoverableElement<E>
where
    E: Styled + Element,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E> Element for HoverableElement<E>
where
    E: Styled + Element,
{
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn id(&self) -> Option<ElementId> {
        self.child.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        self.child.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) {
        let target_bounds = self
            .group
            .as_ref()
            .and_then(|group| group_bounds(group, cx))
            .unwrap_or(bounds);

        let hovered = target_bounds.contains_point(cx.mouse_position());

        let slot = self.cascade_slot;
        let style = hovered.then_some(self.hover_style.clone());
        self.child.style_cascade().set(slot, style);
        self.hovered.store(hovered, SeqCst);

        let hovered = self.hovered.clone();
        cx.on_mouse_event(move |_, event: &MouseMoveEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if target_bounds.contains_point(event.position) != hovered.load(SeqCst) {
                    cx.notify();
                }
            }
        });

        self.child.paint(bounds, state, element_state, cx);
    }
}

impl<E: Styled + Element> Styled for HoverableElement<E> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        self.child.style_cascade()
    }

    fn computed_style(&mut self) -> &Style {
        self.child.computed_style()
    }
}

impl<E: Styled + IdentifiedElement> IdentifiedElement for HoverableElement<E> {}

pub trait Clickable: Element + Sized {
    fn active_style(&mut self) -> &mut StyleRefinement;
    fn listeners(&mut self) -> &mut ClickListeners<Self::ViewState>;

    fn on_click(
        &mut self,
        f: impl Fn(&mut Self::ViewState, &MouseClickEvent, &mut ViewContext<Self::ViewState>)
            + 'static
            + Send
            + Sync,
    ) where
        Self: Sized,
    {
        self.listeners().push(Arc::new(f));
    }

    fn active(mut self, f: impl FnOnce(&mut StyleRefinement) -> &mut StyleRefinement) -> Self
    where
        Self: Sized,
    {
        f(self.active_style());
        self
    }
}

type ClickListeners<V> =
    SmallVec<[Arc<dyn Fn(&mut V, &MouseClickEvent, &mut ViewContext<V>) + Send + Sync>; 1]>;

pub struct ClickableElementState<E: 'static + Send + Sync> {
    mouse_down: Arc<Mutex<Option<MouseDownEvent>>>,
    child_state: E,
}

pub struct MouseClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}

pub struct ClickableElement<E: Element> {
    child: E,
    listeners: ClickListeners<E::ViewState>,
    active_style: StyleRefinement,
    cascade_slot: CascadeSlot,
}

impl<E: Element> ClickableElement<E> {
    pub fn replace_child<E2: Element<ViewState = E::ViewState>>(
        self,
        replace: impl FnOnce(E) -> E2,
    ) -> ClickableElement<E2> {
        ClickableElement {
            child: replace(self.child),
            listeners: self.listeners,
            active_style: self.active_style,
            cascade_slot: self.cascade_slot,
        }
    }
}

impl<E> IntoAnyElement<E::ViewState> for ClickableElement<E>
where
    E: Styled + Element,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E> Element for ClickableElement<E>
where
    E: Styled + Element,
{
    type ViewState = E::ViewState;
    type ElementState = ClickableElementState<E::ElementState>;

    fn id(&self) -> Option<ElementId> {
        self.child.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        if let Some(element_state) = element_state {
            if element_state.mouse_down.lock().is_some() {
                self.child
                    .style_cascade()
                    .set(self.cascade_slot, Some(self.active_style.clone()));
            }

            let (layout_id, child_state) =
                self.child
                    .layout(state, Some(element_state.child_state), cx);
            (
                layout_id,
                ClickableElementState {
                    mouse_down: element_state.mouse_down,
                    child_state,
                },
            )
        } else {
            let (layout_id, child_state) = self.child.layout(state, None, cx);
            (
                layout_id,
                ClickableElementState {
                    mouse_down: Default::default(),
                    child_state,
                },
            )
        }
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) {
        if !self.listeners.is_empty() || self.active_style.is_some() {
            if let Some(mouse_down) = element_state.mouse_down.lock().clone() {
                self.child
                    .style_cascade()
                    .set(self.cascade_slot, Some(self.active_style.clone()));
                let listeners = self.listeners.clone();
                let mouse_down_mutex = element_state.mouse_down.clone();
                cx.on_mouse_event(move |view, up: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(up.position) {
                        for listener in &*listeners {
                            listener(
                                view,
                                &MouseClickEvent {
                                    down: mouse_down.clone(),
                                    up: up.clone(),
                                },
                                cx,
                            );
                        }
                    }

                    mouse_down_mutex.lock().take();
                    cx.notify();
                });
            } else {
                let mouse_down_mutex = element_state.mouse_down.clone();
                cx.on_mouse_event(move |_view, down: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(down.position) {
                        *mouse_down_mutex.lock() = Some(down.clone());
                        cx.notify();
                    }
                });
            }
        }

        self.child
            .paint(bounds, state, &mut element_state.child_state, cx);
    }
}

impl<E: Styled + IdentifiedElement> IdentifiedElement for ClickableElement<E> {}

impl<E> Clickable for ClickableElement<E>
where
    E: Styled + IdentifiedElement,
{
    fn active_style(&mut self) -> &mut StyleRefinement {
        &mut self.active_style
    }

    fn listeners(&mut self) -> &mut ClickListeners<Self::ViewState> {
        &mut self.listeners
    }
}

pub struct Div<V: 'static + Send + Sync, K: ElementKind>(
    ClickableElement<HoverableElement<LayoutNodeElement<V, K>>>,
);

impl<V: 'static + Send + Sync> Div<V, Anonymous> {
    pub fn id(self, id: impl Into<ElementId>) -> Div<V, Identified> {
        Div(self.0.replace_child(|hoverable| {
            hoverable.replace_child(|layout_node| layout_node.identify(id))
        }))
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> LayoutNode<V, K> for Div<V, K> {
    fn state(&mut self) -> &mut LayoutNodeElement<V, K> {
        &mut self.0.child.child
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Styled for Div<V, K> {
    fn style_cascade(&mut self) -> &mut StyleCascade {
        self.0.child.child.style_cascade()
    }

    fn computed_style(&mut self) -> &Style {
        self.0.child.child.computed_style()
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Hoverable for Div<V, K> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        self.0.child.hover_style()
    }
}

impl<V: 'static + Send + Sync> Clickable for Div<V, Identified> {
    fn active_style(&mut self) -> &mut StyleRefinement {
        self.0.active_style()
    }

    fn listeners(&mut self) -> &mut ClickListeners<V> {
        self.0.listeners()
    }
}

impl<V, K> IntoAnyElement<V> for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, K> Element for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    type ViewState = V;
    type ElementState = ClickableElementState<()>;

    fn id(&self) -> Option<ElementId> {
        self.0.id()
    }

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        self.0.layout(state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<Self::ViewState>,
    ) {
        self.0.paint(bounds, state, element_state, cx);
    }
}
