use db::anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct TitleBarSettings {
    pub show_branch_icon: bool,
    pub show_onboarding_banner: bool,
    pub show_user_picture: bool,
    pub show_branch_name: bool,
    pub show_project_items: bool,
    pub show_sign_in: bool,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct TitleBarSettingsContent {
    /// Whether to show the branch icon beside branch switcher in the title bar.
    ///
    /// Default: false
    pub show_branch_icon: Option<bool>,
    /// Whether to show onboarding banners in the title bar.
    ///
    /// Default: true
    pub show_onboarding_banner: Option<bool>,
    /// Whether to show user avatar in the title bar.
    ///
    /// Default: true
    pub show_user_picture: Option<bool>,
    /// Whether to show the branch name button in the titlebar.
    ///
    /// Default: true
    pub show_branch_name: Option<bool>,
    /// Whether to show the project host and name in the titlebar.
    ///
    /// Default: true
    pub show_project_items: Option<bool>,
    /// Whether to show the sign in button in the title bar.
    ///
    /// Default: true
    pub show_sign_in: Option<bool>,
}

impl Settings for TitleBarSettings {
    const KEY: Option<&'static str> = Some("title_bar");

    type FileContent = TitleBarSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut gpui::App) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        sources.json_merge()
    }

    fn import_from_vscode(_: &settings::VsCodeSettings, _: &mut Self::FileContent) {}
}
