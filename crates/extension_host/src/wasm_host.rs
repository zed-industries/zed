pub mod wit;

use crate::capability_granter::CapabilityGranter;
use crate::{ExtensionManifest, ExtensionSettings};
use anyhow::{Context as _, Result, anyhow, bail};
use async_trait::async_trait;
use dap::{DebugRequest, StartDebuggingRequestArgumentsRequest};
use extension::{
    CodeLabel, Command, Completion, ContextServerConfiguration, DebugAdapterBinary,
    DebugTaskDefinition, ExtensionCapability, ExtensionHostProxy, KeyValueStoreDelegate,
    ProjectDelegate, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, Symbol,
    WorktreeDelegate,
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
use gpui::{App, AsyncApp, BackgroundExecutor, Task, Timer};
use http_client::HttpClient;
use language::LanguageName;
use lsp::LanguageServerName;
use moka::sync::Cache;
use node_runtime::NodeRuntime;
use release_channel::ReleaseChannel;
use semver::Version;
use settings::Settings;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::{
        Arc, LazyLock, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use task::{DebugScenario, SpawnInTerminal, TaskTemplate, ZedDebugConfig};
use util::paths::SanitizedPath;
use wasmtime::{
    CacheStore, Engine, Store,
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
    /// The capabilities granted to extensions running on the host.
    pub(crate) granted_capabilities: Vec<ExtensionCapability>,
    _main_thread_message_task: Task<()>,
    main_thread_message_tx: mpsc::UnboundedSender<MainThreadCall>,
}

#[derive(Clone, Debug)]
pub struct WasmExtension {
    tx: UnboundedSender<ExtensionCall>,
    pub manifest: Arc<ExtensionManifest>,
    pub work_dir: Arc<Path>,
    #[allow(unused)]
    pub zed_api_version: Version,
    _task: Arc<Task<Result<(), gpui_tokio::JoinError>>>,
}

impl Drop for WasmExtension {
    fn drop(&mut self) {
        self.tx.close_channel();
    }
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
                    .map_err(|err| store.data().extension_error(err))?;

                Ok(command.into())
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;
                anyhow::Ok(options)
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;

                Ok(labels
                    .into_iter()
                    .map(|label| label.map(Into::into))
                    .collect())
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;

                Ok(labels
                    .into_iter()
                    .map(|label| label.map(Into::into))
                    .collect())
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;

                Ok(completions.into_iter().map(Into::into).collect())
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;

                Ok(output.into())
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;
                anyhow::Ok(command.into())
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?
                else {
                    return Ok(None);
                };

                Ok(Some(configuration.try_into()?))
            }
            .boxed()
        })
        .await?
    }

    async fn suggest_docs_packages(&self, provider: Arc<str>) -> Result<Vec<String>> {
        self.call(|extension, store| {
            async move {
                let packages = extension
                    .call_suggest_docs_packages(store, provider.as_ref())
                    .await?
                    .map_err(|err| store.data().extension_error(err))?;

                Ok(packages)
            }
            .boxed()
        })
        .await?
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
                    .map_err(|err| store.data().extension_error(err))?;

                anyhow::Ok(())
            }
            .boxed()
        })
        .await?
    }

    async fn get_dap_binary(
        &self,
        dap_name: Arc<str>,
        config: DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        worktree: Arc<dyn WorktreeDelegate>,
    ) -> Result<DebugAdapterBinary> {
        self.call(|extension, store| {
            async move {
                let resource = store.data_mut().table().push(worktree)?;
                let dap_binary = extension
                    .call_get_dap_binary(store, dap_name, config, user_installed_path, resource)
                    .await?
                    .map_err(|err| store.data().extension_error(err))?;
                let dap_binary = dap_binary.try_into()?;
                Ok(dap_binary)
            }
            .boxed()
        })
        .await?
    }
    async fn dap_request_kind(
        &self,
        dap_name: Arc<str>,
        config: serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        self.call(|extension, store| {
            async move {
                let kind = extension
                    .call_dap_request_kind(store, dap_name, config)
                    .await?
                    .map_err(|err| store.data().extension_error(err))?;
                Ok(kind.into())
            }
            .boxed()
        })
        .await?
    }

    async fn dap_config_to_scenario(&self, config: ZedDebugConfig) -> Result<DebugScenario> {
        self.call(|extension, store| {
            async move {
                let kind = extension
                    .call_dap_config_to_scenario(store, config)
                    .await?
                    .map_err(|err| store.data().extension_error(err))?;
                Ok(kind)
            }
            .boxed()
        })
        .await?
    }

    async fn dap_locator_create_scenario(
        &self,
        locator_name: String,
        build_config_template: TaskTemplate,
        resolved_label: String,
        debug_adapter_name: String,
    ) -> Result<Option<DebugScenario>> {
        self.call(|extension, store| {
            async move {
                extension
                    .call_dap_locator_create_scenario(
                        store,
                        locator_name,
                        build_config_template,
                        resolved_label,
                        debug_adapter_name,
                    )
                    .await
            }
            .boxed()
        })
        .await?
    }
    async fn run_dap_locator(
        &self,
        locator_name: String,
        config: SpawnInTerminal,
    ) -> Result<DebugRequest> {
        self.call(|extension, store| {
            async move {
                extension
                    .call_run_dap_locator(store, locator_name, config)
                    .await?
                    .map_err(|err| store.data().extension_error(err))
            }
            .boxed()
        })
        .await?
    }
}

