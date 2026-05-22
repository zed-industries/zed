use super::edit_session::{
    EditSession, EditSessionContext, EditSessionMode, EditSessionOutput, EditSessionResult,
    run_session,
};
use crate::{AgentTool, Thread, ToolCallEventStream, ToolInput};
use action_log::ActionLog;
use agent_client_protocol::schema as acp;
use gpui::{App, AsyncApp, Entity, SharedString, Task, WeakEntity};
use language::LanguageRegistry;
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use util::rel_path::RelPath;

const PLANS_DIR: &str = ".zed/plans";

/// Writes (or overwrites) the implementation plan for the current task.
///
/// This tool is only available in Plan Mode. It saves a Markdown plan to
/// `.zed/plans/<slug>.md` inside the first project worktree, opens it in a
/// buffer so the user can edit it inline, and shows the diff in the agent
/// panel. Use it once per turn, providing the full updated plan body each
/// time. On follow-up turns, re-read the existing file first so user edits
/// are preserved.
///
/// The plan file is the source of truth. When the user clicks "Build", a
/// new agent in Write mode will read this file and execute the TODOs.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct WritePlanFileToolInput {
    /// Kebab-case slug used as the filename (without extension). Example: "add-auth-flow".
    /// Must not contain path separators or leading dots. Should be descriptive but short.
    pub slug: String,
    /// The full Markdown body of the plan. Replaces the previous content entirely.
    pub content: String,
}

pub struct WritePlanFileTool {
    project: Entity<Project>,
    session_context: Arc<EditSessionContext>,
}

impl WritePlanFileTool {
    pub fn new(
        project: Entity<Project>,
        thread: WeakEntity<Thread>,
        action_log: Entity<ActionLog>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            project: project.clone(),
            session_context: Arc::new(EditSessionContext::new(
                project,
                thread,
                action_log,
                language_registry,
            )),
        }
    }
}

fn sanitize_slug(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("Plan slug cannot be empty".to_string());
    }
    if trimmed.starts_with('.') {
        return Err("Plan slug cannot start with '.'".to_string());
    }
    if trimmed
        .chars()
        .any(|c| matches!(c, '/' | '\\' | ':' | '\0') || c.is_control())
    {
        return Err(format!(
            "Plan slug '{trimmed}' contains an invalid character. \
             Use only letters, digits, '-', and '_'."
        ));
    }
    // Strip a trailing ".md" if the model included it.
    let without_ext = trimmed
        .strip_suffix(".md")
        .or_else(|| trimmed.strip_suffix(".markdown"))
        .unwrap_or(trimmed);
    if without_ext.is_empty() {
        return Err("Plan slug cannot be just an extension".to_string());
    }
    Ok(without_ext.to_string())
}

impl AgentTool for WritePlanFileTool {
    type Input = WritePlanFileToolInput;
    type Output = EditSessionOutput;

    const NAME: &'static str = "write_plan_file";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Edit
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => match sanitize_slug(&input.slug) {
                Ok(slug) => format!("Write plan `{}/{}.md`", PLANS_DIR, slug).into(),
                Err(_) => "Write plan".into(),
            },
            Err(_) => "Write plan".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx: &mut AsyncApp| {
            let input = input
                .recv()
                .await
                .map_err(|e| EditSessionOutput::error(e.to_string()))?;
            let slug = sanitize_slug(&input.slug).map_err(EditSessionOutput::error)?;

            let worktree_info = cx.update(|cx| {
                project
                    .read(cx)
                    .visible_worktrees(cx)
                    .next()
                    .map(|worktree| {
                        let worktree = worktree.read(cx);
                        (
                            worktree.id(),
                            worktree.root_name().as_unix_str().to_string(),
                        )
                    })
            });

            let (worktree_id, worktree_root_name) = worktree_info.ok_or_else(|| {
                EditSessionOutput::error(
                    "Cannot create plan file: project has no visible worktree.".to_string(),
                )
            })?;

            // Ensure `.zed/plans/` exists in the worktree so EditSession's
            // resolve_path can find the parent directory.
            let plans_dir_rel = RelPath::unix(PLANS_DIR)
                .map_err(|e| EditSessionOutput::error(e.to_string()))?
                .into_arc();
            let plans_dir_project_path = ProjectPath {
                worktree_id,
                path: plans_dir_rel,
            };

            let already_exists = cx.update(|cx| {
                project
                    .read(cx)
                    .entry_for_path(&plans_dir_project_path, cx)
                    .is_some_and(|entry| entry.is_dir())
            });

            if !already_exists {
                let create_task = project.update(cx, |project, cx| {
                    project.create_entry(plans_dir_project_path.clone(), true, cx)
                });
                create_task
                    .await
                    .map_err(|e| EditSessionOutput::error(e.to_string()))?;
            }

            // Compose the worktree-prefixed path the EditSession API expects.
            let plan_file_path =
                PathBuf::from(format!("{}/{}/{}.md", worktree_root_name, PLANS_DIR, slug));

            let session_context = self.session_context.clone();
            let session_result = match EditSession::new(
                plan_file_path,
                EditSessionMode::AgentInternalWrite,
                Self::NAME,
                session_context,
                &event_stream,
                cx,
            )
            .await
            {
                Ok(mut session) => match session.finalize_write(&input.content, cx).await {
                    Ok(()) => EditSessionResult::Completed(session),
                    Err(error) => EditSessionResult::Failed {
                        error,
                        session: Some(session),
                    },
                },
                Err(error) => EditSessionResult::Failed {
                    error,
                    session: None,
                },
            };

            run_session(session_result, cx).await
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        self.session_context.replay_output(output, event_stream, cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_sanitization_accepts_kebab() {
        assert_eq!(sanitize_slug("add-auth-flow").unwrap(), "add-auth-flow");
    }

    #[test]
    fn slug_sanitization_strips_md_suffix() {
        assert_eq!(sanitize_slug("add-auth-flow.md").unwrap(), "add-auth-flow");
        assert_eq!(
            sanitize_slug("add-auth-flow.markdown").unwrap(),
            "add-auth-flow"
        );
    }

    #[test]
    fn slug_sanitization_rejects_path_separators() {
        assert!(sanitize_slug("foo/bar").is_err());
        assert!(sanitize_slug("foo\\bar").is_err());
        assert!(sanitize_slug("../etc/passwd").is_err());
    }

    #[test]
    fn slug_sanitization_rejects_leading_dot() {
        assert!(sanitize_slug(".hidden").is_err());
    }

    #[test]
    fn slug_sanitization_rejects_empty() {
        assert!(sanitize_slug("").is_err());
        assert!(sanitize_slug("   ").is_err());
        assert!(sanitize_slug(".md").is_err());
    }

    #[test]
    fn slug_sanitization_trims_whitespace() {
        assert_eq!(sanitize_slug("  plan-one  ").unwrap(), "plan-one");
    }
}
