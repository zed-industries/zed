use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, BackgroundExecutor, Task};
use lsp::{LanguageServer, Position};
use std::sync::Arc;

// Re-export the config types from lsp crate (defined there to avoid circular dependencies)
pub use lsp::{VirtualDocumentConfig, VirtualDocumentParamBuilder};

/// Schemes reserved by the system that cannot be registered for virtual documents.
const RESERVED_SCHEMES: &[&str] = &["file", "http", "https", "ssh", "untitled"];

/// Extracts a friendly display name from a virtual document URI.
///
/// For example:
/// - `jdt://contents/rt.jar/java.util/ArrayList.class` → "ArrayList.java"
/// - `rust-analyzer://macro-expansion/...` → "macro-expansion"
pub fn display_name_from_uri(uri: &lsp::Uri, config: &VirtualDocumentConfig) -> String {
    let uri_str = uri.to_string();

    // Try to extract the last path segment before any query string
    let path_part = uri_str.split('?').next().unwrap_or(&uri_str);
    let last_segment = path_part.rsplit('/').next().unwrap_or(path_part);

    // Handle Java .class files - convert to .java for display
    if last_segment.ends_with(".class") {
        let name = last_segment.trim_end_matches(".class");
        return format!("{}.java", name);
    }

    // For other cases, just use the last segment if it looks like a filename
    if last_segment.contains('.') && !last_segment.starts_with('.') {
        return last_segment.to_string();
    }

    // Fall back to scheme + truncated path for very long/unusual URIs
    if last_segment.len() > 30 {
        format!(
            "[{}] ...{}",
            config.scheme,
            &last_segment[last_segment.len() - 20..]
        )
    } else if last_segment.is_empty() {
        format!("[{}]", config.scheme)
    } else {
        last_segment.to_string()
    }
}

