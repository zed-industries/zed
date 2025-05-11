use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::outline;
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, Entity, Task};

use indoc::formatdoc;
use itertools::Itertools;
use language::{Anchor, Point};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::{AgentLocation, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::IconName;
use util::markdown::MarkdownInlineCode;

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
    /// - directory1
    /// - directory2
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
                let path = MarkdownInlineCode(&input.path);
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
        _request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
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

        let file_path = input.path.clone();
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
                return Err(anyhow!("{} not found", file_path));
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
                        implementations of symbols in the outline."
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
    use gpui::{AppContext, TestAppContext};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use language_model::fake_provider::FakeLanguageModel;
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
            "/root",
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
        assert_eq!(result.unwrap().content, "This is a small file content");
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
            "/root",
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
        assert_eq!(result.unwrap().content, "Line 2\nLine 3\nLine 4");
    }

    #[gpui::test]
    async fn test_read_file_line_range_edge_cases(cx: &mut TestAppContext) {
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
        assert_eq!(result.unwrap().content, "Line 1\nLine 2");

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
        assert_eq!(result.unwrap().content, "Line 1");

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
        assert_eq!(result.unwrap().content, "Line 3");
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
