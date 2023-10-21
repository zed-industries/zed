use crate::{
    Bounds, DispatchPhase, Element, FocusEvent, FocusHandle, MouseDownEvent, Pixels, Style,
    StyleRefinement, ViewContext, WindowContext,
};
use refineable::Refineable;
use smallvec::SmallVec;
use std::sync::Arc;

pub type FocusListeners<V> = SmallVec<[FocusListener<V>; 2]>;

pub type FocusListener<V> =
    Arc<dyn Fn(&mut V, &FocusHandle, &FocusEvent, &mut ViewContext<V>) + Send + Sync + 'static>;

pub trait Focusable: Element {
    fn focus_listeners(&mut self) -> &mut FocusListeners<Self::ViewState>;
    fn set_focus_style(&mut self, style: StyleRefinement);
    fn set_focus_in_style(&mut self, style: StyleRefinement);
    fn set_in_focus_style(&mut self, style: StyleRefinement);

    fn focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_focus_style(f(StyleRefinement::default()));
        self
    }

    fn focus_in(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_focus_in_style(f(StyleRefinement::default()));
        self
    }

    fn in_focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_in_focus_style(f(StyleRefinement::default()));
        self
    }

    fn on_focus(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Arc::new(move |view, focus_handle, event, cx| {
                if event.focused.as_ref() == Some(focus_handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_blur(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Arc::new(move |view, focus_handle, event, cx| {
                if event.blurred.as_ref() == Some(focus_handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_in(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Arc::new(move |view, focus_handle, event, cx| {
                let descendant_blurred = event
                    .blurred
                    .as_ref()
                    .map_or(false, |blurred| focus_handle.contains(blurred, cx));
                let descendant_focused = event
                    .focused
                    .as_ref()
                    .map_or(false, |focused| focus_handle.contains(focused, cx));

                if !descendant_blurred && descendant_focused {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_out(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Arc::new(move |view, focus_handle, event, cx| {
                let descendant_blurred = event
                    .blurred
                    .as_ref()
                    .map_or(false, |blurred| focus_handle.contains(blurred, cx));
                let descendant_focused = event
                    .focused
                    .as_ref()
                    .map_or(false, |focused| focus_handle.contains(focused, cx));
                if descendant_blurred && !descendant_focused {
                    listener(view, event, cx)
                }
            }));
        self
    }
}

pub trait ElementFocus<V: 'static + Send + Sync>: 'static + Send + Sync {
    fn as_focusable(&self) -> Option<&FocusEnabled<V>>;
    fn as_focusable_mut(&mut self) -> Option<&mut FocusEnabled<V>>;

    fn initialize<R>(
        &mut self,
        focus_handle: Option<FocusHandle>,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(Option<FocusHandle>, &mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(focusable) = self.as_focusable_mut() {
            let focus_handle = focusable
                .focus_handle
                .get_or_insert_with(|| focus_handle.unwrap_or_else(|| cx.focus_handle()))
                .clone();
            for listener in focusable.focus_listeners.iter().cloned() {
                let focus_handle = focus_handle.clone();
                cx.on_focus_changed(move |view, event, cx| {
                    listener(view, &focus_handle, event, cx)
                });
            }
            cx.with_focus(focus_handle.clone(), |cx| f(Some(focus_handle), cx))
        } else {
            f(None, cx)
        }
    }

    fn refine_style(&self, style: &mut Style, cx: &WindowContext) {
        if let Some(focusable) = self.as_focusable() {
            let focus_handle = focusable
                .focus_handle
                .as_ref()
                .expect("must call initialize before refine_style");
            if focus_handle.contains_focused(cx) {
                style.refine(&focusable.focus_in_style);
            }

            if focus_handle.within_focused(cx) {
                style.refine(&focusable.in_focus_style);
            }

            if focus_handle.is_focused(cx) {
                style.refine(&focusable.focus_style);
            }
        }
    }

    fn paint(&self, bounds: Bounds<Pixels>, cx: &mut WindowContext) {
        if let Some(focusable) = self.as_focusable() {
            let focus_handle = focusable
                .focus_handle
                .clone()
                .expect("must call initialize before paint");
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

pub struct FocusEnabled<V: 'static + Send + Sync> {
    pub focus_handle: Option<FocusHandle>,
    pub focus_listeners: FocusListeners<V>,
    pub focus_style: StyleRefinement,
    pub focus_in_style: StyleRefinement,
    pub in_focus_style: StyleRefinement,
}

impl<V> FocusEnabled<V>
where
    V: 'static + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            focus_handle: None,
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }

    pub fn tracked(handle: &FocusHandle) -> Self {
        Self {
            focus_handle: Some(handle.clone()),
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }
}

impl<V> ElementFocus<V> for FocusEnabled<V>
where
    V: 'static + Send + Sync,
{
    fn as_focusable(&self) -> Option<&FocusEnabled<V>> {
        Some(self)
    }

    fn as_focusable_mut(&mut self) -> Option<&mut FocusEnabled<V>> {
        Some(self)
    }
}

impl<V> From<FocusHandle> for FocusEnabled<V>
where
    V: 'static + Send + Sync,
{
    fn from(value: FocusHandle) -> Self {
        Self {
            focus_handle: Some(value),
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }
}

pub struct FocusDisabled;

impl<V> ElementFocus<V> for FocusDisabled
where
    V: 'static + Send + Sync,
{
    fn as_focusable(&self) -> Option<&FocusEnabled<V>> {
        None
    }

    fn as_focusable_mut(&mut self) -> Option<&mut FocusEnabled<V>> {
        None
    }
}
