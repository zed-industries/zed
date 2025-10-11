pub mod disconnected_overlay;
mod remote_connections;
mod remote_servers;
mod ssh_config;

#[cfg(target_os = "windows")]
mod wsl_picker;

use remote::RemoteConnectionOptions;
pub use remote_connections::open_remote_project;

use disconnected_overlay::DisconnectedOverlay;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, WeakEntity, Window,
};
use ordered_float::OrderedFloat;
use picker::{
    Picker, PickerDelegate,
    highlighted_match_with_paths::{HighlightedMatch, HighlightedMatchWithPaths},
};
pub use remote_connections::SshSettings;
pub use remote_servers::RemoteServerProjects;
use settings::Settings;
use std::{path::Path, sync::Arc};
use ui::{KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*, tooltip_container};
use util::{ResultExt, paths::PathExt};
use workspace::{
    CloseIntent, HistoryManager, ModalView, OpenOptions, PathList, SerializedWorkspaceLocation,
    WORKSPACE_DB, Workspace, WorkspaceId, with_active_or_new_workspace,
};
use zed_actions::{OpenRecent, OpenRemote};

pub fn init(cx: &mut App) {
    SshSettings::register(cx);

    #[cfg(target_os = "windows")]
    cx.on_action(|open_wsl: &zed_actions::wsl_actions::OpenFolderInWsl, cx| {
        let create_new_window = open_wsl.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            use gpui::PathPromptOptions;
            use project::DirectoryLister;

            let paths = workspace.prompt_for_open_path(
                PathPromptOptions {
                    files: true,
                    directories: true,
                    multiple: false,
                    prompt: None,
                },
                DirectoryLister::Local(
                    workspace.project().clone(),
                    workspace.app_state().fs.clone(),
                ),
                window,
                cx,
            );

            cx.spawn_in(window, async move |workspace, cx| {
                use util::paths::SanitizedPath;

                let Some(paths) = paths.await.log_err().flatten() else {
                    return;
                };

                let paths = paths
                    .into_iter()
                    .filter_map(|path| SanitizedPath::new(&path).local_to_wsl())
                    .collect::<Vec<_>>();

                if paths.is_empty() {
                    let message = indoc::indoc! { r#"
                        Invalid path specified when trying to open a folder inside WSL.

                        Please note that Zed currently does not support opening network share folders inside wsl.
                    "#};

                    let _ = cx.prompt(gpui::PromptLevel::Critical, "Invalid path", Some(&message), &["Ok"]).await;
                    return;
                }

                workspace.update_in(cx, |workspace, window, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        crate::wsl_picker::WslOpenModal::new(paths, create_new_window, window, cx)
                    });
                }).log_err();
            })
            .detach();
        });
    });

    #[cfg(target_os = "windows")]
    cx.on_action(|open_wsl: &zed_actions::wsl_actions::OpenWsl, cx| {
        let create_new_window = open_wsl.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let handle = cx.entity().downgrade();
            let fs = workspace.project().read(cx).fs().clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                RemoteServerProjects::wsl(create_new_window, fs, window, handle, cx)
            });
        });
    });

    cx.on_action(|open_recent: &OpenRecent, cx| {
        let create_new_window = open_recent.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let Some(recent_projects) = workspace.active_modal::<RecentProjects>(cx) else {
                RecentProjects::open(workspace, create_new_window, window, cx);
                return;
            };

            recent_projects.update(cx, |recent_projects, cx| {
                recent_projects
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(window, cx))
            });
        });
    });
    cx.on_action(|open_remote: &OpenRemote, cx| {
        let from_existing_connection = open_remote.from_existing_connection;
        let create_new_window = open_remote.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            if from_existing_connection {
                cx.propagate();
                return;
            }
            let handle = cx.entity().downgrade();
            let fs = workspace.project().read(cx).fs().clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                RemoteServerProjects::new(create_new_window, fs, window, handle, cx)
            })
        });
    });

    cx.observe_new(DisconnectedOverlay::register).detach();
}

pub struct RecentProjects {
    pub picker: Entity<Picker<RecentProjectsDelegate>>,
    rem_width: f32,
    _subscription: Subscription,
}

impl ModalView for RecentProjects {}

