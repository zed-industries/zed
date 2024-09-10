use gpui::Pixels;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use ui::px;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectPanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, JsonSchema)]
#[serde(default)]
pub struct ProjectPanelSettings {
    /// Whether to show the project panel button in the status bar.
    pub button: bool,
    /// Customize default width (in pixels) taken by project panel
    pub default_width: Pixels,
    /// The position of project panel
    pub dock: ProjectPanelDockPosition,
    /// Whether to show file icons in the project panel.
    pub file_icons: bool,
    /// Whether to show folder icons or chevrons for directories in the project panel.
    pub folder_icons: bool,
    /// Whether to show the git status in the project panel.
    pub git_status: bool,
    /// Amount of indentation (in pixels) for nested items.
    pub indent_size: Pixels,
    /// Whether to reveal it in the project panel automatically,
    /// when a corresponding project entry becomes active.
    /// Gitignored entries are never auto revealed.
    pub auto_reveal_entries: bool,
    /// Whether to fold directories automatically
    /// when directory has only one directory inside.
    pub auto_fold_dirs: bool,
    /// Scrollbar-related settings
    pub scrollbar: ScrollbarSettings,
}

impl Default for ProjectPanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            default_width: px(240.),
            dock: ProjectPanelDockPosition::Left,
            file_icons: true,
            folder_icons: true,
            git_status: true,
            indent_size: px(20.),
            auto_reveal_entries: true,
            auto_fold_dirs: true,
            scrollbar: Default::default(),
        }
    }
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

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: always
    pub show: ShowScrollbar,
}

impl Settings for ProjectPanelSettings {
    const KEY: Option<&'static str> = Some("project_panel");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
