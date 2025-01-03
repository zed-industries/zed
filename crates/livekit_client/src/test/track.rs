use super::*;
#[cfg(not(windows))]
use webrtc::{audio_source::RtcAudioSource, video_source::RtcVideoSource};

#[cfg(not(windows))]
pub use livekit::track::{TrackKind, TrackSource};

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
    #[cfg(not(target_os = "windows"))]
    pub(super) server_track: Arc<TestServerVideoTrack>,
    pub(super) _room: WeakRoom,
}

#[derive(Clone, Debug)]
pub struct RemoteAudioTrack {
    #[cfg(not(target_os = "windows"))]
    pub(super) server_track: Arc<TestServerAudioTrack>,
    pub(super) room: WeakRoom,
}

pub enum RtcTrack {
    Audio(RtcAudioTrack),
    Video(RtcVideoTrack),
}

pub struct RtcAudioTrack {
    #[cfg(not(target_os = "windows"))]
    pub(super) server_track: Arc<TestServerAudioTrack>,
    pub(super) room: WeakRoom,
}

pub struct RtcVideoTrack {
    #[cfg(not(target_os = "windows"))]
    pub(super) _server_track: Arc<TestServerVideoTrack>,
}

#[cfg(not(target_os = "windows"))]
impl RemoteTrack {
    pub fn sid(&self) -> TrackSid {
        match self {
            RemoteTrack::Audio(track) => track.sid(),
            RemoteTrack::Video(track) => track.sid(),
        }
    }

    pub fn kind(&self) -> TrackKind {
        match self {
            RemoteTrack::Audio(_) => TrackKind::Audio,
            RemoteTrack::Video(_) => TrackKind::Video,
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

#[cfg(not(windows))]
impl LocalVideoTrack {
    pub fn create_video_track(_name: &str, _source: RtcVideoSource) -> Self {
        Self {}
    }
}

#[cfg(not(windows))]
impl LocalAudioTrack {
    pub fn create_audio_track(_name: &str, _source: RtcAudioSource) -> Self {
        Self {}
    }
}

#[cfg(not(target_os = "windows"))]
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
            server_track: self.server_track.clone(),
            room: self.room.clone(),
        }
    }
}

#[cfg(not(target_os = "windows"))]
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

#[cfg(not(target_os = "windows"))]
impl RtcTrack {
    pub fn enabled(&self) -> bool {
        match self {
            RtcTrack::Audio(track) => track.enabled(),
            RtcTrack::Video(track) => track.enabled(),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        match self {
            RtcTrack::Audio(track) => track.set_enabled(enabled),
            RtcTrack::Video(_) => {}
        }
    }
}

#[cfg(not(target_os = "windows"))]
impl RtcAudioTrack {
    pub fn set_enabled(&self, enabled: bool) {
        if let Some(room) = self.room.upgrade() {
            let paused_audio_tracks = &mut room.0.lock().paused_audio_tracks;
            if enabled {
                paused_audio_tracks.remove(&self.server_track.sid);
            } else {
                paused_audio_tracks.insert(self.server_track.sid.clone());
            }
        }
    }

    pub fn enabled(&self) -> bool {
        if let Some(room) = self.room.upgrade() {
            !room
                .0
                .lock()
                .paused_audio_tracks
                .contains(&self.server_track.sid)
        } else {
            false
        }
    }
}

impl RtcVideoTrack {
    pub fn enabled(&self) -> bool {
        true
    }
}
