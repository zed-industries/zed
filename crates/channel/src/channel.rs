mod channel_buffer;
mod channel_chat;
mod channel_store;

pub use channel_buffer::{ChannelBuffer, ChannelBufferEvent};
pub use channel_chat::{ChannelChat, ChannelChatEvent, ChannelMessage, ChannelMessageId};
pub use channel_store::{
    Channel, ChannelData, ChannelEvent, ChannelId, ChannelMembership, ChannelPath, ChannelStore,
};

use client::Client;
use std::sync::Arc;

#[cfg(test)]
mod channel_store_tests;

pub fn init(client: &Arc<Client>) {
    channel_buffer::init(client);
    channel_chat::init(client);
}
