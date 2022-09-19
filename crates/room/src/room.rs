mod participant;

use anyhow::Result;
use client::Client;
use gpui::ModelHandle;
use participant::{LocalParticipant, RemoteParticipant};
use project::Project;
use std::sync::Arc;

pub struct Room {
    id: u64,
    local_participant: LocalParticipant,
    remote_participants: Vec<RemoteParticipant>,
    client: Arc<Client>,
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

    pub async fn share(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn unshare(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn mute(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn unmute(&mut self) -> Result<()> {
        todo!()
    }
}
