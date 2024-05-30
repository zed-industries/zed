mod dev_servers;

use client::ProjectId;
use dev_servers::reconnect_to_dev_server;
pub use dev_servers::DevServerProjects;
use feature_flags::FeatureFlagAppExt;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    Subscription, Task, View, ViewContext, WeakView,
};
use ordered_float::OrderedFloat;
use picker::{
    highlighted_match_with_paths::{HighlightedMatchWithPaths, HighlightedText},
    Picker, PickerDelegate,
};
use rpc::proto::DevServerStatus;
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use ui::{
    prelude::*, tooltip_container, ButtonLike, IconWithIndicator, Indicator, KeyBinding, ListItem,
    ListItemSpacing, Tooltip,
};
use util::{paths::PathExt, ResultExt};
use workspace::{
    AppState, ModalView, SerializedWorkspaceLocation, Workspace, WorkspaceId, WORKSPACE_DB,
};

#[derive(PartialEq, Clone, Deserialize, Default)]
pub struct OpenRecent {
    #[serde(default = "default_create_new_window")]
    pub create_new_window: bool,
}

fn default_create_new_window() -> bool {
    true
}

gpui::impl_actions!(projects, [OpenRecent]);
gpui::actions!(projects, [OpenRemote]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(RecentProjects::register).detach();
    cx.observe_new_views(DevServerProjects::register).detach();
}

pub struct RecentProjects {
    pub picker: View<Picker<RecentProjectsDelegate>>,
    rem_width: f32,
    _subscription: Subscription,
}

impl ModalView for RecentProjects {}

impl RecentProjects {
    fn new(delegate: RecentProjectsDelegate, rem_width: f32, cx: &mut ViewContext<Self>) -> Self {
        let picker = cx.new_view(|cx| {
            // We want to use a list when we render paths, because the items can have different heights (multiple paths).
            if delegate.render_paths {
                Picker::list(delegate, cx)
            } else {
                Picker::uniform_list(delegate, cx)
            }
        });
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));
        // We do not want to block the UI on a potentially lengthy call to DB, so we're gonna swap
        // out workspace locations once the future runs to completion.
        cx.spawn(|this, mut cx| async move {
            let workspaces = WORKSPACE_DB
                .recent_workspaces_on_disk()
                .await
                .log_err()
                .unwrap_or_default();
            this.update(&mut cx, move |this, cx| {
                this.picker.update(cx, move |picker, cx| {
                    picker.delegate.set_workspaces(workspaces);
                    picker.update_matches(picker.query(cx), cx)
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

    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, open_recent: &OpenRecent, cx| {
            let Some(recent_projects) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, open_recent.create_new_window, cx);
                return;
            };

            recent_projects.update(cx, |recent_projects, cx| {
                recent_projects
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(cx))
            });
        });
    }

    pub fn open(
        workspace: &mut Workspace,
        create_new_window: bool,
        cx: &mut ViewContext<Workspace>,
    ) {
        let weak = cx.view().downgrade();
        workspace.toggle_modal(cx, |cx| {
            let delegate = RecentProjectsDelegate::new(weak, create_new_window, true);
            let modal = Self::new(delegate, 34., cx);
            modal
        })
    }

    pub fn open_popover(workspace: WeakView<Workspace>, cx: &mut WindowContext<'_>) -> View<Self> {
        cx.new_view(|cx| {
            Self::new(
                RecentProjectsDelegate::new(workspace, false, false),
                20.,
                cx,
            )
        })
    }
}

impl EventEmitter<DismissEvent> for RecentProjects {}

impl FocusableView for RecentProjects {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RecentProjects {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(self.rem_width))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), cx);
                })
            }))
    }
}

pub struct RecentProjectsDelegate {
    workspace: WeakView<Workspace>,
    workspaces: Vec<(WorkspaceId, SerializedWorkspaceLocation)>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
    render_paths: bool,
    create_new_window: bool,
    // Flag to reset index when there is a new query vs not reset index when user delete an item
    reset_selected_match_index: bool,
    has_any_dev_server_projects: bool,
}

