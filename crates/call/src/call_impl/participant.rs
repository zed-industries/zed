use client::{ParticipantIndex, User, proto};
use collections::HashMap;
use gpui::WeakEntity;
use livekit_client::AudioStream;
use project::Project;
use std::sync::Arc;

pub use livekit_client::TrackSid;
pub use livekit_client::{RemoteAudioTrack, RemoteVideoTrack};

#[derive(Clone, Default)]
pub struct LocalParticipant {
    pub projects: Vec<proto::ParticipantProject>,
    pub active_project: Option<WeakEntity<Project>>,
    pub role: proto::ChannelRole,
}

impl LocalParticipant {
    pub fn can_write(&self) -> bool {
        matches!(
            self.role,
            proto::ChannelRole::Admin | proto::ChannelRole::Member
        )
    }
}

pub struct RemoteParticipant {
    pub user: Arc<User>,
    pub peer_id: proto::PeerId,
    pub role: proto::ChannelRole,
    pub projects: Vec<proto::ParticipantProject>,
    pub location: workspace::ParticipantLocation,
    pub participant_index: ParticipantIndex,
    pub muted: bool,
    pub speaking: bool,
    pub video_tracks: HashMap<TrackSid, RemoteVideoTrack>,
    pub audio_tracks: HashMap<TrackSid, (RemoteAudioTrack, AudioStream)>,
}

impl RemoteParticipant {
    pub fn has_video_tracks(&self) -> bool {
        !self.video_tracks.is_empty()
    }

    pub fn can_write(&self) -> bool {
        matches!(
            self.role,
            proto::ChannelRole::Admin | proto::ChannelRole::Member
        )
    }
}
