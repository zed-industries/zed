use gpui::{Anchor, Entity};
use ui::{
    ContextMenu, IconButton, IconName, IconPosition, IconSize, PopoverMenu, Tooltip, prelude::*,
};

use crate::{
    TabularDataPreviewPane,
    settings::{FilterSortOrder, VerticalAlignment},
};

///// Settings related /////
pub(crate) fn settings_popover_menu(view_entity: Entity<TabularDataPreviewPane>) -> PopoverMenu<ContextMenu> {
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

                    let menu = menu
                        .header("Text Alignment")
                        .toggleable_entry(
                            "Top",
                            matches!(settings.vertical_alignment, VerticalAlignment::Top),
                            IconPosition::Start,
                            None,
                            {
                                let view_entity = view_entity.clone();
                                move |_window, cx| {
                                    view_entity.update(cx, |this, cx| {
                                        this.settings.vertical_alignment = VerticalAlignment::Top;
                                        cx.notify();
                                    });
                                }
                            },
                        )
                        .toggleable_entry(
                            "Center",
                            matches!(settings.vertical_alignment, VerticalAlignment::Center),
                            IconPosition::Start,
                            None,
                            {
                                let view_entity = view_entity.clone();
                                move |_window, cx| {
                                    view_entity.update(cx, |this, cx| {
                                        this.settings.vertical_alignment =
                                            VerticalAlignment::Center;
                                        cx.notify();
                                    });
                                }
                            },
                        )
                        .separator()
                        .header("Filter Sort")
                        .toggleable_entry(
                            "A-Z, then Count",
                            settings.filter_sort_order == FilterSortOrder::AlphaThenCount,
                            IconPosition::Start,
                            None,
                            {
                                let view_entity = view_entity.clone();
                                move |_window, cx| {
                                    view_entity.update(cx, |this, cx| {
                                        this.settings.filter_sort_order =
                                            FilterSortOrder::AlphaThenCount;
                                        cx.notify();
                                    });
                                }
                            },
                        )
                        .toggleable_entry(
                            "Count, then A-Z",
                            settings.filter_sort_order == FilterSortOrder::CountThenAlpha,
                            IconPosition::Start,
                            None,
                            {
                                let view_entity = view_entity.clone();
                                move |_window, cx| {
                                    view_entity.update(cx, |this, cx| {
                                        this.settings.filter_sort_order =
                                            FilterSortOrder::CountThenAlpha;
                                        cx.notify();
                                    });
                                }
                            },
                        )
                        .separator()
                        .toggleable_entry(
                            "Display multiline rows",
                            settings.multiline_cells_enabled,
                            IconPosition::Start,
                            None,
                            {
                                let view_entity = view_entity.clone();
                                move |_window, cx| {
                                    view_entity.update(cx, |this, cx| {
                                        this.settings.multiline_cells_enabled =
                                            !this.settings.multiline_cells_enabled;
                                        cx.notify();
                                    });
                                }
                            },
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
    view_entity: &Entity<TableView>,
    settings: &crate::settings::TableViewSettings,
) -> ContextMenu {
    use crate::settings::RowRenderMechanism;

    menu.separator()
        .header("Dev-only: Rendering Mode")
        .toggleable_entry(
            "Variable Height",
            settings.rendering_with == RowRenderMechanism::VariableList,
            IconPosition::Start,
            None,
            {
                let view_entity = view_entity.clone();
                move |_window, cx| {
                    view_entity.update(cx, |view, cx| {
                        view.settings.rendering_with = RowRenderMechanism::VariableList;
                        cx.notify();
                    });
                }
            },
        )
        .toggleable_entry(
            "Uniform Height",
            settings.rendering_with == RowRenderMechanism::UniformList,
            IconPosition::Start,
            None,
            {
                let view_entity = view_entity.clone();
                move |_window, cx| {
                    view_entity.update(cx, |view, cx| {
                        view.settings.rendering_with = RowRenderMechanism::UniformList;
                        cx.notify();
                    });
                }
            },
        )
        .separator()
        .toggleable_entry(
            "Show perf metrics",
            settings.show_perf_metrics_overlay,
            IconPosition::Start,
            None,
            {
                let view_entity = view_entity.clone();
                move |_window, cx| {
                    view_entity.update(cx, |view, cx| {
                        view.settings.show_perf_metrics_overlay =
                            !view.settings.show_perf_metrics_overlay;
                        cx.notify();
                    });
                }
            },
        )
        .toggleable_entry(
            "Show cell positions",
            settings.show_debug_info,
            IconPosition::Start,
            None,
            {
                let view_entity = view_entity.clone();
                move |_window, cx| {
                    view_entity.update(cx, |view, cx| {
                        view.settings.show_debug_info = !view.settings.show_debug_info;
                        cx.notify();
                    });
                }
            },
        )
}
