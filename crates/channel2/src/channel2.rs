mod channel_buffer;
mod channel_chat;
mod channel_store;

use client2::{Client, UserStore};
use gpui2::{AppContext, Model};
use std::sync::Arc;

pub use channel_buffer::{ChannelBuffer, ChannelBufferEvent, ACKNOWLEDGE_DEBOUNCE_INTERVAL};
pub use channel_chat::{
    mentions_to_proto, ChannelChat, ChannelChatEvent, ChannelMessage, ChannelMessageId,
    MessageParams,
};
pub use channel_store::{Channel, ChannelEvent, ChannelId, ChannelMembership, ChannelStore};

#[cfg(test)]
mod channel_store_tests;

pub fn init(client: &Arc<Client>, user_store: Model<UserStore>, cx: &mut AppContext) {
    channel_store::init(client, user_store, cx);
    channel_buffer::init(client);
    channel_chat::init(client);
}
