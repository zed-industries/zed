use anyhow;
use gpui::Pixels;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectPanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct ProjectPanelSettings {
    pub button: bool,
    pub default_width: Pixels,
    pub dock: ProjectPanelDockPosition,
    pub file_icons: bool,
    pub folder_icons: bool,
    pub git_status: bool,
    pub indent_size: f32,
    pub auto_reveal_entries: bool,
    pub auto_fold_dirs: bool,
    pub scrollbar: ScrollbarSettings,
}

/// When to show the scrollbar in the project panel.
///
/// Default: always
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShowScrollbar {
    #[default]
    /// Always show the scrollbar.
    Always,
    /// Never show the scrollbar.
    Never,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: always
    pub show: ShowScrollbar,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettingsContent {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: always
    pub show: Option<ShowScrollbar>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ProjectPanelSettingsContent {
    /// Whether to show the project panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Customize default width (in pixels) taken by project panel
    ///
    /// Default: 240
    pub default_width: Option<f32>,
    /// The position of project panel
    ///
    /// Default: left
    pub dock: Option<ProjectPanelDockPosition>,
    /// Whether to show file icons in the project panel.
    ///
    /// Default: true
    pub file_icons: Option<bool>,
    /// Whether to show folder icons or chevrons for directories in the project panel.
    ///
    /// Default: true
    pub folder_icons: Option<bool>,
    /// Whether to show the git status in the project panel.
    ///
    /// Default: true
    pub git_status: Option<bool>,
    /// Amount of indentation (in pixels) for nested items.
    ///
    /// Default: 20
    pub indent_size: Option<f32>,
    /// Whether to reveal it in the project panel automatically,
    /// when a corresponding project entry becomes active.
    /// Gitignored entries are never auto revealed.
    ///
    /// Default: true
    pub auto_reveal_entries: Option<bool>,
    /// Whether to fold directories automatically
    /// when directory has only one directory inside.
    ///
    /// Default: false
    pub auto_fold_dirs: Option<bool>,
    /// Scrollbar-related settings
    pub scrollbar: Option<ScrollbarSettingsContent>,
}

impl Settings for ProjectPanelSettings {
    const KEY: Option<&'static str> = Some("project_panel");

    type FileContent = ProjectPanelSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
