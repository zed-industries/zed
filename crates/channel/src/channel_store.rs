use crate::channel_buffer::ChannelBuffer;
use anyhow::{anyhow, Result};
use client::{Client, Subscription, User, UserId, UserStore};
use collections::{hash_map, HashMap, HashSet};
use futures::{channel::mpsc, future::Shared, Future, FutureExt, StreamExt};
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, Task, WeakModelHandle};
use rpc::{proto, TypedEnvelope};
use std::{mem, sync::Arc, time::Duration};
use util::ResultExt;

pub const RECONNECT_TIMEOUT: Duration = Duration::from_secs(30);

pub type ChannelId = u64;

pub struct ChannelStore {
    channels_by_id: HashMap<ChannelId, Arc<Channel>>,
    channel_paths: Vec<Vec<ChannelId>>,
    channel_invitations: Vec<Arc<Channel>>,
    channel_participants: HashMap<ChannelId, Vec<Arc<User>>>,
    channels_with_admin_privileges: HashSet<ChannelId>,
    outgoing_invites: HashSet<(ChannelId, UserId)>,
    update_channels_tx: mpsc::UnboundedSender<proto::UpdateChannels>,
    opened_buffers: HashMap<ChannelId, OpenedChannelBuffer>,
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
}

pub struct ChannelMembership {
    pub user: Arc<User>,
    pub kind: proto::channel_member::Kind,
    pub admin: bool,
}

pub enum ChannelEvent {
    ChannelCreated(ChannelId),
    ChannelRenamed(ChannelId),
}

impl Entity for ChannelStore {
    type Event = ChannelEvent;
}

pub enum ChannelMemberStatus {
    Invited,
    Member,
    NotMember,
}

enum OpenedChannelBuffer {
    Open(WeakModelHandle<ChannelBuffer>),
    Loading(Shared<Task<Result<ModelHandle<ChannelBuffer>, Arc<anyhow::Error>>>>),
}

