use crate::{
    AppContext, BorrowWindow, Bounds, DispatchPhase, ElementId, FocusHandle, FocusListeners,
    KeyDownEvent, KeyListener, KeyMatch, LayoutId, MouseClickEvent, MouseClickListener,
    MouseDownEvent, MouseDownListener, MouseMoveEvent, MouseMoveListener, MouseUpEvent,
    MouseUpListener, Pixels, Point, ScrollWheelEvent, ScrollWheelListener, SharedString, Style,
    StyleRefinement, ViewContext, WindowContext,
};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use refineable::Refineable;
pub(crate) use smallvec::SmallVec;
use std::{any::TypeId, mem, sync::Arc};

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
    fn as_stateless(&self) -> &StatelessInteractivity<V>;
    fn as_stateless_mut(&mut self) -> &mut StatelessInteractivity<V>;
    fn as_stateful(&self) -> Option<&StatefulInteractivity<V>>;
    fn as_stateful_mut(&mut self) -> Option<&mut StatefulInteractivity<V>>;

    fn initialize<R>(
        &mut self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(stateful) = self.as_stateful_mut() {
            cx.with_element_id(stateful.id.clone(), |global_id, cx| {
                stateful.key_listeners.push((
                    TypeId::of::<KeyDownEvent>(),
                    Arc::new(move |_, key_down, context, phase, cx| {
                        if phase == DispatchPhase::Bubble {
                            let key_down = key_down.downcast_ref::<KeyDownEvent>().unwrap();
                            if let KeyMatch::Some(action) =
                                cx.match_keystroke(&global_id, &key_down.keystroke, context)
                            {
                                return Some(action);
                            }
                        }

                        None
                    }),
                ));
                let result = stateful.stateless.initialize(cx, f);
                stateful.key_listeners.pop();
                result
            })
        } else {
            cx.with_key_listeners(&self.as_stateless().key_listeners, f)
        }
    }

    fn refine_style(&self, style: &mut Style, bounds: Bounds<Pixels>, cx: &mut ViewContext<V>) {
        let mouse_position = cx.mouse_position();
        let stateless = self.as_stateless();
        if let Some(group_hover) = stateless.group_hover.as_ref() {
            if let Some(group_bounds) = group_bounds(&group_hover.group, cx) {
                if group_bounds.contains_point(&mouse_position) {
                    style.refine(&group_hover.style);
                }
            }
        }
        if bounds.contains_point(&mouse_position) {
            style.refine(&stateless.hover_style);
        }
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        pending_click: Arc<Mutex<Option<MouseDownEvent>>>,
        cx: &mut ViewContext<V>,
    ) {
        let stateless = self.as_stateless();
        for listener in stateless.mouse_down_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseDownEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.mouse_up_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.mouse_move_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseMoveEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.scroll_wheel_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &ScrollWheelEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        let hover_group_bounds = stateless
            .group_hover
            .as_ref()
            .and_then(|group_hover| GroupBounds::get(&group_hover.group, cx));

        if let Some(group_bounds) = hover_group_bounds {
            paint_hover_listener(group_bounds, cx);
        }

        if stateless.hover_style.is_some() {
            paint_hover_listener(bounds, cx);
        }

        if let Some(stateful) = self.as_stateful() {
            let click_listeners = stateful.mouse_click_listeners.clone();

            let mouse_down = pending_click.lock().clone();
            if let Some(mouse_down) = mouse_down {
                cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                        let mouse_click = MouseClickEvent {
                            down: mouse_down.clone(),
                            up: event.clone(),
                        };
                        for listener in &click_listeners {
                            listener(state, &mouse_click, cx);
                        }
                    }

                    *pending_click.lock() = None;
                });
            } else {
                cx.on_mouse_event(move |_state, event: &MouseDownEvent, phase, _cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                        *pending_click.lock() = Some(event.clone());
                    }
                });
            };
        }
    }
}

fn paint_hover_listener<V>(bounds: Bounds<Pixels>, cx: &mut ViewContext<V>)
where
    V: 'static + Send + Sync,
{
    let hovered = bounds.contains_point(&cx.mouse_position());
    cx.on_mouse_event(move |_, event: &MouseMoveEvent, phase, cx| {
        if phase == DispatchPhase::Capture {
            if bounds.contains_point(&event.position) != hovered {
                cx.notify();
            }
        }
    });
}

