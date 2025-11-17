use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, BackgroundExecutor, Task};
use lsp::{LanguageServer, TextDocumentIdentifier};
use std::sync::Arc;

// Re-export the config type from lsp crate (defined there to avoid circular dependencies)
pub use lsp::VirtualDocumentConfig;

/// Store for virtual document handlers, similar to YarnPathStore.
/// Manages the registration and processing of virtual documents from extensions.
pub struct VirtualDocumentStore {
    handlers: HashMap<String, VirtualDocumentConfig>,
    executor: BackgroundExecutor,
}

impl VirtualDocumentStore {
    pub fn new(cx: &mut App) -> Self {
        Self {
            handlers: HashMap::default(),
            executor: cx.background_executor().clone(),
        }
    }

    /// Registers a virtual document handler for a specific URI scheme.
    /// Called by the extension host when extensions export virtual document configs.
    pub fn register_handler(&mut self, config: VirtualDocumentConfig) {
        self.handlers.insert(config.scheme.clone(), config);
    }

    /// Returns the handler configuration for a given URI scheme.
    pub fn handler_for_scheme(&self, scheme: &str) -> Option<&VirtualDocumentConfig> {
        self.handlers.get(scheme)
    }

    /// Processes a virtual URI and returns a task that fetches the document content.
    /// Similar to YarnPathStore::process_path but for non-file URIs.
    pub fn process_uri(
        &self,
        uri: &lsp::Uri,
        language_server: Arc<LanguageServer>,
    ) -> Option<Task<Result<String>>> {
        let scheme = uri.scheme();
        let config = self.handlers.get(scheme)?;

        let params = TextDocumentIdentifier { uri: uri.clone() };
        let request_method = config.content_request_method.clone();
        let executor = self.executor.clone();

        // Send the custom LSP request to get document contents
        Some(executor.spawn(async move {
            language_server
                .request_custom::<TextDocumentIdentifier, String>(&request_method, params)
                .await
                .into_response()
                .context("failed to get virtual document contents")
        }))
    }

    /// Returns all registered handlers (for debugging/inspection).
    pub fn handlers(&self) -> &HashMap<String, VirtualDocumentConfig> {
        &self.handlers
    }
}
