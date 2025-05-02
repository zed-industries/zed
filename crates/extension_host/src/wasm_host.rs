pub mod wit;

use crate::ExtensionManifest;
use anyhow::{Context as _, Result, anyhow, bail};
use async_trait::async_trait;
use extension::{
    CodeLabel, Command, Completion, ContextServerConfiguration, ExtensionHostProxy,
    KeyValueStoreDelegate, ProjectDelegate, SlashCommand, SlashCommandArgumentCompletion,
    SlashCommandOutput, Symbol, WorktreeDelegate,
};
use fs::{Fs, normalize_path};
use futures::future::LocalBoxFuture;
use futures::{
    Future, FutureExt, StreamExt as _,
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    future::BoxFuture,
};
use gpui::{App, AsyncApp, BackgroundExecutor, Task};
use http_client::HttpClient;
use language::LanguageName;
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;
use release_channel::ReleaseChannel;
use semantic_version::SemanticVersion;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use wasmtime::{
    Engine, Store,
    component::{Component, ResourceTable},
};
use wasmtime_wasi::{self as wasi, WasiView};
use wit::Extension;

pub struct WasmHost {
    engine: Engine,
    release_channel: ReleaseChannel,
    http_client: Arc<dyn HttpClient>,
    node_runtime: NodeRuntime,
    pub(crate) proxy: Arc<ExtensionHostProxy>,
    fs: Arc<dyn Fs>,
    pub work_dir: PathBuf,
    _main_thread_message_task: Task<()>,
    main_thread_message_tx: mpsc::UnboundedSender<MainThreadCall>,
}

#[derive(Clone)]
pub struct WasmExtension {
    tx: UnboundedSender<ExtensionCall>,
    pub manifest: Arc<ExtensionManifest>,
    pub work_dir: Arc<Path>,
    #[allow(unused)]
    pub zed_api_version: SemanticVersion,
}

#[async_trait]
impl extension::Extension for WasmExtension {
    fn manifest(&self) -> Arc<ExtensionManifest> {
        self.manifest.clone()
    }

    fn work_dir(&self) -> Arc<Path> {
        self.work_dir.clone()
    }

