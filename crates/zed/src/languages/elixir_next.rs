use anyhow::Result;
use async_trait::async_trait;
pub use language::*;
use lsp::{CompletionItemKind, LanguageServerBinary, SymbolKind};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;
use std::{any::Any, ops::Deref, path::PathBuf, sync::Arc};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct ElixirSettings {
    pub next: ElixirNextSetting,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ElixirNextSetting {
    Off,
    On,
    Local {
        path: String,
        arguments: Vec<String>,
    },
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct ElixirSettingsContent {
    next: Option<ElixirNextSetting>,
}

impl Setting for ElixirSettings {
    const KEY: Option<&'static str> = Some("elixir");

    type FileContent = ElixirSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}

pub struct LocalNextLspAdapter {
    pub path: String,
    pub arguments: Vec<String>,
}

#[async_trait]
impl LspAdapter for LocalNextLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("elixir-next-ls".into())
    }

    fn short_name(&self) -> &'static str {
        "next-ls"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let path = shellexpand::full(&self.path)?;
        Ok(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    // TODO:
    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        match completion.kind.zip(completion.detail.as_ref()) {
            Some((_, detail)) if detail.starts_with("(function)") => {
                let text = detail.strip_prefix("(function) ")?;
                let filter_range = 0..text.find('(').unwrap_or(text.len());
                let source = Rope::from(format!("def {text}").as_str());
                let runs = language.highlight_text(&source, 4..4 + text.len());
                return Some(CodeLabel {
                    text: text.to_string(),
                    runs,
                    filter_range,
                });
            }
            Some((_, detail)) if detail.starts_with("(macro)") => {
                let text = detail.strip_prefix("(macro) ")?;
                let filter_range = 0..text.find('(').unwrap_or(text.len());
                let source = Rope::from(format!("defmacro {text}").as_str());
                let runs = language.highlight_text(&source, 9..9 + text.len());
                return Some(CodeLabel {
                    text: text.to_string(),
                    runs,
                    filter_range,
                });
            }
            Some((
                CompletionItemKind::CLASS
                | CompletionItemKind::MODULE
                | CompletionItemKind::INTERFACE
                | CompletionItemKind::STRUCT,
                _,
            )) => {
                let filter_range = 0..completion
                    .label
                    .find(" (")
                    .unwrap_or(completion.label.len());
                let text = &completion.label[filter_range.clone()];
                let source = Rope::from(format!("defmodule {text}").as_str());
                let runs = language.highlight_text(&source, 10..10 + text.len());
                return Some(CodeLabel {
                    text: completion.label.clone(),
                    runs,
                    filter_range,
                });
            }
            _ => {}
        }

        None
    }

    // TODO:
    async fn label_for_symbol(
        &self,
        name: &str,
        kind: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            SymbolKind::METHOD | SymbolKind::FUNCTION => {
                let text = format!("def {}", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            SymbolKind::CLASS | SymbolKind::MODULE | SymbolKind::INTERFACE | SymbolKind::STRUCT => {
                let text = format!("defmodule {}", name);
                let filter_range = 10..10 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }
}
