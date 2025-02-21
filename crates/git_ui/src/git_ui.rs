use ::settings::Settings;
use git::status::FileStatus;
use git_panel_settings::GitPanelSettings;
use gpui::App;
use project_diff::ProjectDiff;
use ui::{ActiveTheme, Color, Icon, IconName, IntoElement};

pub mod branch_picker;
mod commit_modal;
pub mod git_panel;
mod git_panel_settings;
pub mod project_diff;
pub mod repository_selector;

pub fn init(cx: &mut App) {
    GitPanelSettings::register(cx);
    branch_picker::init(cx);
    cx.observe_new(ProjectDiff::register).detach();
    commit_modal::init(cx);
}

// TODO: Add updated status colors to theme
pub fn git_status_icon(status: FileStatus, cx: &App) -> impl IntoElement {
    let (icon_name, color) = if status.is_conflicted() {
        (
            IconName::Warning,
            cx.theme().colors().version_control_conflict,
        )
    } else if status.is_deleted() {
        (
            IconName::SquareMinus,
            cx.theme().colors().version_control_deleted,
        )
    } else if status.is_modified() {
        (
            IconName::SquareDot,
            cx.theme().colors().version_control_modified,
        )
    } else {
        (
            IconName::SquarePlus,
            cx.theme().colors().version_control_added,
        )
    };
    Icon::new(icon_name).color(Color::Custom(color))
}
