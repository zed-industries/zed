use std::sync::Arc;

use gpui::{AppContext, Model};
use task::{Source, SpawnInTerminal, Task, TaskId};
use ui::Context;

pub struct OneshotSource {
    tasks: Vec<Arc<dyn Task>>,
}

#[derive(Clone)]
struct OneshotTask {
    id: TaskId,
}

impl OneshotTask {
    fn new(prompt: String) -> Self {
        Self { id: TaskId(prompt) }
    }
}

impl Task for OneshotTask {
    fn id(&self) -> &TaskId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.id.0
    }

    fn cwd(&self) -> Option<&std::path::Path> {
        None
    }

    fn exec(&self, cwd: Option<std::path::PathBuf>) -> Option<SpawnInTerminal> {
        if self.id().0.is_empty() {
            return None;
        }
        Some(SpawnInTerminal {
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
        cx.new_model(|_| Box::new(Self { tasks: Vec::new() }) as Box<dyn Source>)
    }

    pub fn spawn(&mut self, prompt: String) -> Arc<dyn Task> {
        let ret = Arc::new(OneshotTask::new(prompt));
        self.tasks.push(ret.clone());
        ret
    }
}

impl Source for OneshotSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_for_path(
        &mut self,
        _path: Option<&std::path::Path>,
        _cx: &mut gpui::ModelContext<Box<dyn Source>>,
    ) -> Vec<Arc<dyn Task>> {
        self.tasks.clone()
    }
}
