mod channel_index;

use crate::{channel_buffer::ChannelBuffer, channel_chat::ChannelChat, ChannelMessage};
use anyhow::{anyhow, Result};
use channel_index::ChannelIndex;
use client::{Client, Subscription, User, UserId, UserStore};
use collections::{hash_map, HashMap, HashSet};
use db::RELEASE_CHANNEL;
use futures::{channel::mpsc, future::Shared, Future, FutureExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task, WeakModelHandle};
use rpc::{
    proto::{self, ChannelVisibility},
    TypedEnvelope,
};
use std::{mem, sync::Arc, time::Duration};
use util::ResultExt;

pub fn init(client: &Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut AppContext) {
    let channel_store =
        cx.add_model(|cx| ChannelStore::new(client.clone(), user_store.clone(), cx));
    cx.set_global(channel_store);
}

pub const RECONNECT_TIMEOUT: Duration = Duration::from_secs(30);

pub type ChannelId = u64;

pub struct ChannelStore {
    pub channel_index: ChannelIndex,
    channel_invitations: Vec<Arc<Channel>>,
    channel_participants: HashMap<ChannelId, Vec<Arc<User>>>,
    outgoing_invites: HashSet<(ChannelId, UserId)>,
    update_channels_tx: mpsc::UnboundedSender<proto::UpdateChannels>,
    opened_buffers: HashMap<ChannelId, OpenedModelHandle<ChannelBuffer>>,
    opened_chats: HashMap<ChannelId, OpenedModelHandle<ChannelChat>>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _rpc_subscription: Subscription,
    _watch_connection_status: Task<Option<()>>,
    disconnect_channel_buffers_task: Option<Task<()>>,
    _update_channels: Task<()>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub visibility: proto::ChannelVisibility,
    pub role: proto::ChannelRole,
    pub unseen_note_version: Option<(u64, clock::Global)>,
    pub unseen_message_id: Option<u64>,
    pub parent_path: Vec<u64>,
}

impl Channel {
    pub fn link(&self) -> String {
        RELEASE_CHANNEL.link_prefix().to_owned()
            + "channel/"
            + &self.slug()
            + "-"
            + &self.id.to_string()
    }

    pub fn slug(&self) -> String {
        let slug: String = self
            .name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect();

        slug.trim_matches(|c| c == '-').to_string()
    }

    pub fn can_edit_notes(&self) -> bool {
        self.role == proto::ChannelRole::Member || self.role == proto::ChannelRole::Admin
    }
}

