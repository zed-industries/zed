use crate::{
    replace::{replace_exact, replace_with_flexible_indent},
    schema::json_schema_for,
};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, AnyToolCard, Tool, ToolCard, ToolResult, ToolUseStatus};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorMode, MultiBuffer, PathKey};
use gpui::{AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, Task};
use language::{
    Anchor, Buffer, Capability, LanguageRegistry, LineEnding, OffsetRangeExt, Rope, TextBuffer,
};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ui::{Disclosure, IconName, Tooltip, Window, prelude::*};
use util::ResultExt;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolInput {
    /// The full path of the file to modify in the project.
    ///
    /// WARNING: When specifying which file path need changing, you MUST
    /// start each path with one of the project's root directories.
    ///
    /// The following examples assume we have two root directories in the project:
    /// - backend
    /// - frontend
    ///
    /// <example>
    /// `backend/src/main.rs`
    ///
    /// Notice how the file path starts with root-1. Without that, the path
    /// would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// `frontend/db.js`
    /// </example>
    pub path: PathBuf,

    /// A user-friendly markdown description of what's being replaced. This will be shown in the UI.
    ///
    /// <example>Fix API endpoint URLs</example>
    /// <example>Update copyright year in `page_footer`</example>
    pub display_description: String,

    /// The text to replace.
    pub old_string: String,

    /// The text to replace it with.
    pub new_string: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct PartialInput {
    #[serde(default)]
    path: String,
    #[serde(default)]
    display_description: String,
    #[serde(default)]
    old_string: String,
    #[serde(default)]
    new_string: String,
}

pub struct EditFileTool;

const DEFAULT_UI_TEXT: &str = "Editing file";

impl Tool for EditFileTool {
    fn name(&self) -> String {
        "edit_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("edit_file_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Pencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<EditFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<EditFileToolInput>(input.clone()) {
            Ok(input) => input.display_description,
            Err(_) => "Editing file".to_string(),
        }
    }

    fn still_streaming_ui_text(&self, input: &serde_json::Value) -> String {
        if let Some(input) = serde_json::from_value::<PartialInput>(input.clone()).ok() {
            let description = input.display_description.trim();
            if !description.is_empty() {
                return description.to_string();
            }

            let path = input.path.trim();
            if !path.is_empty() {
                return path.to_string();
            }
        }

        DEFAULT_UI_TEXT.to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<EditFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let card = window.and_then(|window| {
            window
                .update(cx, |_, window, cx| {
                    cx.new(|cx| {
                        EditFileToolCard::new(
                            input.path.clone(),
                            input.display_description.clone(),
                            project.clone(),
                            window,
                            cx,
                        )
                    })
                })
                .ok()
        });

        let card_clone = card.clone();
        let task = cx.spawn(async move |cx: &mut AsyncApp| {
            let project_path = project.read_with(cx, |project, cx| {
                project
                    .find_project_path(&input.path, cx)
                    .context("Path not found in project")
            })??;

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            if input.old_string.is_empty() {
                return Err(anyhow!(
                    "`old_string` can't be empty, use another tool if you want to create a file."
                ));
            }

            if input.old_string == input.new_string {
                return Err(anyhow!(
                    "The `old_string` and `new_string` are identical, so no changes would be made."
                ));
            }

            let result = cx
                .background_spawn(async move {
                    // Try to match exactly
                    let diff = replace_exact(&input.old_string, &input.new_string, &snapshot)
                        .await
                        // If that fails, try being flexible about indentation
                        .or_else(|| {
                            replace_with_flexible_indent(
                                &input.old_string,
                                &input.new_string,
                                &snapshot,
                            )
                        })?;

                    if diff.edits.is_empty() {
                        return None;
                    }

                    let old_text = snapshot.text();

                    Some((old_text, diff))
                })
                .await;

            let Some((old_text, diff)) = result else {
                let err = buffer.read_with(cx, |buffer, _cx| {
                    let file_exists = buffer
                        .file()
                        .map_or(false, |file| file.disk_state().exists());

                    if !file_exists {
                        anyhow!("{} does not exist", input.path.display())
                    } else if buffer.is_empty() {
                        anyhow!(
                            "{} is empty, so the provided `old_string` wasn't found.",
                            input.path.display()
                        )
                    } else {
                        anyhow!("Failed to match the provided `old_string`")
                    }
                })?;

                return Err(err);
            };

            let snapshot = cx.update(|cx| {
                action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
                let snapshot = buffer.update(cx, |buffer, cx| {
                    buffer.finalize_last_transaction();
                    buffer.apply_diff(diff, cx);
                    buffer.finalize_last_transaction();
                    buffer.snapshot()
                });
                action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
                snapshot
            })?;

            project
                .update(cx, |project, cx| project.save_buffer(buffer, cx))?
                .await?;

            let new_text = snapshot.text();
            let diff_str = cx
                .background_spawn({
                    let old_text = old_text.clone();
                    let new_text = new_text.clone();
                    async move { language::unified_diff(&old_text, &new_text) }
                })
                .await;

            if let Some(card) = card_clone {
                card.update(cx, |card, cx| {
                    card.set_diff(project_path.path.clone(), old_text, new_text, cx);
                })
                .log_err();
            }

            Ok(format!(
                "Edited {}:\n\n```diff\n{}\n```",
                input.path.display(),
                diff_str
            ))
        });

        ToolResult {
            output: task,
            card: card.map(AnyToolCard::from),
        }
    }
}

pub struct EditFileToolCard {
    path: PathBuf,
    description: String,
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    project: Entity<Project>,
    diff_task: Option<Task<Result<()>>>,
    preview: bool,
    full_height: bool,
    index: usize,
}

impl EditFileToolCard {
    thread_local! {
        static NEXT_INDEX: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
    }

    fn next_index() -> usize {
        Self::NEXT_INDEX.with(|cell| {
            let mut index = cell.borrow_mut();
            let current = *index;
            *index = index.wrapping_add(1);
            current
        })
    }

    fn new(
        path: PathBuf,
        description: String,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let multibuffer = cx.new(|_| MultiBuffer::without_headers(Capability::ReadOnly));
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sized_by_content: true,
                },
                multibuffer.clone(),
                Some(project.clone()),
                window,
                cx,
            );
            editor.set_show_scrollbars(false, cx);
            editor.set_show_expand_excerpt_buttons(false, cx);
            editor.set_allow_scrolling(false, cx);
            editor.set_show_gutter(false, cx);
            editor.disable_inline_diagnostics();
            editor.set_show_breakpoints(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_expand_all_diff_hunks(cx);
            editor
        });
        Self {
            path,
            description,
            project,
            editor,
            multibuffer,
            diff_task: None,
            preview: true,
            full_height: false,
            index: Self::next_index(),
        }
    }

    fn set_diff(
        &mut self,
        path: Arc<Path>,
        old_text: String,
        new_text: String,
        cx: &mut Context<Self>,
    ) {
        let language_registry = self.project.read(cx).languages().clone();
        self.diff_task = Some(cx.spawn(async move |this, cx| {
            let buffer = build_buffer(new_text, path.clone(), &language_registry, cx).await?;
            let buffer_diff = build_buffer_diff(old_text, &buffer, &language_registry, cx).await?;

            this.update(cx, |this, cx| {
                this.multibuffer.update(cx, |multibuffer, cx| {
                    let snapshot = buffer.read(cx).snapshot();
                    let diff = buffer_diff.read(cx);
                    let diff_hunk_ranges = diff
                        .hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx)
                        .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot))
                        .collect::<Vec<_>>();
                    let (_, is_newly_added) = multibuffer.set_excerpts_for_path(
                        PathKey::for_buffer(&buffer, cx),
                        buffer,
                        diff_hunk_ranges,
                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
                        cx,
                    );
                    debug_assert!(is_newly_added);
                    multibuffer.add_diff(buffer_diff, cx);
                });
                cx.notify();
            })
        }));
    }
}

