use std::sync::Arc;

use chrono::{DateTime, Utc};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, WeakEntity, Window,
};
use picker::{
    Picker, PickerDelegate,
    highlighted_match_with_paths::{HighlightedMatch, HighlightedMatchWithPaths},
};
use remote::RemoteConnectionOptions;
use settings::Settings;
use ui::{KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*};
use ui_input::ErasedEditor;
use util::{ResultExt, paths::PathExt};
use workspace::{
    MultiWorkspace, OpenMode, OpenOptions, PathList, ProjectGroupKey, SerializedWorkspaceLocation,
    Workspace, WorkspaceDb, WorkspaceId, notifications::DetachAndPromptErr,
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
        _focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let fs = workspace
            .upgrade()
            .map(|ws| ws.read(cx).app_state().fs.clone());

        cx.new(|cx| {
            let delegate = SidebarRecentProjectsDelegate {
                workspace,
                window_project_groups,
                workspaces: Vec::new(),
                filtered_workspaces: Vec::new(),
                selected_index: 0,
                has_any_non_local_projects: false,
                focus_handle: cx.focus_handle(),
            };

            let picker: Entity<Picker<SidebarRecentProjectsDelegate>> = cx.new(|cx| {
                Picker::list(delegate, window, cx)
                    .list_measure_all()
                    .show_scrollbar(true)
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
                let workspaces =
                    workspace::resolve_worktree_workspaces(workspaces, fs.as_ref()).await;
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

pub struct SidebarRecentProjectsDelegate {
    workspace: WeakEntity<Workspace>,
    window_project_groups: Vec<ProjectGroupKey>,
    workspaces: Vec<(
        WorkspaceId,
        SerializedWorkspaceLocation,
        PathList,
        DateTime<Utc>,
    )>,
    filtered_workspaces: Vec<StringMatch>,
    selected_index: usize,
    has_any_non_local_projects: bool,
    focus_handle: FocusHandle,
}

impl SidebarRecentProjectsDelegate {
    pub fn set_workspaces(
        &mut self,
        workspaces: Vec<(
            WorkspaceId,
            SerializedWorkspaceLocation,
            PathList,
            DateTime<Utc>,
        )>,
    ) {
        self.has_any_non_local_projects = workspaces
            .iter()
            .any(|(_, location, _, _)| !matches!(location, SerializedWorkspaceLocation::Local));
        self.workspaces = workspaces;
    }
}

impl EventEmitter<DismissEvent> for SidebarRecentProjectsDelegate {}

impl PickerDelegate for SidebarRecentProjectsDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search recent projects…".into()
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        h_flex()
            .flex_none()
            .h_9()
            .px_2p5()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(editor.render(window, cx))
    }

    fn match_count(&self) -> usize {
        self.filtered_workspaces.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
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
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let is_empty_query = query.is_empty();

        let current_workspace_id = self
            .workspace
            .upgrade()
            .and_then(|ws| ws.read(cx).database_id());

        let candidates: Vec<_> = self
            .workspaces
            .iter()
            .enumerate()
            .filter(|(_, (id, _, paths, _))| {
                Some(*id) != current_workspace_id
                    && !self
                        .window_project_groups
                        .iter()
                        .any(|key| key.path_list() == paths)
            })
            .map(|(id, (_, _, paths, _))| {
                let combined_string = paths
                    .ordered_paths()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("");
                StringMatchCandidate::new(id, &combined_string)
            })
            .collect();

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
            let mut matches = smol::block_on(fuzzy::match_strings(
                &candidates,
                query,
                smart_case,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            ));
            matches.sort_unstable_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.candidate_id.cmp(&b.candidate_id))
            });
            self.filtered_workspaces = matches;
        }

        self.selected_index = 0;
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(hit) = self.filtered_workspaces.get(self.selected_index) else {
            return;
        };
        let Some((_, location, candidate_workspace_paths, _)) =
            self.workspaces.get(hit.candidate_id)
        else {
            return;
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        match location {
            SerializedWorkspaceLocation::Local => {
                if let Some(handle) = window.window_handle().downcast::<MultiWorkspace>() {
                    let paths = candidate_workspace_paths.paths().to_vec();
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
                    let paths = candidate_workspace_paths.paths().to_vec();
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
        let (_, location, paths, _) = self.workspaces.get(hit.candidate_id)?;

        let ordered_paths: Vec<_> = paths
            .ordered_paths()
            .map(|p| p.compact().to_string_lossy().to_string())
            .collect();

        let tooltip_path: SharedString = match &location {
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
        let match_labels: Vec<_> = paths
            .ordered_paths()
            .map(|p| p.compact())
            .map(|path| {
                let (label, path_match) =
                    highlights_for_path(path.as_ref(), &hit.positions, path_start_offset);
                path_start_offset += path_match.text.len();
                label
            })
            .collect();

        let prefix = match &location {
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

        let icon = icon_for_remote_connection(match location {
            SerializedWorkspaceLocation::Local => None,
            SerializedWorkspaceLocation::Remote(options) => Some(options),
        });

        Some(
            ListItem::new(ix)
                .toggle_state(selected)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .child(
                    h_flex()
                        .gap_3()
                        .flex_grow()
                        .when(self.has_any_non_local_projects, |this| {
                            this.child(Icon::new(icon).color(Color::Muted))
                        })
                        .child(highlighted_match.render(window, cx)),
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
                        create_new_window: false,
                    };

                    Button::new("open_local_folder", "Add Local Folders")
                        .key_binding(KeyBinding::for_action_in(&open_action, &focus_handle, cx))
                        .on_click(cx.listener(move |_, _, window, cx| {
                            window.dispatch_action(open_action.boxed_clone(), cx);
                            cx.emit(DismissEvent);
                        }))
                })
                .child(
                    Button::new("open_remote_folder", "Add Remote Folder")
                        .key_binding(KeyBinding::for_action(
                            &OpenRemote {
                                from_existing_connection: false,
                                create_new_window: false,
                            },
                            cx,
                        ))
                        .on_click(cx.listener(|_, _, window, cx| {
                            window.dispatch_action(
                                OpenRemote {
                                    from_existing_connection: false,
                                    create_new_window: false,
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
