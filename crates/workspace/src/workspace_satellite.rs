use gpui::{App, Entity, EventEmitter, Focusable, Window, prelude::*};
use ui::{ParentElement, Render, Styled, div};

use crate::{ActivePaneDecorator, Pane, PaneGroup, Workspace, client_side_decorations, pane};

pub struct WorkspaceSatellite {
    pub(crate) center: PaneGroup,
    workspace: Entity<Workspace>,
}

impl WorkspaceSatellite {
    pub fn new(
        root: Entity<Pane>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut group = PaneGroup::new(root.clone());
        group.set_in_satellite(true);
        group.set_is_center(true);
        group.mark_positions(cx);

        // TODO: can we share this logic with workspace
        cx.subscribe_in(&root, window, Self::handle_pane_event)
            .detach();

        window.focus(&root.focus_handle(cx), cx);

        Self {
            center: group,
            workspace,
        }
    }

    fn handle_pane_event(
        &mut self,
        pane: &Entity<Pane>,
        event: &pane::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, pane::Event::Remove { .. })
            && self.center.first_pane() == self.center.last_pane()
        {
            window.remove_window();
            cx.emit(WorkspaceSatelliteEvent::Closing);
            return;
        }

        self.workspace.update(cx, move |workspace, cx| {
            workspace.handle_pane_event(pane, event, window, cx)
        })
    }
}

pub enum WorkspaceSatelliteEvent {
    Closing,
}

impl EventEmitter<WorkspaceSatelliteEvent> for WorkspaceSatellite {}

impl Render for WorkspaceSatellite {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let pane = self.center.first_pane();
        let weak_workspace = self.workspace.downgrade();
        let decorator = ActivePaneDecorator::new(&pane, &weak_workspace);

        client_side_decorations(
            div()
                .size_full()
                .child(self.center.render(None, &decorator, window, cx)),
            window,
            cx,
        )
    }
}

impl Focusable for WorkspaceSatellite {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.workspace.read(cx).active_pane.focus_handle(cx)
    }
}
