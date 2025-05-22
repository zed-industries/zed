use collections::HashMap;

mod remote_video_track_view;
pub use remote_video_track_view::{RemoteVideoTrackView, RemoteVideoTrackViewEvent};

#[cfg(not(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
)))]
mod livekit_client;
#[cfg(not(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
)))]
pub use livekit_client::*;

#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
))]
mod mock_client;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
))]
pub mod test;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
))]
pub use mock_client::*;

#[derive(Debug, Clone)]
pub enum Participant {
    Local(LocalParticipant),
    Remote(RemoteParticipant),
}

#[derive(Debug, Clone)]
pub enum TrackPublication {
    Local(LocalTrackPublication),
    Remote(RemoteTrackPublication),
}

impl TrackPublication {
    pub fn sid(&self) -> TrackSid {
        match self {
            TrackPublication::Local(local) => local.sid(),
            TrackPublication::Remote(remote) => remote.sid(),
        }
    }

    pub fn is_muted(&self) -> bool {
        match self {
            TrackPublication::Local(local) => local.is_muted(),
            TrackPublication::Remote(remote) => remote.is_muted(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RemoteTrack {
    Audio(RemoteAudioTrack),
    Video(RemoteVideoTrack),
}

impl RemoteTrack {
    pub fn sid(&self) -> TrackSid {
        match self {
            RemoteTrack::Audio(remote_audio_track) => remote_audio_track.sid(),
            RemoteTrack::Video(remote_video_track) => remote_video_track.sid(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LocalTrack {
    Audio(LocalAudioTrack),
    Video(LocalVideoTrack),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RoomEvent {
    ParticipantConnected(RemoteParticipant),
    ParticipantDisconnected(RemoteParticipant),
    LocalTrackPublished {
        publication: LocalTrackPublication,
        track: LocalTrack,
        participant: LocalParticipant,
    },
    LocalTrackUnpublished {
        publication: LocalTrackPublication,
        participant: LocalParticipant,
    },
    LocalTrackSubscribed {
        track: LocalTrack,
    },
    TrackSubscribed {
        track: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackUnsubscribed {
        track: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackSubscriptionFailed {
        participant: RemoteParticipant,
        // error: livekit::track::TrackError,
        track_sid: TrackSid,
    },
    TrackPublished {
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackUnpublished {
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    },
    TrackMuted {
        participant: Participant,
        publication: TrackPublication,
    },
    TrackUnmuted {
        participant: Participant,
        publication: TrackPublication,
    },
    RoomMetadataChanged {
        old_metadata: String,
        metadata: String,
    },
    ParticipantMetadataChanged {
        participant: Participant,
        old_metadata: String,
        metadata: String,
    },
    ParticipantNameChanged {
        participant: Participant,
        old_name: String,
        name: String,
    },
    ParticipantAttributesChanged {
        participant: Participant,
        changed_attributes: HashMap<String, String>,
    },
    ActiveSpeakersChanged {
        speakers: Vec<Participant>,
    },
    ConnectionStateChanged(ConnectionState),
    Connected {
        participants_with_tracks: Vec<(RemoteParticipant, Vec<RemoteTrackPublication>)>,
    },
    Disconnected {
        reason: &'static str,
    },
    Reconnecting,
    Reconnected,
}
