use ui::{
    ActiveTheme as _, AnyElement, ButtonSize, Context, ContextMenu, DropdownMenu, ElementId,
    IconButton, IconName, IconPosition, IconSize, IntoElement as _, ParentElement as _,
    PopoverMenu, Styled as _, Tooltip, Window, div, h_flex,
};

use crate::{
    CsvPreviewView,
    settings::{CopyFormat, CopyMode, FontType, RowRenderMechanism, VerticalAlignment},
};

///// Settings related /////
impl CsvPreviewView {
    /// Render settings panel above the table
    pub(crate) fn render_settings_panel(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let current_mode_text = match self.settings.rendering_with {
            RowRenderMechanism::VariableList => "Variable Height",
            RowRenderMechanism::UniformList => "Uniform Height",
        };

        let current_alignment_text = match self.settings.vertical_alignment {
            VerticalAlignment::Top => "Top",
            VerticalAlignment::Center => "Center",
        };

        let current_font_text = match self.settings.font_type {
            FontType::Ui => "UI Font",
            FontType::Monospace => "Monospace",
        };

        let current_copy_format_text = match self.settings.copy_format {
            CopyFormat::Tsv => "TSV (Tab)",
            CopyFormat::Csv => "CSV (Comma)",
            CopyFormat::Semicolon => "Semicolon",
            CopyFormat::Markdown => "Markdown",
        };

        let current_copy_mode_text = match self.settings.copy_mode {
            CopyMode::Display => "Display Order",
            CopyMode::Data => "File Order",
        };

        let view = cx.entity();
        let rendering_dropdown_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
            menu.entry("Variable Height", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.rendering_with = RowRenderMechanism::VariableList;
                        this.settings.multiline_cells_enabled = true;
                        cx.notify();
                    });
                }
            })
            .entry("Uniform Height", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.rendering_with = RowRenderMechanism::UniformList;
                        this.settings.multiline_cells_enabled = false; // Uniform list doesn't support multiline properly
                        cx.notify();
                    });
                }
            })
        });

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

        let font_dropdown_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
            menu.entry("UI Font", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.font_type = FontType::Ui;
                        cx.notify();
                    });
                }
            })
            .entry("Monospace", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.font_type = FontType::Monospace;
                        cx.notify();
                    });
                }
            })
        });

        let copy_format_dropdown_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
            menu.entry("TSV (Tab)", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.copy_format = CopyFormat::Tsv;
                        cx.notify();
                    });
                }
            })
            .entry("CSV (Comma)", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.copy_format = CopyFormat::Csv;
                        cx.notify();
                    });
                }
            })
            .entry("Semicolon", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.copy_format = CopyFormat::Semicolon;
                        cx.notify();
                    });
                }
            })
            .entry("Markdown", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.copy_format = CopyFormat::Markdown;
                        cx.notify();
                    });
                }
            })
        });

        let copy_mode_dropdown_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
            menu.entry("Display Order", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.copy_mode = CopyMode::Display;
                        cx.notify();
                    });
                }
            })
            .entry("File Order", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.copy_mode = CopyMode::Data;
                        cx.notify();
                    });
                }
            })
        });

        h_flex()
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
                                .child("Rendering Mode:"),
                        )
                        .child(
                            DropdownMenu::new(
                                ElementId::Name("rendering-mode-dropdown".into()),
                                current_mode_text,
                                rendering_dropdown_menu,
                            )
                            .trigger_size(ButtonSize::Compact)
                            .trigger_tooltip(Tooltip::text("Choose between variable height (multiline support) or uniform height (better performance)"))
                        ),
                )
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
                            .trigger_tooltip(Tooltip::text("Choose vertical text alignment within cells"))
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().colors().text_muted)
                                .child("Font Type:"),
                        )
                        .child(
                            DropdownMenu::new(
                                ElementId::Name("font-type-dropdown".into()),
                                current_font_text,
                                font_dropdown_menu,
                            )
                            .trigger_size(ButtonSize::Compact)
                            .trigger_tooltip(Tooltip::text("Choose between UI font and monospace font for better readability"))
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().colors().text_muted)
                                .child("Copy Format:"),
                        )
                        .child(
                            DropdownMenu::new(
                                ElementId::Name("copy-format-dropdown".into()),
                                current_copy_format_text,
                                copy_format_dropdown_menu,
                            )
                            .trigger_size(ButtonSize::Compact)
                            .trigger_tooltip(Tooltip::text("Choose format for copying selected cells (CSV, TSV, Semicolon, or Markdown table)"))
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().colors().text_muted)
                                .child("Copy Order:"),
                        )
                        .child(
                            DropdownMenu::new(
                                ElementId::Name("copy-mode-dropdown".into()),
                                current_copy_mode_text,
                                copy_mode_dropdown_menu,
                            )
                            .trigger_size(ButtonSize::Compact)
                            .trigger_tooltip(Tooltip::text("Choose whether to copy in display order (what you see) or file order (original data)"))
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().colors().text_muted)
                                .child("Experimental:"),
                        )
                        .child(create_experimental_popover_menu(cx))
                )
                .into_any_element()
    }
}

fn create_experimental_popover_menu(
    cx: &mut Context<'_, CsvPreviewView>,
) -> PopoverMenu<ContextMenu> {
    PopoverMenu::new("debug-options-menu")
        .trigger_with_tooltip(
            IconButton::new("debug-options-trigger", IconName::Settings).icon_size(IconSize::Small),
            Tooltip::text("Experimental"),
        )
        .menu({
            let view_entity = cx.entity();
            move |window, cx| {
                let view = view_entity.read(cx);
                let settings = view.settings.clone();
                Some(ContextMenu::build(window, cx, |menu, _, _| {
                    menu.toggleable_entry(
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
                        "Show cell editor row",
                        settings.show_cell_editor_row,
                        IconPosition::Start,
                        None,
                        {
                            let view_entity = view_entity.clone();
                            move |_, cx| {
                                view_entity.update(cx, |view, cx| {
                                    view.settings.show_cell_editor_row =
                                        !view.settings.show_cell_editor_row;
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
                                    view.settings.show_debug_info = !view.settings.show_debug_info;
                                    cx.notify();
                                })
                            }
                        },
                    )
                }))
            }
        })
}
