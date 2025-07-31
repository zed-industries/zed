use gpui::App;

use crate::{RemoteTrack, TrackSid, test::WeakRoom};

#[derive(Clone, Debug)]
pub struct LocalTrackPublication {
    pub(crate) sid: TrackSid,
    pub(crate) room: WeakRoom,
}

#[derive(Clone, Debug)]
pub struct RemoteTrackPublication {
    pub(crate) sid: TrackSid,
    pub(crate) room: WeakRoom,
    pub(crate) track: RemoteTrack,
}

impl LocalTrackPublication {
    pub fn sid(&self) -> TrackSid {
        self.sid.clone()
    }

    pub fn mute(&self, _cx: &App) {
        self.set_mute(true)
    }

    pub fn unmute(&self, _cx: &App) {
        self.set_mute(false)
    }

    fn set_mute(&self, mute: bool) {
        if let Some(room) = self.room.upgrade() {
            room.test_server()
                .set_track_muted(&room.token(), &self.sid, mute)
                .ok();
        }
    }

    pub fn is_muted(&self) -> bool {
        if let Some(room) = self.room.upgrade() {
            room.test_server()
                .is_track_muted(&room.token(), &self.sid)
                .unwrap_or(false)
        } else {
            false
        }
    }
}

impl RemoteTrackPublication {
    pub fn sid(&self) -> TrackSid {
        self.sid.clone()
    }

    pub fn track(&self) -> Option<RemoteTrack> {
        Some(self.track.clone())
    }

    pub fn is_audio(&self) -> bool {
        matches!(self.track, RemoteTrack::Audio(_))
    }

    pub fn is_muted(&self) -> bool {
        if let Some(room) = self.room.upgrade() {
            room.test_server()
                .is_track_muted(&room.token(), &self.sid)
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn is_enabled(&self) -> bool {
        if let Some(room) = self.room.upgrade() {
            !room.0.lock().paused_audio_tracks.contains(&self.sid)
        } else {
            false
        }
    }

    pub fn set_enabled(&self, enabled: bool, _cx: &App) {
        if let Some(room) = self.room.upgrade() {
            let paused_audio_tracks = &mut room.0.lock().paused_audio_tracks;
            if enabled {
                paused_audio_tracks.remove(&self.sid);
            } else {
                paused_audio_tracks.insert(self.sid.clone());
            }
        }
    }
}
