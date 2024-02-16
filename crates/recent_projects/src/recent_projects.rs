mod highlighted_workspace_location;
mod projects;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Result, Subscription, Task,
    View, ViewContext, WeakView,
};
use highlighted_workspace_location::HighlightedWorkspaceLocation;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{prelude::*, tooltip_container, HighlightedLabel, ListItem, ListItemSpacing};
use util::paths::PathExt;
use workspace::{ModalView, Workspace, WorkspaceLocation, WORKSPACE_DB};

pub use projects::OpenRecent;

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
                .unwrap_or_default()
                .into_iter()
                .map(|(_, location)| location)
                .collect();
            this.update(&mut cx, move |this, cx| {
                this.picker.update(cx, move |picker, cx| {
                    picker.delegate.workspace_locations = workspaces;
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
    workspace_locations: Vec<WorkspaceLocation>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
    render_paths: bool,
}

impl RecentProjectsDelegate {
    fn new(workspace: WeakView<Workspace>, render_paths: bool) -> Self {
        Self {
            workspace,
            workspace_locations: vec![],
            selected_match_index: 0,
            matches: Default::default(),
            render_paths,
        }
    }
}
impl EventEmitter<DismissEvent> for RecentProjectsDelegate {}
impl PickerDelegate for RecentProjectsDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self) -> Arc<str> {
        "Search recent projects...".into()
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
            .workspace_locations
            .iter()
            .enumerate()
            .map(|(id, location)| {
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

        self.selected_match_index = self
            .matches
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, m)| OrderedFloat(m.score))
            .map(|(ix, _)| ix)
            .unwrap_or(0);
        Task::ready(())
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some((selected_match, workspace)) = self
            .matches
            .get(self.selected_index())
            .zip(self.workspace.upgrade())
        {
            let workspace_location = &self.workspace_locations[selected_match.candidate_id];
            workspace
                .update(cx, |workspace, cx| {
                    workspace
                        .open_workspace_for_paths(workspace_location.paths().as_ref().clone(), cx)
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
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let Some(r#match) = self.matches.get(ix) else {
            return None;
        };

        let highlighted_location = HighlightedWorkspaceLocation::new(
            &r#match,
            &self.workspace_locations[r#match.candidate_id],
        );

        let tooltip_highlighted_location = highlighted_location.clone();

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
