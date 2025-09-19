use crate::{
    Templates,
    edit_agent::{EditAgent, EditAgentOutput, EditAgentOutputEvent, EditFormat},
    schema::json_schema_for,
    ui::{COLLAPSED_LINES, ToolOutputPreview},
};
use action_log::ActionLog;
use agent_settings;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{
    AnyToolCard, Tool, ToolCard, ToolResult, ToolResultContent, ToolResultOutput, ToolUseStatus,
};
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{
    Editor, EditorMode, MinimapVisibility, MultiBuffer, PathKey, multibuffer_context_lines,
};
use futures::StreamExt;
use gpui::{
    Animation, AnimationExt, AnyWindowHandle, App, AppContext, AsyncApp, Entity, Task,
    TextStyleRefinement, WeakEntity, pulsating_between, px,
};
use indoc::formatdoc;
use language::{
    Anchor, Buffer, Capability, LanguageRegistry, LineEnding, OffsetRangeExt, Point, Rope,
    TextBuffer,
    language_settings::{self, FormatOnSave, SoftWrap},
};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use paths;
use project::{
    Project, ProjectPath,
    lsp_store::{FormatTrigger, LspFormatTarget},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{
    cmp::Reverse,
    collections::HashSet,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::ThemeSettings;
use ui::{CommonAnimationExt, Disclosure, Tooltip, prelude::*};
use util::ResultExt;
use workspace::Workspace;

pub struct EditFileTool;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
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
    /// - /a/b/backend
    /// - /c/d/frontend
    ///
    /// <example>
    /// `backend/src/main.rs`
    ///
    /// Notice how the file path starts with `backend`. Without that, the path
    /// would be ambiguous and the call would fail!
    /// </example>
    ///
    /// <example>
    /// `frontend/db.js`
    /// </example>
    pub path: PathBuf,

    /// The mode of operation on the file. Possible values:
    /// - 'edit': Make granular edits to an existing file.
    /// - 'create': Create a new file if it doesn't exist.
    /// - 'overwrite': Replace the entire contents of an existing file.
    ///
    /// When a file already exists or you just created it, prefer editing
    /// it as opposed to recreating it from scratch.
    pub mode: EditFileMode,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum EditFileMode {
    Edit,
    Create,
    Overwrite,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditFileToolOutput {
    pub original_path: PathBuf,
    pub new_text: String,
    pub old_text: Arc<String>,
    pub raw_output: Option<EditAgentOutput>,
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

    fn needs_confirmation(
        &self,
        input: &serde_json::Value,
        project: &Entity<Project>,
        cx: &App,
    ) -> bool {
        if agent_settings::AgentSettings::get_global(cx).always_allow_tool_actions {
            return false;
        }

        let Ok(input) = serde_json::from_value::<EditFileToolInput>(input.clone()) else {
            // If it's not valid JSON, it's going to error and confirming won't do anything.
            return false;
        };

        // If any path component matches the local settings folder, then this could affect
        // the editor in ways beyond the project source, so prompt.
        let local_settings_folder = paths::local_settings_folder_relative_path();
        let path = Path::new(&input.path);
        if path
            .components()
            .any(|component| component.as_os_str() == local_settings_folder.as_os_str())
        {
            return true;
        }

        // It's also possible that the global config dir is configured to be inside the project,
        // so check for that edge case too.
        if let Ok(canonical_path) = std::fs::canonicalize(&input.path)
            && canonical_path.starts_with(paths::config_dir())
        {
            return true;
        }

        // Check if path is inside the global config directory
        // First check if it's already inside project - if not, try to canonicalize
        let project_path = project.read(cx).find_project_path(&input.path, cx);

        // If the path is inside the project, and it's not one of the above edge cases,
        // then no confirmation is necessary. Otherwise, confirmation is necessary.
        project_path.is_none()
    }

    fn may_perform_edits(&self) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("edit_file_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::ToolPencil
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<EditFileToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<EditFileToolInput>(input.clone()) {
            Ok(input) => {
                let path = Path::new(&input.path);
                let mut description = input.display_description.clone();

                // Add context about why confirmation may be needed
                let local_settings_folder = paths::local_settings_folder_relative_path();
                if path
                    .components()
                    .any(|c| c.as_os_str() == local_settings_folder.as_os_str())
                {
                    description.push_str(" (local settings)");
                } else if let Ok(canonical_path) = std::fs::canonicalize(&input.path)
                    && canonical_path.starts_with(paths::config_dir())
                {
                    description.push_str(" (global settings)");
                }

                description
            }
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

        let project_path = match resolve_path(&input, project.clone(), cx) {
            Ok(path) => path,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
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
        let action_log_clone = action_log.clone();
        let task = cx.spawn(async move |cx: &mut AsyncApp| {
            let edit_format = EditFormat::from_model(model.clone())?;
            let edit_agent = EditAgent::new(
                model,
                project.clone(),
                action_log_clone,
                Templates::new(),
                edit_format,
            );

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            let old_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let old_text = cx
                .background_spawn({
                    let old_snapshot = old_snapshot.clone();
                    async move { Arc::new(old_snapshot.text()) }
                })
                .await;

            if let Some(card) = card_clone.as_ref() {
                card.update(cx, |card, cx| card.initialize(buffer.clone(), cx))?;
            }

            let (output, mut events) = if matches!(input.mode, EditFileMode::Edit) {
                edit_agent.edit(
                    buffer.clone(),
                    input.display_description.clone(),
                    &request,
                    cx,
                )
            } else {
                edit_agent.overwrite(
                    buffer.clone(),
                    input.display_description.clone(),
                    &request,
                    cx,
                )
            };

            let mut hallucinated_old_text = false;
            let mut ambiguous_ranges = Vec::new();
            while let Some(event) = events.next().await {
                match event {
                    EditAgentOutputEvent::Edited { .. } => {
                        if let Some(card) = card_clone.as_ref() {
                            card.update(cx, |card, cx| card.update_diff(cx))?;
                        }
                    }
                    EditAgentOutputEvent::UnresolvedEditRange => hallucinated_old_text = true,
                    EditAgentOutputEvent::AmbiguousEditRange(ranges) => ambiguous_ranges = ranges,
                    EditAgentOutputEvent::ResolvingEditRange(range) => {
                        if let Some(card) = card_clone.as_ref() {
                            card.update(cx, |card, cx| card.reveal_range(range, cx))?;
                        }
                    }
                }
            }
            let agent_output = output.await?;

            // If format_on_save is enabled, format the buffer
            let format_on_save_enabled = buffer
                .read_with(cx, |buffer, cx| {
                    let settings = language_settings::language_settings(
                        buffer.language().map(|l| l.name()),
                        buffer.file(),
                        cx,
                    );
                    !matches!(settings.format_on_save, FormatOnSave::Off)
                })
                .unwrap_or(false);

            if format_on_save_enabled {
                action_log.update(cx, |log, cx| {
                    log.buffer_edited(buffer.clone(), cx);
                })?;
                let format_task = project.update(cx, |project, cx| {
                    project.format(
                        HashSet::from_iter([buffer.clone()]),
                        LspFormatTarget::Buffers,
                        false, // Don't push to history since the tool did it.
                        FormatTrigger::Save,
                        cx,
                    )
                })?;
                format_task.await.log_err();
            }

            project
                .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                .await?;

            // Notify the action log that we've edited the buffer (*after* formatting has completed).
            action_log.update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), cx);
            })?;

            let new_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;
            let (new_text, diff) = cx
                .background_spawn({
                    let new_snapshot = new_snapshot.clone();
                    let old_text = old_text.clone();
                    async move {
                        let new_text = new_snapshot.text();
                        let diff = language::unified_diff(&old_text, &new_text);

                        (new_text, diff)
                    }
                })
                .await;

            let output = EditFileToolOutput {
                original_path: project_path.path.to_path_buf(),
                new_text,
                old_text,
                raw_output: Some(agent_output),
            };

            if let Some(card) = card_clone {
                card.update(cx, |card, cx| {
                    card.update_diff(cx);
                    card.finalize(cx)
                })
                .log_err();
            }

            let input_path = input.path.display();
            if diff.is_empty() {
                anyhow::ensure!(
                    !hallucinated_old_text,
                    formatdoc! {"
                        Some edits were produced but none of them could be applied.
                        Read the relevant sections of {input_path} again so that
                        I can perform the requested edits.
                    "}
                );
                anyhow::ensure!(
                    ambiguous_ranges.is_empty(),
                    {
                        let line_numbers = ambiguous_ranges
                            .iter()
                            .map(|range| range.start.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        formatdoc! {"
                            <old_text> matches more than one position in the file (lines: {line_numbers}). Read the
                            relevant sections of {input_path} again and extend <old_text> so
                            that I can perform the requested edits.
                        "}
                    }
                );
                Ok(ToolResultOutput {
                    content: ToolResultContent::Text("No edits were made.".into()),
                    output: serde_json::to_value(output).ok(),
                })
            } else {
                Ok(ToolResultOutput {
                    content: ToolResultContent::Text(format!(
                        "Edited {}:\n\n```diff\n{}\n```",
                        input_path, diff
                    )),
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
            EditFileToolCard::new(output.original_path.clone(), project.clone(), window, cx)
        });

        cx.spawn({
            let path: Arc<Path> = output.original_path.into();
            let language_registry = project.read(cx).languages().clone();
            let card = card.clone();
            async move |cx| {
                let buffer =
                    build_buffer(output.new_text, path.clone(), &language_registry, cx).await?;
                let buffer_diff =
                    build_buffer_diff(output.old_text.clone(), &buffer, &language_registry, cx)
                        .await?;
                card.update(cx, |card, cx| {
                    card.multibuffer.update(cx, |multibuffer, cx| {
                        let snapshot = buffer.read(cx).snapshot();
                        let diff = buffer_diff.read(cx);
                        let diff_hunk_ranges = diff
                            .hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &snapshot, cx)
                            .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot))
                            .collect::<Vec<_>>();

                        multibuffer.set_excerpts_for_path(
                            PathKey::for_buffer(&buffer, cx),
                            buffer,
                            diff_hunk_ranges,
                            multibuffer_context_lines(cx),
                            cx,
                        );
                        multibuffer.add_diff(buffer_diff, cx);
                        let end = multibuffer.len(cx);
                        card.total_lines =
                            Some(multibuffer.snapshot(cx).offset_to_point(end).row + 1);
                    });

                    cx.notify();
                })?;
                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);

        Some(card.into())
    }
}

/// Validate that the file path is valid, meaning:
///
/// - For `edit` and `overwrite`, the path must point to an existing file.
/// - For `create`, the file must not already exist, but it's parent dir must exist.
fn resolve_path(
    input: &EditFileToolInput,
    project: Entity<Project>,
    cx: &mut App,
) -> Result<ProjectPath> {
    let project = project.read(cx);

    match input.mode {
        EditFileMode::Edit | EditFileMode::Overwrite => {
            let path = project
                .find_project_path(&input.path, cx)
                .context("Can't edit file: path not found")?;

            let entry = project
                .entry_for_path(&path, cx)
                .context("Can't edit file: path not found")?;

            anyhow::ensure!(entry.is_file(), "Can't edit file: path is a directory");
            Ok(path)
        }

        EditFileMode::Create => {
            if let Some(path) = project.find_project_path(&input.path, cx) {
                anyhow::ensure!(
                    project.entry_for_path(&path, cx).is_none(),
                    "Can't create file: file already exists"
                );
            }

            let parent_path = input
                .path
                .parent()
                .context("Can't create file: incorrect path")?;

            let parent_project_path = project.find_project_path(&parent_path, cx);

            let parent_entry = parent_project_path
                .as_ref()
                .and_then(|path| project.entry_for_path(path, cx))
                .context("Can't create file: parent directory doesn't exist")?;

            anyhow::ensure!(
                parent_entry.is_dir(),
                "Can't create file: parent is not a directory"
            );

            let file_name = input
                .path
                .file_name()
                .context("Can't create file: invalid filename")?;

            let new_file_path = parent_project_path.map(|parent| ProjectPath {
                path: Arc::from(parent.path.join(file_name)),
                ..parent
            });

            new_file_path.context("Can't create file")
        }
    }
}

pub struct EditFileToolCard {
    path: PathBuf,
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    project: Entity<Project>,
    buffer: Option<Entity<Buffer>>,
    base_text: Option<Arc<String>>,
    buffer_diff: Option<Entity<BufferDiff>>,
    revealed_ranges: Vec<Range<Anchor>>,
    diff_task: Option<Task<Result<()>>>,
    preview_expanded: bool,
    error_expanded: Option<Entity<Markdown>>,
    full_height_expanded: bool,
    total_lines: Option<u32>,
}

impl EditFileToolCard {
    pub fn new(path: PathBuf, project: Entity<Project>, window: &mut Window, cx: &mut App) -> Self {
        let expand_edit_card = agent_settings::AgentSettings::get_global(cx).expand_edit_card;
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
            // Keep horizontal scrollbar so user can scroll horizontally if needed
            editor.set_show_vertical_scrollbar(false, cx);
            editor.set_minimap_visibility(MinimapVisibility::Disabled, window, cx);
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
            path,
            project,
            editor,
            multibuffer,
            buffer: None,
            base_text: None,
            buffer_diff: None,
            revealed_ranges: Vec::new(),
            diff_task: None,
            preview_expanded: true,
            error_expanded: None,
            full_height_expanded: expand_edit_card,
            total_lines: None,
        }
    }

    pub fn initialize(&mut self, buffer: Entity<Buffer>, cx: &mut App) {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let base_text = buffer_snapshot.text();
        let language_registry = buffer.read(cx).language_registry();
        let text_snapshot = buffer.read(cx).text_snapshot();

        // Create a buffer diff with the current text as the base
        let buffer_diff = cx.new(|cx| {
            let mut diff = BufferDiff::new(&text_snapshot, cx);
            let _ = diff.set_base_text(
                buffer_snapshot.clone(),
                language_registry,
                text_snapshot,
                cx,
            );
            diff
        });

        self.buffer = Some(buffer);
        self.base_text = Some(base_text.into());
        self.buffer_diff = Some(buffer_diff.clone());

        // Add the diff to the multibuffer
        self.multibuffer
            .update(cx, |multibuffer, cx| multibuffer.add_diff(buffer_diff, cx));
    }

    pub fn is_loading(&self) -> bool {
        self.total_lines.is_none()
    }

    pub fn update_diff(&mut self, cx: &mut Context<Self>) {
        let Some(buffer) = self.buffer.as_ref() else {
            return;
        };
        let Some(buffer_diff) = self.buffer_diff.as_ref() else {
            return;
        };

        let buffer = buffer.clone();
        let buffer_diff = buffer_diff.clone();
        let base_text = self.base_text.clone();
        self.diff_task = Some(cx.spawn(async move |this, cx| {
            let text_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot())?;
            let diff_snapshot = BufferDiff::update_diff(
                buffer_diff.clone(),
                text_snapshot.clone(),
                base_text,
                false,
                false,
                None,
                None,
                cx,
            )
            .await?;
            buffer_diff.update(cx, |diff, cx| {
                diff.set_snapshot(diff_snapshot, &text_snapshot, cx)
            })?;
            this.update(cx, |this, cx| this.update_visible_ranges(cx))
        }));
    }

    pub fn reveal_range(&mut self, range: Range<Anchor>, cx: &mut Context<Self>) {
        self.revealed_ranges.push(range);
        self.update_visible_ranges(cx);
    }

    fn update_visible_ranges(&mut self, cx: &mut Context<Self>) {
        let Some(buffer) = self.buffer.as_ref() else {
            return;
        };

        let ranges = self.excerpt_ranges(cx);
        self.total_lines = self.multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                PathKey::for_buffer(buffer, cx),
                buffer.clone(),
                ranges,
                multibuffer_context_lines(cx),
                cx,
            );
            let end = multibuffer.len(cx);
            Some(multibuffer.snapshot(cx).offset_to_point(end).row + 1)
        });
        cx.notify();
    }

    fn excerpt_ranges(&self, cx: &App) -> Vec<Range<Point>> {
        let Some(buffer) = self.buffer.as_ref() else {
            return Vec::new();
        };
        let Some(diff) = self.buffer_diff.as_ref() else {
            return Vec::new();
        };

        let buffer = buffer.read(cx);
        let diff = diff.read(cx);
        let mut ranges = diff
            .hunks_intersecting_range(Anchor::MIN..Anchor::MAX, buffer, cx)
            .map(|diff_hunk| diff_hunk.buffer_range.to_point(buffer))
            .collect::<Vec<_>>();
        ranges.extend(
            self.revealed_ranges
                .iter()
                .map(|range| range.to_point(buffer)),
        );
        ranges.sort_unstable_by_key(|range| (range.start, Reverse(range.end)));

        // Merge adjacent ranges
        let mut ranges = ranges.into_iter().peekable();
        let mut merged_ranges = Vec::new();
        while let Some(mut range) = ranges.next() {
            while let Some(next_range) = ranges.peek() {
                if range.end >= next_range.start {
                    range.end = range.end.max(next_range.end);
                    ranges.next();
                } else {
                    break;
                }
            }

            merged_ranges.push(range);
        }
        merged_ranges
    }

    pub fn finalize(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let ranges = self.excerpt_ranges(cx);
        let buffer = self.buffer.take().context("card was already finalized")?;
        let base_text = self
            .base_text
            .take()
            .context("card was already finalized")?;
        let language_registry = self.project.read(cx).languages().clone();

        // Replace the buffer in the multibuffer with the snapshot
        let buffer = cx.new(|cx| {
            let language = buffer.read(cx).language().cloned();
            let buffer = TextBuffer::new_normalized(
                0,
                cx.entity_id().as_non_zero_u64().into(),
                buffer.read(cx).line_ending(),
                buffer.read(cx).as_rope().clone(),
            );
            let mut buffer = Buffer::build(buffer, None, Capability::ReadWrite);
            buffer.set_language(language, cx);
            buffer
        });

        let buffer_diff = cx.spawn({
            let buffer = buffer.clone();
            async move |_this, cx| {
                build_buffer_diff(base_text, &buffer, &language_registry, cx).await
            }
        });

        cx.spawn(async move |this, cx| {
            let buffer_diff = buffer_diff.await?;
            this.update(cx, |this, cx| {
                this.multibuffer.update(cx, |multibuffer, cx| {
                    let path_key = PathKey::for_buffer(&buffer, cx);
                    multibuffer.clear(cx);
                    multibuffer.set_excerpts_for_path(
                        path_key,
                        buffer,
                        ranges,
                        multibuffer_context_lines(cx),
                        cx,
                    );
                    multibuffer.add_diff(buffer_diff.clone(), cx);
                });

                cx.notify();
            })
        })
        .detach_and_log_err(cx);
        Ok(())
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
        let error_message = match status {
            ToolUseStatus::Error(err) => Some(err),
            _ => None,
        };

        let running_or_pending = match status {
            ToolUseStatus::Running | ToolUseStatus::Pending => Some(()),
            _ => None,
        };

        let should_show_loading = running_or_pending.is_some() && !self.full_height_expanded;

        let path_label_button = h_flex()
            .id(("edit-tool-path-label-button", self.editor.entity_id()))
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
                        Icon::new(IconName::ToolPencil)
                            .size(IconSize::Small)
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
                            .size(IconSize::Small)
                            .color(Color::Ignored),
                    ),
            )
            .on_click({
                let path = self.path.clone();
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
                                                    let snapshot =
                                                        editor.buffer().read(cx).snapshot(cx);
                                                    let first_hunk = editor
                                                        .diff_hunks_in_ranges(
                                                            &[editor::Anchor::min()
                                                                ..editor::Anchor::max()],
                                                            &snapshot,
                                                        )
                                                        .next();
                                                    if let Some(first_hunk) = first_hunk {
                                                        let first_hunk_start =
                                                            first_hunk.multi_buffer_range().start;
                                                        editor.change_selections(
                                                            Default::default(),
                                                            window,
                                                            cx,
                                                            |selections| {
                                                                selections.select_anchor_ranges([
                                                                    first_hunk_start
                                                                        ..first_hunk_start,
                                                                ]);
                                                            },
                                                        )
                                                    }
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
            .when(error_message.is_none(), |header| {
                header.bg(codeblock_header_bg)
            })
            .child(path_label_button)
            .when(should_show_loading, |header| {
                header.pr_1p5().child(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::XSmall)
                        .color(Color::Info)
                        .with_rotate_animation(2),
                )
            })
            .when_some(error_message, |header, error_message| {
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
                                ("edit-file-error-disclosure", self.editor.entity_id()),
                                self.error_expanded.is_some(),
                            )
                            .opened_icon(IconName::ChevronUp)
                            .closed_icon(IconName::ChevronDown)
                            .on_click(cx.listener({
                                let error_message = error_message.clone();

                                move |this, _event, _window, cx| {
                                    if this.error_expanded.is_some() {
                                        this.error_expanded.take();
                                    } else {
                                        this.error_expanded = Some(cx.new(|cx| {
                                            Markdown::new(error_message.clone(), None, None, cx)
                                        }))
                                    }
                                    cx.notify();
                                }
                            })),
                        ),
                )
            })
            .when(error_message.is_none() && !self.is_loading(), |header| {
                header.child(
                    Disclosure::new(
                        ("edit-file-disclosure", self.editor.entity_id()),
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
                        .rems(cx)
                        .to_pixels(ThemeSettings::get_global(cx).agent_font_size(cx))
                        .into(),
                ),
                ..TextStyleRefinement::default()
            });
            let element = editor.render(window, cx);
            (element.into_any_element(), line_height)
        });

        let border_color = cx.theme().colors().border.opacity(0.6);

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
                .rounded_b_md()
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
            .when(error_message.is_some(), |card| card.border_dashed())
            .border_color(border_color)
            .rounded_md()
            .overflow_hidden()
            .child(codeblock_header)
            .when_some(self.error_expanded.as_ref(), |card, error_markdown| {
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
                                .text_ui_sm(cx)
                                .bg(cx.theme().colors().editor_background)
                                .child(MarkdownElement::new(
                                    error_markdown.clone(),
                                    markdown_style(window, cx),
                                )),
                        ),
                )
            })
            .when(self.is_loading() && error_message.is_none(), |card| {
                card.child(waiting_for_diff)
            })
            .when(self.preview_expanded && !self.is_loading(), |card| {
                let editor_view = v_flex()
                    .relative()
                    .h_full()
                    .when(!self.full_height_expanded, |editor_container| {
                        editor_container.max_h(px(COLLAPSED_LINES as f32 * editor_line_height.0))
                    })
                    .overflow_hidden()
                    .border_t_1()
                    .border_color(border_color)
                    .bg(cx.theme().colors().editor_background)
                    .child(editor);

                card.child(
                    ToolOutputPreview::new(editor_view.into_any_element(), self.editor.entity_id())
                        .with_total_lines(self.total_lines.unwrap_or(0) as usize)
                        .toggle_state(self.full_height_expanded)
                        .with_collapsed_fade()
                        .on_toggle({
                            let this = cx.entity().downgrade();
                            move |is_expanded, _window, cx| {
                                if let Some(this) = this.upgrade() {
                                    this.update(cx, |this, _cx| {
                                        this.full_height_expanded = is_expanded;
                                    });
                                }
                            }
                        }),
                )
            })
    }
}

