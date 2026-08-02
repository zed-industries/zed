use crate::{git_panel::GitStatusEntry, git_panel_settings::GitPanelSettings, git_status_icon};
use anyhow::{Context as _, Result};
use buffer_diff::DiffHunkSecondaryStatus;
use editor::{
    DiffStyleControls, Direction, Editor, EditorEvent, EditorSettings, SplittableEditor,
    ToggleSplitDiff,
    actions::{GoToHunk, GoToPreviousHunk},
    file_status_label_color,
};
use git::{
    Commit, Restore, StageAndNext, StageFile, ToggleStaged, UnstageAndNext, UnstageFile,
    repository::RepoPath, status::StageStatus,
};
use gpui::{
    Action, AnyElement, App, AppContext as _, Context, Empty, Entity, EventEmitter, FocusHandle,
    Focusable, HighlightStyle, IntoElement, Render, Subscription, Task, WeakEntity, Window,
};
use language::{Anchor, Buffer, HighlightedText, OffsetRangeExt as _, Point};
use multi_buffer::{MultiBuffer, PathKey, excerpt_context_lines};
use project::{
    Project,
    git_store::{Repository, RepositoryId},
};
use settings::{Settings, SettingsStore, StatusStyle};
use std::{
    any::{Any, TypeId},
    ops::Range,
    sync::Arc,
};
use ui::{DiffStat, Divider, Tooltip, prelude::*};
use util::paths::{PathExt as _, PathStyle};
use workspace::{
    Item, ItemHandle, ItemNavHistory, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
    item::{ItemEvent, SaveOptions, TabContentParams},
    notifications::NotifyTaskExt,
    searchable::SearchableItemHandle,
};

pub struct SoloDiffView {
    repository: Entity<Repository>,
    repository_id: RepositoryId,
    repo_path: RepoPath,
    buffer: Entity<Buffer>,
    diff: Entity<buffer_diff::BufferDiff>,
    editor: Entity<SplittableEditor>,
    workspace: WeakEntity<Workspace>,
    showing_full_file: bool,
    _settings_subscription: Subscription,
}

impl SoloDiffView {
    pub fn open_or_focus(
        entry: GitStatusEntry,
        repository: Entity<Repository>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let Some(workspace_entity) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };

        let existing = workspace_entity
            .read(cx)
            .items_of_type::<SoloDiffView>(cx)
            .find(|item| item.read(cx).matches(&repository, &entry.repo_path, cx));
        if let Some(existing) = existing {
            workspace_entity.update(cx, |workspace, cx| {
                workspace.activate_item(&existing, true, true, window, cx);
            });
            existing.focus_handle(cx).focus(window, cx);
            return Task::ready(Ok(existing));
        }

        let Some(project_path) = repository
            .read(cx)
            .repo_path_to_project_path(&entry.repo_path, cx)
        else {
            return Task::ready(Err(anyhow::anyhow!(
                "could not resolve repository path {:?}",
                entry.repo_path
            )));
        };