    async fn language_server_command(
        &self,
        language_server_id: LanguageServerName,
        language_name: LanguageName,
        worktree: Arc<dyn WorktreeDelegate>,
    ) -> Result<Command> {
        self.call(|extension, store| {
            async move {
                let resource = store.data_mut().table().push(worktree)?;
                let command = extension
                    .call_language_server_command(
                        store,
                        &language_server_id,
                        &language_name,
                        resource,
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(command.into())
            }
            .boxed()
        })
        .await
    }

    async fn language_server_initialization_options(
        &self,
        language_server_id: LanguageServerName,
        language_name: LanguageName,
        worktree: Arc<dyn WorktreeDelegate>,
    ) -> Result<Option<String>> {
        self.call(|extension, store| {
            async move {
                let resource = store.data_mut().table().push(worktree)?;
                let options = extension
                    .call_language_server_initialization_options(
                        store,
                        &language_server_id,
                        &language_name,
                        resource,
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await
    }

    async fn language_server_workspace_configuration(
        &self,
        language_server_id: LanguageServerName,
        worktree: Arc<dyn WorktreeDelegate>,
    ) -> Result<Option<String>> {
        self.call(|extension, store| {
            async move {
                let resource = store.data_mut().table().push(worktree)?;
                let options = extension
                    .call_language_server_workspace_configuration(
                        store,
                        &language_server_id,
                        resource,
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await
    }

    async fn language_server_additional_initialization_options(
        &self,
        language_server_id: LanguageServerName,
        target_language_server_id: LanguageServerName,
        worktree: Arc<dyn WorktreeDelegate>,
    ) -> Result<Option<String>> {
        self.call(|extension, store| {
            async move {
                let resource = store.data_mut().table().push(worktree)?;
                let options = extension
                    .call_language_server_additional_initialization_options(
                        store,
                        &language_server_id,
                        &target_language_server_id,
                        resource,
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await
    }

    async fn language_server_additional_workspace_configuration(
        &self,
        language_server_id: LanguageServerName,
        target_language_server_id: LanguageServerName,
        worktree: Arc<dyn WorktreeDelegate>,
    ) -> Result<Option<String>> {
        self.call(|extension, store| {
            async move {
                let resource = store.data_mut().table().push(worktree)?;
                let options = extension
                    .call_language_server_additional_workspace_configuration(
                        store,
                        &language_server_id,
                        &target_language_server_id,
                        resource,
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await
    }

    async fn labels_for_completions(
        &self,
        language_server_id: LanguageServerName,
        completions: Vec<Completion>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        self.call(|extension, store| {
            async move {
                let labels = extension
                    .call_labels_for_completions(
                        store,
                        &language_server_id,
                        completions.into_iter().map(Into::into).collect(),
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(labels
                    .into_iter()
                    .map(|label| label.map(Into::into))
                    .collect())
            }
            .boxed()
        })
        .await
    }

    async fn labels_for_symbols(
        &self,
        language_server_id: LanguageServerName,
        symbols: Vec<Symbol>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        self.call(|extension, store| {
            async move {
                let labels = extension
                    .call_labels_for_symbols(
                        store,
                        &language_server_id,
                        symbols.into_iter().map(Into::into).collect(),
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(labels
                    .into_iter()
                    .map(|label| label.map(Into::into))
                    .collect())
            }
            .boxed()
        })
        .await
    }

    async fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        arguments: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>> {
        self.call(|extension, store| {
            async move {
                let completions = extension
                    .call_complete_slash_command_argument(store, &command.into(), &arguments)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(completions.into_iter().map(Into::into).collect())
            }
            .boxed()
        })
        .await
    }

    async fn run_slash_command(
        &self,
        command: SlashCommand,
        arguments: Vec<String>,
        delegate: Option<Arc<dyn WorktreeDelegate>>,
    ) -> Result<SlashCommandOutput> {
        self.call(|extension, store| {
            async move {
                let resource = if let Some(delegate) = delegate {
                    Some(store.data_mut().table().push(delegate)?)
                } else {
                    None
                };

                let output = extension
                    .call_run_slash_command(store, &command.into(), &arguments, resource)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(output.into())
            }
            .boxed()
        })
        .await
    }

    async fn context_server_command(
        &self,
        context_server_id: Arc<str>,
        project: Arc<dyn ProjectDelegate>,
    ) -> Result<Command> {
        self.call(|extension, store| {
            async move {
                let project_resource = store.data_mut().table().push(project)?;
                let command = extension
                    .call_context_server_command(store, context_server_id.clone(), project_resource)
                    .await?
                    .map_err(|err| anyhow!("{err}"))?;
                anyhow::Ok(command.into())
            }
            .boxed()
        })
        .await
    }

    async fn context_server_configuration(
        &self,
        context_server_id: Arc<str>,
        project: Arc<dyn ProjectDelegate>,
    ) -> Result<Option<ContextServerConfiguration>> {
        self.call(|extension, store| {
            async move {
                let project_resource = store.data_mut().table().push(project)?;
                let Some(configuration) = extension
                    .call_context_server_configuration(
                        store,
                        context_server_id.clone(),
                        project_resource,
                    )
                    .await?
                    .map_err(|err| anyhow!("{err}"))?
                else {
                    return Ok(None);
                };

                Ok(Some(configuration.try_into()?))
            }
            .boxed()
        })
        .await
    }

    async fn suggest_docs_packages(&self, provider: Arc<str>) -> Result<Vec<String>> {
        self.call(|extension, store| {
            async move {
                let packages = extension
                    .call_suggest_docs_packages(store, provider.as_ref())
                    .await?
                    .map_err(|err| anyhow!("{err:?}"))?;

                Ok(packages)
            }
            .boxed()
        })
        .await
    }

    async fn index_docs(
        &self,
        provider: Arc<str>,
        package_name: Arc<str>,
        kv_store: Arc<dyn KeyValueStoreDelegate>,
    ) -> Result<()> {
        self.call(|extension, store| {
            async move {
                let kv_store_resource = store.data_mut().table().push(kv_store)?;
                extension
                    .call_index_docs(
                        store,
                        provider.as_ref(),
                        package_name.as_ref(),
                        kv_store_resource,
                    )
                    .await?
                    .map_err(|err| anyhow!("{err:?}"))?;

                anyhow::Ok(())
            }
            .boxed()
        })
        .await
    }
}

pub struct WasmState {
    manifest: Arc<ExtensionManifest>,
    pub table: ResourceTable,
    ctx: wasi::WasiCtx,
    pub host: Arc<WasmHost>,
}

type MainThreadCall = Box<dyn Send + for<'a> FnOnce(&'a mut AsyncApp) -> LocalBoxFuture<'a, ()>>;

type ExtensionCall = Box<
    dyn Send + for<'a> FnOnce(&'a mut Extension, &'a mut Store<WasmState>) -> BoxFuture<'a, ()>,
>;

fn wasm_engine() -> wasmtime::Engine {
    static WASM_ENGINE: OnceLock<wasmtime::Engine> = OnceLock::new();

    WASM_ENGINE
        .get_or_init(|| {
            let mut config = wasmtime::Config::new();
            config.wasm_component_model(true);
            config.async_support(true);
            wasmtime::Engine::new(&config).unwrap()
        })
        .clone()
}

impl WasmHost {
    pub fn new(
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        node_runtime: NodeRuntime,
        proxy: Arc<ExtensionHostProxy>,
        work_dir: PathBuf,
        cx: &mut App,
    ) -> Arc<Self> {
        let (tx, mut rx) = mpsc::unbounded::<MainThreadCall>();
        let task = cx.spawn(async move |cx| {
            while let Some(message) = rx.next().await {
                message(cx).await;
            }
        });
        Arc::new(Self {
            engine: wasm_engine(),
            fs,
            work_dir,
            http_client,
            node_runtime,
            proxy,
            release_channel: ReleaseChannel::global(cx),
            _main_thread_message_task: task,
            main_thread_message_tx: tx,
        })
    }

    pub fn load_extension(
        self: &Arc<Self>,
        wasm_bytes: Vec<u8>,
        manifest: &Arc<ExtensionManifest>,
        executor: BackgroundExecutor,
    ) -> Task<Result<WasmExtension>> {
        let this = self.clone();
        let manifest = manifest.clone();
        executor.clone().spawn(async move {
            let zed_api_version = parse_wasm_extension_version(&manifest.id, &wasm_bytes)?;

            let component = Component::from_binary(&this.engine, &wasm_bytes)
                .context("failed to compile wasm component")?;

            let mut store = wasmtime::Store::new(
                &this.engine,
                WasmState {
                    ctx: this.build_wasi_ctx(&manifest).await?,
                    manifest: manifest.clone(),
                    table: ResourceTable::new(),
                    host: this.clone(),
                },
            );

            let mut extension = Extension::instantiate_async(
                &mut store,
                this.release_channel,
                zed_api_version,
                &component,
            )
            .await?;

            extension
                .call_init_extension(&mut store)
                .await
                .context("failed to initialize wasm extension")?;

            let (tx, mut rx) = mpsc::unbounded::<ExtensionCall>();
            executor
                .spawn(async move {
                    while let Some(call) = rx.next().await {
                        (call)(&mut extension, &mut store).await;
                    }
                })
                .detach();

            Ok(WasmExtension {
                manifest: manifest.clone(),
                work_dir: this.work_dir.join(manifest.id.as_ref()).into(),
                tx,
                zed_api_version,
            })
        })
    }

    async fn build_wasi_ctx(&self, manifest: &Arc<ExtensionManifest>) -> Result<wasi::WasiCtx> {
        let extension_work_dir = self.work_dir.join(manifest.id.as_ref());
        self.fs
            .create_dir(&extension_work_dir)
            .await
            .context("failed to create extension work dir")?;

        let file_perms = wasi::FilePerms::all();
        let dir_perms = wasi::DirPerms::all();

        Ok(wasi::WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(&extension_work_dir, ".", dir_perms, file_perms)?
            .preopened_dir(
                &extension_work_dir,
                extension_work_dir.to_string_lossy(),
                dir_perms,
                file_perms,
            )?
            .env("PWD", extension_work_dir.to_string_lossy())
            .env("RUST_BACKTRACE", "full")
            .build())
    }

    pub fn writeable_path_from_extension(&self, id: &Arc<str>, path: &Path) -> Result<PathBuf> {
        let extension_work_dir = self.work_dir.join(id.as_ref());
        let path = normalize_path(&extension_work_dir.join(path));
        if path.starts_with(&extension_work_dir) {
            Ok(path)
        } else {
            Err(anyhow!("cannot write to path {}", path.display()))
        }
    }
}

pub fn parse_wasm_extension_version(
    extension_id: &str,
    wasm_bytes: &[u8],
) -> Result<SemanticVersion> {
    let mut version = None;

    for part in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
        if let wasmparser::Payload::CustomSection(s) =
            part.context("error parsing wasm extension")?
        {
            if s.name() == "zed:api-version" {
                version = parse_wasm_extension_version_custom_section(s.data());
                if version.is_none() {
                    bail!(
                        "extension {} has invalid zed:api-version section: {:?}",
                        extension_id,
                        s.data()
                    );
                }
            }
        }
    }

    // The reason we wait until we're done parsing all of the Wasm bytes to return the version
    // is to work around a panic that can happen inside of Wasmtime when the bytes are invalid.
    //
    // By parsing the entirety of the Wasm bytes before we return, we're able to detect this problem
    // earlier as an `Err` rather than as a panic.
    version.ok_or_else(|| anyhow!("extension {} has no zed:api-version section", extension_id))
}

fn parse_wasm_extension_version_custom_section(data: &[u8]) -> Option<SemanticVersion> {
    if data.len() == 6 {
        Some(SemanticVersion::new(
            u16::from_be_bytes([data[0], data[1]]) as _,
            u16::from_be_bytes([data[2], data[3]]) as _,
            u16::from_be_bytes([data[4], data[5]]) as _,
        ))
    } else {
        None
    }
}

impl WasmExtension {
    pub async fn load(
        extension_dir: PathBuf,
        manifest: &Arc<ExtensionManifest>,
        wasm_host: Arc<WasmHost>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let path = extension_dir.join("extension.wasm");

        let mut wasm_file = wasm_host
            .fs
            .open_sync(&path)
            .await
            .context("failed to open wasm file")?;

        let mut wasm_bytes = Vec::new();
        wasm_file
            .read_to_end(&mut wasm_bytes)
            .context("failed to read wasm")?;

        wasm_host
            .load_extension(wasm_bytes, manifest, cx.background_executor().clone())
            .await
            .with_context(|| format!("failed to load wasm extension {}", manifest.id))
    }

    pub async fn call<T, Fn>(&self, f: Fn) -> T
    where
        T: 'static + Send,
        Fn: 'static
            + Send
            + for<'a> FnOnce(&'a mut Extension, &'a mut Store<WasmState>) -> BoxFuture<'a, T>,
    {
        let (return_tx, return_rx) = oneshot::channel();
        self.tx
            .clone()
            .unbounded_send(Box::new(move |extension, store| {
                async {
                    let result = f(extension, store).await;
                    return_tx.send(result).ok();
                }
                .boxed()
            }))
            .expect("wasm extension channel should not be closed yet");
        return_rx.await.expect("wasm extension channel")
    }
}

impl WasmState {
    fn on_main_thread<T, Fn>(&self, f: Fn) -> impl 'static + Future<Output = T>
    where
        T: 'static + Send,
        Fn: 'static + Send + for<'a> FnOnce(&'a mut AsyncApp) -> LocalBoxFuture<'a, T>,
    {
        let (return_tx, return_rx) = oneshot::channel();
        self.host
            .main_thread_message_tx
            .clone()
            .unbounded_send(Box::new(move |cx| {
                async {
                    let result = f(cx).await;
                    return_tx.send(result).ok();
                }
                .boxed_local()
            }))
            .expect("main thread message channel should not be closed yet");
        async move { return_rx.await.expect("main thread message channel") }
    }

    fn work_dir(&self) -> PathBuf {
        self.host.work_dir.join(self.manifest.id.as_ref())
    }
}

impl wasi::WasiView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasi::WasiCtx {
        &mut self.ctx
    }
}
