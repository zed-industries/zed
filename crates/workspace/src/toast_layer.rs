use std::time::{Duration, Instant};

use gpui::{AnyView, DismissEvent, Entity, FocusHandle, ManagedView, Subscription, Task};
use ui::{animation::DefaultAnimations, prelude::*};

const DEFAULT_TOAST_DURATION: Duration = Duration::from_millis(2400);
const MINIMUM_RESUME_DURATION: Duration = Duration::from_millis(800);

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

struct DismissTimer {
    instant_started: Instant,
    _task: Task<()>,
}

pub struct ToastLayer {
    active_toast: Option<ActiveToast>,
    duration_remaining: Option<Duration>,
    dismiss_timer: Option<DismissTimer>,
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
            duration_remaining: None,
            dismiss_timer: None,
        }
    }

    pub fn toggle_toast<V>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        new_toast: Entity<V>,
    ) where
        V: ToastView,
    {
        if let Some(active_toast) = &self.active_toast {
            let is_close = active_toast.toast.view().downcast::<V>().is_ok();
            let did_close = self.hide_toast(window, cx);
            if is_close || !did_close {
                return;
            }
        }
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

    fn pause_dismiss_timer(&mut self) {
        let Some(dismiss_timer) = self.dismiss_timer.take() else {
            return;
        };
        let Some(duration_remaining) = self.duration_remaining.as_mut() else {
            return;
        };
        *duration_remaining =
            duration_remaining.saturating_sub(dismiss_timer.instant_started.elapsed());
        if *duration_remaining < MINIMUM_RESUME_DURATION {
            *duration_remaining = MINIMUM_RESUME_DURATION;
        }
    }

    /// Starts a timer to automatically dismiss the toast after the specified duration
    pub fn start_dismiss_timer(
        &mut self,
        duration: Duration,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear_dismiss_timer(cx);

        let instant_started = std::time::Instant::now();
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

        self.duration_remaining = Some(duration);
        self.dismiss_timer = Some(DismissTimer {
            instant_started,
            _task: task,
        });
        cx.notify();
    }

    /// Restarts the dismiss timer with a new duration
    pub fn restart_dismiss_timer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(duration) = self.duration_remaining else {
            return;
        };
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active_toast) = &self.active_toast else {
            return div();
        };
        let handle = cx.weak_entity();

        div().absolute().size_full().bottom_0().left_0().child(
            v_flex()
                .id("toast-layer-container")
                .absolute()
                .w_full()
                .bottom(px(0.))
                .flex()
                .flex_col()
                .items_center()
                .track_focus(&active_toast.focus_handle)
                .child(
                    h_flex()
                        .id("active-toast-container")
                        .occlude()
                        .on_hover(move |hover_start, window, cx| {
                            let Some(this) = handle.upgrade() else {
                                return;
                            };
                            if *hover_start {
                                this.update(cx, |this, _| this.pause_dismiss_timer());
                            } else {
                                this.update(cx, |this, cx| this.restart_dismiss_timer(window, cx));
                            }
                            cx.stop_propagation();
                        })
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                        })
                        .child(active_toast.toast.view()),
                )
                .animate_in(AnimationDirection::FromBottom, true),
        )
    }
}
