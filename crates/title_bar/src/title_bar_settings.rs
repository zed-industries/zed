pub use settings::TitleBarVisibility;
use settings::{Settings, SettingsContent};
use ui::App;
use util::MergeFrom;

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
    fn from_settings(s: &SettingsContent, _: &mut App) -> Self {
        let content = s.title_bar.clone().unwrap();
        TitleBarSettings {
            show: content.show.unwrap(),
            show_branch_icon: content.show_branch_icon.unwrap(),
            show_onboarding_banner: content.show_onboarding_banner.unwrap(),
            show_user_picture: content.show_user_picture.unwrap(),
            show_branch_name: content.show_branch_name.unwrap(),
            show_project_items: content.show_project_items.unwrap(),
            show_sign_in: content.show_sign_in.unwrap(),
            show_menus: content.show_menus.unwrap(),
        }
    }

    fn refine(&mut self, s: &SettingsContent, _: &mut App) {
        let Some(content) = &s.title_bar else {
            return;
        };

        self.show.merge_from(&content.show);
        self.show_branch_icon.merge_from(&content.show_branch_icon);
        self.show_onboarding_banner
            .merge_from(&content.show_onboarding_banner);
        self.show_user_picture
            .merge_from(&content.show_user_picture);
        self.show_branch_name.merge_from(&content.show_branch_name);
        self.show_project_items
            .merge_from(&content.show_project_items);
        self.show_sign_in.merge_from(&content.show_sign_in);
        self.show_menus.merge_from(&content.show_menus);
    }

    fn import_from_vscode(_: &settings::VsCodeSettings, _: &mut SettingsContent) {}
}
