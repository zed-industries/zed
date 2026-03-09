use super::tool_permissions::{
    SensitiveSettingsKind, authorize_symlink_escapes, canonicalize_worktree_roots,
    collect_symlink_escapes, sensitive_settings_kind,
};
use crate::{
    AgentTool, ToolCallEventStream, ToolInput, ToolPermissionDecision, decide_permission_for_paths,
};
use agent_client_protocol::ToolKind;
use agent_settings::AgentSettings;
use futures::FutureExt as _;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{path::Path, sync::Arc};
use util::markdown::MarkdownInlineCode;

/// Moves or rename a file or directory in the project, and returns confirmation that the move succeeded.
///
/// If the source and destination directories are the same, but the filename is different, this performs a rename. Otherwise, it performs a move.
///
/// This tool should be used when it's desirable to move or rename a file or directory without changing its contents at all.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MovePathToolInput {
    /// The source path of the file or directory to move/rename.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can move the first file by providing a source_path of "directory1/a/something.txt"
    /// </example>
    pub source_path: String,

    /// The destination path where the file or directory should be moved/renamed to.
    /// If the paths are the same except for the filename, then this will be a rename.
    ///
    /// <example>
    /// To move "directory1/a/something.txt" to "directory2/b/renamed.txt",
    /// provide a destination_path of "directory2/b/renamed.txt"
    /// </example>
    pub destination_path: String,
}

pub struct MovePathTool {
    project: Entity<Project>,
}

impl MovePathTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for MovePathTool {
    type Input = MovePathToolInput;
    type Output = String;

    const NAME: &'static str = "move_path";

    fn kind() -> ToolKind {
        ToolKind::Move
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let src = MarkdownInlineCode(&input.source_path);
            let dest = MarkdownInlineCode(&input.destination_path);
            let src_path = Path::new(&input.source_path);
            let dest_path = Path::new(&input.destination_path);

            match dest_path
                .file_name()
                .and_then(|os_str| os_str.to_os_string().into_string().ok())
            {
                Some(filename) if src_path.parent() == dest_path.parent() => {
                    let filename = MarkdownInlineCode(&filename);
                    format!("Rename {src} to {filename}").into()
                }
                _ => format!("Move {src} to {dest}").into(),
            }
        } else {
            "Move path".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;
            let paths = vec![input.source_path.clone(), input.destination_path.clone()];
            let decision = cx.update(|cx| {
                decide_permission_for_paths(Self::NAME, &paths, AgentSettings::get_global(cx))
            });
            if let ToolPermissionDecision::Deny(reason) = decision {
                return Err(reason);
            }

            let fs = project.read_with(cx, |project, _cx| project.fs().clone());
            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            let symlink_escapes: Vec<(&str, std::path::PathBuf)> =
                project.read_with(cx, |project, cx| {
                    collect_symlink_escapes(
                        project,
                        &input.source_path,
                        &input.destination_path,
                        &canonical_roots,
                        cx,
                    )
                });

            let sensitive_kind =
                sensitive_settings_kind(Path::new(&input.source_path), fs.as_ref())
                    .await
                    .or(
                        sensitive_settings_kind(Path::new(&input.destination_path), fs.as_ref())
                            .await,
                    );

            let needs_confirmation = matches!(decision, ToolPermissionDecision::Confirm)
                || (matches!(decision, ToolPermissionDecision::Allow) && sensitive_kind.is_some());

            let authorize = if !symlink_escapes.is_empty() {
                // Symlink escape authorization replaces (rather than supplements)
                // the normal tool-permission prompt. The symlink prompt already
                // requires explicit user approval with the canonical target shown,
                // which is strictly more security-relevant than a generic confirm.
                Some(cx.update(|cx| {
                    authorize_symlink_escapes(Self::NAME, &symlink_escapes, &event_stream, cx)
                }))
            } else if needs_confirmation {
                Some(cx.update(|cx| {
                    let src = MarkdownInlineCode(&input.source_path);
                    let dest = MarkdownInlineCode(&input.destination_path);
                    let context = crate::ToolPermissionContext::new(
                        Self::NAME,
                        vec![input.source_path.clone(), input.destination_path.clone()],
                    );
                    let title = format!("Move {src} to {dest}");
                    let title = match sensitive_kind {
                        Some(SensitiveSettingsKind::Local) => format!("{title} (local settings)"),
                        Some(SensitiveSettingsKind::Global) => format!("{title} (settings)"),
                        None => title,
                    };
                    event_stream.authorize(title, context, cx)
                }))
            } else {
                None
            };

            if let Some(authorize) = authorize {
                authorize.await.map_err(|e| e.to_string())?;
            }

            let rename_task = project.update(cx, |project, cx| {
                match project
                    .find_project_path(&input.source_path, cx)
                    .and_then(|project_path| project.entry_for_path(&project_path, cx))
                {
                    Some(entity) => match project.find_project_path(&input.destination_path, cx) {
                        Some(project_path) => Ok(project.rename_entry(entity.id, project_path, cx)),
                        None => Err(format!(
                            "Destination path {} was outside the project.",
                            input.destination_path
                        )),
                    },
                    None => Err(format!(
                        "Source path {} was not found in the project.",
                        input.source_path
                    )),
                }
            })?;

            futures::select! {
                result = rename_task.fuse() => result.map_err(|e| format!("Moving {} to {}: {e}", input.source_path, input.destination_path))?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Move cancelled by user".to_string());
                }
            };
            Ok(format!(
                "Moved {} to {}",
                input.source_path, input.destination_path
            ))
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
    async fn test_move_path_symlink_escape_source_requests_authorization(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": { "file.txt": "content" }
                },
                "external": {
                    "secret.txt": "SECRET"
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

        let tool = Arc::new(MovePathTool::new(project));

        let input = MovePathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_moved".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project")
                || title.contains("symlinks outside project"),
            "Authorization title should mention symlink escape, got: {title}",
        );

        auth.response
            .send(acp::PermissionOptionId::new("allow"))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "should succeed after approval: {result:?}");
    }

    #[gpui::test]
    async fn test_move_path_symlink_escape_denied(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": { "file.txt": "content" }
                },
                "external": {
                    "secret.txt": "SECRET"
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

        let tool = Arc::new(MovePathTool::new(project));

        let input = MovePathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_moved".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        drop(auth);

        let result = task.await;
        assert!(result.is_err(), "should fail when denied");
    }

    #[gpui::test]
    async fn test_move_path_symlink_escape_confirm_requires_single_approval(
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
                    "src": { "file.txt": "content" }
                },
                "external": {
                    "secret.txt": "SECRET"
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

        let tool = Arc::new(MovePathTool::new(project));

        let input = MovePathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_moved".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project")
                || title.contains("symlinks outside project"),
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
        assert!(
            result.is_ok(),
            "Tool should succeed after one authorization: {result:?}"
        );
    }

    #[gpui::test]
    async fn test_move_path_symlink_escape_honors_deny_policy(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "move_path".into(),
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
                    "src": { "file.txt": "content" }
                },
                "external": {
                    "secret.txt": "SECRET"
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

        let tool = Arc::new(MovePathTool::new(project));

        let input = MovePathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_moved".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx))
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
