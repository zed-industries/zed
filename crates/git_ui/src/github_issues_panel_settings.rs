use gpui::Pixels;
use settings::{RegisterSetting, Settings};
use ui::px;
use workspace::dock::DockPosition;

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct GithubIssuesPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

impl Settings for GithubIssuesPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let github_issues_panel = content.github_issues_panel.clone().unwrap();
        Self {
            button: github_issues_panel.button.unwrap(),
            dock: github_issues_panel.dock.unwrap().into(),
            default_width: px(github_issues_panel.default_width.unwrap()),
        }
    }
}
