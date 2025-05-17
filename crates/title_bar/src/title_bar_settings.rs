use db::anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Copy, Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(default)]
pub struct TitleBarSettings {
    /// Whether to show the branch icon beside branch switcher in the title bar.
    ///
    /// Default: false
    pub show_branch_icon: bool,
    /// Whether to show onboarding banners in the title bar.
    ///
    /// Default: true
    pub show_onboarding_banner: bool,
    /// Whether to show user avatar in the title bar.
    ///
    /// Default: true
    pub show_user_picture: bool,
    /// Whether to show the branch name button in the titlebar.
    ///
    /// Default: true
    pub show_branch_name: bool,
    /// Whether to show the project host and name in the titlebar.
    ///
    /// Default: true
    pub show_project_items: bool,
}

impl Default for TitleBarSettings {
    fn default() -> Self {
        Self {
            show_branch_icon: false,
            show_onboarding_banner: true,
            show_user_picture: true,
            show_branch_name: true,
            show_project_items: true,
        }
    }
}

impl Settings for TitleBarSettings {
    const KEY: Option<&'static str> = Some("title_bar");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::App) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        sources.json_merge()
    }

    fn import_from_vscode(_: &settings::VsCodeSettings, _: &mut Self::FileContent) {}
}
