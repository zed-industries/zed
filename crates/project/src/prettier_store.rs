use std::{
    ops::{ControlFlow, Range},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{
    FutureExt,
    future::{self, Shared},
    stream::FuturesUnordered,
};
use gpui::{AppContext as _, AsyncApp, Context, Entity, EventEmitter, Task, WeakEntity};
use language::{
    Buffer, LanguageRegistry, LocalFile, OffsetUtf16,
    language_settings::{Formatter, LanguageSettings},
};
use lsp::{LanguageServer, LanguageServerId, LanguageServerName};
use node_runtime::NodeRuntime;
use paths::default_prettier_dir;
use prettier::Prettier;
use settings::Settings;
use smol::stream::StreamExt;
use util::{ResultExt, ToolPermissionDenied, TryFutureExt, rel_path::RelPath};

use crate::{
    File, PathChange, ProjectEntryId, Worktree, lsp_store::WorktreeId,
    project_settings::ProjectSettings, worktree_store::WorktreeStore,
};

pub struct PrettierStore {
    node: NodeRuntime,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    worktree_store: Entity<WorktreeStore>,
    default_prettier: DefaultPrettier,
    installation_retries: HashMap<Option<WorktreeId>, (HashSet<Arc<str>>, Task<()>)>,
    prettiers_per_worktree: HashMap<WorktreeId, HashSet<Option<PathBuf>>>,
    prettier_ignores_per_worktree: HashMap<WorktreeId, HashSet<PathBuf>>,
    prettier_instances: HashMap<PathBuf, PrettierInstance>,
}

pub(crate) enum PrettierStoreEvent {
    LanguageServerRemoved(LanguageServerId),
    LanguageServerAdded {
        new_server_id: LanguageServerId,
        name: LanguageServerName,
        prettier_server: Arc<LanguageServer>,
    },
}

impl EventEmitter<PrettierStoreEvent> for PrettierStore {}

impl PrettierStore {
    pub fn new(
        node: NodeRuntime,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        worktree_store: Entity<WorktreeStore>,
        _: &mut Context<Self>,
    ) -> Self {
        Self {
            node,
            fs,
            languages,
            worktree_store,
            default_prettier: DefaultPrettier::default(),
            installation_retries: HashMap::default(),
            prettiers_per_worktree: HashMap::default(),
            prettier_ignores_per_worktree: HashMap::default(),
            prettier_instances: HashMap::default(),
        }
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        self.prettier_ignores_per_worktree.remove(&id_to_remove);
        self.installation_retries.remove(&Some(id_to_remove));
        let mut prettier_instances_to_clean = FuturesUnordered::new();
        if let Some(prettier_paths) = self.prettiers_per_worktree.remove(&id_to_remove) {
            for path in prettier_paths.iter().flatten() {
                if let Some(prettier_instance) = self.prettier_instances.remove(path) {
                    prettier_instances_to_clean.push(async move {
                        prettier_instance
                            .server()
                            .await
                            .map(|server| server.server_id())
                    });
                }
            }
        }
        cx.spawn(async move |prettier_store, cx| {
            while let Some(prettier_server_id) = prettier_instances_to_clean.next().await {
                if let Some(prettier_server_id) = prettier_server_id {
                    prettier_store
                        .update(cx, |_, cx| {
                            cx.emit(PrettierStoreEvent::LanguageServerRemoved(
                                prettier_server_id,
                            ));
                        })
                        .ok();
                }
            }
        })
        .detach();
    }

    fn prettier_instance_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<(Option<PathBuf>, PrettierTask)>> {
        let buffer = buffer.read(cx);
        let buffer_file = buffer.file();
        if buffer.language().is_none() {
            return Task::ready(None);
        }

        let node = self.node.clone();
        let plugins = LanguageSettings::for_buffer(buffer, cx)
            .prettier
            .plugins
            .iter()
            .map(|plugin| Arc::from(plugin.as_str()))
            .collect::<Vec<_>>();

        match File::from_dyn(buffer_file).map(|file| (file.worktree_id(cx), file.abs_path(cx))) {
            Some((worktree_id, buffer_path)) => {
                let fs = Arc::clone(&self.fs);
                let installed_prettiers = self.prettier_instances.keys().cloned().collect();
                cx.spawn(async move |lsp_store, cx| {
                    match cx
                        .background_spawn(async move {
                            Prettier::locate_prettier_installation(
                                fs.as_ref(),
                                &installed_prettiers,
                                &buffer_path,
                            )
                            .await
                        })
                        .await
                    {
                        Ok(ControlFlow::Break(())) => None,
                        Ok(ControlFlow::Continue(None)) => {
                            let default_task = lsp_store
                                .update(cx, |lsp_store, cx| {
                                    lsp_store.install_default_prettier(
                                        Some(worktree_id),
                                        plugins.into_iter(),
                                        cx,
                                    );
                                    lsp_store
                                        .prettiers_per_worktree
                                        .entry(worktree_id)
                                        .or_default()
                                        .insert(None);
                                    lsp_store.default_prettier.prettier_task(
                                        &node,
                                        Some(worktree_id),
                                        cx,
                                    )
                                })
                                .ok()??;
                            let default_instance = match default_task.await {
                                Ok(instance) => instance,
                                Err(error) => Task::ready(Err(Arc::new(error))).shared(),
                            };
                            Some((None, default_instance))
                        }
                        Ok(ControlFlow::Continue(Some(prettier_dir))) => {
                            lsp_store
                                .update(cx, |lsp_store, _| {
                                    lsp_store
                                        .prettiers_per_worktree
                                        .entry(worktree_id)
                                        .or_default()
                                        .insert(Some(prettier_dir.clone()))
                                })
                                .ok()?;
                            if let Some(prettier_task) = lsp_store
                                .update(cx, |lsp_store, cx| {
                                    lsp_store
                                        .prettier_instances
                                        .get_mut(&prettier_dir)
                                        .and_then(|existing_instance| {
                                            existing_instance.prettier_task(
                                                &node,
                                                Some(&prettier_dir),
                                                Some(worktree_id),
                                                cx,
                                            )
                                        })
                                })
                                .ok()?
                            {
                                log::debug!("Found already started prettier in {prettier_dir:?}");
                                return Some((Some(prettier_dir), prettier_task.await.log_err()?));
                            }

                            log::info!("Found prettier in {prettier_dir:?}, starting.");
                            let new_prettier_task = lsp_store
                                .update(cx, |lsp_store, cx| {
                                    let new_prettier_task = Self::start_prettier(
                                        node,
                                        prettier_dir.clone(),
                                        Some(worktree_id),
                                        cx,
                                    );
                                    lsp_store.prettier_instances.insert(
                                        prettier_dir.clone(),
                                        PrettierInstance {
                                            attempt: 0,
                                            prettier: Some(new_prettier_task.clone()),
                                        },
                                    );
                                    new_prettier_task
                                })
                                .ok()?;
                            Some((Some(prettier_dir), new_prettier_task))
                        }
                        Err(e) => {
                            log::error!("Failed to determine prettier path for buffer: {e:#}");
                            None
                        }
                    }
                })
            }
            None => {
                self.install_default_prettier(None, plugins.into_iter(), cx);
                let new_task = self.default_prettier.prettier_task(&node, None, cx);
                cx.spawn(async move |_, _| {
                    let instance = match new_task?.await {
                        Ok(instance) => instance,
                        Err(error) => Task::ready(Err(Arc::new(error))).shared(),
                    };
                    Some((None, instance))
                })
            }
        }
    }

    fn prettier_ignore_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<PathBuf>> {
        let buffer = buffer.read(cx);
        let buffer_file = buffer.file();
        if buffer.language().is_none() {
            return Task::ready(None);
        }
        match File::from_dyn(buffer_file).map(|file| (file.worktree_id(cx), file.abs_path(cx))) {
            Some((worktree_id, buffer_path)) => {
                let fs = Arc::clone(&self.fs);
                let prettier_ignores = self
                    .prettier_ignores_per_worktree
                    .get(&worktree_id)
                    .cloned()
                    .unwrap_or_default();
                cx.spawn(async move |lsp_store, cx| {
                    match cx
                        .background_spawn(async move {
                            Prettier::locate_prettier_ignore(
                                fs.as_ref(),
                                &prettier_ignores,
                                &buffer_path,
                            )
                            .await
                        })
                        .await
                    {
                        Ok(ControlFlow::Break(())) => None,
                        Ok(ControlFlow::Continue(None)) => None,
                        Ok(ControlFlow::Continue(Some(ignore_dir))) => {
                            log::debug!("Found prettier ignore in {ignore_dir:?}");
                            lsp_store
                                .update(cx, |store, _| {
                                    store
                                        .prettier_ignores_per_worktree
                                        .entry(worktree_id)
                                        .or_default()
                                        .insert(ignore_dir.clone());
                                })
                                .ok();
                            Some(ignore_dir)
                        }
                        Err(e) => {
                            log::error!(
                                "Failed to determine prettier ignore path for buffer: {e:#}"
                            );
                            None
                        }
                    }
                })
            }
            None => Task::ready(None),
        }
    }

    fn start_prettier(
        node: NodeRuntime,
        prettier_dir: PathBuf,
        worktree_id: Option<WorktreeId>,
        cx: &mut Context<Self>,
    ) -> PrettierTask {
        let request_timeout = ProjectSettings::get_global(cx)
            .global_lsp_settings
            .get_request_timeout();

        let node = crate::binary_downloads::scoped_node_runtime(&node, worktree_id, "prettier", cx);
        let prettier_store = cx.weak_entity();
        let permission = move |cx: &mut gpui::App| {
            prettier_store.update(cx, |prettier_store, cx| {
                prettier_store
                    .check_installation_worktree(worktree_id, cx)
                    .map_err(|error| anyhow!(error))?;
                if crate::binary_downloads::request_tool_install(worktree_id, "prettier", cx)
                    .is_some()
                {
                    return Err(anyhow::Error::new(ToolPermissionDenied(
                        "prettier".to_string(),
                    )));
                }
                Ok(())
            })?
        };
        cx.spawn(async move |prettier_store, cx| {
            log::info!("Starting prettier at path {prettier_dir:?}");
            let (new_server_id, fs) = prettier_store.read_with(cx, |prettier_store, _| {
                (
                    prettier_store.languages.next_language_server_id(),
                    prettier_store.fs.clone(),
                )
            })?;
            ensure_prettier_server_file(fs.as_ref())
                .await
                .map_err(Arc::new)?;

            let new_prettier = Prettier::start(
                new_server_id,
                prettier_dir,
                node,
                request_timeout,
                permission,
                cx.clone(),
            )
            .await
            .context("default prettier spawn")
            .map(Arc::new)
            .map_err(Arc::new)?;
            Self::register_new_prettier(
                &prettier_store,
                &new_prettier,
                worktree_id,
                new_server_id,
                cx,
            );
            Ok(new_prettier)
        })
        .shared()
    }

    fn start_default_prettier(
        node: NodeRuntime,
        worktree_id: Option<WorktreeId>,
        cx: &mut Context<PrettierStore>,
    ) -> Task<anyhow::Result<PrettierTask>> {
        cx.spawn(async move |prettier_store, cx| {
            let installation_task = prettier_store.read_with(cx, |prettier_store, _| {
                match &prettier_store.default_prettier.prettier {
                    PrettierInstallation::NotInstalled {
                        installation_task, ..
                    } => ControlFlow::Continue(installation_task.clone()),
                    PrettierInstallation::Installed(default_prettier) => {
                        ControlFlow::Break(default_prettier.clone())
                    }
                }
            })?;
            match installation_task {
                ControlFlow::Continue(None) => {
                    anyhow::bail!("Default prettier is not installed and cannot be started")
                }
                ControlFlow::Continue(Some(installation_task)) => {
                    log::info!("Waiting for default prettier to install");
                    if let Err(e) = installation_task.await {
                        if let Some(denied) = e.downcast_ref::<ToolPermissionDenied>() {
                            return Err(anyhow::Error::new(denied.clone()).context(
                                "Cannot start default prettier due to its installation failure",
                            ));
                        }
                        if !e.is::<PrettierInstallationCancelled>() {
                            prettier_store.update(cx, |project, _| {
                                if let PrettierInstallation::NotInstalled {
                                    installation_task,
                                    attempts,
                                    ..
                                } = &mut project.default_prettier.prettier
                                {
                                    *installation_task = None;
                                    *attempts += 1;
                                }
                            })?;
                        }
                        anyhow::bail!(
                            "Cannot start default prettier due to its installation failure: {e:#}"
                        );
                    }
                    let new_default_prettier =
                        prettier_store.update(cx, |prettier_store, cx| {
                            let new_default_prettier = Self::start_prettier(
                                node,
                                default_prettier_dir().clone(),
                                worktree_id,
                                cx,
                            );
                            prettier_store.default_prettier.prettier =
                                PrettierInstallation::Installed(PrettierInstance {
                                    attempt: 0,
                                    prettier: Some(new_default_prettier.clone()),
                                });
                            new_default_prettier
                        })?;
                    Ok(new_default_prettier)
                }
                ControlFlow::Break(instance) => match instance.prettier {
                    Some(instance) => Ok(instance),
                    None => {
                        let new_default_prettier =
                            prettier_store.update(cx, |prettier_store, cx| {
                                let new_default_prettier = Self::start_prettier(
                                    node,
                                    default_prettier_dir().clone(),
                                    worktree_id,
                                    cx,
                                );
                                prettier_store.default_prettier.prettier =
                                    PrettierInstallation::Installed(PrettierInstance {
                                        attempt: instance.attempt + 1,
                                        prettier: Some(new_default_prettier.clone()),
                                    });
                                new_default_prettier
                            })?;
                        Ok(new_default_prettier)
                    }
                },
            }
        })
    }

    fn register_new_prettier(
        prettier_store: &WeakEntity<Self>,
        prettier: &Prettier,
        worktree_id: Option<WorktreeId>,
        new_server_id: LanguageServerId,
        cx: &mut AsyncApp,
    ) {
        let prettier_dir = prettier.prettier_dir();
        let is_default = prettier.is_default();
        if is_default {
            log::info!("Started default prettier in {prettier_dir:?}");
        } else {
            log::info!("Started prettier in {prettier_dir:?}");
        }
        if let Some(prettier_server) = prettier.server() {
            prettier_store
                .update(cx, |prettier_store, cx| {
                    let name = if is_default {
                        LanguageServerName("prettier (default)".into())
                    } else {
                        let worktree_path = worktree_id
                            .and_then(|id| {
                                prettier_store
                                    .worktree_store
                                    .read(cx)
                                    .worktree_for_id(id, cx)
                            })
                            .map(|worktree| worktree.read(cx).abs_path());
                        let name = match worktree_path {
                            Some(worktree_path) => {
                                if prettier_dir == worktree_path.as_ref() {
                                    let name = prettier_dir
                                        .file_name()
                                        .and_then(|name| name.to_str())
                                        .unwrap_or_default();
                                    format!("prettier ({name})")
                                } else {
                                    let dir_to_display = prettier_dir
                                        .strip_prefix(worktree_path.as_ref())
                                        .ok()
                                        .unwrap_or(prettier_dir);
                                    format!("prettier ({})", dir_to_display.display())
                                }
                            }
                            None => format!("prettier ({})", prettier_dir.display()),
                        };
                        LanguageServerName(name.into())
                    };
                    cx.emit(PrettierStoreEvent::LanguageServerAdded {
                        new_server_id,
                        name,
                        prettier_server: prettier_server.clone(),
                    });
                })
                .ok();
        }
    }

    pub fn update_prettier_settings(
        &self,
        worktree: &Entity<Worktree>,
        changes: &[(Arc<RelPath>, ProjectEntryId, PathChange)],
        cx: &mut Context<Self>,
    ) {
        let prettier_config_files = Prettier::CONFIG_FILE_NAMES
            .iter()
            .map(|name| RelPath::from_unix_str(name).unwrap())
            .collect::<HashSet<_>>();

        let prettier_config_file_changed = changes
            .iter()
            .filter(|(path, _, change)| {
                !matches!(change, PathChange::Loaded)
                    && !path
                        .components()
                        .any(|component| component == "node_modules")
            })
            .find(|(path, _, _)| prettier_config_files.contains(path.as_ref()));

        let Some((config_path, _, _)) = prettier_config_file_changed else {
            return;
        };

        let current_worktree_id = worktree.read(cx).id();

        log::info!(
            "Prettier config file {config_path:?} changed, reloading prettier instances for worktree {current_worktree_id}"
        );

        let prettiers_to_reload = self
            .prettiers_per_worktree
            .get(&current_worktree_id)
            .iter()
            .flat_map(|prettier_paths| prettier_paths.iter())
            .flatten()
            .filter_map(|prettier_path| {
                Some((
                    current_worktree_id,
                    Some(prettier_path.clone()),
                    self.prettier_instances.get(prettier_path)?.clone(),
                ))
            })
            .chain(
                self.default_prettier
                    .instance()
                    .map(|default_prettier| (current_worktree_id, None, default_prettier.clone())),
            )
            .collect::<Vec<_>>();

        let request_timeout = ProjectSettings::get_global(cx)
            .global_lsp_settings
            .get_request_timeout();

        cx.background_spawn(async move {
            let _: Vec<()> = future::join_all(prettiers_to_reload.into_iter().map(|(worktree_id, prettier_path, prettier_instance)| {
                async move {
                    let Some(instance) = prettier_instance.prettier else {
                        return
                    };

                    match instance.await {
                        Ok(prettier) => {
                            prettier.clear_cache(request_timeout).log_err().await;
                        },
                        Err(e) => {
                            match prettier_path {
                                Some(prettier_path) => log::error!(
                                    "Failed to clear prettier {prettier_path:?} cache for worktree {worktree_id:?} on prettier settings update: {e:#}"
                                ),
                                None => log::error!(
                                    "Failed to clear default prettier cache for worktree {worktree_id:?} on prettier settings update: {e:#}"
                                ),
                            }
                        },
                    }
                }
            }))
            .await;
        })
            .detach();
    }

    pub fn install_default_prettier(
        &mut self,
        worktree: Option<WorktreeId>,
        plugins: impl Iterator<Item = Arc<str>>,
        cx: &mut Context<Self>,
    ) {
        if self.worktree_restricted(worktree, cx) {
            return;
        }
        if cfg!(any(test, feature = "test-support")) {
            self.default_prettier.installed_plugins.extend(plugins);
            self.default_prettier.prettier = PrettierInstallation::Installed(PrettierInstance {
                attempt: 0,
                prettier: None,
            });
            return;
        }

        let node =
            crate::binary_downloads::scoped_node_runtime(&self.node, worktree, "prettier", cx);
        self.install_default_prettier_with(
            worktree,
            plugins,
            move |packages, cx| {
                cx.background_spawn(install_prettier_packages(packages, node.clone()))
            },
            cx,
        );
    }

    fn install_default_prettier_with(
        &mut self,
        worktree: Option<WorktreeId>,
        plugins: impl Iterator<Item = Arc<str>>,
        install: impl Fn(HashSet<Arc<str>>, &mut AsyncApp) -> Task<Result<()>> + Clone + 'static,
        cx: &mut Context<Self>,
    ) {
        if self.worktree_restricted(worktree, cx) {
            return;
        }
        let mut new_plugins = plugins.collect::<HashSet<_>>();
        if let Some((pending_plugins, _)) = self.installation_retries.get(&worktree) {
            new_plugins.extend(pending_plugins.iter().cloned());
        }

        new_plugins.retain(|plugin| !self.default_prettier.installed_plugins.contains(plugin));
        let mut installation_attempt = 0;
        let previous_installation_task = match &mut self.default_prettier.prettier {
            PrettierInstallation::NotInstalled {
                installation_task,
                attempts,
                not_installed_plugins,
            } => {
                installation_attempt = *attempts;
                if installation_attempt > prettier::FAIL_THRESHOLD {
                    *installation_task = None;
                    log::warn!(
                        "Default prettier installation had failed {installation_attempt} times, not attempting again",
                    );
                    return;
                }
                if self.default_prettier.installation_worktree == worktree {
                    new_plugins.extend(not_installed_plugins.iter().cloned());
                }
                installation_task.clone()
            }
            PrettierInstallation::Installed { .. } => {
                if new_plugins.is_empty() {
                    return;
                }
                None
            }
        };

        let plugins_to_install = new_plugins.clone();
        let fs = Arc::clone(&self.fs);
        let new_installation_task = cx
            .spawn(async move  |prettier_store, cx| {
                cx.background_executor().timer(Duration::from_millis(30)).await;
                let location_data = prettier_store.update(cx, |prettier_store, cx| {
                    prettier_store.check_installation_worktree(worktree, cx)?;
                    Ok::<_, Arc<anyhow::Error>>(worktree.and_then(|worktree_id| {
                        prettier_store.worktree_store
                            .read(cx)
                            .worktree_for_id(worktree_id, cx)
                            .map(|worktree| worktree.read(cx).abs_path())
                    }).map(|locate_from| {
                        let installed_prettiers = prettier_store.prettier_instances.keys().cloned().collect();
                        (locate_from, installed_prettiers)
                    }))
                })??;
                let locate_prettier_installation = match location_data {
                    Some((locate_from, installed_prettiers)) => Prettier::locate_prettier_installation(
                        fs.as_ref(),
                        &installed_prettiers,
                        locate_from.as_ref(),
                    )
                    .await
                    .context("locate prettier installation").map_err(Arc::new)?,
                    None => ControlFlow::Continue(None),
                };

                match locate_prettier_installation
                {
                    ControlFlow::Break(()) => return Ok(()),
                    ControlFlow::Continue(prettier_path) => {
                        if prettier_path.is_some() {
                            new_plugins.clear();
                        }
                        let previous_installation_result = match previous_installation_task {
                            Some(task) => task.await,
                            None => Ok(()),
                        };
                        prettier_store.update(cx, |prettier_store, cx| {
                            prettier_store.check_installation_worktree(worktree, cx)
                        })??;
                        if let Err(e) = previous_installation_result
                            && !e.is::<ToolPermissionDenied>()
                            && !e.is::<PrettierInstallationCancelled>() {
                                log::error!("Failed to install default prettier: {e:#}");
                                prettier_store.update(cx, |prettier_store, _| {
                                    if let PrettierInstallation::NotInstalled { attempts, .. } = &mut prettier_store.default_prettier.prettier {
                                        *attempts += 1;
                                        installation_attempt = *attempts;
                                    };
                                })?;
                            };
                        if installation_attempt > prettier::FAIL_THRESHOLD {
                            prettier_store.update(cx, |prettier_store, _| {
                                if let PrettierInstallation::NotInstalled { installation_task, .. } = &mut prettier_store.default_prettier.prettier {
                                    *installation_task = None;
                                };
                            })?;
                            log::warn!(
                                "Default prettier installation had failed {installation_attempt} times, not attempting again",
                            );
                            return Ok(());
                        }
                        ensure_prettier_server_file(fs.as_ref()).await.map_err(Arc::new)?;
                        let node_modules = default_prettier_dir().join("node_modules");
                        let default_prettier_installed = prettier_package_is_resolvable(fs.as_ref(), &node_modules.join("prettier")).await;
                        let missing_prettier = prettier_path.is_none() && !default_prettier_installed;
                        let mut needs_install = missing_prettier;
                        let mut cached_plugins = HashSet::default();
                        for plugin in &new_plugins {
                            for entrypoint in prettier::plugin_entry_points(node_modules.join(plugin.as_ref())) {
                                if fs.is_file(&entrypoint).await {
                                    cached_plugins.insert(plugin.clone());
                                    break;
                                }
                            }
                        }
                        prettier_store.update(cx, |prettier_store, cx| {
                            prettier_store.check_installation_worktree(worktree, cx)?;
                            prettier_store.default_prettier.installed_plugins.extend(cached_plugins);
                            new_plugins.retain(|plugin| {
                                !prettier_store.default_prettier.installed_plugins.contains(plugin)
                            });
                            if prettier_store.default_prettier.installation_worktree == worktree
                                && let PrettierInstallation::NotInstalled { not_installed_plugins, .. } = &mut prettier_store.default_prettier.prettier {
                                not_installed_plugins.retain(|plugin| {
                                    !prettier_store.default_prettier.installed_plugins.contains(plugin)
                                });
                                not_installed_plugins.extend(new_plugins.iter().cloned());
                            }
                            needs_install |= !new_plugins.is_empty();
                            Ok::<_, Arc<anyhow::Error>>(())
                        })??;
                        if needs_install {
                            prepare_prettier_directory(fs.as_ref()).await.map_err(Arc::new)?;
                            let wait = prettier_store.update(cx, |prettier_store, cx| {
                                prettier_store.check_installation_worktree(worktree, cx)?;
                                Ok::<_, Arc<anyhow::Error>>(crate::binary_downloads::request_tool_install(worktree, "prettier", cx))
                            })??;
                            if let Some(wait) = wait {
                                prettier_store.update(cx, |prettier_store, cx| {
                                    prettier_store.retry_installation(worktree, new_plugins, install, Some(wait), cx);
                                })?;
                                return Err(Arc::new(anyhow::Error::new(ToolPermissionDenied("prettier".to_string()))));
                            }
                            log::info!("Initializing default prettier with plugins {new_plugins:?}");
                            let installed_plugins = new_plugins.clone();
                            if missing_prettier {
                                new_plugins.insert(Arc::from("prettier"));
                            }
                            if let Err(error) = install(new_plugins, cx).await {
                                if error.is::<ToolPermissionDenied>() {
                                    prettier_store.update(cx, |prettier_store, cx| {
                                        prettier_store.check_installation_worktree(worktree, cx)?;
                                        let wait = crate::binary_downloads::request_tool_install(worktree, "prettier", cx);
                                        prettier_store.retry_installation(worktree, installed_plugins, install, wait, cx);
                                        Ok::<_, Arc<anyhow::Error>>(())
                                    })??;
                                }
                                return Err(Arc::new(error.context("prettier & plugins install")));
                            }
                            log::info!("Initialized default prettier with plugins: {installed_plugins:?}");
                            prettier_store.update(cx, |prettier_store, _| {
                                prettier_store.installation_retries.remove(&worktree);
                                prettier_store.default_prettier.prettier =
                                    PrettierInstallation::Installed(PrettierInstance {
                                        attempt: 0,
                                        prettier: None,
                                    });
                                prettier_store.default_prettier
                                    .installed_plugins
                                    .extend(installed_plugins);
                            })?;
                        } else {
                            prettier_store.update(cx, |prettier_store, _| {
                                prettier_store.installation_retries.remove(&worktree);
                                if let PrettierInstallation::NotInstalled { installation_task, .. } = &mut prettier_store.default_prettier.prettier {
                                    if default_prettier_installed {
                                        prettier_store.default_prettier.prettier =
                                            PrettierInstallation::Installed(PrettierInstance {
                                                attempt: 0,
                                                prettier: None,
                                            });
                                    } else {
                                        *installation_task = None;
                                    }
                                }
                            })?;
                        }
                    }
                }
                Ok(())
            })
            .shared();
        self.default_prettier.installation_worktree = worktree;
        self.default_prettier.prettier = PrettierInstallation::NotInstalled {
            attempts: installation_attempt,
            installation_task: Some(new_installation_task),
            not_installed_plugins: plugins_to_install,
        };
    }

    fn retry_installation(
        &mut self,
        worktree: Option<WorktreeId>,
        plugins: HashSet<Arc<str>>,
        install: impl Fn(HashSet<Arc<str>>, &mut AsyncApp) -> Task<Result<()>> + Clone + 'static,
        wait: Option<postage::watch::Receiver<bool>>,
        cx: &mut Context<Self>,
    ) {
        let pending_plugins = plugins.clone();
        let retry = cx.spawn(async move |prettier_store, cx| {
            if crate::binary_downloads::await_downloads_allowed(wait, "prettier").await {
                prettier_store
                    .update(cx, |prettier_store, cx| {
                        prettier_store.install_default_prettier_with(
                            worktree,
                            plugins.into_iter(),
                            install,
                            cx,
                        );
                    })
                    .ok();
            }
        });
        self.installation_retries
            .insert(worktree, (pending_plugins, retry));
    }

    fn check_installation_worktree(
        &self,
        worktree: Option<WorktreeId>,
        cx: &mut Context<Self>,
    ) -> Result<(), Arc<anyhow::Error>> {
        if self.worktree_restricted(worktree, cx) {
            return Err(Arc::new(anyhow::Error::new(PrettierInstallationCancelled)));
        }
        Ok(())
    }

    fn worktree_restricted_for_buffer(
        &self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> bool {
        let worktree = File::from_dyn(buffer.read(cx).file()).map(|file| file.worktree_id(cx));
        self.worktree_restricted(worktree, cx)
    }

    fn worktree_restricted(&self, worktree: Option<WorktreeId>, cx: &mut Context<Self>) -> bool {
        worktree.is_some_and(|worktree_id| {
            self.worktree_store
                .read(cx)
                .worktree_for_id(worktree_id, cx)
                .is_none()
                || !crate::trusted_worktrees::worktree_trusted(
                    &self.worktree_store,
                    worktree_id,
                    cx,
                )
        })
    }
}

pub fn prettier_plugins_for_language(
    language_settings: &LanguageSettings,
) -> Option<&HashSet<String>> {
    let formatters = language_settings.formatter.as_ref();
    if formatters.contains(&Formatter::Prettier) || formatters.contains(&Formatter::Auto) {
        return Some(&language_settings.prettier.plugins);
    }
    None
}

pub(super) async fn format_with_prettier(
    prettier_store: &WeakEntity<PrettierStore>,
    buffer: &Entity<Buffer>,
    range_utf16: Option<Range<OffsetUtf16>>,
    cx: &mut AsyncApp,
) -> Option<Result<language::Diff>> {
    let restricted_worktree = prettier_store
        .update(cx, |prettier_store, cx| {
            prettier_store.worktree_restricted_for_buffer(buffer, cx)
        })
        .ok()?;
    if restricted_worktree {
        return Some(Err(anyhow!(
            "cannot format with prettier: the buffer belongs to an untrusted worktree in restricted mode"
        )));
    }

    let worktree = buffer.read_with(cx, |buffer, cx| {
        File::from_dyn(buffer.file()).map(|file| file.worktree_id(cx))
    });
    loop {
        if let Err(error) = wait_for_prettier_permission(prettier_store, worktree, cx).await {
            return Some(Err(error));
        }
        let prettier_instance = prettier_store
            .update(cx, |prettier_store, cx| {
                prettier_store.prettier_instance_for_buffer(buffer, cx)
            })
            .ok()?
            .await;

        let ignore_dir = prettier_store
            .update(cx, |prettier_store, cx| {
                prettier_store.prettier_ignore_for_buffer(buffer, cx)
            })
            .ok()?
            .await;

        let (prettier_path, prettier_task) = prettier_instance?;

        let prettier_description = match prettier_path.as_ref() {
            Some(path) => format!("prettier at {path:?}"),
            None => "default prettier instance".to_string(),
        };

        let request_timeout: Duration = cx.update(|app| {
            ProjectSettings::get_global(app)
                .global_lsp_settings
                .get_request_timeout()
        });

        match prettier_task.await {
            Ok(prettier) => {
                if let Err(error) = wait_for_prettier_permission(prettier_store, worktree, cx).await
                {
                    return Some(Err(error));
                }
                let buffer_path = buffer.update(cx, |buffer, cx| {
                    File::from_dyn(buffer.file()).map(|file| file.abs_path(cx))
                });

                let format_result = prettier
                    .format(
                        buffer,
                        buffer_path,
                        ignore_dir,
                        range_utf16,
                        request_timeout,
                        cx,
                    )
                    .await
                    .with_context(|| format!("{} failed to format buffer", prettier_description));

                return Some(format_result);
            }
            Err(error) => {
                let blocked = error.is::<ToolPermissionDenied>();
                prettier_store
                    .update(cx, |project, _| {
                        let instance_to_update = match prettier_path {
                            Some(prettier_path) => {
                                project.prettier_instances.get_mut(&prettier_path)
                            }
                            None => match &mut project.default_prettier.prettier {
                                PrettierInstallation::NotInstalled { .. } => None,
                                PrettierInstallation::Installed(instance) => Some(instance),
                            },
                        };

                        if let Some(instance) = instance_to_update {
                            if blocked {
                                instance.attempt = instance.attempt.saturating_sub(1);
                            } else {
                                instance.attempt += 1;
                            }
                            instance.prettier = None;
                        }
                    })
                    .ok();

                if blocked {
                    continue;
                }
                return Some(Err(anyhow!(
                    "{prettier_description} failed to spawn: {error:#}"
                )));
            }
        }
    }
}

#[derive(Debug)]
pub struct DefaultPrettier {
    prettier: PrettierInstallation,
    installed_plugins: HashSet<Arc<str>>,
    installation_worktree: Option<WorktreeId>,
}

#[derive(Debug)]
pub enum PrettierInstallation {
    NotInstalled {
        attempts: usize,
        installation_task: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
        not_installed_plugins: HashSet<Arc<str>>,
    },
    Installed(PrettierInstance),
}

pub type PrettierTask = Shared<Task<Result<Arc<Prettier>, Arc<anyhow::Error>>>>;

#[derive(Debug, Clone)]
pub struct PrettierInstance {
    attempt: usize,
    prettier: Option<PrettierTask>,
}

impl Default for DefaultPrettier {
    fn default() -> Self {
        Self {
            prettier: PrettierInstallation::NotInstalled {
                attempts: 0,
                installation_task: None,
                not_installed_plugins: HashSet::default(),
            },
            installed_plugins: HashSet::default(),
            installation_worktree: None,
        }
    }
}

impl DefaultPrettier {
    pub fn instance(&self) -> Option<&PrettierInstance> {
        if let PrettierInstallation::Installed(instance) = &self.prettier {
            Some(instance)
        } else {
            None
        }
    }

    pub fn prettier_task(
        &mut self,
        node: &NodeRuntime,
        worktree_id: Option<WorktreeId>,
        cx: &mut Context<PrettierStore>,
    ) -> Option<Task<anyhow::Result<PrettierTask>>> {
        match &mut self.prettier {
            PrettierInstallation::NotInstalled { .. } => Some(
                PrettierStore::start_default_prettier(node.clone(), worktree_id, cx),
            ),
            PrettierInstallation::Installed(existing_instance) => {
                existing_instance.prettier_task(node, None, worktree_id, cx)
            }
        }
    }
}

impl PrettierInstance {
    pub fn prettier_task(
        &mut self,
        node: &NodeRuntime,
        prettier_dir: Option<&Path>,
        worktree_id: Option<WorktreeId>,
        cx: &mut Context<PrettierStore>,
    ) -> Option<Task<anyhow::Result<PrettierTask>>> {
        if self.attempt > prettier::FAIL_THRESHOLD {
            match prettier_dir {
                Some(prettier_dir) => log::warn!(
                    "Prettier from path {prettier_dir:?} exceeded launch threshold, not starting"
                ),
                None => log::warn!("Default prettier exceeded launch threshold, not starting"),
            }
            return None;
        }
        Some(match &self.prettier {
            Some(prettier_task) => Task::ready(Ok(prettier_task.clone())),
            None => match prettier_dir {
                Some(prettier_dir) => {
                    let new_task = PrettierStore::start_prettier(
                        node.clone(),
                        prettier_dir.to_path_buf(),
                        worktree_id,
                        cx,
                    );
                    self.attempt += 1;
                    self.prettier = Some(new_task.clone());
                    Task::ready(Ok(new_task))
                }
                None => {
                    let node = node.clone();
                    cx.spawn(async move |prettier_store, cx| {
                        prettier_store
                            .update(cx, |_, cx| {
                                PrettierStore::start_default_prettier(node, worktree_id, cx)
                            })?
                            .await
                    })
                }
            },
        })
    }

    pub async fn server(&self) -> Option<Arc<LanguageServer>> {
        self.prettier.clone()?.await.ok()?.server().cloned()
    }
}

#[derive(Debug)]
struct PrettierInstallationCancelled;

impl std::fmt::Display for PrettierInstallationCancelled {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Prettier installation cancelled: worktree is missing or restricted")
    }
}

impl std::error::Error for PrettierInstallationCancelled {}

async fn wait_for_prettier_permission(
    prettier_store: &WeakEntity<PrettierStore>,
    worktree: Option<WorktreeId>,
    cx: &mut AsyncApp,
) -> Result<()> {
    loop {
        let wait = prettier_store.update(cx, |prettier_store, cx| {
            prettier_store
                .check_installation_worktree(worktree, cx)
                .map_err(|error| anyhow!(error))?;
            Ok::<_, anyhow::Error>(crate::binary_downloads::request_tool_install(
                worktree, "prettier", cx,
            ))
        })??;
        let Some(wait) = wait else {
            return Ok(());
        };
        anyhow::ensure!(
            crate::binary_downloads::await_downloads_allowed(Some(wait), "prettier").await,
            "Prettier permission wait cancelled"
        );
    }
}

async fn prettier_package_is_resolvable(fs: &dyn Fs, package_dir: &Path) -> bool {
    if !fs.is_dir(package_dir).await {
        return false;
    }
    let mut candidates = Vec::new();
    let package_json_path = package_dir.join("package.json");
    if fs.is_file(&package_json_path).await {
        let Ok(contents) = fs.load(&package_json_path).await else {
            return false;
        };
        let Ok(package_json) =
            serde_json::from_str::<serde_json::Value>(contents.trim_start_matches('\u{feff}'))
        else {
            return false;
        };
        if let Some(main) = package_json
            .get("main")
            .and_then(serde_json::Value::as_str)
            .filter(|main| !main.is_empty())
        {
            let main = path::normalize_path(&package_dir.join(main));
            candidates.push(main.clone());
            candidates.extend([".js", ".json", ".node"].map(|extension| {
                let mut candidate = main.as_os_str().to_os_string();
                candidate.push(extension);
                PathBuf::from(candidate)
            }));
            candidates
                .extend(["index.js", "index.json", "index.node"].map(|index| main.join(index)));
        }
    }
    candidates
        .extend(["index.js", "index.json", "index.node"].map(|index| package_dir.join(index)));
    for candidate in candidates {
        if fs.is_file(&candidate).await {
            return true;
        }
    }
    false
}

async fn prepare_prettier_directory(fs: &dyn Fs) -> Result<()> {
    let default_prettier_dir = default_prettier_dir().as_path();
    match fs.metadata(default_prettier_dir).await.with_context(|| {
        format!("fetching FS metadata for default prettier dir {default_prettier_dir:?}")
    })? {
        Some(prettier_dir_metadata) => anyhow::ensure!(
            prettier_dir_metadata.is_dir,
            "default prettier dir {default_prettier_dir:?} is not a directory"
        ),
        None => fs
            .create_dir(default_prettier_dir)
            .await
            .with_context(|| format!("creating default prettier dir {default_prettier_dir:?}"))?,
    }
    Ok(())
}

async fn install_prettier_packages(
    packages_to_install: HashSet<Arc<str>>,
    node: NodeRuntime,
) -> anyhow::Result<()> {
    let packages_to_install = packages_to_install
        .iter()
        .map(|package_name| package_name.to_string())
        .collect::<Vec<_>>();
    let default_prettier_dir = default_prettier_dir().as_path();

    log::info!("Installing default prettier and plugins: {packages_to_install:?}");
    let borrowed_packages = packages_to_install
        .iter()
        .map(|package_name| package_name.as_str())
        .collect::<Vec<_>>();
    node.npm_install_latest_packages(default_prettier_dir, &borrowed_packages)
        .await
        .context("fetching formatter packages")?;
    anyhow::Ok(())
}

async fn ensure_prettier_server_file(fs: &dyn Fs) -> Result<()> {
    if should_write_prettier_server_file(fs).await {
        prepare_prettier_directory(fs).await?;
        save_prettier_server_file(fs).await?;
    }
    Ok(())
}

async fn save_prettier_server_file(fs: &dyn Fs) -> anyhow::Result<()> {
    let prettier_wrapper_path = default_prettier_dir().join(prettier::PRETTIER_SERVER_FILE);
    fs.save(
        &prettier_wrapper_path,
        &text::Rope::from(prettier::PRETTIER_SERVER_JS),
        text::LineEnding::Unix,
    )
    .await
    .with_context(|| {
        format!(
            "writing {} file at {prettier_wrapper_path:?}",
            prettier::PRETTIER_SERVER_FILE
        )
    })?;
    Ok(())
}

async fn should_write_prettier_server_file(fs: &dyn Fs) -> bool {
    let prettier_wrapper_path = default_prettier_dir().join(prettier::PRETTIER_SERVER_FILE);
    if !fs.is_file(&prettier_wrapper_path).await {
        return true;
    }
    let Ok(prettier_server_file_contents) = fs.load(&prettier_wrapper_path).await else {
        return true;
    };
    prettier_server_file_contents != prettier::PRETTIER_SERVER_JS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        binary_downloads::{self, BinaryDownloads, ToolInstall},
        worktree_store::WorktreeIdCounter,
    };
    use fs::FakeFs;
    use gpui::TestAppContext;
    use settings::{LocalSettingsKind, LocalSettingsPath, SettingsStore};
    use std::{cell::RefCell, rc::Rc};

    #[gpui::test]
    async fn test_default_prettier_install_scope(cx: &mut TestAppContext) {
        init_prettier_test(cx);
        let fs = FakeFs::new(cx.executor());
        let store = test_prettier_store(fs.clone(), cx);
        let installs = Rc::new(RefCell::new(Vec::new()));
        let first_worktree = add_test_worktree(&store, util::path!("/project-a"), cx).await;
        let second_worktree = add_test_worktree(&store, util::path!("/project-b"), cx).await;
        for (index, (worktree, (global, local))) in [first_worktree, second_worktree]
            .into_iter()
            .zip([(true, false), (false, true)])
            .enumerate()
        {
            set_download_settings(global, Some((worktree, local)), cx);
            let plugin = format!("plugin-{index}");
            let task =
                request_test_install(&store, Some(worktree), &[&plugin], &installs, cx).unwrap();
            if local {
                set_download_settings(true, Some((worktree, false)), cx);
                assert!(task.await.is_err());
                assert_eq!(*installs.borrow(), Vec::new());
                set_download_settings(global, Some((worktree, local)), cx);
                cx.run_until_parked();
                cx.executor().timer(Duration::from_millis(31)).await;
                cx.run_until_parked();
                assert_eq!(
                    *installs.borrow(),
                    vec![HashSet::from_iter([
                        Arc::from(plugin.as_str()),
                        Arc::from("prettier")
                    ])]
                );
                assert!(
                    request_test_install(&store, Some(worktree), &[&plugin], &installs, cx)
                        .is_none()
                );
                assert_eq!(installs.borrow().len(), 1);
            } else {
                assert!(task.await.is_err());
                assert_eq!(*installs.borrow(), Vec::new());
                let pending = cx.update(|cx| {
                    BinaryDownloads::try_get_global(cx)
                        .unwrap()
                        .read(cx)
                        .pending_tool_installs()
                });
                assert_eq!(
                    pending,
                    vec![ToolInstall {
                        worktree_id: Some(worktree),
                        tool: "prettier".into()
                    }]
                );
            }
        }
        set_download_settings(false, None, cx);
        for package in ["prettier", "plugin"] {
            let package_dir = default_prettier_dir().join("node_modules").join(package);
            fs.create_dir(&package_dir).await.unwrap();
            fs.save(
                &package_dir.join("package.json"),
                &text::Rope::from("{}"),
                text::LineEnding::Unix,
            )
            .await
            .unwrap();
            fs.save(
                &package_dir.join("index.js"),
                &text::Rope::from("module.exports = {};"),
                text::LineEnding::Unix,
            )
            .await
            .unwrap();
        }
        save_prettier_server_file(fs.as_ref()).await.unwrap();
        let cached_store = test_prettier_store(fs, cx);
        request_test_install(&cached_store, None, &["plugin"], &installs, cx)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(installs.borrow().len(), 1);
        let pending = cx.update(|cx| {
            BinaryDownloads::try_get_global(cx)
                .unwrap()
                .read(cx)
                .pending_tool_installs()
        });
        assert_eq!(
            pending,
            vec![ToolInstall {
                worktree_id: Some(first_worktree),
                tool: "prettier".into()
            }]
        );
    }

    #[gpui::test]
    async fn test_prettier_retries_revoked_node_preparation(cx: &mut TestAppContext) {
        init_prettier_test(cx);
        set_download_settings(true, None, cx);
        let store = test_prettier_store(FakeFs::new(cx.executor()), cx);
        let worktree = add_test_worktree(&store, util::path!("/project"), cx).await;
        let directory = tempfile::tempdir().unwrap();
        let (_options, receiver) = watch::channel(Some(node_runtime::NodeBinaryOptions {
            allow_path_lookup: false,
            allow_binary_downloads: true,
            use_paths: Some((
                directory.path().join("missing-node"),
                directory.path().join("missing-npm"),
            )),
        }));
        let runtime = NodeRuntime::new(
            http_client::FakeHttpClient::with_404_response(),
            None,
            receiver,
            None,
        );
        let runtime = cx.update(|cx| {
            crate::binary_downloads::scoped_node_runtime(&runtime, Some(worktree), "prettier", cx)
        });
        let (release, barrier) = futures::channel::oneshot::channel::<()>();
        let barrier = Rc::new(RefCell::new(Some(barrier)));
        let installs = Rc::new(RefCell::new(Vec::new()));
        let task = store.update(cx, |store, cx| {
            store.install_default_prettier_with(
                Some(worktree),
                [Arc::from("plugin")].into_iter(),
                {
                    let barrier = barrier.clone();
                    let installs = installs.clone();
                    move |packages, cx| {
                        if let Some(barrier) = barrier.borrow_mut().take() {
                            let runtime = runtime.clone();
                            cx.background_spawn(async move {
                                barrier.await?;
                                runtime.binary_path().await?;
                                Ok(())
                            })
                        } else {
                            installs.borrow_mut().push(packages);
                            Task::ready(Ok(()))
                        }
                    }
                },
                cx,
            );
            let PrettierInstallation::NotInstalled {
                installation_task, ..
            } = &store.default_prettier.prettier
            else {
                panic!("unexpected installed prettier")
            };
            installation_task.clone().unwrap()
        });
        cx.executor().timer(Duration::from_millis(31)).await;
        cx.run_until_parked();
        assert!(barrier.borrow().is_none());
        set_download_settings(true, Some((worktree, false)), cx);
        release.send(()).unwrap();
        assert!(task.await.unwrap_err().is::<ToolPermissionDenied>());
        assert_eq!(*installs.borrow(), Vec::new());
        store.read_with(cx, |store, _| {
            let PrettierInstallation::NotInstalled { attempts, .. } =
                &store.default_prettier.prettier
            else {
                panic!("unexpected installed prettier")
            };
            assert_eq!(*attempts, 0);
        });
        cx.update(|cx| {
            BinaryDownloads::try_get_global(cx)
                .unwrap()
                .update(cx, |downloads, cx| {
                    assert_eq!(
                        downloads.pending_tool_installs(),
                        vec![ToolInstall {
                            worktree_id: Some(worktree),
                            tool: "prettier".into()
                        }]
                    );
                    downloads.approve_tool_install(Some(worktree), "prettier", cx);
                });
        });
        cx.run_until_parked();
        cx.executor().timer(Duration::from_millis(31)).await;
        cx.run_until_parked();
        assert_eq!(
            *installs.borrow(),
            vec![HashSet::from_iter([
                Arc::from("prettier"),
                Arc::from("plugin")
            ])]
        );
        assert_eq!(
            store.read_with(cx, |store, _| store
                .default_prettier
                .instance()
                .unwrap()
                .attempt),
            0
        );
    }

    async fn add_test_worktree(
        store: &Entity<PrettierStore>,
        path: &str,
        cx: &mut TestAppContext,
    ) -> WorktreeId {
        let (fs, worktrees) = store.read_with(cx, |store, _| {
            (store.fs.clone(), store.worktree_store.clone())
        });
        let path = Path::new(path);
        fs.create_dir(path).await.unwrap();
        let (worktree, _) = worktrees
            .update(cx, |worktrees, cx| {
                worktrees.find_or_create_worktree(path, true, cx)
            })
            .await
            .unwrap();
        worktree.read_with(cx, |worktree, _| worktree.id())
    }

    fn init_prettier_test(cx: &mut TestAppContext) {
        cx.executor().forbid_parking();
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            binary_downloads::init(cx);
        });
    }

    fn set_download_settings(
        global: bool,
        local: Option<(WorktreeId, bool)>,
        cx: &mut TestAppContext,
    ) {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(global);
            });
            if let Some((worktree, allowed)) = local {
                store
                    .set_local_settings(
                        worktree,
                        LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                        LocalSettingsKind::Settings,
                        Some(&format!("{{\"allow_binary_downloads\":{allowed}}}")),
                        cx,
                    )
                    .unwrap();
            }
        });
    }

    fn test_prettier_store(fs: Arc<FakeFs>, cx: &mut TestAppContext) -> Entity<PrettierStore> {
        let worktree_store =
            cx.new(|cx| WorktreeStore::local(true, fs.clone(), WorktreeIdCounter::get(cx)));
        cx.new(|cx| {
            PrettierStore::new(
                NodeRuntime::unavailable(),
                fs,
                Arc::new(LanguageRegistry::test(cx.background_executor().clone())),
                worktree_store,
                cx,
            )
        })
    }

    fn request_test_install(
        store: &Entity<PrettierStore>,
        worktree: Option<WorktreeId>,
        plugins: &[&str],
        installs: &Rc<RefCell<Vec<HashSet<Arc<str>>>>>,
        cx: &mut TestAppContext,
    ) -> Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>> {
        let installs = installs.clone();
        store.update(cx, |store, cx| {
            store.install_default_prettier_with(
                worktree,
                plugins.iter().map(|plugin| Arc::from(*plugin)),
                move |plugins, _| {
                    installs.borrow_mut().push(plugins);
                    Task::ready(Ok(()))
                },
                cx,
            );
            match &store.default_prettier.prettier {
                PrettierInstallation::NotInstalled {
                    installation_task, ..
                } => installation_task.clone(),
                PrettierInstallation::Installed(_) => None,
            }
        })
    }
}