impl RecentProjectsDelegate {
    fn new(workspace: WeakView<Workspace>, create_new_window: bool, render_paths: bool) -> Self {
        Self {
            workspace,
            workspaces: Vec::new(),
            selected_match_index: 0,
            matches: Default::default(),
            create_new_window,
            render_paths,
            reset_selected_match_index: true,
            has_any_dev_server_projects: false,
        }
    }

    pub fn set_workspaces(&mut self, workspaces: Vec<(WorkspaceId, SerializedWorkspaceLocation)>) {
        self.workspaces = workspaces;
        self.has_any_dev_server_projects = self
            .workspaces
            .iter()
            .any(|(_, location)| matches!(location, SerializedWorkspaceLocation::DevServer(_)));
    }
}
impl EventEmitter<DismissEvent> for RecentProjectsDelegate {}
impl PickerDelegate for RecentProjectsDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, cx: &mut WindowContext) -> Arc<str> {
        let (create_window, reuse_window) = if self.create_new_window {
            (
                cx.keystroke_text_for(&menu::Confirm),
                cx.keystroke_text_for(&menu::SecondaryConfirm),
            )
        } else {
            (
                cx.keystroke_text_for(&menu::SecondaryConfirm),
                cx.keystroke_text_for(&menu::Confirm),
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

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_match_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim_start();
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let candidates = self
            .workspaces
            .iter()
            .enumerate()
            .map(|(id, (_, location))| {
                let combined_string = match location {
                    SerializedWorkspaceLocation::Local(paths, _) => paths
                        .paths()
                        .iter()
                        .map(|path| path.compact().to_string_lossy().into_owned())
                        .collect::<Vec<_>>()
                        .join(""),
                    SerializedWorkspaceLocation::DevServer(dev_server_project) => {
                        format!(
                            "{}{}",
                            dev_server_project.dev_server_name, dev_server_project.path
                        )
                    }
                };

                StringMatchCandidate::new(id, combined_string)
            })
            .collect::<Vec<_>>();
        self.matches = smol::block_on(fuzzy::match_strings(
            candidates.as_slice(),
            query,
            smart_case,
            100,
            &Default::default(),
            cx.background_executor().clone(),
        ));
        self.matches.sort_unstable_by_key(|m| m.candidate_id);

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

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some((selected_match, workspace)) = self
            .matches
            .get(self.selected_index())
            .zip(self.workspace.upgrade())
        {
            let (candidate_workspace_id, candidate_workspace_location) =
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
                        match candidate_workspace_location {
                            SerializedWorkspaceLocation::Local(paths, _) => {
                                let paths = paths.paths().as_ref().clone();
                                if replace_current_window {
                                    cx.spawn(move |workspace, mut cx| async move {
                                        let continue_replacing = workspace
                                            .update(&mut cx, |workspace, cx| {
                                                workspace.prepare_to_close(true, cx)
                                            })?
                                            .await?;
                                        if continue_replacing {
                                            workspace
                                                .update(&mut cx, |workspace, cx| {
                                                    workspace
                                                        .open_workspace_for_paths(true, paths, cx)
                                                })?
                                                .await
                                        } else {
                                            Ok(())
                                        }
                                    })
                                } else {
                                    workspace.open_workspace_for_paths(false, paths, cx)
                                }
                            }
                            SerializedWorkspaceLocation::DevServer(dev_server_project) => {
                                let store = dev_server_projects::Store::global(cx);
                                let Some(project_id) = store.read(cx)
                                    .dev_server_project(dev_server_project.id)
                                    .and_then(|p| p.project_id)
                                else {
                                    let server = store.read(cx).dev_server_for_project(dev_server_project.id);
                                    if server.is_some_and(|server| server.ssh_connection_string.is_some()) {
                                        let reconnect =  reconnect_to_dev_server(cx.view().clone(), server.unwrap().clone(), cx);
                                        let id = dev_server_project.id;
                                        return cx.spawn(|workspace, mut cx| async move {
                                            reconnect.await?;

                                            cx.background_executor().timer(Duration::from_millis(1000)).await;

                                            if let Some(project_id) = store.update(&mut cx, |store, _| {
                                                store.dev_server_project(id)
                                                    .and_then(|p| p.project_id)
                                            })? {
                                                    workspace.update(&mut cx, move |_, cx| {
                                                    open_dev_server_project(replace_current_window, project_id, cx)
                                                    })?.await?;
                                                }
                                            Ok(())
                                        })
                                    } else {
                                        let dev_server_name = dev_server_project.dev_server_name.clone();
                                        return cx.spawn(|workspace, mut cx| async move {
                                            let response =
                                                cx.prompt(gpui::PromptLevel::Warning,
                                                    "Dev Server is offline",
                                                    Some(format!("Cannot connect to {}. To debug open the remote project settings.", dev_server_name).as_str()),
                                                    &["Ok", "Open Settings"]
                                                ).await?;
                                            if response == 1 {
                                                workspace.update(&mut cx, |workspace, cx| {
                                                    let handle = cx.view().downgrade();
                                                    workspace.toggle_modal(cx, |cx| DevServerProjects::new(cx, handle))
                                                })?;
                                            } else {
                                                workspace.update(&mut cx, |workspace, cx| {
                                                    RecentProjects::open(workspace, true, cx);
                                                })?;
                                            }
                                            Ok(())
                                        })
                                    }
                                };
                                open_dev_server_project(replace_current_window, project_id, cx)
                        }
                    }
                }
                })
            .detach_and_log_err(cx);
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _: &mut ViewContext<Picker<Self>>) {}

    fn no_matches_text(&self, _cx: &mut WindowContext) -> SharedString {
        if self.workspaces.is_empty() {
            "Recently opened projects will show up here".into()
        } else {
            "No matches".into()
        }
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let Some(hit) = self.matches.get(ix) else {
            return None;
        };

        let (workspace_id, location) = &self.workspaces[hit.candidate_id];
        let is_current_workspace = self.is_current_workspace(*workspace_id, cx);

        let is_remote = matches!(location, SerializedWorkspaceLocation::DevServer(_));
        let dev_server_status =
            if let SerializedWorkspaceLocation::DevServer(dev_server_project) = location {
                let store = dev_server_projects::Store::global(cx).read(cx);
                Some(
                    store
                        .dev_server_project(dev_server_project.id)
                        .and_then(|p| store.dev_server(p.dev_server_id))
                        .map(|s| s.status)
                        .unwrap_or_default(),
                )
            } else {
                None
            };

        let mut path_start_offset = 0;
        let paths = match location {
            SerializedWorkspaceLocation::Local(paths, _) => paths.paths(),
            SerializedWorkspaceLocation::DevServer(dev_server_project) => {
                Arc::new(vec![PathBuf::from(format!(
                    "{}:{}",
                    dev_server_project.dev_server_name, dev_server_project.path
                ))])
            }
        };

        let (match_labels, paths): (Vec<_>, Vec<_>) = paths
            .iter()
            .map(|path| {
                let path = path.compact();
                let highlighted_text =
                    highlights_for_path(path.as_ref(), &hit.positions, path_start_offset);

                path_start_offset += highlighted_text.1.char_count;
                highlighted_text
            })
            .unzip();

        let highlighted_match = HighlightedMatchWithPaths {
            match_label: HighlightedText::join(match_labels.into_iter().flatten(), ", ").color(
                if matches!(dev_server_status, Some(DevServerStatus::Offline)) {
                    Color::Disabled
                } else {
                    Color::Default
                },
            ),
            paths,
        };

        Some(
            ListItem::new(ix)
                .selected(selected)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .child(
                    h_flex()
                        .flex_grow()
                        .gap_3()
                        .when(self.has_any_dev_server_projects, |this| {
                            this.child(if is_remote {
                                // if disabled, Color::Disabled
                                let indicator_color = match dev_server_status {
                                    Some(DevServerStatus::Online) => Color::Created,
                                    Some(DevServerStatus::Offline) => Color::Hidden,
                                    _ => unreachable!(),
                                };
                                IconWithIndicator::new(
                                    Icon::new(IconName::Server).color(Color::Muted),
                                    Some(Indicator::dot()),
                                )
                                .indicator_color(indicator_color)
                                .indicator_border_color(if selected {
                                    Some(cx.theme().colors().element_selected)
                                } else {
                                    None
                                })
                                .into_any_element()
                            } else {
                                Icon::new(IconName::Screen)
                                    .color(Color::Muted)
                                    .into_any_element()
                            })
                        })
                        .child({
                            let mut highlighted = highlighted_match.clone();
                            if !self.render_paths {
                                highlighted.paths.clear();
                            }
                            highlighted.render(cx)
                        }),
                )
                .when(!is_current_workspace, |el| {
                    let delete_button = div()
                        .child(
                            IconButton::new("delete", IconName::Close)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(move |this, _event, cx| {
                                    cx.stop_propagation();
                                    cx.prevent_default();

                                    this.delegate.delete_recent_project(ix, cx)
                                }))
                                .tooltip(|cx| Tooltip::text("Delete from Recent Projects...", cx)),
                        )
                        .into_any_element();

                    if self.selected_index() == ix {
                        el.end_slot::<AnyElement>(delete_button)
                    } else {
                        el.end_hover_slot::<AnyElement>(delete_button)
                    }
                })
                .tooltip(move |cx| {
                    let tooltip_highlighted_location = highlighted_match.clone();
                    cx.new_view(move |_| MatchTooltip {
                        highlighted_location: tooltip_highlighted_location,
                    })
                    .into()
                }),
        )
    }

    fn render_footer(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        if !cx.has_flag::<feature_flags::Remoting>() {
            return None;
        }
        Some(
            h_flex()
                .border_t_1()
                .py_2()
                .pr_2()
                .border_color(cx.theme().colors().border)
                .justify_end()
                .gap_4()
                .child(
                    ButtonLike::new("remote")
                        .when_some(KeyBinding::for_action(&OpenRemote, cx), |button, key| {
                            button.child(key)
                        })
                        .child(Label::new("New remote project…").color(Color::Muted))
                        .on_click(|_, cx| cx.dispatch_action(OpenRemote.boxed_clone())),
                )
                .child(
                    ButtonLike::new("local")
                        .when_some(
                            KeyBinding::for_action(&workspace::Open, cx),
                            |button, key| button.child(key),
                        )
                        .child(Label::new("Open local folder…").color(Color::Muted))
                        .on_click(|_, cx| cx.dispatch_action(workspace::Open.boxed_clone())),
                )
                .into_any(),
        )
    }
}

