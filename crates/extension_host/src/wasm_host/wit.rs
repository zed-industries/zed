mod since_v0_0_1;
mod since_v0_0_4;
mod since_v0_0_6;
mod since_v0_1_0;
mod since_v0_2_0;
use extension::{KeyValueStoreDelegate, WorktreeDelegate};
use lsp::LanguageServerName;
use release_channel::ReleaseChannel;
use since_v0_2_0 as latest;

use super::{wasm_engine, WasmState};
use anyhow::{anyhow, Context, Result};
use semantic_version::SemanticVersion;
use std::{ops::RangeInclusive, sync::Arc};
use wasmtime::{
    component::{Component, Linker, Resource},
    Store,
};

#[cfg(test)]
pub use latest::CodeLabelSpanLiteral;
pub use latest::{
    zed::extension::lsp::{
        Completion, CompletionKind, CompletionLabelDetails, InsertTextFormat, Symbol, SymbolKind,
    },
    zed::extension::slash_command::{SlashCommandArgumentCompletion, SlashCommandOutput},
    CodeLabel, CodeLabelSpan, Command, ExtensionProject, Range, SlashCommand,
};
pub use since_v0_0_4::LanguageServerConfig;

pub fn new_linker(
    f: impl Fn(&mut Linker<WasmState>, fn(&mut WasmState) -> &mut WasmState) -> Result<()>,
) -> Linker<WasmState> {
    let mut linker = Linker::new(&wasm_engine());
    wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();
    f(&mut linker, wasi_view).unwrap();
    linker
}

fn wasi_view(state: &mut WasmState) -> &mut WasmState {
    state
}

/// Returns whether the given Wasm API version is supported by the Wasm host.
pub fn is_supported_wasm_api_version(
    release_channel: ReleaseChannel,
    version: SemanticVersion,
) -> bool {
    wasm_api_version_range(release_channel).contains(&version)
}

/// Returns the Wasm API version range that is supported by the Wasm host.
#[inline(always)]
pub fn wasm_api_version_range(release_channel: ReleaseChannel) -> RangeInclusive<SemanticVersion> {
    // Note: The release channel can be used to stage a new version of the extension API.
    let _ = release_channel;

    let max_version = match release_channel {
        ReleaseChannel::Dev | ReleaseChannel::Nightly => latest::MAX_VERSION,
        ReleaseChannel::Stable | ReleaseChannel::Preview => since_v0_1_0::MAX_VERSION,
    };

    since_v0_0_1::MIN_VERSION..=max_version
}

pub enum Extension {
    V020(since_v0_2_0::Extension),
    V010(since_v0_1_0::Extension),
    V006(since_v0_0_6::Extension),
    V004(since_v0_0_4::Extension),
    V001(since_v0_0_1::Extension),
}

impl Extension {
    pub async fn instantiate_async(
        store: &mut Store<WasmState>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        component: &Component,
    ) -> Result<Self> {
        if version >= latest::MIN_VERSION {
            // Note: The release channel can be used to stage a new version of the extension API.
            // We always allow the latest in tests so that the extension tests pass on release branches.
            let allow_latest_version = match release_channel {
                ReleaseChannel::Dev | ReleaseChannel::Nightly => true,
                ReleaseChannel::Stable | ReleaseChannel::Preview => {
                    cfg!(any(test, feature = "test-support"))
                }
            };
            if !allow_latest_version {
                Err(anyhow!(
                    "unreleased versions of the extension API can only be used on development builds of Zed"
                ))?;
            }
            let extension =
                latest::Extension::instantiate_async(store, component, latest::linker())
                    .await
                    .context("failed to instantiate wasm extension")?;
            Ok(Self::V020(extension))
        } else if version >= since_v0_1_0::MIN_VERSION {
            let extension = since_v0_1_0::Extension::instantiate_async(
                store,
                component,
                since_v0_1_0::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V010(extension))
        } else if version >= since_v0_0_6::MIN_VERSION {
            let extension = since_v0_0_6::Extension::instantiate_async(
                store,
                component,
                since_v0_0_6::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V006(extension))
        } else if version >= since_v0_0_4::MIN_VERSION {
            let extension = since_v0_0_4::Extension::instantiate_async(
                store,
                component,
                since_v0_0_4::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V004(extension))
        } else {
            let extension = since_v0_0_1::Extension::instantiate_async(
                store,
                component,
                since_v0_0_1::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V001(extension))
        }
    }

