use gpui::{AnyView, DismissEvent, Entity, FocusHandle, Focusable as _, ManagedView, Subscription};
use ui::prelude::*;

pub enum DismissDecision {
    Dismiss(bool),
    Pending,
}

pub trait ToastView: ManagedView {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> DismissDecision {
        DismissDecision::Dismiss(true)
    }

    fn fade_out_background(&self) -> bool {
        false
    }
}

trait ToastViewHandle {
    fn on_before_dismiss(&mut self, window: &mut Window, cx: &mut App) -> DismissDecision;
    fn view(&self) -> AnyView;
    fn fade_out_background(&self, cx: &mut App) -> bool;
}

impl<V: ToastView> ToastViewHandle for Entity<V> {
    fn on_before_dismiss(&mut self, window: &mut Window, cx: &mut App) -> DismissDecision {
        self.update(cx, |this, cx| this.on_before_dismiss(window, cx))
    }

    fn view(&self) -> AnyView {
        self.clone().into()
    }

    fn fade_out_background(&self, cx: &mut App) -> bool {
        self.read(cx).fade_out_background()
    }
}

pub struct ActiveToast {
    toast: Box<dyn ToastViewHandle>,
    _subscriptions: [Subscription; 2],
    previous_focus_handle: Option<FocusHandle>,
    focus_handle: FocusHandle,
}

pub struct ToastLayer {
    active_toast: Option<ActiveToast>,
    dismiss_on_focus_lost: bool,
}

impl Default for ToastLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastLayer {
    pub fn new() -> Self {
        Self {
            active_toast: None,
            dismiss_on_focus_lost: false,
        }
    }

    pub fn toggle_toast<V, B>(&mut self, window: &mut Window, cx: &mut Context<Self>, build_view: B)
    where
        V: ToastView,
        B: FnOnce(&mut Window, &mut Context<V>) -> V,
    {
        if let Some(active_toast) = &self.active_toast {
            let is_close = active_toast.toast.view().downcast::<V>().is_ok();
            let did_close = self.hide_toast(window, cx);
            if is_close || !did_close {
                return;
            }
        }
        let new_toast = cx.new(|cx| build_view(window, cx));
        self.show_toast(new_toast, window, cx);
    }

    fn show_toast<V>(&mut self, new_toast: Entity<V>, window: &mut Window, cx: &mut Context<Self>)
    where
        V: ToastView,
    {
        let focus_handle = cx.focus_handle();
        self.active_toast = Some(ActiveToast {
            toast: Box::new(new_toast.clone()),
            _subscriptions: [
                cx.subscribe_in(
                    &new_toast,
                    window,
                    |this, _, _: &DismissEvent, window, cx| {
                        this.hide_toast(window, cx);
                    },
                ),
                cx.on_focus_out(&focus_handle, window, |this, _event, window, cx| {
                    if this.dismiss_on_focus_lost {
                        this.hide_toast(window, cx);
                    }
                }),
            ],
            previous_focus_handle: window.focused(cx),
            focus_handle,
        });
        cx.defer_in(window, move |_, window, cx| {
            window.focus(&new_toast.focus_handle(cx));
        });
        cx.notify();
    }

    pub fn hide_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(active_toast) = self.active_toast.as_mut() else {
            self.dismiss_on_focus_lost = false;
            return false;
        };

        match active_toast.toast.on_before_dismiss(window, cx) {
            DismissDecision::Dismiss(dismiss) => {
                self.dismiss_on_focus_lost = !dismiss;
                if !dismiss {
                    return false;
                }
            }
            DismissDecision::Pending => {
                self.dismiss_on_focus_lost = false;
                return false;
            }
        }

        if let Some(active_toast) = self.active_toast.take() {
            if let Some(previous_focus) = active_toast.previous_focus_handle {
                if active_toast.focus_handle.contains_focused(window, cx) {
                    previous_focus.focus(window);
                }
            }
            cx.notify();
        }
        true
    }

    pub fn active_toast<V>(&self) -> Option<Entity<V>>
    where
        V: 'static,
    {
        let active_toast = self.active_toast.as_ref()?;
        active_toast.toast.view().downcast::<V>().ok()
    }

    pub fn has_active_toast(&self) -> bool {
        self.active_toast.is_some()
    }
}

impl Render for ToastLayer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active_toast) = &self.active_toast else {
            return div();
        };

        div()
            .absolute()
            .size_full()
            .top_0()
            .left_0()
            .when(active_toast.toast.fade_out_background(cx), |el| {
                let mut background = cx.theme().colors().elevated_surface_background;
                background.fade_out(0.2);
                el.bg(background)
                    .occlude()
                    .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                        this.hide_toast(window, cx);
                    }))
            })
            .child(
                v_flex()
                    .h(px(0.0))
                    .top_20()
                    .flex()
                    .flex_col()
                    .items_center()
                    .track_focus(&active_toast.focus_handle)
                    .child(h_flex().occlude().child(active_toast.toast.view())),
            )
    }
}
