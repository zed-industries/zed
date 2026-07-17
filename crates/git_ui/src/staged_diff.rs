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
use git::{Commit, UnstageAll, UnstageAndNext};
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

pub(crate) struct StagedDiffDelegate;

impl DiffHunkDelegate for StagedDiffDelegate {
    fn toggle(
        &self,
        hunks: Vec<ResolvedDiffHunks>,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.stage_or_unstage(false, hunks, editor, window, cx);
    }

    fn stage_or_unstage(
        &self,
        stage: bool,
        hunks: Vec<ResolvedDiffHunks>,
        editor: &mut Editor,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if stage {
            return;
        }
        let Some(project) = editor.project().cloned() else {
            return;
        };
        for hunks in hunks {
            let index_ranges = hunks
                .hunks
                .into_iter()
                .map(|hunk| hunk.buffer_range)
                .collect::<Vec<_>>();
            if index_ranges.is_empty() {
                continue;
            }
            project
                .update(cx, |project, cx| {
                    project.unstage_staged_hunks(hunks.diff, index_ranges, cx)
                })
                .log_err();
        }
    }

    fn render_hunk_controls(
        &self,
        row: u32,
        status: &DiffHunkStatus,
        hunk_range: Range<editor::Anchor>,
        _is_created_file: bool,
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
                Button::new(("unstage", row as u64), "Unstage")
                    .alpha(if status.is_pending() { 0.66 } else { 1.0 })
                    .tooltip(Tooltip::text("Unstage Hunk"))
                    .on_click({
                        let editor = editor.clone();
                        move |_event, window, cx| {
                            editor.update(cx, |editor, cx| {
                                editor.stage_or_unstage_diff_hunks(
                                    false,
                                    vec![hunk_range.clone()],
                                    window,
                                    cx,
                                );
                            });
                        }
                    }),
            )
            .into_any_element()
    }
}

/// The workspace item for the staged diff. It wraps a single read-only
/// [`DiffMultibuffer`] over [`DiffBase::Staged`] and delegates the [`Item`]
/// surface to it.
pub struct StagedDiff {
    diff: Entity<DiffMultibuffer>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    _diff_event_subscription: Subscription,
}

