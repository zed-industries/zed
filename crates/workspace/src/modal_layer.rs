use gpui::{
    AnyView, App, DismissEvent, Entity, EventEmitter, FocusHandle, Global, ManagedView,
    MouseButton, Subscription, WeakFocusHandle,
};
use ui::prelude::*;

#[derive(Debug)]
pub enum DismissDecision {
    Dismiss(bool),
    Pending,
}

// A modal that hosts a picker forwards its own focus handle to the inner picker,
// so the modal layer recognizes a reopenable picker by focus identity rather than
// requiring each modal wrapper to implement its own opt-in.
#[derive(Default)]
struct ReopenablePickerRegistry {
    handles: Vec<WeakFocusHandle>,
}

impl Global for ReopenablePickerRegistry {}

pub fn register_reopenable_picker(focus_handle: &FocusHandle, cx: &mut App) {
    let registry = cx.default_global::<ReopenablePickerRegistry>();
    registry.handles.retain(|handle| handle.upgrade().is_some());
    if !registry.handles.iter().any(|handle| handle == focus_handle) {
        registry.handles.push(focus_handle.downgrade());
    }
}

pub fn deregister_reopenable_picker(focus_handle: &FocusHandle, cx: &mut App) {
    let registry = cx.default_global::<ReopenablePickerRegistry>();
    registry
        .handles
        .retain(|handle| handle.upgrade().is_some() && handle != focus_handle);
}

fn focus_handle_is_reopenable(focus_handle: &FocusHandle, cx: &App) -> bool {
    cx.try_global::<ReopenablePickerRegistry>()
        .is_some_and(|registry| {
            registry
                .handles
                .iter()
                .filter_map(|handle| handle.upgrade())
                .any(|handle| &handle == focus_handle)
        })
}

pub trait ModalView: ManagedView {
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

    fn render_bare(&self) -> bool {
        false
    }
}

trait ModalViewHandle {
    fn on_before_dismiss(&mut self, window: &mut Window, cx: &mut App) -> DismissDecision;
    fn view(&self) -> AnyView;
    fn focus_handle(&self, cx: &App) -> FocusHandle;
    fn subscribe_dismiss(&self, window: &mut Window, cx: &mut Context<ModalLayer>) -> Subscription;
    fn fade_out_background(&self, cx: &mut App) -> bool;
    fn render_bare(&self, cx: &mut App) -> bool;
}

impl<V: ModalView> ModalViewHandle for Entity<V> {
    fn on_before_dismiss(&mut self, window: &mut Window, cx: &mut App) -> DismissDecision {
        self.update(cx, |this, cx| this.on_before_dismiss(window, cx))
    }

    fn view(&self) -> AnyView {
        self.clone().into()
    }

    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.read(cx).focus_handle(cx)
    }

    fn subscribe_dismiss(&self, window: &mut Window, cx: &mut Context<ModalLayer>) -> Subscription {
        cx.subscribe_in(self, window, |this, _, _: &DismissEvent, window, cx| {
            this.hide_modal(window, cx);
        })
    }

    fn fade_out_background(&self, cx: &mut App) -> bool {
        self.read(cx).fade_out_background()
    }

    fn render_bare(&self, cx: &mut App) -> bool {
        self.read(cx).render_bare()
    }
}

pub struct ActiveModal {
    modal: Box<dyn ModalViewHandle>,
    _subscriptions: [Subscription; 2],
    previous_focus_handle: Option<FocusHandle>,
    focus_handle: FocusHandle,
}

pub struct ModalLayer {
    active_modal: Option<ActiveModal>,
    // Kept alive (hidden) rather than dropped on dismissal so `ReopenLastPicker`
    // can reveal it again with its exact prior state. Left intact across
    // non-reopenable modals (e.g. the command palette), which may trigger the reopen.
    stashed_modal: Option<Box<dyn ModalViewHandle>>,
    // Set when a reveal was requested while another modal was still active (e.g. the
    // which-key popup that is dismissed only after the reopen action fires). The reveal
    // then happens once that modal closes, so it doesn't matter whether the triggering
    // modal is dismissed before or after the action dispatches.
    reveal_stash_when_free: bool,
    dismiss_on_focus_lost: bool,
}

pub(crate) struct ModalOpenedEvent;

impl EventEmitter<ModalOpenedEvent> for ModalLayer {}

impl Default for ModalLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalLayer {
    pub fn new() -> Self {
        Self {
            active_modal: None,
            stashed_modal: None,
            reveal_stash_when_free: false,
            dismiss_on_focus_lost: false,
        }
    }

