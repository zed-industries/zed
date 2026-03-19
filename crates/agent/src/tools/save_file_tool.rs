use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use collections::FxHashSet;
use futures::FutureExt as _;
use gpui::{App, Entity, SharedString, Task};
use language::Buffer;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

use super::tool_permissions::{
    ResolvedProjectPath, SensitiveSettingsKind, authorize_symlink_access,
    canonicalize_worktree_roots, path_has_symlink_escape, resolve_project_path,
    sensitive_settings_kind,
};
use crate::{
    AgentTool, ToolCallEventStream, ToolInput, ToolPermissionDecision, decide_permission_for_path,
};

/// Saves files that have unsaved changes.
///
/// Use this tool when you need to edit files but they have unsaved changes that must be saved first.
/// Only use this tool after asking the user for permission to save their unsaved changes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SaveFileToolInput {
    /// The paths of the files to save.
    pub paths: Vec<PathBuf>,
}

pub struct SaveFileTool {
    project: Entity<Project>,
}

impl SaveFileTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for SaveFileTool {
    type Input = SaveFileToolInput;
    type Output = String;

    const NAME: &'static str = "save_file";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) if input.paths.len() == 1 => "Save file".into(),
            Ok(input) => format!("Save {} files", input.paths.len()).into(),
            Err(_) => "Save files".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        let project = self.project.clone();

        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            // Check for any immediate deny before doing async work.
            for path in &input.paths {
                let path_str = path.to_string_lossy();
                let decision = cx.update(|cx| {
                    decide_permission_for_path(Self::NAME, &path_str, AgentSettings::get_global(cx))
                });
                if let ToolPermissionDecision::Deny(reason) = decision {
                    return Err(reason);
                }
            }

            let input_paths = input.paths;

            let fs = project.read_with(cx, |project, _cx| project.fs().clone());
            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            let mut confirmation_paths: Vec<String> = Vec::new();

            for path in &input_paths {
                let path_str = path.to_string_lossy();
                let decision = cx.update(|cx| {
                    decide_permission_for_path(Self::NAME, &path_str, AgentSettings::get_global(cx))
                });
                let symlink_escape = project.read_with(cx, |project, cx| {
                    path_has_symlink_escape(project, path, &canonical_roots, cx)
                });

                match decision {
                    ToolPermissionDecision::Allow => {
                        if !symlink_escape {
                            let is_sensitive = super::tool_permissions::is_sensitive_settings_path(
                                Path::new(&*path_str),
                                fs.as_ref(),
                            )
                            .await;
                            if is_sensitive {
                                confirmation_paths.push(path_str.to_string());
                            }
                        }
                    }
                    ToolPermissionDecision::Deny(reason) => {
                        return Err(reason);
                    }
                    ToolPermissionDecision::Confirm => {
                        if !symlink_escape {
                            confirmation_paths.push(path_str.to_string());
                        }
                    }
                }
            }

            if !confirmation_paths.is_empty() {
                let title = if confirmation_paths.len() == 1 {
                    format!("Save {}", MarkdownInlineCode(&confirmation_paths[0]))
                } else {
                    let paths: Vec<_> = confirmation_paths
                        .iter()
                        .take(3)
                        .map(|p| p.as_str())
                        .collect();
                    if confirmation_paths.len() > 3 {
                        format!(
                            "Save {}, and {} more",
                            paths.join(", "),
                            confirmation_paths.len() - 3
                        )
                    } else {
                        format!("Save {}", paths.join(", "))
                    }
                };

                let mut settings_kind = None;
                for p in &confirmation_paths {
                    if let Some(kind) = sensitive_settings_kind(Path::new(p), fs.as_ref()).await {
                        settings_kind = Some(kind);
                        break;
                    }
                }
                let title = match settings_kind {
                    Some(SensitiveSettingsKind::Local) => format!("{title} (local settings)"),
                    Some(SensitiveSettingsKind::Global) => format!("{title} (settings)"),
                    None => title,
                };
                let context =
                    crate::ToolPermissionContext::new(Self::NAME, confirmation_paths.clone());
                let authorize = cx.update(|cx| event_stream.authorize(title, context, cx));
                authorize.await.map_err(|e| e.to_string())?;
            }

            let mut buffers_to_save: FxHashSet<Entity<Buffer>> = FxHashSet::default();

            let mut dirty_count: usize = 0;
            let mut clean_paths: Vec<PathBuf> = Vec::new();
            let mut not_found_paths: Vec<PathBuf> = Vec::new();
            let mut open_errors: Vec<(PathBuf, String)> = Vec::new();
            let mut authorization_errors: Vec<(PathBuf, String)> = Vec::new();
            let mut save_errors: Vec<(String, String)> = Vec::new();

