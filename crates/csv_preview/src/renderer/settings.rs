use ui::{
    ActiveTheme as _, AnyElement, ButtonSize, Context, ContextMenu, DropdownMenu, ElementId,
    IntoElement as _, ParentElement as _, Styled as _, Tooltip, Window, div, h_flex,
};

use crate::{CsvPreviewView, settings::VerticalAlignment};

///// Settings related /////
impl CsvPreviewView {
    /// Render settings panel above the table
    pub(crate) fn render_settings_panel(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let current_alignment_text = match self.settings.vertical_alignment {
            VerticalAlignment::Top => "Top",
            VerticalAlignment::Center => "Center",
        };

        let view = cx.entity();
        let alignment_dropdown_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
            menu.entry("Top", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.vertical_alignment = VerticalAlignment::Top;
                        cx.notify();
                    });
                }
            })
            .entry("Center", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.vertical_alignment = VerticalAlignment::Center;
                        cx.notify();
                    });
                }
            })
        });

        let panel = h_flex()
            .gap_4()
            .p_2()
            .bg(cx.theme().colors().surface_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .flex_wrap()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().colors().text_muted)
                            .child("Text Alignment:"),
                    )
                    .child(
                        DropdownMenu::new(
                            ElementId::Name("vertical-alignment-dropdown".into()),
                            current_alignment_text,
                            alignment_dropdown_menu,
                        )
                        .trigger_size(ButtonSize::Compact)
                        .trigger_tooltip(Tooltip::text(
                            "Choose vertical text alignment within cells",
                        )),
                    ),
            );

        #[cfg(feature = "dev-tools")]
        let panel = panel.child(
            h_flex()
                .gap_2()
                .items_center()
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().colors().text_muted)
                        .child("Dev-only:"),
                )
                .child(create_dev_only_popover_menu(cx)),
        );

        panel.into_any_element()
    }
}

#[cfg(feature = "dev-tools")]
fn create_dev_only_popover_menu(
    cx: &mut Context<'_, CsvPreviewView>,
) -> ui::PopoverMenu<ContextMenu> {
    use crate::settings::RowRenderMechanism;
    use ui::{IconButton, IconName, IconPosition, IconSize, PopoverMenu};

    PopoverMenu::new("debug-options-menu")
        .trigger_with_tooltip(
            IconButton::new("debug-options-trigger", IconName::Settings).icon_size(IconSize::Small),
            Tooltip::text(
                "Dev-only section used for debugging purposes.\nWill be removed on public release of CSV feature"
            ),
        )
        .menu({
            let view_entity = cx.entity();
            move |window, cx| {
                let view = view_entity.read(cx);
                let settings = view.settings.clone();
                Some(ContextMenu::build(window, cx, |menu, _, _| {
                    menu.header("Rendering Mode")
                        .toggleable_entry(
                            "Variable Height",
                            settings.rendering_with == RowRenderMechanism::VariableList,
                            IconPosition::Start,
                            None,
                            {
                                let view_entity = view_entity.clone();
                                move |_w, cx| {
                                    view_entity.update(cx, |view, cx| {
                                        view.settings.rendering_with =
                                            RowRenderMechanism::VariableList;
                                        view.settings.multiline_cells_enabled = true;
                                        cx.notify();
                                    })
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
                                move |_w, cx| {
                                    view_entity.update(cx, |view, cx| {
                                        view.settings.rendering_with =
                                            RowRenderMechanism::UniformList;
                                        view.settings.multiline_cells_enabled = false;
                                        cx.notify();
                                    })
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
                                move |_w, cx| {
                                    view_entity.update(cx, |view, cx| {
                                        view.settings.show_perf_metrics_overlay =
                                            !view.settings.show_perf_metrics_overlay;
                                        cx.notify();
                                    })
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
                                move |_, cx| {
                                    view_entity.update(cx, |view, cx| {
                                        view.settings.show_debug_info =
                                            !view.settings.show_debug_info;
                                        cx.notify();
                                    })
                                }
                            },
                        )
                }))
            }
        })
}
