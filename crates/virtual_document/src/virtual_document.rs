use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, BackgroundExecutor, Task};
use lsp::{LanguageServer, TextDocumentIdentifier};
use std::sync::Arc;

// Re-export the config type from lsp crate (defined there to avoid circular dependencies)
pub use lsp::VirtualDocumentConfig;

/// A centralized store for managing virtual document handlers.
///
/// Virtual documents are documents that don't exist as files on disk but are provided
/// by language servers via custom LSP requests. Common examples include:
///
/// - **Java (JDTLS)**: Decompiled `.class` files accessed via `jdt://` URIs
/// - **Rust (rust-analyzer)**: Macro expansions via `rust-analyzer://` URIs
/// - **Go**: Debug adapter sources via `dap-browser://` URIs
///
/// ## Architecture
///
/// The `VirtualDocumentStore` acts as a registry that maps URI schemes to their
/// corresponding handler configurations. When Zed encounters a non-`file://` URI
/// (e.g., during "Go to Definition"), it:
///
/// 1. Looks up the handler for the URI's scheme in this store
/// 2. Uses the handler's `content_request_method` to fetch content from the LSP
/// 3. Creates a read-only buffer with the fetched content
///
/// ## Extension Integration
///
/// Extensions register handlers by implementing the `language_server_virtual_document_configs`
/// WIT export. When a language server starts, Zed calls this export and registers
/// the returned configurations in this store.
///
/// ## Example
///
/// The Java extension registers a handler like this:
///
/// ```ignore
/// VirtualDocumentConfig {
///     scheme: "jdt".to_string(),
///     content_request_method: "java/classFileContents".to_string(),
///     language_name: "Java".to_string(),
///     language_id: "java".to_string(),
/// }
/// ```
///
/// When a user triggers "Go to Definition" on `ArrayList`, JDTLS returns a
/// `jdt://contents/rt.jar/.../ArrayList.class` URI. Zed looks up the `jdt` scheme,
/// finds this handler, and calls `java/classFileContents` to get the decompiled source.
pub struct VirtualDocumentStore {
    /// Maps URI schemes (e.g., "jdt", "rust-analyzer") to their handler configurations.
    handlers: HashMap<String, VirtualDocumentConfig>,
    /// Background executor for spawning async tasks.
    executor: BackgroundExecutor,
}

impl VirtualDocumentStore {
    /// Creates a new `VirtualDocumentStore`.
    ///
    /// The store starts empty; handlers are registered dynamically as language
    /// servers start and report their virtual document capabilities.
    pub fn new(cx: &mut App) -> Self {
        Self {
            handlers: HashMap::default(),
            executor: cx.background_executor().clone(),
        }
    }

    /// Registers a virtual document handler for a specific URI scheme.
    ///
    /// This is called by the extension host when a language server starts and
    /// the associated extension exports virtual document configurations.
    ///
    /// If a handler for the same scheme already exists, it will be replaced.
    /// This allows extensions to update their handlers if needed.
    ///
    /// # Arguments
    ///
    /// * `config` - The handler configuration containing the scheme, LSP method,
    ///   and language information.
    pub fn register_handler(&mut self, config: VirtualDocumentConfig) {
        self.handlers.insert(config.scheme.clone(), config);
    }

    /// Returns the handler configuration for a given URI scheme.
    ///
    /// Returns `None` if no handler is registered for the scheme.
    ///
    /// # Arguments
    ///
    /// * `scheme` - The URI scheme to look up (e.g., "jdt", "rust-analyzer").
    pub fn handler_for_scheme(&self, scheme: &str) -> Option<&VirtualDocumentConfig> {
        self.handlers.get(scheme)
    }

