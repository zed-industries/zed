use crate::ExtensionManifest;
use anyhow::{anyhow, bail, Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use fs::{normalize_path, Fs};
use futures::{
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    future::BoxFuture,
    io::BufReader,
    Future, FutureExt, StreamExt as _,
};
use gpui::BackgroundExecutor;
use language::{LanguageRegistry, LanguageServerBinaryStatus, LspAdapterDelegate};
use node_runtime::NodeRuntime;
use std::{
    env,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use util::{http::HttpClient, SemanticVersion};
use wasmtime::{
    component::{Component, Linker, Resource, ResourceTable},
    Engine, Store,
};
use wasmtime_wasi::preview2::{self as wasi, WasiCtx};

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
    pub(crate) manifest: Arc<ExtensionManifest>,
    #[allow(unused)]
    zed_api_version: SemanticVersion,
}

pub(crate) struct WasmState {
    manifest: Arc<ExtensionManifest>,
    table: ResourceTable,
    ctx: wasi::WasiCtx,
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
        wasi::command::add_to_linker(&mut linker).unwrap();
        wit::Extension::add_to_linker(&mut linker, wasi_view).unwrap();
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
                        zed_api_version = parse_extension_version(s.data());
                        if zed_api_version.is_none() {
                            bail!(
                                "extension {} has invalid zed:api-version section: {:?}",
                                manifest.id,
                                s.data()
                            );
                        }
                    }
                }
            }

            let Some(zed_api_version) = zed_api_version else {
                bail!("extension {} has no zed:api-version section", manifest.id);
            };

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
                wit::Extension::instantiate_async(&mut store, &component, &this.linker)
                    .await
                    .context("failed to instantiate wasm extension")?;
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

    async fn build_wasi_ctx(&self, manifest: &Arc<ExtensionManifest>) -> Result<WasiCtx> {
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

fn parse_extension_version(data: &[u8]) -> Option<SemanticVersion> {
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
        let delegate = self.table.get(&delegate)?;
        Ok(delegate
            .read_text_file(path.into())
            .await
            .map_err(|error| error.to_string()))
    }

    async fn shell_env(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> wasmtime::Result<wit::EnvVars> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate.shell_env().await.into_iter().collect())
    }

    async fn which(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
        binary_name: String,
    ) -> wasmtime::Result<Option<String>> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate
            .which(binary_name.as_ref())
            .await
            .map(|path| path.to_string_lossy().to_string()))
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
            match env::consts::OS {
                "macos" => wit::Os::Mac,
                "linux" => wit::Os::Linux,
                "windows" => wit::Os::Windows,
                _ => panic!("unsupported os"),
            },
            match env::consts::ARCH {
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
        path: String,
        file_type: wit::DownloadedFileType,
    ) -> wasmtime::Result<Result<(), String>> {
        let path = PathBuf::from(path);

        async fn inner(
            this: &mut WasmState,
            url: String,
            path: PathBuf,
            file_type: wit::DownloadedFileType,
        ) -> anyhow::Result<()> {
            let extension_work_dir = this.host.work_dir.join(this.manifest.id.as_ref());

            this.host.fs.create_dir(&extension_work_dir).await?;

            let destination_path = this
                .host
                .writeable_path_from_extension(&this.manifest.id, &path)?;

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
                    let file_name = destination_path
                        .file_name()
                        .ok_or_else(|| anyhow!("invalid download path"))?
                        .to_string_lossy();
                    let zip_filename = format!("{file_name}.zip");
                    let mut zip_path = destination_path.clone();
                    zip_path.set_file_name(zip_filename);

                    futures::pin_mut!(body);
                    this.host.fs.create_file_with(&zip_path, body).await?;

                    let unzip_status = std::process::Command::new("unzip")
                        .current_dir(&extension_work_dir)
                        .arg(&zip_path)
                        .output()?
                        .status;
                    if !unzip_status.success() {
                        Err(anyhow!("failed to unzip {} archive", path.display()))?;
                    }
                }
            }

            Ok(())
        }

        Ok(inner(self, url, path, file_type)
            .await
            .map(|_| ())
            .map_err(|err| err.to_string()))
    }
}

fn wasi_view(state: &mut WasmState) -> &mut WasmState {
    state
}

impl wasi::WasiView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasi::WasiCtx {
        &mut self.ctx
    }
}
