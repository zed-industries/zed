mod since_v0_0_1;
mod since_v0_0_4;
mod since_v0_0_6;
mod since_v0_1_0;
mod since_v0_2_0;
mod since_v0_3_0;
mod since_v0_4_0;
mod since_v0_5_0;
mod since_v0_6_0;
mod since_v0_8_0;
use dap::DebugRequest;
use extension::{DebugTaskDefinition, KeyValueStoreDelegate, WorktreeDelegate};
use gpui::BackgroundExecutor;
use language::LanguageName;
use lsp::LanguageServerName;
use release_channel::ReleaseChannel;
use task::{DebugScenario, SpawnInTerminal, TaskTemplate, ZedDebugConfig};

use latest::dap::StartDebuggingRequestArgumentsRequest;

use super::{WasmState, wasm_engine};
use anyhow::{Context as _, Result};
use semver::Version;
use since_v0_8_0 as latest;
use std::{ops::RangeInclusive, path::PathBuf, sync::Arc};
use wasmtime::{
    Store,
    component::{Component, Linker, Resource},
};

#[cfg(test)]
pub use latest::CodeLabelSpanLiteral;
pub use latest::{
    CodeLabel, CodeLabelSpan, Command, DebugAdapterBinary, ExtensionProject, Range, SlashCommand,
    zed::extension::context_server::ContextServerConfiguration,
    zed::extension::lsp::{
        Completion, CompletionKind, CompletionLabelDetails, InsertTextFormat, Symbol, SymbolKind,
    },
    zed::extension::slash_command::{SlashCommandArgumentCompletion, SlashCommandOutput},
};
pub use since_v0_0_4::LanguageServerConfig;

pub fn new_linker(
    executor: &BackgroundExecutor,
    f: impl FnOnce(&mut Linker<WasmState>) -> wasmtime::Result<()>,
) -> Linker<WasmState> {
    let mut linker = Linker::new(&wasm_engine(executor));
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();
    f(&mut linker).unwrap();
    linker
}

/// Returns whether the given Wasm API version is supported by the Wasm host.
pub fn is_supported_wasm_api_version(release_channel: ReleaseChannel, version: Version) -> bool {
    wasm_api_version_range(release_channel).contains(&version)
}

/// Returns the Wasm API version range that is supported by the Wasm host.
#[inline(always)]
pub fn wasm_api_version_range(release_channel: ReleaseChannel) -> RangeInclusive<Version> {
    // Note: The release channel can be used to stage a new version of the extension API.
    let _ = release_channel;

    let max_version = match release_channel {
        ReleaseChannel::Dev | ReleaseChannel::Nightly => latest::MAX_VERSION,
        ReleaseChannel::Stable | ReleaseChannel::Preview => since_v0_6_0::MAX_VERSION,
    };

    since_v0_0_1::MIN_VERSION..=max_version
}

/// Authorizes access to use unreleased versions of the Wasm API, based on the provided [`ReleaseChannel`].
///
/// Note: If there isn't currently an unreleased Wasm API version this function may be unused. Don't delete it!
pub fn authorize_access_to_unreleased_wasm_api_version(
    release_channel: ReleaseChannel,
) -> Result<()> {
    let allow_unreleased_version = match release_channel {
        ReleaseChannel::Dev | ReleaseChannel::Nightly => true,
        ReleaseChannel::Stable | ReleaseChannel::Preview => {
            // We always allow the latest in tests so that the extension tests pass on release branches.
            cfg!(any(test, feature = "test-support"))
        }
    };

    anyhow::ensure!(
        allow_unreleased_version,
        "unreleased versions of the extension API can only be used on development builds of Zed"
    );

    Ok(())
}

pub enum Extension {
    V0_8_0(since_v0_8_0::Extension),
    V0_6_0(since_v0_6_0::Extension),
    V0_5_0(since_v0_5_0::Extension),
    V0_4_0(since_v0_4_0::Extension),
    V0_3_0(since_v0_3_0::Extension),
    V0_2_0(since_v0_2_0::Extension),
    V0_1_0(since_v0_1_0::Extension),
    V0_0_6(since_v0_0_6::Extension),
    V0_0_4(since_v0_0_4::Extension),
    V0_0_1(since_v0_0_1::Extension),
}

