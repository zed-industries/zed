use ::settings::Settings;
use git::repository::GitFileStatus;
use gpui::{actions, AppContext, Context, Global, Hsla, Model};
use settings::GitPanelSettings;
use ui::{Color, Icon, IconName, IntoElement, SharedString};

pub mod git_panel;
mod settings;

actions!(
    git_ui,
    [
        StageAll,
        UnstageAll,
        RevertAll,
        CommitStagedChanges,
        CommitAllChanges,
        ClearMessage
    ]
);

pub fn init(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
    let git_state = cx.new_model(|_cx| GitState::new());
    cx.set_global(GlobalGitState(git_state));
}

struct GlobalGitState(Model<GitState>);

impl Global for GlobalGitState {}

pub struct GitState {
    commit_message: Option<SharedString>,
}

impl GitState {
    pub fn new() -> Self {
        GitState {
            commit_message: None,
        }
    }

    pub fn set_message(&mut self, message: Option<SharedString>) {
        self.commit_message = message;
    }

    pub fn clear_message(&mut self) {
        self.commit_message = None;
    }

    pub fn get_global(cx: &mut AppContext) -> Model<GitState> {
        cx.global::<GlobalGitState>().0.clone()
    }
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
        GitFileStatus::Added | GitFileStatus::Untracked => {
            Icon::new(IconName::SquarePlus).color(Color::Custom(ADDED_COLOR))
        }
        GitFileStatus::Modified => {
            Icon::new(IconName::SquareDot).color(Color::Custom(MODIFIED_COLOR))
        }
        GitFileStatus::Conflict => Icon::new(IconName::Warning).color(Color::Custom(REMOVED_COLOR)),
        GitFileStatus::Deleted => {
            Icon::new(IconName::SquareMinus).color(Color::Custom(REMOVED_COLOR))
        }
    }
}
