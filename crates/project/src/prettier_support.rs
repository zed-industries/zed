use std::{
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{anyhow, Context, Result};
use collections::HashSet;
use fs::Fs;
use futures::{
    future::{self, Shared},
    FutureExt,
};
use gpui::{AsyncAppContext, Model, ModelContext, Task, WeakModel};
use language::{
    language_settings::{Formatter, LanguageSettings},
    Buffer, LanguageServerName, LocalFile,
};
use lsp::{LanguageServer, LanguageServerId};
use node_runtime::NodeRuntime;
use prettier::Prettier;
use util::{paths::DEFAULT_PRETTIER_DIR, ResultExt, TryFutureExt};

use crate::{
    Event, File, FormatOperation, PathChange, Project, ProjectEntryId, Worktree, WorktreeId,
};

pub fn prettier_plugins_for_language(
    language_settings: &LanguageSettings,
) -> Option<&HashSet<String>> {
    match &language_settings.formatter {
        Formatter::Prettier { .. } | Formatter::Auto => Some(&language_settings.prettier.plugins),
        Formatter::LanguageServer | Formatter::External { .. } | Formatter::CodeActions(_) => None,
    }
}

pub(super) async fn format_with_prettier(
    project: &WeakModel<Project>,
    buffer: &Model<Buffer>,
    cx: &mut AsyncAppContext,
) -> Option<Result<FormatOperation>> {
    let prettier_instance = project
        .update(cx, |project, cx| {
            project.prettier_instance_for_buffer(buffer, cx)
        })
        .ok()?
        .await;

    let Some((prettier_path, prettier_task)) = prettier_instance else {
        return None;
    };

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
                .format(buffer, buffer_path, cx)
                .await
                .map(FormatOperation::Prettier)
                .with_context(|| format!("{} failed to format buffer", prettier_description));

            Some(format_result)
        }
        Err(error) => {
            project
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
                "{} failed to spawn: {error:#}",
                prettier_description
            )))
        }
    }
}

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
        node: &Arc<dyn NodeRuntime>,
        worktree_id: Option<WorktreeId>,
        cx: &mut ModelContext<'_, Project>,
    ) -> Option<Task<anyhow::Result<PrettierTask>>> {
        match &mut self.prettier {
            PrettierInstallation::NotInstalled { .. } => {
                Some(start_default_prettier(Arc::clone(node), worktree_id, cx))
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

fn start_default_prettier(
    node: Arc<dyn NodeRuntime>,
    worktree_id: Option<WorktreeId>,
    cx: &mut ModelContext<'_, Project>,
) -> Task<anyhow::Result<PrettierTask>> {
    cx.spawn(|project, mut cx| async move {
        let installation_task = project.update(&mut cx, |project, _| {
            match &project.default_prettier.prettier {
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
                    project.update(&mut cx, |project, _| {
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
                let new_default_prettier = project.update(&mut cx, |project, cx| {
                    let new_default_prettier =
                        start_prettier(node, DEFAULT_PRETTIER_DIR.clone(), worktree_id, cx);
                    project.default_prettier.prettier =
                        PrettierInstallation::Installed(PrettierInstance {
                            attempt: 0,
                            prettier: Some(new_default_prettier.clone()),
                        });
                    new_default_prettier
                })?;
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
                    })?;
                    return Ok(new_default_prettier);
                }
            },
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
        log::info!("Starting prettier at path {prettier_dir:?}");
        let new_server_id = project.update(&mut cx, |project, _| {
            project.languages.next_language_server_id()
        })?;

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
    project: &WeakModel<Project>,
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
        project
            .update(cx, |project, cx| {
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
            })
            .ok();
    }
}

async fn install_prettier_packages(
    fs: &dyn Fs,
    plugins_to_install: HashSet<Arc<str>>,
    node: Arc<dyn NodeRuntime>,
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

    let default_prettier_dir = DEFAULT_PRETTIER_DIR.as_path();
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

async fn should_write_prettier_server_file(fs: &dyn Fs) -> bool {
    let prettier_wrapper_path = DEFAULT_PRETTIER_DIR.join(prettier::PRETTIER_SERVER_FILE);
    if !fs.is_file(&prettier_wrapper_path).await {
        return true;
    }
    let Ok(prettier_server_file_contents) = fs.load(&prettier_wrapper_path).await else {
        return true;
    };
    prettier_server_file_contents != prettier::PRETTIER_SERVER_JS
}

impl Project {
    pub fn update_prettier_settings(
        &self,
        worktree: &Model<Worktree>,
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

            cx.background_executor()
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
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Option<(Option<PathBuf>, PrettierTask)>> {
        if !self.is_local() {
            return Task::ready(None);
        }
        let buffer = buffer.read(cx);
        let buffer_file = buffer.file();
        if buffer.language().is_none() {
            return Task::ready(None);
        }
        let Some(node) = self.node.clone() else {
            return Task::ready(None);
        };
        match File::from_dyn(buffer_file).map(|file| (file.worktree_id(cx), file.abs_path(cx))) {
            Some((worktree_id, buffer_path)) => {
                let fs = Arc::clone(&self.fs);
                let installed_prettiers = self.prettier_instances.keys().cloned().collect();
                cx.spawn(|project, mut cx| async move {
                    match cx
                        .background_executor()
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
                        Ok(ControlFlow::Break(())) => None,
                        Ok(ControlFlow::Continue(None)) => {
                            let default_instance = project
                                .update(&mut cx, |project, cx| {
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
                                })
                                .ok()?;
                            Some((None, default_instance?.log_err().await?))
                        }
                        Ok(ControlFlow::Continue(Some(prettier_dir))) => {
                            project
                                .update(&mut cx, |project, _| {
                                    project
                                        .prettiers_per_worktree
                                        .entry(worktree_id)
                                        .or_default()
                                        .insert(Some(prettier_dir.clone()))
                                })
                                .ok()?;
                            if let Some(prettier_task) = project
                                .update(&mut cx, |project, cx| {
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
                                .ok()?
                            {
                                log::debug!("Found already started prettier in {prettier_dir:?}");
                                return Some((Some(prettier_dir), prettier_task?.await.log_err()?));
                            }

                            log::info!("Found prettier in {prettier_dir:?}, starting.");
                            let new_prettier_task = project
                                .update(&mut cx, |project, cx| {
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
                cx.spawn(|_, _| async move { Some((None, new_task?.log_err().await?)) })
            }
        }
    }

    pub fn install_default_prettier(
        &mut self,
        worktree: Option<WorktreeId>,
        plugins: impl Iterator<Item = Arc<str>>,
        cx: &mut ModelContext<Self>,
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
        let Some(node) = self.node.as_ref().cloned() else {
            return;
        };
        let fs = Arc::clone(&self.fs);
        let locate_prettier_installation = match worktree.and_then(|worktree_id| {
            self.worktree_for_id(worktree_id, cx)
                .map(|worktree| worktree.read(cx).abs_path())
        }) {
            Some(locate_from) => {
                let installed_prettiers = self.prettier_instances.keys().cloned().collect();
                cx.background_executor().spawn(async move {
                    Prettier::locate_prettier_installation(
                        fs.as_ref(),
                        &installed_prettiers,
                        locate_from.as_ref(),
                    )
                    .await
                })
            }
            None => Task::ready(Ok(ControlFlow::Continue(None))),
        };
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

        log::info!("Initializing default prettier with plugins {new_plugins:?}");
        let plugins_to_install = new_plugins.clone();
        let fs = Arc::clone(&self.fs);
        let new_installation_task = cx
            .spawn(|project, mut cx| async move {
                match locate_prettier_installation
                    .await
                    .context("locate prettier installation")
                    .map_err(Arc::new)?
                {
                    ControlFlow::Break(()) => return Ok(()),
                    ControlFlow::Continue(prettier_path) => {
                        if prettier_path.is_some() {
                            new_plugins.clear();
                        }
                        let mut needs_install = should_write_prettier_server_file(fs.as_ref()).await;
                        if let Some(previous_installation_task) = previous_installation_task {
                            if let Err(e) = previous_installation_task.await {
                                log::error!("Failed to install default prettier: {e:#}");
                                project.update(&mut cx, |project, _| {
                                    if let PrettierInstallation::NotInstalled { attempts, not_installed_plugins, .. } = &mut project.default_prettier.prettier {
                                        *attempts += 1;
                                        new_plugins.extend(not_installed_plugins.iter().cloned());
                                        installation_attempt = *attempts;
                                        needs_install = true;
                                    };
                                })?;
                            }
                        };
                        if installation_attempt > prettier::FAIL_THRESHOLD {
                            project.update(&mut cx, |project, _| {
                                if let PrettierInstallation::NotInstalled { installation_task, .. } = &mut project.default_prettier.prettier {
                                    *installation_task = None;
                                };
                            })?;
                            log::warn!(
                                "Default prettier installation had failed {installation_attempt} times, not attempting again",
                            );
                            return Ok(());
                        }
                        project.update(&mut cx, |project, _| {
                            new_plugins.retain(|plugin| {
                                !project.default_prettier.installed_plugins.contains(plugin)
                            });
                            if let PrettierInstallation::NotInstalled { not_installed_plugins, .. } = &mut project.default_prettier.prettier {
                                not_installed_plugins.retain(|plugin| {
                                    !project.default_prettier.installed_plugins.contains(plugin)
                                });
                                not_installed_plugins.extend(new_plugins.iter().cloned());
                            }
                            needs_install |= !new_plugins.is_empty();
                        })?;
                        if needs_install {
                            let installed_plugins = new_plugins.clone();
                            cx.background_executor()
                                .spawn(async move {
                                    install_prettier_packages(fs.as_ref(), new_plugins, node).await?;
                                    // Save the server file last, so the reinstall need could be determined by the absence of the file.
                                    save_prettier_server_file(fs.as_ref()).await?;
                                    anyhow::Ok(())
                                })
                                .await
                                .context("prettier & plugins install")
                                .map_err(Arc::new)?;
                            log::info!("Initialized prettier with plugins: {installed_plugins:?}");
                            project.update(&mut cx, |project, _| {
                                project.default_prettier.prettier =
                                    PrettierInstallation::Installed(PrettierInstance {
                                        attempt: 0,
                                        prettier: None,
                                    });
                                project.default_prettier
                                    .installed_plugins
                                    .extend(installed_plugins);
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
}
