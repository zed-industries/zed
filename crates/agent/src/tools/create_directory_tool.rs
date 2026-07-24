use super::tool_permissions::{
    authorize_symlink_access, canonicalize_worktree_roots, detect_symlink_escape,
    resolve_creatable_global_skill_path, sensitive_settings_kind,
};
use agent_client_protocol::schema::v1 as acp;
use agent_settings::AgentSettings;
use futures::FutureExt as _;
use gpui::{App, AppContext as _, AsyncApp, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

use crate::{
    AgentTool, ToolCallEventStream, ToolInput, ToolPermissionDecision,
    authorize_with_sensitive_settings, decide_permission_for_path,
};
use std::path::{Path, PathBuf};

/// Creates a new directory at the specified path, and all necessary parent directories. Returns confirmation that the directory was created.
///
/// Use this whenever you need to create new directories. Paths inside the project are created directly.
///
/// This tool can also create a directory **outside** the project. When agent terminal commands are sandboxed, doing so grants those commands write access to exactly that new directory — so, rather than requesting write access to a broad existing parent (e.g. your home directory) just to create something inside it, create the specific directory here first and then write into it. The only other supported path outside the project is `~/.agents/skills` or a descendant, for global agent skills.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateDirectoryToolInput {
    /// The path of the new directory.
    ///
    /// <example>
    /// If the project has the following structure:
    ///
    /// - directory1/
    /// - directory2/
    ///
    /// You can create a new directory by providing a path of "directory1/new_directory"
    /// </example>
    ///
    /// <example>
    /// To create a global agent skill directory, you may provide a path under `~/.agents/skills`, such as `~/.agents/skills/my-skill`.
    /// </example>
    pub path: String,

    /// Justification for creating a directory **outside** the project, shown to
    /// the user (attributed to you) in the approval prompt that grants sandboxed
    /// terminal commands write access to it. Required only for out-of-project
    /// paths; ignored for paths inside the project or the global skills dir.
    #[serde(default)]
    pub reason: Option<String>,
}

pub struct CreateDirectoryTool {
    project: Entity<Project>,
}

impl CreateDirectoryTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for CreateDirectoryTool {
    type Input = CreateDirectoryToolInput;
    type Output = String;

    const NAME: &'static str = "create_directory";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Create directory {}", MarkdownInlineCode(&input.path)).into()
        } else {
            "Create directory".into()
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
            let input = input.recv().await.map_err(|e| e.to_string())?;

            let fs = project.read_with(cx, |project, _cx| project.fs().clone());

            // Resolve where this directory lives. The global agent-skills dir is a
            // special case allowed outside the project; anything else outside the
            // project is handled as a narrow sandbox write grant below.
            let global_skill_directory =
                resolve_creatable_global_skill_path(Path::new(&input.path), fs.as_ref()).await;
            let in_project = project.read_with(cx, |project, cx| {
                project.find_project_path(&input.path, cx).is_some()
            });

            // A path outside the project (and not the global skills dir) can only
            // be created as a narrow sandbox write grant: create the directory and
            // grant sandboxed terminal commands write access to exactly it. The
            // sandbox approval prompt — which shows the real, canonicalized target
            // — fully replaces the normal permission and symlink-escape prompts
            // here.
            if global_skill_directory.is_none() && !in_project {
                return create_out_of_project_directory(&project, &input, &event_stream, cx).await;
            }

            let decision = cx.update(|cx| {
                decide_permission_for_path(Self::NAME, &input.path, AgentSettings::get_global(cx))
            });

            if let ToolPermissionDecision::Deny(reason) = decision {
                return Err(reason);
            }

            let destination_path: Arc<str> = input.path.as_str().into();

            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            let symlink_escape_target = project.read_with(cx, |project, cx| {
                detect_symlink_escape(project, &input.path, &canonical_roots, cx)
                    .map(|(_, target)| target)
            });

            let sensitive_kind =
                sensitive_settings_kind(Path::new(&input.path), &canonical_roots, fs.as_ref())
                    .await;

            let decision =
                if matches!(decision, ToolPermissionDecision::Allow) && sensitive_kind.is_some() {
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
                        &input.path,
                        &canonical_target,
                        &event_stream,
                        cx,
                    )
                }))
            } else {
                match decision {
                    ToolPermissionDecision::Allow => None,
                    ToolPermissionDecision::Confirm => Some(cx.update(|cx| {
                        let title = format!("Create directory {}", MarkdownInlineCode(&input.path));
                        let context =
                            crate::ToolPermissionContext::new(Self::NAME, vec![input.path.clone()]);
                        authorize_with_sensitive_settings(
                            sensitive_kind,
                            context,
                            &title,
                            &event_stream,
                            cx,
                        )
                    })),
                    ToolPermissionDecision::Deny(_) => None,
                }
            };

            if let Some(authorize) = authorize {
                authorize.await.map_err(|e| e.to_string())?;
            }

            if let Some(global_skill_directory) = global_skill_directory {
                futures::select! {
                    result = fs.create_dir(&global_skill_directory).fuse() => {
                        result.map_err(|e| format!("Creating directory {destination_path}: {e}"))?;
                    }
                    _ = event_stream.cancelled_by_user().fuse() => {
                        return Err("Create directory cancelled by user".to_string());
                    }
                }

                return Ok(format!("Created directory {destination_path}"));
            }

            let create_entry = project.update(cx, |project, cx| {
                match project.find_project_path(&input.path, cx) {
                    Some(project_path) => Ok(project.create_entry(project_path, true, cx)),
                    None => Err("Path to create was outside the project".to_string()),
                }
            })?;

            futures::select! {
                result = create_entry.fuse() => {
                    result.map_err(|e| format!("Creating directory {destination_path}: {e}"))?;
                }
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Create directory cancelled by user".to_string());
                }
            }

            Ok(format!("Created directory {destination_path}"))
        })
    }
}

