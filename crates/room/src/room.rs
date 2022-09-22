mod participant;

use anyhow::Result;
use client::{Client, PeerId};
use gpui::{Entity, ModelHandle};
use participant::{LocalParticipant, RemoteParticipant};
use project::Project;
use std::{collections::HashMap, sync::Arc};

pub enum Event {
    PeerChangedActiveProject,
}

pub struct Room {
    id: u64,
    local_participant: LocalParticipant,
    remote_participants: HashMap<PeerId, RemoteParticipant>,
    client: Arc<Client>,
}

impl Entity for Room {
    type Event = Event;
}

impl Room {
    pub async fn create(client: Arc<Client>) -> Result<u64> {
        todo!()
    }

    pub async fn join(id: u64, client: Arc<Client>) -> Result<Self> {
        todo!()
    }

    pub async fn invite(&mut self, user_id: u64) -> Result<()> {
        todo!()
    }

    pub async fn publish_project(&mut self, project: ModelHandle<Project>) -> Result<()> {
        todo!()
    }

    pub async fn unpublish_project(&mut self, project: ModelHandle<Project>) -> Result<()> {
        todo!()
    }

    pub async fn set_active_project(
        &mut self,
        project: Option<&ModelHandle<Project>>,
    ) -> Result<()> {
        todo!()
    }

    pub async fn mute(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn unmute(&mut self) -> Result<()> {
        todo!()
    }
}
