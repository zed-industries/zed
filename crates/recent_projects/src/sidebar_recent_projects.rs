use std::sync::Arc;

use fuzzy_nucleo::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, TaskExt, WeakEntity, Window, WindowHandle,
};
use picker::{
    Picker, PickerDelegate,
    highlighted_match_with_paths::{HighlightedMatch, HighlightedMatchWithPaths},
};
use remote::RemoteConnectionOptions;
use settings::Settings;
use ui::{
    ButtonLike, HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*,
};
use util::{ResultExt, paths::PathExt};
use workspace::{
    MultiWorkspace, OpenMode, OpenOptions, ProjectGroupKey, RecentWorkspace,
    SerializedWorkspaceLocation, Workspace, WorkspaceDb, notifications::DetachAndPromptErr,
};

use zed_actions::OpenRemote;

use crate::{highlights_for_path, icon_for_remote_connection, open_remote_project};

pub struct SidebarRecentProjects {
    pub picker: Entity<Picker<SidebarRecentProjectsDelegate>>,
    _subscription: Subscription,
}

impl SidebarRecentProjects {
    pub fn popover(
        workspace: WeakEntity<Workspace>,
        window_project_groups: Vec<ProjectGroupKey>,
        active_project_group: Option<ProjectGroupKey>,
        _focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let fs = workspace
            .upgrade()
            .map(|ws| ws.read(cx).app_state().fs.clone());
        let multi_workspace = window.window_handle().downcast::<MultiWorkspace>();

        let open_projects: Vec<OpenProject> = window_project_groups
            .into_iter()
            .map(|key| OpenProject {
                label: key
                    .path_list()
                    .ordered_paths()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(", ")
                    .into(),
                is_active: active_project_group
                    .as_ref()
                    .is_some_and(|active| active.matches(&key)),
                key,
            })
            .collect();

        cx.new(|cx| {
            let delegate = SidebarRecentProjectsDelegate {
                workspace,
                multi_workspace,
                open_projects,
                workspaces: Vec::new(),
                filtered_workspaces: Vec::new(),
                selected_index: 0,
                focus_handle: cx.focus_handle(),
            };

            let picker: Entity<Picker<SidebarRecentProjectsDelegate>> = cx.new(|cx| {
                Picker::list(delegate, window, cx)
                    .list_measure_all()
                    .show_scrollbar(true)
                    .initial_width(rems(18.))
                    .popover()
            });

            let picker_focus_handle = picker.focus_handle(cx);
            picker.update(cx, |picker, _| {
                picker.delegate.focus_handle = picker_focus_handle;
            });

            let _subscription =
                cx.subscribe(&picker, |_this: &mut Self, _, _, cx| cx.emit(DismissEvent));

            let db = WorkspaceDb::global(cx);
            cx.spawn_in(window, async move |this, cx| {
                let Some(fs) = fs else { return };
                let workspaces = db
                    .recent_project_workspaces(fs.as_ref())
                    .await
                    .log_err()
                    .unwrap_or_default();
                this.update_in(cx, move |this, window, cx| {
                    this.picker.update(cx, move |picker, cx| {
                        picker.delegate.set_workspaces(workspaces);
                        picker.update_matches(picker.query(cx), window, cx)
                    })
                })
                .ok();
            })
            .detach();

            picker.focus_handle(cx).focus(window, cx);

            Self {
                picker,
                _subscription,
            }
        })
    }
}

impl EventEmitter<DismissEvent> for SidebarRecentProjects {}

impl Focusable for SidebarRecentProjects {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for SidebarRecentProjects {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("SidebarRecentProjects")
            .w(rems(18.))
            .child(self.picker.clone())
    }
}

/// A project group already open in this window, which is switched to in place
/// rather than opened again.
struct OpenProject {
    key: ProjectGroupKey,
    label: SharedString,
    is_active: bool,
}

