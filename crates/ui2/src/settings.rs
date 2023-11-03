use std::ops::Deref;

use gpui2::{rems, AbsoluteLength, AppContext, WindowContext};

use crate::prelude::*;

pub fn init(cx: &mut AppContext) {
    cx.set_global(FakeSettings::default());
}

/// Returns the user settings.
pub fn user_settings(cx: &WindowContext) -> FakeSettings {
    cx.global::<FakeSettings>().clone()
}

pub fn user_settings_mut<'cx>(cx: &'cx mut WindowContext) -> &'cx mut FakeSettings {
    cx.global_mut::<FakeSettings>()
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
pub struct FakeSettings {
    pub default_panel_size: SettingValue<AbsoluteLength>,
    pub list_disclosure_style: SettingValue<DisclosureControlStyle>,
    pub list_indent_depth: SettingValue<AbsoluteLength>,
    pub titlebar: TitlebarSettings,
}

impl Default for FakeSettings {
    fn default() -> Self {
        Self {
            titlebar: TitlebarSettings::default(),
            list_disclosure_style: SettingValue::Default(DisclosureControlStyle::ChevronOnHover),
            list_indent_depth: SettingValue::Default(rems(0.3).into()),
            default_panel_size: SettingValue::Default(rems(16.).into()),
        }
    }
}

impl FakeSettings {}
