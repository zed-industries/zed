//! A source of tasks, based on ad-hoc user command prompt input.

use std::sync::Arc;

use crate::{
    static_source::RevealStrategy, SpawnInTerminal, Task, TaskContext, TaskId, TaskSource,
};
use gpui::{AppContext, Context, Model};

/// A storage and source of tasks generated out of user command prompt inputs.
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

    fn cwd(&self) -> Option<&str> {
        None
    }

    fn exec(&self, cx: TaskContext) -> Option<SpawnInTerminal> {
        if self.id().0.is_empty() {
            return None;
        }
        let TaskContext {
            cwd,
            task_variables,
        } = cx;
        Some(SpawnInTerminal {
            id: self.id().clone(),
            label: self.name().to_owned(),
            command: self.id().0.clone(),
            args: vec![],
            cwd,
            env: task_variables.0,
            use_new_terminal: Default::default(),
            allow_concurrent_runs: Default::default(),
            reveal: RevealStrategy::default(),
        })
    }
}

impl OneshotSource {
    /// Initializes the oneshot source, preparing to store user prompts.
    pub fn new(cx: &mut AppContext) -> Model<Box<dyn TaskSource>> {
        cx.new_model(|_| Box::new(Self { tasks: Vec::new() }) as Box<dyn TaskSource>)
    }

    /// Spawns a certain task based on the user prompt.
    pub fn spawn(&mut self, prompt: String) -> Arc<dyn Task> {
        if let Some(task) = self.tasks.iter().find(|task| task.id().0 == prompt) {
            // If we already have an oneshot task with that command, let's just reuse it.
            task.clone()
        } else {
            let new_oneshot = Arc::new(OneshotTask::new(prompt));
            self.tasks.push(new_oneshot.clone());
            new_oneshot
        }
    }
}

impl TaskSource for OneshotSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_for_path(
        &mut self,
        _path: Option<&std::path::Path>,
        _cx: &mut gpui::ModelContext<Box<dyn TaskSource>>,
    ) -> Vec<Arc<dyn Task>> {
        self.tasks.clone()
    }
}
