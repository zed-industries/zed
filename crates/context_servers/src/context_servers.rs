use command_palette_hooks::CommandPaletteFilter;
use gpui::{actions, AppContext, Model};
use settings::{Settings, SettingsStore};

use crate::manager::{ContextServerManager, ContextServerSettings};

pub mod client;
pub mod manager;
pub mod protocol;
pub mod types;

actions!(context_servers, [Restart]);

/// The namespace for the context servers actions.
const CONTEXT_SERVERS_NAMESPACE: &'static str = "context_servers";

pub fn init(cx: &mut AppContext) {
    ContextServerSettings::register(cx);

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(CONTEXT_SERVERS_NAMESPACE);
    });
}

pub fn watch_context_server_settings(
    context_server_manager: Model<ContextServerManager>,
    cx: &mut AppContext,
) {
    cx.observe_global::<SettingsStore>(move |cx| {
        context_server_manager.update(cx, |this, cx| {
            this.maintain_servers(cx);

            let has_any_context_servers = !this.servers().is_empty();
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                if has_any_context_servers {
                    filter.show_namespace(CONTEXT_SERVERS_NAMESPACE);
                } else {
                    filter.hide_namespace(CONTEXT_SERVERS_NAMESPACE);
                }
            });
        })
    })
    .detach();
}