/// Create a directory that lives **outside** the project by granting sandboxed
/// terminal commands write access to exactly it.
///
/// The directory is created (Linux: eagerly, pinning the inode; macOS: after
/// approval) and the user is shown the real, canonicalized target in the sandbox
/// approval prompt — which is what defends against a concurrent symlink swap: the
/// grant is always against the inode/path the user actually saw. On denial, only
/// the directories we created are removed.
async fn create_out_of_project_directory(
    project: &Entity<Project>,
    input: &CreateDirectoryToolInput,
    event_stream: &ToolCallEventStream,
    cx: &mut AsyncApp,
) -> Result<String, String> {
    // Narrowing a grant to a brand-new directory only makes sense when the
    // project's terminal commands are sandboxed, and only on platforms that can
    // grant a not-yet-existing directory. Otherwise keep the historical
    // "outside the project" rejection.
    let sandboxing = project.read_with(cx, |project, cx| {
        crate::sandboxing::sandboxing_enabled_for_project(project, cx)
    });
    let platform_supported = cfg!(any(target_os = "linux", target_os = "macos"));
    if !sandboxing || !platform_supported {
        return Err("Path to create was outside the project".to_string());
    }

    let Some(reason) = input
        .reason
        .as_deref()
        .map(str::trim)
        .filter(|reason| !reason.is_empty())
    else {
        return Err(
            "Creating a directory outside the project grants sandboxed terminal commands write \
             access to it, so a `reason` is required: briefly justify why the directory is needed, \
             then try again."
                .to_string(),
        );
    };
    let reason = reason.to_string();

    let absolute = resolve_absolute_path(project, &input.path, cx)
        .ok_or_else(|| format!("Couldn't resolve `{}` to an absolute path.", input.path))?;

    let prepared = cx
        .background_spawn(async move { sandbox::GrantableWriteDir::prepare(&absolute) })
        .await
        .map_err(|error| format!("Creating directory {}: {error}", input.path))?;

    let canonical = prepared.canonical_path().to_path_buf();
    let request = crate::sandboxing::SandboxRequest {
        write_paths: vec![canonical.clone()],
        ..Default::default()
    };

    let approve = cx.update(|cx| event_stream.authorize_sandbox(request, reason, cx));
    match approve.await {
        Ok(()) => {
            let display = canonical.display().to_string();
            cx.background_spawn(async move { prepared.finalize() })
                .await
                .map_err(|error| format!("Creating directory {display}: {error}"))?;
            Ok(format!("Created directory {display}"))
        }
        Err(error) => {
            // Roll back exactly what we created; leave the user no litter.
            cx.background_spawn(async move { prepared.discard() }).await;
            Err(format!("Create directory cancelled: {error}"))
        }
    }
}

