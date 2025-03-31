mod channel_buffer;
mod channel_chat;
mod channel_store;

use client::{Client, UserStore};
use gpui::{App, Entity};
use std::sync::Arc;

pub use channel_buffer::{ACKNOWLEDGE_DEBOUNCE_INTERVAL, ChannelBuffer, ChannelBufferEvent};
pub use channel_chat::{
    ChannelChat, ChannelChatEvent, ChannelMessage, ChannelMessageId, MessageParams,
    mentions_to_proto,
};
pub use channel_store::{Channel, ChannelEvent, ChannelMembership, ChannelStore};

#[cfg(test)]
mod channel_store_tests;

pub fn init(client: &Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) {
    channel_store::init(client, user_store, cx);
    channel_buffer::init(&client.clone().into());
    channel_chat::init(&client.clone().into());
}