            for path in input_paths {
                let project_path = match project.read_with(cx, |project, cx| {
                    resolve_project_path(project, &path, &canonical_roots, cx)
                }) {
                    Ok(resolved) => {
                        let (project_path, symlink_canonical_target) = match resolved {
                            ResolvedProjectPath::Safe(path) => (path, None),
                            ResolvedProjectPath::SymlinkEscape {
                                project_path,
                                canonical_target,
                            } => (project_path, Some(canonical_target)),
                        };
                        if let Some(canonical_target) = &symlink_canonical_target {
                            let path_str = path.to_string_lossy();
                            let authorize_task = cx.update(|cx| {
                                authorize_symlink_access(
                                    Self::NAME,
                                    &path_str,
                                    canonical_target,
                                    &event_stream,
                                    cx,
                                )
                            });
                            let result = authorize_task.await;
                            if let Err(err) = result {
                                authorization_errors.push((path.clone(), err.to_string()));
                                continue;
                            }
                        }
                        project_path
                    }
                    Err(_) => {
                        not_found_paths.push(path);
                        continue;
                    }
                };

                let open_buffer_task =
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx));

                let buffer = futures::select! {
                    result = open_buffer_task.fuse() => {
                        match result {
                            Ok(buffer) => buffer,
                            Err(error) => {
                                open_errors.push((path, error.to_string()));
                                continue;
                            }
                        }
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Save cancelled by user".to_string());
                    }
                };

                let is_dirty = buffer.read_with(cx, |buffer, _| buffer.is_dirty());

                if is_dirty {
                    buffers_to_save.insert(buffer);
                    dirty_count += 1;
                } else {
                    clean_paths.push(path);
                }
            }

            // Save each buffer individually since there's no batch save API.
            for buffer in buffers_to_save {
                let path_for_buffer = buffer
                    .read_with(cx, |buffer, _| {
                        buffer
                            .file()
                            .map(|file| file.path().to_rel_path_buf())
                            .map(|path| path.as_rel_path().as_unix_str().to_owned())
                    })
                    .unwrap_or_else(|| "<unknown>".to_string());

                let save_task = project.update(cx, |project, cx| project.save_buffer(buffer, cx));

                let save_result = futures::select! {
                    result = save_task.fuse() => result,
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Save cancelled by user".to_string());
                    }
                };
                if let Err(error) = save_result {
                    save_errors.push((path_for_buffer, error.to_string()));
                }
            }

            let mut lines: Vec<String> = Vec::new();

            let successful_saves = dirty_count.saturating_sub(save_errors.len());
            if successful_saves > 0 {
                lines.push(format!("Saved {} file(s).", successful_saves));
            }
            if !clean_paths.is_empty() {
                lines.push(format!("{} clean.", clean_paths.len()));
            }

            if !not_found_paths.is_empty() {
                lines.push(format!("Not found ({}):", not_found_paths.len()));
                for path in &not_found_paths {
                    lines.push(format!("- {}", path.display()));
                }
            }
            if !open_errors.is_empty() {
                lines.push(format!("Open failed ({}):", open_errors.len()));
                for (path, error) in &open_errors {
                    lines.push(format!("- {}: {}", path.display(), error));
                }
            }
            if !authorization_errors.is_empty() {
                lines.push(format!(
                    "Authorization failed ({}):",
                    authorization_errors.len()
                ));
                for (path, error) in &authorization_errors {
                    lines.push(format!("- {}: {}", path.display(), error));
                }
            }
            if !save_errors.is_empty() {
                lines.push(format!("Save failed ({}):", save_errors.len()));
                for (path, error) in &save_errors {
                    lines.push(format!("- {}: {}", path, error));
                }
            }

            if lines.is_empty() {
                Ok("No paths provided.".to_string())
            } else {
                Ok(lines.join("\n"))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::Fs as _;
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
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
    async fn test_save_file_output_and_effects(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "dirty.txt": "on disk: dirty\n",
                "clean.txt": "on disk: clean\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(SaveFileTool::new(project.clone()));

        // Make dirty.txt dirty in-memory.
        let dirty_project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path("root/dirty.txt", cx)
                .expect("dirty.txt should exist in project")
        });

        let dirty_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(dirty_project_path, cx)
            })
            .await
            .unwrap();
        dirty_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), "in memory: dirty\n")], None, cx);
        });
        assert!(
            dirty_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "dirty.txt buffer should be dirty before save"
        );

        // Ensure clean.txt is opened but remains clean.
        let clean_project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path("root/clean.txt", cx)
                .expect("clean.txt should exist in project")
        });

        let clean_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(clean_project_path, cx)
            })
            .await
            .unwrap();
        assert!(
            !clean_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "clean.txt buffer should start clean"
        );

        let output = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(SaveFileToolInput {
                        paths: vec![
                            PathBuf::from("root/dirty.txt"),
                            PathBuf::from("root/clean.txt"),
                        ],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();

        // Output should mention saved + clean.
        assert!(
            output.contains("Saved 1 file(s)."),
            "expected saved count line, got:\n{output}"
        );
        assert!(
            output.contains("1 clean."),
            "expected clean count line, got:\n{output}"
        );

        // Effect: dirty buffer should now be clean and disk should have new content.
        assert!(
            !dirty_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "dirty.txt buffer should not be dirty after save"
        );

        let disk_dirty = fs.load(path!("/root/dirty.txt").as_ref()).await.unwrap();
        assert_eq!(
            disk_dirty, "in memory: dirty\n",
            "dirty.txt disk content should be updated"
        );

        // Sanity: clean buffer should remain clean and disk unchanged.
        let disk_clean = fs.load(path!("/root/clean.txt").as_ref()).await.unwrap();
        assert_eq!(disk_clean, "on disk: clean\n");

        // Test empty paths case.
        let output = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(SaveFileToolInput { paths: vec![] }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(output, "No paths provided.");

        // Test not-found path case.
        let output = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(SaveFileToolInput {
                        paths: vec![PathBuf::from("nonexistent/path.txt")],
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await
            .unwrap();
        assert!(
            output.contains("Not found (1):"),
            "expected not-found header line, got:\n{output}"
        );
        assert!(
            output.contains("- nonexistent/path.txt"),
            "expected not-found path bullet, got:\n{output}"
        );
    }

    #[gpui::test]
    async fn test_save_file_symlink_escape_requests_authorization(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "src": {}
                },
                "external": {
                    "secret.txt": "secret content"
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link.txt").as_ref(),
            PathBuf::from("../external/secret.txt"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let tool = Arc::new(SaveFileTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.clone().run(
                ToolInput::resolved(SaveFileToolInput {
                    paths: vec![PathBuf::from("project/link.txt")],
                }),
                event_stream,
                cx,
            )
        });

        cx.run_until_parked();

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project"),
            "Expected symlink escape authorization, got: {title}",
        );

        auth.response
            .send(acp::PermissionOptionId::new("allow"))
            .unwrap();

        let _result = task.await;
    }

    #[gpui::test]
    async fn test_save_file_symlink_escape_honors_deny_policy(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "save_file".into(),
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
                    "src": {}
                },
                "external": {
                    "secret.txt": "secret content"
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link.txt").as_ref(),
            PathBuf::from("../external/secret.txt"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let tool = Arc::new(SaveFileTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    ToolInput::resolved(SaveFileToolInput {
                        paths: vec![PathBuf::from("project/link.txt")],
                    }),
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

    #[gpui::test]
    async fn test_save_file_symlink_escape_confirm_requires_single_approval(
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
                    "src": {}
                },
                "external": {
                    "secret.txt": "secret content"
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link.txt").as_ref(),
            PathBuf::from("../external/secret.txt"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let tool = Arc::new(SaveFileTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.clone().run(
                ToolInput::resolved(SaveFileToolInput {
                    paths: vec![PathBuf::from("project/link.txt")],
                }),
                event_stream,
                cx,
            )
        });

        cx.run_until_parked();

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("points outside the project"),
            "Expected symlink escape authorization, got: {title}",
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

        let _result = task.await;
    }

    #[gpui::test]
    async fn test_save_file_symlink_denial_does_not_reduce_success_count(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "dirty.txt": "on disk value\n",
                },
                "external": {
                    "secret.txt": "secret content"
                }
            }),
        )
        .await;

        fs.create_symlink(
            path!("/root/project/link.txt").as_ref(),
            PathBuf::from("../external/secret.txt"),
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let dirty_project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path("project/dirty.txt", cx)
                .expect("dirty.txt should exist in project")
        });
        let dirty_buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(dirty_project_path, cx)
            })
            .await
            .unwrap();
        dirty_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), "in memory value\n")], None, cx);
        });
        assert!(
            dirty_buffer.read_with(cx, |buffer, _| buffer.is_dirty()),
            "dirty.txt should be dirty before save"
        );

        let tool = Arc::new(SaveFileTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.clone().run(
                ToolInput::resolved(SaveFileToolInput {
                    paths: vec![
                        PathBuf::from("project/dirty.txt"),
                        PathBuf::from("project/link.txt"),
                    ],
                }),
                event_stream,
                cx,
            )
        });

        cx.run_until_parked();

        let auth = event_rx.expect_authorization().await;
        auth.response
            .send(acp::PermissionOptionId::new("deny"))
            .unwrap();

        let output = task.await.unwrap();
        assert!(
            output.contains("Saved 1 file(s)."),
            "Expected successful save count to remain accurate, got:\n{output}",
        );
        assert!(
            output.contains("Authorization failed (1):"),
            "Expected authorization failure section, got:\n{output}",
        );
        assert!(
            !output.contains("Save failed"),
            "Authorization denials should not be counted as save failures, got:\n{output}",
        );
    }
}
