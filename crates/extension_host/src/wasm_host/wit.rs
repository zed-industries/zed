mod since_v0_0_1;
mod since_v0_0_4;
mod since_v0_0_6;
mod since_v0_1_0;
mod since_v0_2_0;
mod since_v0_3_0;
mod since_v0_4_0;
use extension::{KeyValueStoreDelegate, WorktreeDelegate};
use language::LanguageName;
use lsp::LanguageServerName;
use release_channel::ReleaseChannel;
use since_v0_4_0 as latest;

use super::{WasmState, wasm_engine};
use anyhow::{Context as _, Result, anyhow};
use semantic_version::SemanticVersion;
use std::{ops::RangeInclusive, sync::Arc};
use wasmtime::{
    Store,
    component::{Component, Linker, Resource},
};

#[cfg(test)]
pub use latest::CodeLabelSpanLiteral;
pub use latest::{
    CodeLabel, CodeLabelSpan, Command, ExtensionProject, Range, SlashCommand,
    zed::extension::lsp::{
        Completion, CompletionKind, CompletionLabelDetails, InsertTextFormat, Symbol, SymbolKind,
    },
    zed::extension::slash_command::{SlashCommandArgumentCompletion, SlashCommandOutput},
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
        ReleaseChannel::Stable | ReleaseChannel::Preview => latest::MAX_VERSION,
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

    if !allow_unreleased_version {
        Err(anyhow!(
            "unreleased versions of the extension API can only be used on development builds of Zed"
        ))?;
    }

    Ok(())
}

pub enum Extension {
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
        store: &mut Store<WasmState>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        component: &Component,
    ) -> Result<Self> {
        // Note: The release channel can be used to stage a new version of the extension API.
        let _ = release_channel;

        if version >= latest::MIN_VERSION {
            let extension =
                latest::Extension::instantiate_async(store, component, latest::linker())
                    .await
                    .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_4_0(extension))
        } else if version >= since_v0_3_0::MIN_VERSION {
            let extension = since_v0_3_0::Extension::instantiate_async(
                store,
                component,
                since_v0_3_0::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_3_0(extension))
        } else if version >= since_v0_2_0::MIN_VERSION {
            let extension = since_v0_2_0::Extension::instantiate_async(
                store,
                component,
                since_v0_2_0::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_2_0(extension))
        } else if version >= since_v0_1_0::MIN_VERSION {
            let extension = since_v0_1_0::Extension::instantiate_async(
                store,
                component,
                since_v0_1_0::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_1_0(extension))
        } else if version >= since_v0_0_6::MIN_VERSION {
            let extension = since_v0_0_6::Extension::instantiate_async(
                store,
                component,
                since_v0_0_6::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_0_6(extension))
        } else if version >= since_v0_0_4::MIN_VERSION {
            let extension = since_v0_0_4::Extension::instantiate_async(
                store,
                component,
                since_v0_0_4::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_0_4(extension))
        } else {
            let extension = since_v0_0_1::Extension::instantiate_async(
                store,
                component,
                since_v0_0_1::linker(),
            )
            .await
            .context("failed to instantiate wasm extension")?;
            Ok(Self::V0_0_1(extension))
        }
    }

    pub async fn call_init_extension(&self, store: &mut Store<WasmState>) -> Result<()> {
        match self {
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
    ) -> Result<Result<Command, String>> {
        match self {
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
    ) -> Result<Result<Option<String>, String>> {
        match self {
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
    ) -> Result<Result<Option<String>, String>> {
        match self {
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

    pub async fn call_language_server_additional_initialization_options(
        &self,
        store: &mut Store<WasmState>,
        language_server_id: &LanguageServerName,
        target_language_server_id: &LanguageServerName,
        resource: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> Result<Result<Option<String>, String>> {
        match self {
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
    ) -> Result<Result<Option<String>, String>> {
        match self {
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
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V0_4_0(ext) => {
                ext.call_labels_for_completions(store, &language_server_id.0, &completions)
                    .await
            }
            Extension::V0_3_0(ext) => Ok(ext
                .call_labels_for_completions(
                    store,
                    &language_server_id.0,
                    &completions.into_iter().collect::<Vec<_>>(),
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
                    &completions.into_iter().collect::<Vec<_>>(),
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
    ) -> Result<Result<Vec<Option<CodeLabel>>, String>> {
        match self {
            Extension::V0_4_0(ext) => {
                ext.call_labels_for_symbols(store, &language_server_id.0, &symbols)
                    .await
            }
            Extension::V0_3_0(ext) => Ok(ext
                .call_labels_for_symbols(
                    store,
                    &language_server_id.0,
                    &symbols.into_iter().collect::<Vec<_>>(),
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
                    &symbols.into_iter().collect::<Vec<_>>(),
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
    ) -> Result<Result<Vec<SlashCommandArgumentCompletion>, String>> {
        match self {
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
    ) -> Result<Result<SlashCommandOutput, String>> {
        match self {
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
            Extension::V0_0_1(_) | Extension::V0_0_4(_) | Extension::V0_0_6(_) => {
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
            | Extension::V0_1_0(_) => Err(anyhow!(
                "`context_server_command` not available prior to v0.2.0"
            )),
        }
    }

    pub async fn call_suggest_docs_packages(
        &self,
        store: &mut Store<WasmState>,
        provider: &str,
    ) -> Result<Result<Vec<String>, String>> {
        match self {
            Extension::V0_4_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_3_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_2_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_1_0(ext) => ext.call_suggest_docs_packages(store, provider).await,
            Extension::V0_0_1(_) | Extension::V0_0_4(_) | Extension::V0_0_6(_) => Err(anyhow!(
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
            Extension::V0_0_1(_) | Extension::V0_0_4(_) | Extension::V0_0_6(_) => {
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
