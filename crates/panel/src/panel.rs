//! # panel
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{Entity, TextStyle, actions};
use settings::Settings;
use theme::ThemeSettings;
use ui::{Tab, prelude::*};

actions!(panel, [NextPanelTab, PreviousPanelTab]);

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
    let id = ElementId::Name(label.clone().to_lowercase().replace(' ', "_").into());
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
    ui::IconButton::new(id, icon)
        // TODO: Change this once we use on_surface_bg in button_like
        .layer(ui::ElevationIndex::ModalSurface)
        .size(ui::ButtonSize::Compact)
}

pub fn panel_filled_icon_button(id: impl Into<SharedString>, icon: IconName) -> ui::IconButton {
    panel_icon_button(id, icon).style(ui::ButtonStyle::Filled)
}

pub fn panel_editor_container(_window: &mut Window, cx: &mut App) -> Div {
    v_flex()
        .size_full()
        .gap(px(8.))
        .p_2()
        .bg(cx.theme().colors().editor_background)
}

pub fn panel_editor_style(monospace: bool, window: &Window, cx: &App) -> EditorStyle {
    let settings = ThemeSettings::get_global(cx);

    let font_size = TextSize::Small.rems().to_pixels(window.rem_size());

    let (font_family, font_fallbacks, font_features, font_weight, line_height) = if monospace {
        (
            settings.buffer_font.family.clone(),
            settings.buffer_font.fallbacks.clone(),
            settings.buffer_font.features.clone(),
            settings.buffer_font.weight,
            font_size * settings.buffer_line_height.value(),
        )
    } else {
        (
            settings.ui_font.family.clone(),
            settings.ui_font.fallbacks.clone(),
            settings.ui_font.features.clone(),
            settings.ui_font.weight,
            window.line_height(),
        )
    };

    EditorStyle {
        background: cx.theme().colors().editor_background,
        local_player: cx.theme().players().local(),
        text: TextStyle {
            color: cx.theme().colors().text,
            font_family,
            font_fallbacks,
            font_features,
            font_size: TextSize::Small.rems().into(),
            font_weight,
            line_height: line_height.into(),
            ..Default::default()
        },
        syntax: cx.theme().syntax().clone(),
        ..Default::default()
    }
}

pub fn panel_editor_element(
    editor: &Entity<Editor>,
    monospace: bool,
    window: &mut Window,
    cx: &mut App,
) -> EditorElement {
    EditorElement::new(editor, panel_editor_style(monospace, window, cx))
}
