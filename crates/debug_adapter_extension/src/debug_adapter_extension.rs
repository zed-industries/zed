mod extension_dap_adapter;
mod extension_locator_adapter;

use std::{path::Path, sync::Arc};

use dap::DapRegistry;
use extension::{ExtensionDebugAdapterProviderProxy, ExtensionHostProxy};
use extension_dap_adapter::ExtensionDapAdapter;
use gpui::App;
use util::ResultExt;

use crate::extension_locator_adapter::ExtensionLocatorAdapter;

pub fn init(extension_host_proxy: Arc<ExtensionHostProxy>, cx: &mut App) {
    let language_server_registry_proxy = DebugAdapterRegistryProxy::new(cx);
    extension_host_proxy.register_debug_adapter_proxy(language_server_registry_proxy);
}

#[derive(Clone)]
struct DebugAdapterRegistryProxy {
    debug_adapter_registry: DapRegistry,
}

impl DebugAdapterRegistryProxy {
    fn new(cx: &mut App) -> Self {
        Self {
            debug_adapter_registry: DapRegistry::global(cx).clone(),
        }
    }
}

impl ExtensionDebugAdapterProviderProxy for DebugAdapterRegistryProxy {
    fn register_debug_adapter(
        &self,
        extension: Arc<dyn extension::Extension>,
        debug_adapter_name: Arc<str>,
        schema_path: &Path,
    ) {
        if let Some(adapter) =
            ExtensionDapAdapter::new(extension, debug_adapter_name, schema_path).log_err()
        {
            self.debug_adapter_registry.add_adapter(Arc::new(adapter));
        }
    }

    fn register_debug_locator(
        &self,
        extension: Arc<dyn extension::Extension>,
        locator_name: Arc<str>,
    ) {
        self.debug_adapter_registry
            .add_locator(Arc::new(ExtensionLocatorAdapter::new(
                extension,
                locator_name,
            )));
    }

    fn unregister_debug_adapter(&self, debug_adapter_name: Arc<str>) {
        self.debug_adapter_registry
            .remove_adapter(&debug_adapter_name);
    }

    fn unregister_debug_locator(&self, locator_name: Arc<str>) {
        self.debug_adapter_registry.remove_locator(&locator_name);
    }
}
