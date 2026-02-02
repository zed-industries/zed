use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyView, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable as _, ManagedView, MouseButton, Subscription, Task, ease_out_cubic,
};
use settings::should_reduce_motion;
use ui::prelude::*;

const MODAL_OPEN_DURATION: Duration = Duration::from_millis(150);
const MODAL_CLOSE_DURATION: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub enum DismissDecision {
    Dismiss(bool),
    Pending,
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

struct ClosingModal {
    modal_view: AnyView,
    fade_out_background: bool,
}

pub struct ModalLayer {
    active_modal: Option<ActiveModal>,
    dismiss_on_focus_lost: bool,
    closing_modal: Option<ClosingModal>,
    animation_generation: usize,
    _close_task: Option<Task<()>>,
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
            dismiss_on_focus_lost: false,
            closing_modal: None,
            animation_generation: 0,
            _close_task: None,
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
        if let Some(active_modal) = &self.active_modal {
            let should_close = active_modal.modal.view().downcast::<V>().is_ok();
            let did_close = self.hide_modal(window, cx);
            if should_close || !did_close {
                return;
            }
        }
        self.cancel_close_animation();
        let new_modal = cx.new(|cx| build_view(window, cx));
        self.show_modal(new_modal, window, cx);
        cx.emit(ModalOpenedEvent);
    }

    fn cancel_close_animation(&mut self) {
        self.closing_modal = None;
        self._close_task = None;
    }

    /// Shows a modal and sets up subscriptions for dismiss events and focus tracking.
    /// The modal is automatically focused after being shown.
    fn show_modal<V>(&mut self, new_modal: Entity<V>, window: &mut Window, cx: &mut Context<Self>)
    where
        V: ModalView,
    {
        self.cancel_close_animation();
        self.animation_generation += 1;

        let focus_handle = cx.focus_handle();
        self.active_modal = Some(ActiveModal {
            modal: Box::new(new_modal.clone()),
            _subscriptions: [
                cx.subscribe_in(
                    &new_modal,
                    window,
                    |this, _, _: &DismissEvent, window, cx| {
                        this.hide_modal(window, cx);
                    },
                ),
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
            window.focus(&new_modal.focus_handle(cx), cx);
        });
        cx.notify();
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
            DismissDecision::Dismiss(false) => {
                self.dismiss_on_focus_lost = true;
                return false;
            }
            DismissDecision::Dismiss(true) => {}
            DismissDecision::Pending => {
                self.dismiss_on_focus_lost = false;
                return false;
            }
        }

        if let Some(active_modal) = self.active_modal.take() {
            if let Some(previous_focus) = &active_modal.previous_focus_handle {
                if active_modal.focus_handle.contains_focused(window, cx) {
                    previous_focus.focus(window, cx);
                }
            }

            let fade_out_background = active_modal.modal.fade_out_background(cx);
            let render_bare = active_modal.modal.render_bare(cx);

            if !render_bare && !should_reduce_motion(cx) {
                self.closing_modal = Some(ClosingModal {
                    modal_view: active_modal.modal.view(),
                    fade_out_background,
                });
                self.animation_generation += 1;
                let generation = self.animation_generation;

                self._close_task = Some(cx.spawn_in(window, async move |this, cx| {
                    cx.background_executor().timer(MODAL_CLOSE_DURATION).await;
                    this.update(cx, |this, cx| {
                        if this.animation_generation == generation {
                            this.closing_modal = None;
                            this._close_task = None;
                            cx.notify();
                        }
                    }).ok();
                }));
            }

            cx.notify();
        }
        self.dismiss_on_focus_lost = false;
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{div, Empty, TestAppContext, UpdateGlobal};
    use settings::SettingsStore;

    struct TestModal {
        focus_handle: FocusHandle,
    }

    impl TestModal {
        fn new(cx: &mut gpui::Context<Self>) -> Self {
            Self {
                focus_handle: cx.focus_handle(),
            }
        }
    }

    impl Render for TestModal {
        fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
            div().track_focus(&self.focus_handle(cx))
        }
    }

    impl gpui::Focusable for TestModal {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl EventEmitter<DismissEvent> for TestModal {}
    impl ModalView for TestModal {}

    struct TestModalB {
        focus_handle: FocusHandle,
    }

    impl TestModalB {
        fn new(cx: &mut gpui::Context<Self>) -> Self {
            Self {
                focus_handle: cx.focus_handle(),
            }
        }
    }

    impl Render for TestModalB {
        fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
            div().track_focus(&self.focus_handle(cx))
        }
    }

    impl gpui::Focusable for TestModalB {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl EventEmitter<DismissEvent> for TestModalB {}
    impl ModalView for TestModalB {}

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    #[gpui::test]
    async fn test_toggle_modal_opens(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            assert!(layer.active_modal.is_some());
        });
    }

    #[gpui::test]
    async fn test_toggle_modal_closes_same_type(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            assert!(layer.active_modal.is_some());
            layer.toggle_modal::<TestModal, _>(window, cx, |_window, cx| TestModal::new(cx));
            assert!(layer.active_modal.is_none());
        });
    }

    #[gpui::test]
    async fn test_toggle_modal_replaces_different_type(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            assert!(layer.active_modal::<TestModal>().is_some());

            layer.toggle_modal(window, cx, |_window, cx| TestModalB::new(cx));
            assert!(layer.active_modal::<TestModalB>().is_some());
            assert!(layer.active_modal::<TestModal>().is_none());
        });
    }

    #[gpui::test]
    async fn test_hide_modal_starts_close_animation(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            layer.hide_modal(window, cx);
            assert!(layer.active_modal.is_none());
            assert!(layer.closing_modal.is_some());
        });
    }

    #[gpui::test]
    async fn test_hide_modal_skips_animation_with_reduce_motion(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.workspace.reduce_motion =
                        Some(settings::ReduceMotion::On);
                });
            });
        });

        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            layer.hide_modal(window, cx);
            assert!(layer.active_modal.is_none());
            assert!(layer.closing_modal.is_none());
        });
    }

    #[gpui::test]
    async fn test_close_animation_completes(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            layer.hide_modal(window, cx);
            assert!(layer.closing_modal.is_some());
        });

        cx.executor().advance_clock(MODAL_CLOSE_DURATION + std::time::Duration::from_millis(50));
        cx.executor().run_until_parked();

        layer.update_in(cx, |layer, _window, _cx| {
            assert!(layer.closing_modal.is_none());
        });
    }

    #[gpui::test]
    async fn test_open_during_close_cancels_animation(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            layer.hide_modal(window, cx);
            assert!(layer.closing_modal.is_some());

            layer.toggle_modal(window, cx, |_window, cx| TestModalB::new(cx));
            assert!(layer.closing_modal.is_none());
            assert!(layer.active_modal.is_some());
        });
    }

    #[gpui::test]
    async fn test_hide_empty_modal_layer(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            let result = layer.hide_modal(window, cx);
            assert!(!result);
        });
    }

    #[gpui::test]
    async fn test_animation_generation_increments(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        let gen0 = layer.read_with(cx, |layer, _| layer.animation_generation);

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
        });
        let gen1 = layer.read_with(cx, |layer, _| layer.animation_generation);
        assert!(gen1 > gen0);

        layer.update_in(cx, |layer, window, cx| {
            layer.hide_modal(window, cx);
        });
        let gen2 = layer.read_with(cx, |layer, _| layer.animation_generation);
        assert!(gen2 > gen1);
    }

    #[gpui::test]
    async fn test_has_active_modal(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, _window, _cx| {
            assert!(!layer.has_active_modal());
        });

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            assert!(layer.has_active_modal());
        });
    }

    #[gpui::test]
    async fn test_active_modal_returns_typed(cx: &mut TestAppContext) {
        init_test(cx);
        let (_view, cx) = cx.add_window_view(|_window, _cx| Empty);
        let layer = cx.new(|_cx| ModalLayer::new());

        layer.update_in(cx, |layer, window, cx| {
            layer.toggle_modal(window, cx, |_window, cx| TestModal::new(cx));
            assert!(layer.active_modal::<TestModal>().is_some());
            assert!(layer.active_modal::<TestModalB>().is_none());
        });
    }
}

