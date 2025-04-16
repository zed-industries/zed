use crate::{replace::replace_with_flexible_indent, schema::json_schema_for};
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, MultiBuffer, PathKey};
use gpui::{
    AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, IntoElement, Task, Window,
};
use language::{
    self, Anchor, Buffer, Capability, LanguageRegistry, LineEnding, OffsetRangeExt as _, Rope,
    TextBuffer,
};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ui::{Tooltip, prelude::*};
use util::ResultExt;

use crate::replace::replace_exact;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindReplaceFileToolInput {
    /// The path of the file to modify.
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

    /// The unique string to find in the file. This string cannot be empty;
    /// if the string is empty, the tool call will fail. Remember, do not use this tool
    /// to create new files from scratch, or to overwrite existing files! Use a different
    /// approach if you want to do that.
    ///
    /// If this string appears more than once in the file, this tool call will fail,
    /// so it is absolutely critical that you verify ahead of time that the string
    /// is unique. You can search within the file to verify this.
    ///
    /// To make the string more likely to be unique, include a minimum of 3 lines of context
    /// before the string you actually want to find, as well as a minimum of 3 lines of
    /// context after the string you want to find. (These lines of context should appear
    /// in the `replace` string as well.) If 3 lines of context is not enough to obtain
    /// a string that appears only once in the file, then double the number of context lines
    /// until the string becomes unique. (Start with 3 lines before and 3 lines after
    /// though, because too much context is needlessly costly.)
    ///
    /// Do not alter the context lines of code in any way, and make sure to preserve all
    /// whitespace and indentation for all lines of code. This string must be exactly as
    /// it appears in the file, because this tool will do a literal find/replace, and if
    /// even one character in this string is different in any way from how it appears
    /// in the file, then the tool call will fail.
    ///
    /// If you get an error that the `find` string was not found, this means that either
    /// you made a mistake, or that the file has changed since you last looked at it.
    /// Either way, when this happens, you should retry doing this tool call until it
    /// succeeds, up to 3 times. Each time you retry, you should take another look at
    /// the exact text of the file in question, to make sure that you are searching for
    /// exactly the right string. Regardless of whether it was because you made a mistake
    /// or because the file changed since you last looked at it, you should be extra
    /// careful when retrying in this way. It's a bad experience for the user if
    /// this `find` string isn't found, so be super careful to get it exactly right!
    ///
    /// <example>
    /// If a file contains this code:
    ///
    /// ```ignore
    /// fn check_user_permissions(user_id: &str) -> Result<bool> {
    ///     // Check if user exists first
    ///     let user = database.find_user(user_id)?;
    ///
    ///     // This is the part we want to modify
    ///     if user.role == "admin" {
    ///         return Ok(true);
    ///     }
    ///
    ///     // Check other permissions
    ///     check_custom_permissions(user_id)
    /// }
    /// ```
    ///
    /// Your find string should include at least 3 lines of context before and after the part
    /// you want to change:
    ///
    /// ```ignore
    /// fn check_user_permissions(user_id: &str) -> Result<bool> {
    ///     // Check if user exists first
    ///     let user = database.find_user(user_id)?;
    ///
    ///     // This is the part we want to modify
    ///     if user.role == "admin" {
    ///         return Ok(true);
    ///     }
    ///
    ///     // Check other permissions
    ///     check_custom_permissions(user_id)
    /// }
    /// ```
    ///
    /// And your replace string might look like:
    ///
    /// ```ignore
    /// fn check_user_permissions(user_id: &str) -> Result<bool> {
    ///     // Check if user exists first
    ///     let user = database.find_user(user_id)?;
    ///
    ///     // This is the part we want to modify
    ///     if user.role == "admin" || user.role == "superuser" {
    ///         return Ok(true);
    ///     }
    ///
    ///     // Check other permissions
    ///     check_custom_permissions(user_id)
    /// }
    /// ```
    /// </example>
    pub find: String,

    /// The string to replace the one unique occurrence of the find string with.
    pub replace: String,
}

pub struct FindReplaceFileToolCard {
    path: PathBuf,
    description: String,
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    project: Entity<Project>,
    diff_task: Option<Task<Result<()>>>,
}

impl FindReplaceFileToolCard {
    fn new(
        path: PathBuf,
        description: String,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadOnly));
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.disable_inline_diagnostics();
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
                    let _is_newly_added = multibuffer.set_excerpts_for_path(
                        PathKey::for_buffer(&buffer, cx),
                        buffer,
                        diff_hunk_ranges,
                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
                        cx,
                    );
                    multibuffer.add_diff(buffer_diff, cx);
                });
                cx.notify();
            })
        }));
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

