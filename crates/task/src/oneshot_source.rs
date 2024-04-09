//! A source of tasks, based on ad-hoc user command prompt input.

use crate::{TaskSource, TaskTemplate, TaskTemplates};
use gpui::{AppContext, Context, Model};

/// A storage and source of tasks generated out of user command prompt inputs.
pub struct OneshotSource {
    tasks: TaskTemplates,
}

impl OneshotSource {
    /// Initializes the oneshot source, preparing to store user prompts.
    pub fn new(cx: &mut AppContext) -> Model<Box<dyn TaskSource>> {
        cx.new_model(|_| {
            Box::new(Self {
                tasks: TaskTemplates::default(),
            }) as Box<dyn TaskSource>
        })
    }

    /// Spawns a certain task based on the user prompt.
    pub fn spawn(&mut self, prompt: String) -> TaskTemplate {
        if let Some(task) = self.tasks.0.iter().find(|task| task.label == prompt) {
            // If we already have an oneshot task with that command, let's just reuse it.
            task.clone()
        } else {
            let new_oneshot = TaskTemplate {
                label: prompt.clone(),
                command: prompt,
                ..TaskTemplate::default()
            };
            self.tasks.0.push(new_oneshot.clone());
            new_oneshot
        }
    }

    /// TODO kb docs
    pub fn remove(&mut self, task_to_remove: &TaskTemplate) {
        self.tasks.0.retain(|task| task != task_to_remove);
    }
}

impl TaskSource for OneshotSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_to_schedule(
        &mut self,
        _cx: &mut gpui::ModelContext<Box<dyn TaskSource>>,
    ) -> TaskTemplates {
        self.tasks.clone()
    }
}
