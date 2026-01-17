use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, BackgroundExecutor, Task};
use lsp::{LanguageServer, Position, TextDocumentIdentifier, TextDocumentPositionParams};
use serde_json::Value;
use std::sync::Arc;

// Re-export the config types from lsp crate (defined there to avoid circular dependencies)
pub use lsp::{VirtualDocumentConfig, VirtualDocumentParamKind};

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

    pub fn register_handler(&mut self, config: VirtualDocumentConfig) {
        self.handlers.insert(config.scheme.clone(), config);
    }

    pub fn handler_for_scheme(&self, scheme: &str) -> Option<&VirtualDocumentConfig> {
        self.handlers.get(scheme)
    }

    /// Fetches virtual document content via the language server.
    /// Returns `None` if no handler is registered for the URI's scheme.
    /// For `UriWithPosition` param kind, falls back to position (0, 0) if not provided.
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

        let params: Value = match &config.param_kind {
            VirtualDocumentParamKind::Uri => {
                serde_json::to_value(TextDocumentIdentifier { uri: uri.clone() })
                    .expect("TextDocumentIdentifier should serialize")
            }
            VirtualDocumentParamKind::RawUri => Value::String(uri.to_string()),
            VirtualDocumentParamKind::UriWithPosition => {
                let pos = position.unwrap_or(Position {
                    line: 0,
                    character: 0,
                });
                serde_json::to_value(TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: pos,
                })
                .expect("TextDocumentPositionParams should serialize")
            }
        };

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
        VirtualDocumentConfig {
            scheme: scheme.to_string(),
            content_request_method: format!("{}/getContents", scheme),
            language_name: "TestLanguage".to_string(),
            language_id: "test".to_string(),
            param_kind: VirtualDocumentParamKind::default(),
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
                param_kind: VirtualDocumentParamKind::default(),
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
                param_kind: VirtualDocumentParamKind::default(),
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

    #[gpui::test]
    fn test_param_kind_variants(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            // Test Uri param kind (JDTLS style)
            let jdt_config = VirtualDocumentConfig {
                scheme: "jdt".to_string(),
                content_request_method: "java/classFileContents".to_string(),
                language_name: "Java".to_string(),
                language_id: "java".to_string(),
                param_kind: VirtualDocumentParamKind::Uri,
            };
            store.register_handler(jdt_config);
            assert_eq!(
                store.handler_for_scheme("jdt").unwrap().param_kind,
                VirtualDocumentParamKind::Uri
            );

            // Test RawUri param kind
            let raw_config = VirtualDocumentConfig {
                scheme: "custom".to_string(),
                content_request_method: "custom/getContents".to_string(),
                language_name: "Custom".to_string(),
                language_id: "custom".to_string(),
                param_kind: VirtualDocumentParamKind::RawUri,
            };
            store.register_handler(raw_config);
            assert_eq!(
                store.handler_for_scheme("custom").unwrap().param_kind,
                VirtualDocumentParamKind::RawUri
            );

            // Test UriWithPosition param kind (rust-analyzer style)
            let ra_config = VirtualDocumentConfig {
                scheme: "rust-analyzer".to_string(),
                content_request_method: "rust-analyzer/expandMacro".to_string(),
                language_name: "Rust".to_string(),
                language_id: "rust".to_string(),
                param_kind: VirtualDocumentParamKind::UriWithPosition,
            };
            store.register_handler(ra_config);
            assert_eq!(
                store
                    .handler_for_scheme("rust-analyzer")
                    .unwrap()
                    .param_kind,
                VirtualDocumentParamKind::UriWithPosition
            );

            // Verify all three are registered
            assert_eq!(store.handlers().len(), 3);
        });
    }

    #[gpui::test]
    fn test_default_param_kind(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let mut store = VirtualDocumentStore::new(cx);

            // Test that default param_kind is Uri
            let config = make_config("test");
            store.register_handler(config);

            assert_eq!(
                store.handler_for_scheme("test").unwrap().param_kind,
                VirtualDocumentParamKind::Uri
            );
        });
    }
}