impl ChannelStore {
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
                if status.is_connected() {
                    this.update(&mut cx, |this, cx| this.handle_connect(cx))
                        .await
                        .log_err()?;
                } else {
                    this.update(&mut cx, |this, cx| this.handle_disconnect(cx));
                }
            }
            Some(())
        });

        Self {
            channels_by_id: HashMap::default(),
            channel_invitations: Vec::default(),
            channel_paths: Vec::default(),
            channel_participants: Default::default(),
            channels_with_admin_privileges: Default::default(),
            outgoing_invites: Default::default(),
            opened_buffers: Default::default(),
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

    pub fn has_children(&self, channel_id: ChannelId) -> bool {
        self.channel_paths.iter().any(|path| {
            if let Some(ix) = path.iter().position(|id| *id == channel_id) {
                path.len() > ix + 1
            } else {
                false
            }
        })
    }

    pub fn channel_count(&self) -> usize {
        self.channel_paths.len()
    }

    pub fn channels(&self) -> impl '_ + Iterator<Item = (usize, &Arc<Channel>)> {
        self.channel_paths.iter().map(move |path| {
            let id = path.last().unwrap();
            let channel = self.channel_for_id(*id).unwrap();
            (path.len() - 1, channel)
        })
    }

    pub fn channel_at_index(&self, ix: usize) -> Option<(usize, &Arc<Channel>)> {
        let path = self.channel_paths.get(ix)?;
        let id = path.last().unwrap();
        let channel = self.channel_for_id(*id).unwrap();
        Some((path.len() - 1, channel))
    }

    pub fn channel_invitations(&self) -> &[Arc<Channel>] {
        &self.channel_invitations
    }

    pub fn channel_for_id(&self, channel_id: ChannelId) -> Option<&Arc<Channel>> {
        self.channels_by_id.get(&channel_id)
    }

    pub fn open_channel_buffer(
        &mut self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<ChannelBuffer>>> {
        // Make sure that a given channel buffer is only opened once per
        // app instance, even if this method is called multiple times
        // with the same channel id while the first task is still running.
        let task = loop {
            match self.opened_buffers.entry(channel_id) {
                hash_map::Entry::Occupied(e) => match e.get() {
                    OpenedChannelBuffer::Open(buffer) => {
                        if let Some(buffer) = buffer.upgrade(cx) {
                            break Task::ready(Ok(buffer)).shared();
                        } else {
                            self.opened_buffers.remove(&channel_id);
                            continue;
                        }
                    }
                    OpenedChannelBuffer::Loading(task) => break task.clone(),
                },
                hash_map::Entry::Vacant(e) => {
                    let client = self.client.clone();
                    let task = cx
                        .spawn(|this, cx| async move {
                            let channel = this.read_with(&cx, |this, _| {
                                this.channel_for_id(channel_id).cloned().ok_or_else(|| {
                                    Arc::new(anyhow!("no channel for id: {}", channel_id))
                                })
                            })?;

                            ChannelBuffer::new(channel, client, cx)
                                .await
                                .map_err(Arc::new)
                        })
                        .shared();
                    e.insert(OpenedChannelBuffer::Loading(task.clone()));
                    cx.spawn({
                        let task = task.clone();
                        |this, mut cx| async move {
                            let result = task.await;
                            this.update(&mut cx, |this, cx| match result {
                                Ok(buffer) => {
                                    cx.observe_release(&buffer, move |this, _, _| {
                                        this.opened_buffers.remove(&channel_id);
                                    })
                                    .detach();
                                    this.opened_buffers.insert(
                                        channel_id,
                                        OpenedChannelBuffer::Open(buffer.downgrade()),
                                    );
                                }
                                Err(error) => {
                                    log::error!("failed to open channel buffer {error:?}");
                                    this.opened_buffers.remove(&channel_id);
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

    pub fn is_user_admin(&self, channel_id: ChannelId) -> bool {
        self.channel_paths.iter().any(|path| {
            if let Some(ix) = path.iter().position(|id| *id == channel_id) {
                path[..=ix]
                    .iter()
                    .any(|id| self.channels_with_admin_privileges.contains(id))
            } else {
                false
            }
        })
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
            let channel = client
                .request(proto::CreateChannel { name, parent_id })
                .await?
                .channel
                .ok_or_else(|| anyhow!("missing channel in response"))?;

            let channel_id = channel.id;

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

    pub fn invite_member(
        &mut self,
        channel_id: ChannelId,
        user_id: UserId,
        admin: bool,
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
                    admin,
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

    pub fn set_member_admin(
        &mut self,
        channel_id: ChannelId,
        user_id: UserId,
        admin: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        if !self.outgoing_invites.insert((channel_id, user_id)) {
            return Task::ready(Err(anyhow!("member request already in progress")));
        }

        cx.notify();
        let client = self.client.clone();
        cx.spawn(|this, mut cx| async move {
            let result = client
                .request(proto::SetChannelMemberAdmin {
                    channel_id,
                    user_id,
                    admin,
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
    ) -> impl Future<Output = Result<()>> {
        let client = self.client.clone();
        async move {
            client
                .request(proto::RespondToChannelInvite { channel_id, accept })
                .await?;
            Ok(())
        }
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
                        admin: member.admin,
                        kind: proto::channel_member::Kind::from_i32(member.kind)?,
                    })
                })
                .collect())
        })
    }

    pub fn remove_channel(&self, channel_id: ChannelId) -> impl Future<Output = Result<()>> {
        let client = self.client.clone();
        async move {
            client.request(proto::RemoveChannel { channel_id }).await?;
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
        self.disconnect_channel_buffers_task.take();

        let mut buffer_versions = Vec::new();
        for buffer in self.opened_buffers.values() {
            if let OpenedChannelBuffer::Open(buffer) = buffer {
                if let Some(buffer) = buffer.upgrade(cx) {
                    let channel_buffer = buffer.read(cx);
                    let buffer = channel_buffer.buffer().read(cx);
                    buffer_versions.push(proto::ChannelBufferVersion {
                        channel_id: channel_buffer.channel().id,
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
                    OpenedChannelBuffer::Open(channel_buffer) => {
                        let Some(channel_buffer) = channel_buffer.upgrade(cx) else {
                            return false;
                        };

                        channel_buffer.update(cx, |channel_buffer, cx| {
                            let channel_id = channel_buffer.channel().id;
                            if let Some(remote_buffer) = response
                                .buffers
                                .iter_mut()
                                .find(|buffer| buffer.channel_id == channel_id)
                            {
                                let channel_id = channel_buffer.channel().id;
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
                    OpenedChannelBuffer::Loading(_) => true,
                });
            });
            anyhow::Ok(())
        })
    }

    fn handle_disconnect(&mut self, cx: &mut ModelContext<Self>) {
        self.channels_by_id.clear();
        self.channel_invitations.clear();
        self.channel_participants.clear();
        self.channels_with_admin_privileges.clear();
        self.channel_paths.clear();
        self.outgoing_invites.clear();
        cx.notify();

        self.disconnect_channel_buffers_task.get_or_insert_with(|| {
            cx.spawn_weak(|this, mut cx| async move {
                cx.background().timer(RECONNECT_TIMEOUT).await;
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        for (_, buffer) in this.opened_buffers.drain() {
                            if let OpenedChannelBuffer::Open(buffer) = buffer {
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
                        name: channel.name,
                    }),
                ),
            }
        }

        let channels_changed = !payload.channels.is_empty() || !payload.remove_channels.is_empty();
        if channels_changed {
            if !payload.remove_channels.is_empty() {
                self.channels_by_id
                    .retain(|channel_id, _| !payload.remove_channels.contains(channel_id));
                self.channel_participants
                    .retain(|channel_id, _| !payload.remove_channels.contains(channel_id));
                self.channels_with_admin_privileges
                    .retain(|channel_id| !payload.remove_channels.contains(channel_id));

                for channel_id in &payload.remove_channels {
                    let channel_id = *channel_id;
                    if let Some(OpenedChannelBuffer::Open(buffer)) =
                        self.opened_buffers.remove(&channel_id)
                    {
                        if let Some(buffer) = buffer.upgrade(cx) {
                            buffer.update(cx, ChannelBuffer::disconnect);
                        }
                    }
                }
            }

            for channel_proto in payload.channels {
                if let Some(existing_channel) = self.channels_by_id.get_mut(&channel_proto.id) {
                    Arc::make_mut(existing_channel).name = channel_proto.name;
                } else {
                    let channel = Arc::new(Channel {
                        id: channel_proto.id,
                        name: channel_proto.name,
                    });
                    self.channels_by_id.insert(channel.id, channel.clone());

                    if let Some(parent_id) = channel_proto.parent_id {
                        let mut ix = 0;
                        while ix < self.channel_paths.len() {
                            let path = &self.channel_paths[ix];
                            if path.ends_with(&[parent_id]) {
                                let mut new_path = path.clone();
                                new_path.push(channel.id);
                                self.channel_paths.insert(ix + 1, new_path);
                                ix += 1;
                            }
                            ix += 1;
                        }
                    } else {
                        self.channel_paths.push(vec![channel.id]);
                    }
                }
            }

            self.channel_paths.sort_by(|a, b| {
                let a = Self::channel_path_sorting_key(a, &self.channels_by_id);
                let b = Self::channel_path_sorting_key(b, &self.channels_by_id);
                a.cmp(b)
            });
            self.channel_paths.dedup();
            self.channel_paths.retain(|path| {
                path.iter()
                    .all(|channel_id| self.channels_by_id.contains_key(channel_id))
            });
        }

        for permission in payload.channel_permissions {
            if permission.is_admin {
                self.channels_with_admin_privileges
                    .insert(permission.channel_id);
            } else {
                self.channels_with_admin_privileges
                    .remove(&permission.channel_id);
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

    fn channel_path_sorting_key<'a>(
        path: &'a [ChannelId],
        channels_by_id: &'a HashMap<ChannelId, Arc<Channel>>,
    ) -> impl 'a + Iterator<Item = Option<&'a str>> {
        path.iter()
            .map(|id| Some(channels_by_id.get(id)?.name.as_str()))
    }
}
