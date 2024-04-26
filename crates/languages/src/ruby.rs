use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gpui::AsyncAppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use project::project_settings::{BinarySettings, ProjectSettings};
use settings::Settings;
use std::{any::Any, ffi::OsString, path::PathBuf, sync::Arc};

pub struct RubyLanguageServer;

impl RubyLanguageServer {
    const SERVER_NAME: &'static str = "solargraph";

    fn server_binary_arguments() -> Vec<OsString> {
        vec!["stdio".into()]
    }
}

#[async_trait(?Send)]
impl LspAdapter for RubyLanguageServer {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(Self::SERVER_NAME.into())
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        cx: &AsyncAppContext,
    ) -> Option<LanguageServerBinary> {
        let configured_binary = cx.update(|cx| {
            ProjectSettings::get_global(cx)
                .lsp
                .get(Self::SERVER_NAME)
                .and_then(|s| s.binary.clone())
        });

        if let Ok(Some(BinarySettings {
            path: Some(path),
            arguments,
        })) = configured_binary
        {
            Some(LanguageServerBinary {
                path: path.into(),
                arguments: arguments
                    .unwrap_or_default()
                    .iter()
                    .map(|arg| arg.into())
                    .collect(),
                env: None,
            })
        } else {
            let env = delegate.shell_env().await;
            let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
            Some(LanguageServerBinary {
                path,
                arguments: Self::server_binary_arguments(),
                env: Some(env),
            })
        }
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
            arguments: Self::server_binary_arguments(),
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
