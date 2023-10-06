mod channel_buffer;
mod channel_chat;
mod channel_store;

use client::{Client, UserStore};
use gpui::{AppContext, ModelHandle};
use std::sync::Arc;

pub use channel_buffer::{ChannelBuffer, ChannelBufferEvent, ACKNOWLEDGE_DEBOUNCE_INTERVAL};
pub use channel_chat::{ChannelChat, ChannelChatEvent, ChannelMessage, ChannelMessageId};
pub use channel_store::{
    Channel, ChannelData, ChannelEvent, ChannelId, ChannelMembership, ChannelPath, ChannelStore,
};

#[cfg(test)]
mod channel_store_tests;

pub fn init(client: &Arc<Client>, user_store: ModelHandle<UserStore>, cx: &mut AppContext) {
    channel_store::init(client, user_store, cx);
    channel_buffer::init(client);
    channel_chat::init(client);
}
