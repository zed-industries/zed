pub mod client;
mod context_server_tool;
mod extension_context_server;
pub mod manager;
pub mod protocol;
mod registry;
mod transport;
pub mod types;

use command_palette_hooks::CommandPaletteFilter;
pub use context_server_settings::{ContextServerSettings, ServerCommand, ServerConfig};
use gpui::{App, actions};

pub use crate::context_server_tool::ContextServerTool;
pub use crate::registry::ContextServerDescriptorRegistry;

actions!(context_servers, [Restart]);

/// The namespace for the context servers actions.
pub const CONTEXT_SERVERS_NAMESPACE: &'static str = "context_servers";

pub fn init(cx: &mut App) {
    context_server_settings::init(cx);
    ContextServerDescriptorRegistry::default_global(cx);
    extension_context_server::init(cx);

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(CONTEXT_SERVERS_NAMESPACE);
    });
}
