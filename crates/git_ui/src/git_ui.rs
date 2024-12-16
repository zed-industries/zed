use ::settings::Settings;
use call::ActiveCall;
use client::proto::ChannelRole;
use git::repository::GitFileStatus;
use gpui::{actions, prelude::*, AppContext, Global, Hsla, Model, ModelContext};
use settings::GitPanelSettings;
use ui::{Color, Icon, IconName, IntoElement, SharedString};

pub mod git_panel;
mod settings;

actions!(
    git_ui,
    [
        StageAll,
        UnstageAll,
        DiscardAll,
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
    co_authors: Vec<SharedString>,
    include_co_authors: bool,
}

impl GitState {
    pub fn new() -> Self {
        GitState {
            commit_message: None,
            co_authors: Vec::new(),
            include_co_authors: true,
        }
    }

    pub fn include_co_authors(&self) -> bool {
        // todo!(): Add a setting or toggle for including co-authors automatically
        self.include_co_authors
    }

    pub fn has_co_authors(&self) -> bool {
        !self.co_authors.is_empty()
    }

    pub fn refresh_co_authors(&mut self, cx: &mut ModelContext<Self>) {
        self.co_authors = ActiveCall::global(cx)
            .read(cx)
            .room()
            .and_then(|room| {
                let room = room.read(cx);
                let mut participants = room.remote_participants().values().collect::<Vec<_>>();
                participants.sort_by_key(|p| p.participant_index.0);

                Some(
                    participants
                        .into_iter()
                        .filter(|p| matches!(p.role, ChannelRole::Member | ChannelRole::Guest))
                        .map(|p| p.user.github_login.clone().into())
                        .collect(),
                )
            })
            .unwrap_or_default();
    }

    pub fn message(&self) -> Option<SharedString> {
        let mut message: String = "".into();

        if let Some(commit_message) = &self.commit_message {
            message.push_str(commit_message);
        }

        let already_co_authored = message.contains("Co-authored-by:");

        if self.include_co_authors() && !already_co_authored {
            let co_authors = &self
                .co_authors
                .clone()
                .into_iter()
                .map(|co_author| format!("Co-authored-by: {}", co_author))
                .collect::<Vec<_>>()
                .join("\n");

            message.push_str("\n");
            message.push_str(&co_authors);
        };

        if message.is_empty() {
            None
        } else {
            Some(message.into())
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

// impl EventEmitter<Event> for GitState {}

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum Event {}

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

// todo!(): Add updated status colors to theme
pub fn git_status_icon(status: GitFileStatus) -> impl IntoElement {
    match status {
        GitFileStatus::Added => Icon::new(IconName::SquarePlus).color(Color::Custom(ADDED_COLOR)),
        GitFileStatus::Modified => {
            Icon::new(IconName::SquareDot).color(Color::Custom(MODIFIED_COLOR))
        }
        GitFileStatus::Conflict => Icon::new(IconName::Warning).color(Color::Custom(REMOVED_COLOR)),
    }
}
