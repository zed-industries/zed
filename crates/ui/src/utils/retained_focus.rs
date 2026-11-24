use gpui::{App, FocusHandle, Global, WeakFocusHandle};

#[derive(Default)]
struct RetainedFocusRegistry {
    handles: Vec<WeakFocusHandle>,
}

impl Global for RetainedFocusRegistry {}

pub trait RetainedFocusExt {
    /// Creates a focus handle that keeps focus when focus would move implicitly, e.g. with focus follows mouse.
    fn retained_focus_handle(&mut self) -> FocusHandle;
}

impl RetainedFocusExt for App {
    fn retained_focus_handle(&mut self) -> FocusHandle {
        let focus_handle = self.focus_handle();
        let registry = self.default_global::<RetainedFocusRegistry>();
        registry.handles.retain(|handle| handle.upgrade().is_some());
        registry.handles.push(focus_handle.downgrade());
        focus_handle
    }
}

/// Returns whether this handle was created with [`RetainedFocusExt::retained_focus_handle`].
pub fn retains_focus(focus_handle: &FocusHandle, cx: &App) -> bool {
    cx.try_global::<RetainedFocusRegistry>()
        .is_some_and(|registry| registry.handles.iter().any(|handle| handle == focus_handle))
}
