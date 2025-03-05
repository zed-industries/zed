use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use gpui::{App, Entity, Task, WeakEntity};
use language::{BufferSnapshot, Language, LanguageName, LspAdapterDelegate};
use project::Project;
use serde::Serialize;
use std::collections::HashMap;
use std::{
    path::PathBuf,
    process::Command,
    sync::{atomic::AtomicBool, Arc},
};
use ui::prelude::*;
use workspace::Workspace;

#[derive(Debug)]
struct DiffFile {
    path: PathBuf,
    old_path: Option<PathBuf>,
    content: String,
    is_binary: bool,
    file_mode_change: bool,
    old_content: Option<String>,
    new_content: Option<String>,
}

#[derive(Debug, Default)]
struct DiffStats {
    files_changed: usize,
    insertions: usize,
    deletions: usize,
}

#[derive(Debug, Serialize)]
struct DiffMetadata {
    path: String,
    stats: FileStats,
    is_staged: bool,
    old_content: Option<String>,
    new_content: Option<String>,
    language_name: Option<String>,
}

#[derive(Debug, Serialize, Default)]
struct FileStats {
    insertions: usize,
    deletions: usize,
}

pub struct DiffSlashCommand;

impl DiffSlashCommand {
    fn get_git_diffs(
        project: &Entity<Project>,
        cx: &App,
    ) -> Result<(Vec<DiffFile>, Vec<DiffFile>)> {
        let staged = Self::get_git_diff_internal(project, true, cx)?;
        let unstaged = Self::get_git_diff_internal(project, false, cx)?;
        Ok((staged, unstaged))
    }

