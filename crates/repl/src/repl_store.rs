use std::sync::Arc;

use anyhow::{Context as _, Result};
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use gpui::{App, Context, Entity, EntityId, Global, Subscription, Task, prelude::*};
use jupyter_websocket_client::RemoteServer;
use language::Language;
use project::{Fs, Project, WorktreeId};
use settings::{Settings, SettingsStore};

use crate::kernels::{
    list_remote_kernelspecs, local_kernel_specifications, python_env_kernel_specifications,
};
use crate::{JupyterSettings, KernelSpecification, Session};

struct GlobalReplStore(Entity<ReplStore>);

impl Global for GlobalReplStore {}

pub struct ReplStore {
    fs: Arc<dyn Fs>,
    enabled: bool,
    sessions: HashMap<EntityId, Entity<Session>>,
    kernel_specifications: Vec<KernelSpecification>,
    selected_kernel_for_worktree: HashMap<WorktreeId, KernelSpecification>,
    kernel_specifications_for_worktree: HashMap<WorktreeId, Vec<KernelSpecification>>,
    _subscriptions: Vec<Subscription>,
}

impl ReplStore {
    const NAMESPACE: &'static str = "repl";

    pub(crate) fn init(fs: Arc<dyn Fs>, cx: &mut App) {
        let store = cx.new(move |cx| Self::new(fs, cx));

        store
            .update(cx, |store, cx| store.refresh_kernelspecs(cx))
            .detach_and_log_err(cx);

        cx.set_global(GlobalReplStore(store))
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalReplStore>().0.clone()
    }

    pub fn new(fs: Arc<dyn Fs>, cx: &mut Context<Self>) -> Self {
        let subscriptions = vec![cx.observe_global::<SettingsStore>(move |this, cx| {
            this.set_enabled(JupyterSettings::enabled(cx), cx);
        })];

        let this = Self {
            fs,
            enabled: JupyterSettings::enabled(cx),
            sessions: HashMap::default(),
            kernel_specifications: Vec::new(),
            _subscriptions: subscriptions,
            kernel_specifications_for_worktree: HashMap::default(),
            selected_kernel_for_worktree: HashMap::default(),
        };
        this.on_enabled_changed(cx);
        this
    }

    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn kernel_specifications_for_worktree(
        &self,
        worktree_id: WorktreeId,
    ) -> impl Iterator<Item = &KernelSpecification> {
        self.kernel_specifications_for_worktree
            .get(&worktree_id)
            .into_iter()
            .flat_map(|specs| specs.iter())
            .chain(self.kernel_specifications.iter())
    }

