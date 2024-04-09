mod since_v0_0_1;
mod since_v0_0_4;
mod since_v0_0_6;
use since_v0_0_6 as latest;

use super::{wasm_engine, WasmState};
use anyhow::{Context, Result};
use language::{LanguageServerName, LspAdapterDelegate};
use semantic_version::SemanticVersion;
use std::{ops::RangeInclusive, sync::Arc};
use wasmtime::{
    component::{Component, Instance, Linker, Resource},
    Store,
};

#[cfg(test)]
pub use latest::CodeLabelSpanLiteral;
pub use latest::{
    zed::extension::lsp::{Completion, CompletionKind, InsertTextFormat, Symbol, SymbolKind},
    CodeLabel, CodeLabelSpan, Command, Range,
};
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

    pub async fn call_language_server_workspace_configuration(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
            Extension::V006(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V004(_) | Extension::V001(_) => Ok(Ok(None)),
        }
    }

    pub async fn call_labels_for_completions(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        completions: Vec<latest::Completion>,
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V001(_) | Extension::V004(_) => Ok(Ok(Vec::new())),
            Extension::V006(ext) => {
                ext.call_labels_for_completions(store, &language_server_id.0, &completions)
                    .await
            }
        }
    }

    pub async fn call_labels_for_symbols(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        symbols: Vec<latest::Symbol>,
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V001(_) | Extension::V004(_) => Ok(Ok(Vec::new())),
            Extension::V006(ext) => {
                ext.call_labels_for_symbols(store, &language_server_id.0, &symbols)
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
