use std::sync::Arc;

use gpui::{elements::ChildView, Element, ElementBox, Entity, View, ViewContext, ViewHandle};
use theme::Theme;

use crate::{Pane, StatusItemView, Workspace};

#[derive(PartialEq, Eq, Default, Copy, Clone)]
pub enum DockPosition {
    #[default]
    Bottom,
    Right,
    Fullscreen,
}

pub struct Dock {
    position: Option<DockPosition>,
    pane: ViewHandle<Pane>,
}

impl Dock {
    pub fn new(cx: &mut ViewContext<Workspace>) -> Self {
        let pane = cx.add_view(Pane::new);
        Self {
            pane,
            position: None,
        }
    }

    pub fn render(&self, _theme: &Theme, position: DockPosition) -> Option<ElementBox> {
        if self.position.is_some() && self.position.unwrap() == position {
            Some(ChildView::new(self.pane.clone()).boxed())
        } else {
            None
        }
    }
}

pub struct ToggleDock {
    dock: Arc<Dock>,
}

impl ToggleDock {
    pub fn new(dock: Arc<Dock>, _cx: &mut ViewContext<Self>) -> Self {
        Self { dock }
    }
}

impl Entity for ToggleDock {
    type Event = ();
}

impl View for ToggleDock {
    fn ui_name() -> &'static str {
        "Dock Toggle"
    }
    // Shift-escape ON
    // Get or insert the dock's last focused terminal
    // Open the dock in fullscreen
    // Focus that terminal

    // Shift-escape OFF
    // Close the dock
    // Return focus to center

    // Behaviors:
    // If the dock is shown, hide it
    // If the dock is hidden, show it
    // If the dock was full screen, open it in last position (bottom or right)
    // If the dock was bottom or right, re-open it in that context (and with the previous % width)
    // On hover, change color and background
    // On shown, change color and background
    // On hidden, change color and background
    // Show tool tip
    fn render(&mut self, _cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        todo!()
    }
}

impl StatusItemView for ToggleDock {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn crate::ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
        //Not applicable
    }
}