fn open_dev_server_project(
    replace_current_window: bool,
    project_id: ProjectId,
    cx: &mut ViewContext<Workspace>,
) -> Task<anyhow::Result<()>> {
    if let Some(app_state) = AppState::global(cx).upgrade() {
        let handle = if replace_current_window {
            cx.window_handle().downcast::<Workspace>()
        } else {
            None
        };

        if let Some(handle) = handle {
            cx.spawn(move |workspace, mut cx| async move {
                let continue_replacing = workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.prepare_to_close(true, cx)
                    })?
                    .await?;
                if continue_replacing {
                    workspace
                        .update(&mut cx, |_workspace, cx| {
                            workspace::join_dev_server_project(
                                project_id,
                                app_state,
                                Some(handle),
                                cx,
                            )
                        })?
                        .await?;
                }
                Ok(())
            })
        } else {
            let task = workspace::join_dev_server_project(project_id, app_state, None, cx);
            cx.spawn(|_, _| async move {
                task.await?;
                Ok(())
            })
        }
    } else {
        Task::ready(Err(anyhow::anyhow!("App state not found")))
    }
}

// Compute the highlighted text for the name and path
fn highlights_for_path(
    path: &Path,
    match_positions: &Vec<usize>,
    path_start_offset: usize,
) -> (Option<HighlightedText>, HighlightedText) {
    let path_string = path.to_string_lossy();
    let path_char_count = path_string.chars().count();
    // Get the subset of match highlight positions that line up with the given path.
    // Also adjusts them to start at the path start
    let path_positions = match_positions
        .iter()
        .copied()
        .skip_while(|position| *position < path_start_offset)
        .take_while(|position| *position < path_start_offset + path_char_count)
        .map(|position| position - path_start_offset)
        .collect::<Vec<_>>();

    // Again subset the highlight positions to just those that line up with the file_name
    // again adjusted to the start of the file_name
    let file_name_text_and_positions = path.file_name().map(|file_name| {
        let text = file_name.to_string_lossy();
        let char_count = text.chars().count();
        let file_name_start = path_char_count - char_count;
        let highlight_positions = path_positions
            .iter()
            .copied()
            .skip_while(|position| *position < file_name_start)
            .take_while(|position| *position < file_name_start + char_count)
            .map(|position| position - file_name_start)
            .collect::<Vec<_>>();
        HighlightedText {
            text: text.to_string(),
            highlight_positions,
            char_count,
            color: Color::Default,
        }
    });

    (
        file_name_text_and_positions,
        HighlightedText {
            text: path_string.to_string(),
            highlight_positions: path_positions,
            char_count: path_char_count,
            color: Color::Default,
        },
    )
}

