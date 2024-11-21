use anyhow::{anyhow, Result};
use client::ParticipantIndex;
use client::{proto, User};
use collections::HashMap;
use gpui::WeakModel;
pub use live_kit_client::Frame;
pub use live_kit_client::{RemoteAudioTrack, RemoteVideoTrack};
use project::Project;
use std::sync::Arc;

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

#[derive(Clone, Debug)]
pub struct RemoteParticipant {
    pub user: Arc<User>,
    pub peer_id: proto::PeerId,
    pub role: proto::ChannelRole,
    pub projects: Vec<proto::ParticipantProject>,
    pub location: ParticipantLocation,
    pub participant_index: ParticipantIndex,
    pub muted: bool,
    pub speaking: bool,
    pub video_tracks: HashMap<live_kit_client::Sid, Arc<RemoteVideoTrack>>,
    pub audio_tracks: HashMap<live_kit_client::Sid, Arc<RemoteAudioTrack>>,
}
