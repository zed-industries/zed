//! # panel
use gpui::actions;
use ui::{Tab, prelude::*};

actions!(
    panel,
    [
        /// Navigates to the next tab in the panel.
        NextPanelTab,
        /// Navigates to the previous tab in the panel.
        PreviousPanelTab
    ]
);

pub trait PanelHeader: workspace::Panel {
    fn header_height(&self, cx: &mut App) -> Pixels {
        Tab::container_height(cx)
    }

    fn panel_header_container(&self, _window: &mut Window, cx: &mut App) -> Div {
        h_flex()
            .h(self.header_height(cx))
            .w_full()
            .px_1()
            .flex_none()
    }
}

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

pub fn panel_button(label: impl Into<SharedString>) -> ui::Button {
    let label = label.into();
    let id = ElementId::Name(label.to_lowercase().replace(' ', "_").into());
    ui::Button::new(id, label)
        .label_size(ui::LabelSize::Small)
        .icon_size(ui::IconSize::Small)
        // TODO: Change this once we use on_surface_bg in button_like
        .layer(ui::ElevationIndex::ModalSurface)
        .size(ui::ButtonSize::Compact)
}

pub fn panel_filled_button(label: impl Into<SharedString>) -> ui::Button {
    panel_button(label).style(ui::ButtonStyle::Filled)
}

pub fn panel_icon_button(id: impl Into<SharedString>, icon: IconName) -> ui::IconButton {
    let id = ElementId::Name(id.into());

    IconButton::new(id, icon)
        // TODO: Change this once we use on_surface_bg in button_like
        .layer(ui::ElevationIndex::ModalSurface)
}

pub fn panel_filled_icon_button(id: impl Into<SharedString>, icon: IconName) -> ui::IconButton {
    panel_icon_button(id, icon).style(ui::ButtonStyle::Filled)
}
