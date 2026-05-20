use gpui::actions;
use ui::prelude::*;

actions!(
    panel,
    [
        /// Navigates to the next tab in the panel.
        NextPanelTab,
        /// Navigates to the previous tab in the panel.
        PreviousPanelTab
    ]
);

pub trait PanelHeader: workspace::Panel {}

/// Implement this trait to enable a panel to have tabs.
pub trait PanelTabs: PanelHeader {
    /// Returns the index of the currently selected tab.
    fn selected_tab(&self, cx: &mut App) -> usize;
    /// Selects the tab at the given index.
    fn select_tab(&self, cx: &mut App, index: usize);
    /// Moves to the next tab.
    fn next_tab(&self, _: NextPanelTab, cx: &mut App) -> Self;
    /// Moves to the previous tab.
    fn previous_tab(&self, _: PreviousPanelTab, cx: &mut App) -> Self;
}

#[derive(IntoElement)]
pub struct PanelTab {}

impl RenderOnce for PanelTab {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
    }
}
