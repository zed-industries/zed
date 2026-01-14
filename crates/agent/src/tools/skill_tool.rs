use agent_client_protocol as acp;
use agent_skills::Skill;
use anyhow::{Result, anyhow};
use futures::StreamExt;
use gpui::{App, Entity, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream};

/// Maximum size for directory listing (100KB)
const MAX_DIRECTORY_LISTING_SIZE: usize = 100 * 1024;

/// Retrieves the content and resources of a skill by name. Use this when a user's request matches a skill's description.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SkillToolInput {
    /// The name of the skill to retrieve
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillToolOutput {
    /// Whether the skill is global or project-local
    pub source: String,
    /// For project-local skills, which worktree it belongs to
    pub worktree: Option<String>,
    /// The full content of SKILL.md
    pub content: String,
    /// List of all files in the skill's directory (capped at 100KB total listing)
    pub files: Vec<String>,
}

impl From<SkillToolOutput> for LanguageModelToolResultContent {
    fn from(output: SkillToolOutput) -> Self {
        let mut result = String::new();
        result.push_str(&format!("Source: {}\n", output.source));
        if let Some(worktree) = &output.worktree {
            result.push_str(&format!("Worktree: {}\n", worktree));
        }
        result.push_str("\n## Skill Content\n\n");
        result.push_str(&output.content);
        if !output.files.is_empty() {
            result.push_str("\n\n## Files in skill directory\n\n");
            for file in &output.files {
                result.push_str(&format!("- {}\n", file));
            }
        }
        LanguageModelToolResultContent::Text(result.into())
    }
}

pub struct SkillTool {
    skills: Arc<Vec<Skill>>,
    project: Entity<Project>,
}

impl SkillTool {
    pub fn new(skills: Arc<Vec<Skill>>, project: Entity<Project>) -> Self {
        Self { skills, project }
    }

    fn find_skill(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.name == name)
    }
}

impl AgentTool for SkillTool {
    type Input = SkillToolInput;
    type Output = SkillToolOutput;

    fn name() -> &'static str {
        "skill"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Get skill `{}`", input.name).into()
        } else {
            "Get skill".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<SkillToolOutput>> {
        let Some(skill) = self.find_skill(&input.name) else {
            return Task::ready(Err(anyhow!(
                "Skill '{}' not found. Available skills: {}",
                input.name,
                self.skills
                    .iter()
                    .map(|s| s.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        };

        let source = match &skill.source {
            agent_skills::SkillSource::Global => "global".to_string(),
            agent_skills::SkillSource::ProjectLocal { .. } => "project-local".to_string(),
        };

        let worktree = match &skill.source {
            agent_skills::SkillSource::Global => None,
            agent_skills::SkillSource::ProjectLocal { worktree_id } => {
                Some(format!("worktree-{}", worktree_id.to_usize()))
            }
        };

        let content = skill.content.clone();
        let directory_path = skill.directory_path.clone();
        let fs = self.project.read(cx).fs().clone();

        cx.spawn(async move |_cx| {
            let mut files = Vec::new();
            let mut total_size = 0;

            if let Ok(mut entries) = fs.read_dir(&directory_path).await {
                while let Some(entry) = entries.next().await {
                    let Ok(path) = entry else {
                        continue;
                    };

                    if let Some(file_name) = path.file_name() {
                        let file_name_str = file_name.to_string_lossy().to_string();
                        let entry_size = file_name_str.len();

                        if total_size + entry_size > MAX_DIRECTORY_LISTING_SIZE {
                            break;
                        }

                        total_size += entry_size;
                        files.push(file_name_str);
                    }
                }
            }

            files.sort();

            Ok(SkillToolOutput {
                source,
                worktree,
                content,
                files,
            })
        })
    }
}
