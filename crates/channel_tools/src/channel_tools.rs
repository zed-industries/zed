mod create_channel_tool;
mod edit_channel_notes_tool;
mod list_channels_tool;
mod move_channel_tool;
mod reorder_channel_tool;
mod schema;
mod streaming_edit_channel_notes_tool;

#[cfg(test)]
mod channel_tools_tests;

pub use create_channel_tool::CreateChannelTool;
pub use edit_channel_notes_tool::EditChannelNotesTool;
pub use list_channels_tool::ListChannelsTool;
pub use move_channel_tool::MoveChannelTool;
pub use reorder_channel_tool::ReorderChannelTool;
pub use streaming_edit_channel_notes_tool::StreamingEditChannelNotesTool;

use assistant_tool::ToolRegistry;
use channel::{Channel, ChannelStore};
use client::ChannelId;
use gpui::{App, Entity};
use std::sync::Arc;

/// Initialize channel tools by registering them with the global ToolRegistry.
/// This should be called after channel::init to ensure ChannelStore is available.
pub fn init(channel_store: Entity<ChannelStore>, cx: &mut App) {
    let registry = ToolRegistry::global(cx);
    registry.register_tool(ListChannelsTool::new(channel_store.clone()));
    registry.register_tool(CreateChannelTool::new(channel_store.clone()));
    registry.register_tool(MoveChannelTool::new(channel_store.clone()));
    registry.register_tool(ReorderChannelTool::new(channel_store.clone()));
    registry.register_tool(EditChannelNotesTool::new(channel_store.clone()));
    registry.register_tool(StreamingEditChannelNotesTool::new(channel_store));
}

/// Helper function to find a channel by name
fn find_channel_by_name(
    channel_store: &Entity<ChannelStore>,
    name: &str,
    cx: &App,
) -> Option<(ChannelId, Arc<Channel>)> {
    let store = channel_store.read(cx);
    store
        .channels()
        .find(|channel| channel.name == name)
        .map(|channel| (channel.id, channel.clone()))
}

/// Visibility options for channels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelVisibility {
    Members,
    Public,
}

impl ChannelVisibility {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "members" | "private" => Some(Self::Members),
            "public" => Some(Self::Public),
            _ => None,
        }
    }
}