impl RecentProjects {
    fn new(
        delegate: RecentProjectsDelegate,
        rem_width: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| {
            // We want to use a list when we render paths, because the items can have different heights (multiple paths).
            if delegate.render_paths {
                Picker::list(delegate, window, cx)
            } else {
                Picker::uniform_list(delegate, window, cx)
            }
        });
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));
        // We do not want to block the UI on a potentially lengthy call to DB, so we're gonna swap
        // out workspace locations once the future runs to completion.
        cx.spawn_in(window, async move |this, cx| {
            let workspaces = WORKSPACE_DB
                .recent_workspaces_on_disk()
                .await
                .log_err()
                .unwrap_or_default();
            this.update_in(cx, move |this, window, cx| {
                this.picker.update(cx, move |picker, cx| {
                    picker.delegate.set_workspaces(workspaces);
                    picker.update_matches(picker.query(cx), window, cx)
                })
            })
            .ok()
        })
        .detach();
        Self {
            picker,
            rem_width,
            _subscription,
        }
    }

    pub fn open(
        workspace: &mut Workspace,
        create_new_window: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let weak = cx.entity().downgrade();
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = RecentProjectsDelegate::new(weak, create_new_window, true);

            Self::new(delegate, 34., window, cx)
        })
    }
}

impl EventEmitter<DismissEvent> for RecentProjects {}

impl Focusable for RecentProjects {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RecentProjects {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RecentProjects")
            .w(rems(self.rem_width))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), window, cx);
                })
            }))
    }
}

pub struct RecentProjectsDelegate {
    workspace: WeakEntity<Workspace>,
    workspaces: Vec<(WorkspaceId, SerializedWorkspaceLocation, PathList)>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
    render_paths: bool,
    create_new_window: bool,
    // Flag to reset index when there is a new query vs not reset index when user delete an item
    reset_selected_match_index: bool,
    has_any_non_local_projects: bool,
}

impl RecentProjectsDelegate {
    fn new(workspace: WeakEntity<Workspace>, create_new_window: bool, render_paths: bool) -> Self {
        Self {
            workspace,
            workspaces: Vec::new(),
            selected_match_index: 0,
            matches: Default::default(),
            create_new_window,
            render_paths,
            reset_selected_match_index: true,
            has_any_non_local_projects: false,
        }
    }

