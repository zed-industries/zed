use anyhow::{Context as _, Result};
use async_trait::async_trait;
use fs::Fs;
use futures::{
    channel::{mpsc::UnboundedSender, oneshot},
    future::BoxFuture,
    Future, FutureExt, StreamExt as _,
};
use gpui::BackgroundExecutor;
use language::WASM_ENGINE;
use node_runtime::NodeRuntime;
use std::sync::Arc;
use util::http::HttpClient;
use wasmtime::{
    component::{Component, Linker, Resource, ResourceTable},
    Engine, Store,
};
use wasmtime_wasi::preview2::{command as wasi_command, WasiCtx, WasiCtxBuilder, WasiView};

pub mod wit {
    wasmtime::component::bindgen!({
        async: true,
        with: {
             "worktree": project::worktree::Snapshot,
        },
    });
}

pub(crate) struct WasmHost {
    engine: Engine,
    linker: Arc<wasmtime::component::Linker<WasmState>>,
    http_client: Arc<dyn HttpClient>,
    node_runtime: Arc<dyn NodeRuntime>,
    fs: Arc<dyn Fs>,
}

#[derive(Clone)]
pub struct WasmExtension {
    tx: UnboundedSender<ExtensionCall>,
}

pub(crate) struct WasmState {
    table: ResourceTable,
    ctx: WasiCtx,
    host: Arc<WasmHost>,
}

type ExtensionCall = Box<
    dyn Send
        + for<'a> FnOnce(&'a mut wit::Extension, &'a mut Store<WasmState>) -> BoxFuture<'a, ()>,
>;

impl WasmHost {
    pub fn new(
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        node_runtime: Arc<dyn NodeRuntime>,
    ) -> Arc<Self> {
        let engine = WASM_ENGINE.clone();
        let mut linker = Linker::new(&engine);
        wasi_command::add_to_linker(&mut linker).unwrap();
        wit::Extension::add_to_linker(&mut linker, |state: &mut WasmState| state).unwrap();
        Arc::new(Self {
            engine,
            linker: Arc::new(linker),
            fs,
            http_client,
            node_runtime,
        })
    }

    pub fn load_extension(
        self: &Arc<Self>,
        wasm_bytes: Vec<u8>,
        executor: BackgroundExecutor,
    ) -> impl 'static + Future<Output = Result<WasmExtension>> {
        let this = self.clone();
        async move {
            let component = Component::from_binary(&this.engine, &wasm_bytes)
                .context("failed to compile wasm component")?;
            let mut store = wasmtime::Store::new(
                &this.engine,
                WasmState {
                    table: ResourceTable::new(),
                    ctx: WasiCtxBuilder::new().inherit_stdio().build(),
                    host: this.clone(),
                },
            );
            let (mut extension, instance) =
                wit::Extension::instantiate_async(&mut store, &component, &this.linker)
                    .await
                    .context("failed to insantiate wasm component")?;
            let (tx, mut rx) = futures::channel::mpsc::unbounded::<ExtensionCall>();
            executor
                .spawn(async move {
                    let _instance = instance;
                    while let Some(call) = rx.next().await {
                        (call)(&mut extension, &mut store).await;
                    }
                })
                .detach();
            Ok(WasmExtension { tx })
        }
    }
}

impl WasmExtension {
    pub async fn call<T, Fn>(&self, f: Fn) -> T
    where
        T: 'static + Send,
        Fn: 'static
            + Send
            + for<'a> FnOnce(&'a mut wit::Extension, &'a mut Store<WasmState>) -> BoxFuture<'a, T>,
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

#[async_trait]
impl wit::HostWorktree for WasmState {
    async fn read_text_file(
        &mut self,
        worktree: Resource<project::worktree::Snapshot>,
        path: String,
    ) -> wasmtime::Result<Result<String, String>> {
        let tree = self.table().get(&worktree)?;
        if tree.entry_for_path(&path).is_none() {
            return Ok(Err(format!("no such path '{path}'")));
        }
        let path = tree.absolutize(path.as_ref())?;
        let content = self.host.fs.load(&path).await?;
        Ok(Ok(content))
    }

    fn drop(&mut self, _worktree: Resource<wit::Worktree>) -> Result<()> {
        // we only ever hand out borrows of worktrees
        Ok(())
    }
}

#[async_trait]
impl wit::ExtensionImports for WasmState {
    async fn npm_package_latest_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<String, String>> {
        async fn inner(this: &mut WasmState, package_name: String) -> anyhow::Result<String> {
            this.host
                .node_runtime
                .npm_package_latest_version(&package_name)
                .await
        }

        Ok(inner(self, package_name)
            .await
            .map_err(|err| err.to_string()))
    }

    async fn latest_github_release(
        &mut self,
        repo: String,
        options: wit::GithubReleaseOptions,
    ) -> wasmtime::Result<Result<wit::GithubRelease, String>> {
        async fn inner(
            this: &mut WasmState,
            repo: String,
            options: wit::GithubReleaseOptions,
        ) -> anyhow::Result<wit::GithubRelease> {
            let release = util::github::latest_github_release(
                &repo,
                options.require_assets,
                options.pre_release,
                this.host.http_client.clone(),
            )
            .await?;
            Ok(wit::GithubRelease {
                version: release.tag_name,
                assets: release
                    .assets
                    .into_iter()
                    .map(|asset| wit::GithubReleaseAsset {
                        name: asset.name,
                        download_url: asset.browser_download_url,
                    })
                    .collect(),
            })
        }

        Ok(inner(self, repo, options).await.map_err(|e| e.to_string()))
    }
}

impl WasiView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