pub struct SidebarRecentProjectsDelegate {
    workspace: WeakEntity<Workspace>,
    multi_workspace: Option<WindowHandle<MultiWorkspace>>,
    open_projects: Vec<OpenProject>,
    workspaces: Vec<RecentWorkspace>,
    filtered_workspaces: Vec<StringMatch>,
    selected_index: usize,
    focus_handle: FocusHandle,
}

impl SidebarRecentProjectsDelegate {
    pub fn set_workspaces(&mut self, workspaces: Vec<RecentWorkspace>) {
        self.workspaces = workspaces;
    }

    /// Candidate ids run over the open projects first and the recents after, so
    /// that one fuzzy match set can cover both sections.
    fn recent_at(&self, candidate_id: usize) -> Option<&RecentWorkspace> {
        self.workspaces
            .get(candidate_id.checked_sub(self.open_projects.len())?)
    }

    /// Mirrors what clicking a sidebar project header does: prefer the workspace the
    /// group was last active in, fall back to any workspace on those paths, and only
    /// reopen the paths when the group has no live workspace left.
    fn activate_open_project(&self, key: ProjectGroupKey, cx: &mut Context<Picker<Self>>) {
        let Some(handle) = self.multi_workspace else {
            return;
        };
        cx.defer(move |cx| {
            let task = handle
                .update(cx, |multi_workspace, window, cx| {
                    let workspace = multi_workspace
                        .last_active_workspace_for_group(&key, cx)
                        .or_else(|| {
                            multi_workspace.workspace_for_paths(
                                key.path_list(),
                                key.host().as_ref(),
                                cx,
                            )
                        });
                    match workspace {
                        Some(workspace) => {
                            multi_workspace.activate(workspace, None, window, cx);
                            None
                        }
                        None => Some(multi_workspace.open_project(
                            key.path_list().paths().to_vec(),
                            OpenMode::Activate,
                            window,
                            cx,
                        )),
                    }
                })
                .log_err()
                .flatten();
            if let Some(task) = task {
                task.detach_and_log_err(cx);
            }
        });
    }
}

impl EventEmitter<DismissEvent> for SidebarRecentProjectsDelegate {}