impl ToolCard for EditFileToolCard {
    fn render(
        &mut self,
        _status: &ToolUseStatus,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let path_label_button = h_flex()
            .id(("code-block-header-label", self.index))
            .w_full()
            .max_w_full()
            .px_1()
            .gap_0p5()
            .cursor_pointer()
            .rounded_sm()
            .hover(|item| item.bg(cx.theme().colors().element_hover.opacity(0.5)))
            .tooltip(Tooltip::text("Jump to File"))
            .child(
                h_flex()
                    .child(
                        Icon::new(IconName::Pencil)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(self.path.display().to_string())
                            .size(LabelSize::Small)
                            .ml_1p5()
                            .mr_0p5(),
                    )
                    .child(
                        Icon::new(IconName::ArrowUpRight)
                            .size(IconSize::XSmall)
                            .color(Color::Ignored),
                    ),
            )
            .into_any_element();

        let codeblock_header_bg = cx
            .theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025));

        let codeblock_header = h_flex()
            .flex_none()
            .p_1()
            .gap_1()
            .justify_between()
            .bg(codeblock_header_bg)
            .rounded_t_md()
            .when(self.preview, |header| {
                header
                    .border_b_1()
                    .border_color(cx.theme().colors().border.opacity(0.6))
            })
            .child(path_label_button)
            .child(
                Disclosure::new(("edit-file-disclosure", self.index), self.preview)
                    .opened_icon(IconName::ChevronUp)
                    .closed_icon(IconName::ChevronDown)
                    .on_click(cx.listener(move |this, _event, _window, _cx| {
                        this.preview = !this.preview;
                    })),
            );

        let editor = self.editor.update(cx, |editor, cx| {
            editor.render(window, cx).into_any_element()
        });

        v_flex()
            .mb_2()
            .border_1()
            .border_color(cx.theme().colors().border.opacity(0.6))
            .rounded_lg()
            .overflow_hidden()
            .child(codeblock_header)
            .when(self.preview, |card| {
                card.child(
                    div()
                        .relative()
                        .map(|buffer_container| {
                            if self.full_height {
                                buffer_container.h_full()
                            } else {
                                buffer_container.max_h_64()
                            }
                        })
                        .child(editor)
                        .child(
                            h_flex()
                                .id("full_height_button")
                                .absolute()
                                .bottom_0()
                                .h_4()
                                .w_full()
                                .justify_center()
                                .rounded_b_md()
                                .bg(cx.theme().colors().editor_background.opacity(0.8))
                                .hover(|style| style.bg(cx.theme().colors().editor_background))
                                .cursor_pointer()
                                .child(
                                    Icon::new(IconName::ChevronDown)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .on_click(cx.listener(move |this, _event, _window, _cx| {
                                    this.full_height = !this.full_height;
                                })),
                        ),
                )
            })

        // .child(div().pl_2().child(editor))
    }
}

