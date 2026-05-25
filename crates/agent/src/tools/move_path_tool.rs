use super::tool_permissions::{
    authorize_symlink_escapes, build_global_skill_project_path, canonicalize_worktree_roots,
    collect_symlink_escapes, ensure_global_skills_worktree, expand_user_home,
    resolve_global_skill_creation_target, resolve_global_skill_path, sensitive_settings_kind,
};
use crate::{
    AgentTool, ToolCallEventStream, ToolInput, ToolPermissionDecision,
    authorize_with_sensitive_settings, decide_permission_for_paths,
};
use agent_client_protocol::schema as acp;
use agent_settings::AgentSettings;
use fs::{RemoveOptions, RenameOptions, read_dir_items};
use futures::FutureExt as _;
use gpui::{App, Entity, SharedString, Task};
use project::{Project, ProjectEntryId, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{path::Path, sync::Arc};
use util::{markdown::MarkdownInlineCode, paths::PathStyle, rel_path::RelPath};

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

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Move
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
                .map_err(|e| e.to_string())?;
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
                    let title = format!("Move {src} to {dest}");
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

            let is_local = project.read_with(cx, |project, _cx| project.is_local());
            let expanded_source = expand_user_home(&input.source_path);
            let expanded_dest = expand_user_home(&input.destination_path);
            let source_skill_target = resolve_global_skill_path(&expanded_source, fs.as_ref()).await;
            let dest_skill_target =
                resolve_global_skill_creation_target(&expanded_dest, fs.as_ref()).await;

            if !is_local
                && let (Some(source), Some(destination)) = (
                    source_skill_target.as_ref(),
                    dest_skill_target.as_ref(),
                )
            {
                futures::select! {
                    result = fs.rename(
                        source,
                        destination,
                        RenameOptions {
                            create_parents: true,
                            ..Default::default()
                        },
                    ).fuse() => {
                        result.map_err(|e| {
                            format!(
                                "Moving {} to {}: {e}",
                                input.source_path, input.destination_path
                            )
                        })?;
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Move cancelled by user".to_string());
                    }
                }
                return Ok(format!(
                    "Moved {} to {}",
                    input.source_path, input.destination_path
                ));
            }

            if !is_local
                && let Some(source) = source_skill_target.as_ref()
                && dest_skill_target.is_none()
            {
                let move_task =
                    move_global_skill_to_project(source, &input.destination_path, &project, &fs, cx)
                        .fuse();
                futures::pin_mut!(move_task);
                futures::select! {
                    result = move_task => {
                        result.map_err(|e| {
                            format!(
                                "Moving {} to {}: {e}",
                                input.source_path, input.destination_path
                            )
                        })?;
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Move cancelled by user".to_string());
                    }
                }
                return Ok(format!(
                    "Moved {} to {}",
                    input.source_path, input.destination_path
                ));
            }

            let source_in_skills = is_local && source_skill_target.is_some();
            let dest_in_skills = is_local && dest_skill_target.is_some();

            // Kept in scope past `rename_task` because `WorktreeStore` only
            // holds invisible worktrees by `Weak`.
            let global_skills_worktree = if source_in_skills || dest_in_skills {
                Some(ensure_global_skills_worktree(&project, cx).await?)
            } else {
                None
            };
            let source_worktree = global_skills_worktree.as_ref().filter(|_| source_in_skills);
            let dest_worktree = global_skills_worktree.as_ref().filter(|_| dest_in_skills);

            let rename_task = project.update(cx, |project, cx| {
                let source_entry_id = if let Some(worktree) = source_worktree {
                    let project_path =
                        build_global_skill_project_path(worktree, &expanded_source, cx)?;
                    worktree
                        .read(cx)
                        .entry_for_path(&project_path.path)
                        .map(|entry| entry.id)
                        .ok_or_else(|| {
                            format!(
                                "Source path {} was not found in the project.",
                                input.source_path
                            )
                        })?
                } else {
                    resolve_source_entry_id(project, &input.source_path, cx)?
                };

                let destination_path = if let Some(worktree) = dest_worktree {
                    build_global_skill_project_path(worktree, &expanded_dest, cx)?
                } else {
                    project
                        .find_project_path(&input.destination_path, cx)
                        .ok_or_else(|| {
                            format!(
                                "Destination path {} was outside the project.",
                                input.destination_path
                            )
                        })?
                };

                Ok::<_, String>(project.rename_entry(source_entry_id, destination_path, cx))
            })?;

            futures::select! {
                result = rename_task.fuse() => result.map_err(|e| format!("Moving {} to {}: {e}", input.source_path, input.destination_path))?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Move cancelled by user".to_string());
                }
            };
            drop(global_skills_worktree);
            Ok(format!(
                "Moved {} to {}",
                input.source_path, input.destination_path
            ))
        })
    }
}