    /// Toggles a modal of type `V`. If a modal of the same type is currently active,
    /// it will be hidden. If a different modal is active, it will be replaced with the new one.
    /// If no modal is active, the new modal will be shown.
    ///
    /// If closing the current modal fails (e.g., due to `on_before_dismiss` returning
    /// `DismissDecision::Dismiss(false)` or `DismissDecision::Pending`), the new modal
    /// will not be shown.
    pub fn toggle_modal<V, B>(&mut self, window: &mut Window, cx: &mut Context<Self>, build_view: B)
    where
        V: ModalView,
        B: FnOnce(&mut Window, &mut Context<V>) -> V,
    {
        // Opening a modal explicitly supersedes any reveal that was waiting for the
        // layer to become free.
        self.reveal_stash_when_free = false;
        if let Some(active_modal) = &self.active_modal {
            let should_close = active_modal.modal.view().downcast::<V>().is_ok();
            let did_close = self.hide_modal(window, cx);
            if should_close || !did_close {
                return;
            }
        }
        let new_modal = cx.new(|cx| build_view(window, cx));
        self.show_modal(Box::new(new_modal), window, cx);
        cx.emit(ModalOpenedEvent);
    }

    /// Shows a modal and sets up subscriptions for dismiss events and focus tracking.
    /// The modal is automatically focused after being shown.
    fn show_modal(
        &mut self,
        modal: Box<dyn ModalViewHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let focus_handle = cx.focus_handle();
        let modal_focus_handle = modal.focus_handle(cx);
        let dismiss_subscription = modal.subscribe_dismiss(window, cx);
        self.active_modal = Some(ActiveModal {
            modal,
            _subscriptions: [
                dismiss_subscription,
                cx.on_focus_out(&focus_handle, window, |this, _event, window, cx| {
                    if this.dismiss_on_focus_lost {
                        this.hide_modal(window, cx);
                    }
                }),
            ],
            previous_focus_handle: window.focused(cx),
            focus_handle,
        });
        cx.defer_in(window, move |_, window, cx| {
            window.focus(&modal_focus_handle, cx);
        });
        cx.notify();
    }

    /// Reveals the most recently stashed reopenable modal, if any. Returns whether
    /// a modal was revealed.
    pub fn reveal_stashed_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.stashed_modal.is_none() {
            return false;
        }
        if self.active_modal.is_some() {
            // Another modal is still closing (e.g. the which-key popup that dismisses
            // only after this action fires). Reveal once it is gone; see `hide_modal`.
            self.reveal_stash_when_free = true;
            return false;
        }
        let Some(modal) = self.stashed_modal.take() else {
            return false;
        };
        self.show_modal(modal, window, cx);
        cx.emit(ModalOpenedEvent);
        true
    }

    /// Attempts to hide the currently active modal.
    ///
    /// The modal's `on_before_dismiss` method is called to determine if dismissal should proceed.
    /// If dismissal is allowed, the modal is removed and focus is restored to the previously
    /// focused element.
    ///
    /// Returns `true` if the modal was successfully hidden, `false` otherwise.
    pub fn hide_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(active_modal) = self.active_modal.as_mut() else {
            self.dismiss_on_focus_lost = false;
            return false;
        };

        match active_modal.modal.on_before_dismiss(window, cx) {
            DismissDecision::Dismiss(should_dismiss) => {
                if !should_dismiss {
                    self.dismiss_on_focus_lost = !should_dismiss;
                    return false;
                }
            }
            DismissDecision::Pending => {
                self.dismiss_on_focus_lost = false;
                return false;
            }
        }

        if let Some(active_modal) = self.active_modal.take() {
            let reopenable = focus_handle_is_reopenable(&active_modal.modal.focus_handle(cx), cx);
            if let Some(previous_focus) = active_modal.previous_focus_handle
                && active_modal.focus_handle.contains_focused(window, cx)
            {
                previous_focus.focus(window, cx);
            }
            if reopenable {
                self.stashed_modal = Some(active_modal.modal);
            }
            cx.notify();
        }
        self.dismiss_on_focus_lost = false;
        // A reveal was requested while this modal was still active; now that the layer
        // is free, honor it.
        if self.reveal_stash_when_free && self.active_modal.is_none() {
            self.reveal_stash_when_free = false;
            self.reveal_stashed_modal(window, cx);
        }
        true
    }

    /// Returns the currently active modal if it is of type `V`.
    pub fn active_modal<V>(&self) -> Option<Entity<V>>
    where
        V: 'static,
    {
        let active_modal = self.active_modal.as_ref()?;
        active_modal.modal.view().downcast::<V>().ok()
    }

    pub fn has_active_modal(&self) -> bool {
        self.active_modal.is_some()
    }
}

impl Render for ModalLayer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active_modal) = &self.active_modal else {
            return div().into_any_element();
        };

        if active_modal.modal.render_bare(cx) {
            return active_modal.modal.view().into_any_element();
        }

        div()
            .absolute()
            .size_full()
            .inset_0()
            .occlude()
            .when(active_modal.modal.fade_out_background(cx), |this| {
                let mut background = cx.theme().colors().elevated_surface_background;
                background.fade_out(0.2);
                this.bg(background)
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.hide_modal(window, cx);
                }),
            )
            .child(
                v_flex()
                    .h(px(0.0))
                    .top_20()
                    .items_center()
                    .track_focus(&active_modal.focus_handle)
                    .child(
                        h_flex()
                            .occlude()
                            .child(active_modal.modal.view())
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.stop_propagation();
                            }),
                    ),
            )
            .into_any_element()
    }
}
