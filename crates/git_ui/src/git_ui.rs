use ::settings::Settings;
use gpui::{actions, AppContext};
use settings::GitPanelSettings;

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

/// What changes should be committed.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) enum CommitMode {
    /// Commit all changes, regardless of their staging status.
    All,
    /// Commit only staged changes.
    #[default]
    Staged,
}
