use super::tool_permissions::canonicalize_worktree_roots;
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use futures::StreamExt;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;

/// Reads a file or lists a directory within an agent skill.
///
/// Skills are located in:
/// - Global: `~/.config/zed/skills/<skill-name>/`
/// - Worktree-specific: `<worktree>/.agents/skills/<skill-name>/`
///
/// Each skill contains:
/// - `SKILL.md` - Main instructions
/// - `scripts/` - Executable scripts
/// - `references/` - Additional documentation
/// - `assets/` - Templates, data files, images
///
/// To use a skill, first read its SKILL.md file, then explore its resources as needed.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SkillToolInput {
    /// The path to read or list within a skills directory.
    /// Use `~` for the home directory prefix.
    ///
    /// Examples:
    /// - `~/.config/zed/skills/brainstorming/SKILL.md`
    /// - `~/.config/zed/skills/brainstorming/references/`
    pub path: String,
}

pub struct SkillTool {
    project: Entity<Project>,
}

impl SkillTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for SkillTool {
    type Input = SkillToolInput;
    type Output = String;

    const NAME: &'static str = "read_skill";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let path = std::path::Path::new(&input.path);
            let skill_name = path
                .components()
                .rev()
                .find_map(|component| {
                    let name = component.as_os_str().to_str()?;
                    if name == "skills"
                        || name == ".agents"
                        || name == ".config"
                        || name == "zed"
                        || name == "~"
                        || name == "SKILL.md"
                        || name == "references"
                        || name == "scripts"
                        || name == "assets"
                    {
                        None
                    } else {
                        Some(name.to_string())
                    }
                })
                .unwrap_or_default();

            if skill_name.is_empty() {
                "Read skill".into()
            } else {
                format!("Read skill `{skill_name}`").into()
            }
        } else {
            "Read skill".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String, String>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            let fs = project.read_with(cx, |project, _cx| project.fs().clone());
            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            let canonical_path = crate::skills::is_skills_path(&input.path, &canonical_roots)
                .ok_or_else(|| {
                    format!(
                        "Path {} is not within a skills directory. \
                         Skills are located at ~/.config/zed/skills/<skill-name>/ \
                         or <worktree>/.agents/skills/<skill-name>/",
                        input.path
                    )
                })?;

            if fs.is_file(&canonical_path).await {
                fs.load(&canonical_path)
                    .await
                    .map_err(|e| format!("Failed to read {}: {e}", input.path))
            } else if fs.is_dir(&canonical_path).await {
                let mut entries = fs
                    .read_dir(&canonical_path)
                    .await
                    .map_err(|e| format!("Failed to list {}: {e}", input.path))?;

                let mut folders = Vec::new();
                let mut files = Vec::new();

                while let Some(entry) = entries.next().await {
                    let path = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();
                    if fs.is_dir(&path).await {
                        folders.push(name);
                    } else {
                        files.push(name);
                    }
                }

                folders.sort();
                files.sort();

                let mut output = String::new();
                if !folders.is_empty() {
                    writeln!(output, "# Folders:\n{}", folders.join("\n"))
                        .map_err(|e| e.to_string())?;
                }
                if !files.is_empty() {
                    writeln!(output, "\n# Files:\n{}", files.join("\n"))
                        .map_err(|e| e.to_string())?;
                }
                if output.is_empty() {
                    output = format!("{} is empty.", input.path);
                }

                Ok(output)
            } else {
                Err(format!("Path not found: {}", input.path))
            }
        })
    }
}
