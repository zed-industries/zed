use gpui::{App, Bounds, DisplayId, Pixels, Size, WindowBounds, WindowHandle};

/// Return the DisplayId of the monitor that contains the center of the given window handle.
pub fn display_id_for_window_center<T: 'static>(
    handle: &WindowHandle<T>,
    cx: &mut App,
) -> Option<DisplayId> {
    handle
        .update(cx, |_, window, cx| {
            let center = window.bounds().center();
            cx.displays()
                .into_iter()
                .find(|display| display.bounds().contains(&center))
                .map(|display| display.id())
        })
        .ok()
        .flatten()
}

/// Compute centered window bounds on the display containing the given window handle.
pub fn centered_bounds_for_window_display<T: 'static>(
    handle: &WindowHandle<T>,
    size: Size<Pixels>,
    cx: &mut App,
) -> WindowBounds {
    let display_id = display_id_for_window_center(handle, cx);
    WindowBounds::Windowed(Bounds::centered(display_id, size, cx))
}

/// Compute centered window bounds on a given display id (or primary if None).
pub fn centered_bounds_for_display_id(
    display_id: Option<DisplayId>,
    size: Size<Pixels>,
    cx: &mut App,
) -> WindowBounds {
    WindowBounds::Windowed(Bounds::centered(display_id, size, cx))
}

/// Return the DisplayId for the currently active window (if any).
pub fn active_display_id(cx: &mut App) -> Option<DisplayId> {
    cx.active_window().and_then(|handle| {
        handle
            .update(cx, |_, window, cx| {
                let center = window.bounds().center();
                cx.displays()
                    .into_iter()
                    .find(|display| display.bounds().contains(&center))
                    .map(|display| display.id())
            })
            .ok()
            .flatten()
    })
}

/// Compute centered window bounds on the display of the currently active window (if any).
pub fn centered_bounds_for_active_display(size: Size<Pixels>, cx: &mut App) -> WindowBounds {
    WindowBounds::Windowed(Bounds::centered(active_display_id(cx), size, cx))
}
