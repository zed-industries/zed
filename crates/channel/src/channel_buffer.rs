use crate::{Channel, ChannelStore};
use anyhow::Result;
use client::{ChannelId, Client, Collaborator, UserStore, ZED_ALWAYS_ACTIVE};
use collections::HashMap;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Task};
use language::proto::serialize_version;
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, PeerId},
};
use std::{sync::Arc, time::Duration};
use text::{BufferId, ReplicaId};
use util::ResultExt;

pub const ACKNOWLEDGE_DEBOUNCE_INTERVAL: Duration = Duration::from_millis(250);

pub(crate) fn init(client: &AnyProtoClient) {
    client.add_entity_message_handler(ChannelBuffer::handle_update_channel_buffer);
    client.add_entity_message_handler(ChannelBuffer::handle_update_channel_buffer_collaborators);
}

pub struct ChannelBuffer {
    pub channel_id: ChannelId,
    connected: bool,
    collaborators: HashMap<PeerId, Collaborator>,
    user_store: Entity<UserStore>,
    channel_store: Entity<ChannelStore>,
    buffer: Entity<language::Buffer>,
    buffer_epoch: u64,
    client: Arc<Client>,
    subscription: Option<client::Subscription>,
    acknowledge_task: Option<Task<Result<()>>>,
}

pub enum ChannelBufferEvent {
    CollaboratorsChanged,
    Disconnected,
    Connected,
    BufferEdited,
    ChannelChanged,
}

impl EventEmitter<ChannelBufferEvent> for ChannelBuffer {}

impl ChannelBuffer {
    pub(crate) async fn new(
        channel: Arc<Channel>,
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        channel_store: Entity<ChannelStore>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Self>> {
        let response = client
            .request(proto::JoinChannelBuffer {
                channel_id: channel.id.0,
            })
            .await?;
        let buffer_id = BufferId::new(response.buffer_id)?;
        let base_text = response.base_text;
        let operations = response
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;

        let buffer = cx.new(|cx| {
            let capability = channel_store.read(cx).channel_capability(channel.id);
            language::Buffer::remote(
                buffer_id,
                ReplicaId::new(response.replica_id as u16),
                capability,
                base_text,
            )
        })?;
        buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx))?;

        let subscription = client.subscribe_to_entity(channel.id.0)?;

