use std::sync::Arc;

use anyhow::Result;
use client::telemetry::Telemetry;
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use gpui::{
    prelude::*, AppContext, EntityId, Global, Model, ModelContext, Subscription, Task, View,
};
use language::Language;
use project::{Fs, Project, WorktreeId};
use settings::{Settings, SettingsStore};

use crate::kernels::{local_kernel_specifications, python_env_kernel_specifications};
use crate::{JupyterSettings, KernelSpecification, Session};

struct GlobalReplStore(Model<ReplStore>);

impl Global for GlobalReplStore {}

pub struct ReplStore {
    fs: Arc<dyn Fs>,
    enabled: bool,
    sessions: HashMap<EntityId, View<Session>>,
    kernel_specifications: Vec<KernelSpecification>,
    selected_kernel_for_worktree: HashMap<WorktreeId, KernelSpecification>,
    kernel_specifications_for_worktree: HashMap<WorktreeId, Vec<KernelSpecification>>,
    telemetry: Arc<Telemetry>,
    _subscriptions: Vec<Subscription>,
}

impl ReplStore {
    const NAMESPACE: &'static str = "repl";

    pub(crate) fn init(fs: Arc<dyn Fs>, telemetry: Arc<Telemetry>, cx: &mut AppContext) {
        let store = cx.new_model(move |cx| Self::new(fs, telemetry, cx));

        store
            .update(cx, |store, cx| store.refresh_kernelspecs(cx))
            .detach_and_log_err(cx);

        cx.set_global(GlobalReplStore(store))
    }

    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalReplStore>().0.clone()
    }

    pub fn new(fs: Arc<dyn Fs>, telemetry: Arc<Telemetry>, cx: &mut ModelContext<Self>) -> Self {
        let subscriptions = vec![cx.observe_global::<SettingsStore>(move |this, cx| {
            this.set_enabled(JupyterSettings::enabled(cx), cx);
        })];

        let this = Self {
            fs,
            telemetry,
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

    pub fn telemetry(&self) -> &Arc<Telemetry> {
        &self.telemetry
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

    pub fn sessions(&self) -> impl Iterator<Item = &View<Session>> {
        self.sessions.values()
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut ModelContext<Self>) {
        if self.enabled == enabled {
            return;
        }

        self.enabled = enabled;
        self.on_enabled_changed(cx);
    }

    fn on_enabled_changed(&self, cx: &mut ModelContext<Self>) {
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
        project: &Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let kernel_specifications = python_env_kernel_specifications(project, worktree_id, cx);
        cx.spawn(move |this, mut cx| async move {
            let kernel_specifications = kernel_specifications
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get python kernelspecs: {:?}", e))?;

            this.update(&mut cx, |this, cx| {
                this.kernel_specifications_for_worktree
                    .insert(worktree_id, kernel_specifications);
                cx.notify();
            })
        })
    }

    pub fn refresh_kernelspecs(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let local_kernel_specifications = local_kernel_specifications(self.fs.clone());

        cx.spawn(|this, mut cx| async move {
            let local_kernel_specifications = local_kernel_specifications.await?;

            let mut kernel_options = Vec::new();
            for kernel_specification in local_kernel_specifications {
                kernel_options.push(KernelSpecification::Jupyter(kernel_specification));
            }

            this.update(&mut cx, |this, cx| {
                this.kernel_specifications = kernel_options;
                cx.notify();
            })
        })
    }

    pub fn set_active_kernelspec(
        &mut self,
        worktree_id: WorktreeId,
        kernelspec: KernelSpecification,
        _cx: &mut ModelContext<Self>,
    ) {
        self.selected_kernel_for_worktree
            .insert(worktree_id, kernelspec);
    }

    pub fn active_kernelspec(
        &self,
        worktree_id: WorktreeId,
        language_at_cursor: Option<Arc<Language>>,
        cx: &AppContext,
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
        cx: &AppContext,
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
                KernelSpecification::Remote(_) => {
                    unimplemented!()
                }
            })
            .cloned()
    }

    pub fn get_session(&self, entity_id: EntityId) -> Option<&View<Session>> {
        self.sessions.get(&entity_id)
    }

    pub fn insert_session(&mut self, entity_id: EntityId, session: View<Session>) {
        self.sessions.insert(entity_id, session);
    }

    pub fn remove_session(&mut self, entity_id: EntityId) {
        self.sessions.remove(&entity_id);
    }
}
