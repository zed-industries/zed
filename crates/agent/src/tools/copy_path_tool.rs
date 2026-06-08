use super::tool_permissions::{
    authorize_symlink_escapes, canonicalize_worktree_roots, collect_symlink_escapes,
    resolve_creatable_global_skill_descendant_path, resolve_global_skill_descendant_path,
    sensitive_settings_kind,
};
use crate::{
    AgentTool, ToolCallEventStream, ToolInput, ToolPermissionDecision,
    authorize_with_sensitive_settings, decide_permission_for_paths,
};
use action_log::ActionLog;
use agent_client_protocol::schema as acp;
use agent_settings::AgentSettings;
use futures::FutureExt as _;
use gpui::{App, AsyncApp, Entity, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::path::Path;
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

/// Copies a file or directory in the project, and returns confirmation that the copy succeeded.
/// Directory contents will be copied recursively.
///
/// This tool should be used when it's desirable to create a copy of a file or directory without modifying the original.
/// It's much more efficient than doing this by separately reading and then writing the file or directory's contents, so this tool should be preferred over that approach whenever copying is the goal.
/// The only supported paths outside the project are descendants of `~/.agents/skills`, for global agent skills.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CopyPathToolInput {
    /// The source path of the file or directory to copy.
    /// If a directory is specified, its contents will be copied recursively.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can copy the first file by providing a source_path of "directory1/a/something.txt"
    /// </example>
    pub source_path: String,
    /// The destination path where the file or directory should be copied to.
    ///
    /// <example>
    /// To copy "directory1/a/something.txt" to "directory2/b/copy.txt", provide a destination_path of "directory2/b/copy.txt"
    /// </example>
    pub destination_path: String,
}

pub struct CopyPathTool {
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
}

impl CopyPathTool {
    pub fn new(project: Entity<Project>, action_log: Entity<ActionLog>) -> Self {
        Self {
            project,
            action_log,
        }
    }

    async fn mark_copied_project_paths_in_action_log(
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        fs: Arc<dyn fs::Fs>,
        destination_absolute_path: std::path::PathBuf,
        event_stream: &ToolCallEventStream,
        cx: &mut AsyncApp,
    ) -> Result<(), String> {
        let copied_paths = fs::read_dir_items(fs.as_ref(), &destination_absolute_path)
            .await
            .map_err(|error| format!("Reading copied paths: {error}"))?;
        let copied_paths = project.read_with(cx, |project, cx| {
            copied_paths
                .into_iter()
                .filter_map(|(path, is_dir)| {
                    (!is_dir)
                        .then(|| project.find_project_path(path, cx))
                        .flatten()
                })
                .collect::<Vec<_>>()
        });

        for project_path in copied_paths {
            let buffer = futures::select! {
                result = project.update(cx, |project, cx| project.open_buffer(project_path.clone(), cx)).fuse() => result,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Copy cancelled by user".to_string());
                }
            };

            if let Ok(buffer) = buffer {
                action_log.update(cx, |action_log, cx| {
                    action_log.buffer_created_with_current_content(buffer, cx);
                });
            }
        }

        Ok(())
    }
}

impl AgentTool for CopyPathTool {
    type Input = CopyPathToolInput;
    type Output = String;

