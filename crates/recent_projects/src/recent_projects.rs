mod highlighted_workspace_location;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    anyhow::Result,
    elements::{Flex, ParentElement},
    AnyElement, AppContext, Element, Task, ViewContext, WeakViewHandle,
};
use highlighted_workspace_location::HighlightedWorkspaceLocation;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate, PickerEvent};
use std::sync::Arc;
use workspace::{
    notifications::simple_message_notification::MessageNotification, Workspace, WorkspaceLocation,
    WORKSPACE_DB,
};

actions!(projects, [OpenRecent]);

pub fn init(cx: &mut AppContext) {
    cx.add_async_action(toggle);
    RecentProjects::init(cx);
}

fn toggle(
    _: &mut Workspace,
    _: &OpenRecent,
    cx: &mut ViewContext<Workspace>,
) -> Option<Task<Result<()>>> {
    Some(cx.spawn(|workspace, mut cx| async move {
        let workspace_locations: Vec<_> = cx
            .background()
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
                workspace.toggle_modal(cx, |_, cx| {
                    let workspace = cx.weak_handle();
                    cx.add_view(|cx| {
                        RecentProjects::new(
                            RecentProjectsDelegate::new(workspace, workspace_locations, true),
                            cx,
                        )
                        .with_max_size(800., 1200.)
                    })
                });
            } else {
                workspace.show_notification(0, cx, |cx| {
                    cx.add_view(|_| MessageNotification::new("No recent projects to open."))
                })
            }
        })?;
        Ok(())
    }))
}

pub fn build_recent_projects(
    workspace: WeakViewHandle<Workspace>,
    workspaces: Vec<WorkspaceLocation>,
    cx: &mut ViewContext<RecentProjects>,
) -> RecentProjects {
    Picker::new(
        RecentProjectsDelegate::new(workspace, workspaces, false),
        cx,
    )
    .with_theme(|theme| theme.picker.clone())
}

pub type RecentProjects = Picker<RecentProjectsDelegate>;

pub struct RecentProjectsDelegate {
    workspace: WeakViewHandle<Workspace>,
    workspace_locations: Vec<WorkspaceLocation>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
    render_paths: bool,
}

impl RecentProjectsDelegate {
    fn new(
        workspace: WeakViewHandle<Workspace>,
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
    fn placeholder_text(&self) -> Arc<str> {
        "Recent Projects...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<RecentProjects>) {
        self.selected_match_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<RecentProjects>,
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
                    .map(|path| {
                        let compact = util::paths::compact(&path);
                        compact.to_string_lossy().into_owned()
                    })
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
            cx.background().clone(),
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

    fn confirm(&mut self, cx: &mut ViewContext<RecentProjects>) {
        if let Some((selected_match, workspace)) = self
            .matches
            .get(self.selected_index())
            .zip(self.workspace.upgrade(cx))
        {
            let workspace_location = &self.workspace_locations[selected_match.candidate_id];
            workspace
                .update(cx, |workspace, cx| {
                    workspace
                        .open_workspace_for_paths(workspace_location.paths().as_ref().clone(), cx)
                })
                .detach_and_log_err(cx);
            cx.emit(PickerEvent::Dismiss);
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<RecentProjects>) {}

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut gpui::MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> AnyElement<Picker<Self>> {
        let theme = theme::current(cx);
        let style = theme.picker.item.in_state(selected).style_for(mouse_state);

        let string_match = &self.matches[ix];

        let highlighted_location = HighlightedWorkspaceLocation::new(
            &string_match,
            &self.workspace_locations[string_match.candidate_id],
        );

        Flex::column()
            .with_child(highlighted_location.names.render(style.label.clone()))
            .with_children(
                highlighted_location
                    .paths
                    .into_iter()
                    .filter(|_| self.render_paths)
                    .map(|highlighted_path| highlighted_path.render(style.label.clone())),
            )
            .flex(1., false)
            .contained()
            .with_style(style.container)
            .into_any_named("match")
    }
}
