#![cfg_attr(target_os = "windows", allow(unused))]

use anyhow::{anyhow, Result};
use client::{proto, ParticipantIndex, User};
use collections::HashMap;
use gpui::WeakModel;
use live_kit_client::AudioStream;
use project::Project;
use std::sync::Arc;

#[cfg(not(target_os = "windows"))]
pub use live_kit_client::id::TrackSid;
pub use live_kit_client::track::{RemoteAudioTrack, RemoteVideoTrack};

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
    pub active_project: Option<WeakModel<Project>>,
    pub role: proto::ChannelRole,
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
    #[cfg(not(target_os = "windows"))]
    pub video_tracks: HashMap<TrackSid, RemoteVideoTrack>,
    #[cfg(not(target_os = "windows"))]
    pub audio_tracks: HashMap<TrackSid, (RemoteAudioTrack, AudioStream)>,
}

impl RemoteParticipant {
    pub fn has_video_tracks(&self) -> bool {
        #[cfg(not(target_os = "windows"))]
        return !self.video_tracks.is_empty();
        #[cfg(target_os = "windows")]
        return false;
    }
}
