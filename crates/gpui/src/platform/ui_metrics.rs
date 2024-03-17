use crate::{Pixels, WindowContext};

/// Gets the title bar offset from the top of the window as defined by the Windows OS
#[cfg(target_os = "windows")]
pub fn windows_title_bar_top_offset(cx: &WindowContext) -> Pixels {
    use crate::try_get_window_inner;

    let inner = try_get_window_inner(cx.get_raw_handle()).expect("WindowsWindowInner");
    inner.title_bar_top_offset()
}

/// Gets the title bar height as defined by the Windows OS
#[cfg(target_os = "windows")]
pub fn windows_title_bar_height(cx: &WindowContext) -> Pixels {
    use crate::try_get_window_inner;

    let inner = try_get_window_inner(cx.get_raw_handle()).expect("WindowsWindowInner");
    inner.title_bar_height()
}

/// Gets the caption button width as defined by the Windows OS
#[cfg(target_os = "windows")]
pub fn windows_caption_button_width(cx: &WindowContext) -> Pixels {
    use crate::try_get_window_inner;

    let inner = try_get_window_inner(cx.get_raw_handle()).expect("WindowsWindowInner");
    inner.caption_button_width()
}

/// Windows title bar top offset emulation used for testing
#[cfg(not(target_os = "windows"))]
pub fn windows_title_bar_top_offset(_cx: &WindowContext) -> Pixels {
    crate::px(0.)
}

/// Windows title bar height emulation used for testing
#[cfg(not(target_os = "windows"))]
pub fn windows_title_bar_height(cx: &WindowContext) -> Pixels {
    crate::px(32.0 * cx.scale_factor())
}

/// Windows caption button width emulation used for testing
#[cfg(not(target_os = "windows"))]
pub fn windows_caption_button_width(cx: &WindowContext) -> Pixels {
    crate::px(36.0 * cx.scale_factor())
}
