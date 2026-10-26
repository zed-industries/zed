use gpui::{App, FocusHandle, Global, WeakFocusHandle};

#[derive(Default)]
struct RetainedFocusRegistry {
    handles: Vec<WeakFocusHandle>,
}

impl Global for RetainedFocusRegistry {}

/// Marks this handle to keep focus when focus would move implicitly, e.g. with focus follows mouse.
pub fn register_retained_focus(focus_handle: &FocusHandle, cx: &mut App) {
    let registry = cx.default_global::<RetainedFocusRegistry>();
    registry.handles.retain(|handle| handle.upgrade().is_some());
    if !registry.handles.iter().any(|handle| handle == focus_handle) {
        registry.handles.push(focus_handle.downgrade());
    }
}

/// Returns whether this handle was registered with [`register_retained_focus`].
pub fn retains_focus(focus_handle: &FocusHandle, cx: &App) -> bool {
    cx.try_global::<RetainedFocusRegistry>()
        .is_some_and(|registry| registry.handles.iter().any(|handle| handle == focus_handle))
}
