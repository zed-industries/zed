use ui::{
    ActiveTheme as _, AnyElement, ButtonSize, Checkbox, Context, ContextMenu, DropdownMenu,
    ElementId, IntoElement as _, ParentElement as _, Styled as _, ToggleState, Tooltip, Window,
    div, h_flex,
};

use crate::CsvPreviewView;

#[derive(Default)]
pub enum RowRenderMechanism {
    /// Default behaviour
    #[default]
    VariableList,
    /// More performance oriented, but all rows are same height
    UniformList,
}

#[derive(Default, Clone, Copy)]
pub enum VerticalAlignment {
    /// Align text to the top of cells
    #[default]
    Top,
    /// Center text vertically in cells
    Center,
}

#[derive(Default, Clone, Copy)]
pub enum FontType {
    /// Use the default UI font
    #[default]
    Ui,
    /// Use monospace font (same as buffer/editor font)
    Monospace,
}

#[derive(Default, Clone, Copy)]
pub enum RowIdentifiers {
    /// Show original line numbers from CSV file
    #[default]
    SrcLines,
    /// Show sequential row numbers starting from 1
    RowNum,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub(crate) enum CopyFormat {
    /// Copy as Tab-Separated Values (TSV)
    #[default]
    Tsv,
    /// Copy as Comma-Separated Values (CSV)
    Csv,
    /// Copy as Semicolon-Separated Values
    Semicolon,
    /// Copy as Markdown table
    Markdown,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub(crate) enum CopyMode {
    /// Copy in display order (what you see after sorting)
    #[default]
    Display,
    /// Copy in original file order (data coordinates)
    Data,
}

pub(crate) struct CsvPreviewSettings {
    pub(crate) rendering_with: RowRenderMechanism,
    pub(crate) vertical_alignment: VerticalAlignment,
    pub(crate) font_type: FontType,
    pub(crate) numbering_type: RowIdentifiers,
    pub(crate) copy_format: CopyFormat,
    pub(crate) copy_mode: CopyMode,
    pub(crate) show_debug_info: bool,
    pub(crate) show_perf_metrics_overlay: bool,
    pub(crate) show_cell_editor_row: bool,
}

impl Default for CsvPreviewSettings {
    fn default() -> Self {
        Self {
            rendering_with: RowRenderMechanism::default(),
            vertical_alignment: VerticalAlignment::default(),
            font_type: FontType::default(),
            numbering_type: RowIdentifiers::default(),
            copy_format: CopyFormat::default(),
            copy_mode: CopyMode::default(),
            show_debug_info: false,
            show_perf_metrics_overlay: false,
            show_cell_editor_row: false,
        }
    }
}

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
                        cx.notify();
                    });
                }
            })
            .entry("Uniform Height", None, {
                let view = view.clone();
                move |_window, cx| {
                    view.update(cx, |this, cx| {
                        this.settings.rendering_with = RowRenderMechanism::UniformList;
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
                    // TODO: Rewrite it to be a menu with checkable elements (✔️ next to it if checked)
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().colors().text_muted)
                                .child("Debug Info:"),
                        )
                        .child(
                            Checkbox::new(
                                "show-perf-metrics-overlay",
                                if self.settings.show_perf_metrics_overlay {
                                    ToggleState::Selected
                                } else {
                                    ToggleState::Unselected
                                },
                            )
                            .label("Show perf metrics")
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.settings.show_perf_metrics_overlay = *checked == ToggleState::Selected;
                                cx.notify();
                            })),
                        ).child(
                            Checkbox::new(
                                "show-cell-editor-row",
                                if self.settings.show_cell_editor_row {
                                    ToggleState::Selected
                                } else {
                                    ToggleState::Unselected
                                },
                            )
                            .label("Show cell editor row")
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.settings.show_cell_editor_row = *checked == ToggleState::Selected;
                                cx.notify();
                            })),
                        )
                        .child(
                            Checkbox::new(
                                "show-debug-info",
                                if self.settings.show_debug_info {
                                    ToggleState::Selected
                                } else {
                                    ToggleState::Unselected
                                },
                            )
                            .label("Show cell positions")
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.settings.show_debug_info = *checked == ToggleState::Selected;
                                cx.notify();
                            })),
                        ),
                )
                .into_any_element()
    }
}