impl Render for ModalLayer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let generation = self.animation_generation;

        let (modal_view, fade_out_background, focus_handle, is_closing) =
            if let Some(active_modal) = &self.active_modal {
                if active_modal.modal.render_bare(cx) {
                    return active_modal.modal.view().into_any_element();
                }
                (
                    active_modal.modal.view(),
                    active_modal.modal.fade_out_background(cx),
                    Some(active_modal.focus_handle.clone()),
                    false,
                )
            } else if let Some(closing_modal) = &self.closing_modal {
                (
                    closing_modal.modal_view.clone(),
                    closing_modal.fade_out_background,
                    None,
                    true,
                )
            } else {
                return div().into_any_element();
            };

        let reduce_motion = should_reduce_motion(cx);

        let modal_content = h_flex()
            .occlude()
            .child(modal_view)
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            });

        let animated_content = if reduce_motion {
            modal_content.into_any_element()
        } else {
            let duration = if is_closing {
                MODAL_CLOSE_DURATION
            } else {
                MODAL_OPEN_DURATION
            };
            modal_content
                .with_animation(
                    ("modal-anim", generation as u64),
                    Animation::new(duration).with_easing(ease_out_cubic),
                    move |this, delta| {
                        let progress = if is_closing { 1.0 - delta } else { delta };
                        let slide = -6.0 * (1.0 - progress);
                        this.opacity(progress).top(px(slide))
                    },
                )
                .into_any_element()
        };

        div()
            .absolute()
            .size_full()
            .inset_0()
            .occlude()
            .when(fade_out_background, |this| {
                let mut background = cx.theme().colors().elevated_surface_background;
                background.fade_out(0.2);
                this.bg(background)
            })
            .when(!is_closing, |this| {
                this.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        this.hide_modal(window, cx);
                    }),
                )
            })
            .child(
                v_flex()
                    .h(px(0.0))
                    .top_20()
                    .items_center()
                    .when_some(focus_handle, |this, handle| this.track_focus(&handle))
                    .child(animated_content),
            )
            .into_any_element()
    }
}
