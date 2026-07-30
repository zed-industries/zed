use crate::{
    diff_multibuffer::DiffMultibuffer,
    git_panel::{GitPanel, GitPanelAddon, GitStatusEntry},
};
use anyhow::{Context as _, Result};
use buffer_diff::DiffHunkStatus;
use editor::{
    DiffHunkDelegate, Editor, EditorEvent, ResolvedDiffHunks, SplittableEditor,
    actions::{GoToHunk, GoToPreviousHunk},
};
use git::{StageAll, StageAndNext};
use gpui::{
    Action, AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render,
    SharedString, Subscription, Task, WeakEntity,
};
use language::Capability;
use project::{
    Project, ProjectPath,
    git_store::diff_buffer_list::{DiffBase, DiffBufferList},
    project_settings::ProjectSettings,
};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    ops::Range,
    sync::Arc,
};
use ui::{DiffStat, Divider, Icon, Tooltip, Window, prelude::*};
use util::ResultExt as _;
use workspace::{
    ItemNavHistory, SerializableItem, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
    item::{Item, ItemEvent, ItemHandle, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};

pub(crate) struct UnstagedDiffDelegate;

impl DiffHunkDelegate for UnstagedDiffDelegate {
    fn toggle(
        &self,
        hunks: Vec<ResolvedDiffHunks>,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.stage_or_unstage(true, hunks, editor, window, cx);
    }

    fn stage_or_unstage(
        &self,
        stage: bool,
        hunks: Vec<ResolvedDiffHunks>,
        editor: &mut Editor,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if !stage {
            return;
        }
        let Some(project) = editor.project().cloned() else {
            return;
        };
        for hunks in hunks {
            let Some(buffer) = hunks.buffer else {
                continue;
            };
            let worktree_ranges = hunks
                .hunks
                .into_iter()
                .map(|hunk| hunk.buffer_range)
                .collect::<Vec<_>>();
            if worktree_ranges.is_empty() {
                continue;
            }
            project
                .update(cx, |project, cx| {
                    project.stage_hunks(buffer, hunks.diff, worktree_ranges, cx)
                })
                .log_err();
        }
    }

    fn restore(
        &self,
        hunks: Vec<ResolvedDiffHunks>,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if hunks.is_empty() || editor.read_only(cx) {
            return;
        }
        editor.transact(window, cx, |editor, window, cx| {
            editor.restore_diff_hunks(hunks, cx);
            let selections = editor
                .selections
                .all::<editor::MultiBufferOffset>(&editor.display_snapshot(cx));
            editor.change_selections(
                editor::SelectionEffects::no_scroll(),
                window,
                cx,
                |selections_state| {
                    selections_state.select(selections);
                },
            );
        });
    }

    fn render_hunk_controls(
        &self,
        row: u32,
        status: &DiffHunkStatus,
        hunk_range: Range<editor::Anchor>,
        is_created_file: bool,
        line_height: Pixels,
        editor: &Entity<Editor>,
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        if !ProjectSettings::get_global(cx)
            .git
            .show_stage_restore_buttons
        {
            return gpui::Empty.into_any_element();
        }
        let hunk_range_for_restore = hunk_range.clone();
        let hunk_range = hunk_range.start..hunk_range.start;
        h_flex()
            .h(line_height)
            .mr_1()
            .gap_1()
            .px_0p5()
            .pb_1()
            .border_x_1()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_b_lg()
            .bg(cx.theme().colors().editor_background)
            .block_mouse_except_scroll()
            .shadow_md()
            .child(
                Button::new(("stage", row as u64), "Stage")
                    .alpha(if status.is_pending() { 0.66 } else { 1.0 })
                    .tooltip(Tooltip::text("Stage Hunk"))
                    .on_click({
                        let editor = editor.clone();
                        move |_event, window, cx| {
                            editor.update(cx, |editor, cx| {
                                editor.stage_or_unstage_diff_hunks(
                                    true,
                                    vec![hunk_range.clone()],
                                    window,
                                    cx,
                                );
                            });
                        }
                    }),
            )
            .child(
                Button::new(("restore", row as u64), "Restore")
                    .tooltip(Tooltip::text("Restore Hunk"))
                    .on_click({
                        let editor = editor.clone();
                        let hunk_range = hunk_range_for_restore;
                        move |_event, window, cx| {
                            editor.update(cx, |editor, cx| {
                                let snapshot = editor.buffer().read(cx).snapshot(cx);
                                let hunks: Vec<_> = editor
                                    .diff_hunks_in_ranges(
                                        std::slice::from_ref(&hunk_range),
                                        &snapshot,
                                    )
                                    .collect();
                                if !hunks.is_empty() {
                                    editor.apply_restore(hunks, window, cx);
                                }
                            });
                        }
                    })
                    .disabled(is_created_file),
            )
            .into_any_element()
    }

    fn render_hunk_as_staged(&self, _status: &DiffHunkStatus, _cx: &App) -> bool {
        false
    }
}

pub struct UnstagedDiff {
    diff: Entity<DiffMultibuffer>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    _diff_event_subscription: Subscription,
}

impl UnstagedDiff {
    pub(crate) fn register(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
        let _ = workspace;
        workspace::register_serializable_item::<Self>(cx);
    }

    pub fn deploy_at(
        workspace: &mut Workspace,
        entry: Option<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        telemetry::event!(
            "Git Unstaged Diff Opened",
            source = if entry.is_some() {
                "Git Panel"
            } else {
                "Action"
            }
        );
        let intended_repo = workspace.project().read(cx).active_repository(cx);
        let existing = workspace.items_of_type::<Self>(cx).next();
        let unstaged_diff = if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity();
            let unstaged_diff =
                cx.new(|cx| Self::new(workspace.project().clone(), workspace_handle, window, cx));
            workspace.add_item_to_active_pane(
                Box::new(unstaged_diff.clone()),
                None,
                true,
                window,
                cx,
            );
            unstaged_diff
        };

        if let Some(intended) = &intended_repo {
            let needs_switch = unstaged_diff
                .read(cx)
                .diff
                .read(cx)
                .repo(cx)
                .map_or(true, |current| current.entity_id() != intended.entity_id());
            if needs_switch {
                unstaged_diff.update(cx, |unstaged_diff, cx| {
                    unstaged_diff.diff.update(cx, |diff, cx| {
                        diff.set_repo(Some(intended.clone()), cx);
                    });
                });
            }
        }

        if let Some(entry) = entry {
            unstaged_diff.update(cx, |unstaged_diff, cx| {
                unstaged_diff.move_to_entry(entry, window, cx);
            });
        }
    }

    pub(crate) fn move_to_entry(
        &mut self,
        entry: GitStatusEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff
            .update(cx, |diff, cx| diff.move_to_entry(entry, window, cx));
    }

    pub(crate) fn new(
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let branch_diff =
            cx.new(|cx| DiffBufferList::new(DiffBase::Index, project.clone(), window, cx));
        let workspace_handle = workspace.downgrade();
        let diff = cx.new(|cx| {
            DiffMultibuffer::new(
                branch_diff,
                Capability::ReadWrite,
                "No unstaged changes",
                move |editor, cx| {
                    editor.set_diff_hunk_delegate(Some(Arc::new(UnstagedDiffDelegate)), cx);
                    editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                        rhs_editor.set_read_only(false);
                        rhs_editor.register_addon(GitPanelAddon {
                            workspace: workspace_handle,
                        });
                    });
                },
                project.clone(),
                workspace.clone(),
                window,
                cx,
            )
        });
        Self::from_diff(diff, project, workspace, cx)
    }

    pub(crate) fn from_diff(
        diff: Entity<DiffMultibuffer>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let diff_event_subscription = cx.subscribe(&diff, |_, _, event: &EditorEvent, cx| {
            cx.emit(event.clone())
        });

        Self {
            diff,
            project,
            workspace: workspace.downgrade(),
            _diff_event_subscription: diff_event_subscription,
        }
    }

    fn button_states(&self, cx: &App) -> ButtonStates {
        let diff = self.diff.read(cx);
        let editor = diff.editor().read(cx).rhs_editor().clone();
        let editor = editor.read(cx);
        let snapshot = diff.multibuffer().read(cx).snapshot(cx);
        let prev_next = snapshot.diff_hunks().nth(1).is_some();
        let (selection, ranges) = diff.selected_ranges(cx);
        let stage = editor
            .diff_hunks_in_ranges(&ranges, &snapshot)
            .next()
            .is_some();
        let restore = editor
            .diff_hunks_in_ranges(&ranges, &snapshot)
            .any(|h| !h.is_created_file());
        let mut stage_all = false;
        self.workspace
            .read_with(cx, |workspace, cx| {
                if let Some(git_panel) = workspace.panel::<GitPanel>(cx) {
                    stage_all = git_panel.read(cx).can_stage_all();
                }
            })
            .ok();
        let restore_all = snapshot.diff_hunks().any(|h| !h.is_created_file());

        ButtonStates {
            stage,
            restore,
            restore_all,
            prev_next,
            selection,
            stage_all,
        }
    }

    fn stage_selected_unstaged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff.update(cx, |diff, cx| {
            diff.stage_or_unstage_selected_hunks(true, move_to_next, window, cx)
        });
    }

    fn restore_selected_unstaged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff.update(cx, |diff, cx| {
            diff.restore_selected_hunks(move_to_next, window, cx)
        });
    }
}