        anyhow::Ok(cx.new(|cx| {
            cx.subscribe(&buffer, Self::on_buffer_update).detach();
            cx.on_release(Self::release).detach();
            let mut this = Self {
                buffer,
                buffer_epoch: response.epoch,
                client,
                connected: true,
                collaborators: Default::default(),
                acknowledge_task: None,
                channel_id: channel.id,
                subscription: Some(subscription.set_entity(&cx.entity(), &cx.to_async())),
                user_store,
                channel_store,
            };
            this.replace_collaborators(response.collaborators, cx);
            this
        })?)
    }

    fn release(&mut self, _: &mut App) {
        if self.connected {
            if let Some(task) = self.acknowledge_task.take() {
                task.detach();
            }
            self.client
                .send(proto::LeaveChannelBuffer {
                    channel_id: self.channel_id.0,
                })
                .log_err();
        }
    }

    pub fn connected(&mut self, cx: &mut Context<Self>) {
        self.connected = true;
        if self.subscription.is_none() {
            let Ok(subscription) = self.client.subscribe_to_entity(self.channel_id.0) else {
                return;
            };
            self.subscription = Some(subscription.set_entity(&cx.entity(), &cx.to_async()));
            cx.emit(ChannelBufferEvent::Connected);
        }
    }

    pub fn remote_id(&self, cx: &App) -> BufferId {
        self.buffer.read(cx).remote_id()
    }

    pub fn user_store(&self) -> &Entity<UserStore> {
        &self.user_store
    }

    pub(crate) fn replace_collaborators(
        &mut self,
        collaborators: Vec<proto::Collaborator>,
        cx: &mut Context<Self>,
    ) {
        let mut new_collaborators = HashMap::default();
        for collaborator in collaborators {
            if let Ok(collaborator) = Collaborator::from_proto(collaborator) {
                new_collaborators.insert(collaborator.peer_id, collaborator);
            }
        }

        for old_collaborator in self.collaborators.values() {
            if !new_collaborators.contains_key(&old_collaborator.peer_id) {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.remove_peer(old_collaborator.replica_id, cx)
                });
            }
        }
        self.collaborators = new_collaborators;
        cx.emit(ChannelBufferEvent::CollaboratorsChanged);
        cx.notify();
    }

    async fn handle_update_channel_buffer(
        this: Entity<Self>,
        update_channel_buffer: TypedEnvelope<proto::UpdateChannelBuffer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let ops = update_channel_buffer
            .payload
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;

        this.update(&mut cx, |this, cx| {
            cx.notify();
            this.buffer
                .update(cx, |buffer, cx| buffer.apply_ops(ops, cx))
        })?;

        Ok(())
    }

    async fn handle_update_channel_buffer_collaborators(
        this: Entity<Self>,
        message: TypedEnvelope<proto::UpdateChannelBufferCollaborators>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.replace_collaborators(message.payload.collaborators, cx);
            cx.emit(ChannelBufferEvent::CollaboratorsChanged);
            cx.notify();
        })
    }

    fn on_buffer_update(
        &mut self,
        _: Entity<language::Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            language::BufferEvent::Operation {
                operation,
                is_local: true,
            } => {
                if *ZED_ALWAYS_ACTIVE
                    && let language::Operation::UpdateSelections { selections, .. } = operation
                    && selections.is_empty()
                {
                    return;
                }
                let operation = language::proto::serialize_operation(operation);
                self.client
                    .send(proto::UpdateChannelBuffer {
                        channel_id: self.channel_id.0,
                        operations: vec![operation],
                    })
                    .log_err();
            }
            language::BufferEvent::Edited => {
                cx.emit(ChannelBufferEvent::BufferEdited);
            }
            _ => {}
        }
    }

    pub fn acknowledge_buffer_version(&mut self, cx: &mut Context<ChannelBuffer>) {
        let buffer = self.buffer.read(cx);
        let version = buffer.version();
        let buffer_id = buffer.remote_id().into();
        let client = self.client.clone();
        let epoch = self.epoch();

        self.acknowledge_task = Some(cx.spawn(async move |_, cx| {
            cx.background_executor()
                .timer(ACKNOWLEDGE_DEBOUNCE_INTERVAL)
                .await;
            client
                .send(proto::AckBufferOperation {
                    buffer_id,
                    epoch,
                    version: serialize_version(&version),
                })
                .ok();
            Ok(())
        }));
    }

    pub fn epoch(&self) -> u64 {
        self.buffer_epoch
    }

    pub fn buffer(&self) -> Entity<language::Buffer> {
        self.buffer.clone()
    }

    pub fn collaborators(&self) -> &HashMap<PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn channel(&self, cx: &App) -> Option<Arc<Channel>> {
        self.channel_store
            .read(cx)
            .channel_for_id(self.channel_id)
            .cloned()
    }

    pub(crate) fn disconnect(&mut self, cx: &mut Context<Self>) {
        log::info!("channel buffer {} disconnected", self.channel_id);
        if self.connected {
            self.connected = false;
            self.subscription.take();
            cx.emit(ChannelBufferEvent::Disconnected);
            cx.notify()
        }
    }

    pub(crate) fn channel_changed(&mut self, cx: &mut Context<Self>) {
        cx.emit(ChannelBufferEvent::ChannelChanged);
        cx.notify()
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn replica_id(&self, cx: &App) -> ReplicaId {
        self.buffer.read(cx).replica_id()
    }
}