    pub fn set_workspaces(
        &mut self,
        workspaces: Vec<(WorkspaceId, SerializedWorkspaceLocation, PathList)>,
    ) {
        self.workspaces = workspaces;
        self.has_any_non_local_projects = !self
            .workspaces
            .iter()
            .all(|(_, location, _)| matches!(location, SerializedWorkspaceLocation::Local));
    }
}
impl EventEmitter<DismissEvent> for RecentProjectsDelegate {}
impl PickerDelegate for RecentProjectsDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, window: &mut Window, _: &mut App) -> Arc<str> {
        let (create_window, reuse_window) = if self.create_new_window {
            (
                window.keystroke_text_for(&menu::Confirm),
                window.keystroke_text_for(&menu::SecondaryConfirm),
            )
        } else {
            (
                window.keystroke_text_for(&menu::SecondaryConfirm),
                window.keystroke_text_for(&menu::Confirm),
            )
        };
        Arc::from(format!(
            "{reuse_window} reuses this window, {create_window} opens a new one",
        ))
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_match_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim_start();
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let candidates = self
            .workspaces
            .iter()
            .enumerate()
            .filter(|(_, (id, _, _))| !self.is_current_workspace(*id, cx))
            .map(|(id, (_, _, paths))| {
                let combined_string = paths
                    .paths()
                    .iter()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("");
                StringMatchCandidate::new(id, &combined_string)
            })
            .collect::<Vec<_>>();
        self.matches = smol::block_on(fuzzy::match_strings(
            candidates.as_slice(),
            query,
            smart_case,
            true,
            100,
            &Default::default(),
            cx.background_executor().clone(),
        ));
        self.matches.sort_unstable_by(|a, b| {
            b.score
                .partial_cmp(&a.score) // Descending score
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.candidate_id.cmp(&b.candidate_id)) // Ascending candidate_id for ties
        });

        if self.reset_selected_match_index {
            self.selected_match_index = self
                .matches
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, m)| OrderedFloat(m.score))
                .map(|(ix, _)| ix)
                .unwrap_or(0);
        }
        self.reset_selected_match_index = true;
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some((selected_match, workspace)) = self
            .matches
            .get(self.selected_index())
            .zip(self.workspace.upgrade())
        {
            let (candidate_workspace_id, candidate_workspace_location, candidate_workspace_paths) =
                &self.workspaces[selected_match.candidate_id];
            let replace_current_window = if self.create_new_window {
                secondary
            } else {
                !secondary
            };
            workspace
                .update(cx, |workspace, cx| {
                    if workspace.database_id() == Some(*candidate_workspace_id) {
                        Task::ready(Ok(()))
                    } else {
                        match candidate_workspace_location.clone() {
                            SerializedWorkspaceLocation::Local => {
                                let paths = candidate_workspace_paths.paths().to_vec();
                                if replace_current_window {
                                    cx.spawn_in(window, async move |workspace, cx| {
                                        let continue_replacing = workspace
                                            .update_in(cx, |workspace, window, cx| {
                                                workspace.prepare_to_close(
                                                    CloseIntent::ReplaceWindow,
                                                    window,
                                                    cx,
                                                )
                                            })?
                                            .await?;
                                        if continue_replacing {
                                            workspace
                                                .update_in(cx, |workspace, window, cx| {
                                                    workspace.open_workspace_for_paths(
                                                        true, paths, window, cx,
                                                    )
                                                })?
                                                .await
                                        } else {
                                            Ok(())
                                        }
                                    })
                                } else {
                                    workspace.open_workspace_for_paths(false, paths, window, cx)
                                }
                            }
                            SerializedWorkspaceLocation::Remote(mut connection) => {
                                let app_state = workspace.app_state().clone();

                                let replace_window = if replace_current_window {
                                    window.window_handle().downcast::<Workspace>()
                                } else {
                                    None
                                };

                                let open_options = OpenOptions {
                                    replace_window,
                                    ..Default::default()
                                };

                                if let RemoteConnectionOptions::Ssh(connection) = &mut connection {
                                    SshSettings::get_global(cx)
                                        .fill_connection_options_from_settings(connection);
                                };

                                let paths = candidate_workspace_paths.paths().to_vec();

                                cx.spawn_in(window, async move |_, cx| {
                                    open_remote_project(
                                        connection.clone(),
                                        paths,
                                        app_state,
                                        open_options,
                                        cx,
                                    )
                                    .await
                                })
                            }
                        }
                    }
                })
                .detach_and_log_err(cx);
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        let text = if self.workspaces.is_empty() {
            "Recently opened projects will show up here".into()
        } else {
            "No matches".into()
        };
        Some(text)
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = self.matches.get(ix)?;

        let (_, location, paths) = self.workspaces.get(hit.candidate_id)?;

        let mut path_start_offset = 0;

        let (match_labels, paths): (Vec<_>, Vec<_>) = paths
            .paths()
            .iter()
            .map(|p| p.compact())
            .map(|path| {
                let highlighted_text =
                    highlights_for_path(path.as_ref(), &hit.positions, path_start_offset);
                path_start_offset += highlighted_text.1.text.len();
                highlighted_text
            })
            .unzip();

        let highlighted_match = HighlightedMatchWithPaths {
            match_label: HighlightedMatch::join(match_labels.into_iter().flatten(), ", "),
            paths,
        };

        Some(
            ListItem::new(ix)
                .toggle_state(selected)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .child(
                    h_flex()
                        .flex_grow()
                        .gap_3()
                        .when(self.has_any_non_local_projects, |this| {
                            this.child(match location {
                                SerializedWorkspaceLocation::Local => Icon::new(IconName::Screen)
                                    .color(Color::Muted)
                                    .into_any_element(),
                                SerializedWorkspaceLocation::Remote(options) => {
                                    Icon::new(match options {
                                        RemoteConnectionOptions::Ssh { .. } => IconName::Server,
                                        RemoteConnectionOptions::Wsl { .. } => IconName::Linux,
                                    })
                                    .color(Color::Muted)
                                    .into_any_element()
                                }
                            })
                        })
                        .child({
                            let mut highlighted = highlighted_match.clone();
                            if !self.render_paths {
                                highlighted.paths.clear();
                            }
                            highlighted.render(window, cx)
                        }),
                )
                .map(|el| {
                    let delete_button = div()
                        .child(
                            IconButton::new("delete", IconName::Close)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(move |this, _event, window, cx| {
                                    cx.stop_propagation();
                                    window.prevent_default();

                                    this.delegate.delete_recent_project(ix, window, cx)
                                }))
                                .tooltip(Tooltip::text("Delete from Recent Projects...")),
                        )
                        .into_any_element();

                    if self.selected_index() == ix {
                        el.end_slot::<AnyElement>(delete_button)
                    } else {
                        el.end_hover_slot::<AnyElement>(delete_button)
                    }
                })
                .tooltip(move |_, cx| {
                    let tooltip_highlighted_location = highlighted_match.clone();
                    cx.new(|_| MatchTooltip {
                        highlighted_location: tooltip_highlighted_location,
                    })
                    .into()
                }),
        )
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_2()
                .gap_2()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("remote", "Open Remote Folder")
                        .key_binding(KeyBinding::for_action(
                            &OpenRemote {
                                from_existing_connection: false,
                                create_new_window: false,
                            },
                            window,
                            cx,
                        ))
                        .on_click(|_, window, cx| {
                            window.dispatch_action(
                                OpenRemote {
                                    from_existing_connection: false,
                                    create_new_window: false,
                                }
                                .boxed_clone(),
                                cx,
                            )
                        }),
                )
                .child(
                    Button::new("local", "Open Local Folder")
                        .key_binding(KeyBinding::for_action(&workspace::Open, window, cx))
                        .on_click(|_, window, cx| {
                            window.dispatch_action(workspace::Open.boxed_clone(), cx)
                        }),
                )
                .into_any(),
        )
    }
}