impl StagedDiff {
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
            "Git Staged Diff Opened",
            source = if entry.is_some() {
                "Git Panel"
            } else {
                "Action"
            }
        );
        let intended_repo = workspace.project().read(cx).active_repository(cx);
        let existing = workspace.items_of_type::<Self>(cx).next();
        let staged_diff = if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let workspace_handle = cx.entity();
            let staged_diff =
                cx.new(|cx| Self::new(workspace.project().clone(), workspace_handle, window, cx));
            workspace.add_item_to_active_pane(
                Box::new(staged_diff.clone()),
                None,
                true,
                window,
                cx,
            );
            staged_diff
        };

        if let Some(intended) = &intended_repo {
            let needs_switch = staged_diff
                .read(cx)
                .diff
                .read(cx)
                .repo(cx)
                .map_or(true, |current| current.entity_id() != intended.entity_id());
            if needs_switch {
                staged_diff.update(cx, |staged_diff, cx| {
                    staged_diff.diff.update(cx, |diff, cx| {
                        diff.set_repo(Some(intended.clone()), cx);
                    });
                });
            }
        }

        if let Some(entry) = entry {
            staged_diff.update(cx, |staged_diff, cx| {
                staged_diff.move_to_entry(entry, window, cx);
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
            cx.new(|cx| DiffBufferList::new(DiffBase::Staged, project.clone(), window, cx));
        let workspace_handle = workspace.downgrade();
        let diff = cx.new(|cx| {
            DiffMultibuffer::new(
                branch_diff,
                Capability::ReadOnly,
                "No staged changes",
                move |editor, cx| {
                    editor.set_diff_hunk_delegate(Some(Arc::new(StagedDiffDelegate)), cx);
                    editor.rhs_editor().update(cx, |rhs_editor, _cx| {
                        rhs_editor.set_read_only(true);
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
        let unstage = editor
            .diff_hunks_in_ranges(&ranges, &snapshot)
            .next()
            .is_some();
        let mut unstage_all = false;
        self.workspace
            .read_with(cx, |workspace, cx| {
                if let Some(git_panel) = workspace.panel::<GitPanel>(cx) {
                    unstage_all = git_panel.read(cx).can_unstage_all();
                }
            })
            .ok();

        ButtonStates {
            unstage,
            prev_next,
            selection,
            unstage_all,
        }
    }

    fn unstage_selected_staged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.diff.update(cx, |diff, cx| {
            diff.stage_or_unstage_selected_hunks(false, move_to_next, window, cx)
        });
    }
}

struct ButtonStates {
    unstage: bool,
    prev_next: bool,
    selection: bool,
    unstage_all: bool,
}

impl EventEmitter<EditorEvent> for StagedDiff {}

impl Focusable for StagedDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.diff.read(cx).focus_handle(cx)
    }
}

impl Item for StagedDiff {
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
        Some("Staged Changes".into())
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
        "Staged Changes".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Git Staged Diff Opened")
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

    fn can_save(&self, _: &App) -> bool {
        false
    }

    fn save(
        &mut self,
        _: SaveOptions,
        _: Entity<Project>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        Task::ready(Ok(()))
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

impl SerializableItem for StagedDiff {
    fn serialized_item_kind() -> &'static str {
        "StagedDiff"
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

impl Render for StagedDiff {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.diff.clone()
    }
}

pub struct StagedDiffToolbar {
    staged_diff: Option<WeakEntity<StagedDiff>>,
    workspace: WeakEntity<Workspace>,
}

impl StagedDiffToolbar {
    pub fn new(workspace: &Workspace, _: &mut Context<Self>) -> Self {
        Self {
            staged_diff: None,
            workspace: workspace.weak_handle(),
        }
    }

    fn staged_diff(&self, _: &App) -> Option<Entity<StagedDiff>> {
        self.staged_diff.as_ref()?.upgrade()
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(staged_diff) = self.staged_diff(cx) {
            staged_diff.focus_handle(cx).focus(window, cx);
        }
        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        })
    }

    fn unstage_selected_staged_hunks(
        &mut self,
        move_to_next: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(staged_diff) = self.staged_diff(cx) else {
            return;
        };
        staged_diff.update(cx, |staged_diff, cx| {
            staged_diff.unstage_selected_staged_hunks(move_to_next, window, cx);
        });
    }

    fn unstage_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.workspace
            .update(cx, |workspace, cx| {
                let Some(panel) = workspace.panel::<GitPanel>(cx) else {
                    return;
                };
                panel.update(cx, |panel, cx| {
                    panel.unstage_all(&Default::default(), window, cx);
                });
            })
            .ok();
    }
}

impl EventEmitter<ToolbarItemEvent> for StagedDiffToolbar {}

impl ToolbarItemView for StagedDiffToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.staged_diff = active_pane_item
            .and_then(|item| item.act_as::<StagedDiff>(cx))
            .map(|entity| entity.downgrade());
        if self.staged_diff.is_some() {
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

impl Render for StagedDiffToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(staged_diff) = self.staged_diff(cx) else {
            return div();
        };
        let focus_handle = staged_diff.focus_handle(cx);
        let button_states = staged_diff.read(cx).button_states(cx);

        let diff = staged_diff.read(cx).diff.read(cx);
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
                    "staged-diff-stat",
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
                            Button::new("unstage", "Unstage")
                                .disabled(!button_states.unstage)
                                .tooltip(Tooltip::text("Unstage Selected Hunks"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.unstage_selected_staged_hunks(false, window, cx)
                                })),
                        )
                    })
                    .when(!button_states.selection, |this| {
                        this.child(
                            Button::new("unstage", "Unstage")
                                .disabled(!button_states.unstage)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Unstage and Go to Next Hunk",
                                    &UnstageAndNext,
                                    &focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.unstage_selected_staged_hunks(true, window, cx)
                                })),
                        )
                    }),
            )
            .child(Divider::vertical())
            .child(
                Button::new("unstage-all", "Unstage All")
                    .width(rems_from_px(80.))
                    .disabled(!button_states.unstage_all)
                    .tooltip(Tooltip::for_action_title_in(
                        "Unstage All Changes",
                        &UnstageAll,
                        &focus_handle,
                    ))
                    .on_click(cx.listener(|this, _, window, cx| this.unstage_all(window, cx))),
            )
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
    }
}

