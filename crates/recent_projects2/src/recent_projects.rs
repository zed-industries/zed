mod highlighted_workspace_location;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, DismissEvent, Div, EventEmitter, FocusHandle, FocusableView, Result, Task,
    View, ViewContext, WeakView,
};
use highlighted_workspace_location::HighlightedWorkspaceLocation;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{prelude::*, ListItem};
use util::paths::PathExt;
use workspace::{
    notifications::simple_message_notification::MessageNotification, Workspace, WorkspaceLocation,
    WORKSPACE_DB,
};

actions!(OpenRecent);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(RecentProjects::register).detach();
}

pub struct RecentProjects {
    picker: View<Picker<RecentProjectsDelegate>>,
}

impl RecentProjects {
    fn new(delegate: RecentProjectsDelegate, cx: &mut ViewContext<Self>) -> Self {
        Self {
            picker: cx.build_view(|cx| Picker::new(delegate, cx)),
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
            let workspace_locations: Vec<_> = cx
                .background_executor()
                .spawn(async {
                    WORKSPACE_DB
                        .recent_workspaces_on_disk()
                        .await
                        .unwrap_or_default()
                        .into_iter()
                        .map(|(_, location)| location)
                        .collect()
                })
                .await;

            workspace.update(&mut cx, |workspace, cx| {
                if !workspace_locations.is_empty() {
                    let weak_workspace = cx.view().downgrade();
                    workspace.toggle_modal(cx, |cx| {
                        let delegate =
                            RecentProjectsDelegate::new(weak_workspace, workspace_locations, true);

                        RecentProjects::new(delegate, cx)
                    });
                } else {
                    workspace.show_notification(0, cx, |cx| {
                        cx.build_view(|_| MessageNotification::new("No recent projects to open."))
                    })
                }
            })?;
            Ok(())
        }))
    }
}

impl EventEmitter<DismissEvent> for RecentProjects {}

impl FocusableView for RecentProjects {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RecentProjects {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        v_stack().w(rems(34.)).child(self.picker.clone())
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
    fn new(
        workspace: WeakView<Workspace>,
        workspace_locations: Vec<WorkspaceLocation>,
        render_paths: bool,
    ) -> Self {
        Self {
            workspace,
            workspace_locations,
            selected_match_index: 0,
            matches: Default::default(),
            render_paths,
        }
    }
}

impl PickerDelegate for RecentProjectsDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self) -> Arc<str> {
        "Recent Projects...".into()
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
            self.dismissed(cx);
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

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

        Some(
            ListItem::new(ix).inset(true).selected(selected).child(
                v_stack()
                    .child(highlighted_location.names)
                    .when(self.render_paths, |this| {
                        this.children(highlighted_location.paths)
                    }),
            ),
        )
    }
}