impl Extension {
    pub async fn instantiate_async(
        executor: &BackgroundExecutor,
        store: &mut Store<WasmState>,
        release_channel: ReleaseChannel,
        version: Version,
        component: &Component,
    ) -> Result<Self> {
        // Note: The release channel can be used to stage a new version of the extension API.
        let _ = release_channel;

        if version >= latest::MIN_VERSION {
            authorize_access_to_unreleased_wasm_api_version(release_channel)?;

            let extension =
                latest::Extension::instantiate_async(store, component, latest::linker(executor))
                    .await
                    .map_err(anyhow::Error::from)
                    .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_8_0(extension))
        } else if version >= since_v0_6_0::MIN_VERSION {
            let extension = since_v0_6_0::Extension::instantiate_async(
                store,
                component,
                since_v0_6_0::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_6_0(extension))
        } else if version >= since_v0_5_0::MIN_VERSION {
            let extension = since_v0_5_0::Extension::instantiate_async(
                store,
                component,
                since_v0_5_0::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_5_0(extension))
        } else if version >= since_v0_4_0::MIN_VERSION {
            let extension = since_v0_4_0::Extension::instantiate_async(
                store,
                component,
                since_v0_4_0::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_4_0(extension))
        } else if version >= since_v0_3_0::MIN_VERSION {
            let extension = since_v0_3_0::Extension::instantiate_async(
                store,
                component,
                since_v0_3_0::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_3_0(extension))
        } else if version >= since_v0_2_0::MIN_VERSION {
            let extension = since_v0_2_0::Extension::instantiate_async(
                store,
                component,
                since_v0_2_0::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_2_0(extension))
        } else if version >= since_v0_1_0::MIN_VERSION {
            let extension = since_v0_1_0::Extension::instantiate_async(
                store,
                component,
                since_v0_1_0::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_1_0(extension))
        } else if version >= since_v0_0_6::MIN_VERSION {
            let extension = since_v0_0_6::Extension::instantiate_async(
                store,
                component,
                since_v0_0_6::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_0_6(extension))
        } else if version >= since_v0_0_4::MIN_VERSION {
            let extension = since_v0_0_4::Extension::instantiate_async(
                store,
                component,
                since_v0_0_4::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_0_4(extension))
        } else {
            let extension = since_v0_0_1::Extension::instantiate_async(
                store,
                component,
                since_v0_0_1::linker(executor),
            )
            .await
            .map_err(anyhow::Error::from)
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_0_1(extension))
        }
    }

    pub async fn call_init_extension(&self, store: &mut Store<WasmState>) -> wasmtime::Result<()> {
        match self {
            Extension::V0_8_0(ext) => ext.call_init_extension(store).await,
            Extension::V0_6_0(ext) => ext.call_init_extension(store).await,
            Extension::V0_5_0(ext) => ext.call_init_extension(store).await,
            Extension::V0_4_0(ext) => ext.call_init_extension(store).await,
            Extension::V0_3_0(ext) => ext.call_init_extension(store).await,
            Extension::V0_2_0(ext) => ext.call_init_extension(store).await,
            Extension::V0_1_0(ext) => ext.call_init_extension(store).await,
            Extension::V0_0_6(ext) => ext.call_init_extension(store).await,
            Extension::V0_0_4(ext) => ext.call_init_extension(store).await,
            Extension::V0_0_1(ext) => ext.call_init_extension(store).await,
        }
    }

    pub async fn call_language_server_command(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        language_name: &LanguageName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<Result<Command, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
            Extension::V0_3_0(ext) => {
                ext.call_language_server_command(store, &language_server_id.0, resource)
                    .await
            }
            Extension::V0_2_0(ext) => Ok(ext
                .call_language_server_command(store, &language_server_id.0, resource)
                .await?
                .map(|command| command.into())),
            Extension::V0_1_0(ext) => Ok(ext
                .call_language_server_command(store, &language_server_id.0, resource)
                .await?
                .map(|command| command.into())),
            Extension::V0_0_6(ext) => Ok(ext
                .call_language_server_command(store, &language_server_id.0, resource)
                .await?
                .map(|command| command.into())),
            Extension::V0_0_4(ext) => Ok(ext
                .call_language_server_command(
                    store,
                    &LanguageServerConfig {
                        name: language_server_id.0.to_string(),
                        language_name: language_name.to_string(),
                    },
                    resource,
                )
                .await?
                .map(|command| command.into())),
            Extension::V0_0_1(ext) => Ok(ext
                .call_language_server_command(
                    store,
                    &LanguageServerConfig {
                        name: language_server_id.0.to_string(),
                        language_name: language_name.to_string(),
                    }
                    .into(),
                    resource,
                )
                .await?
                .map(|command| command.into())),
        }
    }

    pub async fn call_language_server_initialization_options(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        language_name: &LanguageName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_3_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_2_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_1_0(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_0_6(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_0_4(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &LanguageServerConfig {
                        name: language_server_id.0.to_string(),
                        language_name: language_name.to_string(),
                    },
                    resource,
                )
                .await
            }
            Extension::V0_0_1(ext) => {
                ext.call_language_server_initialization_options(
                    store,
                    &LanguageServerConfig {
                        name: language_server_id.0.to_string(),
                        language_name: language_name.to_string(),
                    }
                    .into(),
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
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_3_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_2_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_1_0(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_0_6(ext) => {
                ext.call_language_server_workspace_configuration(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_0_4(_) | Extension::V0_0_1(_) => Ok(Ok(None)),
        }
    }

    pub async fn call_language_server_initialization_options_schema(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<Option<String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_initialization_options_schema(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_6_0(_)
            | Extension::V0_5_0(_)
            | Extension::V0_4_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Ok(None),
        }
    }

    pub async fn call_language_server_workspace_configuration_schema(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<Option<String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_workspace_configuration_schema(
                    store,
                    &language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_6_0(_)
            | Extension::V0_5_0(_)
            | Extension::V0_4_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Ok(None),
        }
    }

    pub async fn call_language_server_additional_initialization_options(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        target_language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_additional_initialization_options(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_language_server_additional_initialization_options(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_language_server_additional_initialization_options(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_language_server_additional_initialization_options(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Ok(Ok(None)),
        }
    }

    pub async fn call_language_server_additional_workspace_configuration(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        target_language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_language_server_additional_workspace_configuration(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_language_server_additional_workspace_configuration(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_language_server_additional_workspace_configuration(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_language_server_additional_workspace_configuration(
                    store,
                    &language_server_id.0,
                    &target_language_server_id.0,
                    resource,
                )
                .await
            }
            Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Ok(Ok(None)),
        }
    }

    pub async fn call_labels_for_completions(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        completions: Vec<latest::Completion>,
    ) -> wasmtime::Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_labels_for_completions(store, &language_server_id.0, &completions)
                    .await
            }
            Extension::V0_6_0(ext) => Ok(ext
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
            Extension::V0_5_0(ext) => Ok(ext
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
            Extension::V0_4_0(ext) => Ok(ext
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
            Extension::V0_3_0(ext) => Ok(ext
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
            Extension::V0_2_0(ext) => Ok(ext
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
            Extension::V0_1_0(ext) => Ok(ext
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
            Extension::V0_0_6(ext) => Ok(ext
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
            Extension::V0_0_1(_) | Extension::V0_0_4(_) => Ok(Ok(Vec::new())),
        }
    }

    pub async fn call_labels_for_symbols(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        symbols: Vec<latest::Symbol>,
    ) -> wasmtime::Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_labels_for_symbols(store, &language_server_id.0, &symbols)
                    .await
            }
            Extension::V0_6_0(ext) => Ok(ext
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
            Extension::V0_5_0(ext) => Ok(ext
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
            Extension::V0_4_0(ext) => Ok(ext
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
            Extension::V0_3_0(ext) => Ok(ext
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
            Extension::V0_2_0(ext) => Ok(ext
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
            Extension::V0_1_0(ext) => Ok(ext
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
            Extension::V0_0_6(ext) => Ok(ext
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
            Extension::V0_0_1(_) | Extension::V0_0_4(_) => Ok(Ok(Vec::new())),
        }
    }

    pub async fn call_complete_slash_command_argument(
        &self,
        store: &mut Store<WasmState>,
        command: &SlashCommand,
        arguments: &[String],
    ) -> wasmtime::Result<Result<Vec<SlashCommandArgumentCompletion>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V0_3_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V0_2_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V0_1_0(ext) => {
                ext.call_complete_slash_command_argument(store, command, arguments)
                    .await
            }
            Extension::V0_0_1(_) | Extension::V0_0_4(_) | Extension::V0_0_6(_) => {
                Ok(Ok(Vec::new()))
            }
        }
    }

    pub async fn call_run_slash_command(
        &self,
        store: &mut Store<WasmState>,
        command: &SlashCommand,
        arguments: &[String],
        resource: Option<Resource<Arc<dyn WorktreeDelegate>>>,
    ) -> wasmtime::Result<Result<SlashCommandOutput, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V0_3_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V0_2_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V0_1_0(ext) => {
                ext.call_run_slash_command(store, command, arguments, resource)
                    .await
            }
            Extension::V0_0_1(_) | Extension::V0_0_4(_) | Extension::V0_0_6(_) => Err(
                wasmtime::Error::msg("`run_slash_command` not available prior to v0.1.0"),
            ),
        }
    }

    pub async fn call_context_server_command(
        &self,
        store: &mut Store<WasmState>,
        context_server_id: Arc<str>,
        project: Resource<ExtensionProject>,
    ) -> wasmtime::Result<Result<Command, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_context_server_command(store, &context_server_id, project)
                    .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_context_server_command(store, &context_server_id, project)
                    .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_context_server_command(store, &context_server_id, project)
                    .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_context_server_command(store, &context_server_id, project)
                    .await
            }
            Extension::V0_3_0(ext) => {
                ext.call_context_server_command(store, &context_server_id, project)
                    .await
            }
            Extension::V0_2_0(ext) => Ok(ext
                .call_context_server_command(store, &context_server_id, project)
                .await?
                .map(Into::into)),
            Extension::V0_0_1(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_6(_)
            | Extension::V0_1_0(_) => Err(wasmtime::Error::msg(
                "`context_server_command` not available prior to v0.2.0",
            )),
        }
    }

    pub async fn call_context_server_configuration(
        &self,
        store: &mut Store<WasmState>,
        context_server_id: Arc<str>,
        project: Resource<ExtensionProject>,
    ) -> wasmtime::Result<Result<Option<ContextServerConfiguration>, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_context_server_configuration(store, &context_server_id, project)
                    .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_context_server_configuration(store, &context_server_id, project)
                    .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_context_server_configuration(store, &context_server_id, project)
                    .await
            }
            Extension::V0_0_1(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_6(_)
            | Extension::V0_1_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_4_0(_) => Err(wasmtime::Error::msg(
                "`context_server_configuration` not available prior to v0.5.0",
            )),
        }
    }

    pub async fn call_suggest_docs_packages(
        &self,
        store: &mut Store<WasmState>,
        provider: &str,
    ) -> wasmtime::Result<Result<Vec<String>, String>> {
        match self {
            Extension::V0_8_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_6_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_5_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_4_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_3_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_2_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_1_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_0_1(_) | Extension::V0_0_4(_) | Extension::V0_0_6(_) => Err(
                wasmtime::Error::msg("`suggest_docs_packages` not available prior to v0.1.0"),
            ),
        }
    }

    pub async fn call_index_docs(
        &self,
        store: &mut Store<WasmState>,
        provider: &str,
        package_name: &str,
        kv_store: Resource<Arc<dyn KeyValueStoreDelegate>>,
    ) -> wasmtime::Result<Result<(), String>> {
        match self {
            Extension::V0_8_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V0_6_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V0_5_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V0_4_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V0_3_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V0_2_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V0_1_0(ext) => {
                ext.call_index_docs(store, provider, package_name, kv_store)
                    .await
            }
            Extension::V0_0_1(_) | Extension::V0_0_4(_) | Extension::V0_0_6(_) => Err(
                wasmtime::Error::msg("`index_docs` not available prior to v0.1.0"),
            ),
        }
    }

    pub async fn call_get_dap_binary(
        &self,
        store: &mut Store<WasmState>,
        adapter_name: Arc<str>,
        task: DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<Result<DebugAdapterBinary, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                let dap_binary = ext
                    .call_get_dap_binary(
                        store,
                        &adapter_name,
                        &task.try_into().into_wasmtime_result()?,
                        user_installed_path.as_ref().and_then(|p| p.to_str()),
                        resource,
                    )
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                Ok(Ok(dap_binary))
            }
            Extension::V0_6_0(ext) => {
                let task: latest::DebugTaskDefinition = task.try_into().into_wasmtime_result()?;
                let dap_binary = ext
                    .call_get_dap_binary(
                        store,
                        &adapter_name,
                        &task.into(),
                        user_installed_path.as_ref().and_then(|p| p.to_str()),
                        resource,
                    )
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                Ok(Ok(dap_binary.into()))
            }
            Extension::V0_5_0(_)
            | Extension::V0_4_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Err(wasmtime::Error::msg(
                "`get_dap_binary` not available prior to v0.6.0",
            )),
        }
    }

    pub async fn call_dap_request_kind(
        &self,
        store: &mut Store<WasmState>,
        adapter_name: Arc<str>,
        config: serde_json::Value,
    ) -> wasmtime::Result<Result<StartDebuggingRequestArgumentsRequest, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                let config = serde_json::to_string(&config)
                    .context("Adapter config is not a valid JSON")
                    .into_wasmtime_result()?;
                let dap_binary = ext
                    .call_dap_request_kind(store, &adapter_name, &config)
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                Ok(Ok(dap_binary))
            }
            Extension::V0_6_0(ext) => {
                let config = serde_json::to_string(&config)
                    .context("Adapter config is not a valid JSON")
                    .into_wasmtime_result()?;
                let dap_binary = ext
                    .call_dap_request_kind(store, &adapter_name, &config)
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                Ok(Ok(dap_binary.into()))
            }
            Extension::V0_5_0(_)
            | Extension::V0_4_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Err(wasmtime::Error::msg(
                "`dap_request_kind` not available prior to v0.6.0",
            )),
        }
    }

    pub async fn call_dap_config_to_scenario(
        &self,
        store: &mut Store<WasmState>,
        config: ZedDebugConfig,
    ) -> wasmtime::Result<Result<DebugScenario, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                let config = config.into();
                let dap_binary = ext
                    .call_dap_config_to_scenario(store, &config)
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                Ok(Ok(dap_binary.try_into().into_wasmtime_result()?))
            }
            Extension::V0_6_0(ext) => {
                let config: latest::DebugConfig = config.into();
                let dap_binary = ext
                    .call_dap_config_to_scenario(store, &config.into())
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                let dap_binary: latest::DebugScenario = dap_binary.into();
                Ok(Ok(dap_binary.try_into().into_wasmtime_result()?))
            }
            Extension::V0_5_0(_)
            | Extension::V0_4_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Err(wasmtime::Error::msg(
                "`dap_config_to_scenario` not available prior to v0.6.0",
            )),
        }
    }

    pub async fn call_dap_locator_create_scenario(
        &self,
        store: &mut Store<WasmState>,
        locator_name: String,
        build_config_template: TaskTemplate,
        resolved_label: String,
        debug_adapter_name: String,
    ) -> wasmtime::Result<Option<DebugScenario>> {
        match self {
            Extension::V0_8_0(ext) => {
                let build_config_template = build_config_template.into();
                let dap_binary = ext
                    .call_dap_locator_create_scenario(
                        store,
                        &locator_name,
                        &build_config_template,
                        &resolved_label,
                        &debug_adapter_name,
                    )
                    .await?;

                Ok(dap_binary
                    .map(TryInto::try_into)
                    .transpose()
                    .into_wasmtime_result()?)
            }
            Extension::V0_6_0(ext) => {
                let build_config_template: latest::dap::TaskTemplate = build_config_template.into();
                let dap_binary = ext
                    .call_dap_locator_create_scenario(
                        store,
                        &locator_name,
                        &build_config_template.into(),
                        &resolved_label,
                        &debug_adapter_name,
                    )
                    .await?;

                Ok(dap_binary
                    .map(|s| latest::DebugScenario::from(s).try_into())
                    .transpose()
                    .into_wasmtime_result()?)
            }
            Extension::V0_5_0(_)
            | Extension::V0_4_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Err(wasmtime::Error::msg(
                "`dap_locator_create_scenario` not available prior to v0.6.0",
            )),
        }
    }

    pub async fn call_run_dap_locator(
        &self,
        store: &mut Store<WasmState>,
        locator_name: String,
        resolved_build_task: SpawnInTerminal,
    ) -> wasmtime::Result<Result<DebugRequest, String>> {
        match self {
            Extension::V0_8_0(ext) => {
                let build_config_template =
                    resolved_build_task.try_into().into_wasmtime_result()?;
                let dap_request = ext
                    .call_run_dap_locator(store, &locator_name, &build_config_template)
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                Ok(Ok(dap_request.into()))
            }
            Extension::V0_6_0(ext) => {
                let build_config_template: latest::dap::TaskTemplate =
                    resolved_build_task.try_into().into_wasmtime_result()?;
                let dap_request = ext
                    .call_run_dap_locator(store, &locator_name, &build_config_template.into())
                    .await?
                    .map_err(|error| wasmtime::Error::msg(format!("{error:?}")))?;

                let dap_request: latest::DebugRequest = dap_request.into();
                Ok(Ok(dap_request.into()))
            }
            Extension::V0_5_0(_)
            | Extension::V0_4_0(_)
            | Extension::V0_3_0(_)
            | Extension::V0_2_0(_)
            | Extension::V0_1_0(_)
            | Extension::V0_0_6(_)
            | Extension::V0_0_4(_)
            | Extension::V0_0_1(_) => Err(wasmtime::Error::msg(
                "`run_dap_locator` not available prior to v0.6.0",
            )),
        }
    }
}

