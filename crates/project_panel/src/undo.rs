use anyhow::{Result, anyhow};
use circular_buffer::CircularBuffer;
use gpui::{Entity, Task};
use project::{Project, ProjectPath};
use ui::App;

const MAX_UNDO_OPERATIONS: usize = 10_000;

pub enum ProjectPanelOperation {
    Batch(Vec<ProjectPanelOperation>),
    Create {
        project_path: ProjectPath,
    },
    Rename {
        old_path: ProjectPath,
        new_path: ProjectPath,
    },
}

impl ProjectPanelOperation {
    pub fn batch(operations: impl IntoIterator<Item = Self>) -> Self {
        let mut operations: Vec<_> = operations.into_iter().collect();
        if operations.len() == 1
            && let Some(operation) = operations.pop()
        {
            operation
        } else {
            Self::Batch(operations)
        }
    }
}

pub struct UndoManager {
    project: Entity<Project>,
    stack: Box<circular_buffer::CircularBuffer<MAX_UNDO_OPERATIONS, ProjectPanelOperation>>,
}

impl UndoManager {
    pub fn new(project: Entity<Project>) -> Self {
        Self {
            project,
            stack: CircularBuffer::boxed(),
        }
    }

    pub fn undo(&mut self, cx: &mut App) {
        if let Some(operation) = self.stack.pop_back() {
            self.revert_operation(operation, cx).detach_and_log_err(cx);
        }
    }

    pub fn record(&mut self, operations: impl IntoIterator<Item = ProjectPanelOperation>) {
        self.stack
            .push_back(ProjectPanelOperation::batch(operations));
    }

    fn revert_operation(&self, operation: ProjectPanelOperation, cx: &mut App) -> Task<Result<()>> {
        match operation {
            ProjectPanelOperation::Create { project_path } => {
                let Some(entry_id) = self
                    .project
                    .read(cx)
                    .entry_for_path(&project_path, cx)
                    .map(|e| e.id)
                else {
                    return Task::ready(Err(anyhow!("no entry for path")));
                };
                let Some(task) = self
                    .project
                    .update(cx, |project, cx| project.delete_entry(entry_id, false, cx))
                else {
                    return Task::ready(Err(anyhow!("failed to trash entry")));
                };
                cx.spawn(async move |_cx| task.await.map(|_| ()))
            }
            ProjectPanelOperation::Rename { old_path, new_path } => {
                let Some(entry_id) = self
                    .project
                    .read(cx)
                    .entry_for_path(&new_path, cx)
                    .map(|e| e.id)
                else {
                    return Task::ready(Err(anyhow!("no entry for path")));
                };
                let task = self.project.update(cx, |project, cx| {
                    project.rename_entry(entry_id, old_path.clone(), cx)
                });
                cx.spawn(async move |_| task.await.map(|_| ()))
            }
            ProjectPanelOperation::Batch(operations) => {
                let tasks: Vec<_> = operations
                    .into_iter()
                    .map(|op| self.revert_operation(op, cx))
                    .collect();

                cx.spawn(async move |_| {
                    let results = futures::future::join_all(tasks).await;
                    let errors: Vec<_> = results.into_iter().filter_map(|r| r.err()).collect();
                    // TODO: better understand what to do with these errors
                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(anyhow!("Some operations failed"))
                    }
                })
            }
        }
    }
}
