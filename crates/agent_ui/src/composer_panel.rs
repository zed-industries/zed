use action_log::ActionLog;
use anyhow::{anyhow, Result};
use gpui::{
    Action, App, AsyncWindowContext, ClickEvent, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Pixels, WeakEntity, Window, actions, prelude::*, px,
};
use language::unified_diff;
use log::error;
use project::Project;
use std::{collections::HashMap, path::PathBuf};
use ui::{
    Button, IconName, IconPosition, Label, LabelSize, ListItem, ListItemSpacing, prelude::*,
};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    notifications::{NotificationId, Toast},
};

actions!(
    composer,
    [ToggleComposer, ApplyAllChanges, RejectAllChanges]
);

struct ComposerApplyToast;
struct ComposerApplyErrorToast;
struct ComposerRejectToast;

/// Represents a file with pending changes in the composer
#[derive(Clone, Debug)]
pub struct ComposerFileEdit {
    /// File path
    pub path: PathBuf,
    /// Original content
    pub original_content: String,
    /// Proposed new content
    pub new_content: String,
    /// Whether this file's changes are staged for application
    pub staged: bool,
    /// Language ID for syntax highlighting
    pub language: Option<String>,
}

impl ComposerFileEdit {
    pub fn new(path: PathBuf, original_content: String, new_content: String) -> Self {
        Self {
            path,
            original_content,
            new_content,
            staged: true,
            language: None,
        }
    }

    /// Get the number of lines added
    pub fn lines_added(&self) -> usize {
        self.new_content
            .lines()
            .count()
            .saturating_sub(self.original_content.lines().count())
    }

    /// Get the number of lines removed
    pub fn lines_removed(&self) -> usize {
        self.original_content
            .lines()
            .count()
            .saturating_sub(self.new_content.lines().count())
    }

    /// Get the file name
    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string()
    }
}

/// The composer panel for multi-file editing
pub struct ComposerPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    action_log: Option<Entity<ActionLog>>,
    /// Pending file edits
    edits: HashMap<PathBuf, ComposerFileEdit>,
    /// Currently selected file for preview
    selected_file: Option<PathBuf>,
    /// Focus handle
    focus_handle: FocusHandle,
    /// Width of the panel
    width: Option<Pixels>,
    /// Whether changes are currently being applied
    applying: bool,
}

