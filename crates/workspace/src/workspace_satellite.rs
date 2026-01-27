use gpui::{EventEmitter, WeakEntity};
use ui::{ParentElement, Render, Styled, div};

use crate::{ActivePaneDecorator, PaneGroup, Workspace, client_side_decorations};

pub struct WorkspaceSatellite {
    pub(crate) center: PaneGroup,
    pub(crate) workspace: WeakEntity<Workspace>,
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
        let decorator = ActivePaneDecorator::new(&pane, &self.workspace);

        client_side_decorations(
            div()
                .size_full()
                .child(self.center.render(None, &decorator, window, cx)),
            window,
            cx,
        )
    }
}
