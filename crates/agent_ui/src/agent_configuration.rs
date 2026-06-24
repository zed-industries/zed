pub mod configure_context_server_modal;
mod manage_profiles_modal;
mod tool_picker;

use std::sync::Arc;

use context_server::ContextServerId;
use extension::ExtensionManifest;
use extension_host::ExtensionStore;
use gpui::App;

pub(crate) use configure_context_server_modal::ConfigureContextServerModal;
pub(crate) use manage_profiles_modal::ManageProfilesModal;

pub(crate) fn resolve_extension_for_context_server(
    id: &ContextServerId,
    cx: &App,
) -> Option<(Arc<str>, Arc<ExtensionManifest>)> {
    ExtensionStore::global(cx)
        .read(cx)
        .installed_extensions()
        .iter()
        .find(|(_, entry)| entry.manifest.context_servers.contains_key(&id.0))
        .map(|(id, entry)| (id.clone(), entry.manifest.clone()))
}
