use db::anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsContent, SettingsKey, SettingsSources, SettingsUi};

#[derive(Copy, Clone, Serialize, Deserialize, JsonSchema, Debug, SettingsUi)]
#[serde(rename_all = "snake_case")]
pub enum TitleBarVisibility {
    Always,
    Never,
    HideInFullScreen,
}

#[derive(Copy, Clone, Debug)]
pub struct TitleBarSettings {
    pub show: TitleBarVisibility,
    pub show_branch_icon: bool,
    pub show_onboarding_banner: bool,
    pub show_user_picture: bool,
    pub show_branch_name: bool,
    pub show_project_items: bool,
    pub show_sign_in: bool,
    pub show_menus: bool,
}

impl Settings for TitleBarSettings {
    fn from_default(s: &SettingsContent) -> Option<Self> {
        let content = s.title_bar?;
        TitleBarSettings {
            show: content.show?,
            show_branch_icon: content.show_branch_icon?,
            show_onboarding_banner: content.show_onboarding_banner?,
            show_user_picture: content.show_user_picture?,
            show_branch_name: content.show_branch_name?,
            show_project_items: content.show_project_items?,
            show_sign_in: content.show_sign_in?,
            show_menus: content.show_menus?,
        }
    }

    fn refine(&mut self, s: &SettingsContent, _: &mut App) {
        let Some(content) = s.title_bar else {
            return
        }

        self.show.refine(&content.show);
        self.show_branch_icon.refine(content.show_branch_icon);
        self.show_onboarding_banner.refine(content.show_onboarding_banner);
        self.show_user_picture.refine(content.show_user_picture);
        self.show_branch_name.refine(content.show_branch_name);
        self.show_project_items.refine(content.show_project_items);
        self.show_sign_in.refine(content.show_sign_in);
        self.show_menus.refine(content.show_menus);
    }

    fn import_from_vscode(_: &settings::VsCodeSettings, _: &mut Self::FileContent) {}
}
