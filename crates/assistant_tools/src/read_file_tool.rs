use crate::schema::json_schema_for;
use action_log::ActionLog;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{Tool, ToolResult};
use assistant_tool::{ToolResultContent, outline};
use gpui::{AnyWindowHandle, App, Entity, Task};
use project::{ImageItem, image_store};

use assistant_tool::ToolResultOutput;
use indoc::formatdoc;
use itertools::Itertools;
use language::{Anchor, Point};
use language_model::{
    LanguageModel, LanguageModelImage, LanguageModelRequest, LanguageModelToolSchemaFormat,
};
use project::{AgentLocation, Project, WorktreeSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::sync::Arc;
use ui::IconName;

/// If the model requests to read a file whose size exceeds this, then
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadFileToolInput {
    /// The relative path of the file to read.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a root directory in a project.
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

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> String {
        "read_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &Entity<Project>, _: &App) -> bool {
        false
    }

    fn may_perform_edits(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./read_file_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::ToolRead
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ReadFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<ReadFileToolInput>(input.clone()) {
            Ok(input) => {
                let path = &input.path;
                match (input.start_line, input.end_line) {
                    (Some(start), Some(end)) => {
                        format!(
                            "[Read file `{}` (lines {}-{})](@selection:{}:({}-{}))",
                            path, start, end, path, start, end
                        )
                    }
                    (Some(start), None) => {
                        format!(
                            "[Read file `{}` (from line {})](@selection:{}:({}-{}))",
                            path, start, path, start, start
                        )
                    }
                    _ => format!("[Read file `{}`](@file:{})", path, path),
                }
            }
            Err(_) => "Read file".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<ReadFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path {} not found in project", &input.path))).into();
        };

        // Error out if this path is either excluded or private in global settings
        let global_settings = WorktreeSettings::get_global(cx);
        if global_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the global `file_scan_exclusions` setting: {}",
                &input.path
            )))
            .into();
        }

        if global_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the global `private_files` setting: {}",
                &input.path
            )))
            .into();
        }

        // Error out if this path is either excluded or private in worktree settings
        let worktree_settings = WorktreeSettings::get(Some((&project_path).into()), cx);
        if worktree_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the worktree `file_scan_exclusions` setting: {}",
                &input.path
            )))
            .into();
        }

        if worktree_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches the worktree `private_files` setting: {}",
                &input.path
            )))
            .into();
        }

        let file_path = input.path.clone();

        if image_store::is_image_file(&project, &project_path, cx) {
            if !model.supports_images() {
                return Task::ready(Err(anyhow!(
                    "Attempted to read an image, but Zed doesn't currently support sending images to {}.",
                    model.name().0
                )))
                .into();
            }

            let task = cx.spawn(async move |cx| -> Result<ToolResultOutput> {
                let image_entity: Entity<ImageItem> = cx
                    .update(|cx| {
                        project.update(cx, |project, cx| {
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

                Ok(ToolResultOutput {
                    content: ToolResultContent::Image(language_model_image),
                    output: None,
                })
            });

            return task.into();
        }

        cx.spawn(async move |cx| {
            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;
            if buffer.read_with(cx, |buffer, _| {
                buffer
                    .file()
                    .as_ref()
                    .map_or(true, |file| !file.disk_state().exists())
            })? {
                anyhow::bail!("{file_path} not found");
            }

            project.update(cx, |project, cx| {
                project.set_agent_location(
                    Some(AgentLocation {
                        buffer: buffer.downgrade(),
                        position: Anchor::MIN,
                    }),
                    cx,
                );
            })?;

            // Check if specific line ranges are provided
            if input.start_line.is_some() || input.end_line.is_some() {
                let mut anchor = None;
                let result = buffer.read_with(cx, |buffer, _cx| {
                    let text = buffer.text();
                    // .max(1) because despite instructions to be 1-indexed, sometimes the model passes 0.
                    let start = input.start_line.unwrap_or(1).max(1);
                    let start_row = start - 1;
                    if start_row <= buffer.max_point().row {
                        let column = buffer.line_indent_for_row(start_row).raw_len();
                        anchor = Some(buffer.anchor_before(Point::new(start_row, column)));
                    }

                    let lines = text.split('\n').skip(start_row as usize);
                    if let Some(end) = input.end_line {
                        let count = end.saturating_sub(start).saturating_add(1); // Ensure at least 1 line
                        Itertools::intersperse(lines.take(count as usize), "\n")
                            .collect::<String>()
                            .into()
                    } else {
                        Itertools::intersperse(lines, "\n")
                            .collect::<String>()
                            .into()
                    }
                })?;

                action_log.update(cx, |log, cx| {
                    log.buffer_read(buffer.clone(), cx);
                })?;

                if let Some(anchor) = anchor {
                    project.update(cx, |project, cx| {
                        project.set_agent_location(
                            Some(AgentLocation {
                                buffer: buffer.downgrade(),
                                position: anchor,
                            }),
                            cx,
                        );
                    })?;
                }

                Ok(result)
            } else {
                // No line ranges specified, so check file size to see if it's too big.
                let file_size = buffer.read_with(cx, |buffer, _cx| buffer.text().len())?;

                if file_size <= outline::AUTO_OUTLINE_SIZE {
                    // File is small enough, so return its contents.
                    let result = buffer.read_with(cx, |buffer, _cx| buffer.text())?;

                    action_log.update(cx, |log, cx| {
                        log.buffer_read(buffer, cx);
                    })?;

                    Ok(result.into())
                } else {
                    // File is too big, so return the outline
                    // and a suggestion to read again with line numbers.
                    let outline =
                        outline::file_outline(project, file_path, action_log, None, cx).await?;
                    Ok(formatdoc! {"
                        This file was too big to read all at once.

                        Here is an outline of its symbols:

                        {outline}

                        Using the line numbers in this outline, you can call this tool again
                        while specifying the start_line and end_line fields to see the
                        implementations of symbols in the outline.

                        Alternatively, you can fall back to the `grep` tool (if available)
                        to search the file for specific content."
                    }
                    .into())
                }
            }
        })
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{AppContext, TestAppContext, UpdateGlobal};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use language_model::fake_provider::FakeLanguageModel;
    use project::{FakeFs, Project, WorktreeSettings};
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
        let model = Arc::new(FakeLanguageModel::default());
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/nonexistent_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log,
                        model,
                        None,
                        cx,
                    )
                    .output
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
        let model = Arc::new(FakeLanguageModel::default());
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/small_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log,
                        model,
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert_eq!(
            result.unwrap().content.as_str(),
            Some("This is a small file content")
        );
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
        let model = Arc::new(FakeLanguageModel::default());

        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/large_file.rs"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        let content = result.unwrap();
        let content = content.as_str().unwrap();
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
                let input = json!({
                    "path": "root/large_file.rs",
                    "offset": 1
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log,
                        model,
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        let content = result.unwrap();
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
                .as_str()
                .unwrap()
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
        let model = Arc::new(FakeLanguageModel::default());
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/multiline.txt",
                    "start_line": 2,
                    "end_line": 4
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log,
                        model,
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert_eq!(
            result.unwrap().content.as_str(),
            Some("Line 2\nLine 3\nLine 4")
        );
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
        let model = Arc::new(FakeLanguageModel::default());

        // start_line of 0 should be treated as 1
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/multiline.txt",
                    "start_line": 0,
                    "end_line": 2
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert_eq!(result.unwrap().content.as_str(), Some("Line 1\nLine 2"));

        // end_line of 0 should result in at least 1 line
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/multiline.txt",
                    "start_line": 1,
                    "end_line": 0
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert_eq!(result.unwrap().content.as_str(), Some("Line 1"));

        // when start_line > end_line, should still return at least 1 line
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/multiline.txt",
                    "start_line": 3,
                    "end_line": 2
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log,
                        model,
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert_eq!(result.unwrap().content.as_str(), Some("Line 3"));
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
            use project::WorktreeSettings;
            use settings::SettingsStore;
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings::<WorktreeSettings>(cx, |settings| {
                    settings.file_scan_exclusions = Some(vec![
                        "**/.secretdir".to_string(),
                        "**/.mymetadata".to_string(),
                    ]);
                    settings.private_files = Some(vec![
                        "**/.mysecrets".to_string(),
                        "**/*.privatekey".to_string(),
                        "**/*.mysensitive".to_string(),
                    ]);
                });
            });
        });

        let project = Project::test(fs.clone(), [path!("/project_root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());

        // Reading a file outside the project worktree should fail
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "/outside_project/sensitive_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read an absolute path outside a worktree"
        );

        // Reading a file within the project should succeed
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/allowed_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(
            result.is_ok(),
            "read_file_tool should be able to read files inside worktrees"
        );

        // Reading files that match file_scan_exclusions should fail
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/.secretdir/config"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read files in .secretdir (file_scan_exclusions)"
        );

        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/.mymetadata"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .mymetadata files (file_scan_exclusions)"
        );

        // Reading private files should fail
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/.mysecrets"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .mysecrets (private_files)"
        );

        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/subdir/special.privatekey"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .privatekey files (private_files)"
        );

        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/subdir/data.mysensitive"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(
            result.is_err(),
            "read_file_tool should error when attempting to read .mysensitive files (private_files)"
        );

        // Reading a normal file should still work, even with private_files configured
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/subdir/normal_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
            })
            .await;
        assert!(result.is_ok(), "Should be able to read normal files");
        assert_eq!(
            result.unwrap().content.as_str().unwrap(),
            "Normal file content"
        );

        // Path traversal attempts with .. should fail
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "project_root/../outside_project/sensitive_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(
                        input,
                        Arc::default(),
                        project.clone(),
                        action_log.clone(),
                        model.clone(),
                        None,
                        cx,
                    )
                    .output
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
                store.update_user_settings::<WorktreeSettings>(cx, |settings| {
                    settings.file_scan_exclusions =
                        Some(vec!["**/.git".to_string(), "**/node_modules".to_string()]);
                    settings.private_files = Some(vec!["**/.env".to_string()]);
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
        let model = Arc::new(FakeLanguageModel::default());
        let tool = Arc::new(ReadFileTool);

        // Test reading allowed files in worktree1
        let input = json!({
            "path": "worktree1/src/main.rs"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
            .await
            .unwrap();

        assert_eq!(
            result.content.as_str().unwrap(),
            "fn main() { println!(\"Hello from worktree1\"); }"
        );

        // Test reading private file in worktree1 should fail
        let input = json!({
            "path": "worktree1/src/secret.rs"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
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
        let input = json!({
            "path": "worktree1/tests/fixture.sql"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
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
        let input = json!({
            "path": "worktree2/lib/public.js"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
            .await
            .unwrap();

        assert_eq!(
            result.content.as_str().unwrap(),
            "export function greet() { return 'Hello from worktree2'; }"
        );

        // Test reading private file in worktree2 should fail
        let input = json!({
            "path": "worktree2/lib/private.js"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
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
        let input = json!({
            "path": "worktree2/docs/internal.md"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
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
        let input = json!({
            "path": "worktree1/src/config.toml"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
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
