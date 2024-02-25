mod highlighted_workspace_location;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Result,
    Subscription, Task, View, ViewContext, WeakView,
};
use highlighted_workspace_location::HighlightedWorkspaceLocation;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{prelude::*, tooltip_container, HighlightedLabel, ListItem, ListItemSpacing, Tooltip};
use util::paths::PathExt;
use workspace::{ModalView, Workspace, WorkspaceId, WorkspaceLocation, WORKSPACE_DB};

gpui::actions!(projects, [OpenRecent]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(RecentProjects::register).detach();
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
                .unwrap_or_default();

            this.update(&mut cx, move |this, cx| {
                this.picker.update(cx, move |picker, cx| {
                    picker.delegate.workspaces = workspaces;
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
        workspace.register_action(|workspace, _: &OpenRecent, cx| {
            let Some(recent_projects) = workspace.active_modal::<Self>(cx) else {
                if let Some(handler) = Self::open(workspace, cx) {
                    handler.detach_and_log_err(cx);
                }
                return;
            };

            recent_projects.update(cx, |recent_projects, cx| {
                recent_projects
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(cx))
            });
        });
    }

    fn open(_: &mut Workspace, cx: &mut ViewContext<Workspace>) -> Option<Task<Result<()>>> {
        Some(cx.spawn(|workspace, mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| {
                let weak_workspace = cx.view().downgrade();
                workspace.toggle_modal(cx, |cx| {
                    let delegate = RecentProjectsDelegate::new(weak_workspace, true);

                    let modal = Self::new(delegate, 34., cx);
                    modal
                });
            })?;
            Ok(())
        }))
    }

    pub fn open_popover(workspace: WeakView<Workspace>, cx: &mut WindowContext<'_>) -> View<Self> {
        cx.new_view(|cx| Self::new(RecentProjectsDelegate::new(workspace, false), 20., cx))
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
    workspaces: Vec<(WorkspaceId, WorkspaceLocation)>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
    render_paths: bool,
    // Flag to reset index when there is a new query vs not reset index when user delete an item
    reset_selected_match_index: bool,
}

impl RecentProjectsDelegate {
    fn new(workspace: WeakView<Workspace>, render_paths: bool) -> Self {
        Self {
            workspace,
            workspaces: vec![],
            selected_match_index: 0,
            matches: Default::default(),
            render_paths,
            reset_selected_match_index: true,
        }
    }
}
impl EventEmitter<DismissEvent> for RecentProjectsDelegate {}
impl PickerDelegate for RecentProjectsDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, cx: &mut WindowContext) -> Arc<str> {
        let action_binding_text = |action: &dyn gpui::Action| {
            cx.bindings_for_action(action)
                .into_iter()
                .next()
                .map(|binding| {
                    binding
                        .keystrokes()
                        .into_iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_else(|| format!("{action:?}"))
        };
        Arc::from(format!(
            "{} reuses the window, {} opens a new one",
            action_binding_text(&menu::Confirm),
            action_binding_text(&menu::SecondaryConfirm),
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
                let combined_string = location
                    .paths()
                    .iter()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("");
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
            let replace_current_window = !secondary;
            workspace
                .update(cx, |workspace, cx| {
                    if workspace.database_id() != *candidate_workspace_id {
                        workspace.open_workspace_for_paths(
                            replace_current_window,
                            candidate_workspace_location.paths().as_ref().clone(),
                            cx,
                        )
                    } else {
                        Task::ready(Ok(()))
                    }
                })
                .detach_and_log_err(cx);
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let Some(r#match) = self.matches.get(ix) else {
            return None;
        };

        let (workspace_id, location) = &self.workspaces[r#match.candidate_id];
        let highlighted_location: HighlightedWorkspaceLocation =
            HighlightedWorkspaceLocation::new(&r#match, location);
        let tooltip_highlighted_location = highlighted_location.clone();

        let is_current_workspace = self.is_current_workspace(*workspace_id, cx);
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(
                    v_flex()
                        .child(highlighted_location.names)
                        .when(self.render_paths, |this| {
                            this.children(highlighted_location.paths.into_iter().map(|path| {
                                HighlightedLabel::new(path.text, path.highlight_positions)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            }))
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
                                .tooltip(|cx| Tooltip::text("Delete From Recent Projects...", cx)),
                        )
                        .into_any_element();

                    if self.selected_index() == ix {
                        el.end_slot::<AnyElement>(delete_button)
                    } else {
                        el.end_hover_slot::<AnyElement>(delete_button)
                    }
                })
                .tooltip(move |cx| {
                    let tooltip_highlighted_location = tooltip_highlighted_location.clone();
                    cx.new_view(move |_| MatchTooltip {
                        highlighted_location: tooltip_highlighted_location,
                    })
                    .into()
                }),
        )
    }
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
                    picker.delegate.workspaces = workspaces;
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
            if workspace_id == workspace.database_id() {
                return true;
            }
        }

        false
    }
}
struct MatchTooltip {
    highlighted_location: HighlightedWorkspaceLocation,
}

impl Render for MatchTooltip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        tooltip_container(cx, |div, _| {
            div.children(
                self.highlighted_location
                    .paths
                    .clone()
                    .into_iter()
                    .map(|path| {
                        HighlightedLabel::new(path.text, path.highlight_positions)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    }),
            )
        })
    }
}
