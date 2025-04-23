use crate::{code_symbols_tool::file_outline, schema::json_schema_for};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{App, Entity, Task};
use indoc::formatdoc;
use itertools::Itertools;
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;
use util::markdown::MarkdownString;

/// If the model requests to read a file whose size exceeds this, then
/// the tool will return an error along with the model's symbol outline,
/// and suggest trying again using line ranges from the outline.
const MAX_FILE_SIZE_TO_READ: usize = 16384;

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
    /// - directory1
    /// - directory2
    ///
    /// If you want to access `file.txt` in `directory1`, you should use the path `directory1/file.txt`.
    /// If you want to access `file.txt` in `directory2`, you should use the path `directory2/file.txt`.
    /// </example>
    pub path: String,

    /// Optional line number to start reading on (1-based index)
    #[serde(default)]
    pub start_line: Option<usize>,

    /// Optional line number to end reading on (1-based index)
    #[serde(default)]
    pub end_line: Option<usize>,
}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> String {
        "read_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./read_file_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::FileSearch
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ReadFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<ReadFileToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownString::inline_code(&input.path);
                match (input.start_line, input.end_line) {
                    (Some(start), None) => format!("Read file {path} (from line {start})"),
                    (Some(start), Some(end)) => format!("Read file {path} (lines {start}-{end})"),
                    _ => format!("Read file {path}"),
                }
            }
            Err(_) => "Read file".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<ReadFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path {} not found in project", &input.path))).into();
        };
        let Some(worktree) = project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("Worktree not found for project path"))).into();
        };
        let exists = worktree.update(cx, |worktree, cx| {
            worktree.file_exists(&project_path.path, cx)
        });

        let file_path = input.path.clone();
        cx.spawn(async move |cx| {
            if !exists.await? {
                return Err(anyhow!("{} not found", file_path))
            }

            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            // Check if specific line ranges are provided
            if input.start_line.is_some() || input.end_line.is_some() {
                let result = buffer.read_with(cx, |buffer, _cx| {
                    let text = buffer.text();
                    let start = input.start_line.unwrap_or(1);
                    let lines = text.split('\n').skip(start - 1);
                    if let Some(end) = input.end_line {
                        let count = end.saturating_sub(start).max(1); // Ensure at least 1 line
                        Itertools::intersperse(lines.take(count), "\n").collect()
                    } else {
                        Itertools::intersperse(lines, "\n").collect()
                    }
                })?;

                action_log.update(cx, |log, cx| {
                    log.track_buffer(buffer, cx);
                })?;

                Ok(result)
            } else {
                // No line ranges specified, so check file size to see if it's too big.
                let file_size = buffer.read_with(cx, |buffer, _cx| buffer.text().len())?;

                if file_size <= MAX_FILE_SIZE_TO_READ {
                    // File is small enough, so return its contents.
                    let result = buffer.read_with(cx, |buffer, _cx| buffer.text())?;

                    action_log.update(cx, |log, cx| {
                        log.track_buffer(buffer, cx);
                    })?;

                    Ok(result)
                } else {
                    // File is too big, so return an error with the outline
                    // and a suggestion to read again with line numbers.
                    let outline = file_outline(project, file_path, action_log, None, cx).await?;
                    Ok(formatdoc! {"
                        This file was too big to read all at once. Here is an outline of its symbols:

                        {outline}

                        Using the line numbers in this outline, you can call this tool again while specifying
                        the start_line and end_line fields to see the implementations of symbols in the outline."
                    })
                }
            }
        }).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_read_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/nonexistent_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(input, &[], project.clone(), action_log, cx)
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
            "/root",
            json!({
                "small_file.txt": "This is a small file content"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/small_file.txt"
                });
                Arc::new(ReadFileTool)
                    .run(input, &[], project.clone(), action_log, cx)
                    .output
            })
            .await;
        assert_eq!(result.unwrap(), "This is a small file content");
    }

    #[gpui::test]
    async fn test_read_large_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "large_file.rs": (0..1000).map(|i| format!("struct Test{} {{\n    a: u32,\n    b: usize,\n}}", i)).collect::<Vec<_>>().join("\n")
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));
        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/large_file.rs"
                });
                Arc::new(ReadFileTool)
                    .run(input, &[], project.clone(), action_log.clone(), cx)
                    .output
            })
            .await;
        let content = result.unwrap();
        assert_eq!(
            content.lines().skip(2).take(6).collect::<Vec<_>>(),
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
                    .run(input, &[], project.clone(), action_log, cx)
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
                .lines()
                .skip(2)
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
            "/root",
            json!({
                "multiline.txt": "Line 1\nLine 2\nLine 3\nLine 4\nLine 5"
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let result = cx
            .update(|cx| {
                let input = json!({
                    "path": "root/multiline.txt",
                    "start_line": 2,
                    "end_line": 4
                });
                Arc::new(ReadFileTool)
                    .run(input, &[], project.clone(), action_log, cx)
                    .output
            })
            .await;
        assert_eq!(result.unwrap(), "Line 2\nLine 3");
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
}
