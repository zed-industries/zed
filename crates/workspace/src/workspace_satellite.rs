use gpui::{Entity, EventEmitter, Focusable};
use ui::{ParentElement, Render, Styled, div};

use crate::{ActivePaneDecorator, PaneGroup, Workspace, client_side_decorations};

pub struct WorkspaceSatellite {
    pub(crate) center: PaneGroup,
    pub(crate) workspace: Entity<Workspace>,
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
