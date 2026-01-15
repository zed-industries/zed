use crate::{AgentTool, SkillTool, Thread, ToolCallEventStream};
use agent_client_protocol::ToolKind;
use agent_skills::{SkillSource, global_skills_dir, project_skills_relative_path};
use anyhow::{Result, anyhow};
use futures::StreamExt;
use gpui::{App, Entity, SharedString, Task, WeakEntity};
use project::{Project, ProjectPath, WorktreeSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

/// Lists files and directories in a given path. Prefer the `grep` or `find_path` tools when searching the codebase.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListDirectoryToolInput {
    /// The fully-qualified path of the directory to list in the project.
    ///
    /// This path should never be absolute, and the first component of the path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - directory1
    /// - directory2
    ///
    /// You can list the contents of `directory1` by using the path `directory1`.
    /// </example>
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - foo
    /// - bar
    ///
    /// If you wanna list contents in the directory `foo/baz`, you should use the path `foo/baz`.
    /// </example>
    pub path: String,
}

pub struct ListDirectoryTool {
    project: Entity<Project>,
    thread: Option<WeakEntity<Thread>>,
}

impl ListDirectoryTool {
    pub fn new(project: Entity<Project>, thread: WeakEntity<Thread>) -> Self {
        Self {
            project,
            thread: Some(thread),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn new_without_thread(project: Entity<Project>) -> Self {
        Self {
            project,
            thread: None,
        }
    }

    /// Check if a path is within a skill directory and list its contents if allowed.
    fn try_list_skill_directory(
        &self,
        input: &ListDirectoryToolInput,
        skill_tool_enabled: bool,
        cx: &App,
    ) -> Option<Task<Result<String>>> {
        if !skill_tool_enabled {
            return None;
        }

        let input_path = Path::new(&input.path);

        // Check if this looks like an absolute path
        if !input_path.is_absolute() {
            return None;
        }

        // Check if the path is within the global skills directory
        // We use starts_with on the path directly rather than canonicalizing,
        // as canonicalization requires the file to exist on the real filesystem
        // which doesn't work with FakeFs in tests.
        let global_skills = global_skills_dir();
        if input_path.starts_with(global_skills) {
            return Some(self.list_skill_directory_contents(
                input_path.to_path_buf(),
                SkillSource::Global,
                cx,
            ));
        }

        // Check if the path is within any worktree's .zed/skills directory
        for worktree in self.project.read(cx).worktrees(cx) {
            let worktree = worktree.read(cx);
            let worktree_skills_dir = worktree.abs_path().join(project_skills_relative_path());
            if input_path.starts_with(&worktree_skills_dir) {
                return Some(self.list_skill_directory_contents(
                    input_path.to_path_buf(),
                    SkillSource::ProjectLocal {
                        worktree_id: worktree.id(),
                    },
                    cx,
                ));
            }
        }

        None
    }

    fn list_skill_directory_contents(
        &self,
        path: std::path::PathBuf,
        _source: SkillSource,
        cx: &App,
    ) -> Task<Result<String>> {
        let fs = self.project.read(cx).fs().clone();

        cx.spawn(async move |_cx| {
            if !fs.is_dir(&path).await {
                return Err(anyhow!("{} is not a directory", path.display()));
            }

            let mut entries = fs.read_dir(&path).await?;
            let mut folders = Vec::new();
            let mut files = Vec::new();

            while let Some(entry) = entries.next().await {
                let Ok(entry_path) = entry else {
                    continue;
                };

                let file_name = entry_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                if fs.is_dir(&entry_path).await {
                    folders.push(file_name);
                } else {
                    files.push(file_name);
                }
            }

            folders.sort();
            files.sort();

            let mut output = String::new();

            if !folders.is_empty() {
                writeln!(output, "# Folders:\n{}", folders.join("\n")).unwrap();
            }

            if !files.is_empty() {
                writeln!(output, "\n# Files:\n{}", files.join("\n")).unwrap();
            }

            if output.is_empty() {
                writeln!(output, "{} is empty.", path.display()).unwrap();
            }

            Ok(output)
        })
    }
}

impl AgentTool for ListDirectoryTool {
    type Input = ListDirectoryToolInput;
    type Output = String;

    fn name() -> &'static str {
        "list_directory"
    }

    fn kind() -> ToolKind {
        ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let path = MarkdownInlineCode(&input.path);
            format!("List the {path} directory's contents").into()
        } else {
            "List directory".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        // Check if the skill tool is enabled (needed for listing skill directories)
        let skill_tool_enabled = self
            .thread
            .as_ref()
            .and_then(|t| t.upgrade())
            .map(|thread| thread.read(cx).has_registered_tool(SkillTool::name()))
            .unwrap_or(false);

        // Sometimes models will return these even though we tell it to give a path and not a glob.
        // When this happens, just list the root worktree directories.
        if matches!(input.path.as_str(), "." | "" | "./" | "*") {
            let output = self
                .project
                .read(cx)
                .worktrees(cx)
                .filter_map(|worktree| {
                    let worktree = worktree.read(cx);
                    let root_entry = worktree.root_entry()?;
                    if root_entry.is_dir() {
                        Some(root_entry.path.display(worktree.path_style()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            return Task::ready(Ok(output));
        }

        // First, try to find the path in the project
        let project_path = self.project.read(cx).find_project_path(&input.path, cx);

        // If not found in project, check if it's a skill directory path (only if skill tool is enabled)
        if project_path.is_none() {
            if let Some(result) = self.try_list_skill_directory(&input, skill_tool_enabled, cx) {
                return result;
            }
            return Task::ready(Err(anyhow!("Path {} not found in project", input.path)));
        }

        let Some(project_path) = project_path else {
            return Task::ready(Err(anyhow!("Path {} not found in project", input.path)));
        };
        let Some(worktree) = self
            .project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("Worktree not found")));
        };

        // Check if the directory whose contents we're listing is itself excluded or private
        let global_settings = WorktreeSettings::get_global(cx);
        if global_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot list directory because its path matches the user's global `file_scan_exclusions` setting: {}",
                &input.path
            )));
        }

