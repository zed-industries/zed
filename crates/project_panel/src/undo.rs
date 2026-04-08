use anyhow::anyhow;
use gpui::{AppContext, SharedString, Task, WeakEntity};
use project::ProjectPath;
use std::collections::VecDeque;
use ui::{App, IntoElement, Label, ParentElement, Styled, v_flex};
use workspace::{
    Workspace,
    notifications::{NotificationId, simple_message_notification::MessageNotification},
};

const MAX_UNDO_OPERATIONS: usize = 10_000;

#[derive(Clone)]
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

pub struct UndoManager {
    workspace: WeakEntity<Workspace>,
    stack: VecDeque<ProjectPanelOperation>,
    /// Maximum number of operations to keep on the undo stack.
    limit: usize,
}

impl UndoManager {
    pub fn new(workspace: WeakEntity<Workspace>) -> Self {
        Self::new_with_limit(workspace, MAX_UNDO_OPERATIONS)
    }

    pub fn new_with_limit(workspace: WeakEntity<Workspace>, limit: usize) -> Self {
        Self {
            workspace,
            limit,
            stack: VecDeque::new(),
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn undo(&mut self, cx: &mut App) {
        if let Some(operation) = self.stack.pop_back() {
            let task = self.revert_operation(operation, cx);
            let workspace = self.workspace.clone();

            cx.spawn(async move |cx| {
                let errors = task.await;
                if !errors.is_empty() {
                    cx.update(|cx| {
                        let messages = errors
                            .iter()
                            .map(|err| SharedString::from(err.to_string()))
                            .collect();

                        Self::show_errors(workspace, messages, cx)
                    })
                }
            })
            .detach();
        }
    }

    pub fn record(&mut self, operation: ProjectPanelOperation) {
        if self.stack.len() >= self.limit {
            self.stack.pop_front();
        }

        self.stack.push_back(operation);
    }

    pub fn record_batch(&mut self, operations: impl IntoIterator<Item = ProjectPanelOperation>) {
        let mut operations = operations.into_iter().collect::<Vec<_>>();
        let operation = match operations.len() {
            0 => return,
            1 => operations.pop().unwrap(),
            _ => ProjectPanelOperation::Batch(operations),
        };

        self.record(operation);
    }

    /// Attempts to revert the provided `operation`, returning a vector of errors
    /// in case there was any failure while reverting the operation.
    ///
    /// For all operations other than [`crate::undo::ProjectPanelOperation::Batch`], a maximum
    /// of one error is returned.
    fn revert_operation(
        &self,
        operation: ProjectPanelOperation,
        cx: &mut App,
    ) -> Task<Vec<anyhow::Error>> {
        match operation {
            ProjectPanelOperation::Create { project_path } => {
                let Some(workspace) = self.workspace.upgrade() else {
                    return Task::ready(vec![anyhow!("Failed to obtain workspace.")]);
                };

                let result = workspace.update(cx, |workspace, cx| {
                    workspace.project().update(cx, |project, cx| {
                        let entry_id = project
                            .entry_for_path(&project_path, cx)
                            .map(|entry| entry.id)
                            .ok_or_else(|| anyhow!("No entry for path."))?;

                        project
                            .delete_entry(entry_id, true, cx)
                            .ok_or_else(|| anyhow!("Failed to trash entry."))
                    })
                });

                let task = match result {
                    Ok(task) => task,
                    Err(err) => return Task::ready(vec![err]),
                };

                cx.spawn(async move |_| match task.await {
                    Ok(_) => vec![],
                    Err(err) => vec![err],
                })
            }
            ProjectPanelOperation::Rename { old_path, new_path } => {
                let Some(workspace) = self.workspace.upgrade() else {
                    return Task::ready(vec![anyhow!("Failed to obtain workspace.")]);
                };

                let result = workspace.update(cx, |workspace, cx| {
                    workspace.project().update(cx, |project, cx| {
                        let entry_id = project
                            .entry_for_path(&new_path, cx)
                            .map(|entry| entry.id)
                            .ok_or_else(|| anyhow!("No entry for path."))?;

                        Ok(project.rename_entry(entry_id, old_path.clone(), cx))
                    })
                });

                let task = match result {
                    Ok(task) => task,
                    Err(err) => return Task::ready(vec![err]),
                };

                cx.spawn(async move |_| match task.await {
                    Ok(_) => vec![],
                    Err(err) => vec![err],
                })
            }
            ProjectPanelOperation::Batch(operations) => {
                // When reverting operations in a batch, we reverse the order of
                // operations to handle dependencies between them. For example,
                // if a batch contains the following order of operations:
                //
                // 1. Create `src/`
                // 2. Create `src/main.rs`
                //
                // If we first try to revert the directory creation, it would
                // fail because there's still files inside the directory.
                // Operations are also reverted sequentially in order to avoid
                // this same problem.
                let tasks: Vec<_> = operations
                    .into_iter()
                    .rev()
                    .map(|operation| self.revert_operation(operation, cx))
                    .collect();

                cx.spawn(async move |_| {
                    let mut errors = Vec::new();
                    for task in tasks {
                        errors.extend(task.await);
                    }
                    errors
                })
            }
        }
    }

    /// Displays a notification with the list of provided errors ensuring that,
    /// when more than one error is provided, which can be the case when dealing
    /// with undoing a [`crate::undo::ProjectPanelOperation::Batch`], a list is
    /// displayed with each of the errors, instead of a single message.
    fn show_errors(workspace: WeakEntity<Workspace>, messages: Vec<SharedString>, cx: &mut App) {
        workspace
            .update(cx, move |workspace, cx| {
                let notification_id =
                    NotificationId::Named(SharedString::new_static("project_panel_undo"));

                workspace.show_notification(notification_id, cx, move |cx| {
                    cx.new(|cx| {
                        if let [err] = messages.as_slice() {
                            MessageNotification::new(err.to_string(), cx)
                                .with_title("Failed to undo Project Panel Operation")
                        } else {
                            MessageNotification::new_from_builder(cx, move |_, _| {
                                v_flex()
                                    .gap_1()
                                    .children(
                                        messages
                                            .iter()
                                            .map(|message| Label::new(format!("- {message}"))),
                                    )
                                    .into_any_element()
                            })
                            .with_title("Failed to undo Project Panel Operations")
                        }
                    })
                })
            })
            .ok();
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ProjectPanel, project_panel_tests,
        undo::{ProjectPanelOperation, UndoManager},
    };
    use gpui::{Entity, TestAppContext, VisualTestContext};
    use project::{FakeFs, Project, ProjectPath};
    use std::sync::Arc;
    use util::rel_path::rel_path;
    use workspace::MultiWorkspace;

    struct TestContext {
        project: Entity<Project>,
        panel: Entity<ProjectPanel>,
    }

    async fn init_test(cx: &mut TestAppContext) -> TestContext {
        project_panel_tests::init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let panel = workspace.update_in(cx, ProjectPanel::new);
        cx.run_until_parked();

        TestContext { project, panel }
    }

    #[gpui::test]
    async fn test_limit(cx: &mut TestAppContext) {
        let test_context = init_test(cx).await;
        let worktree_id = test_context.project.update(cx, |project, cx| {
            project.visible_worktrees(cx).next().unwrap().read(cx).id()
        });

        let build_create_operation = |file_name: &str| ProjectPanelOperation::Create {
            project_path: ProjectPath {
                path: Arc::from(rel_path(file_name)),
                worktree_id,
            },
        };

        // Since we're updating the `ProjectPanel`'s undo manager with one whose
        // limit is 3 operations, we only need to create 4 operations which
        // we'll record, in order to confirm that the oldest operation is
        // evicted.
        let operation_a = build_create_operation("file_a.txt");
        let operation_b = build_create_operation("file_b.txt");
        let operation_c = build_create_operation("file_c.txt");
        let operation_d = build_create_operation("file_d.txt");

        test_context.panel.update(cx, move |panel, _cx| {
            panel.undo_manager = UndoManager::new_with_limit(panel.workspace.clone(), 3);
            panel.undo_manager.record(operation_a);
            panel.undo_manager.record(operation_b);
            panel.undo_manager.record(operation_c);
            panel.undo_manager.record(operation_d);

            assert_eq!(panel.undo_manager.stack.len(), 3);
        });
    }
}
