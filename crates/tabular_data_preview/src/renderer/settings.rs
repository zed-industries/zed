use gpui::{Anchor, Entity};
use ui::{
    ContextMenu, IconButton, IconName, IconPosition, IconSize, PopoverMenu, Tooltip, prelude::*,
};

use crate::{
    TabularDataPreviewPane,
    settings::{FilterSortOrder, TabularDataPreviewSettings, VerticalAlignment},
};

///// Settings related /////

/// Adds a toggleable entry that applies `set` to the pane's settings when clicked.
fn toggle_entry(
    menu: ContextMenu,
    label: &'static str,
    selected: bool,
    view_entity: &Entity<TabularDataPreviewPane>,
    set: impl Fn(&mut TabularDataPreviewSettings) + 'static,
) -> ContextMenu {
    let view_entity = view_entity.clone();
    menu.toggleable_entry(label, selected, IconPosition::Start, None, move |_, cx| {
        view_entity.update(cx, |this, cx| {
            set(&mut this.settings);
            cx.notify();
        });
    })
}

pub(crate) fn settings_popover_menu(
    view_entity: Entity<TabularDataPreviewPane>,
) -> PopoverMenu<ContextMenu> {
    PopoverMenu::new("table-settings-menu")
        .trigger_with_tooltip(
            IconButton::new("table-settings-trigger", IconName::Settings)
                .icon_size(IconSize::Small)
                .size(ButtonSize::Compact),
            Tooltip::text("Table Settings"),
        )
        .anchor(Anchor::TopRight)
        .menu(move |window, cx| {
            let view_entity = view_entity.clone();
            Some(ContextMenu::build_persistent(
                window,
                cx,
                move |menu, _window, cx| {
                    let settings = view_entity.read(cx).settings.clone();

                    let menu = toggle_entry(
                        menu.header("Text Alignment"),
                        "Top",
                        matches!(settings.vertical_alignment, VerticalAlignment::Top),
                        &view_entity,
                        |settings| settings.vertical_alignment = VerticalAlignment::Top,
                    );
                    let menu = toggle_entry(
                        menu,
                        "Center",
                        matches!(settings.vertical_alignment, VerticalAlignment::Center),
                        &view_entity,
                        |settings| settings.vertical_alignment = VerticalAlignment::Center,
                    );

                    let menu = menu.separator().header("Filter Sort");
                    let menu = toggle_entry(
                        menu,
                        "A-Z, then Count",
                        settings.filter_sort_order == FilterSortOrder::AlphaThenCount,
                        &view_entity,
                        |settings| settings.filter_sort_order = FilterSortOrder::AlphaThenCount,
                    );
                    let menu = toggle_entry(
                        menu,
                        "Count, then A-Z",
                        settings.filter_sort_order == FilterSortOrder::CountThenAlpha,
                        &view_entity,
                        |settings| settings.filter_sort_order = FilterSortOrder::CountThenAlpha,
                    );

                    let menu = toggle_entry(
                        menu.separator(),
                        "Display multiline rows",
                        settings.multiline_cells_enabled,
                        &view_entity,
                        |settings| settings.multiline_cells_enabled = !settings.multiline_cells_enabled,
                    );

                    #[cfg(feature = "dev-tools")]
                    let menu = append_dev_only_entries(menu, &view_entity, &settings);

                    menu
                },
            ))
        })
}

#[cfg(feature = "dev-tools")]
fn append_dev_only_entries(
    menu: ContextMenu,
    view_entity: &Entity<TabularDataPreviewPane>,
    settings: &TabularDataPreviewSettings,
) -> ContextMenu {
    use crate::settings::RowRenderMechanism;

    let menu = menu.separator().header("Dev-only: Rendering Mode");
    let menu = toggle_entry(
        menu,
        "Variable Height",
        settings.rendering_with == RowRenderMechanism::VariableList,
        view_entity,
        |settings| settings.rendering_with = RowRenderMechanism::VariableList,
    );
    let menu = toggle_entry(
        menu,
        "Uniform Height",
        settings.rendering_with == RowRenderMechanism::UniformList,
        view_entity,
        |settings| settings.rendering_with = RowRenderMechanism::UniformList,
    );

    let menu = toggle_entry(
        menu.separator(),
        "Show perf metrics",
        settings.show_perf_metrics_overlay,
        view_entity,
        |settings| settings.show_perf_metrics_overlay = !settings.show_perf_metrics_overlay,
    );
    toggle_entry(
        menu,
        "Show cell positions",
        settings.show_debug_info,
        view_entity,
        |settings| settings.show_debug_info = !settings.show_debug_info,
    )
}