    const NAME: &'static str = "copy_path";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Move
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> ui::SharedString {
        if let Ok(input) = input {
            let src = MarkdownInlineCode(&input.source_path);
            let dest = MarkdownInlineCode(&input.destination_path);
            format!("Copy {src} to {dest}").into()
        } else {
            "Copy path".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        let action_log = self.action_log.clone();
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|e| e.to_string())?;
            let paths = vec![input.source_path.clone(), input.destination_path.clone()];
            let decision = cx.update(|cx| {
                decide_permission_for_paths(Self::NAME, &paths, &AgentSettings::get_global(cx))
            });
            if let ToolPermissionDecision::Deny(reason) = decision {
                return Err(reason);
            }

            let fs = project.read_with(cx, |project, _cx| project.fs().clone());
            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            let global_source_path =
                resolve_global_skill_descendant_path(Path::new(&input.source_path), fs.as_ref())
                    .await;
            let global_destination_path = resolve_creatable_global_skill_descendant_path(
                Path::new(&input.destination_path),
                fs.as_ref(),
            )
            .await;

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

            let sensitive_kind = sensitive_settings_kind(
                Path::new(&input.source_path),
                &canonical_roots,
                fs.as_ref(),
            )
            .await
            .or(sensitive_settings_kind(
                Path::new(&input.destination_path),
                &canonical_roots,
                fs.as_ref(),
            )
            .await);

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
                    let title = format!("Copy {src} to {dest}");
                    authorize_with_sensitive_settings(
                        sensitive_kind,
                        context,
                        &title,
                        &event_stream,
                        cx,
                    )
                }))
            } else {
                None
            };

            if let Some(authorize) = authorize {
                authorize.await.map_err(|e| e.to_string())?;
            }

            if global_source_path.is_some() || global_destination_path.is_some() {
                let source_path = if let Some(global_source_path) = global_source_path {
                    global_source_path
                } else {
                    project.read_with(cx, |project, cx| {
                        let project_path = project.find_project_path(&input.source_path, cx).ok_or_else(|| {
                            format!("Source path {} was not found in the project.", input.source_path)
                        })?;
                        project.entry_for_path(&project_path, cx).ok_or_else(|| {
                            format!("Source path {} was not found in the project.", input.source_path)
                        })?;
                        project.absolute_path(&project_path, cx).ok_or_else(|| {
                            format!("Source path {} could not be resolved.", input.source_path)
                        })
                    })?
                };

                let destination_path = if let Some(global_destination_path) = global_destination_path
                {
                    global_destination_path
                } else {
                    project.read_with(cx, |project, cx| {
                        let project_path = project.find_project_path(&input.destination_path, cx).ok_or_else(|| {
                            format!(
                                "Destination path {} was outside the project.",
                                input.destination_path
                            )
                        })?;
                        project.absolute_path(&project_path, cx).ok_or_else(|| {
                            format!(
                                "Destination path {} could not be resolved.",
                                input.destination_path
                            )
                        })
                    })?
                };

                futures::select! {
                    result = fs::copy_recursive(
                        fs.as_ref(),
                        &source_path,
                        &destination_path,
                        fs::CopyOptions::default(),
                    ).fuse() => {
                        result.map_err(|e| format!("Copying {} to {}: {e}", input.source_path, input.destination_path))?;
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Copy cancelled by user".to_string());
                    }
                }

                return Ok(format!(
                    "Copied {} to {}",
                    input.source_path, input.destination_path
                ));
            }

            let (copy_task, destination_absolute_path) = project.update(cx, |project, cx| {
                let source_project_path = project.find_project_path(&input.source_path, cx).ok_or_else(|| {
                    format!("Source path {} was not found in the project.", input.source_path)
                })?;
                let entity = project.entry_for_path(&source_project_path, cx).ok_or_else(|| {
                    format!("Source path {} was not found in the project.", input.source_path)
                })?;
                let destination_project_path = project.find_project_path(&input.destination_path, cx).ok_or_else(|| {
                    format!(
                        "Destination path {} was outside the project.",
                        input.destination_path
                    )
                })?;
                let destination_absolute_path = project.absolute_path(&destination_project_path, cx).ok_or_else(|| {
                    format!(
                        "Destination path {} could not be resolved.",
                        input.destination_path
                    )
                })?;
                Result::<_, String>::Ok((
                    project.copy_entry(entity.id, destination_project_path, cx),
                    destination_absolute_path,
                ))
            })?;

            let result = futures::select! {
                result = copy_task.fuse() => result,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Copy cancelled by user".to_string());
                }
            };
            result.map_err(|e| {
                format!(
                    "Copying {} to {}: {e}",
                    input.source_path, input.destination_path
                )
            })?;

            Self::mark_copied_project_paths_in_action_log(
                project,
                action_log,
                fs,
                destination_absolute_path,
                &event_stream,
                cx,
            )
            .await?;

            Ok(format!(
                "Copied {} to {}",
                input.source_path, input.destination_path
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use action_log::ActionLog;
    use fs::Fs as _;
    use gpui::{AppContext as _, TestAppContext};
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
    async fn test_copy_path_tracks_created_file_in_action_log(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root/project"),
            json!({
                "source.txt": "copied content"
            }),
        )
        .await;
        let destination_path = path!("/root/project/destination.txt");
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log.clone()));
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CopyPathToolInput {
                    source_path: path!("/root/project/source.txt").to_string(),
                    destination_path: destination_path.to_string(),
                }),
                event_stream,
                cx,
            )
        });

        let result = task.await;
        assert!(result.is_ok(), "should copy file: {result:?}");
        assert_eq!(
            fs.load(destination_path.as_ref()).await.unwrap(),
            "copied content"
        );
        cx.run_until_parked();
        assert_eq!(
            action_log.read_with(cx, |action_log, cx| action_log.changed_buffers(cx).count()),
            1,
            "copied file should be present in the action log"
        );

        action_log
            .update(cx, |action_log, cx| action_log.reject_all_edits(None, cx))
            .await;
        cx.run_until_parked();

        assert!(
            !fs.is_file(destination_path.as_ref()).await,
            "copied file should be deleted when rejecting action-log edits"
        );
    }

    #[gpui::test]
    async fn test_copy_path_tracks_created_directory_files_in_action_log(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root/project"),
            json!({
                "source": {
                    "one.txt": "one",
                    "nested": {
                        "two.txt": "two"
                    }
                }
            }),
        )
        .await;
        let destination_path = path!("/root/project/destination");
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log.clone()));
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CopyPathToolInput {
                    source_path: path!("/root/project/source").to_string(),
                    destination_path: destination_path.to_string(),
                }),
                event_stream,
                cx,
            )
        });

        let result = task.await;
        assert!(result.is_ok(), "should copy directory: {result:?}");
        assert_eq!(
            fs.load(path!("/root/project/destination/one.txt").as_ref())
                .await
                .unwrap(),
            "one"
        );
        assert_eq!(
            fs.load(path!("/root/project/destination/nested/two.txt").as_ref())
                .await
                .unwrap(),
            "two"
        );
        cx.run_until_parked();
        cx.background_executor.run_until_parked();
        cx.run_until_parked();
        assert_eq!(
            action_log.read_with(cx, |action_log, cx| action_log.changed_buffers(cx).count()),
            2,
            "copied directory files should be present in the action log"
        );

        action_log
            .update(cx, |action_log, cx| action_log.reject_all_edits(None, cx))
            .await;
        cx.run_until_parked();

        assert!(
            !fs.is_file(path!("/root/project/destination/one.txt").as_ref())
                .await,
            "copied top-level file should be deleted when rejecting action-log edits"
        );
        assert!(
            !fs.is_file(path!("/root/project/destination/nested/two.txt").as_ref())
                .await,
            "copied nested file should be deleted when rejecting action-log edits"
        );
    }

    #[gpui::test]
    async fn test_copy_path_global_skill_directory_to_project(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root/project"), json!({})).await;
        let skill_dir = agent_skills::global_skills_dir().join("my-skill");
        fs.insert_tree(&skill_dir, json!({ "SKILL.md": "content" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log));
        let input_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("my-skill")
            .to_string_lossy()
            .into_owned();

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CopyPathToolInput {
                    source_path: input_path,
                    destination_path: path!("/root/project/my-skill").to_string(),
                }),
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("agent skills"),
            "Authorization title should mention agent skills, got: {title}",
        );
        assert!(
            auth.options
                .first_option_of_kind(acp::PermissionOptionKind::AllowAlways)
                .is_none(),
            "agent skills prompt must not offer an \"Always allow\" option: {:?}",
            auth.options,
        );
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .expect("authorization response should send");

        let result = task.await;
        assert!(result.is_ok(), "should copy after approval: {result:?}");
        assert!(fs.is_dir(&skill_dir).await);
        assert_eq!(
            fs.load(path!("/root/project/my-skill/SKILL.md").as_ref())
                .await
                .unwrap(),
            "content"
        );
    }

    #[gpui::test]
    async fn test_copy_path_project_directory_to_global_skill_directory(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root/project"),
            json!({ "exported-skill": { "SKILL.md": "content" } }),
        )
        .await;
        let skills_dir = agent_skills::global_skills_dir();
        fs.create_dir(&skills_dir).await.unwrap();
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log));
        let destination_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("exported-skill")
            .to_string_lossy()
            .into_owned();

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CopyPathToolInput {
                    source_path: path!("/root/project/exported-skill").to_string(),
                    destination_path,
                }),
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        let title = auth.tool_call.fields.title.as_deref().unwrap_or("");
        assert!(
            title.contains("agent skills"),
            "Authorization title should mention agent skills, got: {title}",
        );
        assert!(
            auth.options
                .first_option_of_kind(acp::PermissionOptionKind::AllowAlways)
                .is_none(),
            "agent skills prompt must not offer an \"Always allow\" option: {:?}",
            auth.options,
        );
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .expect("authorization response should send");

        let result = task.await;
        assert!(result.is_ok(), "should copy after approval: {result:?}");
        assert!(
            fs.is_dir(path!("/root/project/exported-skill").as_ref())
                .await
        );
        assert_eq!(
            fs.load(skills_dir.join("exported-skill").join("SKILL.md").as_ref())
                .await
                .unwrap(),
            "content"
        );
    }

    #[gpui::test]
    async fn test_copy_path_symlink_escape_source_requests_authorization(cx: &mut TestAppContext) {
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

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log));

        let input = CopyPathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_copy".into(),
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
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "should succeed after approval: {result:?}");
    }

    #[gpui::test]
    async fn test_copy_path_symlink_escape_denied(cx: &mut TestAppContext) {
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

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log));

        let input = CopyPathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_copy".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        drop(auth);

        let result = task.await;
        assert!(result.is_err(), "should fail when denied");
    }

    #[gpui::test]
    async fn test_copy_path_symlink_escape_confirm_requires_single_approval(
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

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log));

        let input = CopyPathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_copy".into(),
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
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        assert!(
            !matches!(
                event_rx.try_recv(),
                Ok(Ok(crate::ThreadEvent::ToolCallAuthorization(_)))
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
    async fn test_copy_path_symlink_escape_honors_deny_policy(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "copy_path".into(),
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

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(CopyPathTool::new(project, action_log));

        let input = CopyPathToolInput {
            source_path: "project/link_to_external".into(),
            destination_path: "project/external_copy".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx))
            .await;

        assert!(result.is_err(), "Tool should fail when policy denies");
        assert!(
            !matches!(
                event_rx.try_recv(),
                Ok(Ok(crate::ThreadEvent::ToolCallAuthorization(_)))
            ),
            "Deny policy should not emit symlink authorization prompt",
        );
    }
}
