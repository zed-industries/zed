use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use gpui::{AnyView, DismissEvent, Entity, EntityId, FocusHandle, ManagedView, Subscription, Task};
use ui::{animation::DefaultAnimations, prelude::*};
use zed_actions::toast;

use crate::Workspace;

const DEFAULT_TOAST_DURATION: Duration = Duration::from_secs(10);
const MINIMUM_RESUME_DURATION: Duration = Duration::from_millis(800);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_workspace, _: &toast::RunAction, window, cx| {
            let workspace = cx.entity();
            let window = window.window_handle();
            cx.defer(move |cx| {
                let action = workspace
                    .read(cx)
                    .toast_layer
                    .read(cx)
                    .active_toast
                    .as_ref()
                    .and_then(|active_toast| active_toast.action.clone());

                if let Some(on_click) = action.and_then(|action| action.on_click) {
                    window
                        .update(cx, |_, window, cx| {
                            on_click(window, cx);
                        })
                        .ok();
                }
            });
        });
    })
    .detach();
}

pub trait ToastView: ManagedView {
    fn action(&self) -> Option<ToastAction>;
}

#[derive(Clone)]
pub struct ToastAction {
    pub id: ElementId,
    pub label: SharedString,
    pub on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl ToastAction {
    pub fn new(
        label: SharedString,
        on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    ) -> Self {
        let id = ElementId::Name(label.clone());

        Self {
            id,
            label,
            on_click,
        }
    }
}

trait ToastViewHandle {
    fn view(&self) -> AnyView;
}

impl<V: ToastView> ToastViewHandle for Entity<V> {
    fn view(&self) -> AnyView {
        self.clone().into()
    }
}

pub struct ActiveToast {
    id: EntityId,
    toast: Box<dyn ToastViewHandle>,
    action: Option<ToastAction>,
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

    pub fn toggle_toast<V>(&mut self, cx: &mut Context<Self>, new_toast: Entity<V>)
    where
        V: ToastView,
    {
        if let Some(active_toast) = &self.active_toast {
            let show_new = active_toast.id != new_toast.entity_id();
            self.hide_toast(cx);
            if !show_new {
                return;
            }
        }
        self.show_toast(new_toast, cx);
    }

    pub fn show_toast<V>(&mut self, new_toast: Entity<V>, cx: &mut Context<Self>)
    where
        V: ToastView,
    {
        let action = new_toast.read(cx).action();
        let focus_handle = cx.focus_handle();

        self.active_toast = Some(ActiveToast {
            _subscriptions: [cx.subscribe(&new_toast, |this, _, _: &DismissEvent, cx| {
                this.hide_toast(cx);
            })],
            id: new_toast.entity_id(),
            toast: Box::new(new_toast),
            action,
            focus_handle,
        });

        self.start_dismiss_timer(DEFAULT_TOAST_DURATION, cx);

        cx.notify();
    }

    pub fn hide_toast(&mut self, cx: &mut Context<Self>) {
        self.active_toast.take();
        cx.notify();
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
    pub fn start_dismiss_timer(&mut self, duration: Duration, cx: &mut Context<Self>) {
        self.clear_dismiss_timer(cx);

        let instant_started = std::time::Instant::now();
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(duration).await;

            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| this.hide_toast(cx)).ok();
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
    pub fn restart_dismiss_timer(&mut self, cx: &mut Context<Self>) {
        let Some(duration) = self.duration_remaining else {
            return;
        };
        self.start_dismiss_timer(duration, cx);
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

        div().absolute().size_full().bottom_0().left_0().child(
            v_flex()
                .id(("toast-layer-container", active_toast.id))
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
                        .on_hover(cx.listener(|this, hover_start, _window, cx| {
                            if *hover_start {
                                this.pause_dismiss_timer();
                            } else {
                                this.restart_dismiss_timer(cx);
                            }
                            cx.stop_propagation();
                        }))
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                        })
                        .child(active_toast.toast.view()),
                )
                .animate_in(AnimationDirection::FromBottom, true),
        )
    }
}