        let project = workspace_entity.read(cx).project().clone();
        let repo_path = entry.repo_path;
        window.spawn(cx, async move |cx| {
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
                .await?;
            let diff = project
                .update(cx, |project, cx| {
                    project.open_uncommitted_diff(buffer.clone(), cx)
                })
                .await?;

            workspace_entity.update_in(cx, |workspace, window, cx| {
                let workspace_handle = cx.entity();
                let view = cx.new(|cx| {
                    Self::new(
                        project,
                        repository,
                        repo_path,
                        buffer,
                        diff,
                        workspace_handle,
                        window,
                        cx,
                    )
                });

                workspace.add_item_to_active_pane(Box::new(view.clone()), None, true, window, cx);
                view
            })
        })
    }

    fn new(
        project: Entity<Project>,
        repository: Entity<Repository>,
        repo_path: RepoPath,
        buffer: Entity<Buffer>,
        diff: Entity<buffer_diff::BufferDiff>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let repository_id = repository.read(cx).id;
        let showing_full_file = EditorSettings::get_global(cx).file_diff.show_full_file;
        let multibuffer = cx
            .new(|cx| Self::build_multibuffer(buffer.clone(), diff.clone(), showing_full_file, cx));
        let editor = cx.new(|cx| {
            let editor = SplittableEditor::new(
                EditorSettings::get_global(cx).diff_view_style,
                multibuffer,
                project.clone(),
                workspace.clone(),
                window,
                cx,
            );
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.set_should_serialize(false, cx);
                editor.set_allow_git_diff_scrollbar_markers(showing_full_file, cx);
                let snapshot = editor.snapshot(window, cx);
                editor.go_to_hunk_before_or_after_position(
                    &snapshot,
                    language::Point::new(0, 0),
                    Direction::Next,
                    true,
                    window,
                    cx,
                );
            });
            editor
        });

        let mut previous_diff_view_style = EditorSettings::get_global(cx).diff_view_style;
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
                let diff_view_style = EditorSettings::get_global(cx).diff_view_style;
                if diff_view_style != previous_diff_view_style {
                    this.editor.update(cx, |editor, cx| {
                        if editor.diff_view_style() != diff_view_style {
                            editor.toggle_split(&ToggleSplitDiff, window, cx);
                        }
                    });
                    previous_diff_view_style = diff_view_style;
                    cx.notify();
                }
            });

        Self {
            repository,
            repository_id,
            repo_path,
            buffer,
            diff,
            editor,
            workspace: workspace.downgrade(),
            showing_full_file,
            _settings_subscription: settings_subscription,
        }
    }

    fn build_multibuffer(
        buffer: Entity<Buffer>,
        diff: Entity<buffer_diff::BufferDiff>,
        showing_full_file: bool,
        cx: &mut Context<MultiBuffer>,
    ) -> MultiBuffer {
        let (ranges, context_line_count) =
            Self::excerpt_ranges(&buffer, &diff, showing_full_file, cx);

        let mut multibuffer = MultiBuffer::without_headers(buffer.read(cx).capability());
        multibuffer.set_excerpts_for_buffer(buffer, ranges, context_line_count, cx);
        multibuffer.add_diff(diff, cx);
        multibuffer
    }

    fn excerpt_ranges(
        buffer: &Entity<Buffer>,
        diff: &Entity<buffer_diff::BufferDiff>,
        showing_full_file: bool,
        cx: &App,
    ) -> (Vec<Range<Point>>, u32) {
        if showing_full_file {
            (vec![Point::zero()..buffer.read(cx).max_point()], 0)
        } else {
            (
                Self::hunk_ranges(buffer, diff, cx),
                excerpt_context_lines(cx),
            )
        }
    }

    fn hunk_ranges(
        buffer: &Entity<Buffer>,
        diff: &Entity<buffer_diff::BufferDiff>,
        cx: &App,
    ) -> Vec<Range<Point>> {
        let buffer = buffer.read(cx);
        diff.read(cx)
            .snapshot(cx)
            .hunks_intersecting_range(
                Anchor::min_for_buffer(buffer.remote_id())
                    ..Anchor::max_for_buffer(buffer.remote_id()),
                buffer,
            )
            .map(|diff_hunk| diff_hunk.buffer_range.to_point(buffer))
            .collect()
    }

    fn set_showing_full_file(&mut self, showing_full_file: bool, cx: &mut Context<Self>) {
        if self.showing_full_file == showing_full_file {
            return;
        }

        let (ranges, context_line_count) =
            Self::excerpt_ranges(&self.buffer, &self.diff, showing_full_file, cx);

        self.editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&self.buffer, cx);
            editor.remove_excerpts_for_path(path.clone(), cx);
            editor.update_excerpts_for_path(
                path,
                self.buffer.clone(),
                ranges,
                context_line_count,
                self.diff.clone(),
                cx,
            );
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.set_allow_git_diff_scrollbar_markers(showing_full_file, cx);
            });
        });

        self.showing_full_file = showing_full_file;
        cx.notify();
    }

    fn matches(&self, repository: &Entity<Repository>, repo_path: &RepoPath, cx: &App) -> bool {
        self.repository_id == repository.read(cx).id && &self.repo_path == repo_path
    }

    fn button_states(&self, cx: &App) -> SoloDiffButtonStates {
        let editor = self.editor.read(cx).rhs_editor().read(cx);
        let multibuffer = editor.buffer().read(cx);
        let snapshot = multibuffer.snapshot(cx);
        let prev_next = snapshot.diff_hunks().nth(1).is_some();
        let mut selection = true;

        let mut ranges = editor
            .selections
            .disjoint_anchor_ranges()
            .collect::<Vec<_>>();
        if !ranges.iter().any(|range| range.start != range.end) {
            selection = false;
            let anchor = editor.selections.newest_anchor().head();
            if let Some((_, excerpt_range)) = snapshot.excerpt_containing(anchor..anchor)
                && let Some(range) = snapshot
                    .anchor_in_buffer(excerpt_range.context.start)
                    .zip(snapshot.anchor_in_buffer(excerpt_range.context.end))
                    .map(|(start, end)| start..end)
            {
                ranges = vec![range];
            } else {
                ranges = Vec::new();
            }
        }

        let mut stage = false;
        let mut unstage = false;
        for hunk in editor.diff_hunks_in_ranges(&ranges, &snapshot) {
            match hunk.status.secondary {
                DiffHunkSecondaryStatus::HasSecondaryHunk
                | DiffHunkSecondaryStatus::SecondaryHunkAdditionPending => {
                    stage = true;
                }
                DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk => {
                    stage = true;
                    unstage = true;
                }
                DiffHunkSecondaryStatus::NoSecondaryHunk
                | DiffHunkSecondaryStatus::SecondaryHunkRemovalPending => {
                    unstage = true;
                }
            }
        }

        let stage_status = self
            .repository
            .read(cx)
            .status_for_path(&self.repo_path)
            .map(|entry| entry.status.staging())
            .unwrap_or(StageStatus::Unstaged);

        SoloDiffButtonStates {
            stage,
            unstage,
            restore: stage || unstage,
            prev_next,
            selection,
            stage_file: stage_status.has_unstaged(),
            unstage_file: stage_status.has_staged(),
        }
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut App) {
        self.focus_handle(cx).focus(window, cx);
        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        });
    }

    fn change_file_stage(&self, stage: bool, window: &mut Window, cx: &mut Context<Self>) {
        let repository = self.repository.clone();
        let repo_path = self.repo_path.clone();
        let workspace = self.workspace.clone();
        let task = cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, cx| {
                    if stage {
                        repository.stage_entries(vec![repo_path], cx)
                    } else {
                        repository.unstage_entries(vec![repo_path], cx)
                    }
                })
                .await
                .with_context(|| {
                    if stage {
                        "failed to stage file"
                    } else {
                        "failed to unstage file"
                    }
                })
        });
        task.detach_and_notify_err(workspace, window, cx);
    }
}

