mod since_v0_0_1;
mod since_v0_0_4;
mod since_v0_0_6;

use std::ops::RangeInclusive;
use std::sync::Arc;

use anyhow::bail;
use anyhow::{Context, Result};
use language::{LanguageServerName, LspAdapterDelegate};
use semantic_version::SemanticVersion;
use wasmtime::{
    component::{Component, Instance, Linker, Resource},
    Store,
};

use super::{wasm_engine, WasmState};

use since_v0_0_6 as latest;

pub use latest::{CodeLabel, Command};
pub use since_v0_0_4::LanguageServerConfig;

pub fn new_linker(
    f: impl Fn(&mut Linker<WasmState>, fn(&mut WasmState) -> &mut WasmState) -> Result<()>,
) -> Linker<WasmState> {
    let mut linker = Linker::new(&wasm_engine());
    wasmtime_wasi::command::add_to_linker(&mut linker).unwrap();
    f(&mut linker, wasi_view).unwrap();
    linker
}

fn wasi_view(state: &mut WasmState) -> &mut WasmState {
    state
}

/// Returns whether the given Wasm API version is supported by the Wasm host.
pub fn is_supported_wasm_api_version(version: SemanticVersion) -> bool {
    wasm_api_version_range().contains(&version)
}

/// Returns the Wasm API version range that is supported by the Wasm host.
#[inline(always)]
pub fn wasm_api_version_range() -> RangeInclusive<SemanticVersion> {
    since_v0_0_1::MIN_VERSION..=latest::MAX_VERSION
}

pub enum Extension {
    V006(since_v0_0_6::Extension),
    V004(since_v0_0_4::Extension),
    V001(since_v0_0_1::Extension),
}

