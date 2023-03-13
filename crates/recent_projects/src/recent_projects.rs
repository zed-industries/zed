mod highlighted_workspace_location;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    elements::{ChildView, Flex, ParentElement},
    AnyViewHandle, Element, ElementBox, Entity, MutableAppContext, RenderContext, Task, View,
    ViewContext, ViewHandle,
};
use highlighted_workspace_location::HighlightedWorkspaceLocation;
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use settings::Settings;
use workspace::{
    notifications::simple_message_notification::MessageNotification, OpenPaths, Workspace,
    WorkspaceLocation, WORKSPACE_DB,
};

actions!(projects, [OpenRecent]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(RecentProjectsView::toggle);
    Picker::<RecentProjectsView>::init(cx);
}

struct RecentProjectsView {
    picker: ViewHandle<Picker<Self>>,
    workspace_locations: Vec<WorkspaceLocation>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
}

impl RecentProjectsView {
    fn new(workspace_locations: Vec<WorkspaceLocation>, cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.weak_handle();
        Self {
            picker: cx.add_view(|cx| {
                Picker::new("Recent Projects...", handle, cx).with_max_size(800., 1200.)
            }),
            workspace_locations,
            selected_match_index: 0,
            matches: Default::default(),
        }
    }

    fn toggle(_: &mut Workspace, _: &OpenRecent, cx: &mut ViewContext<Workspace>) {
        cx.spawn(|workspace, mut cx| async move {
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
                        let view = cx.add_view(|cx| Self::new(workspace_locations, cx));
                        cx.subscribe(&view, Self::on_event).detach();
                        view
                    });
                } else {
                    workspace.show_notification(0, cx, |cx| {
                        cx.add_view(|_| {
                            MessageNotification::new_message("No recent projects to open.")
                        })
                    })
                }
            });
        })
        .detach();
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => workspace.dismiss_modal(cx),
        }
    }
}

pub enum Event {
    Dismissed,
}

impl Entity for RecentProjectsView {
    type Event = Event;
}

impl View for RecentProjectsView {
    fn ui_name() -> &'static str {
        "RecentProjectsView"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.picker.clone(), cx).boxed()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.picker);
        }
    }
}

impl PickerDelegate for RecentProjectsView {
    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Self>) {
        self.selected_match_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> gpui::Task<()> {
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
                    .map(|path| path.to_string_lossy().to_owned())
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

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selected_match) = &self.matches.get(self.selected_index()) {
            let workspace_location = &self.workspace_locations[selected_match.candidate_id];
            cx.dispatch_global_action(OpenPaths {
                paths: workspace_location.paths().as_ref().clone(),
            });
            cx.emit(Event::Dismissed);
        }
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut gpui::MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> ElementBox {
        let settings = cx.global::<Settings>();
        let string_match = &self.matches[ix];
        let style = settings.theme.picker.item.style_for(mouse_state, selected);

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
                    .map(|highlighted_path| highlighted_path.render(style.label.clone())),
            )
            .flex(1., false)
            .contained()
            .with_style(style.container)
            .named("match")
    }
}
