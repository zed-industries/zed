#![cfg_attr(all(target_os = "windows", target_env = "gnu"), allow(unused))]

use anyhow::{anyhow, Result};
use client::{proto, ParticipantIndex, User};
use collections::HashMap;
use gpui::WeakEntity;
use livekit_client::AudioStream;
use project::Project;
use std::sync::Arc;

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
pub use livekit_client::id::TrackSid;
pub use livekit_client::track::{RemoteAudioTrack, RemoteVideoTrack};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParticipantLocation {
    SharedProject { project_id: u64 },
    UnsharedProject,
    External,
}

impl ParticipantLocation {
    pub fn from_proto(location: Option<proto::ParticipantLocation>) -> Result<Self> {
        match location.and_then(|l| l.variant) {
            Some(proto::participant_location::Variant::SharedProject(project)) => {
                Ok(Self::SharedProject {
                    project_id: project.id,
                })
            }
            Some(proto::participant_location::Variant::UnsharedProject(_)) => {
                Ok(Self::UnsharedProject)
            }
            Some(proto::participant_location::Variant::External(_)) => Ok(Self::External),
            None => Err(anyhow!("participant location was not provided")),
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
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    pub video_tracks: HashMap<TrackSid, RemoteVideoTrack>,
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    pub audio_tracks: HashMap<TrackSid, (RemoteAudioTrack, AudioStream)>,
}

impl RemoteParticipant {
    pub fn has_video_tracks(&self) -> bool {
        #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
        return !self.video_tracks.is_empty();
        #[cfg(all(target_os = "windows", target_env = "gnu"))]
        return false;
    }

    pub fn can_write(&self) -> bool {
        matches!(
            self.role,
            proto::ChannelRole::Admin | proto::ChannelRole::Member
        )
    }
}
