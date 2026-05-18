use crate::{self as settings, RegisterSetting, Settings};
use gpui::CursorStyle;

#[derive(Debug, Clone, Copy, RegisterSetting)]
pub struct ClickableElementCursorSettings(pub CursorStyle);

impl Settings for ClickableElementCursorSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self(match content.clickable_element_cursor.unwrap_or_default() {
            settings::ClickableElementCursor::Pointer => CursorStyle::PointingHand,
            settings::ClickableElementCursor::Default => CursorStyle::Arrow,
        })
    }
}
