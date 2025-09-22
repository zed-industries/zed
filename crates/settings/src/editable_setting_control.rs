use fs::Fs;
use gpui::{App, RenderOnce, SharedString};

use crate::{settings_content::SettingsContent, update_settings_file};

/// A UI control that can be used to edit a setting.
pub trait EditableSettingControl: RenderOnce {
    /// The type of the setting value.
    type Value: Send;

    /// Returns the name of this setting.
    fn name(&self) -> SharedString;

    /// Reads the setting value from the settings.
    fn read(cx: &App) -> Self::Value;

    /// Applies the given setting file to the settings file contents.
    ///
    /// This will be called when writing the setting value back to the settings file.
    fn apply(settings: &mut SettingsContent, value: Self::Value, cx: &App);

    /// Writes the given setting value to the settings files.
    fn write(value: Self::Value, cx: &App) {
        let fs = <dyn Fs>::global(cx);

        update_settings_file(fs, cx, move |settings, cx| {
            Self::apply(settings, value, cx);
        });
    }
}
