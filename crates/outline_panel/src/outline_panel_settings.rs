use anyhow;
use gpui::{px, Pixels};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutlinePanelDockPosition {
    Left,
    Right,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, JsonSchema)]
pub struct OutlinePanelSettings {
    /// Whether to show the outline panel button in the status bar.
    pub button: bool,
    /// Customize default width (in pixels) taken by outline panel
    pub default_width: Pixels,
    /// The position of outline panel
    pub dock: OutlinePanelDockPosition,
    /// Whether to show file icons in the outline panel.
    pub file_icons: bool,
    /// Whether to show folder icons or chevrons for directories in the outline panel.
    pub folder_icons: bool,
    /// Whether to show the git status in the outline panel.
    pub git_status: bool,
    /// Amount of indentation (in pixels) for nested items.
    pub indent_size: Pixels,
    /// Whether to reveal it in the outline panel automatically,
    /// when a corresponding project entry becomes active.
    /// Gitignored entries are never auto revealed.
    pub auto_reveal_entries: bool,
    /// Whether to fold directories automatically
    /// when directory has only one directory inside.
    pub auto_fold_dirs: bool,
}

impl Default for OutlinePanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            default_width: px(240.),
            dock: OutlinePanelDockPosition::Left,
            file_icons: true,
            folder_icons: true,
            auto_fold_dirs: true,
            auto_reveal_entries: true,
            indent_size: px(20.),
            git_status: true,
        }
    }
}

impl Settings for OutlinePanelSettings {
    const KEY: Option<&'static str> = Some("outline_panel");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
