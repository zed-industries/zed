use fs::Fs;
use gpui::{AppContext, RenderOnce, SharedString};

use crate::{update_settings_file, Settings};

/// A UI control that can be used to edit a setting.
pub trait EditableSettingControl: RenderOnce {
    /// The type of the setting value.
    type Value: Send;

    /// The settings type to which this setting belongs.
    type Settings: Settings;

    /// Returns the name of this setting.
    fn name(&self) -> SharedString;

    /// Reads the setting value from the settings.
    fn read(cx: &AppContext) -> Self::Value;

    /// Applies the given setting file to the settings file contents.
    ///
    /// This will be called when writing the setting value back to the settings file.
    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        cx: &AppContext,
    );

    /// Writes the given setting value to the settings files.
    fn write(value: Self::Value, cx: &AppContext) {
        let fs = <dyn Fs>::global(cx);

        update_settings_file::<Self::Settings>(fs, cx, move |settings, cx| {
            Self::apply(settings, value, cx);
        });
    }
}
