use crate::{Client, Subscription, User, UserStore};
use anyhow::Result;
use futures::Future;
use gpui::{AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use rpc::{proto, TypedEnvelope};
use std::sync::Arc;

pub struct ChannelStore {
    channels: Vec<Channel>,
    channel_invitations: Vec<Channel>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    rpc_subscription: Subscription,
}

#[derive(Debug, PartialEq)]
pub struct Channel {
    pub id: u64,
    pub name: String,
    pub parent_id: Option<u64>,
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
            rpc_subscription,
        }
    }

    pub fn channels(&self) -> &[Channel] {
        &self.channels
    }

    pub fn channel_invitations(&self) -> &[Channel] {
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
        let payload = message.payload;
        this.update(&mut cx, |this, cx| {
            this.channels
                .retain(|channel| !payload.remove_channels.contains(&channel.id));
            this.channel_invitations
                .retain(|channel| !payload.remove_channel_invitations.contains(&channel.id));

            for channel in payload.channel_invitations {
                if let Some(existing_channel) = this
                    .channel_invitations
                    .iter_mut()
                    .find(|c| c.id == channel.id)
                {
                    existing_channel.name = channel.name;
                    continue;
                }

                this.channel_invitations.insert(
                    0,
                    Channel {
                        id: channel.id,
                        name: channel.name,
                        parent_id: None,
                    },
                );
            }

            for channel in payload.channels {
                if let Some(existing_channel) =
                    this.channels.iter_mut().find(|c| c.id == channel.id)
                {
                    existing_channel.name = channel.name;
                    continue;
                }

                if let Some(parent_id) = channel.parent_id {
                    if let Some(ix) = this.channels.iter().position(|c| c.id == parent_id) {
                        this.channels.insert(
                            ix + 1,
                            Channel {
                                id: channel.id,
                                name: channel.name,
                                parent_id: Some(parent_id),
                            },
                        );
                    }
                } else {
                    this.channels.insert(
                        0,
                        Channel {
                            id: channel.id,
                            name: channel.name,
                            parent_id: None,
                        },
                    );
                }
            }
            cx.notify();
        });

        Ok(())
    }
}