async fn build_buffer(
    mut text: String,
    path: Arc<Path>,
    language_registry: &Arc<language::LanguageRegistry>,
    cx: &mut AsyncApp,
) -> Result<Entity<Buffer>> {
    let line_ending = LineEnding::detect(&text);
    LineEnding::normalize(&mut text);
    let text = Rope::from(text);
    let language = cx
        .update(|_cx| language_registry.language_for_file_path(&path))?
        .await
        .ok();
    let buffer = cx.new(|cx| {
        let buffer = TextBuffer::new_normalized(
            0,
            cx.entity_id().as_non_zero_u64().into(),
            line_ending,
            text,
        );
        let mut buffer = Buffer::build(buffer, None, Capability::ReadWrite);
        buffer.set_language(language, cx);
        buffer
    })?;
    Ok(buffer)
}

async fn build_buffer_diff(
    mut old_text: String,
    buffer: &Entity<Buffer>,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    LineEnding::normalize(&mut old_text);

    let buffer = cx.update(|cx| buffer.read(cx).snapshot())?;

    let base_buffer = cx
        .update(|cx| {
            Buffer::build_snapshot(
                old_text.clone().into(),
                buffer.language().cloned(),
                Some(language_registry.clone()),
                cx,
            )
        })?
        .await;

    let diff_snapshot = cx
        .update(|cx| {
            BufferDiffSnapshot::new_with_base_buffer(
                buffer.text.clone(),
                Some(old_text.into()),
                base_buffer,
                cx,
            )
        })?
        .await;

    cx.new(|cx| {
        let mut diff = BufferDiff::new(&buffer.text, cx);
        diff.set_snapshot(diff_snapshot, &buffer.text, cx);
        diff
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn still_streaming_ui_text_with_path() {
        let tool = EditFileTool;
        let input = json!({
            "path": "src/main.rs",
            "display_description": "",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(tool.still_streaming_ui_text(&input), "src/main.rs");
    }

    #[test]
    fn still_streaming_ui_text_with_description() {
        let tool = EditFileTool;
        let input = json!({
            "path": "",
            "display_description": "Fix error handling",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(tool.still_streaming_ui_text(&input), "Fix error handling");
    }

    #[test]
    fn still_streaming_ui_text_with_path_and_description() {
        let tool = EditFileTool;
        let input = json!({
            "path": "src/main.rs",
            "display_description": "Fix error handling",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(tool.still_streaming_ui_text(&input), "Fix error handling");
    }

    #[test]
    fn still_streaming_ui_text_no_path_or_description() {
        let tool = EditFileTool;
        let input = json!({
            "path": "",
            "display_description": "",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(tool.still_streaming_ui_text(&input), DEFAULT_UI_TEXT);
    }

    #[test]
    fn still_streaming_ui_text_with_null() {
        let tool = EditFileTool;
        let input = serde_json::Value::Null;

        assert_eq!(tool.still_streaming_ui_text(&input), DEFAULT_UI_TEXT);
    }

    #[test]
    fn unique_card_indices() {
        let index1 = EditFileToolCard::next_index();
        let index2 = EditFileToolCard::next_index();
        let index3 = EditFileToolCard::next_index();

        assert_ne!(index1, index2);
        assert_ne!(index2, index3);
        assert_ne!(index1, index3);
    }
}
