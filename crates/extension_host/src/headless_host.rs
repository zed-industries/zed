use std::{path::PathBuf, sync::Arc};

use anyhow::{anyhow, Context as _, Result};
use client::{proto, TypedEnvelope};
use collections::{HashMap, HashSet};
use extension::{Extension, ExtensionManifest};
use fs::{Fs, RemoveOptions, RenameOptions};
use gpui::{AppContext, AsyncAppContext, Context, Model, ModelContext, Task, WeakModel};
use http_client::HttpClient;
use language::{LanguageConfig, LanguageName, LanguageQueries, LanguageRegistry, LoadedLanguage};
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;

use crate::{
    extension_lsp_adapter::ExtensionLspAdapter,
    wasm_host::{WasmExtension, WasmHost},
    ExtensionRegistrationHooks,
};

pub struct HeadlessExtensionStore {
    pub registration_hooks: Arc<dyn ExtensionRegistrationHooks>,
    pub fs: Arc<dyn Fs>,
    pub extension_dir: PathBuf,
    pub wasm_host: Arc<WasmHost>,
    pub loaded_extensions: HashMap<Arc<str>, Arc<str>>,
    pub loaded_languages: HashMap<Arc<str>, Vec<LanguageName>>,
    pub loaded_language_servers: HashMap<Arc<str>, Vec<(LanguageServerName, LanguageName)>>,
}

#[derive(Clone, Debug)]
pub struct ExtensionVersion {
    pub id: String,
    pub version: String,
    pub dev: bool,
}

impl HeadlessExtensionStore {
    pub fn new(
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        languages: Arc<LanguageRegistry>,
        extension_dir: PathBuf,
        node_runtime: NodeRuntime,
        cx: &mut AppContext,
    ) -> Model<Self> {
        let registration_hooks = Arc::new(HeadlessRegistrationHooks::new(languages.clone()));
        cx.new_model(|cx| Self {
            registration_hooks: registration_hooks.clone(),
            fs: fs.clone(),
            wasm_host: WasmHost::new(
                fs.clone(),
                http_client.clone(),
                node_runtime,
                registration_hooks,
                extension_dir.join("work"),
                cx,
            ),
            extension_dir,
            loaded_extensions: Default::default(),
            loaded_languages: Default::default(),
            loaded_language_servers: Default::default(),
        })
    }

    pub fn sync_extensions(
        &mut self,
        extensions: Vec<ExtensionVersion>,
        cx: &ModelContext<Self>,
    ) -> Task<Result<Vec<ExtensionVersion>>> {
        let on_client = HashSet::from_iter(extensions.iter().map(|e| e.id.as_str()));
        let to_remove: Vec<Arc<str>> = self
            .loaded_extensions
            .keys()
            .filter(|id| !on_client.contains(id.as_ref()))
            .cloned()
            .collect();
        let to_load: Vec<ExtensionVersion> = extensions
            .into_iter()
            .filter(|e| {
                if e.dev {
                    return true;
                }
                !self
                    .loaded_extensions
                    .get(e.id.as_str())
                    .is_some_and(|loaded| loaded.as_ref() == e.version.as_str())
            })
            .collect();

        cx.spawn(|this, mut cx| async move {
            let mut missing = Vec::new();

            for extension_id in to_remove {
                log::info!("removing extension: {}", extension_id);
                this.update(&mut cx, |this, cx| {
                    this.uninstall_extension(&extension_id, cx)
                })?
                .await?;
            }

            for extension in to_load {
                if let Err(e) = Self::load_extension(this.clone(), extension.clone(), &mut cx).await
                {
                    log::info!("failed to load extension: {}, {:?}", extension.id, e);
                    missing.push(extension)
                } else if extension.dev {
                    missing.push(extension)
                }
            }

            Ok(missing)
        })
    }

    pub async fn load_extension(
        this: WeakModel<Self>,
        extension: ExtensionVersion,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let (fs, wasm_host, extension_dir) = this.update(cx, |this, _cx| {
            this.loaded_extensions.insert(
                extension.id.clone().into(),
                extension.version.clone().into(),
            );
            (
                this.fs.clone(),
                this.wasm_host.clone(),
                this.extension_dir.join(&extension.id),
            )
        })?;

        let manifest = Arc::new(ExtensionManifest::load(fs.clone(), &extension_dir).await?);

        debug_assert!(!manifest.languages.is_empty() || !manifest.language_servers.is_empty());

        if manifest.version.as_ref() != extension.version.as_str() {
            anyhow::bail!(
                "mismatched versions: ({}) != ({})",
                manifest.version,
                extension.version
            )
        }

        for language_path in &manifest.languages {
            let language_path = extension_dir.join(language_path);
            let config = fs.load(&language_path.join("config.toml")).await?;
            let mut config = ::toml::from_str::<LanguageConfig>(&config)?;

            this.update(cx, |this, _cx| {
                this.loaded_languages
                    .entry(manifest.id.clone())
                    .or_default()
                    .push(config.name.clone());

                config.grammar = None;

                this.registration_hooks.register_language(
                    config.name.clone(),
                    None,
                    config.matcher.clone(),
                    Arc::new(move || {
                        Ok(LoadedLanguage {
                            config: config.clone(),
                            queries: LanguageQueries::default(),
                            context_provider: None,
                            toolchain_provider: None,
                        })
                    }),
                );
            })?;
        }

        if manifest.language_servers.is_empty() {
            return Ok(());
        }

        let wasm_extension: Arc<dyn Extension> =
            Arc::new(WasmExtension::load(extension_dir, &manifest, wasm_host.clone(), &cx).await?);

        for (language_server_name, language_server_config) in &manifest.language_servers {
            for language in language_server_config.languages() {
                this.update(cx, |this, _cx| {
                    this.loaded_language_servers
                        .entry(manifest.id.clone())
                        .or_default()
                        .push((language_server_name.clone(), language.clone()));
                    this.registration_hooks.register_lsp_adapter(
                        language.clone(),
                        ExtensionLspAdapter {
                            extension: wasm_extension.clone(),
                            language_server_id: language_server_name.clone(),
                            language_name: language,
                        },
                    );
                })?;
            }
        }

        Ok(())
    }