#[cfg(test)]
mod tests {
    use crate::project_diff::{self, ProjectDiff};
    use git::repository::RepoPath;
    use gpui::{Action as _, TestAppContext};
    use language::Point;
    use project::{FakeFs, Fs as _};
    use serde_json::json;
    use settings::{DiffViewStyle, SettingsStore};
    use std::path::Path;
    use unindent::Unindent as _;
    use util::{path, rel_path::rel_path};
    use workspace::MultiWorkspace;

    use super::*;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.diff_view_style = Some(DiffViewStyle::Unified);
                });
            });
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    #[gpui::test]
    async fn test_staged_changes_deploy_as_a_separate_staged_diff_item(cx: &mut TestAppContext) {
        init_test(cx);

        let committed_contents = r#"
            fn main() {
                println!("hello world");
            }
        "#
        .unindent();
        let staged_contents = r#"
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();
        let file_contents = r#"
            // print goodbye
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "src": {
                    "main.rs": file_contents.clone(),
                }
            }),
        )
        .await;

        fs.set_head_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", committed_contents)],
            "deadbeef",
        );
        fs.set_index_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", staged_contents.clone())],
        );

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        cx.focus(&workspace);
        cx.update(|window, cx| {
            window.dispatch_action(project_diff::Diff.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let uncommitted_item = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<ProjectDiff>(cx).unwrap()
        });

        workspace.update_in(cx, |workspace, window, cx| {
            StagedDiff::deploy_at(workspace, None, window, cx);
        });
        cx.run_until_parked();

        workspace.update(cx, |workspace, cx| {
            let staged_diff = workspace.active_item_as::<StagedDiff>(cx).unwrap();
            assert_ne!(staged_diff.entity_id(), uncommitted_item.entity_id());
            let staged_item = workspace
                .active_item(cx)
                .unwrap()
                .act_as::<DiffMultibuffer>(cx)
                .unwrap();
            assert_ne!(staged_item.entity_id(), uncommitted_item.entity_id());
            assert_eq!(
                staged_item.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
                DiffBase::Staged
            );
            assert!(staged_item.read_with(cx, |diff, cx| diff.multibuffer().read(cx).read_only()));
            assert_eq!(workspace.items_of_type::<ProjectDiff>(cx).count(), 1);
            assert_eq!(workspace.items_of_type::<StagedDiff>(cx).count(), 1);

            let active_item = workspace.active_item(cx).unwrap();
            assert!(active_item.act_as::<StagedDiff>(cx).is_some());
            assert!(active_item.act_as::<DiffMultibuffer>(cx).is_some());
            assert_eq!(
                active_item
                    .to_serializable_item_handle(cx)
                    .unwrap()
                    .serialized_item_kind(),
                "StagedDiff"
            );
            assert_eq!(active_item.tab_content_text(0, cx), "Staged Changes");
            assert!(!active_item.can_save(cx));
        });
    }

    #[gpui::test]
    async fn test_toggle_staged_unstages_from_staged_view(cx: &mut TestAppContext) {
        init_test(cx);

        let committed_contents = r#"
            fn main() {
                println!("hello world");
            }
        "#
        .unindent();
        let staged_contents = r#"
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();
        let file_contents = r#"
            // print goodbye
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "src": {
                    "main.rs": file_contents,
                }
            }),
        )
        .await;
        fs.set_head_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", committed_contents.clone())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", staged_contents)],
        );
        let repo = fs
            .open_repo(path!("/project/.git").as_ref(), Some("git".as_ref()))
            .unwrap();

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        workspace.update_in(cx, |workspace, window, cx| {
            StagedDiff::deploy_at(workspace, None, window, cx);
        });
        cx.run_until_parked();

        let editor = workspace.update(cx, |workspace, cx| {
            let staged_diff = workspace.active_item_as::<StagedDiff>(cx).unwrap();
            let staged_diff = staged_diff.read(cx);
            staged_diff
                .diff
                .read(cx)
                .editor()
                .read(cx)
                .rhs_editor()
                .clone()
        });
        editor.read_with(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            assert_eq!(
                editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], &snapshot)
                    .count(),
                1
            );
        });

        // Hold back FS events so the first assertions observe the optimistic
        // state rather than a reloaded diff.
        fs.pause_events();

        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([Point::new(1, 0)..Point::new(1, 0)]);
            });
        });
        cx.focus(&editor);
        cx.update(|window, cx| {
            window.dispatch_action(git::ToggleStaged.boxed_clone(), cx);
        });
        cx.run_until_parked();

        // The hunk is optimistically suppressed from the staged view, and the
        // index write has landed.
        editor.read_with(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            assert_eq!(
                editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], &snapshot)
                    .count(),
                0
            );
        });
        assert_eq!(
            repo.load_index_text(RepoPath::from_rel_path(rel_path("src/main.rs")))
                .await
                .unwrap(),
            committed_contents
        );

        fs.unpause_events_and_flush();
        cx.run_until_parked();

        // Once the write is reconciled, the staged view remains empty.
        editor.read_with(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            assert_eq!(
                editor
                    .diff_hunks_in_ranges(&[editor::Anchor::Min..editor::Anchor::Max], &snapshot)
                    .count(),
                0
            );
        });
    }

    #[gpui::test]
    async fn test_staged_diff_restores_as_staged_diff(cx: &mut TestAppContext) {
        init_test(cx);

        let committed_contents = r#"
            fn main() {
                println!("hello world");
            }
        "#
        .unindent();
        let staged_contents = r#"
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();
        let file_contents = r#"
            // print goodbye
            fn main() {
                println!("goodbye world");
            }
        "#
        .unindent();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "src": {
                    "main.rs": file_contents,
                }
            }),
        )
        .await;

        fs.set_head_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", committed_contents)],
            "deadbeef",
        );
        fs.set_index_for_repo(
            Path::new(path!("/project/.git")),
            &[("src/main.rs", staged_contents)],
        );

        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.run_until_parked();

        let project = workspace.update(cx, |workspace, _| workspace.project().clone());
        let workspace_id = workspace::WorkspaceId::from_i64(1);
        let item_id = 42;

        let restore_task = workspace.update_in(cx, |_workspace, window, cx| {
            <StagedDiff as SerializableItem>::deserialize(
                project.clone(),
                cx.entity().downgrade(),
                workspace_id,
                item_id,
                window,
                cx,
            )
        });
        let restored_staged_diff = restore_task.await.unwrap();

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(
                Box::new(restored_staged_diff.clone()),
                None,
                true,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        workspace.update(cx, |workspace, cx| {
            let active_item = workspace.active_item(cx).unwrap();
            assert!(active_item.act_as::<StagedDiff>(cx).is_some());
            assert!(active_item.act_as::<DiffMultibuffer>(cx).is_some());
            assert_eq!(
                active_item
                    .to_serializable_item_handle(cx)
                    .unwrap()
                    .serialized_item_kind(),
                "StagedDiff"
            );
            assert_eq!(active_item.tab_content_text(0, cx), "Staged Changes");
            assert!(!active_item.can_save(cx));
            assert_eq!(workspace.items_of_type::<ProjectDiff>(cx).count(), 0);
            assert_eq!(workspace.items_of_type::<StagedDiff>(cx).count(), 1);
            let diff = active_item.act_as::<DiffMultibuffer>(cx).unwrap();
            assert_eq!(
                diff.read_with(cx, |diff, cx| diff.diff_base(cx).clone()),
                DiffBase::Staged
            );
        });
    }
}