impl EventEmitter<EditorEvent> for SoloDiffView {}

impl Focusable for SoloDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for SoloDiffView {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Diff).color(Color::Muted))
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.buffer
            .read(cx)
            .file()
            .and_then(|file| {
                Some(
                    file.full_path(cx)
                        .file_name()?
                        .to_string_lossy()
                        .to_string(),
                )
            })
            .unwrap_or_else(|| {
                self.repo_path
                    .as_ref()
                    .display(PathStyle::local())
                    .into_owned()
            })
            .into()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        Some(
            self.buffer
                .read(cx)
                .file()
                .map(|file| file.full_path(cx).compact().to_string_lossy().into_owned())
                .unwrap_or_else(|| {
                    self.repo_path
                        .as_ref()
                        .display(PathStyle::local())
                        .into_owned()
                })
                .into(),
        )
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Solo Diff View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.deactivated(window, cx);
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<SplittableEditor>() {
            None
        } else {
            self.editor.act_as_type(type_id, cx)
        }
    }

    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, _| {
                editor.set_nav_history(Some(nav_history));
            })
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor.update(cx, |editor, cx| {
            editor
                .rhs_editor()
                .update(cx, |editor, cx| editor.navigate(data, window, cx))
        })
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<gpui::Font>)> {
        let text: SharedString = self
            .repo_path
            .as_ref()
            .display(PathStyle::local())
            .into_owned()
            .into();

        // When the git panel is set to convey status via label color rather
        // than an icon, tint the whole path like multibuffer headers do.
        let mut highlights = Vec::new();
        if GitPanelSettings::get_global(cx).status_style == StatusStyle::LabelColor
            && let Some(status) = self
                .repository
                .read(cx)
                .status_for_path(&self.repo_path)
                .map(|entry| entry.status)
        {
            highlights.push((
                0..text.len(),
                HighlightStyle::color(file_status_label_color(Some(status)).color(cx)),
            ));
        }

        Some((
            vec![HighlightedText { text, highlights }],
            Some(
                theme_settings::ThemeSettings::get_global(cx)
                    .buffer_font
                    .clone(),
            ),
        ))
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.rhs_editor().update(cx, |editor, cx| {
                editor.added_to_workspace(workspace, window, cx)
            })
        });
    }

    fn can_save(&self, cx: &App) -> bool {
        self.editor.read(cx).rhs_editor().read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(options, project, window, cx)
    }
}

impl Render for SoloDiffView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

pub struct SoloDiffStyleToolbar {
    solo_diff: Option<WeakEntity<SoloDiffView>>,
}