    pub fn pure_jupyter_kernel_specifications(&self) -> impl Iterator<Item = &KernelSpecification> {
        self.kernel_specifications.iter()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &Entity<Session>> {
        self.sessions.values()
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut Context<Self>) {
        if self.enabled == enabled {
            return;
        }

        self.enabled = enabled;
        self.on_enabled_changed(cx);
    }

    fn on_enabled_changed(&self, cx: &mut Context<Self>) {
        if !self.enabled {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.hide_namespace(Self::NAMESPACE);
            });

            return;
        }

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.show_namespace(Self::NAMESPACE);
        });

        cx.notify();
    }

    pub fn refresh_python_kernelspecs(
        &mut self,
        worktree_id: WorktreeId,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let kernel_specifications = python_env_kernel_specifications(project, worktree_id, cx);
        cx.spawn(async move |this, cx| {
            let kernel_specifications = kernel_specifications
                .await
                .context("getting python kernelspecs")?;

            this.update(cx, |this, cx| {
                this.kernel_specifications_for_worktree
                    .insert(worktree_id, kernel_specifications);
                cx.notify();
            })
        })
    }

    fn get_remote_kernel_specifications(
        &self,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<Vec<KernelSpecification>>>> {
        match (
            std::env::var("JUPYTER_SERVER"),
            std::env::var("JUPYTER_TOKEN"),
        ) {
            (Ok(server), Ok(token)) => {
                let remote_server = RemoteServer {
                    base_url: server,
                    token,
                };
                let http_client = cx.http_client();
                Some(cx.spawn(async move |_, _| {
                    list_remote_kernelspecs(remote_server, http_client)
                        .await
                        .map(|specs| specs.into_iter().map(KernelSpecification::Remote).collect())
                }))
            }
            _ => None,
        }
    }

    pub fn refresh_kernelspecs(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let local_kernel_specifications = local_kernel_specifications(self.fs.clone());

        let remote_kernel_specifications = self.get_remote_kernel_specifications(cx);

        let all_specs = cx.background_spawn(async move {
            let mut all_specs = local_kernel_specifications
                .await?
                .into_iter()
                .map(KernelSpecification::Jupyter)
                .collect::<Vec<_>>();

            if let Some(remote_task) = remote_kernel_specifications
                && let Ok(remote_specs) = remote_task.await
            {
                all_specs.extend(remote_specs);
            }

            anyhow::Ok(all_specs)
        });

        cx.spawn(async move |this, cx| {
            let all_specs = all_specs.await;

            if let Ok(specs) = all_specs {
                this.update(cx, |this, cx| {
                    this.kernel_specifications = specs;
                    cx.notify();
                })
                .ok();
            }

            anyhow::Ok(())
        })
    }

    pub fn set_active_kernelspec(
        &mut self,
        worktree_id: WorktreeId,
        kernelspec: KernelSpecification,
        _cx: &mut Context<Self>,
    ) {
        self.selected_kernel_for_worktree
            .insert(worktree_id, kernelspec);
    }

    pub fn active_kernelspec(
        &self,
        worktree_id: WorktreeId,
        language_at_cursor: Option<Arc<Language>>,
        cx: &App,
    ) -> Option<KernelSpecification> {
        let selected_kernelspec = self.selected_kernel_for_worktree.get(&worktree_id).cloned();

        if let Some(language_at_cursor) = language_at_cursor {
            selected_kernelspec
                .or_else(|| self.kernelspec_legacy_by_lang_only(language_at_cursor, cx))
        } else {
            selected_kernelspec
        }
    }

    fn kernelspec_legacy_by_lang_only(
        &self,
        language_at_cursor: Arc<Language>,
        cx: &App,
    ) -> Option<KernelSpecification> {
        let settings = JupyterSettings::get_global(cx);
        let selected_kernel = settings
            .kernel_selections
            .get(language_at_cursor.code_fence_block_name().as_ref());

        let found_by_name = self
            .kernel_specifications
            .iter()
            .find(|runtime_specification| {
                if let (Some(selected), KernelSpecification::Jupyter(runtime_specification)) =
                    (selected_kernel, runtime_specification)
                {
                    // Top priority is the selected kernel
                    return runtime_specification.name.to_lowercase() == selected.to_lowercase();
                }
                false
            })
            .cloned();

        if let Some(found_by_name) = found_by_name {
            return Some(found_by_name);
        }

        self.kernel_specifications
            .iter()
            .find(|kernel_option| match kernel_option {
                KernelSpecification::Jupyter(runtime_specification) => {
                    runtime_specification.kernelspec.language.to_lowercase()
                        == language_at_cursor.code_fence_block_name().to_lowercase()
                }
                KernelSpecification::PythonEnv(runtime_specification) => {
                    runtime_specification.kernelspec.language.to_lowercase()
                        == language_at_cursor.code_fence_block_name().to_lowercase()
                }
                KernelSpecification::Remote(remote_spec) => {
                    remote_spec.kernelspec.language.to_lowercase()
                        == language_at_cursor.code_fence_block_name().to_lowercase()
                }
            })
            .cloned()
    }

    pub fn get_session(&self, entity_id: EntityId) -> Option<&Entity<Session>> {
        self.sessions.get(&entity_id)
    }

    pub fn insert_session(&mut self, entity_id: EntityId, session: Entity<Session>) {
        self.sessions.insert(entity_id, session);
    }

    pub fn remove_session(&mut self, entity_id: EntityId) {
        self.sessions.remove(&entity_id);
    }

    #[cfg(test)]
    pub fn set_kernel_specs_for_testing(
        &mut self,
        specs: Vec<KernelSpecification>,
        cx: &mut Context<Self>,
    ) {
        self.kernel_specifications = specs;
        cx.notify();
    }
}
