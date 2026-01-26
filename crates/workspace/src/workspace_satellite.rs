use gpui::{Entity, EventEmitter, WeakEntity};
use ui::{ParentElement, Render, Styled, div};

use crate::{ActivePaneDecorator, PaneGroup, Workspace};

pub struct WorkspaceSatellite {
    pub(crate) center: PaneGroup,
    pub(crate) workspace: WeakEntity<Workspace>,
}

pub enum WorkspaceSatelliteEvent {
    FocusedPane(Entity<Pane>),
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

        div()
            .size_full()
            .child(self.center.render(None, &decorator, window, cx))
    }
}