pub struct SoloDiffGitToolbar {
    solo_diff: Option<WeakEntity<SoloDiffView>>,
}

impl SoloDiffStyleToolbar {
    pub fn new(_: &mut Context<Self>) -> Self {
        Self { solo_diff: None }
    }

    fn solo_diff(&self) -> Option<Entity<SoloDiffView>> {
        self.solo_diff.as_ref()?.upgrade()
    }

    fn toggle_showing_full_file(&mut self, cx: &mut Context<Self>) {
        if let Some(solo_diff) = self.solo_diff() {
            solo_diff.update(cx, |solo_diff, cx| {
                solo_diff.set_showing_full_file(!solo_diff.showing_full_file, cx);
            });
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for SoloDiffStyleToolbar {}

impl ToolbarItemView for SoloDiffStyleToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.solo_diff = active_pane_item
            .and_then(|item| item.act_as::<SoloDiffView>(cx))
            .map(|entity| entity.downgrade());
        if self.solo_diff.is_some() {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for SoloDiffStyleToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(solo_diff) = self.solo_diff() else {
            return Empty.into_any_element();
        };

        let (editor_entity, showing_full_file, status) = {
            let solo_diff = solo_diff.read(cx);
            (
                solo_diff.editor.clone(),
                solo_diff.showing_full_file,
                solo_diff
                    .repository
                    .read(cx)
                    .status_for_path(&solo_diff.repo_path)
                    .map(|entry| entry.status),
            )
        };

        let show_status_icon =
            GitPanelSettings::get_global(cx).status_style != StatusStyle::LabelColor;

        let (expand_icon, expand_tooltip) = if showing_full_file {
            (IconName::ChevronDownUp, "Show Changes Only")
        } else {
            (IconName::ChevronUpDown, "Show Full File")
        };

        h_flex()
            .pl_0p5()
            .gap_1()
            .child(
                IconButton::new("solo-diff-toggle-excerpts", expand_icon)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text(expand_tooltip))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_showing_full_file(cx);
                    })),
            )
            .child(DiffStyleControls::new(editor_entity))
            .child(Divider::vertical().mr_1())
            .when_some(
                show_status_icon.then_some(status).flatten(),
                |this, status| this.child(git_status_icon(status)),
            )
            .into_any_element()
    }
}

impl SoloDiffGitToolbar {
    pub fn new(_: &mut Context<Self>) -> Self {
        Self { solo_diff: None }
    }

    fn solo_diff(&self) -> Option<Entity<SoloDiffView>> {
        self.solo_diff.as_ref()?.upgrade()
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(solo_diff) = self.solo_diff() {
            solo_diff.update(cx, |solo_diff, cx| {
                solo_diff.dispatch_action(action, window, cx);
            });
        }
    }

