use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent, SlashCommandOutput,
    SlashCommandOutputSection, SlashCommandResult,
};
use futures::Stream;
use gpui::{App, Entity, Task, WeakEntity};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};
use project::Project;
use std::{
    process::Command,
    sync::{atomic::AtomicBool, Arc},
};
use ui::prelude::*;
use workspace::Workspace;

pub struct DiffSlashCommand;

impl DiffSlashCommand {
    fn get_git_diff(project: &Entity<Project>, cx: &App) -> Result<String> {
        let project = project.read(cx);
        let worktree = project
            .worktrees(cx)
            .next()
            .ok_or_else(|| anyhow!("No worktree found"))?;
        
        let root_path = worktree.read(cx).abs_path().to_owned();

        let output = Command::new("git")
            .arg("diff")
            .current_dir(root_path)
            .output()
            .map_err(|e| anyhow!("Failed to execute git command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!(
                "Git command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

impl SlashCommand for DiffSlashCommand {
    fn name(&self) -> String {
        "diff".into()
    }

    fn description(&self) -> String {
        "Insert git diff".into()
    }

    fn icon(&self) -> IconName {
        IconName::GitBranch
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn accepts_arguments(&self) -> bool {
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let project = workspace.read(cx).project().clone();

        match Self::get_git_diff(&project, cx) {
            Ok(diff_text) => {
                let mut output = SlashCommandOutput::default();
                output.text.push_str("```diff\n");
                output.text.push_str(&diff_text);
                if !diff_text.ends_with('\n') {
                    output.text.push('\n');
                }
                output.text.push_str("```\n");

                Task::ready(Ok(output.to_event_stream()))
            }
            Err(e) => Task::ready(Err(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }

    #[gpui::test]
    async fn test_diff_command_no_repo(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        // Create a project without git repo
        let project = Project::test(fs, ["/test".as_ref()], cx).await;

        let result = DiffSlashCommand::get_git_diff(&project, cx);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to execute git command"));
    }

    #[gpui::test]
    async fn test_diff_command_with_repo(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        // Create a project with git repo
        let project = Project::test(fs, ["/test".as_ref()], cx).await;

        // Initialize git repo
        Command::new("git")
            .args(&["init"])
            .current_dir("/test")
            .output()
            .expect("Failed to init git repo");

        // Create and add a test file
        std::fs::write("/test/file.txt", "test content").unwrap();
        Command::new("git")
            .args(&["add", "file.txt"])
            .current_dir("/test")
            .output()
            .expect("Failed to add file");

        // Modify the file to create a diff
        std::fs::write("/test/file.txt", "modified content").unwrap();

        let result = DiffSlashCommand::get_git_diff(&project, cx);
        assert!(result.is_ok());
        let diff = result.unwrap();
        assert!(diff.contains("diff --git"));
        assert!(diff.contains("modified content"));
    }
}
