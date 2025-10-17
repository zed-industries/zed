use action_log::ActionLog;
use agent_client_protocol::{self as acp, ToolCallUpdateFields};
use anyhow::{Context as _, Result, anyhow};
use gpui::{App, Entity, SharedString, Task};
use indoc::formatdoc;
use language::Point;
use language_model::{LanguageModelImage, LanguageModelToolResultContent};
use project::{AgentLocation, ImageItem, Project, WorktreeSettings, image_store};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::sync::Arc;
use util::markdown::MarkdownCodeBlock;

use crate::{AgentTool, ToolCallEventStream, outline};

/// Reads the content of the given file in the project.
///
/// - Never attempt to read a path that hasn't been previously mentioned.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadFileToolInput {
    /// The relative path of the file to read.
    ///
    /// This path should never be absolute, and the first component of the path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - /a/b/directory1
    /// - /c/d/directory2
    ///
    /// If you want to access `file.txt` in `directory1`, you should use the path `directory1/file.txt`.
    /// If you want to access `file.txt` in `directory2`, you should use the path `directory2/file.txt`.
    /// </example>
    pub path: String,
    /// Optional line number to start reading on (1-based index)
    #[serde(default)]
    pub start_line: Option<u32>,
    /// Optional line number to end reading on (1-based index, inclusive)
    #[serde(default)]
    pub end_line: Option<u32>,
}

pub struct ReadFileTool {
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
}

impl ReadFileTool {
    pub fn new(project: Entity<Project>, action_log: Entity<ActionLog>) -> Self {
        Self {
            project,
            action_log,
        }
    }
}

impl AgentTool for ReadFileTool {
    type Input = ReadFileToolInput;
    type Output = LanguageModelToolResultContent;

