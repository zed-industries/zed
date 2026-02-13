use super::tool_permissions::{
    ResolvedProjectPath, authorize_symlink_access, canonicalize_worktree_roots,
    resolve_project_path,
};
use crate::AgentTool;
use agent_client_protocol::ToolKind;
use anyhow::{Context as _, Result};
use futures::FutureExt as _;
use gpui::{App, AppContext as _, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use util::markdown::MarkdownEscaped;

/// This tool opens a file or URL with the default application associated with it on the user's operating system:
///
/// - On macOS, it's equivalent to the `open` command
/// - On Windows, it's equivalent to `start`
/// - On Linux, it uses something like `xdg-open`, `gio open`, `gnome-open`, `kde-open`, `wslview` as appropriate
///
/// For example, it can open a web browser with a URL, open a PDF file with the default PDF viewer, etc.
///
/// You MUST ONLY use this tool when the user has explicitly requested opening something. You MUST NEVER assume that the user would like for you to use this tool.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct OpenToolInput {
    /// The path or URL to open with the default application.
    path_or_url: String,
}

pub struct OpenTool {
    project: Entity<Project>,
}

impl OpenTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for OpenTool {
    type Input = OpenToolInput;
    type Output = String;

    const NAME: &'static str = "open";

    fn kind() -> ToolKind {
        ToolKind::Execute
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Open `{}`", MarkdownEscaped(&input.path_or_url)).into()
        } else {
            "Open file or URL".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: crate::ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        // If path_or_url turns out to be a path in the project, make it absolute.
        let abs_path = to_absolute_path(&input.path_or_url, self.project.clone(), cx);
        let initial_title = self.initial_title(Ok(input.clone()), cx);

        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let fs = project.read_with(cx, |project, _cx| project.fs().clone());
            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            // Symlink escape authorization replaces (rather than supplements)
            // the normal tool-permission prompt. The symlink prompt already
            // requires explicit user approval with the canonical target shown,
            // which is strictly more security-relevant than a generic confirm.
            let symlink_escape = project.read_with(cx, |project, cx| {
                match resolve_project_path(
                    project,
                    PathBuf::from(&input.path_or_url),
                    &canonical_roots,
                    cx,
                ) {
                    Ok(ResolvedProjectPath::SymlinkEscape {
                        canonical_target, ..
                    }) => Some(canonical_target),
                    _ => None,
                }
            });

            let authorize = if let Some(canonical_target) = symlink_escape {
                cx.update(|cx| {
                    authorize_symlink_access(
                        Self::NAME,
                        &input.path_or_url,
                        &canonical_target,
                        &event_stream,
                        cx,
                    )
                })
            } else {
                cx.update(|cx| {
                    let context = crate::ToolPermissionContext::new(
                        Self::NAME,
                        vec![input.path_or_url.clone()],
                    );
                    event_stream.authorize(initial_title, context, cx)
                })
            };

            futures::select! {
                result = authorize.fuse() => result?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    anyhow::bail!("Open cancelled by user");
                }
            }

            let path_or_url = input.path_or_url.clone();
            cx.background_spawn(async move {
                match abs_path {
                    Some(path) => open::that(path),
                    None => open::that(path_or_url),
                }
                .context("Failed to open URL or file path")
            })
            .await?;

            Ok(format!("Successfully opened {}", input.path_or_url))
        })
    }
}

fn to_absolute_path(
    potential_path: &str,
    project: Entity<Project>,
    cx: &mut App,
) -> Option<PathBuf> {
    let project = project.read(cx);
    project
        .find_project_path(PathBuf::from(potential_path), cx)
        .and_then(|project_path| project.absolute_path(&project_path, cx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use std::path::Path;
    use tempfile::TempDir;

    #[gpui::test]
    async fn test_to_absolute_path(cx: &mut TestAppContext) {
        init_test(cx);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_string_lossy().into_owned();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            &temp_path,
            serde_json::json!({
                "src": {
                    "main.rs": "fn main() {}",
                    "lib.rs": "pub fn lib_fn() {}"
                },
                "docs": {
                    "readme.md": "# Project Documentation"
                }
            }),
        )
        .await;

        // Use the temp_path as the root directory, not just its filename
        let project = Project::test(fs.clone(), [temp_dir.path()], cx).await;

        // Test cases where the function should return Some
        cx.update(|cx| {
            // Project-relative paths should return Some
            // Create paths using the last segment of the temp path to simulate a project-relative path
            let root_dir_name = Path::new(&temp_path)
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("temp"))
                .to_string_lossy();

            assert!(
                to_absolute_path(&format!("{root_dir_name}/src/main.rs"), project.clone(), cx)
                    .is_some(),
                "Failed to resolve main.rs path"
            );

            assert!(
                to_absolute_path(
                    &format!("{root_dir_name}/docs/readme.md",),
                    project.clone(),
                    cx,
                )
                .is_some(),
                "Failed to resolve readme.md path"
            );

            // External URL should return None
            let result = to_absolute_path("https://example.com", project.clone(), cx);
            assert_eq!(result, None, "External URLs should return None");

            // Path outside project
            let result = to_absolute_path("../invalid/path", project.clone(), cx);
            assert_eq!(result, None, "Paths outside the project should return None");
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }
}