impl ComposerPanel {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            workspace,
            project,
            action_log: None,
            edits: HashMap::new(),
            selected_file: None,
            focus_handle: cx.focus_handle(),
            width: Some(px(400.0)),
            applying: false,
        }
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let project = workspace
            .read_with(&cx, |workspace, _| workspace.project().clone())
            .map_err(|err| anyhow!(err))?;
        let workspace_clone = workspace.clone();
        let project_clone = project.clone();
        workspace
            .update_in(&mut cx, move |_, _, cx| {
                cx.new(|cx| ComposerPanel::new(workspace_clone.clone(), project_clone.clone(), cx))
            })
            .map_err(|err| anyhow!(err))
    }

    /// Associate an action log to record applied edits.
    pub fn set_action_log(&mut self, action_log: Option<Entity<ActionLog>>) {
        self.action_log = action_log;
    }

    /// Add a file edit to the composer
    pub fn add_edit(&mut self, edit: ComposerFileEdit, cx: &mut Context<Self>) {
        let path = edit.path.clone();
        self.edits.insert(path.clone(), edit);

        // Auto-select first file if none selected
        if self.selected_file.is_none() {
            self.selected_file = Some(path);
        }

        cx.notify();
    }

    /// Remove a file edit
    pub fn remove_edit(&mut self, path: &PathBuf, cx: &mut Context<Self>) {
        self.edits.remove(path);

        // Clear selection if removed file was selected
        if self.selected_file.as_ref() == Some(path) {
            self.selected_file = self.edits.keys().next().cloned();
        }

        cx.notify();
    }

    /// Toggle staging for a file
    pub fn toggle_staged(&mut self, path: &PathBuf, cx: &mut Context<Self>) {
        if let Some(edit) = self.edits.get_mut(path) {
            edit.staged = !edit.staged;
            cx.notify();
        }
    }

    /// Apply all staged changes
    pub fn apply_all_changes(&mut self, cx: &mut Context<Self>) {
        if self.applying {
            return;
        }

        let staged_edits: Vec<_> = self
            .edits
            .values()
            .filter(|edit| edit.staged)
            .cloned()
            .collect();

        if staged_edits.is_empty() {
            return;
        }

        self.applying = true;
        cx.notify();

        let project = self.project.clone();
        let action_log = self.action_log.clone();

        cx.spawn(async move |this, mut cx| {
            let mut applied_paths = Vec::new();
            let mut errors = Vec::new();

            for edit in staged_edits {
                let path = edit.path.clone();
                let result = async {
                    let buffer = {
                        let buffer_task = project.update(&mut cx, |project, cx| {
                            let project_path = project
                                .find_project_path(&path, cx)
                                .ok_or_else(|| {
                                    anyhow!(
                                        "Unable to locate `{}` in the current workspace",
                                        path.display()
                                    )
                                })?;
                            Ok::<_, anyhow::Error>(project.open_buffer(project_path, cx))
                        })??;

                        buffer_task.await?
                    };

                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.finalize_last_transaction(cx);
                        buffer.start_transaction(cx);
                        buffer.set_text(edit.new_content.clone(), cx);
                        buffer.end_transaction(cx);
                        buffer.refresh_preview();
                    });

                    if let Some(action_log) = action_log.as_ref() {
                        action_log.update(&mut cx, |log, cx| {
                            log.buffer_edited(buffer.clone(), cx);
                        })?;
                    }

                    project
                        .update(&mut cx, |project, cx| project.save_buffer(buffer.clone(), cx))?
                        .await?;

                    Ok::<(), anyhow::Error>(())
                }
                .await;

                match result {
                    Ok(()) => applied_paths.push(path),
                    Err(err) => {
                        error!(
                            "Composer failed to apply edit for {}: {err:?}",
                            path.display()
                        );
                        errors.push(format!("{}: {err}", path.display()));
                    }
                }
            }

            let _ = this.update(&mut cx, |this, cx| {
                for path in &applied_paths {
                    this.edits.remove(path);
                }

                if let Some(selected) = this.selected_file.clone() {
                    if applied_paths.iter().any(|p| p == &selected) {
                        this.selected_file = this.edits.keys().next().cloned();
                    }
                } else {
                    this.selected_file = this.edits.keys().next().cloned();
                }

                this.applying = false;
                cx.notify();
            });

            if !errors.is_empty() {
                for message in &errors {
                    error!("Composer apply error: {message}");
                }
                if let Some(workspace) = this.workspace.upgrade() {
                    let message = if errors.len() == 1 {
                        format!("Failed to apply changes: {}", errors[0])
                    } else {
                        format!(
                            "Failed to apply {} files. Check logs for details.",
                            errors.len()
                        )
                    };
                    workspace
                        .update(&mut cx, |workspace, cx| {
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::unique::<ComposerApplyErrorToast>(),
                                    message,
                                ),
                                cx,
                            );
                        })
                        .ok();
                }
            } else if !applied_paths.is_empty() {
                if let Some(workspace) = this.workspace.upgrade() {
                    let message = if applied_paths.len() == 1 {
                        format!(
                            "Applied changes to {}",
                            applied_paths[0].to_string_lossy()
                        )
                    } else {
                        format!("Applied changes to {} files", applied_paths.len())
                    };
                    workspace
                        .update(&mut cx, |workspace, cx| {
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::unique::<ComposerApplyToast>(),
                                    message,
                                )
                                .autohide(),
                                cx,
                            );
                        })
                        .ok();
                }
            }
        })
        .detach();
    }

    /// Reject all staged changes
    pub fn reject_all_changes(&mut self, cx: &mut Context<Self>) {
        let staged_paths: Vec<_> = self
            .edits
            .iter()
            .filter(|(_, edit)| edit.staged)
            .map(|(path, _)| path.clone())
            .collect();

        let removed_count = staged_paths.len();

        for path in staged_paths.iter() {
            self.edits.remove(&path);
        }

        self.selected_file = self.edits.keys().next().cloned();
        cx.notify();

        if removed_count > 0 {
            if let Some(workspace) = self.workspace.upgrade() {
                let message = if removed_count == 1 {
                    "Discarded staged changes".to_string()
                } else {
                    format!("Discarded staged changes for {removed_count} files")
                };
                workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_toast(
                            Toast::new(
                                NotificationId::unique::<ComposerRejectToast>(),
                                message,
                            )
                            .autohide(),
                            cx,
                        );
                    })
                    .ok();
            }
        }
    }

    /// Get count of staged files
    pub fn staged_count(&self) -> usize {
        self.edits.values().filter(|edit| edit.staged).count()
    }

    /// Get total count of files
    pub fn total_count(&self) -> usize {
        self.edits.len()
    }

    /// Select a file for preview
    pub fn select_file(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.selected_file = Some(path);
        cx.notify();
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(
                Label::new(format!(
                    "Composer ({} / {})",
                    self.staged_count(),
                    self.total_count()
                ))
                .size(LabelSize::Small),
            )
            .child(
                Button::new("apply_all", "Apply All")
                    .icon_position(IconPosition::Start)
                    .icon(IconName::Check)
                    .disabled(self.staged_count() == 0 || self.applying)
                    .on_click(cx.listener(|this, _, _window, cx| this.apply_all_changes(cx))),
            )
            .child(
                Button::new("reject_all", "Reject All")
                    .icon_position(IconPosition::Start)
                    .icon(IconName::Close)
                    .disabled(self.staged_count() == 0 || self.applying)
                    .on_click(cx.listener(|this, _, _window, cx| this.reject_all_changes(cx))),
            )
    }

    fn render_file_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_1()
            .p_2()
            .children(self.edits.iter().enumerate().map(|(index, (path, edit))| {
                let is_selected = self.selected_file.as_ref() == Some(path);
                let path_clone = path.clone();
                let lines_summary = format!(
                    "+{} -{} lines",
                    edit.lines_added(),
                    edit.lines_removed()
                );

                ListItem::new(("composer-edit", index))
                    .spacing(ListItemSpacing::Sparse)
                    .inset(true)
                    .toggle(edit.staged)
                    .toggle_state(is_selected)
                    .on_toggle(cx.listener({
                        let path = path.clone();
                        move |this, _event: &ClickEvent, _window, cx| {
                            this.toggle_staged(&path, cx);
                            cx.stop_propagation();
                        }
                    }))
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.select_file(path_clone.clone(), cx);
                    }))
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new(edit.file_name()).size(LabelSize::Small))
                            .child(
                                Label::new(lines_summary)
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    )
            }))
    }

    fn render_preview(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self
            .selected_file
            .as_ref()
            .and_then(|path| self.edits.get(path))
        {
            Some(edit) => {
                let diff = unified_diff(&edit.original_content, &edit.new_content);
                let diff_lines: Vec<_> = diff
                    .lines()
                    .map(|line| {
                        let label = Label::new(line.to_string()).buffer_font(cx);
                        let color = match line.chars().next() {
                            Some('+') => Color::Success,
                            Some('-') => Color::Error,
                            Some('@') => Color::Accent,
                            Some('d') if line.starts_with("diff") => Color::Accent,
                            Some('i') if line.starts_with("index") => Color::Muted,
                            Some(' ') | None => Color::Default,
                            _ => Color::Muted,
                        };
                        label.color(color).into_any_element()
                    })
                    .collect();

                let diff_view = if diff_lines.is_empty() {
                    Label::new("No changes detected")
                        .color(Color::Muted)
                        .size(LabelSize::Small)
                        .into_any_element()
                } else {
                    v_flex().gap_1().children(diff_lines).into_any_element()
                };

                v_flex()
                    .p_4()
                    .gap_2()
                    .child(Label::new(format!("Preview: {}", edit.file_name())))
                    .child(
                        v_flex()
                            .id("composer-diff-preview")
                            .h_full()
                            .p_2()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_md()
                            .overflow_y_scroll()
                            .child(diff_view),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Label::new("Original").color(Color::Error))
                            .child(Label::new("→").color(Color::Muted))
                            .child(Label::new("Modified").color(Color::Success)),
                    )
            }
            None => v_flex()
                .p_4()
                .child(
                    Label::new("No file selected")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                ),
        }
    }
}

