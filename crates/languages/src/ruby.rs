use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf, sync::Arc};

pub struct RubyLanguageServer;

#[async_trait]
impl LspAdapter for RubyLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("solargraph".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!("solargraph must be installed manually"))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "solargraph".into(),
            env: None,
            arguments: vec!["stdio".into()],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let label = &item.label;
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            lsp::CompletionItemKind::METHOD => grammar.highlight_id_for_name("function.method")?,
            lsp::CompletionItemKind::CONSTANT => grammar.highlight_id_for_name("constant")?,
            lsp::CompletionItemKind::CLASS | lsp::CompletionItemKind::MODULE => {
                grammar.highlight_id_for_name("type")?
            }
            lsp::CompletionItemKind::KEYWORD => {
                if label.starts_with(':') {
                    grammar.highlight_id_for_name("string.special.symbol")?
                } else {
                    grammar.highlight_id_for_name("keyword")?
                }
            }
            lsp::CompletionItemKind::VARIABLE => {
                if label.starts_with('@') {
                    grammar.highlight_id_for_name("property")?
                } else {
                    return None;
                }
            }
            _ => return None,
        };
        Some(language::CodeLabel {
            text: label.clone(),
            runs: vec![(0..label.len(), highlight_id)],
            filter_range: 0..label.len(),
        })
    }

    async fn label_for_symbol(
        &self,
        label: &str,
        kind: lsp::SymbolKind,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let grammar = language.grammar()?;
        match kind {
            lsp::SymbolKind::METHOD => {
                let mut parts = label.split('#');
                let classes = parts.next()?;
                let method = parts.next()?;
                if parts.next().is_some() {
                    return None;
                }

                let class_id = grammar.highlight_id_for_name("type")?;
                let method_id = grammar.highlight_id_for_name("function.method")?;

                let mut ix = 0;
                let mut runs = Vec::new();
                for (i, class) in classes.split("::").enumerate() {
                    if i > 0 {
                        ix += 2;
                    }
                    let end_ix = ix + class.len();
                    runs.push((ix..end_ix, class_id));
                    ix = end_ix;
                }

                ix += 1;
                let end_ix = ix + method.len();
                runs.push((ix..end_ix, method_id));
                Some(language::CodeLabel {
                    text: label.to_string(),
                    runs,
                    filter_range: 0..label.len(),
                })
            }
            lsp::SymbolKind::CONSTANT => {
                let constant_id = grammar.highlight_id_for_name("constant")?;
                Some(language::CodeLabel {
                    text: label.to_string(),
                    runs: vec![(0..label.len(), constant_id)],
                    filter_range: 0..label.len(),
                })
            }
            lsp::SymbolKind::CLASS | lsp::SymbolKind::MODULE => {
                let class_id = grammar.highlight_id_for_name("type")?;

                let mut ix = 0;
                let mut runs = Vec::new();
                for (i, class) in label.split("::").enumerate() {
                    if i > 0 {
                        ix += "::".len();
                    }
                    let end_ix = ix + class.len();
                    runs.push((ix..end_ix, class_id));
                    ix = end_ix;
                }

                Some(language::CodeLabel {
                    text: label.to_string(),
                    runs,
                    filter_range: 0..label.len(),
                })
            }
            _ => return None,
        }
    }
}
