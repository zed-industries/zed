use crate::Channel;
use anyhow::Result;
use client::{Client, Collaborator, UserStore};
use collections::HashMap;
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle};
use rpc::{
    proto::{self, PeerId},
    TypedEnvelope,
};
use std::sync::Arc;
use util::ResultExt;

pub(crate) fn init(client: &Arc<Client>) {
    client.add_model_message_handler(ChannelBuffer::handle_update_channel_buffer);
    client.add_model_message_handler(ChannelBuffer::handle_update_channel_buffer_collaborators);
}

pub struct ChannelBuffer {
    pub(crate) channel: Arc<Channel>,
    connected: bool,
    collaborators: HashMap<PeerId, Collaborator>,
    user_store: ModelHandle<UserStore>,
    buffer: ModelHandle<language::Buffer>,
    buffer_epoch: u64,
    client: Arc<Client>,
    subscription: Option<client::Subscription>,
}

pub enum ChannelBufferEvent {
    CollaboratorsChanged,
    Disconnected,
}

impl Entity for ChannelBuffer {
    type Event = ChannelBufferEvent;

    fn release(&mut self, _: &mut AppContext) {
        if self.connected {
            self.client
                .send(proto::LeaveChannelBuffer {
                    channel_id: self.channel.id,
                })
                .log_err();
        }
    }
}

impl ChannelBuffer {
    pub(crate) async fn new(
        channel: Arc<Channel>,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        mut cx: AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let response = client
            .request(proto::JoinChannelBuffer {
                channel_id: channel.id,
            })
            .await?;

        let base_text = response.base_text;
        let operations = response
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;

        let buffer = cx.add_model(|_| {
            language::Buffer::remote(response.buffer_id, response.replica_id as u16, base_text)
        });
        buffer.update(&mut cx, |buffer, cx| buffer.apply_ops(operations, cx))?;

        let subscription = client.subscribe_to_entity(channel.id)?;

        anyhow::Ok(cx.add_model(|cx| {
            cx.subscribe(&buffer, Self::on_buffer_update).detach();

            let mut this = Self {
                buffer,
                buffer_epoch: response.epoch,
                client,
                connected: true,
                collaborators: Default::default(),
                channel,
                subscription: Some(subscription.set_model(&cx.handle(), &mut cx.to_async())),
                user_store,
            };
            this.replace_collaborators(response.collaborators, cx);
            this
        }))
    }

    pub fn user_store(&self) -> &ModelHandle<UserStore> {
        &self.user_store
    }

    pub(crate) fn replace_collaborators(
        &mut self,
        collaborators: Vec<proto::Collaborator>,
        cx: &mut ModelContext<Self>,
    ) {
        let mut new_collaborators = HashMap::default();
        for collaborator in collaborators {
            if let Ok(collaborator) = Collaborator::from_proto(collaborator) {
                new_collaborators.insert(collaborator.peer_id, collaborator);
            }
        }

        for (_, old_collaborator) in &self.collaborators {
            if !new_collaborators.contains_key(&old_collaborator.peer_id) {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.remove_peer(old_collaborator.replica_id as u16, cx)
                });
            }
        }
        self.collaborators = new_collaborators;
        cx.emit(ChannelBufferEvent::CollaboratorsChanged);
        cx.notify();
    }

    async fn handle_update_channel_buffer(
        this: ModelHandle<Self>,
        update_channel_buffer: TypedEnvelope<proto::UpdateChannelBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
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
        this: ModelHandle<Self>,
        message: TypedEnvelope<proto::UpdateChannelBufferCollaborators>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.replace_collaborators(message.payload.collaborators, cx);
            cx.emit(ChannelBufferEvent::CollaboratorsChanged);
            cx.notify();
        });

        Ok(())
    }

    fn on_buffer_update(
        &mut self,
        _: ModelHandle<language::Buffer>,
        event: &language::Event,
        _: &mut ModelContext<Self>,
    ) {
        if let language::Event::Operation(operation) = event {
            let operation = language::proto::serialize_operation(operation);
            self.client
                .send(proto::UpdateChannelBuffer {
                    channel_id: self.channel.id,
                    operations: vec![operation],
                })
                .log_err();
        }
    }

    pub fn epoch(&self) -> u64 {
        self.buffer_epoch
    }

    pub fn buffer(&self) -> ModelHandle<language::Buffer> {
        self.buffer.clone()
    }

    pub fn collaborators(&self) -> &HashMap<PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn channel(&self) -> Arc<Channel> {
        self.channel.clone()
    }

    pub(crate) fn disconnect(&mut self, cx: &mut ModelContext<Self>) {
        log::info!("channel buffer {} disconnected", self.channel.id);
        if self.connected {
            self.connected = false;
            self.subscription.take();
            cx.emit(ChannelBufferEvent::Disconnected);
            cx.notify()
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn replica_id(&self, cx: &AppContext) -> u16 {
        self.buffer.read(cx).replica_id()
    }
}