impl Focusable for ComposerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for ComposerPanel {}

impl Panel for ComposerPanel {
    fn persistent_name() -> &'static str {
        "ComposerPanel"
    }

    fn panel_key() -> &'static str {
        "ComposerPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Right | DockPosition::Left)
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // Position can be changed
    }

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.width.unwrap_or(px(400.0))
    }

    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.width = size;
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::FileDiff)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Composer - Multi-file editing")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleComposer)
    }

    fn activation_priority(&self) -> u32 {
        5
    }
}

impl Render for ComposerPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("ComposerPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .child(self.render_toolbar(cx))
            .when(self.applying, |this| {
                this.child(
                    v_flex()
                        .p_2()
                        .child(
                            Label::new("Applying changes...")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
            })
            .child(
                h_flex()
                    .flex_1()
                    .child(
                        v_flex()
                            .id("composer-file-list")
                            .w(px(250.0))
                            .h_full()
                            .border_r_1()
                            .border_color(cx.theme().colors().border)
                            .overflow_y_scroll()
                            .child(self.render_file_list(cx)),
                    )
                    .child(v_flex().flex_1().h_full().child(self.render_preview(cx))),
            )
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleComposer, window, cx| {
                    workspace.toggle_panel_focus::<ComposerPanel>(window, cx);
                })
                .register_action(|workspace, _: &ApplyAllChanges, _window, cx| {
                    if let Some(panel) = workspace.panel::<ComposerPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.apply_all_changes(cx));
                    }
                })
                .register_action(|workspace, _: &RejectAllChanges, _window, cx| {
                    if let Some(panel) = workspace.panel::<ComposerPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.reject_all_changes(cx));
                    }
                });
        },
    )
    .detach();
}
