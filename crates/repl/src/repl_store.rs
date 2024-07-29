use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use gpui::{
    prelude::*, AppContext, EntityId, Global, Model, ModelContext, Subscription, Task, View,
};
use language::Language;
use project::Fs;
use settings::{Settings, SettingsStore};

use crate::kernels::kernel_specifications;
use crate::{JupyterSettings, KernelSpecification, Session};

struct GlobalReplStore(Model<ReplStore>);

impl Global for GlobalReplStore {}

pub struct ReplStore {
    fs: Arc<dyn Fs>,
    enabled: bool,
    sessions: HashMap<EntityId, View<Session>>,
    kernel_specifications: Vec<KernelSpecification>,
    _subscriptions: Vec<Subscription>,
}

impl ReplStore {
    pub(crate) fn init(fs: Arc<dyn Fs>, cx: &mut AppContext) {
        let store = cx.new_model(move |cx| Self::new(fs, cx));

        store
            .update(cx, |store, cx| store.refresh_kernelspecs(cx))
            .detach_and_log_err(cx);

        cx.set_global(GlobalReplStore(store))
    }

    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalReplStore>().0.clone()
    }

    pub fn new(fs: Arc<dyn Fs>, cx: &mut ModelContext<Self>) -> Self {
        let subscriptions = vec![cx.observe_global::<SettingsStore>(move |this, cx| {
            this.set_enabled(JupyterSettings::enabled(cx), cx);
        })];

        Self {
            fs,
            enabled: JupyterSettings::enabled(cx),
            sessions: HashMap::default(),
            kernel_specifications: Vec::new(),
            _subscriptions: subscriptions,
        }
    }

    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn kernel_specifications(&self) -> impl Iterator<Item = &KernelSpecification> {
        self.kernel_specifications.iter()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &View<Session>> {
        self.sessions.values()
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut ModelContext<Self>) {
        if self.enabled != enabled {
            self.enabled = enabled;
            cx.notify();
        }
    }

    pub fn refresh_kernelspecs(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let kernel_specifications = kernel_specifications(self.fs.clone());
        cx.spawn(|this, mut cx| async move {
            let kernel_specifications = kernel_specifications.await?;

            this.update(&mut cx, |this, cx| {
                this.kernel_specifications = kernel_specifications;
                cx.notify();
            })
        })
    }

    pub fn kernelspec(
        &self,
        language: &Language,
        cx: &mut ModelContext<Self>,
    ) -> Option<KernelSpecification> {
        let settings = JupyterSettings::get_global(cx);
        let language_name = language.code_fence_block_name();
        let selected_kernel = settings.kernel_selections.get(language_name.as_ref());

        self.kernel_specifications
            .iter()
            .find(|runtime_specification| {
                if let Some(selected) = selected_kernel {
                    // Top priority is the selected kernel
                    runtime_specification.name.to_lowercase() == selected.to_lowercase()
                } else {
                    // Otherwise, we'll try to find a kernel that matches the language
                    runtime_specification.kernelspec.language.to_lowercase()
                        == language_name.to_lowercase()
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