/// Registry mapping URI schemes to virtual document handler configurations.
///
/// Virtual documents are documents provided by language servers via custom LSP requests
/// (e.g., decompiled `.class` files via `jdt://` URIs, macro expansions via `rust-analyzer://`).
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

    /// Returns an error if the scheme is reserved (e.g., "file", "http").
    pub fn register_handler(&mut self, config: VirtualDocumentConfig) -> Result<()> {
        if RESERVED_SCHEMES.contains(&config.scheme.as_str()) {
            anyhow::bail!(
                "cannot register virtual document handler for reserved scheme: {}",
                config.scheme
            );
        }
        self.handlers.insert(config.scheme.clone(), config);
        Ok(())
    }

    pub fn handler_for_scheme(&self, scheme: &str) -> Option<&VirtualDocumentConfig> {
        self.handlers.get(scheme)
    }

    /// Fetches virtual document content via the language server.
    /// Returns `None` if no handler is registered for the URI's scheme.
    pub fn process_uri(
        &self,
        uri: &lsp::Uri,
        language_server: Arc<LanguageServer>,
        position: Option<Position>,
    ) -> Option<Task<Result<String>>> {
        let scheme = uri.scheme();
        let config = self.handlers.get(scheme)?;

        let request_method = config.content_request_method.clone();
        let executor = self.executor.clone();

        // Use the builder callback to construct request parameters
        let params = config.param_builder.build_params(uri, position);

        Some(executor.spawn(async move {
            language_server
                .request_custom::<_, _, String>(request_method, params)
                .await
                .into_response()
                .context("failed to get virtual document contents")
        }))
    }

    pub fn handlers(&self) -> &HashMap<String, VirtualDocumentConfig> {
        &self.handlers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn make_config(scheme: &str) -> VirtualDocumentConfig {
        VirtualDocumentConfig::new(
            scheme,
            format!("{}/getContents", scheme),
            "TestLanguage",
            "test",
        )
    }

    #[gpui::test]
    fn test_register_handler(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            assert!(store.handlers().is_empty());

            let config = make_config("jdt");
            store.register_handler(config).unwrap();

            assert_eq!(store.handlers().len(), 1);
            assert!(store.handlers().contains_key("jdt"));
        });
    }

    #[gpui::test]
    fn test_handler_for_scheme(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            assert!(store.handler_for_scheme("jdt").is_none());

            let config = make_config("jdt");
            store.register_handler(config).unwrap();

            let handler = store.handler_for_scheme("jdt");
            assert!(handler.is_some());
            assert_eq!(handler.unwrap().scheme, "jdt");
            assert_eq!(handler.unwrap().content_request_method, "jdt/getContents");

            assert!(store.handler_for_scheme("rust-analyzer").is_none());
        });
    }

    #[gpui::test]
    fn test_register_multiple_handlers(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            store.register_handler(make_config("jdt")).unwrap();
            store
                .register_handler(make_config("rust-analyzer"))
                .unwrap();
            store.register_handler(make_config("dap-browser")).unwrap();

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

            let config1 =
                VirtualDocumentConfig::new("jdt", "java/classFileContents", "Java", "java");
            store.register_handler(config1).unwrap();

            assert_eq!(
                store
                    .handler_for_scheme("jdt")
                    .unwrap()
                    .content_request_method,
                "java/classFileContents"
            );

            let config2 = VirtualDocumentConfig::new("jdt", "java/newMethod", "Java", "java");
            store.register_handler(config2).unwrap();

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
    fn test_reserved_scheme_rejected(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            let file_config = make_config("file");
            assert!(store.register_handler(file_config).is_err());

            let http_config = make_config("http");
            assert!(store.register_handler(http_config).is_err());

            assert!(store.handlers().is_empty());
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

    #[gpui::test]
    fn test_param_builder_variants(cx: &mut gpui::TestAppContext) {
        use std::str::FromStr;

        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            // Test UriParamBuilder (default)
            let jdt_config =
                VirtualDocumentConfig::new("jdt", "java/classFileContents", "Java", "java");
            store.register_handler(jdt_config).unwrap();

            // Test RawUriParamBuilder
            let raw_config = VirtualDocumentConfig::with_raw_uri(
                "custom",
                "custom/getContents",
                "Custom",
                "custom",
            );
            store.register_handler(raw_config).unwrap();

            // Test UriWithPositionParamBuilder
            let ra_config = VirtualDocumentConfig::with_position(
                "rust-analyzer",
                "rust-analyzer/expandMacro",
                "Rust",
                "rust",
            );
            store.register_handler(ra_config).unwrap();

            // Verify all handlers are registered
            assert_eq!(store.handlers().len(), 3);
            assert!(store.handler_for_scheme("jdt").is_some());
            assert!(store.handler_for_scheme("custom").is_some());
            assert!(store.handler_for_scheme("rust-analyzer").is_some());

            // Test that builders produce correct output
            let test_uri = lsp::Uri::from_str("jdt://test").unwrap();
            let jdt_builder = &store.handler_for_scheme("jdt").unwrap().param_builder;
            let params = jdt_builder.build_params(&test_uri, None);
            assert_eq!(params["uri"], "jdt://test");

            let custom_builder = &store.handler_for_scheme("custom").unwrap().param_builder;
            let params = custom_builder.build_params(&test_uri, None);
            assert_eq!(params, "jdt://test");
        });
    }

    #[gpui::test]
    fn test_default_param_builder(cx: &mut gpui::TestAppContext) {
        use std::str::FromStr;

        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            let config = make_config("test");
            store.register_handler(config).unwrap();

            // Verify default builder is UriParamBuilder
            let test_uri = lsp::Uri::from_str("test://file").unwrap();
            let builder = &store.handler_for_scheme("test").unwrap().param_builder;
            let params = builder.build_params(&test_uri, None);
            assert_eq!(params["uri"], "test://file");
        });
    }

    #[test]
    fn test_display_name_from_uri_java_class() {
        let config = VirtualDocumentConfig::new("jdt", "java/classFileContents", "Java", "java");

        let uri = lsp::Uri::from_str("jdt://contents/rt.jar/java.util/ArrayList.class").unwrap();
        assert_eq!(display_name_from_uri(&uri, &config), "ArrayList.java");

        let uri2 =
            lsp::Uri::from_str("jdt://contents/some.jar/com.example/MyClass.class?query").unwrap();
        assert_eq!(display_name_from_uri(&uri2, &config), "MyClass.java");
    }

    #[test]
    fn test_display_name_from_uri_regular_file() {
        let config = make_config("custom");

        let uri = lsp::Uri::from_str("custom://path/to/file.rs").unwrap();
        assert_eq!(display_name_from_uri(&uri, &config), "file.rs");
    }

    #[test]
    fn test_display_name_from_uri_no_extension() {
        let config = make_config("ra");

        let uri = lsp::Uri::from_str("ra://macro-expansion").unwrap();
        assert_eq!(display_name_from_uri(&uri, &config), "macro-expansion");
    }

    #[test]
    fn test_display_name_from_uri_empty_path() {
        let config = make_config("test");

        let uri = lsp::Uri::from_str("test://").unwrap();
        assert_eq!(display_name_from_uri(&uri, &config), "[test]");
    }
}
