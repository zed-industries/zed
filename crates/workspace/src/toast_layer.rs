use std::time::Duration;

use gpui::{AnyView, DismissEvent, Entity, FocusHandle, ManagedView, Subscription, Task};
use ui::prelude::*;

const DEFAULT_TOAST_DURATION: Duration = Duration::from_millis(3000);

pub trait ToastView: ManagedView {}

trait ToastViewHandle {
    fn view(&self) -> AnyView;
}

impl<V: ToastView> ToastViewHandle for Entity<V> {
    fn view(&self) -> AnyView {
        self.clone().into()
    }
}

pub struct ActiveToast {
    toast: Box<dyn ToastViewHandle>,
    _subscriptions: [Subscription; 1],
    focus_handle: FocusHandle,
}

pub struct ToastLayer {
    active_toast: Option<ActiveToast>,
    dismiss_timer: Option<Task<()>>,
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
            dismiss_timer: None,
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

    pub fn show_toast<V>(
        &mut self,
        new_toast: Entity<V>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) where
        V: ToastView,
    {
        let focus_handle = cx.focus_handle();

        self.active_toast = Some(ActiveToast {
            toast: Box::new(new_toast.clone()),
            _subscriptions: [cx.subscribe_in(
                &new_toast,
                window,
                |this, _, _: &DismissEvent, window, cx| {
                    this.hide_toast(window, cx);
                },
            )],
            focus_handle,
        });

        self.start_dismiss_timer(DEFAULT_TOAST_DURATION, window, cx);

        cx.notify();
    }

    pub fn hide_toast(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> bool {
        cx.notify();

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

    /// Starts a timer to automatically dismiss the toast after the specified duration
    pub fn start_dismiss_timer(
        &mut self,
        duration: Duration,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear_dismiss_timer(cx);

        let task = cx.spawn(|this, mut cx| async move {
            cx.background_executor().timer(duration).await;

            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, cx| {
                    this.active_toast.take();
                    cx.notify();
                })
                .ok();
            }
        });

        self.dismiss_timer = Some(task);
        cx.notify();
    }

    /// Restarts the dismiss timer with a new duration
    pub fn restart_dismiss_timer(
        &mut self,
        duration: Duration,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_dismiss_timer(duration, window, cx);
        cx.notify();
    }

    /// Clears the dismiss timer if one exists
    pub fn clear_dismiss_timer(&mut self, cx: &mut Context<Self>) {
        self.dismiss_timer.take();
        cx.notify();
    }
}

impl Render for ToastLayer {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active_toast) = &self.active_toast else {
            return div();
        };

        div().absolute().size_full().bottom_0().left_0().child(
            v_flex()
                .absolute()
                .w_full()
                .bottom_10()
                .flex()
                .flex_col()
                .items_center()
                .track_focus(&active_toast.focus_handle)
                .child(h_flex().occlude().child(active_toast.toast.view())),
        )
    }
}
