use gpui::{AppContext, Global, Model, ModelContext};
use project::Fs;

#[allow(unused)]
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use crate::runtimes::{get_runtime_specifications, RuntimeSpecification};

// Per workspace
pub struct RuntimeManager {
    pub fs: Arc<dyn Fs>,
    pub runtime_specifications: Vec<RuntimeSpecification>,
}

#[derive(Clone)]
pub struct RuntimeManagerGlobal(Model<RuntimeManager>);

impl Global for RuntimeManagerGlobal {}

impl RuntimeManager {
    pub fn new(fs: Arc<dyn Fs>, _cx: &mut AppContext) -> Self {
        Self {
            fs,
            runtime_specifications: Default::default(),
        }
    }

    pub fn load(&mut self, cx: &mut ModelContext<Self>) {
        let task = get_runtime_specifications(self.fs.clone());

        cx.spawn(|this, mut cx| async move {
            let runtime_specs = task.await?;
            this.update(&mut cx, |this, _cx| {
                this.runtime_specifications = runtime_specs;
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn kernelspec(&self, language_name: Arc<str>) -> Option<RuntimeSpecification> {
        self.runtime_specifications
            .iter()
            .find(|runtime_specification| {
                runtime_specification.kernelspec.language == language_name.to_string()
            })
            .cloned()
    }

    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<RuntimeManagerGlobal>()
            .map(|runtime_manager| runtime_manager.0.clone())
    }

    pub fn set_global(runtime_manager: Model<Self>, cx: &mut AppContext) {
        cx.set_global(RuntimeManagerGlobal(runtime_manager));
    }

    pub fn remove_global(cx: &mut AppContext) {
        if RuntimeManager::global(cx).is_some() {
            cx.remove_global::<RuntimeManagerGlobal>();
        }
    }
}