pub struct WasmState {
    manifest: Arc<ExtensionManifest>,
    pub table: ResourceTable,
    ctx: wasi::WasiCtx,
    pub host: Arc<WasmHost>,
    pub(crate) capability_granter: CapabilityGranter,
}

std::thread_local! {
    /// Used by the crash handler to ignore panics in extension-related threads.
    pub static IS_WASM_THREAD: AtomicBool = const { AtomicBool::new(false) };
}

type MainThreadCall = Box<dyn Send + for<'a> FnOnce(&'a mut AsyncApp) -> LocalBoxFuture<'a, ()>>;

type ExtensionCall = Box<
    dyn Send + for<'a> FnOnce(&'a mut Extension, &'a mut Store<WasmState>) -> BoxFuture<'a, ()>,
>;

fn wasm_engine(executor: &BackgroundExecutor) -> wasmtime::Engine {
    static WASM_ENGINE: OnceLock<wasmtime::Engine> = OnceLock::new();
    WASM_ENGINE
        .get_or_init(|| {
            let mut config = wasmtime::Config::new();
            config.wasm_component_model(true);
            config.async_support(true);
            config
                .enable_incremental_compilation(cache_store())
                .unwrap();
            // Async support introduces the issue that extension execution happens during `Future::poll`,
            // which could block an async thread.
            // https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#execution-in-poll
            //
            // Epoch interruption is a lightweight mechanism to allow the extensions to yield control
            // back to the executor at regular intervals.
            config.epoch_interruption(true);

            let engine = wasmtime::Engine::new(&config).unwrap();

            // It might be safer to do this on a non-async thread to make sure it makes progress
            // regardless of if extensions are blocking.
            // However, due to our current setup, this isn't a likely occurrence and we'd rather
            // not have a dedicated thread just for this. If it becomes an issue, we can consider
            // creating a separate thread for epoch interruption.
            let engine_ref = engine.weak();
            executor
                .spawn(async move {
                    // Somewhat arbitrary interval, as it isn't a guaranteed interval.
                    // But this is a rough upper bound for how long the extension execution can block on
                    // `Future::poll`.
                    const EPOCH_INTERVAL: Duration = Duration::from_millis(100);
                    let mut timer = Timer::interval(EPOCH_INTERVAL);
                    while (timer.next().await).is_some() {
                        // Exit the loop and thread once the engine is dropped.
                        let Some(engine) = engine_ref.upgrade() else {
                            break;
                        };
                        engine.increment_epoch();
                    }
                })
                .detach();

            engine
        })
        .clone()
}