impl RecentProjectsDelegate {
    fn delete_recent_project(&self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(selected_match) = self.matches.get(ix) {
            let (workspace_id, _) = self.workspaces[selected_match.candidate_id];
            cx.spawn(move |this, mut cx| async move {
                let _ = WORKSPACE_DB.delete_workspace_by_id(workspace_id).await;
                let workspaces = WORKSPACE_DB
                    .recent_workspaces_on_disk()
                    .await
                    .unwrap_or_default();
                this.update(&mut cx, move |picker, cx| {
                    picker.delegate.set_workspaces(workspaces);
                    picker.delegate.set_selected_index(ix - 1, cx);
                    picker.delegate.reset_selected_match_index = false;
                    picker.update_matches(picker.query(cx), cx)
                })
            })
            .detach();
        }
    }

    fn is_current_workspace(
        &self,
        workspace_id: WorkspaceId,
        cx: &mut ViewContext<Picker<Self>>,
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        tooltip_container(cx, |div, _| {
            self.highlighted_location.render_paths_children(div)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use editor::Editor;
    use gpui::{TestAppContext, WindowHandle};
    use project::Project;
    use serde_json::json;
    use workspace::{open_paths, AppState, LocalPaths};

    use super::*;

    #[gpui::test]
    async fn test_prompts_on_dirty_before_submit(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/dir",
                json!({
                    "main.ts": "a"
                }),
            )
            .await;
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/dir/main.ts")],
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
            .update(cx, |workspace, _| assert!(!workspace.is_edited()))
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
            .update(cx, |_, cx| {
                editor.update(cx, |editor, cx| editor.insert("EDIT", cx));
            })
            .unwrap();
        workspace
            .update(cx, |workspace, _| assert!(workspace.is_edited(), "After inserting more text into the editor without saving, we should have a dirty project"))
            .unwrap();

        let recent_projects_picker = open_recent_projects(&workspace, cx);
        workspace
            .update(cx, |_, cx| {
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
                        LocalPaths::new(vec!["/test/path/"]).into(),
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
            .update(cx, |workspace, cx| {
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
        // Cancel
        cx.simulate_prompt_answer(0);
        assert!(
            !cx.has_pending_prompt(),
            "Should have no pending prompt after cancelling"
        );
        workspace
            .update(cx, |workspace, _| {
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
    ) -> View<Picker<RecentProjectsDelegate>> {
        cx.dispatch_action(
            (*workspace).into(),
            OpenRecent {
                create_new_window: false,
            },
        );
        workspace
            .update(cx, |workspace, cx| {
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
            Project::init_settings(cx);
            state
        })
    }
}
