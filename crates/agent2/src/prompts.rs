use super::*;

struct BasePrompt {
    project: Entity<Project>,
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
