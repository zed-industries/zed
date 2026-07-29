use db::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use gpui::{App, Entity, Task};
use project::{Project, TaskContexts, TaskSourceKind};
use serde::{Deserialize, Serialize};
use task::{ResolvedTask, TaskTemplate};
use util::ResultExt as _;
use workspace::{WorkspaceDb, WorkspaceId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunConfigurationId {
    pub worktree_root: String,
    pub source_directory: String,
    pub task_label: String,
}

#[derive(Clone, Debug)]
pub struct RunConfiguration {
    pub id: RunConfigurationId,
    pub task_source_kind: TaskSourceKind,
    pub task_template: TaskTemplate,
}

impl RunConfiguration {
    pub fn resolve(&self, task_contexts: &TaskContexts) -> Option<ResolvedTask> {
        let context = match &self.task_source_kind {
            TaskSourceKind::Worktree { id, .. } => task_contexts
                .active_item_context
                .as_ref()
                .filter(|(worktree_id, _, _)| worktree_id.as_ref() == Some(id))
                .map(|(_, _, context)| context)
                .or_else(|| task_contexts.task_context_for_worktree_id(*id)),
            _ => task_contexts.active_context(),
        }?;

        self.task_template
            .resolve_task(&self.task_source_kind.to_id_base(), context)
    }
}

pub fn discover_run_configurations(
    project: Entity<Project>,
    cx: &mut App,
) -> Task<Vec<RunConfiguration>> {
    let Some(task_inventory) = project
        .read(cx)
        .task_store()
        .read(cx)
        .task_inventory()
        .cloned()
    else {
        return Task::ready(Vec::new());
    };

    let task_lists = project
        .read(cx)
        .visible_worktrees(cx)
        .map(|worktree| {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            let worktree_root = worktree.abs_path().to_string_lossy().into_owned();
            let tasks = task_inventory
                .read(cx)
                .list_tasks(None, None, Some(worktree_id), cx);
            (worktree_id, worktree_root, tasks)
        })
        .collect::<Vec<_>>();

    cx.spawn(async move |_| {
        let mut run_configurations = Vec::new();
        for (worktree_id, worktree_root, tasks) in task_lists {
            for (task_source_kind, task_template) in tasks.await {
                let TaskSourceKind::Worktree {
                    id,
                    directory_in_worktree,
                    ..
                } = &task_source_kind
                else {
                    continue;
                };
                if *id != worktree_id || directory_in_worktree.file_name() != Some(".zed") {
                    continue;
                }

                run_configurations.push(RunConfiguration {
                    id: RunConfigurationId {
                        worktree_root: worktree_root.clone(),
                        source_directory: directory_in_worktree.as_unix_str().to_string(),
                        task_label: task_template.label.clone(),
                    },
                    task_source_kind,
                    task_template,
                });
            }
        }
        run_configurations
    })
}

struct RunConfigurationsDb(ThreadSafeConnection);

impl Domain for RunConfigurationsDb {
    const NAME: &str = stringify!(RunConfigurationsDb);

    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE run_configuration_selections (
            workspace_id INTEGER PRIMARY KEY,
            worktree_root TEXT NOT NULL,
            source_directory TEXT NOT NULL,
            task_label TEXT NOT NULL,
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
        ) STRICT;
    )];
}

db::static_connection!(RunConfigurationsDb, [WorkspaceDb]);

impl RunConfigurationsDb {
    query! {
        async fn set_selected(
            workspace_id: WorkspaceId,
            worktree_root: String,
            source_directory: String,
            task_label: String
        ) -> Result<()> {
            INSERT INTO run_configuration_selections (
                workspace_id,
                worktree_root,
                source_directory,
                task_label
            )
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(workspace_id) DO UPDATE SET
                worktree_root = ?2,
                source_directory = ?3,
                task_label = ?4
        }
    }

    query! {
        fn selected(
            workspace_id: WorkspaceId
        ) -> Result<Option<(String, String, String)>> {
            SELECT worktree_root, source_directory, task_label
            FROM run_configuration_selections
            WHERE workspace_id = ?1
        }
    }
}

pub fn load_selected_run_configuration(
    workspace_id: Option<WorkspaceId>,
    cx: &App,
) -> Option<RunConfigurationId> {
    let (worktree_root, source_directory, task_label) = RunConfigurationsDb::global(cx)
        .selected(workspace_id?)
        .log_err()
        .flatten()?;
    Some(RunConfigurationId {
        worktree_root,
        source_directory,
        task_label,
    })
}

pub fn persist_selected_run_configuration(
    workspace_id: Option<WorkspaceId>,
    id: RunConfigurationId,
    cx: &App,
) {
    let Some(workspace_id) = workspace_id else {
        return;
    };
    let db = RunConfigurationsDb::global(cx);
    db::write_and_log(cx, move || async move {
        db.set_selected(
            workspace_id,
            id.worktree_root,
            id.source_directory,
            id.task_label,
        )
        .await
    });
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use serde_json::json;
    use util::path;

    use super::*;

    #[gpui::test]
    async fn discovers_only_zed_worktree_tasks(cx: &mut TestAppContext) {
        crate::tests::init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                ".zed": {
                    "tasks.json": r#"[
                        {"label": "dev", "command": "pnpm"},
                        {"label": "admin", "command": "pnpm"}
                    ]"#,
                },
                ".vscode": {
                    "tasks.json": r#"{
                        "version": "2.0.0",
                        "tasks": [
                            {"label": "vscode task", "type": "shell", "command": "echo"}
                        ]
                    }"#,
                }
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let discovery = cx.update(|cx| discover_run_configurations(project, cx));
        let run_configurations = discovery.await;

        assert_eq!(
            run_configurations
                .iter()
                .map(|configuration| configuration.task_template.label.as_str())
                .collect::<Vec<_>>(),
            ["dev", "admin"]
        );
        assert!(
            run_configurations
                .iter()
                .all(|configuration| configuration.id.source_directory == ".zed")
        );
    }

    #[gpui::test]
    async fn persists_selection_per_workspace(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        let workspace_db = cx.update(|cx| WorkspaceDb::global(cx));
        let first_workspace = workspace_db.next_id().await.unwrap();
        let second_workspace = workspace_db.next_id().await.unwrap();
        let selected = RunConfigurationId {
            worktree_root: "/workspace".to_string(),
            source_directory: ".zed".to_string(),
            task_label: "dev".to_string(),
        };

        let run_configurations_db = cx.update(|cx| RunConfigurationsDb::global(cx));
        run_configurations_db
            .set_selected(
                first_workspace,
                selected.worktree_root.clone(),
                selected.source_directory.clone(),
                selected.task_label.clone(),
            )
            .await
            .unwrap();

        assert_eq!(
            cx.read(|cx| load_selected_run_configuration(Some(first_workspace), cx)),
            Some(selected)
        );
        assert_eq!(
            cx.read(|cx| load_selected_run_configuration(Some(second_workspace), cx)),
            None
        );
    }
}
