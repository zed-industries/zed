//! User interface interactions (notifications, quick-pick, input-box, status bar, clipboard).

pub use crate::wit::zed::extension::ui::{NotificationLevel, QuickPickItem};

/// Shows a notification toast with the given level and message.
pub fn notify(level: NotificationLevel, message: &str) {
    crate::wit::zed::extension::ui::notify(level, message);
}

/// Requests the host to display a quick-pick modal. The user's selection will be delivered via `ui_response`.
pub fn quick_pick(
    request_id: u64,
    items: &[QuickPickItem],
    placeholder: Option<&str>,
) -> Result<(), String> {
    crate::wit::zed::extension::ui::quick_pick(request_id, items, placeholder)
}

/// Requests the host to display a modal text input box. The user's response will be delivered via `ui_response`.
pub fn input_box(request_id: u64, prompt: &str, default_value: Option<&str>) -> Result<(), String> {
    crate::wit::zed::extension::ui::input_box(request_id, prompt, default_value)
}

/// Sets or removes an extension status indicator in Zed's status bar.
pub fn set_status(id: &str, text: Option<&str>) -> Result<(), String> {
    crate::wit::zed::extension::ui::set_status(id, text)
}

/// Writes text to the system clipboard.
pub fn write_clipboard(text: &str) -> Result<(), String> {
    crate::wit::zed::extension::ui::write_clipboard(text)
}

/// Opens a path in the active workspace and optionally positions the cursor at row and column.
pub fn open_path(path: &str, row: Option<u32>, column: Option<u32>) -> Result<(), String> {
    crate::wit::zed::extension::ui::open_path(path, row, column)
}
