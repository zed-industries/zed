use anyhow::{anyhow, Result};
use client::proto;
use gpui::ModelHandle;
use project::Project;

pub enum ParticipantLocation {
    Project { project_id: u64 },
    External,
}

impl ParticipantLocation {
    pub fn from_proto(location: Option<proto::ParticipantLocation>) -> Result<Self> {
        match location.and_then(|l| l.variant) {
            Some(proto::participant_location::Variant::Project(project)) => Ok(Self::Project {
                project_id: project.id,
            }),
            Some(proto::participant_location::Variant::External(_)) => Ok(Self::External),
            None => Err(anyhow!("participant location was not provided")),
        }
    }
}

pub struct RemoteParticipant {
    pub user_id: u64,
    pub projects: Vec<ModelHandle<Project>>,
    pub location: ParticipantLocation,
}
