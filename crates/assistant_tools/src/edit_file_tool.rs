use crate::{
    Templates,
    edit_agent::{EditAgent, EditAgentOutputEvent},
    schema::json_schema_for,
};
use anyhow::{Result, anyhow};
use assistant_tool::{
    ActionLog, AnyToolCard, Tool, ToolCard, ToolResult, ToolResultOutput, ToolUseStatus,
};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorMode, MultiBuffer, PathKey};
use futures::StreamExt;
use gpui::{
    Animation, AnimationExt, AnyWindowHandle, App, AppContext, AsyncApp, Entity, EntityId, Task,
    TextStyleRefinement, WeakEntity, pulsating_between,
};
use indoc::formatdoc;
use language::{
    Anchor, Buffer, Capability, LanguageRegistry, LineEnding, OffsetRangeExt, Rope, TextBuffer,
    language_settings::SoftWrap,
};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::ThemeSettings;
use ui::{Disclosure, Tooltip, prelude::*};
use util::ResultExt;
use workspace::Workspace;

pub struct EditFileTool;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolInput {
    /// A one-line, user-friendly markdown description of the edit. This will be
    /// shown in the UI and also passed to another model to perform the edit.
    ///
    /// Be terse, but also descriptive in what you want to achieve with this
    /// edit. Avoid generic instructions.
    ///
    /// NEVER mention the file path in this description.
    ///
    /// <example>Fix API endpoint URLs</example>
    /// <example>Update copyright year in `page_footer`</example>
    ///
    /// Make sure to include this field before all the others in the input object
    /// so that we can display it immediately.
    pub display_description: String,

    /// The full path of the file to create or modify in the project.
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

