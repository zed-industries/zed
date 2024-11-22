use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::{ExtensionChangeListeners, OnSnippetExtensionChange};
use gpui::AppContext;

use crate::SnippetRegistry;

pub fn init(cx: &AppContext) {
    let extension_change_listeners = ExtensionChangeListeners::global(cx);
    extension_change_listeners.register_snippet_listener(ExtensionSnippetListener {
        snippet_registry: SnippetRegistry::global(cx),
    });
}

struct ExtensionSnippetListener {
    snippet_registry: Arc<SnippetRegistry>,
}

impl OnSnippetExtensionChange for ExtensionSnippetListener {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()> {
        self.snippet_registry
            .register_snippets(path, snippet_contents)
    }
}
