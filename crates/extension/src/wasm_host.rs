use crate::ExtensionManifest;
use anyhow::{anyhow, bail, Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use fs::Fs;
use futures::{
    channel::{mpsc::UnboundedSender, oneshot},
    future::BoxFuture,
    io::BufReader,
    Future, FutureExt, StreamExt as _,
};
use gpui::BackgroundExecutor;
use language::{LanguageRegistry, LanguageServerBinaryStatus, LspAdapterDelegate};
use node_runtime::NodeRuntime;
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};
use util::{http::HttpClient, SemanticVersion};
use wasmtime::{
    component::{Component, Linker, Resource, ResourceTable},
    Engine, Store,
};
use wasmtime_wasi::preview2::{command as wasi_command, WasiCtx, WasiCtxBuilder, WasiView};

pub mod wit {
    wasmtime::component::bindgen!({
        async: true,
        path: "../extension_api/wit",
        with: {
             "worktree": super::ExtensionWorktree,
        },
    });
}

pub type ExtensionWorktree = Arc<dyn LspAdapterDelegate>;

pub(crate) struct WasmHost {
    engine: Engine,
    linker: Arc<wasmtime::component::Linker<WasmState>>,
    http_client: Arc<dyn HttpClient>,
    node_runtime: Arc<dyn NodeRuntime>,
    language_registry: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    pub(crate) work_dir: PathBuf,
}

#[derive(Clone)]
pub struct WasmExtension {
    tx: UnboundedSender<ExtensionCall>,
    #[allow(unused)]
    zed_api_version: SemanticVersion,
}

pub(crate) struct WasmState {
    manifest: Arc<ExtensionManifest>,
    table: ResourceTable,
    ctx: WasiCtx,
    host: Arc<WasmHost>,
}

type ExtensionCall = Box<
    dyn Send
        + for<'a> FnOnce(&'a mut wit::Extension, &'a mut Store<WasmState>) -> BoxFuture<'a, ()>,
>;

static WASM_ENGINE: OnceLock<wasmtime::Engine> = OnceLock::new();

