pub mod client;
pub mod manager;
pub mod protocol;
mod registry;
pub mod types;

use command_palette_hooks::CommandPaletteFilter;
use gpui::{actions, AppContext};
use settings::Settings;

use crate::manager::ContextServerSettings;
pub use crate::registry::ContextServerFactoryRegistry;

actions!(context_servers, [Restart]);

/// The namespace for the context servers actions.
pub const CONTEXT_SERVERS_NAMESPACE: &'static str = "context_servers";

pub fn init(cx: &mut AppContext) {
    ContextServerSettings::register(cx);
    ContextServerFactoryRegistry::default_global(cx);

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(CONTEXT_SERVERS_NAMESPACE);
    });
}