    fn uninstall_extension(
        &mut self,
        extension_id: &Arc<str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        self.loaded_extensions.remove(extension_id);
        let languages_to_remove = self
            .loaded_languages
            .remove(extension_id)
            .unwrap_or_default();
        self.registration_hooks
            .remove_languages(&languages_to_remove, &[]);
        for (language_server_name, language) in self
            .loaded_language_servers
            .remove(extension_id)
            .unwrap_or_default()
        {
            self.registration_hooks
                .remove_lsp_adapter(&language, &language_server_name);
        }

        let path = self.extension_dir.join(&extension_id.to_string());
        let fs = self.fs.clone();
        cx.spawn(|_, _| async move {
            fs.remove_dir(
                &path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
        })
    }

    pub fn install_extension(
        &mut self,
        extension: ExtensionVersion,
        tmp_path: PathBuf,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let path = self.extension_dir.join(&extension.id);
        let fs = self.fs.clone();

        cx.spawn(|this, mut cx| async move {
            if fs.is_dir(&path).await {
                this.update(&mut cx, |this, cx| {
                    this.uninstall_extension(&extension.id.clone().into(), cx)
                })?
                .await?;
            }

            fs.rename(&tmp_path, &path, RenameOptions::default())
                .await?;

            Self::load_extension(this, extension, &mut cx).await
        })
    }

    pub async fn handle_sync_extensions(
        extension_store: Model<HeadlessExtensionStore>,
        envelope: TypedEnvelope<proto::SyncExtensions>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::SyncExtensionsResponse> {
        let requested_extensions =
            envelope
                .payload
                .extensions
                .into_iter()
                .map(|p| ExtensionVersion {
                    id: p.id,
                    version: p.version,
                    dev: p.dev,
                });
        let missing_extensions = extension_store
            .update(&mut cx, |extension_store, cx| {
                extension_store.sync_extensions(requested_extensions.collect(), cx)
            })?
            .await?;

        Ok(proto::SyncExtensionsResponse {
            missing_extensions: missing_extensions
                .into_iter()
                .map(|e| proto::Extension {
                    id: e.id,
                    version: e.version,
                    dev: e.dev,
                })
                .collect(),
            tmp_dir: paths::remote_extensions_uploads_dir()
                .to_string_lossy()
                .to_string(),
        })
    }

    pub async fn handle_install_extension(
        extensions: Model<HeadlessExtensionStore>,
        envelope: TypedEnvelope<proto::InstallExtension>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let extension = envelope
            .payload
            .extension
            .with_context(|| anyhow!("Invalid InstallExtension request"))?;

        extensions
            .update(&mut cx, |extensions, cx| {
                extensions.install_extension(
                    ExtensionVersion {
                        id: extension.id,
                        version: extension.version,
                        dev: extension.dev,
                    },
                    PathBuf::from(envelope.payload.tmp_dir),
                    cx,
                )
            })?
            .await?;

        Ok(proto::Ack {})
    }
}

struct HeadlessRegistrationHooks {
    language_registry: Arc<LanguageRegistry>,
}

impl HeadlessRegistrationHooks {
    fn new(language_registry: Arc<LanguageRegistry>) -> Self {
        Self { language_registry }
    }
}

impl ExtensionRegistrationHooks for HeadlessRegistrationHooks {
    fn register_language(
        &self,
        language: LanguageName,
        _grammar: Option<Arc<str>>,
        matcher: language::LanguageMatcher,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + 'static + Send + Sync>,
    ) {
        log::info!("registering language: {:?}", language);
        self.language_registry
            .register_language(language, None, matcher, load)
    }
    fn register_lsp_adapter(&self, language: LanguageName, adapter: ExtensionLspAdapter) {
        log::info!("registering lsp adapter {:?}", language);
        self.language_registry
            .register_lsp_adapter(language, Arc::new(adapter) as _);
    }

    fn register_wasm_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        self.language_registry.register_wasm_grammars(grammars)
    }

    fn remove_lsp_adapter(&self, language: &LanguageName, server_name: &LanguageServerName) {
        self.language_registry
            .remove_lsp_adapter(language, server_name)
    }

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        _grammars_to_remove: &[Arc<str>],
    ) {
        self.language_registry
            .remove_languages(languages_to_remove, &[])
    }

    fn update_lsp_status(
        &self,
        server_name: LanguageServerName,
        status: language::LanguageServerBinaryStatus,
    ) {
        self.language_registry
            .update_lsp_status(server_name, status)
    }
}
