use crate::{Templates, edit_agent::EditAgent, schema::json_schema_for};
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, AnyToolCard, Tool, ToolCard, ToolResult, ToolUseStatus};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorMode, MultiBuffer, PathKey};
use gpui::{
    AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, EntityId, Task, WeakEntity,
};
use language::{
    Anchor, Buffer, Capability, LanguageRegistry, LineEnding, OffsetRangeExt, Rope, TextBuffer,
};
use language_model::{
    LanguageModelRegistry, LanguageModelRequestMessage, LanguageModelToolSchemaFormat,
};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ui::{Disclosure, Tooltip, Window, prelude::*};
use util::ResultExt;
use workspace::Workspace;

pub struct EditFileTool;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolInput {
    /// A user-friendly markdown description of the edit. This will be shown in the UI.
    ///
    /// <example>Fix API endpoint URLs</example>
    /// <example>Update copyright year in `page_footer`</example>
    ///
    /// Make sure to include this field before all the others in the input object
    /// so that we can display it immediately.
    pub display_description: String,

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
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct PartialInput {
    #[serde(default)]
    path: String,
    #[serde(default)]
    display_description: String,
}

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
        messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<EditFileToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!(
                "Path {} not found in project",
                input.path.display()
            )))
            .into();
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

        let card = window.and_then(|window| {
            window
                .update(cx, |_, window, cx| {
                    cx.new(|cx| {
                        EditFileToolCard::new(input.path.clone(), project.clone(), window, cx)
                    })
                })
                .ok()
        });

        let card_clone = card.clone();
        // todo!("read model from settings...")
        let models = LanguageModelRegistry::read_global(cx);
        let model = models
            .available_models(cx)
            .find(|model| model.id().0 == "claude-3-7-sonnet-latest")
            .unwrap();
        let provider = models.provider(&model.provider_id()).unwrap();
        let authenticated = provider.authenticate(cx);
        let messages = messages.to_vec();

        // todo!("reuse templates")
        let edit_agent = EditAgent::new(model, action_log, Templates::new());
        let task = cx.spawn(async move |cx: &mut AsyncApp| {
            authenticated.await?;
            if !exists.await? {
                return Err(anyhow!("{} not found", input.path.display()));
            }

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            edit_agent
                .edit(
                    buffer.clone(),
                    input.display_description.clone(),
                    messages,
                    cx,
                )
                .await?;
            let new_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            project
                .update(cx, |project, cx| project.save_buffer(buffer, cx))?
                .await?;

            let old_text = cx.background_spawn({
                let old_snapshot = old_snapshot.clone();
                async move { old_snapshot.text() }
            });
            let new_text = cx.background_spawn({
                let new_snapshot = new_snapshot.clone();
                async move { new_snapshot.text() }
            });
            let diff = cx.background_spawn(async move {
                language::unified_diff(&old_snapshot.text(), &new_snapshot.text())
            });
            let (old_text, new_text, diff) = futures::join!(old_text, new_text, diff);

            if let Some(card) = card_clone {
                card.update(cx, |card, cx| {
                    card.set_diff(project_path.path.clone(), old_text, new_text, cx);
                })
                .log_err();
            }

            Ok(format!(
                "Edited {}:\n\n```diff\n{}\n```",
                input.path.display(),
                diff
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
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    project: Entity<Project>,
    diff_task: Option<Task<Result<()>>>,
    preview_expanded: bool,
    full_height_expanded: bool,
    editor_unique_id: EntityId,
}

impl EditFileToolCard {
    fn new(path: PathBuf, project: Entity<Project>, window: &mut Window, cx: &mut App) -> Self {
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
            editor.set_show_gutter(false, cx);
            editor.disable_inline_diagnostics();
            editor.disable_scrolling(cx);
            editor.disable_expand_excerpt_buttons(cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_expand_all_diff_hunks(cx);
            editor
        });
        Self {
            editor_unique_id: editor.entity_id(),
            path,
            project,
            editor,
            multibuffer,
            diff_task: None,
            preview_expanded: true,
            full_height_expanded: false,
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
        status: &ToolUseStatus,
        window: &mut Window,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let failed = matches!(status, ToolUseStatus::Error(_));

        let path_label_button = h_flex()
            .id(("edit-tool-path-label-button", self.editor_unique_id))
            .w_full()
            .max_w_full()
            .px_1()
            .gap_0p5()
            .cursor_pointer()
            .rounded_sm()
            .opacity(0.8)
            .hover(|label| {
                label
                    .opacity(1.)
                    .bg(cx.theme().colors().element_hover.opacity(0.5))
            })
            .tooltip(Tooltip::text("Jump to File"))
            .child(
                h_flex()
                    .child(
                        Icon::new(IconName::Pencil)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        div()
                            .text_size(rems(0.8125))
                            .child(self.path.display().to_string())
                            .ml_1p5()
                            .mr_0p5(),
                    )
                    .child(
                        Icon::new(IconName::ArrowUpRight)
                            .size(IconSize::XSmall)
                            .color(Color::Ignored),
                    ),
            )
            .on_click({
                let path = self.path.clone();
                let workspace = workspace.clone();
                move |_, window, cx| {
                    workspace
                        .update(cx, {
                            |workspace, cx| {
                                let Some(project_path) =
                                    workspace.project().read(cx).find_project_path(&path, cx)
                                else {
                                    return;
                                };
                                let open_task =
                                    workspace.open_path(project_path, None, true, window, cx);
                                window
                                    .spawn(cx, async move |cx| {
                                        let item = open_task.await?;
                                        if let Some(active_editor) = item.downcast::<Editor>() {
                                            active_editor
                                                .update_in(cx, |editor, window, cx| {
                                                    editor.go_to_singleton_buffer_point(
                                                        language::Point::new(0, 0),
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .log_err();
                                        }
                                        anyhow::Ok(())
                                    })
                                    .detach_and_log_err(cx);
                            }
                        })
                        .ok();
                }
            })
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
            .rounded_t_md()
            .when(!failed, |header| header.bg(codeblock_header_bg))
            .child(path_label_button)
            .map(|container| {
                if failed {
                    container.child(
                        Icon::new(IconName::Close)
                            .size(IconSize::Small)
                            .color(Color::Error),
                    )
                } else {
                    container.child(
                        Disclosure::new(
                            ("edit-file-disclosure", self.editor_unique_id),
                            self.preview_expanded,
                        )
                        .opened_icon(IconName::ChevronUp)
                        .closed_icon(IconName::ChevronDown)
                        .on_click(cx.listener(
                            move |this, _event, _window, _cx| {
                                this.preview_expanded = !this.preview_expanded;
                            },
                        )),
                    )
                }
            });

        let editor = self.editor.update(cx, |editor, cx| {
            editor.render(window, cx).into_any_element()
        });

        let (full_height_icon, full_height_tooltip_label) = if self.full_height_expanded {
            (IconName::ChevronUp, "Collapse Code Block")
        } else {
            (IconName::ChevronDown, "Expand Code Block")
        };

        let gradient_overlay = div()
            .absolute()
            .bottom_0()
            .left_0()
            .w_full()
            .h_2_5()
            .rounded_b_lg()
            .bg(gpui::linear_gradient(
                0.,
                gpui::linear_color_stop(cx.theme().colors().editor_background, 0.),
                gpui::linear_color_stop(cx.theme().colors().editor_background.opacity(0.), 1.),
            ));

        let border_color = cx.theme().colors().border.opacity(0.6);

        v_flex()
            .mb_2()
            .border_1()
            .when(failed, |card| card.border_dashed())
            .border_color(border_color)
            .rounded_lg()
            .overflow_hidden()
            .child(codeblock_header)
            .when(!failed && self.preview_expanded, |card| {
                card.child(
                    v_flex()
                        .relative()
                        .overflow_hidden()
                        .border_t_1()
                        .border_color(border_color)
                        .bg(cx.theme().colors().editor_background)
                        .map(|editor_container| {
                            if self.full_height_expanded {
                                editor_container.h_full()
                            } else {
                                editor_container.max_h_64()
                            }
                        })
                        .child(div().pl_1().child(editor))
                        .when(!self.full_height_expanded, |editor_container| {
                            editor_container.child(gradient_overlay)
                        }),
                )
            })
            .when(!failed && self.preview_expanded, |card| {
                card.child(
                    h_flex()
                        .id(("edit-tool-card-inner-hflex", self.editor_unique_id))
                        .flex_none()
                        .cursor_pointer()
                        .h_5()
                        .justify_center()
                        .rounded_b_md()
                        .border_t_1()
                        .border_color(border_color)
                        .bg(cx.theme().colors().editor_background)
                        .hover(|style| style.bg(cx.theme().colors().element_hover.opacity(0.1)))
                        .child(
                            Icon::new(full_height_icon)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .tooltip(Tooltip::text(full_height_tooltip_label))
                        .on_click(cx.listener(move |this, _event, _window, _cx| {
                            this.full_height_expanded = !this.full_height_expanded;
                        })),
                )
            })
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

// todo!("add unit tests for failure modes of edit, like file not found, etc.")
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn still_streaming_ui_text_with_path() {
        let input = json!({
            "path": "src/main.rs",
            "display_description": "",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(EditFileTool.still_streaming_ui_text(&input), "src/main.rs");
    }

    #[test]
    fn still_streaming_ui_text_with_description() {
        let input = json!({
            "path": "",
            "display_description": "Fix error handling",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(
            EditFileTool.still_streaming_ui_text(&input),
            "Fix error handling",
        );
    }

    #[test]
    fn still_streaming_ui_text_with_path_and_description() {
        let input = json!({
            "path": "src/main.rs",
            "display_description": "Fix error handling",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(
            EditFileTool.still_streaming_ui_text(&input),
            "Fix error handling",
        );
    }

    #[test]
    fn still_streaming_ui_text_no_path_or_description() {
        let input = json!({
            "path": "",
            "display_description": "",
            "old_string": "old code",
            "new_string": "new code"
        });

        assert_eq!(
            EditFileTool.still_streaming_ui_text(&input),
            DEFAULT_UI_TEXT,
        );
    }

    #[test]
    fn still_streaming_ui_text_with_null() {
        let input = serde_json::Value::Null;

        assert_eq!(
            EditFileTool.still_streaming_ui_text(&input),
            DEFAULT_UI_TEXT,
        );
    }
}