impl ToolCard for FindReplaceFileToolCard {
    fn render(
        &mut self,
        status: &ToolUseStatus,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let header = h_flex()
            .id("tool-label-container")
            .gap_1p5()
            .max_w_full()
            .overflow_x_scroll()
            .child(
                Icon::new(IconName::Pencil)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            .child(Label::new("Edit ").size(LabelSize::Small))
            .child(
                div()
                    .size(px(3.))
                    .rounded_full()
                    .bg(cx.theme().colors().text),
            )
            .child(Label::new(self.path.display().to_string()).size(LabelSize::Small))
            .into_any_element();

        let header2 = h_flex()
            .id("code-block-header-label")
            .w_full()
            .max_w_full()
            .px_1()
            .gap_0p5()
            .cursor_pointer()
            .rounded_sm()
            .hover(|item| item.bg(cx.theme().colors().element_hover.opacity(0.5)))
            .tooltip(Tooltip::text("Jump to File"));
        // todo!
        // .child(
        //     h_flex()
        //         .gap_0p5()
        //         .children(
        //             file_icons::FileIcons::get_icon(&path_range.path, cx)
        //                 .map(Icon::from_path)
        //                 .map(|icon| icon.color(Color::Muted).size(IconSize::XSmall)),
        //         )
        //         .child(content)
        //         .child(
        //             Icon::new(IconName::ArrowUpRight)
        //                 .size(IconSize::XSmall)
        //                 .color(Color::Ignored),
        //         ),
        // )
        // .on_click({
        //     let path_range = path_range.clone();
        //     move |_, window, cx| {
        //         workspace
        //             .update(cx, {
        //                 |workspace, cx| {
        //                     if let Some(project_path) = workspace
        //                         .project()
        //                         .read(cx)
        //                         .find_project_path(&path_range.path, cx)
        //                     {
        //                         let target = path_range.range.as_ref().map(|range| {
        //                             Point::new(
        //                                 // Line number is 1-based
        //                                 range.start.line.saturating_sub(1),
        //                                 range.start.col.unwrap_or(0),
        //                             )
        //                         });
        //                         let open_task =
        //                             workspace.open_path(project_path, None, true, window, cx);
        //                         window
        //                             .spawn(cx, async move |cx| {
        //                                 let item = open_task.await?;
        //                                 if let Some(target) = target {
        //                                     if let Some(active_editor) =
        //                                         item.downcast::<Editor>()
        //                                     {
        //                                         active_editor
        //                                             .downgrade()
        //                                             .update_in(cx, |editor, window, cx| {
        //                                                 editor.go_to_singleton_buffer_point(
        //                                                     target, window, cx,
        //                                                 );
        //                                             })
        //                                             .log_err();
        //                                     }
        //                                 }
        //                                 anyhow::Ok(())
        //                             })
        //                             .detach_and_log_err(cx);
        //                     }
        //                 }
        //             })
        //             .ok();
        //     }
        // })
        // .into_any_element();

        let content = match status {
            ToolUseStatus::NeedsConfirmation | ToolUseStatus::Pending | ToolUseStatus::Running => {
                div()
                    // .child(Label::new(&self.description).size(LabelSize::Small))
                    .into_any_element()
            }
            ToolUseStatus::Finished(str) => {
                dbg!(&str);
                self.editor.clone().into_any_element()
            }
            ToolUseStatus::Error(error) => div()
                .child(
                    Label::new(error.to_string())
                        .color(Color::Error)
                        .size(LabelSize::Small),
                )
                .into_any_element(),
        };

        v_flex()
            .my_2()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_sm()
            .gap_1()
            .child(header)
            .child(content)
    }
}

pub struct FindReplaceFileTool;

impl Tool for FindReplaceFileTool {
    fn name(&self) -> String {
        "find_replace_file".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("find_replace_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Pencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<FindReplaceFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<FindReplaceFileToolInput>(input.clone()) {
            Ok(input) => input.display_description,
            Err(_) => "Edit file".to_string(),
        }
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
        let input = match serde_json::from_value::<FindReplaceFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };
        let card = window.and_then(|window| {
            window
                .update(cx, |_, window, cx| {
                    cx.new(|cx| {
                        FindReplaceFileToolCard::new(
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

        let output = cx.spawn({
            let card = card.clone();
            async move |cx: &mut AsyncApp| {
            let project_path = project.read_with(cx, |project, cx| {
                project
                    .find_project_path(&input.path, cx)
                    .context("Path not found in project")
            })??;

            let buffer = project
                .update(cx, |project, cx| project.open_buffer(project_path.clone(), cx))?
                .await?;

            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            if input.find.is_empty() {
                return Err(anyhow!("`find` string cannot be empty. Use a different tool if you want to create a file."));
            }

            if input.find == input.replace {
                return Err(anyhow!("The `find` and `replace` strings are identical, so no changes would be made."));
            }

            let result = cx
                .background_spawn(async move {
                    // Try to match exactly
                    let diff = replace_exact(&input.find, &input.replace, &snapshot)
                    .await
                    // If that fails, try being flexible about indentation
                    .or_else(|| replace_with_flexible_indent(&input.find, &input.replace, &snapshot))?;

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
                            "{} is empty, so the provided `find` string wasn't found.",
                            input.path.display()
                        )
                    } else {
                        anyhow!("Failed to match the provided `find` string")
                    }
                })?;

                return Err(err)
            };

            let snapshot = cx.update(|cx| {
                action_log.update(cx, |log, cx| {
                    log.buffer_read(buffer.clone(), cx)
                });
                let snapshot = buffer.update(cx, |buffer, cx| {
                    buffer.finalize_last_transaction();
                    buffer.apply_diff(diff, cx);
                    buffer.finalize_last_transaction();
                    buffer.snapshot()
                });
                action_log.update(cx, |log, cx| {
                    log.buffer_edited(buffer.clone(), cx)
                });
                snapshot
            })?;

            project.update( cx, |project, cx| {
                project.save_buffer(buffer, cx)
            })?.await?;

            let new_text = snapshot.text();

            let diff_str = cx.background_spawn({
                // todo! probably don't need this
                let old_text = old_text.clone();
                let new_text = new_text.clone();
                async move {
                    language::unified_diff(&old_text, &new_text)
                }
            }).await;

            if let Some(card) = card {
                card.update(cx, |card, cx| {
                    card.set_diff(project_path.path.clone(), old_text, new_text, cx);
                }).log_err();
            }

            Ok(format!("Edited {}:\n\n```diff\n{}\n```", input.path.display(), diff_str))
        }});

        ToolResult {
            output,
            card: card.map(|card| card.into()),
        }
    }
}
