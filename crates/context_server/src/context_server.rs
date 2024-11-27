pub mod client;
mod context_server_tool;
mod extension_context_server;
pub mod manager;
pub mod protocol;
mod registry;
pub mod types;

use command_palette_hooks::CommandPaletteFilter;
pub use context_server_settings::{ContextServerSettings, ServerCommand, ServerConfig};
use gpui::{actions, AppContext};

pub use crate::context_server_tool::ContextServerTool;
pub use crate::registry::ContextServerFactoryRegistry;

actions!(context_servers, [Restart]);

/// The namespace for the context servers actions.
pub const CONTEXT_SERVERS_NAMESPACE: &'static str = "context_servers";

pub fn init(cx: &mut AppContext) {
    context_server_settings::init(cx);
    ContextServerFactoryRegistry::default_global(cx);
    extension_context_server::init(cx);

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(CONTEXT_SERVERS_NAMESPACE);
    });
}