fn resolve_source_entry_id(
    project: &Project,
    source_path: &str,
    cx: &App,
) -> Result<ProjectEntryId, String> {
    let project_path: ProjectPath = project
        .find_project_path(source_path, cx)
        .ok_or_else(|| format!("Source path {source_path} was not found in the project."))?;
    project
        .entry_for_path(&project_path, cx)
        .map(|entry| entry.id)
        .ok_or_else(|| format!("Source path {source_path} was not found in the project."))
}

async fn move_global_skill_to_project(
    source: &Path,
    destination_path: &str,
    project: &Entity<Project>,
    fs: &Arc<dyn fs::Fs>,
    cx: &mut gpui::AsyncApp,
) -> Result<(), String> {
    let source_metadata = fs
        .metadata(source)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Source path {} was not found.", source.display()))?;

    let (destination_project_path, destination_worktree) =
        project.read_with(cx, |project, cx| -> Result<_, String> {
            let destination_project_path = project
                .find_project_path(destination_path, cx)
                .ok_or_else(|| {
                    format!("Destination path {destination_path} was outside the project.")
                })?;
            if project
                .entry_for_path(&destination_project_path, cx)
                .is_some()
            {
                return Err(format!(
                    "Destination path {destination_path} already exists."
                ));
            }
            let destination_worktree = project
                .worktree_for_id(destination_project_path.worktree_id, cx)
                .ok_or_else(|| format!("No worktree for path {destination_path}"))?;
            Ok((destination_project_path, destination_worktree))
        })?;

    let items = read_dir_items(fs.as_ref(), source)
        .await
        .map_err(|e| e.to_string())?;

    for (item_path, is_directory) in items {
        let relative_path = item_path.strip_prefix(source).map_err(|_| {
            format!(
                "Could not resolve {} relative to {}",
                item_path.display(),
                source.display()
            )
        })?;
        let path = if relative_path.as_os_str().is_empty() {
            destination_project_path.path.clone()
        } else {
            let relative_path = RelPath::new(relative_path, PathStyle::local())
                .map_err(|e| format!("Invalid source path {}: {e}", item_path.display()))?;
            destination_project_path.path.join(relative_path.as_ref())
        };

        let content = if is_directory {
            None
        } else {
            Some(fs.load_bytes(&item_path).await.map_err(|e| e.to_string())?)
        };

        destination_worktree
            .update(cx, |worktree, cx| {
                worktree.create_entry(path, is_directory, content, cx)
            })
            .await
            .map_err(|e| e.to_string())?;
    }

    if source_metadata.is_dir {
        fs.remove_dir(
            source,
            RemoveOptions {
                recursive: true,
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.to_string())?;
    } else {
        fs.remove_file(source, RemoveOptions::default())
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
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
                event_rx.try_recv(),
                Ok(Ok(crate::ThreadEvent::ToolCallAuthorization(_)))
            ),
            "Deny policy should not emit symlink authorization prompt",
        );
    }

    /// Moving a project file into the global skills directory must succeed
    /// (previously failed with "parent directory doesn't exist" because
    /// `find_project_path` can't see the hidden skills worktree).
    #[gpui::test]
    async fn test_move_path_project_to_global_skill(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "draft-skill": {
                        "SKILL.md": "---\nname: draft-skill\ndescription: in-progress skill\n---\nbody",
                    },
                },
            }),
        )
        .await;
        let skills_dir = agent_skills::global_skills_dir();
        fs.create_dir(&skills_dir).await.unwrap();
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();
        let tool = Arc::new(MovePathTool::new(project));

        let dest = skills_dir
            .join("draft-skill")
            .to_string_lossy()
            .into_owned();
        let input = MovePathToolInput {
            source_path: "project/draft-skill".into(),
            destination_path: dest.clone(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        assert!(
            auth.tool_call
                .fields
                .title
                .as_deref()
                .is_some_and(|t| t.ends_with("(agent skills)")),
            "got: {:?}",
            auth.tool_call.fields.title,
        );
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "move should succeed: {result:?}");
        assert!(fs.is_dir(&skills_dir.join("draft-skill")).await);
        assert!(
            fs.is_file(&skills_dir.join("draft-skill").join("SKILL.md"))
                .await
        );
        assert!(
            !fs.is_dir(Path::new(path!("/root/project/draft-skill")))
                .await
        );
    }

    #[gpui::test]
    async fn test_move_path_global_skill_to_project(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "project": {} }))
            .await;
        let skills_dir = agent_skills::global_skills_dir();
        fs.create_dir(&skills_dir).await.unwrap();
        fs.create_dir(&skills_dir.join("shared-skill"))
            .await
            .unwrap();
        fs.insert_file(
            skills_dir.join("shared-skill").join("SKILL.md"),
            b"---\nname: shared-skill\ndescription: promote me\n---\nbody".to_vec(),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();
        let tool = Arc::new(MovePathTool::new(project));

        let source = skills_dir
            .join("shared-skill")
            .to_string_lossy()
            .into_owned();
        let input = MovePathToolInput {
            source_path: source,
            destination_path: "project/shared-skill".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        assert!(
            auth.tool_call
                .fields
                .title
                .as_deref()
                .is_some_and(|t| t.ends_with("(agent skills)")),
            "got: {:?}",
            auth.tool_call.fields.title,
        );
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "move should succeed: {result:?}");
        assert!(
            fs.is_dir(Path::new(path!("/root/project/shared-skill")))
                .await
        );
        assert!(
            fs.is_file(Path::new(path!("/root/project/shared-skill/SKILL.md")))
                .await
        );
        assert!(!fs.is_dir(&skills_dir.join("shared-skill")).await);
    }

    #[gpui::test]
    async fn test_move_path_global_skill_to_project_in_remote_project(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "zed": {} })).await;
        let skills_dir = agent_skills::global_skills_dir();
        fs.create_dir(&skills_dir).await.unwrap();
        fs.create_dir(&skills_dir.join("test-skill")).await.unwrap();
        fs.insert_file(
            skills_dir.join("test-skill").join("SKILL.md"),
            b"---\nname: test-skill\ndescription: move me\n---\nbody".to_vec(),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root/zed").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.mark_as_collab_for_testing();
        });
        cx.executor().run_until_parked();
        let tool = Arc::new(MovePathTool::new(project));

        let input = MovePathToolInput {
            source_path: "~/.agents/skills/test-skill".into(),
            destination_path: "zed/.agents/skills/test-skill".into(),
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        assert!(
            auth.tool_call
                .fields
                .title
                .as_deref()
                .is_some_and(|title| title.ends_with("(agent skills)")),
            "got: {:?}",
            auth.tool_call.fields.title,
        );
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "move should succeed: {result:?}");
        assert!(
            fs.is_dir(Path::new(path!("/root/zed/.agents/skills/test-skill")))
                .await
        );
        assert!(
            fs.is_file(Path::new(path!(
                "/root/zed/.agents/skills/test-skill/SKILL.md"
            )))
            .await
        );
        assert!(!fs.is_dir(&skills_dir.join("test-skill")).await);
    }

    /// Both source and destination resolve through the hidden worktree.
    #[gpui::test]
    async fn test_move_path_rename_within_global_skills(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "project": {} }))
            .await;
        let skills_dir = agent_skills::global_skills_dir();
        fs.create_dir(&skills_dir).await.unwrap();
        fs.create_dir(&skills_dir.join("old-name")).await.unwrap();
        fs.insert_file(
            skills_dir.join("old-name").join("SKILL.md"),
            b"---\nname: old-name\ndescription: rename me\n---\nbody".to_vec(),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();
        let tool = Arc::new(MovePathTool::new(project));

        let source = skills_dir.join("old-name").to_string_lossy().into_owned();
        let destination = skills_dir.join("new-name").to_string_lossy().into_owned();
        let input = MovePathToolInput {
            source_path: source,
            destination_path: destination,
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "rename should succeed: {result:?}");
        assert!(fs.is_dir(&skills_dir.join("new-name")).await);
        assert!(!fs.is_dir(&skills_dir.join("old-name")).await);
    }

    #[gpui::test]
    async fn test_move_path_rename_within_global_skills_in_remote_project(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "project": {} }))
            .await;
        let skills_dir = agent_skills::global_skills_dir();
        fs.create_dir(&skills_dir).await.unwrap();
        fs.create_dir(&skills_dir.join("old-name")).await.unwrap();
        fs.insert_file(
            skills_dir.join("old-name").join("SKILL.md"),
            b"---\nname: old-name\ndescription: rename me\n---\nbody".to_vec(),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.mark_as_collab_for_testing();
        });
        cx.executor().run_until_parked();
        let tool = Arc::new(MovePathTool::new(project));

        let source = skills_dir.join("old-name").to_string_lossy().into_owned();
        let destination = skills_dir.join("new-name").to_string_lossy().into_owned();
        let input = MovePathToolInput {
            source_path: source,
            destination_path: destination,
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        assert!(
            auth.tool_call
                .fields
                .title
                .as_deref()
                .is_some_and(|t| t.ends_with("(agent skills)")),
            "got: {:?}",
            auth.tool_call.fields.title,
        );
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "rename should succeed: {result:?}");
        assert!(fs.is_dir(&skills_dir.join("new-name")).await);
        assert!(!fs.is_dir(&skills_dir.join("old-name")).await);
    }

    /// Cross-host project-to-global moves stay rejected on remote/collab
    /// projects rather than pretending a filesystem rename can cross machines.
    #[gpui::test]
    async fn test_move_path_skip_skills_carve_out_for_remote_projects(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    "draft-skill": { "SKILL.md": "body" },
                },
            }),
        )
        .await;
        let skills_dir = agent_skills::global_skills_dir();
        fs.create_dir(&skills_dir).await.unwrap();
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.mark_as_collab_for_testing();
        });
        cx.executor().run_until_parked();
        let tool = Arc::new(MovePathTool::new(project));

        let dest = skills_dir
            .join("draft-skill")
            .to_string_lossy()
            .into_owned();
        let input = MovePathToolInput {
            source_path: "project/draft-skill".into(),
            destination_path: dest,
        };

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| tool.run(ToolInput::resolved(input), event_stream, cx));

        let auth = event_rx.expect_authorization().await;
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(
            result
                .as_ref()
                .err()
                .is_some_and(|e| e.contains("outside the project")),
            "got: {result:?}",
        );
        assert!(!fs.is_dir(&skills_dir.join("draft-skill")).await);
        assert!(
            fs.is_dir(Path::new(path!("/root/project/draft-skill")))
                .await
        );
    }
}