    /// Processes a virtual URI and returns a task that fetches the document content.
    ///
    /// This method:
    /// 1. Looks up the handler for the URI's scheme
    /// 2. Creates a custom LSP request using the handler's `content_request_method`
    /// 3. Returns a task that, when awaited, yields the document content as a string
    ///
    /// Returns `None` if no handler is registered for the URI's scheme.
    ///
    /// # Arguments
    ///
    /// * `uri` - The virtual document URI (e.g., `jdt://contents/...`)
    /// * `language_server` - The language server to send the request to
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(task) = store.process_uri(&uri, language_server.clone()) {
    ///     let content = task.await?;
    ///     // Create buffer with content...
    /// }
    /// ```
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
                .request_custom::<_, _, String>(request_method, params)
                .await
                .into_response()
                .context("failed to get virtual document contents")
        }))
    }

    /// Returns all registered handlers.
    ///
    /// This is primarily useful for debugging and inspection purposes.
    pub fn handlers(&self) -> &HashMap<String, VirtualDocumentConfig> {
        &self.handlers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn make_config(scheme: &str) -> VirtualDocumentConfig {
        VirtualDocumentConfig {
            scheme: scheme.to_string(),
            content_request_method: format!("{}/getContents", scheme),
            language_name: "TestLanguage".to_string(),
            language_id: "test".to_string(),
        }
    }

    #[gpui::test]
    fn test_register_handler(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            // Initially empty
            assert!(store.handlers().is_empty());

            // Register a handler
            let config = make_config("jdt");
            store.register_handler(config);

            assert_eq!(store.handlers().len(), 1);
            assert!(store.handlers().contains_key("jdt"));
        });
    }

    #[gpui::test]
    fn test_handler_for_scheme(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            // No handler registered
            assert!(store.handler_for_scheme("jdt").is_none());

            // Register handler
            let config = make_config("jdt");
            store.register_handler(config);

            // Now it should be found
            let handler = store.handler_for_scheme("jdt");
            assert!(handler.is_some());
            assert_eq!(handler.unwrap().scheme, "jdt");
            assert_eq!(handler.unwrap().content_request_method, "jdt/getContents");

            // Other schemes still not found
            assert!(store.handler_for_scheme("rust-analyzer").is_none());
        });
    }

    #[gpui::test]
    fn test_register_multiple_handlers(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            store.register_handler(make_config("jdt"));
            store.register_handler(make_config("rust-analyzer"));
            store.register_handler(make_config("dap-browser"));

            assert_eq!(store.handlers().len(), 3);
            assert!(store.handler_for_scheme("jdt").is_some());
            assert!(store.handler_for_scheme("rust-analyzer").is_some());
            assert!(store.handler_for_scheme("dap-browser").is_some());
        });
    }

    #[gpui::test]
    fn test_handler_replacement(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            // Register initial handler
            let config1 = VirtualDocumentConfig {
                scheme: "jdt".to_string(),
                content_request_method: "java/classFileContents".to_string(),
                language_name: "Java".to_string(),
                language_id: "java".to_string(),
            };
            store.register_handler(config1);

            assert_eq!(
                store
                    .handler_for_scheme("jdt")
                    .unwrap()
                    .content_request_method,
                "java/classFileContents"
            );

            // Replace with new handler
            let config2 = VirtualDocumentConfig {
                scheme: "jdt".to_string(),
                content_request_method: "java/newMethod".to_string(),
                language_name: "Java".to_string(),
                language_id: "java".to_string(),
            };
            store.register_handler(config2);

            // Should still have only one handler, but with updated method
            assert_eq!(store.handlers().len(), 1);
            assert_eq!(
                store
                    .handler_for_scheme("jdt")
                    .unwrap()
                    .content_request_method,
                "java/newMethod"
            );
        });
    }

    #[gpui::test]
    fn test_process_uri_no_handler(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let store = VirtualDocumentStore::new(cx);

            // Create a URI with unregistered scheme
            let uri = lsp::Uri::from_str("jdt://contents/some/path").unwrap();

            // This requires a language server, but we can test the None case
            // when no handler is registered - process_uri should return None
            // We can't easily test with a real LanguageServer here, but we verify
            // the handler lookup path
            assert!(store.handler_for_scheme(uri.scheme()).is_none());
        });
    }
}
