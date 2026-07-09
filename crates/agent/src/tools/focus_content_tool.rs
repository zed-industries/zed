use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema::v1 as acp;
use anyhow::{Result, anyhow};
use futures::FutureExt as _;
use gpui::{App, Entity, SharedString, Task};
use language::{Anchor, Bias, Buffer, Point};
use project::{AgentContentFocus, AgentLocation, Project, WorktreeSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{ops::Range, sync::Arc};

use super::tool_permissions::{
    ResolvedProjectPath, canonicalize_worktree_roots, resolve_project_path,
};

fn tool_content_err(error: impl std::fmt::Display) -> String {
    error.to_string()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FocusContentAction {
    #[default]
    Focus,
    Clear,
}

/// Focus the user's editor view on a specific file range and optionally point at a precise token or span inside it.
///
/// This is a visual attention tool. Use it when explaining existing code or other editor text and you want the user to look at a section while you discuss it.
/// Prefer `pointer_text` for pointing at a keyword or expression. Use columns only when the target text is ambiguous.
/// Line numbers are 1-based. Columns are 1-based character columns; end columns are exclusive.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FocusContentToolInput {
    /// Whether to focus content or clear the current focus.
    #[serde(default)]
    pub action: FocusContentAction,
    /// The relative path of the file to focus. Required when action is `focus`.
    ///
    /// This path should never be absolute, and the first component of the path should always be a root directory in a project.
    #[serde(default)]
    pub path: Option<String>,
    /// Optional line number to start focusing on (1-based index). Defaults to 1.
    #[serde(default)]
    pub start_line: Option<u32>,
    /// Optional line number to end focusing on (1-based index, inclusive). Defaults to `start_line`.
    #[serde(default)]
    pub end_line: Option<u32>,
    /// Optional character column where the focused range starts (1-based, inclusive).
    #[serde(default)]
    pub start_column: Option<u32>,
    /// Optional character column where the focused range ends (1-based, exclusive).
    #[serde(default)]
    pub end_column: Option<u32>,
    /// Optional line number for the precise pointer target (1-based index). Defaults to `start_line`.
    #[serde(default)]
    pub pointer_line: Option<u32>,
    /// Optional character column for the pointer target (1-based index).
    #[serde(default)]
    pub pointer_column: Option<u32>,
    /// Optional exclusive character column for highlighting a precise pointer span.
    #[serde(default)]
    pub pointer_end_column: Option<u32>,
    /// Optional text to find on `pointer_line` and highlight as the precise pointer span.
    #[serde(default)]
    pub pointer_text: Option<String>,
    /// Which occurrence of `pointer_text` on `pointer_line` to use (1-based). Defaults to 1.
    #[serde(default)]
    pub pointer_occurrence: Option<u32>,
}

pub struct FocusContentTool {
    project: Entity<Project>,
}

impl FocusContentTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

struct ResolvedFocus {
    range: Range<Anchor>,
    pointer: Anchor,
    pointer_range: Option<Range<Anchor>>,
    start_line: u32,
    end_line: u32,
}

fn resolve_line_range(
    start_line: Option<u32>,
    end_line: Option<u32>,
    max_line: u32,
) -> Result<(u32, u32), String> {
    let start = start_line.unwrap_or(1).max(1);
    if start > max_line {
        return Err(format!(
            "start_line {start} is past the end of the file, which has {max_line} lines"
        ));
    }

    let end = end_line.unwrap_or(start).max(start).min(max_line);
    Ok((start, end))
}

fn line_text(buffer: &Buffer, row: u32) -> String {
    buffer
        .text_for_range(Point::new(row, 0)..Point::new(row, buffer.line_len(row)))
        .collect()
}

fn character_column_to_byte_column(line: &str, column: u32) -> u32 {
    let target_character = column.max(1).saturating_sub(1) as usize;
    line.char_indices()
        .nth(target_character)
        .map(|(byte_index, _)| byte_index as u32)
        .unwrap_or(line.len() as u32)
}

fn point_for_character_column(buffer: &Buffer, row: u32, column: u32, bias: Bias) -> Point {
    let line = line_text(buffer, row);
    let byte_column = character_column_to_byte_column(&line, column);
    buffer.clip_point(Point::new(row, byte_column), bias)
}

fn line_end_point(buffer: &Buffer, row: u32) -> Point {
    let max_point = buffer.max_point();
    if row < max_point.row {
        Point::new(row + 1, 0)
    } else {
        Point::new(row, buffer.line_len(row))
    }
}

fn text_occurrence_range(line: &str, text: &str, occurrence: u32) -> Option<Range<u32>> {
    let occurrence_index = occurrence.max(1).saturating_sub(1) as usize;
    line.match_indices(text)
        .nth(occurrence_index)
        .map(|(start, matched)| start as u32..(start + matched.len()) as u32)
}

fn resolve_focus(buffer: &Buffer, input: &FocusContentToolInput) -> Result<ResolvedFocus, String> {
    let max_line = buffer.max_point().row.saturating_add(1).max(1);
    let (start_line, end_line) = resolve_line_range(input.start_line, input.end_line, max_line)?;
    let start_row = start_line - 1;
    let end_row = end_line - 1;

    let start_point = input.start_column.map_or_else(
        || Point::new(start_row, 0),
        |column| point_for_character_column(buffer, start_row, column, Bias::Left),
    );
    let mut end_point = input.end_column.map_or_else(
        || line_end_point(buffer, end_row),
        |column| point_for_character_column(buffer, end_row, column, Bias::Right),
    );
    if end_row == start_row && end_point.column < start_point.column {
        end_point = start_point;
    }

    let pointer_line = input.pointer_line.unwrap_or(start_line).max(1);
    if pointer_line > max_line {
        return Err(format!(
            "pointer_line {pointer_line} is past the end of the file, which has {max_line} lines"
        ));
    }
    let pointer_row = pointer_line - 1;
    if !(start_line..=end_line).contains(&pointer_line) {
        return Err(format!(
            "pointer_line {pointer_line} must be within the focused line range {start_line}-{end_line}"
        ));
    }

    let pointer_range = if let Some(pointer_text) = input.pointer_text.as_deref() {
        if pointer_text.is_empty() {
            None
        } else {
            let line = line_text(buffer, pointer_row);
            let Some(range) =
                text_occurrence_range(&line, pointer_text, input.pointer_occurrence.unwrap_or(1))
            else {
                return Err(format!(
                    "Could not find pointer_text `{pointer_text}` on line {pointer_line}"
                ));
            };
            let start = buffer.clip_point(Point::new(pointer_row, range.start), Bias::Left);
            let end = buffer.clip_point(Point::new(pointer_row, range.end), Bias::Right);
            Some(buffer.anchor_before(start)..buffer.anchor_after(end))
        }
    } else if let Some(pointer_column) = input.pointer_column {
        let start = point_for_character_column(buffer, pointer_row, pointer_column, Bias::Left);
        input
            .pointer_end_column
            .filter(|end_column| *end_column > pointer_column)
            .map(|end_column| {
                let end = point_for_character_column(buffer, pointer_row, end_column, Bias::Right);
                buffer.anchor_before(start)..buffer.anchor_after(end)
            })
    } else {
        None
    };

    let pointer = pointer_range
        .as_ref()
        .map(|range| range.start)
        .unwrap_or_else(|| {
            let point = input.pointer_column.map_or(start_point, |column| {
                point_for_character_column(buffer, pointer_row, column, Bias::Left)
            });
            buffer.anchor_before(point)
        });

    Ok(ResolvedFocus {
        range: buffer.anchor_before(start_point)..buffer.anchor_after(end_point),
        pointer,
        pointer_range,
        start_line,
        end_line,
    })
}

impl AgentTool for FocusContentTool {
    type Input = FocusContentToolInput;
    type Output = String;

    const NAME: &'static str = "focus_content";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) if input.action == FocusContentAction::Clear => {
                "Clear focused content".into()
            }
            Ok(input) => input
                .path
                .as_ref()
                .and_then(|path| {
                    let project = self.project.read(cx);
                    project
                        .find_project_path(path, cx)
                        .and_then(|project_path| {
                            project.short_full_path_for_project_path(&project_path, cx)
                        })
                })
                .map(|path| format!("Focus `{path}`").into())
                .unwrap_or_else(|| "Focus content".into()),
            Err(_) => "Focus content".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(tool_content_err)?;

            if input.action == FocusContentAction::Clear {
                project.update(cx, |project, cx| {
                    project.set_agent_content_focus(None, cx);
                    project.set_agent_location(None, cx);
                });
                event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
                    acp::ToolCallContent::Content(acp::Content::new(
                        "Cleared focused content.",
                    )),
                ]));
                return Ok("Cleared focused content.".to_string());
            }

            let path = input
                .path
                .as_deref()
                .filter(|path| !path.is_empty())
                .ok_or_else(|| "path is required when action is `focus`".to_string())?;

            let fs = project.read_with(cx, |project, _cx| project.fs().clone());
            let canonical_roots = canonicalize_worktree_roots(&project, &fs, cx).await;

            let project_path = project
                .read_with(cx, |project, cx| {
                    let resolved = resolve_project_path(project, path, &canonical_roots, cx)?;
                    match resolved {
                        ResolvedProjectPath::Safe(project_path) => Ok(project_path),
                        ResolvedProjectPath::SymlinkEscape {
                            canonical_target, ..
                        } => Err(anyhow!(
                            "Cannot focus `{path}` because it resolves outside the project to {}",
                            canonical_target.display()
                        )),
                    }
                })
                .map_err(tool_content_err)?;

            project
                .read_with(cx, |_project, cx| {
                    let global_settings = WorktreeSettings::get_global(cx);
                    if global_settings.is_path_excluded(&project_path.path) {
                        anyhow::bail!(
                            "Cannot focus content because its path matches the global `file_scan_exclusions` setting: {path}"
                        );
                    }

                    if global_settings.is_path_private(&project_path.path) {
                        anyhow::bail!(
                            "Cannot focus content because its path matches the global `private_files` setting: {path}"
                        );
                    }

                    let worktree_settings = WorktreeSettings::get(Some((&project_path).into()), cx);
                    if worktree_settings.is_path_excluded(&project_path.path) {
                        anyhow::bail!(
                            "Cannot focus content because its path matches the worktree `file_scan_exclusions` setting: {path}"
                        );
                    }

                    if worktree_settings.is_path_private(&project_path.path) {
                        anyhow::bail!(
                            "Cannot focus content because its path matches the worktree `private_files` setting: {path}"
                        );
                    }

                    anyhow::Ok(())
                })
                .map_err(tool_content_err)?;

            let abs_path = project
                .read_with(cx, |project, cx| project.absolute_path(&project_path, cx))
                .ok_or_else(|| format!("Failed to convert {path} to absolute path"))?;

            if fs.is_dir(&abs_path).await {
                return Err(format!("{path} is a directory, not a file."));
            }

            event_stream.update_fields(acp::ToolCallUpdateFields::new().locations(vec![
                acp::ToolCallLocation::new(&abs_path)
                    .line(input.start_line.map(|line| line.saturating_sub(1))),
            ]));

            let open_buffer_task =
                project.update(cx, |project, cx| project.open_buffer(project_path.clone(), cx));
            let buffer = futures::select! {
                result = open_buffer_task.fuse() => result.map_err(tool_content_err)?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Focus content cancelled by user".to_string());
                }
            };

            if buffer.read_with(cx, |buffer, _| {
                buffer
                    .file()
                    .as_ref()
                    .is_none_or(|file| !file.disk_state().exists())
            }) {
                return Err(format!("{path} not found"));
            }

            let resolved_focus =
                buffer.read_with(cx, |buffer, _cx| resolve_focus(buffer, &input))?;

            project.update(cx, |project, cx| {
                project.set_agent_location(
                    Some(AgentLocation {
                        buffer: buffer.downgrade(),
                        position: resolved_focus.pointer,
                    }),
                    cx,
                );
                project.set_agent_content_focus(
                    Some(AgentContentFocus {
                        buffer: buffer.downgrade(),
                        range: resolved_focus.range.clone(),
                        pointer: resolved_focus.pointer,
                        pointer_range: resolved_focus.pointer_range.clone(),
                    }),
                    cx,
                );
            });

            let output = format!(
                "Focused {path} lines {}-{}.",
                resolved_focus.start_line, resolved_focus.end_line
            );
            event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
                acp::ToolCallContent::Content(acp::Content::new(output.clone())),
            ]));

            Ok(output)
        })
    }

    fn replay(
        &self,
        _input: Self::Input,
        output: Self::Output,
        event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Result<()> {
        event_stream.update_fields(acp::ToolCallUpdateFields::new().content(vec![
            acp::ToolCallContent::Content(acp::Content::new(output)),
        ]));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use language::ToPoint;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_focus_content_sets_project_focus_and_pointer(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "src": {
                    "main.rs": "fn main() {\n    let answer = 42;\n}\n"
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(FocusContentTool::new(project.clone()));
        let project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path("root/src/main.rs", cx)
                .expect("test file should resolve to a project path")
        });
        let opened_buffer = cx
            .update(|cx| project.update(cx, |project, cx| project.open_buffer(project_path, cx)))
            .await
            .expect("test file should open before focus_content runs");

        let result = cx
            .update(|cx| {
                tool.run(
                    ToolInput::resolved(FocusContentToolInput {
                        action: FocusContentAction::Focus,
                        path: Some("root/src/main.rs".to_string()),
                        start_line: Some(2),
                        end_line: Some(2),
                        start_column: None,
                        end_column: None,
                        pointer_line: Some(2),
                        pointer_column: None,
                        pointer_end_column: None,
                        pointer_text: Some("answer".to_string()),
                        pointer_occurrence: None,
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        assert_eq!(
            result.expect("focus_content should focus the requested file"),
            "Focused root/src/main.rs lines 2-2."
        );

        let focus = project.read_with(cx, |project, _| {
            project
                .agent_content_focus()
                .expect("focus_content should set project focus state")
        });
        let buffer = focus
            .buffer
            .upgrade()
            .expect("focused buffer should stay open while project focus is set");
        assert_eq!(buffer.entity_id(), opened_buffer.entity_id());
        let pointer_range = focus
            .pointer_range
            .expect("pointer_text should resolve a pointer range");
        let pointer_points = buffer.read_with(cx, |buffer, _| {
            pointer_range.start.to_point(buffer)..pointer_range.end.to_point(buffer)
        });
        assert_eq!(pointer_points.start, Point::new(1, 8));
        assert_eq!(pointer_points.end, Point::new(1, 14));

        let agent_location = project.read_with(cx, |project, _| {
            project
                .agent_location()
                .expect("focus_content should update the agent location")
        });
        assert_eq!(
            buffer.read_with(cx, |buffer, _| agent_location.position.to_point(buffer)),
            Point::new(1, 8)
        );
    }

    #[gpui::test]
    async fn test_focus_content_clear_removes_project_state(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "file.txt": "hello\n" }))
            .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(FocusContentTool::new(project.clone()));

        cx.update(|cx| {
            tool.clone().run(
                ToolInput::resolved(FocusContentToolInput {
                    action: FocusContentAction::Focus,
                    path: Some("root/file.txt".to_string()),
                    start_line: Some(1),
                    end_line: Some(1),
                    start_column: None,
                    end_column: None,
                    pointer_line: None,
                    pointer_column: None,
                    pointer_end_column: None,
                    pointer_text: None,
                    pointer_occurrence: None,
                }),
                ToolCallEventStream::test().0,
                cx,
            )
        })
        .await
        .expect("focus_content should set state before clear");

        assert!(project.read_with(cx, |project, _| project.agent_content_focus().is_some()));

        let result = cx
            .update(|cx| {
                tool.run(
                    ToolInput::resolved(FocusContentToolInput {
                        action: FocusContentAction::Clear,
                        path: None,
                        start_line: None,
                        end_line: None,
                        start_column: None,
                        end_column: None,
                        pointer_line: None,
                        pointer_column: None,
                        pointer_end_column: None,
                        pointer_text: None,
                        pointer_occurrence: None,
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        assert_eq!(
            result.expect("focus_content clear should succeed"),
            "Cleared focused content."
        );
        assert!(project.read_with(cx, |project, _| project.agent_content_focus().is_none()));
        assert!(project.read_with(cx, |project, _| project.agent_location().is_none()));
    }

    #[gpui::test]
    async fn test_focus_content_rejects_pointer_outside_focused_range(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "file.txt": "one\ntwo\nthree\n" }))
            .await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let tool = Arc::new(FocusContentTool::new(project));

        let result = cx
            .update(|cx| {
                tool.run(
                    ToolInput::resolved(FocusContentToolInput {
                        action: FocusContentAction::Focus,
                        path: Some("root/file.txt".to_string()),
                        start_line: Some(1),
                        end_line: Some(1),
                        start_column: None,
                        end_column: None,
                        pointer_line: Some(2),
                        pointer_column: None,
                        pointer_end_column: None,
                        pointer_text: None,
                        pointer_occurrence: None,
                    }),
                    ToolCallEventStream::test().0,
                    cx,
                )
            })
            .await;

        assert!(
            result
                .expect_err("pointer outside focused range should be rejected")
                .contains("must be within the focused line range")
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }
}
