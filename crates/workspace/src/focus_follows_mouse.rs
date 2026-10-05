use gpui::{
    AnyWindowHandle, AppContext as _, Context, FocusHandle, Focusable, Global,
    StatefulInteractiveElement, Task,
};

use crate::workspace_settings;

#[derive(Default)]
struct FfmState {
    // The window and element to be focused
    handles: Option<(AnyWindowHandle, FocusHandle)>,
    // The debounced task which will do the focusing
    _debounce_task: Option<Task<()>>,
}

impl Global for FfmState {}

pub trait FocusFollowsMouse<E: Focusable>: StatefulInteractiveElement {
    fn focus_follows_mouse(
        self,
        settings: workspace_settings::FocusFollowsMouse,
        cx: &Context<E>,
    ) -> Self {
        if settings.enabled {
            self.on_hover(cx.listener(move |this, enter, window, cx| {
                if *enter {
                    let window_handle = window.window_handle();
                    let focus_handle = this.focus_handle(cx);

                    let state = cx.try_global::<FfmState>();

                    // Only replace the target if the new handle doesn't contain the existing one.
                    // This ensures that hovering over a parent (e.g., Dock) doesn't override
                    // a more specific child target (e.g., a Pane inside the Dock).
                    let should_replace = state
                        .and_then(|s| s.handles.as_ref())
                        .map(|(_, existing)| !focus_handle.contains(existing, window))
                        .unwrap_or(true);

                    if !should_replace {
                        return;
                    }

                    let debounce_task = cx.spawn(async move |_this, cx| {
                        cx.background_executor().timer(settings.debounce).await;

                        cx.update(|cx| {
                            let state = cx.default_global::<FfmState>();
                            let Some((window, focus)) = state.handles.take() else {
                                return;
                            };

                            let _ = cx.update_window(window, move |_view, window, cx| {
                                window.focus(&focus, cx);
                            });
                        });
                    });

                    cx.set_global(FfmState {
                        handles: Some((window_handle, focus_handle)),
                        _debounce_task: Some(debounce_task),
                    });
                }
            }))
        } else {
            self
        }
    }
}

impl<E: Focusable, T: StatefulInteractiveElement> FocusFollowsMouse<E> for T {}
