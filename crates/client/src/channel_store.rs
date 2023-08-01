use crate::{Client, Subscription, User, UserStore};
use anyhow::Result;
use futures::Future;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use rpc::{proto, TypedEnvelope};
use std::sync::Arc;

pub struct ChannelStore {
    channels: Vec<Arc<Channel>>,
    channel_invitations: Vec<Arc<Channel>>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _rpc_subscription: Subscription,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Channel {
    pub id: u64,
    pub name: String,
    pub parent_id: Option<u64>,
    pub depth: usize,
}

impl Entity for ChannelStore {
    type Event = ();
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

    pub fn create_channel(
        &self,
        name: &str,
        parent_id: Option<u64>,
    ) -> impl Future<Output = Result<u64>> {
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
        channel_id: u64,
        user_id: u64,
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
        channel_id: u64,
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

    pub fn is_channel_invite_pending(&self, channel: &Arc<Channel>) -> bool {
        false
    }

    pub fn remove_member(
        &self,
        channel_id: u64,
        user_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        todo!()
    }

    pub fn channel_members(
        &self,
        channel_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        todo!()
    }

    pub fn add_guest_channel(&self, channel_id: u64) -> Task<Result<()>> {
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
        cx.notify();
    }
}