    fn get_git_diff_internal(
        project: &Entity<Project>,
        staged: bool,
        cx: &App,
    ) -> Result<Vec<DiffFile>> {
        let project = project.read(cx);
        let worktree = project
            .worktrees(cx)
            .next()
            .ok_or_else(|| anyhow!("No worktree found"))?;

        let root_path = worktree.read(cx).abs_path().to_owned();

        let args = if staged {
            vec![
                "diff",
                "--cached",
                "--no-color",
                "--patch-with-raw",
                "--full-index",
            ]
        } else {
            vec!["diff", "--no-color", "--patch-with-raw", "--full-index"]
        };

        let output = Command::new("git")
            .args(&args)
            .current_dir(&root_path)
            .output()
            .map_err(|e| anyhow!("Failed to execute git command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!(
                "Git command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let diff_output = String::from_utf8_lossy(&output.stdout);
        Self::parse_git_diff(&diff_output, staged)
    }

    fn parse_git_diff(diff_output: &str, is_staged: bool) -> Result<Vec<DiffFile>> {
        let mut diff_files = Vec::new();
        let mut current_file = None;
        let mut current_content = String::new();
        let mut old_content = String::new();
        let mut new_content = String::new();
        let mut in_old_content = false;
        let mut in_new_content = false;

        for line in diff_output.lines() {
            if line.starts_with("diff --git ") {
                // Save previous file if exists
                if let Some(file) = current_file.take() {
                    let is_binary = current_content.contains("Binary files");
                    let file_mode_change = current_content.contains("mode ");
                    diff_files.push(DiffFile {
                        path: file,
                        old_path: None,
                        content: current_content.clone(),
                        is_binary,
                        file_mode_change,
                        old_content: if old_content.is_empty() {
                            None
                        } else {
                            Some(old_content.clone())
                        },
                        new_content: if new_content.is_empty() {
                            None
                        } else {
                            Some(new_content.clone())
                        },
                    });
                    current_content.clear();
                    old_content.clear();
                    new_content.clear();
                }

                // Parse new file path
                let path = line
                    .split(' ')
                    .nth(3)
                    .ok_or_else(|| anyhow!("Invalid diff header"))?
                    .trim_start_matches("b/")
                    .to_string();
                current_file = Some(PathBuf::from(path));
            } else if line.starts_with("--- ") {
                in_old_content = true;
                in_new_content = false;
            } else if line.starts_with("+++ ") {
                in_old_content = false;
                in_new_content = true;
            } else if line.starts_with('+') && !line.starts_with("+++") {
                if in_new_content {
                    new_content.push_str(&line[1..]);
                    new_content.push('\n');
                }
            } else if line.starts_with('-') && !line.starts_with("---") {
                if in_old_content {
                    old_content.push_str(&line[1..]);
                    old_content.push('\n');
                }
            } else if !line.starts_with('@') {
                if in_old_content {
                    old_content.push_str(line);
                    old_content.push('\n');
                }
                if in_new_content {
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }

            current_content.push_str(line);
            current_content.push('\n');
        }

        // Add last file
        if let Some(file) = current_file {
            let is_binary = current_content.contains("Binary files");
            let file_mode_change = current_content.contains("mode ");
            diff_files.push(DiffFile {
                path: file,
                old_path: None,
                content: current_content,
                is_binary,
                file_mode_change,
                old_content: if old_content.is_empty() {
                    None
                } else {
                    Some(old_content)
                },
                new_content: if new_content.is_empty() {
                    None
                } else {
                    Some(new_content)
                },
            });
        }

        Ok(diff_files)
    }

    fn calculate_stats(files: &[DiffFile]) -> DiffStats {
        let mut stats = DiffStats::default();
        stats.files_changed = files.len();

        for file in files {
            if !file.is_binary {
                for line in file.content.lines() {
                    if line.starts_with('+') && !line.starts_with("+++") {
                        stats.insertions += 1;
                    } else if line.starts_with('-') && !line.starts_with("---") {
                        stats.deletions += 1;
                    }
                }
            }
        }

        stats
    }

    fn format_diff_output(
        staged_files: Vec<DiffFile>,
        unstaged_files: Vec<DiffFile>,
    ) -> SlashCommandOutput {
        let mut output = SlashCommandOutput::default();

        let staged_stats = Self::calculate_stats(&staged_files);
        let unstaged_stats = Self::calculate_stats(&unstaged_files);

        if !staged_files.is_empty() {
            output.text.push_str("# Staged Changes\n");
            output.text.push_str(&format!(
                "📊 {} files changed, {} insertions(+), {} deletions(-)\n\n",
                staged_stats.files_changed, staged_stats.insertions, staged_stats.deletions
            ));
            Self::append_files_to_output(&mut output, staged_files, true);
        }

        if !unstaged_files.is_empty() {
            if !output.text.is_empty() {
                output.text.push_str("\n");
            }
            output.text.push_str("# Unstaged Changes\n");
            output.text.push_str(&format!(
                "📊 {} files changed, {} insertions(+), {} deletions(-)\n\n",
                unstaged_stats.files_changed, unstaged_stats.insertions, unstaged_stats.deletions
            ));
            Self::append_files_to_output(&mut output, unstaged_files, false);
        }

        if output.text.is_empty() {
            output.text.push_str("No changes found");
        }

        output
    }

    fn append_files_to_output(
        output: &mut SlashCommandOutput,
        files: Vec<DiffFile>,
        is_staged: bool,
    ) {
        for file in files {
            let file_path = file.path.to_string_lossy().into_owned();
            let extension = file
                .path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_owned();

            let language_name = Some(
                Language::new(
                    language::LanguageConfig {
                        name: LanguageName::new(&extension),
                        ..Default::default()
                    },
                    None,
                )
                .name()
                .to_string(),
            );

            let start_pos = output.text.len();

            // Calculate stats
            let mut file_stats = FileStats::default();
            if !file.is_binary {
                for line in file.content.lines() {
                    if line.starts_with('+') && !line.starts_with("+++") {
                        file_stats.insertions += 1;
                    } else if line.starts_with('-') && !line.starts_with("---") {
                        file_stats.deletions += 1;
                    }
                }
            }

            // Add file header with stats and path
            output.text.push_str(&format!(
                "📄 {} (+{}, -{})\n",
                file_path, file_stats.insertions, file_stats.deletions
            ));

            // Add diff content with syntax highlighting
            if file.is_binary {
                output.text.push_str("Binary file differs\n");
            } else if file.file_mode_change {
                output.text.push_str(&file.content);
            } else {
                // Use diff-{language} format for proper syntax highlighting and diff colors
                output.text.push_str(&format!("```diff-{}\n", extension));
                output.text.push_str(&file.content);
                output.text.push_str("```\n");

                // Add the full file context with syntax highlighting
                if let (Some(old), Some(new)) = (file.old_content.as_ref(), file.new_content.as_ref()) {
                    output.text.push_str("\nFull context:\n");
                    // Use the language's syntax highlighting for the full files
                    output.text.push_str(&format!("```{}\n", extension));
                    output.text.push_str("--- Old\n");
                    output.text.push_str(old);
                    output.text.push_str("\n+++ New\n");
                    output.text.push_str(new);
                    output.text.push_str("```\n");
                }
            }
            output.text.push('\n');

            // Add section with metadata
            output.sections.push(SlashCommandOutputSection {
                range: start_pos..output.text.len(),
                icon: IconName::GitBranch,
                label: if is_staged {
                    format!("Staged: {}", file_path).into()
                } else {
                    file_path.clone().into()
                },
                metadata: Some(
                    serde_json::to_value(DiffMetadata {
                        path: file_path,
                        stats: file_stats,
                        is_staged,
                        old_content: file.old_content,
                        new_content: file.new_content,
                        language_name,
                    })
                    .unwrap(),
                ),
            });
        }
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

        match Self::get_git_diffs(&project, cx) {
            Ok((staged_files, unstaged_files)) => {
                let output = Self::format_diff_output(staged_files, unstaged_files);
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
    async fn test_parse_git_diff() {
        let diff_output = r#"diff --git a/src/main.rs b/src/main.rs
index 1234567..89abcdef 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
+use std::fmt;
 fn main() {
-    println!("Hello");
+    println!("Hello, World!");
 }
"#;
        let diff_files = DiffSlashCommand::parse_git_diff(diff_output, false).unwrap();
        assert_eq!(diff_files.len(), 1);
        assert_eq!(diff_files[0].path, PathBuf::from("src/main.rs"));
        assert!(!diff_files[0].is_binary);
        assert!(!diff_files[0].file_mode_change);
    }

    #[gpui::test]
    async fn test_binary_file_diff() {
        let diff_output = r#"diff --git a/image.png b/image.png
index 1234567..89abcdef 100644
Binary files a/image.png and b/image.png differ
"#;
        let diff_files = DiffSlashCommand::parse_git_diff(diff_output, false).unwrap();
        assert_eq!(diff_files.len(), 1);
        assert_eq!(diff_files[0].path, PathBuf::from("image.png"));
        assert!(diff_files[0].is_binary);
    }

    #[gpui::test]
    async fn test_file_mode_change() {
        let diff_output = r#"diff --git a/script.sh b/script.sh
old mode 100644
new mode 100755
"#;
        let diff_files = DiffSlashCommand::parse_git_diff(diff_output, false).unwrap();
        assert_eq!(diff_files.len(), 1);
        assert_eq!(diff_files[0].path, PathBuf::from("script.sh"));
        assert!(diff_files[0].file_mode_change);
    }

    #[gpui::test]
    async fn test_diff_stats_calculation() {
        let diff_output = r#"diff --git a/src/main.rs b/src/main.rs
index 1234567..89abcdef 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
+use std::fmt;
+use std::io;
 fn main() {
-    println!("Hello");
+    println!("Hello, World!");
 }
"#;
        let diff_files = DiffSlashCommand::parse_git_diff(diff_output, false).unwrap();
        let stats = DiffSlashCommand::calculate_stats(&diff_files);
        assert_eq!(stats.files_changed, 1);
        assert_eq!(stats.insertions, 3);
        assert_eq!(stats.deletions, 1);
    }

    #[gpui::test]
    async fn test_staged_and_unstaged_separation() {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, ["/test".as_ref()], cx).await;

        // Initialize git repo and create some changes
        Command::new("git")
            .args(&["init"])
            .current_dir("/test")
            .output()
            .expect("Failed to init git repo");

        // Create and stage one file
        std::fs::write("/test/staged.txt", "staged content").unwrap();
        Command::new("git")
            .args(&["add", "staged.txt"])
            .current_dir("/test")
            .output()
            .expect("Failed to add file");

        // Create unstaged file
        std::fs::write("/test/unstaged.txt", "unstaged content").unwrap();

        let (staged, unstaged) = DiffSlashCommand::get_git_diffs(&project, cx).unwrap();
        assert_eq!(staged.len(), 1);
        assert_eq!(unstaged.len(), 1);
        assert_eq!(staged[0].path, PathBuf::from("staged.txt"));
        assert_eq!(unstaged[0].path, PathBuf::from("unstaged.txt"));
    }
}
