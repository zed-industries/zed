use std::ops::Deref;

use gpui3::{rems, AbsoluteLength};

use crate::DisclosureControlStyle;

// This is a fake static example of user settings overriding the default settings
pub fn user_settings() -> Settings {
    let mut settings = Settings::default();
    settings.list_indent_depth = SettingValue::UserDefined(rems(0.5).into());
    // settings.ui_scale = SettingValue::UserDefined(2.);
    settings
}

#[derive(Clone)]
pub enum SettingValue<T> {
    UserDefined(T),
    Default(T),
}

impl<T> Deref for SettingValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::UserDefined(value) => value,
            Self::Default(value) => value,
        }
    }
}

#[derive(Clone)]
pub struct TitlebarSettings {
    pub show_project_owner: SettingValue<bool>,
    pub show_git_status: SettingValue<bool>,
    pub show_git_controls: SettingValue<bool>,
}

impl Default for TitlebarSettings {
    fn default() -> Self {
        Self {
            show_project_owner: SettingValue::Default(true),
            show_git_status: SettingValue::Default(true),
            show_git_controls: SettingValue::Default(true),
        }
    }
}

// These should be merged into settings
#[derive(Clone)]
pub struct Settings {
    pub default_panel_size: SettingValue<AbsoluteLength>,
    pub list_disclosure_style: SettingValue<DisclosureControlStyle>,
    pub list_indent_depth: SettingValue<AbsoluteLength>,
    pub titlebar: TitlebarSettings,
    pub ui_scale: SettingValue<f32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            titlebar: TitlebarSettings::default(),
            list_disclosure_style: SettingValue::Default(DisclosureControlStyle::ChevronOnHover),
            list_indent_depth: SettingValue::Default(rems(0.3).into()),
            default_panel_size: SettingValue::Default(rems(16.).into()),
            ui_scale: SettingValue::Default(1.),
        }
    }
}

impl Settings {}
