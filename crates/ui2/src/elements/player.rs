use gpui3::{Hsla, ViewContext};

use crate::theme;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum PlayerStatus {
    #[default]
    Offline,
    Online,
    InCall,
    Away,
    DoNotDisturb,
    Invisible,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum MicStatus {
    Muted,
    #[default]
    Unmuted,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum VideoStatus {
    On,
    #[default]
    Off,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum ScreenShareStatus {
    Shared,
    #[default]
    NotShared,
}

#[derive(Clone)]
pub struct PlayerCallStatus {
    pub mic_status: MicStatus,
    /// Indicates if the player is currently speaking
    /// And the intensity of the volume coming through
    ///
    /// 0.0 - 1.0
    pub voice_activity: f32,
    pub video_status: VideoStatus,
    pub screen_share_status: ScreenShareStatus,
    pub in_current_project: bool,
    pub disconnected: bool,
    pub following: Option<Vec<Player>>,
    pub followers: Option<Vec<Player>>,
}

impl PlayerCallStatus {
    pub fn new() -> Self {
        Self {
            mic_status: MicStatus::default(),
            voice_activity: 0.,
            video_status: VideoStatus::default(),
            screen_share_status: ScreenShareStatus::default(),
            in_current_project: true,
            disconnected: false,
            following: None,
            followers: None,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Player {
    index: usize,
    avatar_src: String,
    username: String,
    status: PlayerStatus,
}

#[derive(Clone)]
pub struct PlayerWithCallStatus {
    player: Player,
    call_status: PlayerCallStatus,
}

impl PlayerWithCallStatus {
    pub fn new(player: Player, call_status: PlayerCallStatus) -> Self {
        Self {
            player,
            call_status,
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_call_status(&self) -> &PlayerCallStatus {
        &self.call_status
    }
}

impl Player {
    pub fn new(index: usize, avatar_src: String, username: String) -> Self {
        Self {
            index,
            avatar_src,
            username,
            status: Default::default(),
        }
    }

    pub fn set_status(mut self, status: PlayerStatus) -> Self {
        self.status = status;
        self
    }

    pub fn cursor_color<S: 'static>(&self, cx: &mut ViewContext<S>) -> Hsla {
        let theme = theme(cx);
        let index = self.index % 8;
        theme.players[self.index].cursor
    }

    pub fn selection_color<S: 'static>(&self, cx: &mut ViewContext<S>) -> Hsla {
        let theme = theme(cx);
        let index = self.index % 8;
        theme.players[self.index].selection
    }

    pub fn avatar_src(&self) -> &str {
        &self.avatar_src
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
