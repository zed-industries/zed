mod v0_0_1;
mod v0_0_4;

use super::{wasm_engine, WasmState};
use anyhow::{Context, Result};
use language::LspAdapterDelegate;
use std::sync::Arc;
use util::SemanticVersion;
use wasmtime::{
    component::{Component, Instance, Linker, Resource},
    Store,
};

use v0_0_4 as latest;

pub use latest::{Command, LanguageServerConfig};

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
    v0_0_1::VERSION <= version && version <= v0_0_4::VERSION
}

pub enum Extension {
    V004(v0_0_4::Extension),
    V001(v0_0_1::Extension),
}

impl Extension {
    pub async fn instantiate_async(
        store: &mut Store<WasmState>,
        version: SemanticVersion,
        component: &Component,
    ) -> Result<(Self, Instance)> {
        if version < latest::VERSION {
            let (extension, instance) =
                v0_0_1::Extension::instantiate_async(store, &component, v0_0_1::linker())
                    .await
                    .context("failed to instantiate wasm extension")?;
            Ok((Self::V001(extension), instance))
        } else {
            let (extension, instance) =
                v0_0_4::Extension::instantiate_async(store, &component, v0_0_4::linker())
                    .await
                    .context("failed to instantiate wasm extension")?;
            Ok((Self::V004(extension), instance))
        }
    }

    pub async fn call_init_extension(&self, store: &mut Store<WasmState>) -> Result<()> {
        match self {
            Extension::V004(ext) => ext.call_init_extension(store).await,
            Extension::V001(ext) => ext.call_init_extension(store).await,
        }
    }

    pub async fn call_language_server_command(
        &self,
        store: &mut Store<WasmState>,
        config: &LanguageServerConfig,
        resource: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> Result<Result<Command, String>> {
        match self {
            Extension::V004(ext) => {
                ext.call_language_server_command(store, config, resource)
                    .await
            }
            Extension::V001(ext) => Ok(ext
                .call_language_server_command(store, &config.clone().into(), resource)
                .await?
                .map(|command| command.into())),
        }
    }

    pub async fn call_language_server_initialization_options(
        &self,
        store: &mut Store<WasmState>,
        config: &LanguageServerConfig,
        resource: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
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
}