        if global_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot list directory because its path matches the user's global `private_files` setting: {}",
                &input.path
            )));
        }

        let worktree_settings = WorktreeSettings::get(Some((&project_path).into()), cx);
        if worktree_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot list directory because its path matches the user's worktree`file_scan_exclusions` setting: {}",
                &input.path
            )));
        }

        if worktree_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot list directory because its path matches the user's worktree `private_paths` setting: {}",
                &input.path
            )));
        }

        let worktree_snapshot = worktree.read(cx).snapshot();
        let worktree_root_name = worktree.read(cx).root_name();

        let Some(entry) = worktree_snapshot.entry_for_path(&project_path.path) else {
            return Task::ready(Err(anyhow!("Path not found: {}", input.path)));
        };

        if !entry.is_dir() {
            return Task::ready(Err(anyhow!("{} is not a directory.", input.path)));
        }
        let worktree_snapshot = worktree.read(cx).snapshot();

        let mut folders = Vec::new();
        let mut files = Vec::new();

        for entry in worktree_snapshot.child_entries(&project_path.path) {
            // Skip private and excluded files and directories
            if global_settings.is_path_private(&entry.path)
                || global_settings.is_path_excluded(&entry.path)
            {
                continue;
            }

            let project_path: ProjectPath = (worktree_snapshot.id(), entry.path.clone()).into();
            if worktree_settings.is_path_excluded(&project_path.path)
                || worktree_settings.is_path_private(&project_path.path)
            {
                continue;
            }

            let full_path = worktree_root_name
                .join(&entry.path)
                .display(worktree_snapshot.path_style())
                .into_owned();
            if entry.is_dir() {
                folders.push(full_path);
            } else {
                files.push(full_path);
            }
        }

        let mut output = String::new();

        if !folders.is_empty() {
            writeln!(output, "# Folders:\n{}", folders.join("\n")).unwrap();
        }

        if !files.is_empty() {
            writeln!(output, "\n# Files:\n{}", files.join("\n")).unwrap();
        }

        if output.is_empty() {
            writeln!(output, "{} is empty.", input.path).unwrap();
        }

        Task::ready(Ok(output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ContextServerRegistry, Templates, Thread};
    use gpui::{AppContext as _, TestAppContext, UpdateGlobal};
    use indoc::indoc;
    use language_model::fake_provider::FakeLanguageModel;
    use project::{FakeFs, Project};
    use prompt_store::ProjectContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn platform_paths(path_str: &str) -> String {
        if cfg!(target_os = "windows") {
            path_str.replace("/", "\\")
        } else {
            path_str.to_string()
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    #[gpui::test]
    async fn test_list_directory_separates_files_and_dirs(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "src": {
                    "main.rs": "fn main() {}",
                    "lib.rs": "pub fn hello() {}",
                    "models": {
                        "user.rs": "struct User {}",
                        "post.rs": "struct Post {}"
                    },
                    "utils": {
                        "helper.rs": "pub fn help() {}"
                    }
                },
                "tests": {
                    "integration_test.rs": "#[test] fn test() {}"
                },
                "README.md": "# Project",
                "Cargo.toml": "[package]"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let tool = Arc::new(ListDirectoryTool::new_without_thread(project));

        // Test listing root directory
        let input = ListDirectoryToolInput {
            path: "project".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert_eq!(
            output,
            platform_paths(indoc! {"
                # Folders:
                project/src
                project/tests

                # Files:
                project/Cargo.toml
                project/README.md
            "})
        );

        // Test listing src directory
        let input = ListDirectoryToolInput {
            path: "project/src".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert_eq!(
            output,
            platform_paths(indoc! {"
                # Folders:
                project/src/models
                project/src/utils

                # Files:
                project/src/lib.rs
                project/src/main.rs
            "})
        );

        // Test listing directory with only files
        let input = ListDirectoryToolInput {
            path: "project/tests".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert!(!output.contains("# Folders:"));
        assert!(output.contains("# Files:"));
        assert!(output.contains(&platform_paths("project/tests/integration_test.rs")));
    }

    #[gpui::test]
    async fn test_list_directory_empty_directory(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "empty_dir": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let tool = Arc::new(ListDirectoryTool::new_without_thread(project));

        let input = ListDirectoryToolInput {
            path: "project/empty_dir".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert_eq!(output, "project/empty_dir is empty.\n");
    }

    #[gpui::test]
    async fn test_list_directory_error_cases(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "file.txt": "content"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let tool = Arc::new(ListDirectoryTool::new_without_thread(project));

        // Test non-existent path
        let input = ListDirectoryToolInput {
            path: "project/nonexistent".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await;
        assert!(output.unwrap_err().to_string().contains("Path not found"));

        // Test trying to list a file instead of directory
        let input = ListDirectoryToolInput {
            path: "project/file.txt".into(),
        };
        let output = cx
            .update(|cx| tool.run(input, ToolCallEventStream::test().0, cx))
            .await;
        assert!(
            output
                .unwrap_err()
                .to_string()
                .contains("is not a directory")
        );
    }

    #[gpui::test]
    async fn test_list_directory_security(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                "normal_dir": {
                    "file1.txt": "content",
                    "file2.txt": "content"
                },
                ".mysecrets": "SECRET_KEY=abc123",
                ".secretdir": {
                    "config": "special configuration",
                    "secret.txt": "secret content"
                },
                ".mymetadata": "custom metadata",
                "visible_dir": {
                    "normal.txt": "normal content",
                    "special.privatekey": "private key content",
                    "data.mysensitive": "sensitive data",
                    ".hidden_subdir": {
                        "hidden_file.txt": "hidden content"
                    }
                }
            }),
        )
        .await;

        // Configure settings explicitly
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_exclusions = Some(vec![
                        "**/.secretdir".to_string(),
                        "**/.mymetadata".to_string(),
                        "**/.hidden_subdir".to_string(),
                    ]);
                    settings.project.worktree.private_files = Some(
                        vec![
                            "**/.mysecrets".to_string(),
                            "**/*.privatekey".to_string(),
                            "**/*.mysensitive".to_string(),
                        ]
                        .into(),
                    );
                });
            });
        });

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let tool = Arc::new(ListDirectoryTool::new_without_thread(project));

        // Listing root directory should exclude private and excluded files
        let input = ListDirectoryToolInput {
            path: "project".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();

        // Should include normal directories
        assert!(output.contains("normal_dir"), "Should list normal_dir");
        assert!(output.contains("visible_dir"), "Should list visible_dir");

        // Should NOT include excluded or private files
        assert!(
            !output.contains(".secretdir"),
            "Should not list .secretdir (file_scan_exclusions)"
        );
        assert!(
            !output.contains(".mymetadata"),
            "Should not list .mymetadata (file_scan_exclusions)"
        );
        assert!(
            !output.contains(".mysecrets"),
            "Should not list .mysecrets (private_files)"
        );

        // Trying to list an excluded directory should fail
        let input = ListDirectoryToolInput {
            path: "project/.secretdir".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await;
        assert!(
            output
                .unwrap_err()
                .to_string()
                .contains("file_scan_exclusions"),
            "Error should mention file_scan_exclusions"
        );

        // Listing a directory should exclude private files within it
        let input = ListDirectoryToolInput {
            path: "project/visible_dir".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();

        // Should include normal files
        assert!(output.contains("normal.txt"), "Should list normal.txt");

        // Should NOT include private files
        assert!(
            !output.contains("privatekey"),
            "Should not list .privatekey files (private_files)"
        );
        assert!(
            !output.contains("mysensitive"),
            "Should not list .mysensitive files (private_files)"
        );

        // Should NOT include subdirectories that match exclusions
        assert!(
            !output.contains(".hidden_subdir"),
            "Should not list .hidden_subdir (file_scan_exclusions)"
        );
    }

    #[gpui::test]
    async fn test_list_directory_with_multiple_worktree_settings(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        // Create first worktree with its own private files
        fs.insert_tree(
            path!("/worktree1"),
            json!({
                ".zed": {
                    "settings.json": r#"{
                        "file_scan_exclusions": ["**/fixture.*"],
                        "private_files": ["**/secret.rs", "**/config.toml"]
                    }"#
                },
                "src": {
                    "main.rs": "fn main() { println!(\"Hello from worktree1\"); }",
                    "secret.rs": "const API_KEY: &str = \"secret_key_1\";",
                    "config.toml": "[database]\nurl = \"postgres://localhost/db1\""
                },
                "tests": {
                    "test.rs": "mod tests { fn test_it() {} }",
                    "fixture.sql": "CREATE TABLE users (id INT, name VARCHAR(255));"
                }
            }),
        )
        .await;

        // Create second worktree with different private files
        fs.insert_tree(
            path!("/worktree2"),
            json!({
                ".zed": {
                    "settings.json": r#"{
                        "file_scan_exclusions": ["**/internal.*"],
                        "private_files": ["**/private.js", "**/data.json"]
                    }"#
                },
                "lib": {
                    "public.js": "export function greet() { return 'Hello from worktree2'; }",
                    "private.js": "const SECRET_TOKEN = \"private_token_2\";",
                    "data.json": "{\"api_key\": \"json_secret_key\"}"
                },
                "docs": {
                    "README.md": "# Public Documentation",
                    "internal.md": "# Internal Secrets and Configuration"
                }
            }),
        )
        .await;

        // Set global settings
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_exclusions =
                        Some(vec!["**/.git".to_string(), "**/node_modules".to_string()]);
                    settings.project.worktree.private_files =
                        Some(vec!["**/.env".to_string()].into());
                });
            });
        });

        let project = Project::test(
            fs.clone(),
            [path!("/worktree1").as_ref(), path!("/worktree2").as_ref()],
            cx,
        )
        .await;

        // Wait for worktrees to be fully scanned
        cx.executor().run_until_parked();

        let tool = Arc::new(ListDirectoryTool::new_without_thread(project));

        // Test listing worktree1/src - should exclude secret.rs and config.toml based on local settings
        let input = ListDirectoryToolInput {
            path: "worktree1/src".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert!(output.contains("main.rs"), "Should list main.rs");
        assert!(
            !output.contains("secret.rs"),
            "Should not list secret.rs (local private_files)"
        );
        assert!(
            !output.contains("config.toml"),
            "Should not list config.toml (local private_files)"
        );

        // Test listing worktree1/tests - should exclude fixture.sql based on local settings
        let input = ListDirectoryToolInput {
            path: "worktree1/tests".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert!(output.contains("test.rs"), "Should list test.rs");
        assert!(
            !output.contains("fixture.sql"),
            "Should not list fixture.sql (local file_scan_exclusions)"
        );

        // Test listing worktree2/lib - should exclude private.js and data.json based on local settings
        let input = ListDirectoryToolInput {
            path: "worktree2/lib".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert!(output.contains("public.js"), "Should list public.js");
        assert!(
            !output.contains("private.js"),
            "Should not list private.js (local private_files)"
        );
        assert!(
            !output.contains("data.json"),
            "Should not list data.json (local private_files)"
        );

        // Test listing worktree2/docs - should exclude internal.md based on local settings
        let input = ListDirectoryToolInput {
            path: "worktree2/docs".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();
        assert!(output.contains("README.md"), "Should list README.md");
        assert!(
            !output.contains("internal.md"),
            "Should not list internal.md (local file_scan_exclusions)"
        );

        // Test trying to list an excluded directory directly
        let input = ListDirectoryToolInput {
            path: "worktree1/src/secret.rs".into(),
        };
        let output = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await;
        assert!(
            output
                .unwrap_err()
                .to_string()
                .contains("Cannot list directory"),
        );
    }

    #[gpui::test]
    async fn test_list_skill_directory_when_enabled(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        // Create global skills directory with files
        let skills_dir = paths::skills_dir();
        let skill_content = "---\nname: my-skill\ndescription: Test\n---\n\nContent";
        fs.insert_tree(
            skills_dir,
            json!({
                "my-skill": {
                    "SKILL.md": skill_content,
                    "helper.py": "# helper script",
                    "data": {
                        "config.json": "{}"
                    }
                }
            }),
        )
        .await;

        // Create project
        fs.insert_tree(path!("/project"), json!({ "file.txt": "hello" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        // Create a skill and a thread with skill tool enabled
        let skill = agent_skills::parse_skill(
            &skills_dir.join("my-skill/SKILL.md"),
            skill_content,
            agent_skills::SkillSource::Global,
        )
        .unwrap();

        let skills = Arc::new(vec![skill]);
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());

        let thread = cx.new(|cx| {
            let mut thread = Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                skills.clone(),
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            );
            // Add skill tool to enable skill directory access
            thread.add_tool(SkillTool::new(skills.clone(), project.clone()));
            thread
        });

        let tool = Arc::new(ListDirectoryTool::new(project.clone(), thread.downgrade()));

        // List the skill directory using absolute path
        let skill_dir_path = skills_dir.join("my-skill");
        let input = ListDirectoryToolInput {
            path: skill_dir_path.to_string_lossy().to_string(),
        };

        let (event_stream, _) = ToolCallEventStream::test();
        let result = cx.update(|cx| tool.run(input, event_stream, cx)).await;

        assert!(
            result.is_ok(),
            "Should list skill directory: {:?}",
            result.err()
        );
        let output = result.unwrap();

        // Verify directory contents are listed
        assert!(output.contains("SKILL.md"));
        assert!(output.contains("helper.py"));
        assert!(output.contains("data")); // subdirectory
    }

    #[gpui::test]
    async fn test_list_skill_directory_denied_when_disabled(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        // Create global skills directory
        let skills_dir = paths::skills_dir();
        fs.insert_tree(
            skills_dir,
            json!({
                "my-skill": {
                    "SKILL.md": "---\nname: my-skill\ndescription: Test\n---\n\nContent"
                }
            }),
        )
        .await;

        // Create project
        fs.insert_tree(path!("/project"), json!({ "file.txt": "hello" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        // Create thread WITHOUT skill tool
        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
        let model = Arc::new(FakeLanguageModel::default());

        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                Arc::new(Vec::new()), // No skills!
                context_server_registry,
                Templates::new(),
                Some(model),
                cx,
            )
        });

        let tool = Arc::new(ListDirectoryTool::new(project.clone(), thread.downgrade()));

        // Try to list skill directory
        let skill_dir_path = skills_dir.join("my-skill");
        let input = ListDirectoryToolInput {
            path: skill_dir_path.to_string_lossy().to_string(),
        };

        let (event_stream, _) = ToolCallEventStream::test();
        let result = cx.update(|cx| tool.run(input, event_stream, cx)).await;

        assert!(
            result.is_err(),
            "Should NOT be able to list skill directory when skill tool disabled"
        );
    }
}