impl PickerDelegate for SidebarRecentProjectsDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "sidebar recent projects"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Switch or open a project…".into()
    }

    fn match_count(&self) -> usize {
        self.filtered_workspaces.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let open_count = self
            .filtered_workspaces
            .iter()
            .filter(|hit| hit.candidate_id < self.open_projects.len())
            .count();
        if open_count > 0 && open_count < self.filtered_workspaces.len() {
            vec![open_count - 1]
        } else {
            Vec::new()
        }
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query = query.trim_start();
        let case = fuzzy_nucleo::Case::smart_if_uppercase_in(query);
        let is_empty_query = query.is_empty();

        let current_workspace_id = self
            .workspace
            .upgrade()
            .and_then(|ws| ws.read(cx).database_id());

        let open_count = self.open_projects.len();
        let mut candidates: Vec<_> = self
            .open_projects
            .iter()
            .enumerate()
            .map(|(id, open)| StringMatchCandidate::new(id, &open.label))
            .collect();

        candidates.extend(
            self.workspaces
                .iter()
                .enumerate()
                .filter(|(_, workspace)| {
                    Some(workspace.workspace_id) != current_workspace_id
                        && !self
                            .open_projects
                            .iter()
                            .any(|open| open.key.matches(&workspace.project_group_key()))
                })
                .map(|(id, workspace)| {
                    let combined_string = workspace
                        .identity_paths
                        .ordered_paths()
                        .map(|path| path.compact().to_string_lossy().into_owned())
                        .collect::<Vec<_>>()
                        .concat();
                    StringMatchCandidate::new(open_count + id, &combined_string)
                }),
        );

        if is_empty_query {
            self.filtered_workspaces = candidates
                .into_iter()
                .map(|candidate| StringMatch {
                    candidate_id: candidate.id,
                    score: 0.0,
                    positions: Vec::new(),
                    string: candidate.string,
                })
                .collect();
        } else {
            self.filtered_workspaces = match_strings(
                &candidates,
                query,
                case,
                fuzzy_nucleo::LengthPenalty::On,
                100,
            );
        }

        // Fuzzy scores would otherwise interleave the two sections and the separator
        // between them would stop meaning anything.
        self.filtered_workspaces
            .sort_by_key(|hit| hit.candidate_id >= open_count);

        self.selected_index = 0;
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(hit) = self.filtered_workspaces.get(self.selected_index) else {
            return;
        };

        if let Some(open) = self.open_projects.get(hit.candidate_id) {
            if !open.is_active {
                self.activate_open_project(open.key.clone(), cx);
            }
            cx.emit(DismissEvent);
            return;
        }

        let Some(recent_workspace) = self.recent_at(hit.candidate_id) else {
            return;
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        match &recent_workspace.location {
            SerializedWorkspaceLocation::Local => {
                if let Some(handle) = window.window_handle().downcast::<MultiWorkspace>() {
                    let paths = recent_workspace.paths.paths().to_vec();
                    cx.defer(move |cx| {
                        if let Some(task) = handle
                            .update(cx, |multi_workspace, window, cx| {
                                multi_workspace.open_project(paths, OpenMode::Activate, window, cx)
                            })
                            .log_err()
                        {
                            task.detach_and_log_err(cx);
                        }
                    });
                }
            }
            SerializedWorkspaceLocation::Remote(connection) => {
                let mut connection = connection.clone();
                workspace.update(cx, |workspace, cx| {
                    let app_state = workspace.app_state().clone();
                    let replace_window = window.window_handle().downcast::<MultiWorkspace>();
                    let open_options = OpenOptions {
                        requesting_window: replace_window,
                        ..Default::default()
                    };
                    if let RemoteConnectionOptions::Ssh(connection) = &mut connection {
                        crate::RemoteSettings::get_global(cx)
                            .fill_connection_options_from_settings(connection);
                    };
                    let paths = recent_workspace.paths.paths().to_vec();
                    cx.spawn_in(window, async move |_, cx| {
                        open_remote_project(connection.clone(), paths, app_state, open_options, cx)
                            .await
                    })
                    .detach_and_prompt_err(
                        "Failed to open project",
                        window,
                        cx,
                        |_, _, _| None,
                    );
                });
            }
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        let text = if self.workspaces.is_empty() {
            "Recently opened projects will show up here"
        } else {
            "No matches"
        };
        Some(text.into())
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = self.filtered_workspaces.get(ix)?;

        if let Some(open) = self.open_projects.get(hit.candidate_id) {
            let hint = if open.is_active {
                Some("Active")
            } else if selected {
                Some("Switch")
            } else {
                None
            };
            return Some(
                ListItem::new(ix)
                    .toggle_state(selected)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .child(
                        h_flex()
                            .w_full()
                            .min_w_0()
                            .gap_3()
                            .justify_between()
                            .child(
                                h_flex()
                                    .min_w_0()
                                    .gap_3()
                                    .child(Icon::new(IconName::FileTree).color(
                                        if open.is_active {
                                            Color::Accent
                                        } else {
                                            Color::Muted
                                        },
                                    ))
                                    .child(
                                        HighlightedLabel::new(
                                            open.label.clone(),
                                            hit.positions.clone(),
                                        )
                                        .truncate(),
                                    ),
                            )
                            .children(hint.map(|hint| {
                                Label::new(hint).size(LabelSize::XSmall).color(Color::Muted)
                            })),
                    )
                    .tooltip({
                        let path = open.label.clone();
                        move |_, cx| {
                            Tooltip::with_meta("Switch to Project", None, path.clone(), cx)
                        }
                    })
                    .into_any_element(),
            );
        }

        let workspace = self.recent_at(hit.candidate_id)?;

        let ordered_paths: Vec<_> = workspace
            .identity_paths
            .ordered_paths()
            .map(|p| p.compact().to_string_lossy().to_string())
            .collect();

        let tooltip_path: SharedString = match &workspace.location {
            SerializedWorkspaceLocation::Remote(options) => {
                let host = options.display_name();
                if ordered_paths.len() == 1 {
                    format!("{} ({})", ordered_paths[0], host).into()
                } else {
                    format!("{}\n({})", ordered_paths.join("\n"), host).into()
                }
            }
            _ => ordered_paths.join("\n").into(),
        };

        let mut path_start_offset = 0;
        let match_labels: Vec<_> = workspace
            .identity_paths
            .ordered_paths()
            .map(|p| p.compact())
            .map(|path| {
                let (label, path_match) =
                    highlights_for_path(path.as_ref(), &hit.positions, path_start_offset);
                path_start_offset += path_match.text.len();
                label
            })
            .collect();

        let prefix = match &workspace.location {
            SerializedWorkspaceLocation::Remote(options) => {
                Some(SharedString::from(options.display_name()))
            }
            _ => None,
        };

        let highlighted_match = HighlightedMatchWithPaths {
            prefix,
            match_label: HighlightedMatch::join(match_labels.into_iter().flatten(), ", "),
            paths: Vec::new(),
            active: false,
        };

        // The icon is what separates a recent from an open project at a glance, so it
        // is always shown here even for purely local lists.
        let icon = match &workspace.location {
            SerializedWorkspaceLocation::Local => IconName::HistoryRerun,
            SerializedWorkspaceLocation::Remote(options) => {
                icon_for_remote_connection(Some(options))
            }
        };

        Some(
            ListItem::new(ix)
                .toggle_state(selected)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .child(
                    h_flex()
                        .w_full()
                        .min_w_0()
                        .gap_3()
                        .justify_between()
                        .child(
                            h_flex()
                                .min_w_0()
                                .gap_3()
                                .flex_grow_1()
                                .child(Icon::new(icon).color(Color::Muted))
                                .child(highlighted_match.render(window, cx)),
                        )
                        .when(selected, |this| {
                            this.child(
                                Label::new("Open")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                        }),
                )
                .tooltip(move |_, cx| {
                    Tooltip::with_meta(
                        "Open Project in This Window",
                        None,
                        tooltip_path.clone(),
                        cx,
                    )
                })
                .into_any_element(),
        )
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();

        Some(
            v_flex()
                .p_1p5()
                .flex_1()
                .gap_1()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child({
                    let open_action = workspace::Open {
                        create_new_window: Some(false),
                    };

                    ButtonLike::new("open_local_folder")
                        .child(
                            h_flex()
                                .w_full()
                                .gap_1()
                                .justify_between()
                                .child(Label::new("Open Local Folders"))
                                .child(KeyBinding::for_action_in(&open_action, &focus_handle, cx)),
                        )
                        .on_click(cx.listener(move |_, _, window, cx| {
                            window.dispatch_action(open_action.boxed_clone(), cx);
                            cx.emit(DismissEvent);
                        }))
                })
                .child(
                    ButtonLike::new("open_remote_folder")
                        .child(
                            h_flex()
                                .w_full()
                                .gap_1()
                                .justify_between()
                                .child(Label::new("Open Remote Folder"))
                                .child(KeyBinding::for_action(
                                    &OpenRemote {
                                        from_existing_connection: false,
                                        create_new_window: Some(false),
                                    },
                                    cx,
                                )),
                        )
                        .on_click(cx.listener(|_, _, window, cx| {
                            window.dispatch_action(
                                OpenRemote {
                                    from_existing_connection: false,
                                    create_new_window: Some(false),
                                }
                                .boxed_clone(),
                                cx,
                            );
                            cx.emit(DismissEvent);
                        })),
                )
                .into_any(),
        )
    }
}
