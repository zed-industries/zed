use ::settings::Settings;
use gpui::AppContext;
use settings::GitPanelSettings;

pub mod git_panel;
mod settings;

pub fn init(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
}
