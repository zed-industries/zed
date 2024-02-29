use anyhow;
use gpui::Pixels;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Settings;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectPanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
pub struct ProjectPanelSettings {
    pub default_width: Pixels,
    pub dock: ProjectPanelDockPosition,
    pub file_icons: bool,
    pub folder_icons: bool,
    pub git_status: bool,
    pub indent_size: f32,
    pub auto_reveal_entries: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct ProjectPanelSettingsContent {
    /// Customise default width (in pixels) taken by project panel
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
}

impl Settings for ProjectPanelSettings {
    const KEY: Option<&'static str> = Some("project_panel");

    type FileContent = ProjectPanelSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
