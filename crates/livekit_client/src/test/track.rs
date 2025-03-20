use std::sync::Arc;

use crate::{ParticipantIdentity, TrackSid};

use super::{TestServerAudioTrack, TestServerVideoTrack, WeakRoom};

#[derive(Clone, Debug)]
pub enum LocalTrack {
    Audio(LocalAudioTrack),
    Video(LocalVideoTrack),
}

#[derive(Clone, Debug)]
pub struct LocalVideoTrack {}

#[derive(Clone, Debug)]
pub struct LocalAudioTrack {}

#[derive(Clone, Debug)]
pub struct RemoteVideoTrack {
    pub(super) server_track: Arc<TestServerVideoTrack>,
    pub(super) _room: WeakRoom,
}

#[derive(Clone, Debug)]
pub struct RemoteAudioTrack {
    pub(super) server_track: Arc<TestServerAudioTrack>,
    pub(super) room: WeakRoom,
}

impl RemoteAudioTrack {
    pub fn sid(&self) -> TrackSid {
        self.server_track.sid.clone()
    }

    pub fn publisher_id(&self) -> ParticipantIdentity {
        self.server_track.publisher_id.clone()
    }

    pub fn start(&self) {
        if let Some(room) = self.room.upgrade() {
            room.0
                .lock()
                .paused_audio_tracks
                .remove(&self.server_track.sid);
        }
    }

    pub fn stop(&self) {
        if let Some(room) = self.room.upgrade() {
            room.0
                .lock()
                .paused_audio_tracks
                .insert(self.server_track.sid.clone());
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        if enabled {
            self.start()
        } else {
            self.stop()
        }
    }
}

impl RemoteVideoTrack {
    pub fn sid(&self) -> TrackSid {
        self.server_track.sid.clone()
    }

    pub fn publisher_id(&self) -> ParticipantIdentity {
        self.server_track.publisher_id.clone()
    }

    pub(crate) fn set_enabled(&self, _enabled: bool) {}
}