struct ButtonStates {
    stage: bool,
    restore: bool,
    restore_all: bool,
    prev_next: bool,
    selection: bool,
    stage_all: bool,
}

impl EventEmitter<EditorEvent> for UnstagedDiff {}

impl Focusable for UnstagedDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.diff.read(cx).focus_handle(cx)
    }
}

impl Item for UnstagedDiff {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch).color(Color::Muted))
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.diff
            .update(cx, |diff, cx| diff.deactivated(window, cx));
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.diff
            .update(cx, |diff, cx| diff.navigate(data, window, cx))
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Unstaged Changes".into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, _cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(0, _cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Unstaged Changes".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Git Unstaged Diff Opened")
    }

    fn as_searchable(&self, _: &Entity<Self>, cx: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.diff.read(cx).editor().clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.diff.read(cx).for_each_project_item(cx, f);
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff
            .update(cx, |diff, cx| diff.set_nav_history(nav_history, cx));
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(None);
        };
        let project = self.project.clone();
        Task::ready(Some(cx.new(|cx| Self::new(project, workspace, window, cx))))
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.diff.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.diff.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _cx: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.diff
            .update(cx, |diff, cx| diff.save(options, project, window, cx))
    }

    fn save_as(
        &mut self,
        _: Entity<Project>,
        _: ProjectPath,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.diff
            .update(cx, |diff, cx| diff.reload(project, window, cx))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<DiffMultibuffer>() {
            Some(self.diff.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(
                self.diff
                    .read(cx)
                    .editor()
                    .read(cx)
                    .rhs_editor()
                    .clone()
                    .into(),
            )
        } else if type_id == TypeId::of::<SplittableEditor>() {
            Some(self.diff.read(cx).editor().clone().into())
        } else if type_id == TypeId::of::<DiffBufferList>() {
            Some(self.diff.read(cx).branch_diff().clone().into())
        } else {
            None
        }
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff.update(cx, |diff, cx| {
            diff.added_to_workspace(workspace, window, cx)
        });
    }
}