fn markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let ui_font_size = TextSize::Default.rems(cx);
    let mut text_style = window.text_style();

    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(ui_font_size.into()),
        color: Some(cx.theme().colors().text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        selection_background_color: cx.theme().colors().element_selection_background,
        ..Default::default()
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
    old_text: Arc<String>,
    buffer: &Entity<Buffer>,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    let buffer = cx.update(|cx| buffer.read(cx).snapshot())?;

    let old_text_rope = cx
        .background_spawn({
            let old_text = old_text.clone();
            async move { Rope::from(old_text.as_str()) }
        })
        .await;
    let base_buffer = cx
        .update(|cx| {
            Buffer::build_snapshot(
                old_text_rope,
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
                Some(old_text),
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
    use ::fs::Fs;
    use client::TelemetrySettings;
    use gpui::{TestAppContext, UpdateGlobal};
    use language_model::fake_provider::FakeLanguageModel;
    use serde_json::json;
    use settings::SettingsStore;
    use std::fs;
    use util::path;

    #[gpui::test]
    async fn test_edit_nonexistent_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());
        let result = cx
            .update(|cx| {
                let input = serde_json::to_value(EditFileToolInput {
                    display_description: "Some edit".into(),
                    path: "root/nonexistent_file.txt".into(),
                    mode: EditFileMode::Edit,
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
            "Can't edit file: path not found"
        );
    }

    #[gpui::test]
    async fn test_resolve_path_for_creating_file(cx: &mut TestAppContext) {
        let mode = &EditFileMode::Create;

        let result = test_resolve_path(mode, "root/new.txt", cx);
        assert_resolved_path_eq(result.await, "new.txt");

        let result = test_resolve_path(mode, "new.txt", cx);
        assert_resolved_path_eq(result.await, "new.txt");

        let result = test_resolve_path(mode, "dir/new.txt", cx);
        assert_resolved_path_eq(result.await, "dir/new.txt");

        let result = test_resolve_path(mode, "root/dir/subdir/existing.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't create file: file already exists"
        );

        let result = test_resolve_path(mode, "root/dir/nonexistent_dir/new.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't create file: parent directory doesn't exist"
        );
    }

    #[gpui::test]
    async fn test_resolve_path_for_editing_file(cx: &mut TestAppContext) {
        let mode = &EditFileMode::Edit;

        let path_with_root = "root/dir/subdir/existing.txt";
        let path_without_root = "dir/subdir/existing.txt";
        let result = test_resolve_path(mode, path_with_root, cx);
        assert_resolved_path_eq(result.await, path_without_root);

        let result = test_resolve_path(mode, path_without_root, cx);
        assert_resolved_path_eq(result.await, path_without_root);

        let result = test_resolve_path(mode, "root/nonexistent.txt", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't edit file: path not found"
        );

        let result = test_resolve_path(mode, "root/dir", cx);
        assert_eq!(
            result.await.unwrap_err().to_string(),
            "Can't edit file: path is a directory"
        );
    }

    async fn test_resolve_path(
        mode: &EditFileMode,
        path: &str,
        cx: &mut TestAppContext,
    ) -> anyhow::Result<ProjectPath> {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({
                "dir": {
                    "subdir": {
                        "existing.txt": "hello"
                    }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        let input = EditFileToolInput {
            display_description: "Some edit".into(),
            path: path.into(),
            mode: mode.clone(),
        };

        cx.update(|cx| resolve_path(&input, project, cx))
    }

    fn assert_resolved_path_eq(path: anyhow::Result<ProjectPath>, expected: &str) {
        let actual = path
            .expect("Should return valid path")
            .path
            .to_str()
            .unwrap()
            .replace("\\", "/"); // Naive Windows paths normalization
        assert_eq!(actual, expected);
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
            TelemetrySettings::register(cx);
            agent_settings::AgentSettings::register(cx);
            Project::init_settings(cx);
        });
    }

    fn init_test_with_config(cx: &mut TestAppContext, data_dir: &Path) {
        cx.update(|cx| {
            paths::set_custom_data_dir(data_dir.to_str().unwrap());
            // Set custom data directory (config will be under data_dir/config)

            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            TelemetrySettings::register(cx);
            agent_settings::AgentSettings::register(cx);
            Project::init_settings(cx);
        });
    }

    #[gpui::test]
    async fn test_format_on_save(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        // Set up a Rust language with LSP formatting support
        let rust_language = Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Rust".into(),
                matcher: language::LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        // Register the language and fake LSP
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_language);

        let mut fake_language_servers = language_registry.register_fake_lsp(
            "Rust",
            language::FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    document_formatting_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        // Create the file
        fs.save(
            path!("/root/src/main.rs").as_ref(),
            &"initial content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        // Open the buffer to trigger LSP initialization
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/src/main.rs"), cx)
            })
            .await
            .unwrap();

        // Register the buffer with language servers
        let _handle = project.update(cx, |project, cx| {
            project.register_buffer_with_language_servers(&buffer, cx)
        });

        const UNFORMATTED_CONTENT: &str = "fn main() {println!(\"Hello!\");}\n";
        const FORMATTED_CONTENT: &str =
            "This file was formatted by the fake formatter in the test.\n";

        // Get the fake language server and set up formatting handler
        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.set_request_handler::<lsp::request::Formatting, _, _>({
            |_, _| async move {
                Ok(Some(vec![lsp::TextEdit {
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(1, 0)),
                    new_text: FORMATTED_CONTENT.to_string(),
                }]))
            }
        });

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());

        // First, test with format_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.format_on_save = Some(FormatOnSave::On);
                    settings.project.all_languages.defaults.formatter =
                        Some(language::language_settings::SelectedFormatter::Auto);
                });
            });
        });

        // Have the model stream unformatted content
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = serde_json::to_value(EditFileToolInput {
                    display_description: "Create main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                })
                .unwrap();
                Arc::new(EditFileTool)
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
            });

            // Stream the unformatted content
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(UNFORMATTED_CONTENT.to_string());
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Read the file to verify it was formatted automatically
        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            // Ignore carriage returns on Windows
            new_content.replace("\r\n", "\n"),
            FORMATTED_CONTENT,
            "Code should be formatted when format_on_save is enabled"
        );

        let stale_buffer_count = action_log.read_with(cx, |log, cx| log.stale_buffers(cx).count());

        assert_eq!(
            stale_buffer_count, 0,
            "BUG: Buffer is incorrectly marked as stale after format-on-save. Found {} stale buffers. \
             This causes the agent to think the file was modified externally when it was just formatted.",
            stale_buffer_count
        );

        // Next, test with format_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.format_on_save =
                        Some(FormatOnSave::Off);
                });
            });
        });

        // Stream unformatted edits again
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = serde_json::to_value(EditFileToolInput {
                    display_description: "Update main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                })
                .unwrap();
                Arc::new(EditFileTool)
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
            });

            // Stream the unformatted content
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(UNFORMATTED_CONTENT.to_string());
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Verify the file was not formatted
        let new_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            // Ignore carriage returns on Windows
            new_content.replace("\r\n", "\n"),
            UNFORMATTED_CONTENT,
            "Code should not be formatted when format_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_remove_trailing_whitespace(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"src": {}})).await;

        // Create a simple file with trailing whitespace
        fs.save(
            path!("/root/src/main.rs").as_ref(),
            &"initial content".into(),
            language::LineEnding::Unix,
        )
        .await
        .unwrap();

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());

        // First, test with remove_trailing_whitespace_on_save enabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .remove_trailing_whitespace_on_save = Some(true);
                });
            });
        });

        const CONTENT_WITH_TRAILING_WHITESPACE: &str =
            "fn main() {  \n    println!(\"Hello!\");  \n}\n";

        // Have the model stream content that contains trailing whitespace
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = serde_json::to_value(EditFileToolInput {
                    display_description: "Create main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                })
                .unwrap();
                Arc::new(EditFileTool)
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
            });

            // Stream the content with trailing whitespace
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(
                CONTENT_WITH_TRAILING_WHITESPACE.to_string(),
            );
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Read the file to verify trailing whitespace was removed automatically
        assert_eq!(
            // Ignore carriage returns on Windows
            fs.load(path!("/root/src/main.rs").as_ref())
                .await
                .unwrap()
                .replace("\r\n", "\n"),
            "fn main() {\n    println!(\"Hello!\");\n}\n",
            "Trailing whitespace should be removed when remove_trailing_whitespace_on_save is enabled"
        );

        // Next, test with remove_trailing_whitespace_on_save disabled
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .project
                        .all_languages
                        .defaults
                        .remove_trailing_whitespace_on_save = Some(false);
                });
            });
        });

        // Stream edits again with trailing whitespace
        let edit_result = {
            let edit_task = cx.update(|cx| {
                let input = serde_json::to_value(EditFileToolInput {
                    display_description: "Update main function".into(),
                    path: "root/src/main.rs".into(),
                    mode: EditFileMode::Overwrite,
                })
                .unwrap();
                Arc::new(EditFileTool)
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
            });

            // Stream the content with trailing whitespace
            cx.executor().run_until_parked();
            model.send_last_completion_stream_text_chunk(
                CONTENT_WITH_TRAILING_WHITESPACE.to_string(),
            );
            model.end_last_completion_stream();

            edit_task.await
        };
        assert!(edit_result.is_ok());

        // Wait for any async operations (e.g. formatting) to complete
        cx.executor().run_until_parked();

        // Verify the file still has trailing whitespace
        // Read the file again - it should still have trailing whitespace
        let final_content = fs.load(path!("/root/src/main.rs").as_ref()).await.unwrap();
        assert_eq!(
            // Ignore carriage returns on Windows
            final_content.replace("\r\n", "\n"),
            CONTENT_WITH_TRAILING_WHITESPACE,
            "Trailing whitespace should remain when remove_trailing_whitespace_on_save is disabled"
        );
    }

    #[gpui::test]
    async fn test_needs_confirmation(cx: &mut TestAppContext) {
        init_test(cx);
        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({})).await;

        // Test 1: Path with .zed component should require confirmation
        let input_with_zed = json!({
            "display_description": "Edit settings",
            "path": ".zed/settings.json",
            "mode": "edit"
        });
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        cx.update(|cx| {
            assert!(
                tool.needs_confirmation(&input_with_zed, &project, cx),
                "Path with .zed component should require confirmation"
            );
        });

        // Test 2: Absolute path should require confirmation
        let input_absolute = json!({
            "display_description": "Edit file",
            "path": "/etc/hosts",
            "mode": "edit"
        });
        cx.update(|cx| {
            assert!(
                tool.needs_confirmation(&input_absolute, &project, cx),
                "Absolute path should require confirmation"
            );
        });

        // Test 3: Relative path without .zed should not require confirmation
        let input_relative = json!({
            "display_description": "Edit file",
            "path": "root/src/main.rs",
            "mode": "edit"
        });
        cx.update(|cx| {
            assert!(
                !tool.needs_confirmation(&input_relative, &project, cx),
                "Relative path without .zed should not require confirmation"
            );
        });

        // Test 4: Path with .zed in the middle should require confirmation
        let input_zed_middle = json!({
            "display_description": "Edit settings",
            "path": "root/.zed/tasks.json",
            "mode": "edit"
        });
        cx.update(|cx| {
            assert!(
                tool.needs_confirmation(&input_zed_middle, &project, cx),
                "Path with .zed in any component should require confirmation"
            );
        });

        // Test 5: When always_allow_tool_actions is enabled, no confirmation needed
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.always_allow_tool_actions = true;
            agent_settings::AgentSettings::override_global(settings, cx);

            assert!(
                !tool.needs_confirmation(&input_with_zed, &project, cx),
                "When always_allow_tool_actions is true, no confirmation should be needed"
            );
            assert!(
                !tool.needs_confirmation(&input_absolute, &project, cx),
                "When always_allow_tool_actions is true, no confirmation should be needed for absolute paths"
            );
        });
    }

    #[gpui::test]
    async fn test_ui_text_shows_correct_context(cx: &mut TestAppContext) {
        // Set up a custom config directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        init_test_with_config(cx, temp_dir.path());

        let tool = Arc::new(EditFileTool);

        // Test ui_text shows context for various paths
        let test_cases = vec![
            (
                json!({
                    "display_description": "Update config",
                    "path": ".zed/settings.json",
                    "mode": "edit"
                }),
                "Update config (local settings)",
                ".zed path should show local settings context",
            ),
            (
                json!({
                    "display_description": "Fix bug",
                    "path": "src/.zed/local.json",
                    "mode": "edit"
                }),
                "Fix bug (local settings)",
                "Nested .zed path should show local settings context",
            ),
            (
                json!({
                    "display_description": "Update readme",
                    "path": "README.md",
                    "mode": "edit"
                }),
                "Update readme",
                "Normal path should not show additional context",
            ),
            (
                json!({
                    "display_description": "Edit config",
                    "path": "config.zed",
                    "mode": "edit"
                }),
                "Edit config",
                ".zed as extension should not show context",
            ),
        ];

        for (input, expected_text, description) in test_cases {
            cx.update(|_cx| {
                let ui_text = tool.ui_text(&input);
                assert_eq!(ui_text, expected_text, "Failed for case: {}", description);
            });
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_outside_project(cx: &mut TestAppContext) {
        init_test(cx);
        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());

        // Create a project in /project directory
        fs.insert_tree("/project", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        // Test file outside project requires confirmation
        let input_outside = json!({
            "display_description": "Edit file",
            "path": "/outside/file.txt",
            "mode": "edit"
        });
        cx.update(|cx| {
            assert!(
                tool.needs_confirmation(&input_outside, &project, cx),
                "File outside project should require confirmation"
            );
        });

        // Test file inside project doesn't require confirmation
        let input_inside = json!({
            "display_description": "Edit file",
            "path": "project/file.txt",
            "mode": "edit"
        });
        cx.update(|cx| {
            assert!(
                !tool.needs_confirmation(&input_inside, &project, cx),
                "File inside project should not require confirmation"
            );
        });
    }

    #[gpui::test]
    async fn test_needs_confirmation_config_paths(cx: &mut TestAppContext) {
        // Set up a custom data directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        init_test_with_config(cx, temp_dir.path());

        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/home/user/myproject", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/home/user/myproject").as_ref()], cx).await;

        // Get the actual local settings folder name
        let local_settings_folder = paths::local_settings_folder_relative_path();

        // Test various config path patterns
        let test_cases = vec![
            (
                format!("{}/settings.json", local_settings_folder.display()),
                true,
                "Top-level local settings file".to_string(),
            ),
            (
                format!(
                    "myproject/{}/settings.json",
                    local_settings_folder.display()
                ),
                true,
                "Local settings in project path".to_string(),
            ),
            (
                format!("src/{}/config.toml", local_settings_folder.display()),
                true,
                "Local settings in subdirectory".to_string(),
            ),
            (
                ".zed.backup/file.txt".to_string(),
                true,
                ".zed.backup is outside project".to_string(),
            ),
            (
                "my.zed/file.txt".to_string(),
                true,
                "my.zed is outside project".to_string(),
            ),
            (
                "myproject/src/file.zed".to_string(),
                false,
                ".zed as file extension".to_string(),
            ),
            (
                "myproject/normal/path/file.rs".to_string(),
                false,
                "Normal file without config paths".to_string(),
            ),
        ];

        for (path, should_confirm, description) in test_cases {
            let input = json!({
                "display_description": "Edit file",
                "path": path,
                "mode": "edit"
            });
            cx.update(|cx| {
                assert_eq!(
                    tool.needs_confirmation(&input, &project, cx),
                    should_confirm,
                    "Failed for case: {} - path: {}",
                    description,
                    path
                );
            });
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_global_config(cx: &mut TestAppContext) {
        // Set up a custom data directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        init_test_with_config(cx, temp_dir.path());

        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());

        // Create test files in the global config directory
        let global_config_dir = paths::config_dir();
        fs::create_dir_all(&global_config_dir).unwrap();
        let global_settings_path = global_config_dir.join("settings.json");
        fs::write(&global_settings_path, "{}").unwrap();

        fs.insert_tree("/project", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        // Test global config paths
        let test_cases = vec![
            (
                global_settings_path.to_str().unwrap().to_string(),
                true,
                "Global settings file should require confirmation",
            ),
            (
                global_config_dir
                    .join("keymap.json")
                    .to_str()
                    .unwrap()
                    .to_string(),
                true,
                "Global keymap file should require confirmation",
            ),
            (
                "project/normal_file.rs".to_string(),
                false,
                "Normal project file should not require confirmation",
            ),
        ];

        for (path, should_confirm, description) in test_cases {
            let input = json!({
                "display_description": "Edit file",
                "path": path,
                "mode": "edit"
            });
            cx.update(|cx| {
                assert_eq!(
                    tool.needs_confirmation(&input, &project, cx),
                    should_confirm,
                    "Failed for case: {}",
                    description
                );
            });
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_with_multiple_worktrees(cx: &mut TestAppContext) {
        init_test(cx);
        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());

        // Create multiple worktree directories
        fs.insert_tree(
            "/workspace/frontend",
            json!({
                "src": {
                    "main.js": "console.log('frontend');"
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/workspace/backend",
            json!({
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/workspace/shared",
            json!({
                ".zed": {
                    "settings.json": "{}"
                }
            }),
        )
        .await;

        // Create project with multiple worktrees
        let project = Project::test(
            fs.clone(),
            [
                path!("/workspace/frontend").as_ref(),
                path!("/workspace/backend").as_ref(),
                path!("/workspace/shared").as_ref(),
            ],
            cx,
        )
        .await;

        // Test files in different worktrees
        let test_cases = vec![
            ("frontend/src/main.js", false, "File in first worktree"),
            ("backend/src/main.rs", false, "File in second worktree"),
            (
                "shared/.zed/settings.json",
                true,
                ".zed file in third worktree",
            ),
            ("/etc/hosts", true, "Absolute path outside all worktrees"),
            (
                "../outside/file.txt",
                true,
                "Relative path outside worktrees",
            ),
        ];

        for (path, should_confirm, description) in test_cases {
            let input = json!({
                "display_description": "Edit file",
                "path": path,
                "mode": "edit"
            });
            cx.update(|cx| {
                assert_eq!(
                    tool.needs_confirmation(&input, &project, cx),
                    should_confirm,
                    "Failed for case: {} - path: {}",
                    description,
                    path
                );
            });
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_edge_cases(cx: &mut TestAppContext) {
        init_test(cx);
        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".zed": {
                    "settings.json": "{}"
                },
                "src": {
                    ".zed": {
                        "local.json": "{}"
                    }
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        // Test edge cases
        let test_cases = vec![
            // Empty path - find_project_path returns Some for empty paths
            ("", false, "Empty path is treated as project root"),
            // Root directory
            ("/", true, "Root directory should be outside project"),
            // Parent directory references - find_project_path resolves these
            (
                "project/../other",
                false,
                "Path with .. is resolved by find_project_path",
            ),
            (
                "project/./src/file.rs",
                false,
                "Path with . should work normally",
            ),
            // Windows-style paths (if on Windows)
            #[cfg(target_os = "windows")]
            ("C:\\Windows\\System32\\hosts", true, "Windows system path"),
            #[cfg(target_os = "windows")]
            ("project\\src\\main.rs", false, "Windows-style project path"),
        ];

        for (path, should_confirm, description) in test_cases {
            let input = json!({
                "display_description": "Edit file",
                "path": path,
                "mode": "edit"
            });
            cx.update(|cx| {
                assert_eq!(
                    tool.needs_confirmation(&input, &project, cx),
                    should_confirm,
                    "Failed for case: {} - path: {}",
                    description,
                    path
                );
            });
        }
    }

    #[gpui::test]
    async fn test_ui_text_with_all_path_types(cx: &mut TestAppContext) {
        init_test(cx);
        let tool = Arc::new(EditFileTool);

        // Test UI text for various scenarios
        let test_cases = vec![
            (
                json!({
                    "display_description": "Update config",
                    "path": ".zed/settings.json",
                    "mode": "edit"
                }),
                "Update config (local settings)",
                ".zed path should show local settings context",
            ),
            (
                json!({
                    "display_description": "Fix bug",
                    "path": "src/.zed/local.json",
                    "mode": "edit"
                }),
                "Fix bug (local settings)",
                "Nested .zed path should show local settings context",
            ),
            (
                json!({
                    "display_description": "Update readme",
                    "path": "README.md",
                    "mode": "edit"
                }),
                "Update readme",
                "Normal path should not show additional context",
            ),
            (
                json!({
                    "display_description": "Edit config",
                    "path": "config.zed",
                    "mode": "edit"
                }),
                "Edit config",
                ".zed as extension should not show context",
            ),
        ];

        for (input, expected_text, description) in test_cases {
            cx.update(|_cx| {
                let ui_text = tool.ui_text(&input);
                assert_eq!(ui_text, expected_text, "Failed for case: {}", description);
            });
        }
    }

    #[gpui::test]
    async fn test_needs_confirmation_with_different_modes(cx: &mut TestAppContext) {
        init_test(cx);
        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "existing.txt": "content",
                ".zed": {
                    "settings.json": "{}"
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        // Test different EditFileMode values
        let modes = vec![
            EditFileMode::Edit,
            EditFileMode::Create,
            EditFileMode::Overwrite,
        ];

        for mode in modes {
            // Test .zed path with different modes
            let input_zed = json!({
                "display_description": "Edit settings",
                "path": "project/.zed/settings.json",
                "mode": mode
            });
            cx.update(|cx| {
                assert!(
                    tool.needs_confirmation(&input_zed, &project, cx),
                    ".zed path should require confirmation regardless of mode: {:?}",
                    mode
                );
            });

            // Test outside path with different modes
            let input_outside = json!({
                "display_description": "Edit file",
                "path": "/outside/file.txt",
                "mode": mode
            });
            cx.update(|cx| {
                assert!(
                    tool.needs_confirmation(&input_outside, &project, cx),
                    "Outside path should require confirmation regardless of mode: {:?}",
                    mode
                );
            });

            // Test normal path with different modes
            let input_normal = json!({
                "display_description": "Edit file",
                "path": "project/normal.txt",
                "mode": mode
            });
            cx.update(|cx| {
                assert!(
                    !tool.needs_confirmation(&input_normal, &project, cx),
                    "Normal path should not require confirmation regardless of mode: {:?}",
                    mode
                );
            });
        }
    }

    #[gpui::test]
    async fn test_always_allow_tool_actions_bypasses_all_checks(cx: &mut TestAppContext) {
        // Set up with custom directories for deterministic testing
        let temp_dir = tempfile::tempdir().unwrap();
        init_test_with_config(cx, temp_dir.path());

        let tool = Arc::new(EditFileTool);
        let fs = project::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

        // Enable always_allow_tool_actions
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.always_allow_tool_actions = true;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        // Test that all paths that normally require confirmation are bypassed
        let global_settings_path = paths::config_dir().join("settings.json");
        fs::create_dir_all(paths::config_dir()).unwrap();
        fs::write(&global_settings_path, "{}").unwrap();

        let test_cases = vec![
            ".zed/settings.json",
            "project/.zed/config.toml",
            global_settings_path.to_str().unwrap(),
            "/etc/hosts",
            "/absolute/path/file.txt",
            "../outside/project.txt",
        ];

        for path in test_cases {
            let input = json!({
                "display_description": "Edit file",
                "path": path,
                "mode": "edit"
            });
            cx.update(|cx| {
                assert!(
                    !tool.needs_confirmation(&input, &project, cx),
                    "Path {} should not require confirmation when always_allow_tool_actions is true",
                    path
                );
            });
        }

        // Disable always_allow_tool_actions and verify confirmation is required again
        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.always_allow_tool_actions = false;
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        // Verify .zed path requires confirmation again
        let input = json!({
            "display_description": "Edit file",
            "path": ".zed/settings.json",
            "mode": "edit"
        });
        cx.update(|cx| {
            assert!(
                tool.needs_confirmation(&input, &project, cx),
                ".zed path should require confirmation when always_allow_tool_actions is false"
            );
        });
    }
}