trait IntoWasmtimeResult<T> {
    fn into_wasmtime_result(self) -> wasmtime::Result<T>;
}

impl<T> IntoWasmtimeResult<T> for Result<T> {
    fn into_wasmtime_result(self) -> wasmtime::Result<T> {
        self.map_err(wasmtime::Error::from_anyhow)
    }
}

trait ToWasmtimeResult<T> {
    fn to_wasmtime_result(self) -> wasmtime::Result<Result<T, String>>;
}

impl<T> ToWasmtimeResult<T> for Result<T> {
    fn to_wasmtime_result(self) -> wasmtime::Result<Result<T, String>> {
        Ok(self.map_err(|error| format!("{error:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{capability_granter::CapabilityGranter, wasm_host::WasmHost};
    use extension::{ExtensionHostProxy, ExtensionManifest};
    use fs::{FakeFs, Fs};
    use gpui::{TestAppContext, UpdateGlobal as _};
    use http_client::{FakeHttpClient, Response};
    use node_runtime::NodeRuntime;
    use project::binary_downloads::{self, BinaryDownloads, ToolInstall};
    use settings::SettingsStore;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use wasmtime::component::ResourceTable;
    use wasmtime_wasi::WasiCtxBuilder;

    #[gpui::test]
    async fn test_extension_download_permission_across_api_versions(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            release_channel::init(Version::new(0, 0, 0), cx);
            extension::init(cx);
            binary_downloads::init(cx);
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
        });
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(PathBuf::from("/work/test-extension").as_path())
            .await
            .unwrap();
        let requests = Arc::new(AtomicUsize::new(0));
        let client = FakeHttpClient::create({
            let requests = requests.clone();
            move |_| {
                requests.fetch_add(1, Ordering::SeqCst);
                async { Ok(Response::builder().status(200).body("binary".into())?) }
            }
        });
        let host = cx.update(|cx| {
            WasmHost::new(
                fs.clone(),
                client,
                NodeRuntime::unavailable(),
                Arc::new(ExtensionHostProxy::default()),
                PathBuf::from("/work"),
                cx,
            )
        });
        let manifest = toml::from_str::<ExtensionManifest>(
            "id = 'test-extension'\nname = 'Test'\nversion = '1.0.0'\nschema_version = 0",
        )
        .unwrap();
        let manifest = Arc::new(manifest);
        let mut state = WasmState {
            capability_granter: CapabilityGranter::new(
                host.granted_capabilities.clone(),
                manifest.clone(),
            ),
            manifest,
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().build(),
            host,
            language_server_status_source: None,
        };
        let tool = "extension `test-extension`";
        let expected_error = util::downloads_disabled_error(tool);
        for approval in [None, Some(tool)] {
            let allowed = approval == Some(tool);
            if let Some(approval) = approval {
                cx.update(|cx| {
                    BinaryDownloads::try_get_global(cx)
                        .unwrap()
                        .update(cx, |store, cx| {
                            store.approve_tool_install(None, approval, cx);
                        });
                });
            }
            for legacy in [true, false] {
                let download = if legacy {
                    since_v0_1_0::ExtensionImports::download_file(
                        &mut state,
                        "https://example.com/tool".to_owned(),
                        "legacy".to_owned(),
                        since_v0_1_0::DownloadedFileType::Uncompressed,
                    )
                    .await
                    .unwrap()
                } else {
                    latest::ExtensionImports::download_file(
                        &mut state,
                        "https://example.com/tool".to_owned(),
                        "current".to_owned(),
                        latest::DownloadedFileType::Uncompressed,
                    )
                    .await
                    .unwrap()
                };
                for result in [
                    download,
                    fetch_extension_data(&mut state, legacy, false).await,
                    fetch_extension_data(&mut state, legacy, true).await,
                ] {
                    if allowed {
                        assert_eq!(result, Ok(()));
                    } else {
                        let error = result.expect_err("request must require approval");
                        assert_eq!(error.lines().next(), Some(expected_error.as_str()));
                    }
                }
            }
            assert_eq!(requests.load(Ordering::SeqCst), if allowed { 6 } else { 0 });
            assert_eq!(
                fs.is_file(PathBuf::from("/work/test-extension/legacy").as_path())
                    .await,
                allowed
            );
            assert_eq!(
                fs.is_file(PathBuf::from("/work/test-extension/current").as_path())
                    .await,
                allowed
            );
            cx.update(|cx| {
                assert_eq!(
                    BinaryDownloads::try_get_global(cx)
                        .unwrap()
                        .read(cx)
                        .pending_tool_installs(),
                    if allowed {
                        Vec::new()
                    } else {
                        vec![ToolInstall {
                            worktree_id: None,
                            tool: tool.into(),
                        }]
                    },
                );
            });
        }
    }

    async fn fetch_extension_data(
        state: &mut WasmState,
        legacy: bool,
        streaming: bool,
    ) -> Result<(), String> {
        if legacy {
            let request = since_v0_1_0::http_client::HttpRequest {
                method: since_v0_1_0::http_client::HttpMethod::Get,
                url: "https://example.com/tool".to_owned(),
                headers: Vec::new(),
                body: None,
                redirect_policy: since_v0_1_0::http_client::RedirectPolicy::FollowAll,
            };
            if streaming {
                since_v0_1_0::http_client::Host::fetch_stream(state, request)
                    .await
                    .unwrap()
                    .map(|_| ())
            } else {
                since_v0_1_0::http_client::Host::fetch(state, request)
                    .await
                    .unwrap()
                    .map(|_| ())
            }
        } else {
            let request = latest::http_client::HttpRequest {
                method: latest::http_client::HttpMethod::Get,
                url: "https://example.com/tool".to_owned(),
                headers: Vec::new(),
                body: None,
                redirect_policy: latest::http_client::RedirectPolicy::FollowAll,
            };
            if streaming {
                latest::http_client::Host::fetch_stream(state, request)
                    .await
                    .unwrap()
                    .map(|_| ())
            } else {
                latest::http_client::Host::fetch(state, request)
                    .await
                    .unwrap()
                    .map(|_| ())
            }
        }
    }
}
