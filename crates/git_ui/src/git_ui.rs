use ::settings::Settings;
use git::repository::GitFileStatus;
use gpui::{actions, AppContext, Hsla};
use settings::GitPanelSettings;
use ui::{Color, Icon, IconName, IntoElement};

pub mod git_panel;
mod settings;

actions!(
    git_ui,
    [
        StageAll,
        UnstageAll,
        DiscardAll,
        CommitStagedChanges,
        CommitAllChanges
    ]
);

pub fn init(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
}

const ADDED_COLOR: Hsla = Hsla {
    h: 142. / 360.,
    s: 0.68,
    l: 0.45,
    a: 1.0,
};
const MODIFIED_COLOR: Hsla = Hsla {
    h: 48. / 360.,
    s: 0.76,
    l: 0.47,
    a: 1.0,
};
const REMOVED_COLOR: Hsla = Hsla {
    h: 355. / 360.,
    s: 0.65,
    l: 0.65,
    a: 1.0,
};

// TODO: Add updated status colors to theme
pub fn git_status_icon(status: GitFileStatus) -> impl IntoElement {
    match status {
        GitFileStatus::Added => Icon::new(IconName::SquarePlus).color(Color::Custom(ADDED_COLOR)),
        GitFileStatus::Modified => {
            Icon::new(IconName::SquareDot).color(Color::Custom(MODIFIED_COLOR))
        }
        GitFileStatus::Conflict => Icon::new(IconName::Warning).color(Color::Custom(REMOVED_COLOR)),
    }
}