impl Extension {
    pub async fn instantiate_async(
        store: &mut Store<WasmState>,
        version: SemanticVersion,
        component: &Component,
    ) -> Result<(Self, Instance)> {
        if version >= latest::MIN_VERSION {
            let (extension, instance) =
                latest::Extension::instantiate_async(store, &component, latest::linker())
                    .await
                    .context("failed to instantiate wasm extension")?;
            Ok((Self::V006(extension), instance))
        } else if version >= since_v0_0_4::MIN_VERSION {
            let (extension, instance) = since_v0_0_4::Extension::instantiate_async(
                store,
                &component,
                since_v0_0_4::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok((Self::V004(extension), instance))
        } else {
            let (extension, instance) = since_v0_0_1::Extension::instantiate_async(
                store,
                &component,
                since_v0_0_1::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok((Self::V001(extension), instance))
        }
    }

    pub async fn call_init_extension(&self, store: &mut Store<WasmState>) -> Result<()> {
        match self {
            Extension::V006(ext) => ext.call_init_extension(store).await,
            Extension::V004(ext) => ext.call_init_extension(store).await,
            Extension::V001(ext) => ext.call_init_extension(store).await,
        }
    }

    pub async fn call_language_server_command(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        config: &LanguageServerConfig,
        resource: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> Result<Result<Command, String>> {
        match self {
            Extension::V006(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
            Extension::V004(ext) => Ok(ext
                .call_language_server_command(store, config, resource)
                .await?
                .map(|command| command.into())),
            Extension::V001(ext) => Ok(ext
                .call_language_server_command(store, &config.clone().into(), resource)
                .await?
                .map(|command| command.into())),
        }
    }

    pub async fn call_language_server_initialization_options(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        config: &LanguageServerConfig,
        resource: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
            Extension::V006(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V004(ext) => {
                ext.call_language_server_initialization_options(store, config, resource)
                    .await
            }
            Extension::V001(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &config.clone().into(),
                    resource,
                )
                .await
            }
        }
    }

    pub async fn call_labels_for_completions(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        completions: &[lsp::CompletionItem],
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V001(_) | Extension::V004(_) => {
                bail!("unsupported function: 'labels_for_completions'")
            }
            Extension::V006(ext) => {
                let completions = completions
                    .into_iter()
                    .map(lsp_completion_item_to_wit_completion)
                    .collect::<Vec<_>>();

                ext.call_labels_for_completions(store, &language_server_id.0, &completions)
                    .await
            }
        }
    }
}

trait ToWasmtimeResult<T> {
    fn to_wasmtime_result(self) -> wasmtime::Result<Result<T, String>>;
}

impl<T> ToWasmtimeResult<T> for Result<T> {
    fn to_wasmtime_result(self) -> wasmtime::Result<Result<T, String>> {
        Ok(self.map_err(|error| error.to_string()))
    }
}

fn lsp_completion_item_to_wit_completion(completion: &lsp::CompletionItem) -> latest::Completion {
    latest::Completion {
        label: completion.label.clone(),
        detail: completion.detail.clone(),
        kind: completion
            .kind
            .map(lsp_completion_item_to_wit_completion_kind),
        insert_text_format: completion
            .insert_text_format
            .map(lsp_insert_text_format_to_wit_insert_text_format),
    }
}

fn lsp_completion_item_to_wit_completion_kind(
    value: lsp::CompletionItemKind,
) -> latest::zed::extension::lsp::CompletionItemKind {
    use latest::zed::extension::lsp::CompletionItemKind;

    match value {
        lsp::CompletionItemKind::TEXT => CompletionItemKind::Text,
        lsp::CompletionItemKind::METHOD => CompletionItemKind::Method,
        lsp::CompletionItemKind::FUNCTION => CompletionItemKind::Function,
        lsp::CompletionItemKind::CONSTRUCTOR => CompletionItemKind::Constructor,
        lsp::CompletionItemKind::FIELD => CompletionItemKind::Field,
        lsp::CompletionItemKind::VARIABLE => CompletionItemKind::Variable,
        lsp::CompletionItemKind::CLASS => CompletionItemKind::Class,
        lsp::CompletionItemKind::INTERFACE => CompletionItemKind::Interface,
        lsp::CompletionItemKind::MODULE => CompletionItemKind::Module,
        lsp::CompletionItemKind::PROPERTY => CompletionItemKind::Property,
        lsp::CompletionItemKind::UNIT => CompletionItemKind::Unit,
        lsp::CompletionItemKind::VALUE => CompletionItemKind::Value,
        lsp::CompletionItemKind::ENUM => CompletionItemKind::Enum,
        lsp::CompletionItemKind::KEYWORD => CompletionItemKind::Keyword,
        lsp::CompletionItemKind::SNIPPET => CompletionItemKind::Snippet,
        lsp::CompletionItemKind::COLOR => CompletionItemKind::Color,
        lsp::CompletionItemKind::FILE => CompletionItemKind::File,
        lsp::CompletionItemKind::REFERENCE => CompletionItemKind::Reference,
        lsp::CompletionItemKind::FOLDER => CompletionItemKind::Folder,
        lsp::CompletionItemKind::ENUM_MEMBER => CompletionItemKind::EnumMember,
        lsp::CompletionItemKind::CONSTANT => CompletionItemKind::Constant,
        lsp::CompletionItemKind::STRUCT => CompletionItemKind::Struct,
        lsp::CompletionItemKind::EVENT => CompletionItemKind::Event,
        lsp::CompletionItemKind::OPERATOR => CompletionItemKind::Operator,
        lsp::CompletionItemKind::TYPE_PARAMETER => CompletionItemKind::TypeParameter,
        _ => {
            // TODO: Make sure this works and deal with `.unwrap`s properly.
            let kind = serde_json::to_string(&value).unwrap();
            let value: i32 = serde_json::from_str(&kind).unwrap();
            CompletionItemKind::Other(value)
        }
    }
}

fn lsp_insert_text_format_to_wit_insert_text_format(
    value: lsp::InsertTextFormat,
) -> latest::zed::extension::lsp::InsertTextFormat {
    use latest::zed::extension::lsp::InsertTextFormat;

    match value {
        lsp::InsertTextFormat::PLAIN_TEXT => InsertTextFormat::PlainText,
        lsp::InsertTextFormat::SNIPPET => InsertTextFormat::Snippet,
        _ => {
            // TODO: Make sure this works and deal with `.unwrap`s properly.
            let kind = serde_json::to_string(&value).unwrap();
            let value: i32 = serde_json::from_str(&kind).unwrap();
            InsertTextFormat::Other(value)
        }
    }
}
