use std::{
    ops::ControlFlow,
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
    Buffer, LanguageRegistry, LocalFile,
    language_settings::{Formatter, LanguageSettings, SelectedFormatter},
};
use lsp::{LanguageServer, LanguageServerId, LanguageServerName};
use node_runtime::NodeRuntime;
use paths::default_prettier_dir;
use prettier::Prettier;
use smol::stream::StreamExt;
use util::{ResultExt, TryFutureExt, rel_path::RelPath};

use crate::{
    File, PathChange, ProjectEntryId, Worktree, lsp_store::WorktreeId,
    worktree_store::WorktreeStore,
};

pub struct PrettierStore {
    node: NodeRuntime,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    worktree_store: Entity<WorktreeStore>,
    default_prettier: DefaultPrettier,
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
            prettiers_per_worktree: HashMap::default(),
            prettier_ignores_per_worktree: HashMap::default(),
            prettier_instances: HashMap::default(),
        }
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        self.prettier_ignores_per_worktree.remove(&id_to_remove);
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
                            let default_instance = lsp_store
                                .update(cx, |lsp_store, cx| {
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
                                .ok()?;
                            Some((None, default_instance?.log_err().await?))
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
                                    lsp_store.prettier_instances.get_mut(&prettier_dir).map(
                                        |existing_instance| {
                                            existing_instance.prettier_task(
                                                &node,
                                                Some(&prettier_dir),
                                                Some(worktree_id),
                                                cx,
                                            )
                                        },
                                    )
                                })
                                .ok()?
                            {
                                log::debug!("Found already started prettier in {prettier_dir:?}");
                                return Some((Some(prettier_dir), prettier_task?.await.log_err()?));
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
                let new_task = self.default_prettier.prettier_task(&node, None, cx);
                cx.spawn(async move |_, _| Some((None, new_task?.log_err().await?)))
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
        cx.spawn(async move |prettier_store, cx| {
            log::info!("Starting prettier at path {prettier_dir:?}");
            let new_server_id = prettier_store.read_with(cx, |prettier_store, _| {
                prettier_store.languages.next_language_server_id()
            })?;

            let new_prettier = Prettier::start(new_server_id, prettier_dir, node, cx.clone())
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
                        LanguageServerName("prettier (default)".to_string().into())
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
            .map(|name| RelPath::unix(name).unwrap())
            .collect::<HashSet<_>>();

        let prettier_config_file_changed = changes
            .iter()
            .filter(|(_, _, change)| !matches!(change, PathChange::Loaded))
            .filter(|(path, _, _)| {
                !path
                    .components()
                    .any(|component| component == "node_modules")
            })
            .find(|(path, _, _)| prettier_config_files.contains(path.as_ref()));
        let current_worktree_id = worktree.read(cx).id();
        if let Some((config_path, _, _)) = prettier_config_file_changed {
            log::info!(
                "Prettier config file {config_path:?} changed, reloading prettier instances for worktree {current_worktree_id}"
            );
            let prettiers_to_reload =
                self.prettiers_per_worktree
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
                    .chain(self.default_prettier.instance().map(|default_prettier| {
                        (current_worktree_id, None, default_prettier.clone())
                    }))
                    .collect::<Vec<_>>();

            cx.background_spawn(async move {
                let _: Vec<()> = future::join_all(prettiers_to_reload.into_iter().map(|(worktree_id, prettier_path, prettier_instance)| {
                    async move {
                        if let Some(instance) = prettier_instance.prettier {
                            match instance.await {
                                Ok(prettier) => {
                                    prettier.clear_cache().log_err().await;
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
                    }
                }))
                .await;
            })
                .detach();
        }
    }

    pub fn install_default_prettier(
        &mut self,
        worktree: Option<WorktreeId>,
        plugins: impl Iterator<Item = Arc<str>>,
        cx: &mut Context<Self>,
    ) {
        if cfg!(any(test, feature = "test-support")) {
            self.default_prettier.installed_plugins.extend(plugins);
            self.default_prettier.prettier = PrettierInstallation::Installed(PrettierInstance {
                attempt: 0,
                prettier: None,
            });
            return;
        }

        let mut new_plugins = plugins.collect::<HashSet<_>>();
        let node = self.node.clone();

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
                new_plugins.extend(not_installed_plugins.iter().cloned());
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
                    worktree.and_then(|worktree_id| {
                        prettier_store.worktree_store
                            .read(cx)
                            .worktree_for_id(worktree_id, cx)
                            .map(|worktree| worktree.read(cx).abs_path())
                    }).map(|locate_from| {
                        let installed_prettiers = prettier_store.prettier_instances.keys().cloned().collect();
                        (locate_from, installed_prettiers)
                    })
                })?;
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
                        let mut needs_install = should_write_prettier_server_file(fs.as_ref()).await;
                        if let Some(previous_installation_task) = previous_installation_task
                            && let Err(e) = previous_installation_task.await {
                                log::error!("Failed to install default prettier: {e:#}");
                                prettier_store.update(cx, |prettier_store, _| {
                                    if let PrettierInstallation::NotInstalled { attempts, not_installed_plugins, .. } = &mut prettier_store.default_prettier.prettier {
                                        *attempts += 1;
                                        new_plugins.extend(not_installed_plugins.iter().cloned());
                                        installation_attempt = *attempts;
                                        needs_install = true;
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
                        prettier_store.update(cx, |prettier_store, _| {
                            new_plugins.retain(|plugin| {
                                !prettier_store.default_prettier.installed_plugins.contains(plugin)
                            });
                            if let PrettierInstallation::NotInstalled { not_installed_plugins, .. } = &mut prettier_store.default_prettier.prettier {
                                not_installed_plugins.retain(|plugin| {
                                    !prettier_store.default_prettier.installed_plugins.contains(plugin)
                                });
                                not_installed_plugins.extend(new_plugins.iter().cloned());
                            }
                            needs_install |= !new_plugins.is_empty();
                        })?;
                        if needs_install {
                            log::info!("Initializing default prettier with plugins {new_plugins:?}");
                            let installed_plugins = new_plugins.clone();
                            cx.background_spawn(async move {
                                install_prettier_packages(fs.as_ref(), new_plugins, node).await?;
                                // Save the server file last, so the reinstall need could be determined by the absence of the file.
                                save_prettier_server_file(fs.as_ref()).await?;
                                anyhow::Ok(())
                            })
                                .await
                                .context("prettier & plugins install")
                                .map_err(Arc::new)?;
                            log::info!("Initialized default prettier with plugins: {installed_plugins:?}");
                            prettier_store.update(cx, |prettier_store, _| {
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
                                if let PrettierInstallation::NotInstalled { .. } = &mut prettier_store.default_prettier.prettier {
                                    prettier_store.default_prettier.prettier =
                                        PrettierInstallation::Installed(PrettierInstance {
                                            attempt: 0,
                                            prettier: None,
                                        });
                                }
                            })?;
                        }
                    }
                }
                Ok(())
            })
            .shared();
        self.default_prettier.prettier = PrettierInstallation::NotInstalled {
            attempts: installation_attempt,
            installation_task: Some(new_installation_task),
            not_installed_plugins: plugins_to_install,
        };
    }

    pub fn on_settings_changed(
        &mut self,
        language_formatters_to_check: Vec<(Option<WorktreeId>, LanguageSettings)>,
        cx: &mut Context<Self>,
    ) {
        let mut prettier_plugins_by_worktree = HashMap::default();
        for (worktree, language_settings) in language_formatters_to_check {
            if language_settings.prettier.allowed
                && let Some(plugins) = prettier_plugins_for_language(&language_settings)
            {
                prettier_plugins_by_worktree
                    .entry(worktree)
                    .or_insert_with(HashSet::default)
                    .extend(plugins.iter().cloned());
            }
        }
        for (worktree, prettier_plugins) in prettier_plugins_by_worktree {
            self.install_default_prettier(
                worktree,
                prettier_plugins.into_iter().map(Arc::from),
                cx,
            );
        }
    }
}

pub fn prettier_plugins_for_language(
    language_settings: &LanguageSettings,
) -> Option<&HashSet<String>> {
    match &language_settings.formatter {
        SelectedFormatter::Auto => Some(&language_settings.prettier.plugins),

        SelectedFormatter::List(list) => list
            .as_ref()
            .contains(&Formatter::Prettier)
            .then_some(&language_settings.prettier.plugins),
    }
}

pub(super) async fn format_with_prettier(
    prettier_store: &WeakEntity<PrettierStore>,
    buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) -> Option<Result<language::Diff>> {
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

    match prettier_task.await {
        Ok(prettier) => {
            let buffer_path = buffer
                .update(cx, |buffer, cx| {
                    File::from_dyn(buffer.file()).map(|file| file.abs_path(cx))
                })
                .ok()
                .flatten();

            let format_result = prettier
                .format(buffer, buffer_path, ignore_dir, cx)
                .await
                .with_context(|| format!("{} failed to format buffer", prettier_description));

            Some(format_result)
        }
        Err(error) => {
            prettier_store
                .update(cx, |project, _| {
                    let instance_to_update = match prettier_path {
                        Some(prettier_path) => project.prettier_instances.get_mut(&prettier_path),
                        None => match &mut project.default_prettier.prettier {
                            PrettierInstallation::NotInstalled { .. } => None,
                            PrettierInstallation::Installed(instance) => Some(instance),
                        },
                    };

                    if let Some(instance) = instance_to_update {
                        instance.attempt += 1;
                        instance.prettier = None;
                    }
                })
                .log_err();

            Some(Err(anyhow!(
                "{prettier_description} failed to spawn: {error:#}"
            )))
        }
    }
}

#[derive(Debug)]
pub struct DefaultPrettier {
    prettier: PrettierInstallation,
    installed_plugins: HashSet<Arc<str>>,
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
                    self.attempt += 1;
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

async fn install_prettier_packages(
    fs: &dyn Fs,
    plugins_to_install: HashSet<Arc<str>>,
    node: NodeRuntime,
) -> anyhow::Result<()> {
    let packages_to_versions = future::try_join_all(
        plugins_to_install
            .iter()
            .chain(Some(&"prettier".into()))
            .map(|package_name| async {
                let returned_package_name = package_name.to_string();
                let latest_version = node
                    .npm_package_latest_version(package_name)
                    .await
                    .with_context(|| {
                        format!("fetching latest npm version for package {returned_package_name}")
                    })?;
                anyhow::Ok((returned_package_name, latest_version))
            }),
    )
    .await
    .context("fetching latest npm versions")?;

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

    log::info!("Installing default prettier and plugins: {packages_to_versions:?}");
    let borrowed_packages = packages_to_versions
        .iter()
        .map(|(package, version)| (package.as_str(), version.as_str()))
        .collect::<Vec<_>>();
    node.npm_install_packages(default_prettier_dir, &borrowed_packages)
        .await
        .context("fetching formatter packages")?;
    anyhow::Ok(())
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
