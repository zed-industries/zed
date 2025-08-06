use crate::{
    templates::{BaseTemplate, Template, Templates, WorktreeData},
    thread::Prompt,
};
use anyhow::Result;
use gpui::{App, Entity};
use project::Project;

pub struct BasePrompt {
    project: Entity<Project>,
}

impl BasePrompt {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl Prompt for BasePrompt {
    fn render(&self, templates: &Templates, cx: &App) -> Result<String> {
        BaseTemplate {
            os: std::env::consts::OS.to_string(),
            shell: util::get_system_shell(),
            worktrees: self
                .project
                .read(cx)
                .worktrees(cx)
                .map(|worktree| WorktreeData {
                    root_name: worktree.read(cx).root_name().to_string(),
                })
                .collect(),
        }
        .render(templates)
    }
}