#[derive(Deref, DerefMut)]
pub struct StatefulInteractivity<V: 'static + Send + Sync> {
    pub id: ElementId,
    #[deref]
    #[deref_mut]
    stateless: StatelessInteractivity<V>,
    pub mouse_click_listeners: SmallVec<[MouseClickListener<V>; 2]>,
}

impl<V> ElementInteractivity<V> for StatefulInteractivity<V>
where
    V: 'static + Send + Sync,
{
    fn as_stateful(&self) -> Option<&StatefulInteractivity<V>> {
        Some(self)
    }

    fn as_stateful_mut(&mut self) -> Option<&mut StatefulInteractivity<V>> {
        Some(self)
    }

    fn as_stateless(&self) -> &StatelessInteractivity<V> {
        &self.stateless
    }

    fn as_stateless_mut(&mut self) -> &mut StatelessInteractivity<V> {
        &mut self.stateless
    }
}

impl<V> From<ElementId> for StatefulInteractivity<V>
where
    V: 'static + Send + Sync,
{
    fn from(id: ElementId) -> Self {
        Self {
            id,
            stateless: StatelessInteractivity::default(),
            mouse_click_listeners: SmallVec::new(),
        }
    }
}

pub struct StatelessInteractivity<V> {
    pub mouse_down_listeners: SmallVec<[MouseDownListener<V>; 2]>,
    pub mouse_up_listeners: SmallVec<[MouseUpListener<V>; 2]>,
    pub mouse_move_listeners: SmallVec<[MouseMoveListener<V>; 2]>,
    pub scroll_wheel_listeners: SmallVec<[ScrollWheelListener<V>; 2]>,
    pub key_listeners: SmallVec<[(TypeId, KeyListener<V>); 32]>,
    pub hover_style: StyleRefinement,
    pub group_hover: Option<GroupStyle>,
}

pub struct GroupStyle {
    pub group: SharedString,
    pub style: StyleRefinement,
}

#[derive(Default)]
pub struct GroupBounds(HashMap<SharedString, SmallVec<[Bounds<Pixels>; 1]>>);

impl GroupBounds {
    pub fn get(name: &SharedString, cx: &mut AppContext) -> Option<Bounds<Pixels>> {
        cx.default_global::<Self>()
            .0
            .get(name)
            .and_then(|bounds_stack| bounds_stack.last())
            .cloned()
    }

    pub fn push(name: SharedString, bounds: Bounds<Pixels>, cx: &mut AppContext) {
        cx.default_global::<Self>()
            .0
            .entry(name)
            .or_default()
            .push(bounds);
    }

    pub fn pop(name: &SharedString, cx: &mut AppContext) {
        cx.default_global::<GroupBounds>()
            .0
            .get_mut(name)
            .unwrap()
            .pop();
    }
}

pub fn group_bounds(name: &SharedString, cx: &mut AppContext) -> Option<Bounds<Pixels>> {
    cx.default_global::<GroupBounds>()
        .0
        .get(name)
        .and_then(|bounds_stack| bounds_stack.last().cloned())
}

impl<V> Default for StatelessInteractivity<V> {
    fn default() -> Self {
        Self {
            mouse_down_listeners: SmallVec::new(),
            mouse_up_listeners: SmallVec::new(),
            mouse_move_listeners: SmallVec::new(),
            scroll_wheel_listeners: SmallVec::new(),
            key_listeners: SmallVec::new(),
            hover_style: StyleRefinement::default(),
            group_hover: None,
        }
    }
}

impl<V> ElementInteractivity<V> for StatelessInteractivity<V>
where
    V: 'static + Send + Sync,
{
    fn as_stateful(&self) -> Option<&StatefulInteractivity<V>> {
        None
    }

    fn as_stateful_mut(&mut self) -> Option<&mut StatefulInteractivity<V>> {
        None
    }

    fn as_stateless(&self) -> &StatelessInteractivity<V> {
        self
    }

    fn as_stateless_mut(&mut self) -> &mut StatelessInteractivity<V> {
        self
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
