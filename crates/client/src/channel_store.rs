use crate::{Client, Subscription, User, UserStore};
use anyhow::Result;
use collections::HashMap;
use futures::Future;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use rpc::{proto, TypedEnvelope};
use std::sync::Arc;

pub type ChannelId = u64;
pub type UserId = u64;

pub struct ChannelStore {
    channels: Vec<Arc<Channel>>,
    channel_invitations: Vec<Arc<Channel>>,
    channel_participants: HashMap<ChannelId, Vec<Arc<User>>>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _rpc_subscription: Subscription,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub parent_id: Option<ChannelId>,
    pub depth: usize,
}

impl Entity for ChannelStore {
    type Event = ();
}

pub enum ChannelMemberStatus {
    Invited,
    Member,
}

impl ChannelStore {
    pub fn new(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let rpc_subscription =
            client.add_message_handler(cx.handle(), Self::handle_update_channels);

        Self {
            channels: vec![],
            channel_invitations: vec![],
            channel_participants: Default::default(),
            client,
            user_store,
            _rpc_subscription: rpc_subscription,
        }
    }

    pub fn channels(&self) -> &[Arc<Channel>] {
        &self.channels
    }

    pub fn channel_invitations(&self) -> &[Arc<Channel>] {
        &self.channel_invitations
    }

    pub fn channel_for_id(&self, channel_id: ChannelId) -> Option<Arc<Channel>> {
        self.channels.iter().find(|c| c.id == channel_id).cloned()
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
    ) -> impl Future<Output = Result<ChannelId>> {
        let client = self.client.clone();
        let name = name.to_owned();
        async move {
            Ok(client
                .request(proto::CreateChannel { name, parent_id })
                .await?
                .channel_id)
        }
    }

    pub fn invite_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
        admin: bool,
    ) -> impl Future<Output = Result<()>> {
        let client = self.client.clone();
        async move {
            client
                .request(proto::InviteChannelMember {
                    channel_id,
                    user_id,
                    admin,
                })
                .await?;
            Ok(())
        }
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

    pub fn get_channel_members(
        &self,
        channel_id: ChannelId,
    ) -> impl 'static + Future<Output = Result<HashMap<UserId, ChannelMemberStatus>>> {
        let client = self.client.clone();
        async move {
            let response = client
                .request(proto::GetChannelMembers { channel_id })
                .await?;
            let mut result = HashMap::default();
            for member_id in response.members {
                result.insert(member_id, ChannelMemberStatus::Member);
            }
            for invitee_id in response.invited_members {
                result.insert(invitee_id, ChannelMemberStatus::Invited);
            }
            Ok(result)
        }
    }

    pub fn remove_channel(&self, channel_id: ChannelId) -> impl Future<Output = Result<()>> {
        let client = self.client.clone();
        async move {
            client.request(proto::RemoveChannel { channel_id }).await?;
            Ok(())
        }
    }

    pub fn is_channel_invite_pending(&self, _: &Arc<Channel>) -> bool {
        false
    }

    pub fn remove_member(
        &self,
        channel_id: ChannelId,
        user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        todo!()
    }

    pub fn channel_members(
        &self,
        channel_id: ChannelId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        todo!()
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
        self.channels
            .retain(|channel| !payload.remove_channels.contains(&channel.id));
        self.channel_invitations
            .retain(|channel| !payload.remove_channel_invitations.contains(&channel.id));
        self.channel_participants
            .retain(|channel_id, _| !payload.remove_channels.contains(channel_id));

        for channel in payload.channel_invitations {
            if let Some(existing_channel) = self
                .channel_invitations
                .iter_mut()
                .find(|c| c.id == channel.id)
            {
                Arc::make_mut(existing_channel).name = channel.name;
                continue;
            }

            self.channel_invitations.insert(
                0,
                Arc::new(Channel {
                    id: channel.id,
                    name: channel.name,
                    parent_id: None,
                    depth: 0,
                }),
            );
        }

        for channel in payload.channels {
            if let Some(existing_channel) = self.channels.iter_mut().find(|c| c.id == channel.id) {
                Arc::make_mut(existing_channel).name = channel.name;
                continue;
            }

            if let Some(parent_id) = channel.parent_id {
                if let Some(ix) = self.channels.iter().position(|c| c.id == parent_id) {
                    let depth = self.channels[ix].depth + 1;
                    self.channels.insert(
                        ix + 1,
                        Arc::new(Channel {
                            id: channel.id,
                            name: channel.name,
                            parent_id: Some(parent_id),
                            depth,
                        }),
                    );
                }
            } else {
                self.channels.insert(
                    0,
                    Arc::new(Channel {
                        id: channel.id,
                        name: channel.name,
                        parent_id: None,
                        depth: 0,
                    }),
                );
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
}
