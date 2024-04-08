//! A source of tasks, based on ad-hoc user command prompt input.

use std::sync::Arc;

use crate::{Task, TaskId, TaskSource, TaskTemplate};
use gpui::{AppContext, Context, Model};

/// A storage and source of tasks generated out of user command prompt inputs.
pub struct OneshotSource {
    tasks: Vec<Arc<dyn Task>>,
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
            let new_oneshot = TaskTemplate::oneshot(prompt);
            self.tasks.push(new_oneshot.clone());
            new_oneshot
        }
    }
    /// Removes a task with a given ID from this source.
    pub fn remove(&mut self, id: &TaskId) {
        let position = self.tasks.iter().position(|task| task.id() == id);
        if let Some(position) = position {
            self.tasks.remove(position);
        }
    }
}

impl TaskSource for OneshotSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_to_schedule(
        &mut self,
        _cx: &mut gpui::ModelContext<Box<dyn TaskSource>>,
    ) -> Vec<Arc<dyn Task>> {
        self.tasks.clone()
    }
}
