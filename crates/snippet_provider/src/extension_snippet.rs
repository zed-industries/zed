use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::{ExtensionHostProxy, ExtensionSnippetProxy};
use gpui::App;

use crate::SnippetRegistry;

pub fn init(cx: &mut App) {
    let proxy = ExtensionHostProxy::default_global(cx);
    proxy.register_snippet_proxy(SnippetRegistryProxy {
        snippet_registry: SnippetRegistry::global(cx),
    });
}

struct SnippetRegistryProxy {
    snippet_registry: Arc<SnippetRegistry>,
}

impl ExtensionSnippetProxy for SnippetRegistryProxy {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()> {
        self.snippet_registry
            .register_snippets(path, snippet_contents)
    }
}