fn cache_store() -> Arc<IncrementalCompilationCache> {
    static CACHE_STORE: LazyLock<Arc<IncrementalCompilationCache>> =
        LazyLock::new(|| Arc::new(IncrementalCompilationCache::new()));
    CACHE_STORE.clone()
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

        let extension_settings = ExtensionSettings::get_global(cx);

        Arc::new(Self {
            engine: wasm_engine(cx.background_executor()),
            fs,
            work_dir,
            http_client,
            node_runtime,
            proxy,
            release_channel: ReleaseChannel::global(cx),
            granted_capabilities: extension_settings.granted_capabilities.clone(),
            _main_thread_message_task: task,
            main_thread_message_tx: tx,
        })
    }

    pub fn load_extension(
        self: &Arc<Self>,
        wasm_bytes: Vec<u8>,
        manifest: &Arc<ExtensionManifest>,
        cx: &AsyncApp,
    ) -> Task<Result<WasmExtension>> {
        let this = self.clone();
        let manifest = manifest.clone();
        let executor = cx.background_executor().clone();
        let load_extension_task = async move {
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
                    capability_granter: CapabilityGranter::new(
                        this.granted_capabilities.clone(),
                        manifest.clone(),
                    ),
                },
            );
            // Store will yield after 1 tick, and get a new deadline of 1 tick after each yield.
            store.set_epoch_deadline(1);
            store.epoch_deadline_async_yield_and_update(1);

            let mut extension = Extension::instantiate_async(
                &executor,
                &mut store,
                this.release_channel,
                zed_api_version.clone(),
                &component,
            )
            .await?;

            extension
                .call_init_extension(&mut store)
                .await
                .context("failed to initialize wasm extension")?;

            let (tx, mut rx) = mpsc::unbounded::<ExtensionCall>();
            let extension_task = async move {
                // note: Setting the thread local here will slowly "poison" all tokio threads
                // causing us to not record their panics any longer.
                //
                // This is fine though, the main zed binary only uses tokio for livekit and wasm extensions.
                // Livekit seldom (if ever) panics 🤞 so the likelihood of us missing a panic in sentry is very low.
                IS_WASM_THREAD.with(|v| v.store(true, Ordering::Release));
                while let Some(call) = rx.next().await {
                    (call)(&mut extension, &mut store).await;
                }
            };

            anyhow::Ok((
                extension_task,
                manifest.clone(),
                this.work_dir.join(manifest.id.as_ref()).into(),
                tx,
                zed_api_version,
            ))
        };
        cx.spawn(async move |cx| {
            let (extension_task, manifest, work_dir, tx, zed_api_version) =
                cx.background_executor().spawn(load_extension_task).await?;
            // we need to run run the task in a tokio context as wasmtime_wasi may
            // call into tokio, accessing its runtime handle when we trigger the `engine.increment_epoch()` above.
            let task = Arc::new(gpui_tokio::Tokio::spawn(cx, extension_task)?);

            Ok(WasmExtension {
                manifest,
                work_dir,
                tx,
                zed_api_version,
                _task: task,
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
        let path = SanitizedPath::new(&extension_work_dir).to_string();
        #[cfg(target_os = "windows")]
        let path = path.replace('\\', "/");

        let mut ctx = wasi::WasiCtxBuilder::new();
        ctx.inherit_stdio()
            .env("PWD", &path)
            .env("RUST_BACKTRACE", "full");

        ctx.preopened_dir(&path, ".", dir_perms, file_perms)?;
        ctx.preopened_dir(&path, &path, dir_perms, file_perms)?;

        Ok(ctx.build())
    }

    pub fn writeable_path_from_extension(&self, id: &Arc<str>, path: &Path) -> Result<PathBuf> {
        let extension_work_dir = self.work_dir.join(id.as_ref());
        let path = normalize_path(&extension_work_dir.join(path));
        anyhow::ensure!(
            path.starts_with(&extension_work_dir),
            "cannot write to path {path:?}",
        );
        Ok(path)
    }
}

pub fn parse_wasm_extension_version(extension_id: &str, wasm_bytes: &[u8]) -> Result<Version> {
    let mut version = None;

    for part in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
        if let wasmparser::Payload::CustomSection(s) =
            part.context("error parsing wasm extension")?
            && s.name() == "zed:api-version"
        {
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

    // The reason we wait until we're done parsing all of the Wasm bytes to return the version
    // is to work around a panic that can happen inside of Wasmtime when the bytes are invalid.
    //
    // By parsing the entirety of the Wasm bytes before we return, we're able to detect this problem
    // earlier as an `Err` rather than as a panic.
    version.with_context(|| format!("extension {extension_id} has no zed:api-version section"))
}

fn parse_wasm_extension_version_custom_section(data: &[u8]) -> Option<Version> {
    if data.len() == 6 {
        Some(Version::new(
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
        extension_dir: &Path,
        manifest: &Arc<ExtensionManifest>,
        wasm_host: Arc<WasmHost>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let path = extension_dir.join("extension.wasm");

        let mut wasm_file = wasm_host
            .fs
            .open_sync(&path)
            .await
            .context(format!("opening wasm file, path: {path:?}"))?;

        let mut wasm_bytes = Vec::new();
        wasm_file
            .read_to_end(&mut wasm_bytes)
            .context(format!("reading wasm file, path: {path:?}"))?;

        wasm_host
            .load_extension(wasm_bytes, manifest, cx)
            .await
            .with_context(|| format!("loading wasm extension: {}", manifest.id))
    }

    pub async fn call<T, Fn>(&self, f: Fn) -> Result<T>
    where
        T: 'static + Send,
        Fn: 'static
            + Send
            + for<'a> FnOnce(&'a mut Extension, &'a mut Store<WasmState>) -> BoxFuture<'a, T>,
    {
        let (return_tx, return_rx) = oneshot::channel();
        self.tx
            .unbounded_send(Box::new(move |extension, store| {
                async {
                    let result = f(extension, store).await;
                    return_tx.send(result).ok();
                }
                .boxed()
            }))
            .map_err(|_| {
                anyhow!(
                    "wasm extension channel should not be closed yet, extension {} (id {})",
                    self.manifest.name,
                    self.manifest.id,
                )
            })?;
        return_rx.await.with_context(|| {
            format!(
                "wasm extension channel, extension {} (id {})",
                self.manifest.name, self.manifest.id,
            )
        })
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
            .unwrap_or_else(|_| {
                panic!(
                    "main thread message channel should not be closed yet, extension {} (id {})",
                    self.manifest.name, self.manifest.id,
                )
            });
        let name = self.manifest.name.clone();
        let id = self.manifest.id.clone();
        async move {
            return_rx.await.unwrap_or_else(|_| {
                panic!("main thread message channel, extension {name} (id {id})")
            })
        }
    }

    fn work_dir(&self) -> PathBuf {
        self.host.work_dir.join(self.manifest.id.as_ref())
    }

    fn extension_error(&self, message: String) -> anyhow::Error {
        anyhow!(
            "from extension \"{}\" version {}: {}",
            self.manifest.name,
            self.manifest.version,
            message
        )
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

/// Wrapper around a mini-moka bounded cache for storing incremental compilation artifacts.
/// Since wasm modules have many similar elements, this can save us a lot of work at the
/// cost of a small memory footprint. However, we don't want this to be unbounded, so we use
/// a LFU/LRU cache to evict less used cache entries.
#[derive(Debug)]
struct IncrementalCompilationCache {
    cache: Cache<Vec<u8>, Vec<u8>>,
}

impl IncrementalCompilationCache {
    fn new() -> Self {
        let cache = Cache::builder()
            // Cap this at 32 MB for now. Our extensions turn into roughly 512kb in the cache,
            // which means we could store 64 completely novel extensions in the cache, but in
            // practice we will more than that, which is more than enough for our use case.
            .max_capacity(32 * 1024 * 1024)
            .weigher(|k: &Vec<u8>, v: &Vec<u8>| (k.len() + v.len()).try_into().unwrap_or(u32::MAX))
            .build();
        Self { cache }
    }
}

impl CacheStore for IncrementalCompilationCache {
    fn get(&self, key: &[u8]) -> Option<Cow<'_, [u8]>> {
        self.cache.get(key).map(|v| v.into())
    }

    fn insert(&self, key: &[u8], value: Vec<u8>) -> bool {
        self.cache.insert(key.to_vec(), value);
        true
    }
}