    fn name() -> &'static str {
        "read_file"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input
            && let Some(project_path) = self.project.read(cx).find_project_path(&input.path, cx)
            && let Some(path) = self
                .project
                .read(cx)
                .short_full_path_for_project_path(&project_path, cx)
        {
            match (input.start_line, input.end_line) {
                (Some(start), Some(end)) => {
                    format!("Read file `{path}` (lines {}-{})", start, end,)
                }
                (Some(start), None) => {
                    format!("Read file `{path}` (from line {})", start)
                }
                _ => format!("Read file `{path}`"),
            }
            .into()
        } else {
            "Read file".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<LanguageModelToolResultContent>> {
        let Some(project_path) = self.project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path {} not found in project", &input.path)));
        };
        let Some(abs_path) = self.project.read(cx).absolute_path(&project_path, cx) else {
            return Task::ready(Err(anyhow!(
                "Failed to convert {} to absolute path",
                &input.path
            )));
        };

        // Error out if this path is either excluded or private in global settings
        let global_settings = WorktreeSettings::get_global(cx);
        if global_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the global `file_scan_exclusions` setting: {}",
                &input.path
            )));
        }

        if global_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the global `private_files` setting: {}",
                &input.path
            )));
        }

        // Error out if this path is either excluded or private in worktree settings
        let worktree_settings = WorktreeSettings::get(Some((&project_path).into()), cx);
        if worktree_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the worktree `file_scan_exclusions` setting: {}",
                &input.path
            )));
        }

        if worktree_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the worktree `private_files` setting: {}",
                &input.path
            )));
        }

        let file_path = input.path.clone();

        event_stream.update_fields(ToolCallUpdateFields {
            locations: Some(vec![acp::ToolCallLocation {
                path: abs_path.clone(),
                line: input.start_line.map(|line| line.saturating_sub(1)),
                meta: None,
            }]),
            ..Default::default()
        });

        if image_store::is_image_file(&self.project, &project_path, cx) {
            return cx.spawn(async move |cx| {
                let image_entity: Entity<ImageItem> = cx
                    .update(|cx| {
                        self.project.update(cx, |project, cx| {
                            project.open_image(project_path.clone(), cx)
                        })
                    })?
                    .await?;

                let image =
                    image_entity.read_with(cx, |image_item, _| Arc::clone(&image_item.image))?;

                let language_model_image = cx
                    .update(|cx| LanguageModelImage::from_image(image, cx))?
                    .await
                    .context("processing image")?;

                Ok(language_model_image.into())
            });
        }

        let project = self.project.clone();
        let action_log = self.action_log.clone();

        cx.spawn(async move |cx| {
            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| {
                        project.open_buffer(project_path.clone(), cx)
                    })
                })?
                .await?;
            if buffer.read_with(cx, |buffer, _| {
                buffer
                    .file()
                    .as_ref()
                    .is_none_or(|file| !file.disk_state().exists())
            })? {
                anyhow::bail!("{file_path} not found");
            }

            let mut anchor = None;

            // Check if specific line ranges are provided
            let result = if input.start_line.is_some() || input.end_line.is_some() {
                let result = buffer.read_with(cx, |buffer, _cx| {
                    // .max(1) because despite instructions to be 1-indexed, sometimes the model passes 0.
                    let start = input.start_line.unwrap_or(1).max(1);
                    let start_row = start - 1;
                    if start_row <= buffer.max_point().row {
                        let column = buffer.line_indent_for_row(start_row).raw_len();
                        anchor = Some(buffer.anchor_before(Point::new(start_row, column)));
                    }

                    let mut end_row = input.end_line.unwrap_or(u32::MAX);
                    if end_row <= start_row {
                        end_row = start_row + 1; // read at least one lines
                    }
                    let start = buffer.anchor_before(Point::new(start_row, 0));
                    let end = buffer.anchor_before(Point::new(end_row, 0));
                    buffer.text_for_range(start..end).collect::<String>()
                })?;

                action_log.update(cx, |log, cx| {
                    log.buffer_read(buffer.clone(), cx);
                })?;

                Ok(result.into())
            } else {
                // No line ranges specified, so check file size to see if it's too big.
                let buffer_content = outline::get_buffer_content_or_outline(
                    buffer.clone(),
                    Some(&abs_path.to_string_lossy()),
                    cx,
                )
                .await?;

                action_log.update(cx, |log, cx| {
                    log.buffer_read(buffer.clone(), cx);
                })?;

                if buffer_content.is_outline {
                    Ok(formatdoc! {"
                        This file was too big to read all at once.

                        {}

                        Using the line numbers in this outline, you can call this tool again
                        while specifying the start_line and end_line fields to see the
                        implementations of symbols in the outline.

                        Alternatively, you can fall back to the `grep` tool (if available)
                        to search the file for specific content.", buffer_content.text
                    }
                    .into())
                } else {
                    Ok(buffer_content.text.into())
                }
            };

            project.update(cx, |project, cx| {
                project.set_agent_location(
                    Some(AgentLocation {
                        buffer: buffer.downgrade(),
                        position: anchor.unwrap_or(text::Anchor::MIN),
                    }),
                    cx,
                );
                if let Ok(LanguageModelToolResultContent::Text(text)) = &result {
                    let markdown = MarkdownCodeBlock {
                        tag: &input.path,
                        text,
                    }
                    .to_string();
                    event_stream.update_fields(ToolCallUpdateFields {
                        content: Some(vec![acp::ToolCallContent::Content {
                            content: markdown.into(),
                        }]),
                        ..Default::default()
                    })
                }
            })?;

            result
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{AppContext, TestAppContext, UpdateGlobal as _};
    use language::{Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_read_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(ReadFileTool::new(project, action_log));
        let (event_stream, _) = ToolCallEventStream::test();

        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/nonexistent_file.txt".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.run(input, event_stream, cx)
            })
            .await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "root/nonexistent_file.txt not found"
        );
    }

    #[gpui::test]
    async fn test_read_small_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "small_file.txt": "This is a small file content"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(ReadFileTool::new(project, action_log));
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/small_file.txt".into(),
                    start_line: None,
                    end_line: None,
                };
                tool.run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert_eq!(result.unwrap(), "This is a small file content".into());
    }

    #[gpui::test]
    async fn test_read_large_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "large_file.rs": (0..1000).map(|i| format!("struct Test{} {{\n    a: u32,\n    b: usize,\n}}", i)).collect::<Vec<_>>().join("\n")
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(ReadFileTool::new(project, action_log));
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/large_file.rs".into(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await
            .unwrap();
        let content = result.to_str().unwrap();

        assert_eq!(
            content.lines().skip(4).take(6).collect::<Vec<_>>(),
            vec![
                "struct Test0 [L1-4]",
                " a [L2]",
                " b [L3]",
                "struct Test1 [L5-8]",
                " a [L6]",
                " b [L7]",
            ]
        );

        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/large_file.rs".into(),
                    start_line: None,
                    end_line: None,
                };
                tool.run(input, ToolCallEventStream::test().0, cx)
            })
            .await
            .unwrap();
        let content = result.to_str().unwrap();
        let expected_content = (0..1000)
            .flat_map(|i| {
                vec![
                    format!("struct Test{} [L{}-{}]", i, i * 4 + 1, i * 4 + 4),
                    format!(" a [L{}]", i * 4 + 2),
                    format!(" b [L{}]", i * 4 + 3),
                ]
            })
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(
            content
                .lines()
                .skip(4)
                .take(expected_content.len())
                .collect::<Vec<_>>(),
            expected_content
        );
    }

    #[gpui::test]
    async fn test_read_file_with_line_range(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "multiline.txt": "Line 1\nLine 2\nLine 3\nLine 4\nLine 5"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(ReadFileTool::new(project, action_log));
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/multiline.txt".to_string(),
                    start_line: Some(2),
                    end_line: Some(4),
                };
                tool.run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert_eq!(result.unwrap(), "Line 2\nLine 3\nLine 4\n".into());
    }

    #[gpui::test]
    async fn test_read_file_line_range_edge_cases(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "multiline.txt": "Line 1\nLine 2\nLine 3\nLine 4\nLine 5"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(ReadFileTool::new(project, action_log));

        // start_line of 0 should be treated as 1
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/multiline.txt".to_string(),
                    start_line: Some(0),
                    end_line: Some(2),
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert_eq!(result.unwrap(), "Line 1\nLine 2\n".into());

        // end_line of 0 should result in at least 1 line
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/multiline.txt".to_string(),
                    start_line: Some(1),
                    end_line: Some(0),
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert_eq!(result.unwrap(), "Line 1\n".into());

        // when start_line > end_line, should still return at least 1 line
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "root/multiline.txt".to_string(),
                    start_line: Some(3),
                    end_line: Some(2),
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert_eq!(result.unwrap(), "Line 3\n".into());
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(
            r#"
            (line_comment) @annotation

            (struct_item
                "struct" @context
                name: (_) @name) @item
            (enum_item
                "enum" @context
                name: (_) @name) @item
            (enum_variant
                name: (_) @name) @item
            (field_declaration
                name: (_) @name) @item
            (impl_item
                "impl" @context
                trait: (_)? @name
                "for"? @context
                type: (_) @name
                body: (_ "{" (_)* "}")) @item
            (function_item
                "fn" @context
                name: (_) @name) @item
            (mod_item
                "mod" @context
                name: (_) @name) @item
            "#,
        )
        .unwrap()
    }

    #[gpui::test]
    async fn test_read_file_security(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            path!("/"),
            json!({
                "project_root": {
                    "allowed_file.txt": "This file is in the project",
                    ".mysecrets": "SECRET_KEY=abc123",
                    ".secretdir": {
                        "config": "special configuration"
                    },
                    ".mymetadata": "custom metadata",
                    "subdir": {
                        "normal_file.txt": "Normal file content",
                        "special.privatekey": "private key content",
                        "data.mysensitive": "sensitive data"
                    }
                },
                "outside_project": {
                    "sensitive_file.txt": "This file is outside the project"
                }
            }),
        )
        .await;

        cx.update(|cx| {
            use gpui::UpdateGlobal;
            use settings::SettingsStore;
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_exclusions = Some(vec![
                        "**/.secretdir".to_string(),
                        "**/.mymetadata".to_string(),
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

        let project = Project::test(fs.clone(), [path!("/project_root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(ReadFileTool::new(project, action_log));

        // Reading a file outside the project worktree should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "/outside_project/sensitive_file.txt".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read an absolute path outside a worktree"
        );

        // Reading a file within the project should succeed
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/allowed_file.txt".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_ok(),
            "read_file_tool should be able to read files inside worktrees"
        );

        // Reading files that match file_scan_exclusions should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/.secretdir/config".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read files in .secretdir (file_scan_exclusions)"
        );

        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/.mymetadata".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .mymetadata files (file_scan_exclusions)"
        );

        // Reading private files should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/.mysecrets".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .mysecrets (private_files)"
        );

        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/subdir/special.privatekey".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .privatekey files (private_files)"
        );

        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/subdir/data.mysensitive".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .mysensitive files (private_files)"
        );

        // Reading a normal file should still work, even with private_files configured
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/subdir/normal_file.txt".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(result.is_ok(), "Should be able to read normal files");
        assert_eq!(result.unwrap(), "Normal file content".into());

        // Path traversal attempts with .. should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "project_root/../outside_project/sensitive_file.txt".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.run(input, ToolCallEventStream::test().0, cx)
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read a relative path that resolves to outside a worktree"
        );
    }

    #[gpui::test]
    async fn test_read_file_with_multiple_worktree_settings(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        // Create first worktree with its own private_files setting
        fs.insert_tree(
            path!("/worktree1"),
            json!({
                "src": {
                    "main.rs": "fn main() { println!(\"Hello from worktree1\"); }",
                    "secret.rs": "const API_KEY: &str = \"secret_key_1\";",
                    "config.toml": "[database]\nurl = \"postgres://localhost/db1\""
                },
                "tests": {
                    "test.rs": "mod tests { fn test_it() {} }",
                    "fixture.sql": "CREATE TABLE users (id INT, name VARCHAR(255));"
                },
                ".zed": {
                    "settings.json": r#"{
                        "file_scan_exclusions": ["**/fixture.*"],
                        "private_files": ["**/secret.rs", "**/config.toml"]
                    }"#
                }
            }),
        )
        .await;

        // Create second worktree with different private_files setting
        fs.insert_tree(
            path!("/worktree2"),
            json!({
                "lib": {
                    "public.js": "export function greet() { return 'Hello from worktree2'; }",
                    "private.js": "const SECRET_TOKEN = \"private_token_2\";",
                    "data.json": "{\"api_key\": \"json_secret_key\"}"
                },
                "docs": {
                    "README.md": "# Public Documentation",
                    "internal.md": "# Internal Secrets and Configuration"
                },
                ".zed": {
                    "settings.json": r#"{
                        "file_scan_exclusions": ["**/internal.*"],
                        "private_files": ["**/private.js", "**/data.json"]
                    }"#
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

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let tool = Arc::new(ReadFileTool::new(project.clone(), action_log.clone()));

        // Test reading allowed files in worktree1
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "worktree1/src/main.rs".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await
            .unwrap();

        assert_eq!(
            result,
            "fn main() { println!(\"Hello from worktree1\"); }".into()
        );

        // Test reading private file in worktree1 should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "worktree1/src/secret.rs".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("worktree `private_files` setting"),
            "Error should mention worktree private_files setting"
        );

        // Test reading excluded file in worktree1 should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "worktree1/tests/fixture.sql".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("worktree `file_scan_exclusions` setting"),
            "Error should mention worktree file_scan_exclusions setting"
        );

        // Test reading allowed files in worktree2
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "worktree2/lib/public.js".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await
            .unwrap();

        assert_eq!(
            result,
            "export function greet() { return 'Hello from worktree2'; }".into()
        );

        // Test reading private file in worktree2 should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "worktree2/lib/private.js".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("worktree `private_files` setting"),
            "Error should mention worktree private_files setting"
        );

        // Test reading excluded file in worktree2 should fail
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "worktree2/docs/internal.md".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("worktree `file_scan_exclusions` setting"),
            "Error should mention worktree file_scan_exclusions setting"
        );

        // Test that files allowed in one worktree but not in another are handled correctly
        // (e.g., config.toml is private in worktree1 but doesn't exist in worktree2)
        let result = cx
            .update(|cx| {
                let input = ReadFileToolInput {
                    path: "worktree1/src/config.toml".to_string(),
                    start_line: None,
                    end_line: None,
                };
                tool.clone().run(input, ToolCallEventStream::test().0, cx)
            })
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("worktree `private_files` setting"),
            "Config.toml should be blocked by worktree1's private_files setting"
        );
    }
}