impl SerializableItem for UnstagedDiff {
    fn serialized_item_kind() -> &'static str {
        "UnstagedDiff"
    }

    fn cleanup(
        _: workspace::WorkspaceId,
        _: Vec<workspace::ItemId>,
        _: &mut Window,
        _: &mut App,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        _: workspace::WorkspaceId,
        _: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            let workspace = workspace.upgrade().context("workspace gone")?;
            cx.update(|window, cx| Ok(cx.new(|cx| Self::new(project, workspace, window, cx))))?
        })
    }

    fn serialize(
        &mut self,
        _: &mut Workspace,
        _: workspace::ItemId,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        Some(Task::ready(Ok(())))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

impl Render for UnstagedDiff {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.diff.clone()
    }
}

pub struct UnstagedDiffToolbar {
    unstaged_diff: Option<WeakEntity<UnstagedDiff>>,
    workspace: WeakEntity<Workspace>,
}

impl UnstagedDiffToolbar {
    pub fn new(workspace: &Workspace, _: &mut Context<Self>) -> Self {
        Self {
            unstaged_diff: None,
            workspace: workspace.weak_handle(),
        }
    }

    fn unstaged_diff(&self, _: &App) -> Option<Entity<UnstagedDiff>> {
        self.unstaged_diff.as_ref()?.upgrade()
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(unstaged_diff) = self.unstaged_diff(cx) {
            unstaged_diff.focus_handle(cx).focus(window, cx);
        }
        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        })
    }

    fn stage_selected_unstaged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(unstaged_diff) = self.unstaged_diff(cx) else {
            return;
        };
        unstaged_diff.update(cx, |unstaged_diff, cx| {
            unstaged_diff.stage_selected_unstaged_hunks(move_to_next, window, cx);
        });
    }

    fn restore_selected_unstaged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(unstaged_diff) = self.unstaged_diff(cx) else {
            return;
        };
        unstaged_diff.update(cx, |unstaged_diff, cx| {
            unstaged_diff.restore_selected_unstaged_hunks(move_to_next, window, cx);
        });
    }

    fn stage_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.workspace
            .update(cx, |workspace, cx| {
                let Some(panel) = workspace.panel::<GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.stage_all(&Default::default(), window, cx);
                });
            })
            .ok();
    }

    fn restore_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(unstaged_diff) = self.unstaged_diff(cx) else {
            return;
        };
        let diff = unstaged_diff.read(cx).diff.read(cx);
        let editor = diff.editor().read(cx).rhs_editor().clone();
        let snapshot = diff.multibuffer().read(cx).snapshot(cx);
        let hunks: Vec<_> = snapshot
            .diff_hunks()
            .filter(|h| !h.is_created_file())
            .collect();
        if !hunks.is_empty() {
            editor.update(cx, |editor, cx| {
                editor.apply_restore(hunks, window, cx);
            });
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for UnstagedDiffToolbar {}

impl ToolbarItemView for UnstagedDiffToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.unstaged_diff = active_pane_item
            .and_then(|item| item.act_as::<UnstagedDiff>(cx))
            .map(|entity| entity.downgrade());
        if self.unstaged_diff.is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl Render for UnstagedDiffToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(unstaged_diff) = self.unstaged_diff(cx) else {
            return div();
        };
        let focus_handle = unstaged_diff.focus_handle(cx);
        let button_states = unstaged_diff.read(cx).button_states(cx);

        let diff = unstaged_diff.read(cx).diff.read(cx);
        let (additions, deletions) = diff.calculate_changed_lines(cx);
        let is_multibuffer_empty = diff.multibuffer().read(cx).is_empty();

        h_flex()
            .my_neg_1()
            .py_1()
            .gap_1p5()
            .flex_wrap()
            .justify_between()
            .when(!is_multibuffer_empty, |this| {
                this.child(DiffStat::new(
                    "unstaged-diff-stat",
                    additions as usize,
                    deletions as usize,
                ))
                .child(Divider::vertical().ml_1())
            })
            // n.b. the only reason these arrows are here is because we don't
            // support "undo" for staging so we need a way to go back.
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
                    .when(button_states.selection, |this| {
                        this.child(
                            Button::new("stage", "Stage")
                                .disabled(!button_states.stage)
                                .tooltip(Tooltip::text("Stage Selected Hunks"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.stage_selected_unstaged_hunks(false, window, cx)
                                })),
                        )
                    })
                    .when(!button_states.selection, |this| {
                        this.child(
                            Button::new("stage", "Stage")
                                .disabled(!button_states.stage)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Stage and Go to Next Hunk",
                                    &StageAndNext,
                                    &focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.stage_selected_unstaged_hunks(true, window, cx)
                                })),
                        )
                    })
                    .child(
                        Button::new("restore", "Restore")
                            .disabled(!button_states.restore)
                            .tooltip(Tooltip::text("Restore Selected Hunks"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.restore_selected_unstaged_hunks(false, window, cx)
                            })),
                    ),
            )
            .child(Divider::vertical())
            .child(
                Button::new("stage-all", "Stage All")
                    .width(rems_from_px(80.))
                    .disabled(!button_states.stage_all)
                    .tooltip(Tooltip::for_action_title_in(
                        "Stage All Changes",
                        &StageAll,
                        &focus_handle,
                    ))
                    .on_click(cx.listener(|this, _, window, cx| this.stage_all(window, cx))),
            )
            .child(Divider::vertical())
            .child(
                Button::new("restore-all", "Restore All")
                    .width(rems_from_px(80.))
                    .disabled(!button_states.restore_all)
                    .tooltip(Tooltip::text("Restore All Changes"))
                    .on_click(cx.listener(|this, _, window, cx| this.restore_all(window, cx))),
            )
    }
}
