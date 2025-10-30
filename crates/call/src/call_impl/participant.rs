use anyhow::{Context as _, Result};
use client::{ParticipantIndex, User, proto};
use collections::HashMap;
use gpui::WeakEntity;
use livekit_client::AudioStream;
use project::Project;
use std::sync::Arc;

pub use livekit_client::TrackSid;
pub use livekit_client::{RemoteAudioTrack, RemoteVideoTrack};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParticipantLocation {
    SharedProject { project_id: u64 },
    UnsharedProject,
    External,
}

impl ParticipantLocation {
    pub fn from_proto(location: Option<proto::ParticipantLocation>) -> Result<Self> {
        match location
            .and_then(|l| l.variant)
            .context("participant location was not provided")?
        {
            proto::participant_location::Variant::SharedProject(project) => {
                Ok(Self::SharedProject {
                    project_id: project.id,
                })
            }
            proto::participant_location::Variant::UnsharedProject(_) => Ok(Self::UnsharedProject),
            proto::participant_location::Variant::External(_) => Ok(Self::External),
        }
    }
}

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
    pub location: ParticipantLocation,
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
