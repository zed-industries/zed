use super::*;
use webrtc::{audio_source::RtcAudioSource, video_source::RtcVideoSource};

#[derive(Clone, Debug)]
pub enum LocalTrack {
    Audio(LocalAudioTrack),
    Video(LocalVideoTrack),
}

#[derive(Clone, Debug)]
pub enum RemoteTrack {
    Audio(RemoteAudioTrack),
    Video(RemoteVideoTrack),
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

pub enum RtcTrack {
    Audio(RtcAudioTrack),
    Video(RtcVideoTrack),
}

pub struct RtcAudioTrack {
    pub(super) _server_track: Arc<TestServerAudioTrack>,
}

pub struct RtcVideoTrack {
    pub(super) _server_track: Arc<TestServerVideoTrack>,
}

impl RemoteTrack {
    pub fn sid(&self) -> TrackSid {
        match self {
            RemoteTrack::Audio(track) => track.sid(),
            RemoteTrack::Video(track) => track.sid(),
        }
    }

    pub fn publisher_id(&self) -> ParticipantIdentity {
        match self {
            RemoteTrack::Audio(track) => track.publisher_id(),
            RemoteTrack::Video(track) => track.publisher_id(),
        }
    }

    pub fn rtc_track(&self) -> RtcTrack {
        match self {
            RemoteTrack::Audio(track) => RtcTrack::Audio(track.rtc_track()),
            RemoteTrack::Video(track) => RtcTrack::Video(track.rtc_track()),
        }
    }
}

impl LocalVideoTrack {
    pub fn create_video_track(_name: &str, _source: RtcVideoSource) -> Self {
        Self {}
    }
}

impl LocalAudioTrack {
    pub fn create_audio_track(_name: &str, _source: RtcAudioSource) -> Self {
        Self {}
    }
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

    pub fn rtc_track(&self) -> RtcAudioTrack {
        RtcAudioTrack {
            _server_track: self.server_track.clone(),
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

    pub fn rtc_track(&self) -> RtcVideoTrack {
        RtcVideoTrack {
            _server_track: self.server_track.clone(),
        }
    }
}

impl RtcTrack {
    pub fn enabled(&self) -> bool {
        match self {
            RtcTrack::Audio(track) => track.enabled(),
            RtcTrack::Video(track) => track.enabled(),
        }
    }

    pub fn set_enabled(&self, _enabled: bool) {}
}

impl RtcAudioTrack {
    pub fn enabled(&self) -> bool {
        true
    }
}

impl RtcVideoTrack {
    pub fn enabled(&self) -> bool {
        true
    }
}