impl WasmHost {
    pub fn new(
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        node_runtime: Arc<dyn NodeRuntime>,
        language_registry: Arc<LanguageRegistry>,
        work_dir: PathBuf,
    ) -> Arc<Self> {
        let engine = WASM_ENGINE
            .get_or_init(|| {
                let mut config = wasmtime::Config::new();
                config.wasm_component_model(true);
                config.async_support(true);
                wasmtime::Engine::new(&config).unwrap()
            })
            .clone();
        let mut linker = Linker::new(&engine);
        wasi_command::add_to_linker(&mut linker).unwrap();
        wit::Extension::add_to_linker(&mut linker, |state: &mut WasmState| state).unwrap();
        Arc::new(Self {
            engine,
            linker: Arc::new(linker),
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
            let component = Component::from_binary(&this.engine, &wasm_bytes)
                .context("failed to compile wasm component")?;

            let mut zed_api_version = None;
            for part in wasmparser::Parser::new(0).parse_all(&wasm_bytes) {
                if let wasmparser::Payload::CustomSection(s) = part? {
                    if s.name() == "zed:api-version" {
                        if s.data().len() != 6 {
                            bail!(
                                "extension {} has invalid zed:api-version section: {:?}",
                                manifest.id,
                                s.data()
                            );
                        }

                        let major = u16::from_be_bytes(s.data()[0..2].try_into().unwrap()) as _;
                        let minor = u16::from_be_bytes(s.data()[2..4].try_into().unwrap()) as _;
                        let patch = u16::from_be_bytes(s.data()[4..6].try_into().unwrap()) as _;
                        zed_api_version = Some(SemanticVersion {
                            major,
                            minor,
                            patch,
                        })
                    }
                }
            }

            let Some(zed_api_version) = zed_api_version else {
                bail!("extension {} has no zed:api-version section", manifest.id);
            };

            let mut store = wasmtime::Store::new(
                &this.engine,
                WasmState {
                    manifest,
                    table: ResourceTable::new(),
                    ctx: WasiCtxBuilder::new()
                        .inherit_stdio()
                        .env("RUST_BACKTRACE", "1")
                        .build(),
                    host: this.clone(),
                },
            );
            let (mut extension, instance) =
                wit::Extension::instantiate_async(&mut store, &component, &this.linker)
                    .await
                    .context("failed to instantiate wasm component")?;
            let (tx, mut rx) = futures::channel::mpsc::unbounded::<ExtensionCall>();
            executor
                .spawn(async move {
                    extension.call_init_extension(&mut store).await.unwrap();

                    let _instance = instance;
                    while let Some(call) = rx.next().await {
                        (call)(&mut extension, &mut store).await;
                    }
                })
                .detach();
            Ok(WasmExtension {
                tx,
                zed_api_version,
            })
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
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
        path: String,
    ) -> wasmtime::Result<Result<String, String>> {
        let delegate = self.table().get(&delegate)?;
        Ok(delegate
            .read_text_file(path.into())
            .await
            .map_err(|error| error.to_string()))
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

        Ok(inner(self, repo, options)
            .await
            .map_err(|err| err.to_string()))
    }

    async fn current_platform(&mut self) -> Result<(wit::Os, wit::Architecture)> {
        Ok((
            match std::env::consts::OS {
                "macos" => wit::Os::Mac,
                "linux" => wit::Os::Linux,
                "windows" => wit::Os::Windows,
                _ => panic!("unsupported os"),
            },
            match std::env::consts::ARCH {
                "aarch64" => wit::Architecture::Aarch64,
                "x86" => wit::Architecture::X86,
                "x86_64" => wit::Architecture::X8664,
                _ => panic!("unsupported architecture"),
            },
        ))
    }

    async fn set_language_server_installation_status(
        &mut self,
        server_name: String,
        status: wit::LanguageServerInstallationStatus,
    ) -> wasmtime::Result<()> {
        let status = match status {
            wit::LanguageServerInstallationStatus::CheckingForUpdate => {
                LanguageServerBinaryStatus::CheckingForUpdate
            }
            wit::LanguageServerInstallationStatus::Downloading => {
                LanguageServerBinaryStatus::Downloading
            }
            wit::LanguageServerInstallationStatus::Downloaded => {
                LanguageServerBinaryStatus::Downloaded
            }
            wit::LanguageServerInstallationStatus::Cached => LanguageServerBinaryStatus::Cached,
            wit::LanguageServerInstallationStatus::Failed(error) => {
                LanguageServerBinaryStatus::Failed { error }
            }
        };

        self.host
            .language_registry
            .update_lsp_status(language::LanguageServerName(server_name.into()), status);
        Ok(())
    }

    async fn download_file(
        &mut self,
        url: String,
        filename: String,
        file_type: wit::DownloadedFileType,
    ) -> wasmtime::Result<Result<(), String>> {
        async fn inner(
            this: &mut WasmState,
            url: String,
            filename: String,
            file_type: wit::DownloadedFileType,
        ) -> anyhow::Result<()> {
            this.host.fs.create_dir(&this.host.work_dir).await?;
            let container_dir = this.host.work_dir.join(this.manifest.id.as_ref());
            let destination_path = container_dir.join(&filename);

            let mut response = this
                .host
                .http_client
                .get(&url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;

            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            let body = BufReader::new(response.body_mut());

            match file_type {
                wit::DownloadedFileType::Uncompressed => {
                    futures::pin_mut!(body);
                    this.host
                        .fs
                        .create_file_with(&destination_path, body)
                        .await?;
                }
                wit::DownloadedFileType::Gzip => {
                    let body = GzipDecoder::new(body);
                    futures::pin_mut!(body);
                    this.host
                        .fs
                        .create_file_with(&destination_path, body)
                        .await?;
                }
                wit::DownloadedFileType::GzipTar => {
                    let body = GzipDecoder::new(body);
                    futures::pin_mut!(body);
                    this.host
                        .fs
                        .extract_tar_file(&destination_path, Archive::new(body))
                        .await?;
                }
                wit::DownloadedFileType::Zip => {
                    let zip_filename = format!("{filename}.zip");
                    let mut zip_path = destination_path.clone();
                    zip_path.set_file_name(zip_filename);
                    futures::pin_mut!(body);
                    this.host.fs.create_file_with(&zip_path, body).await?;

                    let unzip_status = std::process::Command::new("unzip")
                        .current_dir(&container_dir)
                        .arg(&zip_path)
                        .output()?
                        .status;
                    if !unzip_status.success() {
                        Err(anyhow!("failed to unzip {filename} archive"))?;
                    }
                }
            }

            Ok(())
        }

        Ok(inner(self, url, filename, file_type)
            .await
            .map(|_| ())
            .map_err(|err| err.to_string()))
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
