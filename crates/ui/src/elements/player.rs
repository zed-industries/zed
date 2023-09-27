use crate::theme;
use gpui2::{Hsla, ViewContext};

#[derive(Default, PartialEq)]
pub enum PlayerStatus {
    #[default]
    Offline,
    Online,
    InCall,
    Away,
    DoNotDisturb,
    Invisible,
}

pub struct PlayerCallStatus {
    mic_on: bool,
    /// Indicates if the player is currently speaking
    /// And the intensity of the volume coming through
    ///
    /// 0.0 - 1.0
    voice_activity: f32,
    video_on: bool,
    screen_shared: bool,
    in_current_project: bool,
    disconnected: bool,
    following: Option<Vec<Player>>,
    followers: Option<Vec<Player>>,
}

impl PlayerCallStatus {
    pub fn new() -> Self {
        Self {
            mic_on: true,
            voice_activity: 0.,
            video_on: false,
            screen_shared: false,
            in_current_project: true,
            disconnected: false,
            following: None,
            followers: None,
        }
    }

    pub fn mic_on(mut self, mic_on: bool) -> Self {
        self.mic_on = mic_on;
        self
    }

    pub fn voice_activity(mut self, voice_activity: f32) -> Self {
        self.voice_activity = voice_activity;
        self
    }

    pub fn video_on(mut self, video_on: bool) -> Self {
        self.video_on = video_on;
        self
    }

    pub fn screen_shared(mut self, screen_shared: bool) -> Self {
        self.screen_shared = screen_shared;
        self
    }

    pub fn in_current_project(mut self, in_current_project: bool) -> Self {
        self.in_current_project = in_current_project;
        self
    }

    pub fn disconnected(mut self, disconnected: bool) -> Self {
        self.disconnected = disconnected;
        self
    }

    pub fn following(mut self, following: Vec<Player>) -> Self {
        self.following = Some(following);
        self
    }

    pub fn followers(mut self, followers: Vec<Player>) -> Self {
        self.followers = Some(followers);
        self
    }
}

pub struct Player {
    index: usize,
    profile_photo: String,
    username: String,
    status: PlayerStatus,
}

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
    pub fn new(index: usize, profile_photo: String, username: String) -> Self {
        Self {
            index,
            profile_photo,
            username,
            status: Default::default(),
        }
    }

    pub fn set_status(mut self, status: PlayerStatus) -> Self {
        self.status = status;
        self
    }

    pub fn cursor_color<V>(self, cx: &mut ViewContext<V>, index: usize) -> Hsla {
        let theme = theme(cx);
        theme.players[self.index].cursor
    }

    pub fn selection_color<V>(self, cx: &mut ViewContext<V>, index: usize) -> Hsla {
        let theme = theme(cx);
        theme.players[self.index].selection
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}
