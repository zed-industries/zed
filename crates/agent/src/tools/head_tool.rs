use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, Entity, SharedString, Task};
use project::{Project, WorktreeSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::sync::Arc;
use util::markdown::MarkdownCodeBlock;

/// Reads the first N bytes of a file in the project
///
/// - Useful for quickly previewing the beginning of files
/// - More efficient than reading the entire file when only the start is needed
/// - By default reads the first 1024 bytes
/// - Can be used to check file headers, magic numbers, or initial content
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HeadToolInput {
    /// The relative path of the file to read.
    ///
    /// This path should never be absolute, and the first component of the path should always be a root directory in a project.
    pub path: String,
    /// Number of bytes to read from the beginning of the file. Defaults to 1024.
    #[serde(default = "default_byte_count")]
    pub bytes: u32,
}

fn default_byte_count() -> u32 {
    1024
}

pub struct HeadTool {
    project: Entity<Project>,
}

impl HeadTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for HeadTool {
    type Input = HeadToolInput;
    type Output = String;

    fn name() -> &'static str {
        "head"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => {
                if let Some(project_path) = self.project.read(cx).find_project_path(&input.path, cx)
                    && let Some(path) = self
                        .project
                        .read(cx)
                        .short_full_path_for_project_path(&project_path, cx)
                {
                    format!("Read first {} bytes of `{}`", input.bytes, path)
                } else {
                    format!("Read first {} bytes of file", input.bytes)
                }
            }
            Err(_) => "Read beginning of file".into(),
        }
        .into()
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
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
        let bytes_to_read = input.bytes.max(1) as usize; // Ensure at least 1 byte is read

        event_stream.update_fields(acp::ToolCallUpdateFields {
            locations: Some(vec![acp::ToolCallLocation {
                path: abs_path.clone(),
                line: Some(0),
                meta: None,
            }]),
            ..Default::default()
        });

        let project = self.project.clone();

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

            let result = buffer.read_with(cx, |buffer, _cx| {
                let full_text = buffer.text();
                let total_bytes = full_text.len();
                let bytes_read = bytes_to_read.min(total_bytes);

                let text = if bytes_read < total_bytes {
                    &full_text[..bytes_read]
                } else {
                    &full_text
                };

                if bytes_read < total_bytes {
                    format!("{}\n\n(showing first {} of {} bytes)", text, bytes_read, total_bytes)
                } else {
                    format!("{}\n\n(file has only {} bytes total)", text, total_bytes)
                }
            })?;

            // Update the event stream with formatted content
            let markdown = MarkdownCodeBlock {
                tag: &file_path,
                text: &result,
            }
            .to_string();

            event_stream.update_fields(acp::ToolCallUpdateFields {
                content: Some(vec![acp::ToolCallContent::Content {
                    content: markdown.into(),
                }]),
                ..Default::default()
            });

            Ok(result)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToolCallEventStream;
    use gpui::{TestAppContext, UpdateGlobal};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_head_tool_basic(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "test.txt": "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(HeadTool::new(project.clone()));

        // Test reading first 20 bytes
        let input = HeadToolInput {
            path: "root/test.txt".to_string(),
            bytes: 20,
        };

        let result = cx
            .update(|cx| tool.clone().run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();

        assert!(result.starts_with("Line 1\nLine 2\nLine 3"));
        assert!(result.contains("showing first 20 of"));

        // Test reading first 50 bytes
        let input = HeadToolInput {
            path: "root/test.txt".to_string(),
            bytes: 50,
        };

        let result = cx
            .update(|cx| tool.run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();

        assert!(result.starts_with("Line 1\nLine 2"));
        assert!(result.contains("showing first 50 of"));
    }

    #[gpui::test]
    async fn test_head_tool_small_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "small.txt": "Line 1\nLine 2\nLine 3"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(HeadTool::new(project));

        // Request more bytes than exist
        let input = HeadToolInput {
            path: "root/small.txt".to_string(),
            bytes: 1000,
        };

        let result = cx
            .update(|cx| tool.run(input, ToolCallEventStream::test().0, cx))
            .await
            .unwrap();

        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
        assert!(result.contains("Line 3"));
        assert!(result.contains("file has only"));
    }

    #[gpui::test]
    async fn test_head_tool_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({})).await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(HeadTool::new(project));

        let input = HeadToolInput {
            path: "root/nonexistent.txt".to_string(),
            bytes: 1024,
        };

        let result = cx
            .update(|cx| tool.run(input, ToolCallEventStream::test().0, cx))
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "root/nonexistent.txt not found"
        );
    }

    #[gpui::test]
    async fn test_head_tool_security(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/"),
            json!({
                "project_root": {
                    "allowed.txt": "This is allowed",
                    ".secret": "SECRET_KEY=abc123",
                    "private.key": "private key content"
                },
                "outside": {
                    "sensitive.txt": "Outside project"
                }
            }),
        )
        .await;

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_exclusions = Some(vec!["**/.secret".to_string()]);
                    settings.project.worktree.private_files = Some(vec!["**/*.key".to_string()].into());
                });
            });
        });

        let project = Project::test(fs.clone(), [path!("/project_root").as_ref()], cx).await;
        let tool = Arc::new(HeadTool::new(project));

        // Reading allowed file should succeed
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    HeadToolInput {
                        path: "project_root/allowed.txt".to_string(),
                        bytes: 1024,
                    },
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(result.is_ok());

        // Reading excluded file should fail
        let result = cx
            .update(|cx| {
                tool.clone().run(
                    HeadToolInput {
                        path: "project_root/.secret".to_string(),
                        bytes: 1024,
                    },
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(result.is_err());

        // Reading private file should fail
        let result = cx
            .update(|cx| {
                tool.run(
                    HeadToolInput {
                        path: "project_root/private.key".to_string(),
                        bytes: 1024,
                    },
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;
        assert!(result.is_err());
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }
}
