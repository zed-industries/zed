mod extension_dap_adapter;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use dap::DapRegistry;
use extension::{
    ExtensionDebugAdapterProviderProxy, ExtensionGrammarProxy, ExtensionHostProxy,
    ExtensionLanguageProxy,
};
use language::{LanguageMatcher, LanguageName, LanguageRegistry, LoadedLanguage};

pub fn init(extension_host_proxy: Arc<ExtensionHostProxy>, cx: &mut App) {
    let language_server_registry_proxy = DebugAdapterRegistryProxy::new(cx);
    extension_host_proxy.register_debug_adapter_proxy(proxy);
}

#[derive(Clone)]
struct DebugAdapterRegistryProxy {
    debug_adapter_registry: DapRegistry,
}

impl DebugAdapterRegistryProxy {
    fn new(cx: &mut App) -> Self {
        Self {
            debug_adapter_registry: DapRegistry::global(cx),
        }
    }
}

impl ExtensionDebugAdapterProviderProxy for DebugAdapterRegistryProxy {
    fn get_binary(&self, extension: Arc<dyn extension::Extension>, debug_adapter_name: Arc<str>) {
        self.debug_adapter_registry
            .add_adapter(ExtensionDapAdapter::new(extension, debug_adapter_name));
    }
}
