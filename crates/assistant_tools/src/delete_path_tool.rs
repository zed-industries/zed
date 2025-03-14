use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Arc};
use util::paths::PathMatcher;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeletePathToolInput {
    /// The glob to match files in the project to delete.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can delete the first two files by providing a glob of "*thing*.txt"
    /// </example>
    pub glob: String,
}

pub struct DeletePathTool;

impl Tool for DeletePathTool {
    fn name(&self) -> String {
        "delete-path".into()
    }

    fn description(&self) -> String {
        include_str!("./delete_path_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(DeletePathToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let glob = match serde_json::from_value::<DeletePathToolInput>(input) {
            Ok(input) => input.glob,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };
        let path_matcher = match PathMatcher::new(&[glob.clone()]) {
            Ok(matcher) => matcher,
            Err(err) => return Task::ready(Err(anyhow!("Invalid glob: {}", err))),
        };

        struct Match {
            display_path: String,
            path: PathBuf,
        }

        let mut matches = Vec::new();
        let mut deleted_paths = Vec::new();
        let mut errors = Vec::new();

        for worktree_handle in project.read(cx).worktrees(cx) {
            let worktree = worktree_handle.read(cx);
            let worktree_root = worktree.abs_path().to_path_buf();

            // Don't consider ignored entries.
            for entry in worktree.entries(false, 0) {
                if path_matcher.is_match(&entry.path) {
                    matches.push(Match {
                        path: worktree_root.join(&entry.path),
                        display_path: entry.path.display().to_string(),
                    });
                }
            }
        }

        if matches.is_empty() {
            return Task::ready(Ok(format!("No paths in the project matched {glob:?}")));
        }

        let paths_matched = matches.len();

        // Delete the files
        for Match { path, display_path } in matches {
            match fs::remove_file(&path) {
                Ok(()) => {
                    deleted_paths.push(display_path);
                }
                Err(file_err) => {
                    // Try to remove directory if it's not a file. Retrying as a directory
                    // on error saves a syscall compared to checking whether it's
                    // a directory up front for every single file.
                    if let Err(dir_err) = fs::remove_dir_all(&path) {
                        let error = if path.is_dir() {
                            format!("Failed to delete directory {}: {dir_err}", display_path)
                        } else {
                            format!("Failed to delete file {}: {file_err}", display_path)
                        };

                        errors.push(error);
                    } else {
                        deleted_paths.push(display_path);
                    }
                }
            }
        }

        if errors.is_empty() {
            // 0 deleted paths should never happen if there were no errors;
            // we already returned if matches was empty.
            let answer = if deleted_paths.len() == 1 {
                format!(
                    "Deleted {}",
                    deleted_paths.first().unwrap_or(&String::new())
                )
            } else {
                // Sort to group entries in the same directory together
                deleted_paths.sort();

                let mut buf = format!("Deleted these {} paths:\n", deleted_paths.len());

                for path in deleted_paths.iter() {
                    buf.push('\n');
                    buf.push_str(path);
                }

                buf
            };

            Task::ready(Ok(answer))
        } else {
            if deleted_paths.is_empty() {
                Task::ready(Err(anyhow!(
                    "{glob:?} matched {} deleted because of {}:\n{}",
                    if paths_matched == 1 {
                        "1 path, but it was not".to_string()
                    } else {
                        format!("{} paths, but none were", paths_matched)
                    },
                    if errors.len() == 1 {
                        "this error".to_string()
                    } else {
                        format!("{} errors", errors.len())
                    },
                    errors.join("\n")
                )))
            } else {
                // Sort to group entries in the same directory together
                deleted_paths.sort();
                Task::ready(Ok(format!(
                    "Deleted {} paths matching glob {glob:?}:\n{}\n\nErrors:\n{}",
                    deleted_paths.len(),
                    deleted_paths.join("\n"),
                    errors.join("\n")
                )))
            }
        }
    }
}