    /// If true, this tool will recreate the file from scratch.
    /// If false, this tool will produce granular edits to an existing file.
    ///
    /// When a file already exists or you just created it, always prefer editing
    /// it as opposed to recreating it from scratch.
    pub create_or_overwrite: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolOutput {
    pub original_path: PathBuf,
    pub new_text: String,
    pub old_text: String,
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
        request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        model: Arc<dyn LanguageModel>,
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
        let task = cx.spawn(async move |cx: &mut AsyncApp| {
            let edit_agent = EditAgent::new(model, project.clone(), action_log, Templates::new());

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            let exists = buffer.read_with(cx, |buffer, _| {
                buffer
                    .file()
                    .as_ref()
                    .map_or(false, |file| file.disk_state().exists())
            })?;
            if !input.create_or_overwrite && !exists {
                return Err(anyhow!("{} not found", input.path.display()));
            }

            let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let old_text = cx
                .background_spawn({
                    let old_snapshot = old_snapshot.clone();
                    async move { old_snapshot.text() }
                })
                .await;

            let (output, mut events) = if input.create_or_overwrite {
                edit_agent.overwrite(
                    buffer.clone(),
                    input.display_description.clone(),
                    &request,
                    cx,
                )
            } else {
                edit_agent.edit(
                    buffer.clone(),
                    input.display_description.clone(),
                    &request,
                    cx,
                )
            };

            let mut hallucinated_old_text = false;
            while let Some(event) = events.next().await {
                match event {
                    EditAgentOutputEvent::Edited => {
                        if let Some(card) = card_clone.as_ref() {
                            let new_snapshot =
                                buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
                            let new_text = cx
                                .background_spawn({
                                    let new_snapshot = new_snapshot.clone();
                                    async move { new_snapshot.text() }
                                })
                                .await;
                            card.update(cx, |card, cx| {
                                card.set_diff(
                                    project_path.path.clone(),
                                    old_text.clone(),
                                    new_text,
                                    cx,
                                );
                            })
                            .log_err();
                        }
                    }
                    EditAgentOutputEvent::OldTextNotFound(_) => hallucinated_old_text = true,
                }
            }
            output.await?;

            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                .await?;

            let new_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let new_text = cx.background_spawn({
                let new_snapshot = new_snapshot.clone();
                async move { new_snapshot.text() }
            });
            let diff = cx.background_spawn(async move {
                language::unified_diff(&old_snapshot.text(), &new_snapshot.text())
            });
            let (new_text, diff) = futures::join!(new_text, diff);

            let output = EditFileToolOutput {
                original_path: project_path.path.to_path_buf(),
                new_text: new_text.clone(),
                old_text: old_text.clone(),
            };

            if let Some(card) = card_clone {
                card.update(cx, |card, cx| {
                    card.set_diff(project_path.path.clone(), old_text, new_text, cx);
                })
                .log_err();
            }

            let input_path = input.path.display();
            if diff.is_empty() {
                if hallucinated_old_text {
                    Err(anyhow!(formatdoc! {"
                        Some edits were produced but none of them could be applied.
                        Read the relevant sections of {input_path} again so that
                        I can perform the requested edits.
                    "}))
                } else {
                    Ok("No edits were made.".to_string().into())
                }
            } else {
                Ok(ToolResultOutput {
                    content: format!("Edited {}:\n\n```diff\n{}\n```", input_path, diff),
                    output: serde_json::to_value(output).ok(),
                })
            }
        });

        ToolResult {
            output: task,
            card: card.map(AnyToolCard::from),
        }
    }

    fn deserialize_card(
        self: Arc<Self>,
        output: serde_json::Value,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyToolCard> {
        let output = match serde_json::from_value::<EditFileToolOutput>(output) {
            Ok(output) => output,
            Err(_) => return None,
        };

        let card = cx.new(|cx| {
            let mut card = EditFileToolCard::new(output.original_path.clone(), project, window, cx);
            card.set_diff(
                output.original_path.into(),
                output.old_text,
                output.new_text,
                cx,
            );
            card
        });

        Some(card.into())
    }
}

pub struct EditFileToolCard {
    path: PathBuf,
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    project: Entity<Project>,
    diff_task: Option<Task<Result<()>>>,
    preview_expanded: bool,
    error_expanded: bool,
    full_height_expanded: bool,
    total_lines: Option<u32>,
    editor_unique_id: EntityId,
}

impl EditFileToolCard {
    pub fn new(path: PathBuf, project: Entity<Project>, window: &mut Window, cx: &mut App) -> Self {
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
            editor.set_show_gutter(false, cx);
            editor.disable_inline_diagnostics();
            editor.disable_expand_excerpt_buttons(cx);
            editor.disable_scrollbars_and_minimap(cx);
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.scroll_manager.set_forbid_vertical_scroll(true);
            editor.set_show_indent_guides(false, cx);
            editor.set_read_only(true);
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
            error_expanded: false,
            full_height_expanded: false,
            total_lines: None,
        }
    }

    pub fn has_diff(&self) -> bool {
        self.total_lines.is_some()
    }

    pub fn set_diff(
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
                this.total_lines = this.multibuffer.update(cx, |multibuffer, cx| {
                    let snapshot = buffer.read(cx).snapshot();
                    let diff = buffer_diff.read(cx);
                    let diff_hunk_ranges = diff
                        .hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx)
                        .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot))
                        .collect::<Vec<_>>();
                    multibuffer.clear(cx);
                    multibuffer.set_excerpts_for_path(
                        PathKey::for_buffer(&buffer, cx),
                        buffer,
                        diff_hunk_ranges,
                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
                        cx,
                    );
                    multibuffer.add_diff(buffer_diff, cx);
                    let end = multibuffer.len(cx);
                    Some(multibuffer.snapshot(cx).offset_to_point(end).row + 1)
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
        let (failed, error_message) = match status {
            ToolUseStatus::Error(err) => (true, Some(err.to_string())),
            _ => (false, None),
        };

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
            .when(failed, |header| {
                header.child(
                    h_flex()
                        .gap_1()
                        .child(
                            Icon::new(IconName::Close)
                                .size(IconSize::Small)
                                .color(Color::Error),
                        )
                        .child(
                            Disclosure::new(
                                ("edit-file-error-disclosure", self.editor_unique_id),
                                self.error_expanded,
                            )
                            .opened_icon(IconName::ChevronUp)
                            .closed_icon(IconName::ChevronDown)
                            .on_click(cx.listener(
                                move |this, _event, _window, _cx| {
                                    this.error_expanded = !this.error_expanded;
                                },
                            )),
                        ),
                )
            })
            .when(!failed && self.has_diff(), |header| {
                header.child(
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
            });

        let (editor, editor_line_height) = self.editor.update(cx, |editor, cx| {
            let line_height = editor
                .style()
                .map(|style| style.text.line_height_in_pixels(window.rem_size()))
                .unwrap_or_default();

            editor.set_text_style_refinement(TextStyleRefinement {
                font_size: Some(
                    TextSize::Small
                        .rems()
                        .to_pixels(ThemeSettings::get_global(cx).agent_font_size(cx))
                        .into(),
                ),
                ..TextStyleRefinement::default()
            });
            let element = editor.render(window, cx);
            (element.into_any_element(), line_height)
        });

        let (full_height_icon, full_height_tooltip_label) = if self.full_height_expanded {
            (IconName::ChevronUp, "Collapse Code Block")
        } else {
            (IconName::ChevronDown, "Expand Code Block")
        };

        let gradient_overlay =
            div()
                .absolute()
                .bottom_0()
                .left_0()
                .w_full()
                .h_2_5()
                .bg(gpui::linear_gradient(
                    0.,
                    gpui::linear_color_stop(cx.theme().colors().editor_background, 0.),
                    gpui::linear_color_stop(cx.theme().colors().editor_background.opacity(0.), 1.),
                ));

        let border_color = cx.theme().colors().border.opacity(0.6);

        const DEFAULT_COLLAPSED_LINES: u32 = 10;
        let is_collapsible = self.total_lines.unwrap_or(0) > DEFAULT_COLLAPSED_LINES;

        let waiting_for_diff = {
            let styles = [
                ("w_4_5", (0.1, 0.85), 2000),
                ("w_1_4", (0.2, 0.75), 2200),
                ("w_2_4", (0.15, 0.64), 1900),
                ("w_3_5", (0.25, 0.72), 2300),
                ("w_2_5", (0.3, 0.56), 1800),
            ];

            let mut container = v_flex()
                .p_3()
                .gap_1()
                .border_t_1()
                .rounded_md()
                .border_color(border_color)
                .bg(cx.theme().colors().editor_background);

            for (width_method, pulse_range, duration_ms) in styles.iter() {
                let (min_opacity, max_opacity) = *pulse_range;
                let placeholder = match *width_method {
                    "w_4_5" => div().w_3_4(),
                    "w_1_4" => div().w_1_4(),
                    "w_2_4" => div().w_2_4(),
                    "w_3_5" => div().w_3_5(),
                    "w_2_5" => div().w_2_5(),
                    _ => div().w_1_2(),
                }
                .id("loading_div")
                .h_1()
                .rounded_full()
                .bg(cx.theme().colors().element_active)
                .with_animation(
                    "loading_pulsate",
                    Animation::new(Duration::from_millis(*duration_ms))
                        .repeat()
                        .with_easing(pulsating_between(min_opacity, max_opacity)),
                    |label, delta| label.opacity(delta),
                );

                container = container.child(placeholder);
            }

            container
        };

        v_flex()
            .mb_2()
            .border_1()
            .when(failed, |card| card.border_dashed())
            .border_color(border_color)
            .rounded_md()
            .overflow_hidden()
            .child(codeblock_header)
            .when(failed && self.error_expanded, |card| {
                card.child(
                    v_flex()
                        .p_2()
                        .gap_1()
                        .border_t_1()
                        .border_dashed()
                        .border_color(border_color)
                        .bg(cx.theme().colors().editor_background)
                        .rounded_b_md()
                        .child(
                            Label::new("Error")
                                .size(LabelSize::XSmall)
                                .color(Color::Error),
                        )
                        .child(
                            div()
                                .rounded_md()
                                .text_ui_sm()
                                .bg(cx.theme().colors().editor_background)
                                .children(
                                    error_message
                                        .map(|error| div().child(error).into_any_element()),
                                ),
                        ),
                )
            })
            .when(!self.has_diff() && !failed, |card| {
                card.child(waiting_for_diff)
            })
            .when(
                !failed && self.preview_expanded && self.has_diff(),
                |card| {
                    card.child(
                        v_flex()
                            .relative()
                            .h_full()
                            .when(!self.full_height_expanded, |editor_container| {
                                editor_container
                                    .max_h(DEFAULT_COLLAPSED_LINES as f32 * editor_line_height)
                            })
                            .overflow_hidden()
                            .border_t_1()
                            .border_color(border_color)
                            .bg(cx.theme().colors().editor_background)
                            .child(editor)
                            .when(
                                !self.full_height_expanded && is_collapsible,
                                |editor_container| editor_container.child(gradient_overlay),
                            ),
                    )
                    .when(is_collapsible, |card| {
                        card.child(
                            h_flex()
                                .id(("expand-button", self.editor_unique_id))
                                .flex_none()
                                .cursor_pointer()
                                .h_5()
                                .justify_center()
                                .border_t_1()
                                .rounded_b_md()
                                .border_color(border_color)
                                .bg(cx.theme().colors().editor_background)
                                .hover(|style| {
                                    style.bg(cx.theme().colors().element_hover.opacity(0.1))
                                })
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
                },
            )
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

    let secondary_diff = cx.new(|cx| {
        let mut diff = BufferDiff::new(&buffer, cx);
        diff.set_snapshot(diff_snapshot.clone(), &buffer, cx);
        diff
    })?;

    cx.new(|cx| {
        let mut diff = BufferDiff::new(&buffer.text, cx);
        diff.set_snapshot(diff_snapshot, &buffer, cx);
        diff.set_secondary_diff(secondary_diff);
        diff
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use language_model::fake_provider::FakeLanguageModel;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_edit_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());
        let result = cx
            .update(|cx| {
                let input = serde_json::to_value(EditFileToolInput {
                    display_description: "Some edit".into(),
                    path: "root/nonexistent_file.txt".into(),
                    create_or_overwrite: false,
                })
                .unwrap();
                Arc::new(EditFileTool)
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

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }
}
