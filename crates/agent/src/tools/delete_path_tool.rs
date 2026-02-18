use super::tool_permissions::{
    SensitiveSettingsKind, authorize_symlink_access, canonicalize_worktree_roots,
    detect_symlink_escape, sensitive_settings_kind,
};
use crate::{AgentTool, ToolCallEventStream, ToolPermissionDecision, decide_permission_for_path};
use action_log::ActionLog;
use agent_client_protocol::ToolKind;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use futures::{FutureExt as _, SinkExt, StreamExt, channel::mpsc};
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::path::Path;
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

/// Deletes the file or directory (and the directory's contents, recursively) at the specified path in the project, and returns confirmation of the deletion.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeletePathToolInput {
    /// The path of the file or directory to delete.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can delete the first file by providing a path of "directory1/a/something.txt"
    /// </example>
    pub path: String,
}

pub struct DeletePathTool {
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
}

impl DeletePathTool {
    pub fn new(project: Entity<Project>, action_log: Entity<ActionLog>) -> Self {
        Self {
            project,
            action_log,
        }
    }
}

impl AgentTool for DeletePathTool {
    type Input = DeletePathToolInput;
    type Output = String;

    const NAME: &'static str = "delete_path";

    fn kind() -> ToolKind {
        ToolKind::Delete
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Delete “`{}`”", input.path).into()
        } else {
            "Delete path".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let path = input.path;

        let settings = AgentSettings::get_global(cx);
        let decision = decide_permission_for_path(Self::NAME, &path, settings);

        if let ToolPermissionDecision::Deny(reason) = decision {
            return Task::ready(Err(anyhow!("{}", reason)));
        }

        let project = self.project.clone();
        let action_log = self.action_log.clone();
        cx.spawn(async move |cx| {
            let fs = project.read_with(cx, |project, _cx| project.fs().clone());
            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            let symlink_escape_target = project.read_with(cx, |project, cx| {
                detect_symlink_escape(project, &path, &canonical_roots, cx)
                    .map(|(_, target)| target)
            });

            let settings_kind = sensitive_settings_kind(Path::new(&path), fs.as_ref()).await;

            let decision =
                if matches!(decision, ToolPermissionDecision::Allow) && settings_kind.is_some() {
                    ToolPermissionDecision::Confirm
                } else {
                    decision
                };

            let authorize = if let Some(canonical_target) = symlink_escape_target {
                // Symlink escape authorization replaces (rather than supplements)
                // the normal tool-permission prompt. The symlink prompt already
                // requires explicit user approval with the canonical target shown,
                // which is strictly more security-relevant than a generic confirm.
                Some(cx.update(|cx| {
                    authorize_symlink_access(
                        Self::NAME,
                        &path,
                        &canonical_target,
                        &event_stream,
                        cx,
                    )
                }))
            } else {
                match decision {
                    ToolPermissionDecision::Allow => None,
                    ToolPermissionDecision::Confirm => Some(cx.update(|cx| {
                        let context =
                            crate::ToolPermissionContext::new(Self::NAME, vec![path.clone()]);
                        let title = format!("Delete {}", MarkdownInlineCode(&path));
                        let title = match settings_kind {
                            Some(SensitiveSettingsKind::Local) => {
                                format!("{title} (local settings)")
                            }
                            Some(SensitiveSettingsKind::Global) => format!("{title} (settings)"),
                            None => title,
                        };
                        event_stream.authorize(title, context, cx)
                    })),
                    ToolPermissionDecision::Deny(_) => None,
                }
            };

            if let Some(authorize) = authorize {
                authorize.await?;
            }

            let (project_path, worktree_snapshot) = project.read_with(cx, |project, cx| {
                let project_path = project.find_project_path(&path, cx).ok_or_else(|| {
                    anyhow!("Couldn't delete {path} because that path isn't in this project.")
                })?;
                let worktree = project
                    .worktree_for_id(project_path.worktree_id, cx)
                    .ok_or_else(|| {
                        anyhow!("Couldn't delete {path} because that path isn't in this project.")
                    })?;
                let worktree_snapshot = worktree.read(cx).snapshot();
                anyhow::Ok((project_path, worktree_snapshot))
            })?;

            let (mut paths_tx, mut paths_rx) = mpsc::channel(256);
            cx.background_spawn({
                let project_path = project_path.clone();
                async move {
                    for entry in
                        worktree_snapshot.traverse_from_path(true, false, false, &project_path.path)
                    {
                        if !entry.path.starts_with(&project_path.path) {
                            break;
                        }
                        paths_tx
                            .send(ProjectPath {
                                worktree_id: project_path.worktree_id,
                                path: entry.path.clone(),
                            })
                            .await?;
                    }
                    anyhow::Ok(())
                }
            })
            .detach();

            loop {
                let path_result = futures::select! {
                    path = paths_rx.next().fuse() => path,
                    _ = event_stream.cancelled_by_user().fuse() => {
                        anyhow::bail!("Delete cancelled by user");
                    }
                };
                let Some(path) = path_result else {
                    break;
                };
                if let Ok(buffer) = project
                    .update(cx, |project, cx| project.open_buffer(path, cx))
                    .await
                {
                    action_log.update(cx, |action_log, cx| {
                        action_log.will_delete_buffer(buffer.clone(), cx)
                    });
                }
            }

            let deletion_task = project
                .update(cx, |project, cx| {
                    project.delete_file(project_path, false, cx)
                })
                .with_context(|| {
                    format!("Couldn't delete {path} because that path isn't in this project.")
                })?;

            futures::select! {
                result = deletion_task.fuse() => {
                    result.with_context(|| format!("Deleting {path}"))?;
                }
                _ = event_stream.cancelled_by_user().fuse() => {
                    anyhow::bail!("Delete cancelled by user");
                }
            }
            Ok(format!("Deleted {path}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_client_protocol as acp;
    use fs::Fs as _;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::PathBuf;
    use util::path;

    use crate::ToolCallEventStream;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
        cx.update(|cx| {
            let mut settings = AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Allow;
            AgentSettings::override_global(settings, cx);
        });
    }

    #[gpui::test]
    async fn test_delete_path_symlink_escape_requests_authorization(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": { "main.rs": "fn main() {}" }
                },
                "external": {
                    "data": { "file.txt": "content" }
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link_to_external").as_ref(),
            PathBuf::from("../external"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(DeletePathTool::new(project, action_log));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                DeletePathToolInput {
                    path: "project/link_to_external".into(),
                },
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project") || title.contains("symlink"),
            "Authorization title should mention symlink escape, got: {title}",
        );

        auth.response
            .send(acp::PermissionOptionId::new("allow"))
            .unwrap();

        let result = task.await;
        // FakeFs cannot delete symlink entries (they are neither Dir nor File
        // internally), so the deletion itself may fail. The important thing is
        // that the authorization was requested and accepted — any error must
        // come from the fs layer, not from a permission denial.
        if let Err(err) = &result {
            let msg = format!("{err:#}");
            assert!(
                !msg.contains("denied") && !msg.contains("authorization"),
                "Error should not be a permission denial, got: {msg}",
            );
        }
    }

    #[gpui::test]
    async fn test_delete_path_symlink_escape_denied(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": { "main.rs": "fn main() {}" }
                },
                "external": {
                    "data": { "file.txt": "content" }
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link_to_external").as_ref(),
            PathBuf::from("../external"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(DeletePathTool::new(project, action_log));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                DeletePathToolInput {
                    path: "project/link_to_external".into(),
                },
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;

        drop(auth);

        let result = task.await;
        assert!(
            result.is_err(),
            "Tool should fail when authorization is denied"
        );
    }

    #[gpui::test]
    async fn test_delete_path_symlink_escape_confirm_requires_single_approval(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = AgentSettings::get_global(cx).clone();
            settings.tool_permissions.default = settings::ToolPermissionMode::Confirm;
            AgentSettings::override_global(settings, cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": { "main.rs": "fn main() {}" }
                },
                "external": {
                    "data": { "file.txt": "content" }
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link_to_external").as_ref(),
            PathBuf::from("../external"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(DeletePathTool::new(project, action_log));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                DeletePathToolInput {
                    path: "project/link_to_external".into(),
                },
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project") || title.contains("symlink"),
            "Authorization title should mention symlink escape, got: {title}",
        );

        auth.response
            .send(acp::PermissionOptionId::new("allow"))
            .unwrap();

        assert!(
            !matches!(
                event_rx.try_next(),
                Ok(Some(Ok(crate::ThreadEvent::ToolCallAuthorization(_))))
            ),
            "Expected a single authorization prompt",
        );

        let result = task.await;
        if let Err(err) = &result {
            let message = format!("{err:#}");
            assert!(
                !message.contains("denied") && !message.contains("authorization"),
                "Error should not be a permission denial, got: {message}",
            );
        }
    }

    #[gpui::test]
    async fn test_delete_path_symlink_escape_honors_deny_policy(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "delete_path".into(),
                agent_settings::ToolRules {
                    default: Some(settings::ToolPermissionMode::Deny),
                    ..Default::default()
                },
            );
            AgentSettings::override_global(settings, cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": { "main.rs": "fn main() {}" }
                },
                "external": {
                    "data": { "file.txt": "content" }
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link_to_external").as_ref(),
            PathBuf::from("../external"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(DeletePathTool::new(project, action_log));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| {
                tool.run(
                    DeletePathToolInput {
                        path: "project/link_to_external".into(),
                    },
                    event_stream,
                    cx,
                )
            })
            .await;

        assert!(result.is_err(), "Tool should fail when policy denies");
        assert!(
            !matches!(
                event_rx.try_next(),
                Ok(Some(Ok(crate::ThreadEvent::ToolCallAuthorization(_))))
            ),
            "Deny policy should not emit symlink authorization prompt",
        );
    }
}
