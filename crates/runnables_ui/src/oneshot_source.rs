use std::sync::Arc;

use gpui::{AppContext, Model};
use runnable::{Runnable, RunnableId, Source};
use ui::Context;

pub struct OneshotSource {
    runnables: Vec<Arc<dyn runnable::Runnable>>,
}

#[derive(Clone)]
struct OneshotRunnable {
    id: RunnableId,
}

impl OneshotRunnable {
    fn new(prompt: String) -> Self {
        Self {
            id: RunnableId(prompt),
        }
    }
}

impl Runnable for OneshotRunnable {
    fn id(&self) -> &runnable::RunnableId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.id.0
    }

    fn cwd(&self) -> Option<&std::path::Path> {
        None
    }

    fn exec(&self, cwd: Option<std::path::PathBuf>) -> Option<runnable::SpawnInTerminal> {
        if self.id().0.is_empty() {
            return None;
        }
        Some(runnable::SpawnInTerminal {
            id: self.id().clone(),
            label: self.name().to_owned(),
            command: self.id().0.clone(),
            args: vec![],
            cwd,
            env: Default::default(),
            use_new_terminal: Default::default(),
            allow_concurrent_runs: Default::default(),
            separate_shell: true,
        })
    }
}

impl OneshotSource {
    pub fn new(cx: &mut AppContext) -> Model<Box<dyn Source>> {
        cx.new_model(|_| Box::new(Self { runnables: vec![] }) as Box<dyn Source>)
    }

    pub fn spawn(&mut self, prompt: String) -> Arc<dyn runnable::Runnable> {
        let ret = Arc::new(OneshotRunnable::new(prompt));
        self.runnables.push(ret.clone());
        ret
    }
}

impl Source for OneshotSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn runnables_for_path(
        &mut self,
        _path: Option<&std::path::Path>,
        _cx: &mut gpui::ModelContext<Box<dyn Source>>,
    ) -> Vec<Arc<dyn runnable::Runnable>> {
        self.runnables.clone()
    }
}
