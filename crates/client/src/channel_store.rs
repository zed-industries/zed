use crate::Status;
use crate::{Client, Subscription, User, UserStore};
use anyhow::anyhow;
use anyhow::Result;
use collections::HashMap;
use collections::HashSet;
use futures::Future;
use futures::StreamExt;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use rpc::{proto, TypedEnvelope};
use std::sync::Arc;

pub type ChannelId = u64;
pub type UserId = u64;

pub struct ChannelStore {
    channels_by_id: HashMap<ChannelId, Arc<Channel>>,
    channel_paths: Vec<Vec<ChannelId>>,
    channel_invitations: Vec<Arc<Channel>>,
    channel_participants: HashMap<ChannelId, Vec<Arc<User>>>,
    channels_with_admin_privileges: HashSet<ChannelId>,
    outgoing_invites: HashSet<(ChannelId, UserId)>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _rpc_subscription: Subscription,
    _watch_connection_status: Task<()>,
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

impl ChannelStore {
    pub fn new(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let rpc_subscription =
            client.add_message_handler(cx.handle(), Self::handle_update_channels);

        let mut connection_status = client.status();
        let watch_connection_status = cx.spawn_weak(|this, mut cx| async move {
            while let Some(status) = connection_status.next().await {
                if matches!(status, Status::ConnectionLost | Status::SignedOut) {
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, cx| {
                            this.channels_by_id.clear();
                            this.channel_invitations.clear();
                            this.channel_participants.clear();
                            this.channels_with_admin_privileges.clear();
                            this.channel_paths.clear();
                            this.outgoing_invites.clear();
                            cx.notify();
                        });
                    } else {
                        break;
                    }
                }
            }
        });
        Self {
            channels_by_id: HashMap::default(),
            channel_invitations: Vec::default(),
            channel_paths: Vec::default(),
            channel_participants: Default::default(),
            channels_with_admin_privileges: Default::default(),
            outgoing_invites: Default::default(),
            client,
            user_store,
            _rpc_subscription: rpc_subscription,
            _watch_connection_status: watch_connection_status,
        }
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
                this.update_channels(
                    proto::UpdateChannels {
                        channels: vec![channel],
                        ..Default::default()
                    },
                    cx,
                );

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
                this.update_channels(
                    proto::UpdateChannels {
                        channels: vec![channel],
                        ..Default::default()
                    },
                    cx,
                );

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
        this.update(&mut cx, |this, cx| {
            this.update_channels(message.payload, cx);
        });
        Ok(())
    }

    pub(crate) fn update_channels(
        &mut self,
        payload: proto::UpdateChannels,
        cx: &mut ModelContext<ChannelStore>,
    ) {
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
            }

            for channel in payload.channels {
                if let Some(existing_channel) = self.channels_by_id.get_mut(&channel.id) {
                    let existing_channel = Arc::make_mut(existing_channel);
                    existing_channel.name = channel.name;
                    continue;
                }
                self.channels_by_id.insert(
                    channel.id,
                    Arc::new(Channel {
                        id: channel.id,
                        name: channel.name,
                    }),
                );

                if let Some(parent_id) = channel.parent_id {
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

        let mut all_user_ids = Vec::new();
        let channel_participants = payload.channel_participants;
        for entry in &channel_participants {
            for user_id in entry.participant_user_ids.iter() {
                if let Err(ix) = all_user_ids.binary_search(user_id) {
                    all_user_ids.insert(ix, *user_id);
                }
            }
        }

        // TODO: Race condition if an update channels messages comes in while resolving avatars
        let users = self
            .user_store
            .update(cx, |user_store, cx| user_store.get_users(all_user_ids, cx));
        cx.spawn(|this, mut cx| async move {
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
        })
        .detach();

        cx.notify();
    }

    fn channel_path_sorting_key<'a>(
        path: &'a [ChannelId],
        channels_by_id: &'a HashMap<ChannelId, Arc<Channel>>,
    ) -> impl 'a + Iterator<Item = Option<&'a str>> {
        path.iter()
            .map(|id| Some(channels_by_id.get(id)?.name.as_str()))
    }
}