/// Resolve a model-provided path to an absolute, lexically-normalized path.
/// Relative paths are joined onto the first worktree root.
fn resolve_absolute_path(
    project: &Entity<Project>,
    raw: &str,
    cx: &mut AsyncApp,
) -> Option<PathBuf> {
    let path = Path::new(raw);
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        let base = project.read_with(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        })?;
        base.join(path)
    };
    util::paths::normalize_lexically(&absolute).ok()
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
    async fn test_create_directory_allows_global_skill_directory(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root/project"), json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let tool = Arc::new(CreateDirectoryTool::new(project));
        let input_path = PathBuf::from("~")
            .join(".agents")
            .join("skills")
            .join("my-skill")
            .to_string_lossy()
            .into_owned();
        let created_path = agent_skills::global_skills_dir().join("my-skill");

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CreateDirectoryToolInput {
                    path: input_path,
                    reason: None,
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
        assert!(
            result.is_ok(),
            "Tool should create global skill directory: {result:?}"
        );
        assert!(fs.is_dir(&created_path).await);
    }

    #[gpui::test]
    async fn test_create_directory_rejects_other_global_paths(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root/project"), json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let tool = Arc::new(CreateDirectoryTool::new(project));
        let outside_path = agent_skills::global_skills_dir()
            .parent()
            .expect("global skills directory should have a parent")
            .join("not-skills");

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| {
                tool.run(
                    ToolInput::resolved(CreateDirectoryToolInput {
                        path: outside_path.to_string_lossy().into_owned(),
                        reason: None,
                    }),
                    event_stream,
                    cx,
                )
            })
            .await;

        assert!(
            result.is_err(),
            "Tool should reject paths outside the project and global skills directory"
        );
        assert!(!fs.is_dir(&outside_path).await);
        assert!(
            !matches!(
                event_rx.try_recv(),
                Ok(Ok(crate::ThreadEvent::ToolCallAuthorization(_)))
            ),
            "Non-skill global path should not emit an agent-skills authorization prompt",
        );
    }

    #[gpui::test]
    async fn test_create_directory_symlink_escape_requests_authorization(cx: &mut TestAppContext) {
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

        let tool = Arc::new(CreateDirectoryTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CreateDirectoryToolInput {
                    path: "project/link_to_external".into(),
                    reason: None,
                }),
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
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("allow"),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(
            result.is_ok(),
            "Tool should succeed after authorization: {result:?}"
        );
    }

    #[gpui::test]
    async fn test_create_directory_symlink_escape_denied(cx: &mut TestAppContext) {
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

        let tool = Arc::new(CreateDirectoryTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CreateDirectoryToolInput {
                    path: "project/link_to_external".into(),
                    reason: None,
                }),
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
    async fn test_create_directory_symlink_escape_confirm_requires_single_approval(
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

        let tool = Arc::new(CreateDirectoryTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CreateDirectoryToolInput {
                    path: "project/link_to_external".into(),
                    reason: None,
                }),
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
    async fn test_create_directory_symlink_escape_honors_deny_policy(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let mut settings = AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "create_directory".into(),
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

        let tool = Arc::new(CreateDirectoryTool::new(project));

        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| {
                tool.run(
                    ToolInput::resolved(CreateDirectoryToolInput {
                        path: "project/link_to_external".into(),
                        reason: None,
                    }),
                    event_stream,
                    cx,
                )
            })
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

    /// Out-of-project creation goes through the sandbox write-grant prompt and,
    /// on approval, creates the *specific* new directory (not its broad parent).
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[gpui::test]
    #[ignore]
    async fn test_create_directory_out_of_project_creates_and_grants(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "project": { "src": {} } }))
            .await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        // The sandbox create path operates on the *real* filesystem, so use a
        // real directory outside the (fake) project.
        let scratch = tempfile::tempdir().unwrap();
        let target = scratch.path().join("new_grant_dir");
        assert!(!target.exists());

        let tool = Arc::new(CreateDirectoryTool::new(project));
        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let path_input = target.to_string_lossy().into_owned();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CreateDirectoryToolInput {
                    path: path_input,
                    reason: Some("scratch space for the build".into()),
                }),
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        let details = acp_thread::sandbox_authorization_details_from_meta(&auth.tool_call.meta)
            .expect("out-of-project create should request a sandbox write grant");
        // The grant is for exactly the new directory, not its parent.
        assert_eq!(
            details.write_paths,
            vec![scratch.path().canonicalize().unwrap().join("new_grant_dir")]
        );

        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new(acp_thread::SandboxPermission::AllowThread.as_id()),
                acp::PermissionOptionKind::AllowAlways,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_ok(), "expected success, got {result:?}");
        assert!(
            target.is_dir(),
            "the new directory should have been created"
        );
    }

    /// Denying the grant removes the directory we eagerly created, leaving no
    /// trace on the filesystem.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[gpui::test]
    #[ignore]
    async fn test_create_directory_out_of_project_denied_cleans_up(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "project": { "src": {} } }))
            .await;
        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let scratch = tempfile::tempdir().unwrap();
        let target = scratch.path().join("denied_dir");

        let tool = Arc::new(CreateDirectoryTool::new(project));
        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let path_input = target.to_string_lossy().into_owned();
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(CreateDirectoryToolInput {
                    path: path_input,
                    reason: Some("scratch space".into()),
                }),
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new(acp_thread::SandboxPermission::Deny.as_id()),
                acp::PermissionOptionKind::RejectOnce,
            ))
            .unwrap();

        let result = task.await;
        assert!(result.is_err(), "denied create should fail");
        assert!(
            !target.exists(),
            "denied create should leave no directory behind"
        );
    }
}
