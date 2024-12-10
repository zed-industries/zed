use crate::{Channel, ChannelStore};
use anyhow::Result;
use client::{ChannelId, Client, Collaborator, UserStore, ZED_ALWAYS_ACTIVE};
use collections::HashMap;
use gpui::{AppContext, AsyncAppContext, Context, EventEmitter, Model, Task};
use language::proto::serialize_version;
use rpc::{
    proto::{self, PeerId},
    AnyProtoClient, TypedEnvelope,
};
use std::{sync::Arc, time::Duration};
use text::BufferId;
use util::ResultExt;

pub const ACKNOWLEDGE_DEBOUNCE_INTERVAL: Duration = Duration::from_millis(250);

pub(crate) fn init(client: &AnyProtoClient) {
    client.add_model_message_handler(ChannelBuffer::handle_update_channel_buffer);
    client.add_model_message_handler(ChannelBuffer::handle_update_channel_buffer_collaborators);
}

pub struct ChannelBuffer {
    pub channel_id: ChannelId,
    connected: bool,
    collaborators: HashMap<PeerId, Collaborator>,
    user_store: Model<UserStore>,
    channel_store: Model<ChannelStore>,
    buffer: Model<language::Buffer>,
    buffer_epoch: u64,
    client: Arc<Client>,
    subscription: Option<client::Subscription>,
    acknowledge_task: Option<Task<Result<()>>>,
}

pub enum ChannelBufferEvent {
    CollaboratorsChanged,
    Disconnected,
    BufferEdited,
    ChannelChanged,
}

impl EventEmitter<ChannelBufferEvent> for ChannelBuffer {}

impl ChannelBuffer {
    pub(crate) async fn new(
        channel: Arc<Channel>,
        client: Arc<Client>,
        user_store: Model<UserStore>,
        channel_store: Model<ChannelStore>,
        mut cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
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

        let buffer = cx.new_model(|model, cx| {
            let capability = channel_store.read(cx).channel_capability(channel.id);
            language::Buffer::remote(buffer_id, response.replica_id as u16, capability, base_text)
        })?;
        buffer.update(&mut cx, |buffer, model, cx| {
            buffer.apply_ops(operations, model, cx)
        })?;

        let subscription = client.subscribe_to_entity(channel.id.0)?;

        anyhow::Ok(cx.new_model(|model, cx| {
            model
                .subscribe(&buffer, cx, Self::on_buffer_update)
                .detach();
            model.on_release(cx, Self::release).detach();
            let mut this = Self {
                buffer,
                buffer_epoch: response.epoch,
                client,
                connected: true,
                collaborators: Default::default(),
                acknowledge_task: None,
                channel_id: channel.id,
                subscription: Some(subscription.set_model(model, &mut cx.to_async())),
                user_store,
                channel_store,
            };
            this.replace_collaborators(response.collaborators, model, cx);
            this
        })?)
    }

    fn release(&mut self, _: &mut AppContext) {
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

    pub fn remote_id(&self, cx: &AppContext) -> BufferId {
        self.buffer.read(cx).remote_id()
    }

    pub fn user_store(&self) -> &Model<UserStore> {
        &self.user_store
    }

    pub(crate) fn replace_collaborators(
        &mut self,
        collaborators: Vec<proto::Collaborator>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let mut new_collaborators = HashMap::default();
        for collaborator in collaborators {
            if let Ok(collaborator) = Collaborator::from_proto(collaborator) {
                new_collaborators.insert(collaborator.peer_id, collaborator);
            }
        }

        for (_, old_collaborator) in &self.collaborators {
            if !new_collaborators.contains_key(&old_collaborator.peer_id) {
                self.buffer.update(cx, |buffer, model, cx| {
                    buffer.remove_peer(old_collaborator.replica_id, model, cx)
                });
            }
        }
        self.collaborators = new_collaborators;
        model.emit(cx, ChannelBufferEvent::CollaboratorsChanged);
        model.notify(cx);
    }

    async fn handle_update_channel_buffer(
        this: Model<Self>,
        update_channel_buffer: TypedEnvelope<proto::UpdateChannelBuffer>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let ops = update_channel_buffer
            .payload
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;

        this.update(&mut cx, |this, model, cx| {
            model.notify(cx);
            this.buffer
                .update(cx, |buffer, model, cx| buffer.apply_ops(ops, model, cx))
        })?;

        Ok(())
    }

    async fn handle_update_channel_buffer_collaborators(
        this: Model<Self>,
        message: TypedEnvelope<proto::UpdateChannelBufferCollaborators>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, model, cx| {
            this.replace_collaborators(message.payload.collaborators, model, cx);
            model.emit(cx, ChannelBufferEvent::CollaboratorsChanged);
            model.notify(cx);
        })
    }

    fn on_buffer_update(
        &mut self,
        _: Model<language::Buffer>,
        event: &language::BufferEvent,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        match event {
            language::BufferEvent::Operation {
                operation,
                is_local: true,
            } => {
                if *ZED_ALWAYS_ACTIVE {
                    if let language::Operation::UpdateSelections { selections, .. } = operation {
                        if selections.is_empty() {
                            return;
                        }
                    }
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
                model.emit(cx, ChannelBufferEvent::BufferEdited);
            }
            _ => {}
        }
    }

    pub fn acknowledge_buffer_version(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        let buffer = self.buffer.read(cx);
        let version = buffer.version();
        let buffer_id = buffer.remote_id().into();
        let client = self.client.clone();
        let epoch = self.epoch();

        self.acknowledge_task = Some(cx.spawn(move |cx| async move {
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

    pub fn buffer(&self) -> Model<language::Buffer> {
        self.buffer.clone()
    }

    pub fn collaborators(&self) -> &HashMap<PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn channel(&self, cx: &AppContext) -> Option<Arc<Channel>> {
        self.channel_store
            .read(cx)
            .channel_for_id(self.channel_id)
            .cloned()
    }

    pub(crate) fn disconnect(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        log::info!("channel buffer {} disconnected", self.channel_id);
        if self.connected {
            self.connected = false;
            self.subscription.take();
            model.emit(cx, ChannelBufferEvent::Disconnected);
            model.notify(cx)
        }
    }

    pub(crate) fn channel_changed(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        model.emit(cx, ChannelBufferEvent::ChannelChanged);
        model.notify(cx)
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn replica_id(&self, cx: &AppContext) -> u16 {
        self.buffer.read(cx).replica_id()
    }
}