    pub async fn call_init_extension(&self, store: &mut Store<WasmState>) -> Result<()> {
        match self {
            Extension::V020(ext) => ext.call_init_extension(store).await,
            Extension::V010(ext) => ext.call_init_extension(store).await,
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
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Command, String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
            Extension::V010(ext) => Ok(ext
                .call_language_server_command(store, &language_server_id.0, resource)
                .await?
                .map(|command| command.into())),
            Extension::V006(ext) => Ok(ext
                .call_language_server_command(store, &language_server_id.0, resource)
                .await?
                .map(|command| command.into())),
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
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V010(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
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
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V010(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
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
            Extension::V020(ext) => {
                ext.call_labels_for_completions(store, &language_server_id.0, &completions)
                    .await
            }
            Extension::V010(ext) => Ok(ext
                .call_labels_for_completions(
                    store,
                    &language_server_id.0,
                    &completions.into_iter().map(Into::into).collect::<Vec<_>>(),
                )
                .await?
                .map(|labels| {
                    labels
                        .into_iter()
                        .map(|label| label.map(Into::into))
                        .collect()
                })),
            Extension::V006(ext) => Ok(ext
                .call_labels_for_completions(
                    store,
                    &language_server_id.0,
                    &completions.into_iter().map(Into::into).collect::<Vec<_>>(),
                )
                .await?
                .map(|labels| {
                    labels
                        .into_iter()
                        .map(|label| label.map(Into::into))
                        .collect()
                })),
            Extension::V001(_) | Extension::V004(_) => Ok(Ok(Vec::new())),
        }
    }

    pub async fn call_labels_for_symbols(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        symbols: Vec<latest::Symbol>,
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_labels_for_symbols(store, &language_server_id.0, &symbols)
                    .await
            }
            Extension::V010(ext) => Ok(ext
                .call_labels_for_symbols(
                    store,
                    &language_server_id.0,
                    &symbols.into_iter().map(Into::into).collect::<Vec<_>>(),
                )
                .await?
                .map(|labels| {
                    labels
                        .into_iter()
                        .map(|label| label.map(Into::into))
                        .collect()
                })),
            Extension::V006(ext) => Ok(ext
                .call_labels_for_symbols(
                    store,
                    &language_server_id.0,
                    &symbols.into_iter().map(Into::into).collect::<Vec<_>>(),
                )
                .await?
                .map(|labels| {
                    labels
                        .into_iter()
                        .map(|label| label.map(Into::into))
                        .collect()
                })),
            Extension::V001(_) | Extension::V004(_) => Ok(Ok(Vec::new())),
        }
    }

    pub async fn call_complete_slash_command_argument(
        &self,
        store: &mut Store<WasmState>,
        command: &SlashCommand,
        arguments: &[String],
    ) -> Result<Result<Vec<SlashCommandArgumentCompletion>, String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V010(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V001(_) | Extension::V004(_) | Extension::V006(_) => Ok(Ok(Vec::new())),
        }
    }

    pub async fn call_run_slash_command(
        &self,
        store: &mut Store<WasmState>,
        command: &SlashCommand,
        arguments: &[String],
        resource: Option<Resource<Arc<dyn WorktreeDelegate>>>,
    ) -> Result<Result<SlashCommandOutput, String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V010(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V001(_) | Extension::V004(_) | Extension::V006(_) => {
                Err(anyhow!("`run_slash_command` not available prior to v0.1.0"))
            }
        }
    }

    pub async fn call_context_server_command(
        &self,
        store: &mut Store<WasmState>,
        context_server_id: Arc<str>,
        project: Resource<ExtensionProject>,
    ) -> Result<Result<Command, String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_context_server_command(store, &context_server_id, project)
                    .await
            }
            Extension::V001(_) | Extension::V004(_) | Extension::V006(_) | Extension::V010(_) => {
                Err(anyhow!(
                    "`context_server_command` not available prior to v0.2.0"
                ))
            }
        }
    }

    pub async fn call_suggest_docs_packages(
        &self,
        store: &mut Store<WasmState>,
        provider: &str,
    ) -> Result<Result<Vec<String>, String>> {
        match self {
            Extension::V020(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V010(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V001(_) | Extension::V004(_) | Extension::V006(_) => Err(anyhow!(
                "`suggest_docs_packages` not available prior to v0.1.0"
            )),
        }
    }

    pub async fn call_index_docs(
        &self,
        store: &mut Store<WasmState>,
        provider: &str,
        package_name: &str,
        kv_store: Resource<Arc<dyn KeyValueStoreDelegate>>,
    ) -> Result<Result<(), String>> {
        match self {
            Extension::V020(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V010(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V001(_) | Extension::V004(_) | Extension::V006(_) => {
                Err(anyhow!("`index_docs` not available prior to v0.1.0"))
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