    fn stage_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(solo_diff) = self.solo_diff() {
            solo_diff.update(cx, |solo_diff, cx| {
                solo_diff.change_file_stage(true, window, cx);
            });
        }
    }

    fn unstage_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(solo_diff) = self.solo_diff() {
            solo_diff.update(cx, |solo_diff, cx| {
                solo_diff.change_file_stage(false, window, cx);
            });
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for SoloDiffGitToolbar {}

impl ToolbarItemView for SoloDiffGitToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.solo_diff = active_pane_item
            .and_then(|item| item.act_as::<SoloDiffView>(cx))
            .map(|entity| entity.downgrade());
        if self.solo_diff.is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

struct SoloDiffButtonStates {
    stage: bool,
    unstage: bool,
    restore: bool,
    prev_next: bool,
    selection: bool,
    stage_file: bool,
    unstage_file: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use multi_buffer::MultiBufferRow;

    #[gpui::test]
    fn test_changes_only_multibuffer_has_one_buffer_and_expand_controls(cx: &mut TestAppContext) {
        let base_text = (0..20)
            .map(|line| format!("line {line}"))
            .collect::<Vec<_>>()
            .join("\n");
        let current_text = base_text.replace("line 10", "changed line 10");
        let buffer = cx.new(|cx| Buffer::local(current_text, cx));
        let diff = cx.new(|cx| {
            buffer_diff::BufferDiff::new_with_base_text(
                &base_text,
                &buffer.read(cx).text_snapshot(),
                cx,
            )
        });
        let multibuffer = cx.new(|cx| SoloDiffView::build_multibuffer(buffer, diff, false, cx));

        let (is_singleton, buffer_count, has_expand_controls) =
            multibuffer.update(cx, |multibuffer, cx| {
                (
                    multibuffer.is_singleton(),
                    multibuffer.snapshot(cx).all_buffer_ids().count(),
                    multibuffer
                        .snapshot(cx)
                        .row_infos(MultiBufferRow(0))
                        .any(|row| row.expand_info.is_some()),
                )
            });

        assert!(!is_singleton);
        assert_eq!(buffer_count, 1);
        assert!(has_expand_controls);
    }
}

impl Render for SoloDiffGitToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(solo_diff) = self.solo_diff() else {
            return gpui::Empty.into_any_element();
        };

        let focus_handle = solo_diff.focus_handle(cx);
        let solo_diff = solo_diff.read(cx);
        let button_states = solo_diff.button_states(cx);
        let status_entry = solo_diff
            .repository
            .read(cx)
            .status_for_path(&solo_diff.repo_path);
        let diff_stat = status_entry.and_then(|entry| entry.diff_stat);

        h_flex()
            .my_neg_1()
            .py_1()
            .gap_1p5()
            .flex_wrap()
            .justify_between()
            .children(diff_stat.map(|stat| {
                DiffStat::new("solo-diff-stat", stat.added as usize, stat.deleted as usize)
            }))
            .child(Divider::vertical().ml_1())
            .child(
                h_group_sm()
                    .child(
                        IconButton::new("up", IconName::ArrowUp)
                            .icon_size(IconSize::Small)
                            .disabled(!button_states.prev_next)
                            .tooltip(Tooltip::for_action_title_in(
                                "Go to Previous Hunk",
                                &GoToPreviousHunk,
                                &focus_handle,
                            ))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&GoToPreviousHunk, window, cx)
                            })),
                    )
                    .child(
                        IconButton::new("down", IconName::ArrowDown)
                            .icon_size(IconSize::Small)
                            .disabled(!button_states.prev_next)
                            .tooltip(Tooltip::for_action_title_in(
                                "Go to Next Hunk",
                                &GoToHunk,
                                &focus_handle,
                            ))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&GoToHunk, window, cx)
                            })),
                    ),
            )
            .child(Divider::vertical())
            .child(
                h_group_sm()
                    .when(button_states.selection, |el| {
                        el.child(
                            Button::new("stage", "Toggle Staged")
                                .disabled(!button_states.stage && !button_states.unstage)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Toggle Staged",
                                    &ToggleStaged,
                                    &focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.dispatch_action(&ToggleStaged, window, cx)
                                })),
                        )
                    })
                    .when(!button_states.selection, |el| {
                        el.child(
                            Button::new("stage", "Stage")
                                .disabled(!button_states.stage)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Stage and Go to Next Hunk",
                                    &StageAndNext,
                                    &focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.dispatch_action(&StageAndNext, window, cx)
                                })),
                        )
                        .child(
                            Button::new("unstage", "Unstage")
                                .disabled(!button_states.unstage)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Unstage and Go to Next Hunk",
                                    &UnstageAndNext,
                                    &focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.dispatch_action(&UnstageAndNext, window, cx)
                                })),
                        )
                    })
                    .child(
                        Button::new("restore", "Restore")
                            .tooltip(Tooltip::for_action_title_in(
                                "Restore selected hunk",
                                &Restore,
                                &focus_handle,
                            ))
                            .disabled(!button_states.restore)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&Restore, window, cx)
                            })),
                    ),
            )
            .child(Divider::vertical())
            .child(h_group_sm().child(if button_states.stage_file {
                Button::new("stage-file", "Stage All")
                    .width(rems_from_px(80.))
                    .disabled(!button_states.stage_file)
                    .tooltip(Tooltip::for_action_title_in(
                        "Stage All",
                        &StageFile,
                        &focus_handle,
                    ))
                    .on_click(cx.listener(|this, _, window, cx| this.stage_file(window, cx)))
            } else {
                Button::new("unstage-file", "Unstage All")
                    .width(rems_from_px(80.))
                    .disabled(!button_states.unstage_file)
                    .tooltip(Tooltip::for_action_title_in(
                        "Unstage All",
                        &UnstageFile,
                        &focus_handle,
                    ))
                    .on_click(cx.listener(|this, _, window, cx| this.unstage_file(window, cx)))
            }))
            .child(Divider::vertical())
            .child(
                Button::new("commit", "Commit")
                    .tooltip(Tooltip::for_action_title_in(
                        "Commit",
                        &Commit,
                        &focus_handle,
                    ))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.dispatch_action(&Commit, window, cx);
                    })),
            )
            .into_any_element()
    }
}
