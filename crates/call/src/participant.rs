use anyhow::{anyhow, Result};
use client::{proto, User};
use collections::HashMap;
use gpui::WeakModelHandle;
pub use live_kit_client::Frame;
use project::Project;
use std::{fmt, sync::Arc};

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

#[derive(Clone, Debug)]
pub struct RemoteParticipant {
    pub user: Arc<User>,
    pub projects: Vec<proto::ParticipantProject>,
    pub location: ParticipantLocation,
    pub tracks: HashMap<live_kit_client::Sid, Arc<RemoteVideoTrack>>,
}

#[derive(Clone)]
pub struct RemoteVideoTrack {
    pub(crate) live_kit_track: Arc<live_kit_client::RemoteVideoTrack>,
}

impl fmt::Debug for RemoteVideoTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RemoteVideoTrack").finish()
    }
}

impl RemoteVideoTrack {
    pub fn frames(&self) -> async_broadcast::Receiver<Frame> {
        self.live_kit_track.frames()
    }
}