// Compute the highlighted text for the name and path
fn highlights_for_path(
    path: &Path,
    match_positions: &Vec<usize>,
    path_start_offset: usize,
) -> (Option<HighlightedMatch>, HighlightedMatch) {
    let path_string = path.to_string_lossy();
    let path_text = path_string.to_string();
    let path_byte_len = path_text.len();
    // Get the subset of match highlight positions that line up with the given path.
    // Also adjusts them to start at the path start
    let path_positions = match_positions
        .iter()
        .copied()
        .skip_while(|position| *position < path_start_offset)
        .take_while(|position| *position < path_start_offset + path_byte_len)
        .map(|position| position - path_start_offset)
        .collect::<Vec<_>>();

    // Again subset the highlight positions to just those that line up with the file_name
    // again adjusted to the start of the file_name
    let file_name_text_and_positions = path.file_name().map(|file_name| {
        let file_name_text = file_name.to_string_lossy().into_owned();
        let file_name_start_byte = path_byte_len - file_name_text.len();
        let highlight_positions = path_positions
            .iter()
            .copied()
            .skip_while(|position| *position < file_name_start_byte)
            .take_while(|position| *position < file_name_start_byte + file_name_text.len())
            .map(|position| position - file_name_start_byte)
            .collect::<Vec<_>>();
        HighlightedMatch {
            text: file_name_text,
            highlight_positions,
            color: Color::Default,
        }
    });

    (
        file_name_text_and_positions,
        HighlightedMatch {
            text: path_text,
            highlight_positions: path_positions,
            color: Color::Default,
        },
    )
}
impl RecentProjectsDelegate {
    fn delete_recent_project(
        &self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if let Some(selected_match) = self.matches.get(ix) {
            let (workspace_id, _, _) = self.workspaces[selected_match.candidate_id];
            cx.spawn_in(window, async move |this, cx| {
                let _ = WORKSPACE_DB.delete_workspace_by_id(workspace_id).await;
                let workspaces = WORKSPACE_DB
                    .recent_workspaces_on_disk()
                    .await
                    .unwrap_or_default();
                this.update_in(cx, move |picker, window, cx| {
                    picker.delegate.set_workspaces(workspaces);
                    picker
                        .delegate
                        .set_selected_index(ix.saturating_sub(1), window, cx);
                    picker.delegate.reset_selected_match_index = false;
                    picker.update_matches(picker.query(cx), window, cx);
                    // After deleting a project, we want to update the history manager to reflect the change.
                    // But we do not emit a update event when user opens a project, because it's handled in `workspace::load_workspace`.
                    if let Some(history_manager) = HistoryManager::global(cx) {
                        history_manager
                            .update(cx, |this, cx| this.delete_history(workspace_id, cx));
                    }
                })
            })
            .detach();
        }
    }

