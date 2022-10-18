use anyhow::{anyhow, Result};
use client::{proto, User};
use collections::HashMap;
use gpui::{Task, WeakModelHandle};
use media::core_video::CVImageBuffer;
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
    pub active_project: Option<WeakModelHandle<Project>>,
}

#[derive(Clone)]
pub struct RemoteParticipant {
    pub user: Arc<User>,
    pub projects: Vec<proto::ParticipantProject>,
    pub location: ParticipantLocation,
    pub tracks: HashMap<live_kit_client::Sid, RemoteVideoTrack>,
}

#[derive(Clone)]
pub struct RemoteVideoTrack {
    pub(crate) frame: Option<CVImageBuffer>,
    pub(crate) _live_kit_track: Arc<live_kit_client::RemoteVideoTrack>,
    pub(crate) _maintain_frame: Arc<Task<()>>,
}

impl RemoteVideoTrack {
    pub fn frame(&self) -> Option<&CVImageBuffer> {
        self.frame.as_ref()
    }
}