pub struct ChannelMembership {
    pub user: Arc<User>,
    pub kind: proto::channel_member::Kind,
    pub role: proto::ChannelRole,
}
impl ChannelMembership {
    pub fn sort_key(&self) -> MembershipSortKey {
        MembershipSortKey {
            role_order: match self.role {
                proto::ChannelRole::Admin => 0,
                proto::ChannelRole::Member => 1,
                proto::ChannelRole::Banned => 2,
                proto::ChannelRole::Guest => 3,
            },
            kind_order: match self.kind {
                proto::channel_member::Kind::Member => 0,
                proto::channel_member::Kind::AncestorMember => 1,
                proto::channel_member::Kind::Invitee => 2,
            },
            username_order: self.user.github_login.as_str(),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct MembershipSortKey<'a> {
    role_order: u8,
    kind_order: u8,
    username_order: &'a str,
}

pub enum ChannelEvent {
    ChannelCreated(ChannelId),
    ChannelRenamed(ChannelId),
}

impl Entity for ChannelStore {
    type Event = ChannelEvent;
}

enum OpenedModelHandle<E: Entity> {
    Open(WeakModelHandle<E>),
    Loading(Shared<Task<Result<ModelHandle<E>, Arc<anyhow::Error>>>>),
}

impl ChannelStore {
    pub fn global(cx: &AppContext) -> ModelHandle<Self> {
        cx.global::<ModelHandle<Self>>().clone()
    }

    pub fn new(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let rpc_subscription =
            client.add_message_handler(cx.handle(), Self::handle_update_channels);

        let mut connection_status = client.status();
        let (update_channels_tx, mut update_channels_rx) = mpsc::unbounded();
        let watch_connection_status = cx.spawn_weak(|this, mut cx| async move {
            while let Some(status) = connection_status.next().await {
                let this = this.upgrade(&cx)?;
                match status {
                    client::Status::Connected { .. } => {
                        this.update(&mut cx, |this, cx| this.handle_connect(cx))
                            .await
                            .log_err()?;
                    }
                    client::Status::SignedOut | client::Status::UpgradeRequired => {
                        this.update(&mut cx, |this, cx| this.handle_disconnect(false, cx));
                    }
                    _ => {
                        this.update(&mut cx, |this, cx| this.handle_disconnect(true, cx));
                    }
                }
            }
            Some(())
        });

        Self {
            channel_invitations: Vec::default(),
            channel_index: ChannelIndex::default(),
            channel_participants: Default::default(),
            outgoing_invites: Default::default(),
            opened_buffers: Default::default(),
            opened_chats: Default::default(),
            update_channels_tx,
            client,
            user_store,
            _rpc_subscription: rpc_subscription,
            _watch_connection_status: watch_connection_status,
            disconnect_channel_buffers_task: None,
            _update_channels: cx.spawn_weak(|this, mut cx| async move {
                while let Some(update_channels) = update_channels_rx.next().await {
                    if let Some(this) = this.upgrade(&cx) {
                        let update_task = this.update(&mut cx, |this, cx| {
                            this.update_channels(update_channels, cx)
                        });
                        if let Some(update_task) = update_task {
                            update_task.await.log_err();
                        }
                    }
                }
            }),
        }
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn channel_has_children(&self) -> bool {
        self.channel_index
            .by_id()
            .iter()
            .any(|(_, channel)| channel.parent_path.contains(&channel.id))
    }

    /// Returns the number of unique channels in the store
    pub fn channel_count(&self) -> usize {
        self.channel_index.by_id().len()
    }

    /// Returns the index of a channel ID in the list of unique channels
    pub fn index_of_channel(&self, channel_id: ChannelId) -> Option<usize> {
        self.channel_index
            .by_id()
            .keys()
            .position(|id| *id == channel_id)
    }

    /// Returns an iterator over all unique channels
    pub fn channels(&self) -> impl '_ + Iterator<Item = &Arc<Channel>> {
        self.channel_index.by_id().values()
    }

    /// Iterate over all entries in the channel DAG
    pub fn ordered_channels(&self) -> impl '_ + Iterator<Item = (usize, &Arc<Channel>)> {
        self.channel_index
            .ordered_channels()
            .iter()
            .filter_map(move |id| {
                let channel = self.channel_index.by_id().get(id)?;
                Some((channel.parent_path.len(), channel))
            })
    }

    pub fn channel_at_index(&self, ix: usize) -> Option<&Arc<Channel>> {
        let channel_id = self.channel_index.ordered_channels().get(ix)?;
        self.channel_index.by_id().get(channel_id)
    }

    pub fn channel_at(&self, ix: usize) -> Option<&Arc<Channel>> {
        self.channel_index.by_id().values().nth(ix)
    }

    pub fn has_channel_invitation(&self, channel_id: ChannelId) -> bool {
        self.channel_invitations
            .iter()
            .any(|channel| channel.id == channel_id)
    }

    pub fn channel_invitations(&self) -> &[Arc<Channel>] {
        &self.channel_invitations
    }

    pub fn channel_for_id(&self, channel_id: ChannelId) -> Option<&Arc<Channel>> {
        self.channel_index.by_id().get(&channel_id)
    }

    pub fn has_open_channel_buffer(&self, channel_id: ChannelId, cx: &AppContext) -> bool {
        if let Some(buffer) = self.opened_buffers.get(&channel_id) {
            if let OpenedModelHandle::Open(buffer) = buffer {
                return buffer.upgrade(cx).is_some();
            }
        }
        false
    }

    pub fn open_channel_buffer(
        &mut self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<ChannelBuffer>>> {
        let client = self.client.clone();
        let user_store = self.user_store.clone();
        let channel_store = cx.handle();
        self.open_channel_resource(
            channel_id,
            |this| &mut this.opened_buffers,
            |channel, cx| ChannelBuffer::new(channel, client, user_store, channel_store, cx),
            cx,
        )
    }

    pub fn fetch_channel_messages(
        &self,
        message_ids: Vec<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ChannelMessage>>> {
        let request = if message_ids.is_empty() {
            None
        } else {
            Some(
                self.client
                    .request(proto::GetChannelMessagesById { message_ids }),
            )
        };
        cx.spawn_weak(|this, mut cx| async move {
            if let Some(request) = request {
                let response = request.await?;
                let this = this
                    .upgrade(&cx)
                    .ok_or_else(|| anyhow!("channel store dropped"))?;
                let user_store = this.read_with(&cx, |this, _| this.user_store.clone());
                ChannelMessage::from_proto_vec(response.messages, &user_store, &mut cx).await
            } else {
                Ok(Vec::new())
            }
        })
    }

    pub fn has_channel_buffer_changed(&self, channel_id: ChannelId) -> Option<bool> {
        self.channel_index
            .by_id()
            .get(&channel_id)
            .map(|channel| channel.unseen_note_version.is_some())
    }

    pub fn has_new_messages(&self, channel_id: ChannelId) -> Option<bool> {
        self.channel_index
            .by_id()
            .get(&channel_id)
            .map(|channel| channel.unseen_message_id.is_some())
    }

    pub fn notes_changed(
        &mut self,
        channel_id: ChannelId,
        epoch: u64,
        version: &clock::Global,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_index.note_changed(channel_id, epoch, version);
        cx.notify();
    }

    pub fn new_message(
        &mut self,
        channel_id: ChannelId,
        message_id: u64,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_index.new_message(channel_id, message_id);
        cx.notify();
    }

    pub fn acknowledge_message_id(
        &mut self,
        channel_id: ChannelId,
        message_id: u64,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_index
            .acknowledge_message_id(channel_id, message_id);
        cx.notify();
    }

    pub fn acknowledge_notes_version(
        &mut self,
        channel_id: ChannelId,
        epoch: u64,
        version: &clock::Global,
        cx: &mut ModelContext<Self>,
    ) {
        self.channel_index
            .acknowledge_note_version(channel_id, epoch, version);
        cx.notify();
    }

    pub fn open_channel_chat(
        &mut self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<ChannelChat>>> {
        let client = self.client.clone();
        let user_store = self.user_store.clone();
        let this = cx.handle();
        self.open_channel_resource(
            channel_id,
            |this| &mut this.opened_chats,
            |channel, cx| ChannelChat::new(channel, this, user_store, client, cx),
            cx,
        )
    }

    /// Asynchronously open a given resource associated with a channel.
    ///
    /// Make sure that the resource is only opened once, even if this method
    /// is called multiple times with the same channel id while the first task
    /// is still running.
    fn open_channel_resource<T: Entity, F, Fut>(
        &mut self,
        channel_id: ChannelId,
        get_map: fn(&mut Self) -> &mut HashMap<ChannelId, OpenedModelHandle<T>>,
        load: F,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<T>>>
    where
        F: 'static + FnOnce(Arc<Channel>, AsyncAppContext) -> Fut,
        Fut: Future<Output = Result<ModelHandle<T>>>,
    {
        let task = loop {
            match get_map(self).entry(channel_id) {
                hash_map::Entry::Occupied(e) => match e.get() {
                    OpenedModelHandle::Open(model) => {
                        if let Some(model) = model.upgrade(cx) {
                            break Task::ready(Ok(model)).shared();
                        } else {
                            get_map(self).remove(&channel_id);
                            continue;
                        }
                    }
                    OpenedModelHandle::Loading(task) => {
                        break task.clone();
                    }
                },
                hash_map::Entry::Vacant(e) => {
                    let task = cx
                        .spawn(|this, cx| async move {
                            let channel = this.read_with(&cx, |this, _| {
                                this.channel_for_id(channel_id).cloned().ok_or_else(|| {
                                    Arc::new(anyhow!("no channel for id: {}", channel_id))
                                })
                            })?;

                            load(channel, cx).await.map_err(Arc::new)
                        })
                        .shared();

                    e.insert(OpenedModelHandle::Loading(task.clone()));
                    cx.spawn({
                        let task = task.clone();
                        |this, mut cx| async move {
                            let result = task.await;
                            this.update(&mut cx, |this, _| match result {
                                Ok(model) => {
                                    get_map(this).insert(
                                        channel_id,
                                        OpenedModelHandle::Open(model.downgrade()),
                                    );
                                }
                                Err(_) => {
                                    get_map(this).remove(&channel_id);
                                }
                            });
                        }
                    })
                    .detach();
                    break task;
                }
            }
        };
        cx.foreground()
            .spawn(async move { task.await.map_err(|error| anyhow!("{}", error)) })
    }

    pub fn is_channel_admin(&self, channel_id: ChannelId) -> bool {
        let Some(channel) = self.channel_for_id(channel_id) else {
            return false;
        };
        channel.role == proto::ChannelRole::Admin
    }

    pub fn channel_participants(&self, channel_id: ChannelId) -> &[Arc<User>] {
        self.channel_participants
            .get(&channel_id)
            .map_or(&[], |v| v.as_slice())
    }

    pub fn create_channel(
        &self,
        name: &str,
        parent_id: Option<ChannelId>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ChannelId>> {
        let client = self.client.clone();
        let name = name.trim_start_matches("#").to_owned();
        cx.spawn(|this, mut cx| async move {
            let response = client
                .request(proto::CreateChannel { name, parent_id })
                .await?;

            let channel = response
                .channel
                .ok_or_else(|| anyhow!("missing channel in response"))?;
            let channel_id = channel.id;

            // let parent_edge = if let Some(parent_id) = parent_id {
            //     vec![ChannelEdge {
            //         channel_id: channel.id,
            //         parent_id,
            //     }]
            // } else {
            //     vec![]
            // };

            this.update(&mut cx, |this, cx| {
                let task = this.update_channels(
                    proto::UpdateChannels {
                        channels: vec![channel],
                        ..Default::default()
                    },
                    cx,
                );
                assert!(task.is_none());

                // This event is emitted because the collab panel wants to clear the pending edit state
                // before this frame is rendered. But we can't guarantee that the collab panel's future
                // will resolve before this flush_effects finishes. Synchronously emitting this event
                // ensures that the collab panel will observe this creation before the frame completes
                cx.emit(ChannelEvent::ChannelCreated(channel_id));
            });

            Ok(channel_id)
        })
    }

    pub fn move_channel(
        &mut self,
        channel_id: ChannelId,
        to: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(|_, _| async move {
            let _ = client
                .request(proto::MoveChannel { channel_id, to })
                .await?;

            Ok(())
        })
    }

    pub fn set_channel_visibility(
        &mut self,
        channel_id: ChannelId,
        visibility: ChannelVisibility,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(|_, _| async move {
            let _ = client
                .request(proto::SetChannelVisibility {
                    channel_id,
                    visibility: visibility.into(),
                })
                .await?;

            Ok(())
        })
    }

    pub fn invite_member(
        &mut self,
        channel_id: ChannelId,
        user_id: UserId,
        role: proto::ChannelRole,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.outgoing_invites.insert((channel_id, user_id)) {
            return Task::ready(Err(anyhow!("invite request already in progress")));
        }

        cx.notify();
        let client = self.client.clone();
        cx.spawn(|this, mut cx| async move {
            let result = client
                .request(proto::InviteChannelMember {
                    channel_id,
                    user_id,
                    role: role.into(),
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.outgoing_invites.remove(&(channel_id, user_id));
                cx.notify();
            });

            result?;

            Ok(())
        })
    }

    pub fn remove_member(
        &mut self,
        channel_id: ChannelId,
        user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.outgoing_invites.insert((channel_id, user_id)) {
            return Task::ready(Err(anyhow!("invite request already in progress")));
        }

        cx.notify();
        let client = self.client.clone();
        cx.spawn(|this, mut cx| async move {
            let result = client
                .request(proto::RemoveChannelMember {
                    channel_id,
                    user_id,
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.outgoing_invites.remove(&(channel_id, user_id));
                cx.notify();
            });
            result?;
            Ok(())
        })
    }

    pub fn set_member_role(
        &mut self,
        channel_id: ChannelId,
        user_id: UserId,
        role: proto::ChannelRole,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.outgoing_invites.insert((channel_id, user_id)) {
            return Task::ready(Err(anyhow!("member request already in progress")));
        }

        cx.notify();
        let client = self.client.clone();
        cx.spawn(|this, mut cx| async move {
            let result = client
                .request(proto::SetChannelMemberRole {
                    channel_id,
                    user_id,
                    role: role.into(),
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.outgoing_invites.remove(&(channel_id, user_id));
                cx.notify();
            });

            result?;
            Ok(())
        })
    }

    pub fn rename(
        &mut self,
        channel_id: ChannelId,
        new_name: &str,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        let name = new_name.to_string();
        cx.spawn(|this, mut cx| async move {
            let channel = client
                .request(proto::RenameChannel { channel_id, name })
                .await?
                .channel
                .ok_or_else(|| anyhow!("missing channel in response"))?;
            this.update(&mut cx, |this, cx| {
                let task = this.update_channels(
                    proto::UpdateChannels {
                        channels: vec![channel],
                        ..Default::default()
                    },
                    cx,
                );
                assert!(task.is_none());

                // This event is emitted because the collab panel wants to clear the pending edit state
                // before this frame is rendered. But we can't guarantee that the collab panel's future
                // will resolve before this flush_effects finishes. Synchronously emitting this event
                // ensures that the collab panel will observe this creation before the frame complete
                cx.emit(ChannelEvent::ChannelRenamed(channel_id))
            });
            Ok(())
        })
    }

    pub fn respond_to_channel_invite(
        &mut self,
        channel_id: ChannelId,
        accept: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.background().spawn(async move {
            client
                .request(proto::RespondToChannelInvite { channel_id, accept })
                .await?;
            Ok(())
        })
    }

    pub fn get_channel_member_details(
        &self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ChannelMembership>>> {
        let client = self.client.clone();
        let user_store = self.user_store.downgrade();
        cx.spawn(|_, mut cx| async move {
            let response = client
                .request(proto::GetChannelMembers { channel_id })
                .await?;

            let user_ids = response.members.iter().map(|m| m.user_id).collect();
            let user_store = user_store
                .upgrade(&cx)
                .ok_or_else(|| anyhow!("user store dropped"))?;
            let users = user_store
                .update(&mut cx, |user_store, cx| user_store.get_users(user_ids, cx))
                .await?;

            Ok(users
                .into_iter()
                .zip(response.members)
                .filter_map(|(user, member)| {
                    Some(ChannelMembership {
                        user,
                        role: member.role(),
                        kind: member.kind(),
                    })
                })
                .collect())
        })
    }

    pub fn remove_channel(&self, channel_id: ChannelId) -> impl Future<Output = Result<()>> {
        let client = self.client.clone();
        async move {
            client.request(proto::DeleteChannel { channel_id }).await?;
            Ok(())
        }
    }

    pub fn has_pending_channel_invite_response(&self, _: &Arc<Channel>) -> bool {
        false
    }

    pub fn has_pending_channel_invite(&self, channel_id: ChannelId, user_id: UserId) -> bool {
        self.outgoing_invites.contains(&(channel_id, user_id))
    }

    async fn handle_update_channels(
        this: ModelHandle<Self>,
        message: TypedEnvelope<proto::UpdateChannels>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            this.update_channels_tx
                .unbounded_send(message.payload)
                .unwrap();
        });
        Ok(())
    }

    fn handle_connect(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.channel_index.clear();
        self.channel_invitations.clear();
        self.channel_participants.clear();
        self.channel_index.clear();
        self.outgoing_invites.clear();
        self.disconnect_channel_buffers_task.take();

        for chat in self.opened_chats.values() {
            if let OpenedModelHandle::Open(chat) = chat {
                if let Some(chat) = chat.upgrade(cx) {
                    chat.update(cx, |chat, cx| {
                        chat.rejoin(cx);
                    });
                }
            }
        }

        let mut buffer_versions = Vec::new();
        for buffer in self.opened_buffers.values() {
            if let OpenedModelHandle::Open(buffer) = buffer {
                if let Some(buffer) = buffer.upgrade(cx) {
                    let channel_buffer = buffer.read(cx);
                    let buffer = channel_buffer.buffer().read(cx);
                    buffer_versions.push(proto::ChannelBufferVersion {
                        channel_id: channel_buffer.channel_id,
                        epoch: channel_buffer.epoch(),
                        version: language::proto::serialize_version(&buffer.version()),
                    });
                }
            }
        }

        if buffer_versions.is_empty() {
            return Task::ready(Ok(()));
        }

        let response = self.client.request(proto::RejoinChannelBuffers {
            buffers: buffer_versions,
        });

        cx.spawn(|this, mut cx| async move {
            let mut response = response.await?;

            this.update(&mut cx, |this, cx| {
                this.opened_buffers.retain(|_, buffer| match buffer {
                    OpenedModelHandle::Open(channel_buffer) => {
                        let Some(channel_buffer) = channel_buffer.upgrade(cx) else {
                            return false;
                        };

                        channel_buffer.update(cx, |channel_buffer, cx| {
                            let channel_id = channel_buffer.channel_id;
                            if let Some(remote_buffer) = response
                                .buffers
                                .iter_mut()
                                .find(|buffer| buffer.channel_id == channel_id)
                            {
                                let channel_id = channel_buffer.channel_id;
                                let remote_version =
                                    language::proto::deserialize_version(&remote_buffer.version);

                                channel_buffer.replace_collaborators(
                                    mem::take(&mut remote_buffer.collaborators),
                                    cx,
                                );

                                let operations = channel_buffer
                                    .buffer()
                                    .update(cx, |buffer, cx| {
                                        let outgoing_operations =
                                            buffer.serialize_ops(Some(remote_version), cx);
                                        let incoming_operations =
                                            mem::take(&mut remote_buffer.operations)
                                                .into_iter()
                                                .map(language::proto::deserialize_operation)
                                                .collect::<Result<Vec<_>>>()?;
                                        buffer.apply_ops(incoming_operations, cx)?;
                                        anyhow::Ok(outgoing_operations)
                                    })
                                    .log_err();

                                if let Some(operations) = operations {
                                    let client = this.client.clone();
                                    cx.background()
                                        .spawn(async move {
                                            let operations = operations.await;
                                            for chunk in
                                                language::proto::split_operations(operations)
                                            {
                                                client
                                                    .send(proto::UpdateChannelBuffer {
                                                        channel_id,
                                                        operations: chunk,
                                                    })
                                                    .ok();
                                            }
                                        })
                                        .detach();
                                    return true;
                                }
                            }

                            channel_buffer.disconnect(cx);
                            false
                        })
                    }
                    OpenedModelHandle::Loading(_) => true,
                });
            });
            anyhow::Ok(())
        })
    }

    fn handle_disconnect(&mut self, wait_for_reconnect: bool, cx: &mut ModelContext<Self>) {
        cx.notify();

        self.disconnect_channel_buffers_task.get_or_insert_with(|| {
            cx.spawn_weak(|this, mut cx| async move {
                if wait_for_reconnect {
                    cx.background().timer(RECONNECT_TIMEOUT).await;
                }

                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        for (_, buffer) in this.opened_buffers.drain() {
                            if let OpenedModelHandle::Open(buffer) = buffer {
                                if let Some(buffer) = buffer.upgrade(cx) {
                                    buffer.update(cx, |buffer, cx| buffer.disconnect(cx));
                                }
                            }
                        }
                    });
                }
            })
        });
    }

    pub(crate) fn update_channels(
        &mut self,
        payload: proto::UpdateChannels,
        cx: &mut ModelContext<ChannelStore>,
    ) -> Option<Task<Result<()>>> {
        if !payload.remove_channel_invitations.is_empty() {
            self.channel_invitations
                .retain(|channel| !payload.remove_channel_invitations.contains(&channel.id));
        }
        for channel in payload.channel_invitations {
            match self
                .channel_invitations
                .binary_search_by_key(&channel.id, |c| c.id)
            {
                Ok(ix) => Arc::make_mut(&mut self.channel_invitations[ix]).name = channel.name,
                Err(ix) => self.channel_invitations.insert(
                    ix,
                    Arc::new(Channel {
                        id: channel.id,
                        visibility: channel.visibility(),
                        role: channel.role(),
                        name: channel.name,
                        unseen_note_version: None,
                        unseen_message_id: None,
                        parent_path: channel.parent_path,
                    }),
                ),
            }
        }

        let channels_changed = !payload.channels.is_empty()
            || !payload.delete_channels.is_empty()
            || !payload.unseen_channel_messages.is_empty()
            || !payload.unseen_channel_buffer_changes.is_empty();

        if channels_changed {
            if !payload.delete_channels.is_empty() {
                self.channel_index.delete_channels(&payload.delete_channels);
                self.channel_participants
                    .retain(|channel_id, _| !&payload.delete_channels.contains(channel_id));

                for channel_id in &payload.delete_channels {
                    let channel_id = *channel_id;
                    if payload
                        .channels
                        .iter()
                        .any(|channel| channel.id == channel_id)
                    {
                        continue;
                    }
                    if let Some(OpenedModelHandle::Open(buffer)) =
                        self.opened_buffers.remove(&channel_id)
                    {
                        if let Some(buffer) = buffer.upgrade(cx) {
                            buffer.update(cx, ChannelBuffer::disconnect);
                        }
                    }
                }
            }

            let mut index = self.channel_index.bulk_insert();
            for channel in payload.channels {
                let id = channel.id;
                let channel_changed = index.insert(channel);

                if channel_changed {
                    if let Some(OpenedModelHandle::Open(buffer)) = self.opened_buffers.get(&id) {
                        if let Some(buffer) = buffer.upgrade(cx) {
                            buffer.update(cx, ChannelBuffer::channel_changed);
                        }
                    }
                }
            }

            for unseen_buffer_change in payload.unseen_channel_buffer_changes {
                let version = language::proto::deserialize_version(&unseen_buffer_change.version);
                index.note_changed(
                    unseen_buffer_change.channel_id,
                    unseen_buffer_change.epoch,
                    &version,
                );
            }

            for unseen_channel_message in payload.unseen_channel_messages {
                index.new_messages(
                    unseen_channel_message.channel_id,
                    unseen_channel_message.message_id,
                );
            }
        }

        cx.notify();
        if payload.channel_participants.is_empty() {
            return None;
        }

        let mut all_user_ids = Vec::new();
        let channel_participants = payload.channel_participants;
        for entry in &channel_participants {
            for user_id in entry.participant_user_ids.iter() {
                if let Err(ix) = all_user_ids.binary_search(user_id) {
                    all_user_ids.insert(ix, *user_id);
                }
            }
        }

        let users = self
            .user_store
            .update(cx, |user_store, cx| user_store.get_users(all_user_ids, cx));
        Some(cx.spawn(|this, mut cx| async move {
            let users = users.await?;

            this.update(&mut cx, |this, cx| {
                for entry in &channel_participants {
                    let mut participants: Vec<_> = entry
                        .participant_user_ids
                        .iter()
                        .filter_map(|user_id| {
                            users
                                .binary_search_by_key(&user_id, |user| &user.id)
                                .ok()
                                .map(|ix| users[ix].clone())
                        })
                        .collect();

                    participants.sort_by_key(|u| u.id);

                    this.channel_participants
                        .insert(entry.channel_id, participants);
                }

                cx.notify();
            });
            anyhow::Ok(())
        }))
    }
}
