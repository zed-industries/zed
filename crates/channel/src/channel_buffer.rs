use crate::Channel;
use anyhow::Result;
use client::Client;
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle};
use rpc::{proto, TypedEnvelope};
use std::sync::Arc;
use util::ResultExt;

pub(crate) fn init(client: &Arc<Client>) {
    client.add_model_message_handler(ChannelBuffer::handle_update_channel_buffer);
    client.add_model_message_handler(ChannelBuffer::handle_add_channel_buffer_collaborator);
    client.add_model_message_handler(ChannelBuffer::handle_remove_channel_buffer_collaborator);
}

pub struct ChannelBuffer {
    pub(crate) channel: Arc<Channel>,
    connected: bool,
    collaborators: Vec<proto::Collaborator>,
    buffer: ModelHandle<language::Buffer>,
    client: Arc<Client>,
    subscription: Option<client::Subscription>,
}

pub enum Event {
    CollaboratorsChanged,
    Disconnected,
}

impl Entity for ChannelBuffer {
    type Event = Event;

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

        let collaborators = response.collaborators;

        let buffer = cx.add_model(|_| {
            language::Buffer::remote(response.buffer_id, response.replica_id as u16, base_text)
        });
        buffer.update(&mut cx, |buffer, cx| buffer.apply_ops(operations, cx))?;

        let subscription = client.subscribe_to_entity(channel.id)?;

        anyhow::Ok(cx.add_model(|cx| {
            cx.subscribe(&buffer, Self::on_buffer_update).detach();

            Self {
                buffer,
                client,
                connected: true,
                collaborators,
                channel,
                subscription: Some(subscription.set_model(&cx.handle(), &mut cx.to_async())),
            }
        }))
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

    async fn handle_add_channel_buffer_collaborator(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::AddChannelBufferCollaborator>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let collaborator = envelope.payload.collaborator.ok_or_else(|| {
            anyhow::anyhow!(
                "Should have gotten a collaborator in the AddChannelBufferCollaborator message"
            )
        })?;

        this.update(&mut cx, |this, cx| {
            this.collaborators.push(collaborator);
            cx.emit(Event::CollaboratorsChanged);
            cx.notify();
        });

        Ok(())
    }

    async fn handle_remove_channel_buffer_collaborator(
        this: ModelHandle<Self>,
        message: TypedEnvelope<proto::RemoveChannelBufferCollaborator>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.collaborators.retain(|collaborator| {
                if collaborator.peer_id == message.payload.peer_id {
                    this.buffer.update(cx, |buffer, cx| {
                        buffer.remove_peer(collaborator.replica_id as u16, cx)
                    });
                    false
                } else {
                    true
                }
            });
            cx.emit(Event::CollaboratorsChanged);
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

    pub fn buffer(&self) -> ModelHandle<language::Buffer> {
        self.buffer.clone()
    }

    pub fn collaborators(&self) -> &[proto::Collaborator] {
        &self.collaborators
    }

    pub fn channel(&self) -> Arc<Channel> {
        self.channel.clone()
    }

    pub(crate) fn disconnect(&mut self, cx: &mut ModelContext<Self>) {
        if self.connected {
            self.connected = false;
            self.subscription.take();
            cx.emit(Event::Disconnected);
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
