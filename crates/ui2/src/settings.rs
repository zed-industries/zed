use gpui3::{rems, AbsoluteLength};

use crate::DisclosureControlStyle;

// This is a fake static example of user settings overriding the default settings
pub fn user_settings() -> Settings {
    let mut settings = Settings::new();
    settings.list_indent_depth = Some(rems(0.5).into());
    settings
}

#[derive(Clone, Copy)]
pub struct TitlebarSettings {
    pub show_project_owner: Option<bool>,
    pub show_git_status: Option<bool>,
    pub show_git_controls: Option<bool>,
}

impl Default for TitlebarSettings {
    fn default() -> Self {
        Self {
            show_project_owner: Some(true),
            show_git_status: Some(true),
            show_git_controls: Some(true),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Settings {
    pub default_panel_size: Option<AbsoluteLength>,
    pub list_disclosure_style: Option<DisclosureControlStyle>,
    pub list_indent_depth: Option<AbsoluteLength>,
    pub titlebar: TitlebarSettings,
    pub ui_scale: Option<f32>,
}

// These should be merged into settings
impl Settings {
    pub fn new() -> Self {
        Self {
            titlebar: TitlebarSettings::default(),
            list_disclosure_style: None,
            list_indent_depth: None,
            default_panel_size: None,
            ui_scale: None,
        }
    }

    pub fn titlebar_show_project_owner(&self) -> bool {
        self.titlebar.show_project_owner.unwrap_or(
            Settings::default()
                .titlebar
                .show_project_owner
                .expect("titlebar_show_project_owner default not set."),
        )
    }

    pub fn list_disclosure_style(&self) -> DisclosureControlStyle {
        self.list_disclosure_style.unwrap_or(
            Settings::default()
                .list_disclosure_style
                .expect("list_disclosure_style default not set."),
        )
    }

    pub fn list_indent_depth(&self) -> AbsoluteLength {
        self.list_indent_depth.unwrap_or(
            Settings::default()
                .list_indent_depth
                .expect("list_indent_depth default not set."),
        )
    }

    pub fn default_panel_size(&self) -> AbsoluteLength {
        self.default_panel_size.unwrap_or(
            Settings::default()
                .default_panel_size
                .expect("default_panel_size default not set."),
        )
    }

    pub fn ui_scale(&self) -> f32 {
        self.ui_scale.unwrap_or(
            Settings::default()
                .ui_scale
                .expect("ui_scale default not set."),
        )
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            titlebar: TitlebarSettings::default(),
            list_disclosure_style: Some(DisclosureControlStyle::ChevronOnHover),
            list_indent_depth: Some(rems(0.3).into()),
            default_panel_size: Some(rems(16.).into()),
            ui_scale: Some(1.),
        }
    }
}
