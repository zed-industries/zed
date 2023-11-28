use std::{
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use collections::HashSet;
use fs::Fs;
use futures::{
    future::{self, Shared},
    FutureExt,
};
use gpui::{AsyncAppContext, ModelContext, ModelHandle, Task};
use language::{
    language_settings::{Formatter, LanguageSettings},
    Buffer, Language, LanguageServerName, LocalFile,
};
use lsp::LanguageServerId;
use node_runtime::NodeRuntime;
use prettier::Prettier;
use util::{paths::DEFAULT_PRETTIER_DIR, ResultExt, TryFutureExt};

use crate::{
    Event, File, FormatOperation, PathChange, Project, ProjectEntryId, Worktree, WorktreeId,
};

pub(super) async fn format_with_prettier(
    project: &ModelHandle<Project>,
    buffer: &ModelHandle<Buffer>,
    cx: &mut AsyncAppContext,
) -> Option<FormatOperation> {
    if let Some((prettier_path, prettier_task)) = project
        .update(cx, |project, cx| {
            project.prettier_instance_for_buffer(buffer, cx)
        })
        .await
    {
        match prettier_task.await {
            Ok(prettier) => {
                let buffer_path = buffer.update(cx, |buffer, cx| {
                    File::from_dyn(buffer.file()).map(|file| file.abs_path(cx))
                });
                match prettier.format(buffer, buffer_path, cx).await {
                    Ok(new_diff) => return Some(FormatOperation::Prettier(new_diff)),
                    Err(e) => {
                        log::error!(
                            "Prettier instance from {prettier_path:?} failed to format a buffer: {e:#}"
                        );
                    }
                }
            }
            Err(e) => project.update(cx, |project, _| {
                let instance_to_update = match prettier_path {
                    Some(prettier_path) => {
                        log::error!(
                            "Prettier instance from path {prettier_path:?} failed to spawn: {e:#}"
                        );
                        project.prettier_instances.get_mut(&prettier_path)
                    }
                    None => {
                        log::error!("Default prettier instance failed to spawn: {e:#}");
                        match &mut project.default_prettier.prettier {
                            PrettierInstallation::NotInstalled { .. } => None,
                            PrettierInstallation::Installed(instance) => Some(instance),
                        }
                    }
                };

                if let Some(instance) = instance_to_update {
                    instance.attempt += 1;
                    instance.prettier = None;
                }
            }),
        }
    }

    None
}

pub struct DefaultPrettier {
    prettier: PrettierInstallation,
    installed_plugins: HashSet<&'static str>,
}

pub enum PrettierInstallation {
    NotInstalled {
        attempts: usize,
        installation_process: Option<Shared<Task<Result<(), Arc<anyhow::Error>>>>>,
    },
    Installed(PrettierInstance),
}

pub type PrettierTask = Shared<Task<Result<Arc<Prettier>, Arc<anyhow::Error>>>>;

#[derive(Clone)]
pub struct PrettierInstance {
    attempt: usize,
    prettier: Option<PrettierTask>,
}

impl Default for DefaultPrettier {
    fn default() -> Self {
        Self {
            prettier: PrettierInstallation::NotInstalled {
                attempts: 0,
                installation_process: None,
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
        node: &Arc<dyn NodeRuntime>,
        worktree_id: Option<WorktreeId>,
        cx: &mut ModelContext<'_, Project>,
    ) -> Option<Task<anyhow::Result<PrettierTask>>> {
        match &mut self.prettier {
            PrettierInstallation::NotInstalled { .. } => {
                // `start_default_prettier` will start the installation process if it's not already running and wait for it to finish
                let new_task = start_default_prettier(Arc::clone(node), worktree_id, cx);
                Some(cx.spawn(|_, _| async move { new_task.await }))
            }
            PrettierInstallation::Installed(existing_instance) => {
                existing_instance.prettier_task(node, None, worktree_id, cx)
            }
        }
    }
}

impl PrettierInstance {
    pub fn prettier_task(
        &mut self,
        node: &Arc<dyn NodeRuntime>,
        prettier_dir: Option<&Path>,
        worktree_id: Option<WorktreeId>,
        cx: &mut ModelContext<'_, Project>,
    ) -> Option<Task<anyhow::Result<PrettierTask>>> {
        if self.attempt > prettier::LAUNCH_THRESHOLD {
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
                    let new_task = start_prettier(
                        Arc::clone(node),
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
                    let node = Arc::clone(node);
                    cx.spawn(|project, mut cx| async move {
                        project
                            .update(&mut cx, |_, cx| {
                                start_default_prettier(node, worktree_id, cx)
                            })
                            .await
                    })
                }
            },
        })
    }
}

fn start_default_prettier(
    node: Arc<dyn NodeRuntime>,
    worktree_id: Option<WorktreeId>,
    cx: &mut ModelContext<'_, Project>,
) -> Task<anyhow::Result<PrettierTask>> {
    cx.spawn(|project, mut cx| async move {
        loop {
            let installation_process = project.update(&mut cx, |project, _| {
                match &project.default_prettier.prettier {
                    PrettierInstallation::NotInstalled {
                        installation_process,
                        ..
                    } => ControlFlow::Continue(installation_process.clone()),
                    PrettierInstallation::Installed(default_prettier) => {
                        ControlFlow::Break(default_prettier.clone())
                    }
                }
            });
            match installation_process {
                ControlFlow::Continue(None) => {
                    anyhow::bail!("Default prettier is not installed and cannot be started")
                }
                ControlFlow::Continue(Some(installation_process)) => {
                    if let Err(e) = installation_process.await {
                        anyhow::bail!(
                            "Cannot start default prettier due to its installation failure: {e:#}"
                        );
                    }
                    let new_default_prettier = project.update(&mut cx, |project, cx| {
                        let new_default_prettier =
                            start_prettier(node, DEFAULT_PRETTIER_DIR.clone(), worktree_id, cx);
                        project.default_prettier.prettier =
                            PrettierInstallation::Installed(PrettierInstance {
                                attempt: 0,
                                prettier: Some(new_default_prettier.clone()),
                            });
                        new_default_prettier
                    });
                    return Ok(new_default_prettier);
                }
                ControlFlow::Break(instance) => match instance.prettier {
                    Some(instance) => return Ok(instance),
                    None => {
                        let new_default_prettier = project.update(&mut cx, |project, cx| {
                            let new_default_prettier =
                                start_prettier(node, DEFAULT_PRETTIER_DIR.clone(), worktree_id, cx);
                            project.default_prettier.prettier =
                                PrettierInstallation::Installed(PrettierInstance {
                                    attempt: instance.attempt + 1,
                                    prettier: Some(new_default_prettier.clone()),
                                });
                            new_default_prettier
                        });
                        return Ok(new_default_prettier);
                    }
                },
            }
        }
    })
}

fn start_prettier(
    node: Arc<dyn NodeRuntime>,
    prettier_dir: PathBuf,
    worktree_id: Option<WorktreeId>,
    cx: &mut ModelContext<'_, Project>,
) -> PrettierTask {
    cx.spawn(|project, mut cx| async move {
        let new_server_id = project.update(&mut cx, |project, _| {
            project.languages.next_language_server_id()
        });

        let new_prettier = Prettier::start(new_server_id, prettier_dir, node, cx.clone())
            .await
            .context("default prettier spawn")
            .map(Arc::new)
            .map_err(Arc::new)?;
        register_new_prettier(&project, &new_prettier, worktree_id, new_server_id, &mut cx);
        Ok(new_prettier)
    })
    .shared()
}

fn register_new_prettier(
    project: &ModelHandle<Project>,
    prettier: &Prettier,
    worktree_id: Option<WorktreeId>,
    new_server_id: LanguageServerId,
    cx: &mut AsyncAppContext,
) {
    let prettier_dir = prettier.prettier_dir();
    let is_default = prettier.is_default();
    if is_default {
        log::info!("Started default prettier in {prettier_dir:?}");
    } else {
        log::info!("Started prettier in {prettier_dir:?}");
    }
    if let Some(prettier_server) = prettier.server() {
        project.update(cx, |project, cx| {
            let name = if is_default {
                LanguageServerName(Arc::from("prettier (default)"))
            } else {
                let worktree_path = worktree_id
                    .and_then(|id| project.worktree_for_id(id, cx))
                    .map(|worktree| worktree.update(cx, |worktree, _| worktree.abs_path()));
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
                LanguageServerName(Arc::from(name))
            };
            project
                .supplementary_language_servers
                .insert(new_server_id, (name, Arc::clone(prettier_server)));
            cx.emit(Event::LanguageServerAdded(new_server_id));
        });
    }
}

async fn install_default_prettier(
    plugins_to_install: HashSet<&'static str>,
    node: Arc<dyn NodeRuntime>,
    fs: Arc<dyn Fs>,
) -> anyhow::Result<()> {
    let packages_to_versions =
        future::try_join_all(plugins_to_install.iter().chain(Some(&"prettier")).map(
            |package_name| async {
                let returned_package_name = package_name.to_string();
                let latest_version = node
                    .npm_package_latest_version(package_name)
                    .await
                    .with_context(|| {
                        format!("fetching latest npm version for package {returned_package_name}")
                    })?;
                anyhow::Ok((returned_package_name, latest_version))
            },
        ))
        .await
        .context("fetching latest npm versions")?;

    log::info!("Fetching default prettier and plugins: {packages_to_versions:?}");
    let borrowed_packages = packages_to_versions
        .iter()
        .map(|(package, version)| (package.as_str(), version.as_str()))
        .collect::<Vec<_>>();
    node.npm_install_packages(DEFAULT_PRETTIER_DIR.as_path(), &borrowed_packages)
        .await
        .context("fetching formatter packages")?;
    anyhow::Ok(())
}

async fn save_prettier_server_file(fs: &dyn Fs) -> Result<(), anyhow::Error> {
    let prettier_wrapper_path = DEFAULT_PRETTIER_DIR.join(prettier::PRETTIER_SERVER_FILE);
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

impl Project {
    pub fn update_prettier_settings(
        &self,
        worktree: &ModelHandle<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut ModelContext<'_, Project>,
    ) {
        let prettier_config_files = Prettier::CONFIG_FILE_NAMES
            .iter()
            .map(Path::new)
            .collect::<HashSet<_>>();

        let prettier_config_file_changed = changes
            .iter()
            .filter(|(_, _, change)| !matches!(change, PathChange::Loaded))
            .filter(|(path, _, _)| {
                !path
                    .components()
                    .any(|component| component.as_os_str().to_string_lossy() == "node_modules")
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

            cx.background()
                .spawn(async move {
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

    fn prettier_instance_for_buffer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Option<(Option<PathBuf>, PrettierTask)>> {
        let buffer = buffer.read(cx);
        let buffer_file = buffer.file();
        let Some(buffer_language) = buffer.language() else {
            return Task::ready(None);
        };
        if buffer_language.prettier_parser_name().is_none() {
            return Task::ready(None);
        }

        if self.is_local() {
            let Some(node) = self.node.as_ref().map(Arc::clone) else {
                return Task::ready(None);
            };
            match File::from_dyn(buffer_file).map(|file| (file.worktree_id(cx), file.abs_path(cx)))
            {
                Some((worktree_id, buffer_path)) => {
                    let fs = Arc::clone(&self.fs);
                    let installed_prettiers = self.prettier_instances.keys().cloned().collect();
                    return cx.spawn(|project, mut cx| async move {
                        match cx
                            .background()
                            .spawn(async move {
                                Prettier::locate_prettier_installation(
                                    fs.as_ref(),
                                    &installed_prettiers,
                                    &buffer_path,
                                )
                                .await
                            })
                            .await
                        {
                            Ok(ControlFlow::Break(())) => {
                                return None;
                            }
                            Ok(ControlFlow::Continue(None)) => {
                                let default_instance = project.update(&mut cx, |project, cx| {
                                    project
                                        .prettiers_per_worktree
                                        .entry(worktree_id)
                                        .or_default()
                                        .insert(None);
                                    project.default_prettier.prettier_task(
                                        &node,
                                        Some(worktree_id),
                                        cx,
                                    )
                                });
                                Some((None, default_instance?.log_err().await?))
                            }
                            Ok(ControlFlow::Continue(Some(prettier_dir))) => {
                                project.update(&mut cx, |project, _| {
                                    project
                                        .prettiers_per_worktree
                                        .entry(worktree_id)
                                        .or_default()
                                        .insert(Some(prettier_dir.clone()))
                                });
                                if let Some(prettier_task) =
                                    project.update(&mut cx, |project, cx| {
                                        project.prettier_instances.get_mut(&prettier_dir).map(
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
                                {
                                    log::debug!(
                                        "Found already started prettier in {prettier_dir:?}"
                                    );
                                    return Some((
                                        Some(prettier_dir),
                                        prettier_task?.await.log_err()?,
                                    ));
                                }

                                log::info!("Found prettier in {prettier_dir:?}, starting.");
                                let new_prettier_task = project.update(&mut cx, |project, cx| {
                                    let new_prettier_task = start_prettier(
                                        node,
                                        prettier_dir.clone(),
                                        Some(worktree_id),
                                        cx,
                                    );
                                    project.prettier_instances.insert(
                                        prettier_dir.clone(),
                                        PrettierInstance {
                                            attempt: 0,
                                            prettier: Some(new_prettier_task.clone()),
                                        },
                                    );
                                    new_prettier_task
                                });
                                Some((Some(prettier_dir), new_prettier_task))
                            }
                            Err(e) => {
                                log::error!("Failed to determine prettier path for buffer: {e:#}");
                                return None;
                            }
                        }
                    });
                }
                None => {
                    let new_task = self.default_prettier.prettier_task(&node, None, cx);
                    return cx
                        .spawn(|_, _| async move { Some((None, new_task?.log_err().await?)) });
                }
            }
        } else {
            return Task::ready(None);
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn install_default_prettier(
        &mut self,
        _worktree: Option<WorktreeId>,
        _new_language: &Language,
        language_settings: &LanguageSettings,
        _cx: &mut ModelContext<Self>,
    ) {
        // suppress unused code warnings
        match &language_settings.formatter {
            Formatter::Prettier { .. } | Formatter::Auto => {}
            Formatter::LanguageServer | Formatter::External { .. } => return,
        };
        let _ = &self.default_prettier.installed_plugins;
    }

    #[cfg(not(any(test, feature = "test-support")))]
    pub fn install_default_prettier(
        &mut self,
        worktree: Option<WorktreeId>,
        new_language: &Language,
        language_settings: &LanguageSettings,
        cx: &mut ModelContext<Self>,
    ) {
        match &language_settings.formatter {
            Formatter::Prettier { .. } | Formatter::Auto => {}
            Formatter::LanguageServer | Formatter::External { .. } => return,
        };
        let Some(node) = self.node.as_ref().cloned() else {
            return;
        };

        let mut prettier_plugins = None;
        if new_language.prettier_parser_name().is_some() {
            prettier_plugins
                .get_or_insert_with(|| HashSet::<&'static str>::default())
                .extend(
                    new_language
                        .lsp_adapters()
                        .iter()
                        .flat_map(|adapter| adapter.prettier_plugins()),
                )
        }
        let Some(prettier_plugins) = prettier_plugins else {
            return;
        };

        let fs = Arc::clone(&self.fs);
        let locate_prettier_installation = match worktree.and_then(|worktree_id| {
            self.worktree_for_id(worktree_id, cx)
                .map(|worktree| worktree.read(cx).abs_path())
        }) {
            Some(locate_from) => {
                let installed_prettiers = self.prettier_instances.keys().cloned().collect();
                cx.background().spawn(async move {
                    Prettier::locate_prettier_installation(
                        fs.as_ref(),
                        &installed_prettiers,
                        locate_from.as_ref(),
                    )
                    .await
                })
            }
            None => Task::ready(Ok(ControlFlow::Break(()))),
        };
        let mut plugins_to_install = prettier_plugins;
        plugins_to_install
            .retain(|plugin| !self.default_prettier.installed_plugins.contains(plugin));
        let mut installation_attempts = 0;
        let previous_installation_process = match &self.default_prettier.prettier {
            PrettierInstallation::NotInstalled {
                installation_process,
                attempts,
            } => {
                installation_attempts = *attempts;
                installation_process.clone()
            }
            PrettierInstallation::Installed { .. } => {
                if plugins_to_install.is_empty() {
                    return;
                }
                None
            }
        };

        if installation_attempts > prettier::LAUNCH_THRESHOLD {
            log::warn!(
                "Default prettier installation has failed {installation_attempts} times, not attempting again",
            );
            return;
        }

        let fs = Arc::clone(&self.fs);
        self.default_prettier.prettier = PrettierInstallation::NotInstalled {
            attempts: installation_attempts + 1,
            installation_process: Some(
                cx.spawn(|this, mut cx| async move {
                    match locate_prettier_installation
                        .await
                        .context("locate prettier installation")
                        .map_err(Arc::new)?
                    {
                        ControlFlow::Break(()) => return Ok(()),
                        ControlFlow::Continue(Some(_non_default_prettier)) => {
                            save_prettier_server_file(fs.as_ref()).await?;
                            return Ok(());
                        }
                        ControlFlow::Continue(None) => {
                            let mut needs_install = match previous_installation_process {
                                Some(previous_installation_process) => {
                                    previous_installation_process.await.is_err()
                                }
                                None => true,
                            };
                            this.update(&mut cx, |this, _| {
                                plugins_to_install.retain(|plugin| {
                                    !this.default_prettier.installed_plugins.contains(plugin)
                                });
                                needs_install |= !plugins_to_install.is_empty();
                            });
                            if needs_install {
                                let installed_plugins = plugins_to_install.clone();
                                cx.background()
                                    // TODO kb instead of always installing, try to start the existing installation first?
                                    .spawn(async move {
                                        save_prettier_server_file(fs.as_ref()).await?;
                                        install_default_prettier(plugins_to_install, node, fs).await
                                    })
                                    .await
                                    .context("prettier & plugins install")
                                    .map_err(Arc::new)?;
                                this.update(&mut cx, |this, _| {
                                    this.default_prettier.prettier =
                                        PrettierInstallation::Installed(PrettierInstance {
                                            attempt: 0,
                                            prettier: None,
                                        });
                                    this.default_prettier
                                        .installed_plugins
                                        .extend(installed_plugins);
                                });
                            }
                        }
                    }
                    Ok(())
                })
                .shared(),
            ),
        };
    }
}