    fn is_current_workspace(
        &self,
        workspace_id: WorkspaceId,
        cx: &mut Context<Picker<Self>>,
    ) -> bool {
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            if Some(workspace_id) == workspace.database_id() {
                return true;
            }
        }

        false
    }
}
struct MatchTooltip {
    highlighted_location: HighlightedMatchWithPaths,
}

impl Render for MatchTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(cx, |div, _| {
            self.highlighted_location.render_paths_children(div)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dap::debugger_settings::DebuggerSettings;
    use editor::Editor;
    use gpui::{TestAppContext, UpdateGlobal, WindowHandle};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::{AppState, open_paths};

    use super::*;

    #[gpui::test]
    async fn test_prompts_on_dirty_before_submit(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .session
                        .get_or_insert_default()
                        .restore_unsaved_buffers = Some(false)
                });
            });
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "main.ts": "a"
                }),
            )
            .await;
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir/main.ts"))],
                app_state,
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        let workspace = cx.update(|cx| cx.windows()[0].downcast::<Workspace>().unwrap());
        workspace
            .update(cx, |workspace, _, _| assert!(!workspace.is_edited()))
            .unwrap();

        let editor = workspace
            .read_with(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap()
            })
            .unwrap();
        workspace
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| editor.insert("EDIT", window, cx));
            })
            .unwrap();
        workspace
            .update(cx, |workspace, _, _| assert!(workspace.is_edited(), "After inserting more text into the editor without saving, we should have a dirty project"))
            .unwrap();

        let recent_projects_picker = open_recent_projects(&workspace, cx);
        workspace
            .update(cx, |_, _, cx| {
                recent_projects_picker.update(cx, |picker, cx| {
                    assert_eq!(picker.query(cx), "");
                    let delegate = &mut picker.delegate;
                    delegate.matches = vec![StringMatch {
                        candidate_id: 0,
                        score: 1.0,
                        positions: Vec::new(),
                        string: "fake candidate".to_string(),
                    }];
                    delegate.set_workspaces(vec![(
                        WorkspaceId::default(),
                        SerializedWorkspaceLocation::Local,
                        PathList::new(&[path!("/test/path")]),
                    )]);
                });
            })
            .unwrap();

        assert!(
            !cx.has_pending_prompt(),
            "Should have no pending prompt on dirty project before opening the new recent project"
        );
        cx.dispatch_action(*workspace, menu::Confirm);
        workspace
            .update(cx, |workspace, _, cx| {
                assert!(
                    workspace.active_modal::<RecentProjects>(cx).is_none(),
                    "Should remove the modal after selecting new recent project"
                )
            })
            .unwrap();
        assert!(
            cx.has_pending_prompt(),
            "Dirty workspace should prompt before opening the new recent project"
        );
        cx.simulate_prompt_answer("Cancel");
        assert!(
            !cx.has_pending_prompt(),
            "Should have no pending prompt after cancelling"
        );
        workspace
            .update(cx, |workspace, _, _| {
                assert!(
                    workspace.is_edited(),
                    "Should be in the same dirty project after cancelling"
                )
            })
            .unwrap();
    }

    fn open_recent_projects(
        workspace: &WindowHandle<Workspace>,
        cx: &mut TestAppContext,
    ) -> Entity<Picker<RecentProjectsDelegate>> {
        cx.dispatch_action(
            (*workspace).into(),
            OpenRecent {
                create_new_window: false,
            },
        );
        workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_modal::<RecentProjects>(cx)
                    .unwrap()
                    .read(cx)
                    .picker
                    .clone()
            })
            .unwrap()
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            language::init(cx);
            crate::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            DebuggerSettings::register(cx);
            Project::init_settings(cx);
            state
        })
    }
}
