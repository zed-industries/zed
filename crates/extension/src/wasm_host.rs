pub(crate) mod wit;

use crate::ExtensionManifest;
use anyhow::{anyhow, bail, Context as _, Result};
use fs::{normalize_path, Fs};
use futures::{
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    future::BoxFuture,
    Future, FutureExt, StreamExt as _,
};
use gpui::BackgroundExecutor;
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use util::{http::HttpClient, SemanticVersion};
use wasmtime::{
    component::{Component, ResourceTable},
    Engine, Store,
};
use wasmtime_wasi as wasi;
use wit::Extension;

pub(crate) struct WasmHost {
    engine: Engine,
    http_client: Arc<dyn HttpClient>,
    node_runtime: Arc<dyn NodeRuntime>,
    pub(crate) language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    pub(crate) work_dir: PathBuf,
}

#[derive(Clone)]
pub struct WasmExtension {
    tx: UnboundedSender<ExtensionCall>,
    pub(crate) manifest: Arc<ExtensionManifest>,
    #[allow(unused)]
    pub zed_api_version: SemanticVersion,
}

pub(crate) struct WasmState {
    manifest: Arc<ExtensionManifest>,
    pub(crate) table: ResourceTable,
    ctx: wasi::WasiCtx,
    pub(crate) host: Arc<WasmHost>,
}

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
        node_runtime: Arc<dyn NodeRuntime>,
        language_registry: Arc<LanguageRegistry>,
        work_dir: PathBuf,
    ) -> Arc<Self> {
        Arc::new(Self {
            engine: wasm_engine(),
            fs,
            work_dir,
            http_client,
            node_runtime,
            language_registry,
        })
    }

    pub fn load_extension(
        self: &Arc<Self>,
        wasm_bytes: Vec<u8>,
        manifest: Arc<ExtensionManifest>,
        executor: BackgroundExecutor,
    ) -> impl 'static + Future<Output = Result<WasmExtension>> {
        let this = self.clone();
        async move {
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

            let (mut extension, instance) =
                Extension::instantiate_async(&mut store, zed_api_version, &component).await?;

            extension
                .call_init_extension(&mut store)
                .await
                .context("failed to initialize wasm extension")?;

            let (tx, mut rx) = mpsc::unbounded::<ExtensionCall>();
            executor
                .spawn(async move {
                    let _instance = instance;
                    while let Some(call) = rx.next().await {
                        (call)(&mut extension, &mut store).await;
                    }
                })
                .detach();

            Ok(WasmExtension {
                manifest,
                tx,
                zed_api_version,
            })
        }
    }

    async fn build_wasi_ctx(&self, manifest: &Arc<ExtensionManifest>) -> Result<wasi::WasiCtx> {
        use cap_std::{ambient_authority, fs::Dir};

        let extension_work_dir = self.work_dir.join(manifest.id.as_ref());
        self.fs
            .create_dir(&extension_work_dir)
            .await
            .context("failed to create extension work dir")?;

        let work_dir_preopen = Dir::open_ambient_dir(&extension_work_dir, ambient_authority())
            .context("failed to preopen extension work directory")?;
        let current_dir_preopen = work_dir_preopen
            .try_clone()
            .context("failed to preopen extension current directory")?;
        let extension_work_dir = extension_work_dir.to_string_lossy();

        let perms = wasi::FilePerms::all();
        let dir_perms = wasi::DirPerms::all();

        Ok(wasi::WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(current_dir_preopen, dir_perms, perms, ".")
            .preopened_dir(work_dir_preopen, dir_perms, perms, &extension_work_dir)
            .env("PWD", &extension_work_dir)
            .env("RUST_BACKTRACE", "full")
            .build())
    }

    pub fn path_from_extension(&self, id: &Arc<str>, path: &Path) -> PathBuf {
        let extension_work_dir = self.work_dir.join(id.as_ref());
        normalize_path(&extension_work_dir.join(path))
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
    for part in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
        if let wasmparser::Payload::CustomSection(s) = part? {
            if s.name() == "zed:api-version" {
                let version = parse_wasm_extension_version_custom_section(s.data());
                if let Some(version) = version {
                    return Ok(version);
                } else {
                    bail!(
                        "extension {} has invalid zed:api-version section: {:?}",
                        extension_id,
                        s.data()
                    );
                }
            }
        }
    }
    bail!("extension {} has no zed:api-version section", extension_id)
}

fn parse_wasm_extension_version_custom_section(data: &[u8]) -> Option<SemanticVersion> {
    if data.len() == 6 {
        Some(SemanticVersion {
            major: u16::from_be_bytes([data[0], data[1]]) as _,
            minor: u16::from_be_bytes([data[2], data[3]]) as _,
            patch: u16::from_be_bytes([data[4], data[5]]) as _,
        })
    } else {
        None
    }
}

impl WasmExtension {
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
