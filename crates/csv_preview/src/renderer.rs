use gpui::{AnyElement, ElementId, Entity, MouseButton};
use std::ops::Range;
use ui::{
    Button, ButtonSize, ButtonStyle, ContextMenu, DefiniteLength, DropdownMenu, SharedString,
    Table, TableColumnWidths, TableResizeBehavior, Tooltip, div, h_flex, prelude::*,
};

use crate::{
    CsvPreviewView, FontType, NumberingType, Ordering, RowRenderMechanism, VerticalAlignment,
    cell_selection::TableSelection,
    data_ordering::{OrderingDirection, generate_ordered_indices},
};

impl Render for CsvPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .w_full()
            .h_full()
            .p_4()
            .bg(theme.colors().editor_background)
            .child(self.render_settings_panel(window, cx))
            .child({
                if self.contents.headers.is_empty() {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_32()
                        .text_ui(cx)
                        .when(
                            matches!(self.settings.font_type, crate::FontType::Ui),
                            |div| div.font_ui(cx),
                        )
                        .when(
                            matches!(self.settings.font_type, crate::FontType::Monospace),
                            |div| div.font_buffer(cx),
                        )
                        .text_color(cx.theme().colors().text_muted)
                        .child("No CSV content to display")
                        .into_any_element()
                } else {
                    self.render_table_with_cols(cx)
                }
            })
            // Workaround to be able to `end_selection`, when cursor is not over selectable cell, but within the table
            .on_mouse_up(MouseButton::Left, {
                let view = cx.entity();
                move |_event, _window, cx| {
                    view.update(cx, |this, cx| {
                        this.selection.end_selection();
                        cx.notify();
                    });
                }
            })
    }
}

impl CsvPreviewView {
    /// Create header for data, which is orderable with text on the left and order button on the right
    fn create_header_element_for_orderables(
        &self,
        header_text: String,
        cx: &mut Context<'_, CsvPreviewView>,
        col_idx: usize,
    ) -> AnyElement {
        // CSV data columns: text + sort button
        h_flex()
            .justify_between()
            .items_center()
            .w_full()
            .map(|div| match self.settings.font_type {
                FontType::Ui => div.font_ui(cx),
                FontType::Monospace => div.font_buffer(cx),
            })
            .child(div().child(header_text))
            .child(self.create_sort_button(cx, col_idx))
            .into_any_element()
    }

    fn create_sort_button(&self, cx: &mut Context<'_, CsvPreviewView>, col_idx: usize) -> Button {
        let sort_btn = Button::new(
            ElementId::NamedInteger("sort-button".into(), col_idx as u64),
            match self.ordering {
                Some(ordering) if ordering.col_idx == col_idx => match ordering.direction {
                    OrderingDirection::Asc => "↑",
                    OrderingDirection::Desc => "↓",
                },
                _ => "↕", // Unsorted/available for sorting
            },
        )
        .size(ButtonSize::Compact)
        .style(if self.ordering.is_some_and(|o| o.col_idx == col_idx) {
            ButtonStyle::Filled
        } else {
            ButtonStyle::Subtle
        })
        .on_click(cx.listener(move |this, _event, _window, cx| {
            let new_ordering = match this.ordering {
                Some(ordering) if ordering.col_idx == col_idx => {
                    // Same column clicked - cycle through states
                    match ordering.direction {
                        OrderingDirection::Asc => Some(Ordering {
                            col_idx,
                            direction: OrderingDirection::Desc,
                        }),
                        OrderingDirection::Desc => None, // Clear sorting
                    }
                }
                _ => {
                    // Different column or no sorting - start with ascending
                    Some(Ordering {
                        col_idx,
                        direction: OrderingDirection::Asc,
                    })
                }
            };

            this.ordering = new_ordering;
            cx.notify();
        }));
        sort_btn
    }

    pub(crate) fn create_table<const COLS: usize>(
        &self,
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        assert!(COLS > 0, "Expected to have at least 1 column");

        let remaining_col_number = COLS - 1;
        let fraction = if remaining_col_number > 0 {
            1. / remaining_col_number as f32
        } else {
            1. // only columns with line numbers is present. Put 100%, but it will be overwritten anyways :D
        };
        let mut widths = [DefiniteLength::Fraction(fraction); COLS];
        let line_number_width = self.calculate_row_identifier_column_width();
        widths[0] = DefiniteLength::Absolute(AbsoluteLength::Pixels(line_number_width.into()));

        let mut resize_behaviors = [TableResizeBehavior::Resizable; COLS];
        resize_behaviors[0] = TableResizeBehavior::None;

        self.create_table_inner(
            self.contents.rows.len(),
            widths,
            resize_behaviors,
            current_widths,
            cx,
        )
    }

    fn create_table_inner<const COLS: usize>(
        &self,
        row_count: usize,
        widths: [DefiniteLength; COLS],
        resize_behaviors: [TableResizeBehavior; COLS],
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Create headers array with interactive elements
        let mut headers = Vec::with_capacity(COLS);

        // First column: row identifier (clickable to toggle between Lines and Rows)
        let row_identifier_text = match self.settings.numbering_type {
            NumberingType::Lines => "Lines",
            NumberingType::Rows => "Rows",
        };

        let view = cx.entity();
        headers.push(
            div()
                .map(|div| match self.settings.font_type {
                    FontType::Ui => div.font_ui(cx),
                    FontType::Monospace => div.font_buffer(cx),
                })
                .child(
                    Button::new(
                        ElementId::Name("row-identifier-toggle".into()),
                        row_identifier_text,
                    )
                    .style(ButtonStyle::Subtle)
                    .size(ButtonSize::Compact)
                    .tooltip(Tooltip::text(
                        "Click to toggle between file line numbers and sequential row numbers",
                    ))
                    .on_click(move |_event, _window, cx| {
                        view.update(cx, |this, cx| {
                            this.settings.numbering_type = match this.settings.numbering_type {
                                NumberingType::Lines => NumberingType::Rows,
                                NumberingType::Rows => NumberingType::Lines,
                            };
                            cx.notify();
                        });
                    }),
                )
                .into_any_element(),
        );

        // Add the actual CSV headers with ordering buttons
        for i in 0..(COLS - 1) {
            let header_text = self
                .contents
                .headers
                .get(i)
                .map(|h| h.as_ref().to_string())
                .unwrap_or_else(|| format!("Col {}", i + 1));

            headers.push(self.create_header_element_for_orderables(header_text, cx, i));
        }

        // Manually construct array to avoid Debug trait requirement
        let headers_array: [AnyElement; COLS] = {
            assert_eq!(headers.len(), COLS, "Headers vector has wrong length");
            let mut iter = headers.into_iter();
            std::array::from_fn(|_| iter.next().unwrap())
        };

        let table = Table::new()
            .interactable(&self.table_interaction_state)
            .striped()
            .column_widths(widths)
            .resizable_columns(resize_behaviors, current_widths, cx)
            .header(headers_array);

        // Choose rendering method based on settings
        match self.settings.rendering_with {
            RowRenderMechanism::VariableList => table
                .variable_list(row_count, {
                    let row_identifier_text_color = cx.theme().colors().editor_line_number;
                    let selected_bg = cx.theme().colors().element_selected;
                    cx.processor(move |this, display_index: usize, _window, cx| {
                        Self::render_table_row_for_variable_list::<COLS>(
                            this,
                            display_index,
                            row_identifier_text_color,
                            selected_bg,
                            cx,
                        )
                    })
                })
                .into_any_element(),
            RowRenderMechanism::UniformList => table
                .uniform_list("csv-table", row_count, {
                    let row_identifier_text_color = cx.theme().colors().editor_line_number;
                    let selected_bg = cx.theme().colors().element_selected;
                    cx.processor(move |this, range: Range<usize>, _window, cx| {
                        Self::render_table_rows_for_uniform_list::<COLS>(
                            this,
                            range,
                            row_identifier_text_color,
                            selected_bg,
                            cx,
                        )
                    })
                })
                .into_any_element(),
        }
    }

    /// Render settings panel above the table
    fn render_settings_panel(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
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

        h_flex()
            .gap_4()
            .p_2()
            .bg(cx.theme().colors().surface_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
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
            .into_any_element()
    }

    /// Render a single row for variable_list (supports variable heights)
    fn render_single_table_row<const COLS: usize>(
        this: &CsvPreviewView,
        display_index: usize,
        row_identifier_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &Context<CsvPreviewView>,
    ) -> Option<[AnyElement; COLS]> {
        let ordered_indices = generate_ordered_indices(this.ordering, &this.contents);

        // Get the actual row index from our ordered indices
        let row_index = *ordered_indices.get(display_index)?;
        let row = this.contents.rows.get(row_index)?;

        let mut elements = Vec::with_capacity(COLS);

        // First column: row identifier (line numbers or sequential row numbers)
        let row_identifier: SharedString = match this.settings.numbering_type {
            NumberingType::Lines => this
                .contents
                .line_numbers
                .get(row_index)?
                .display_string()
                .into(),
            NumberingType::Rows => (display_index + 1).to_string().into(),
        };
        elements.push(
            div()
                .flex()
                .child(row_identifier)
                .text_color(row_identifier_text_color)
                .h_full()
                // Row identifiers are always centered
                .items_center()
                .map(|div| match this.settings.font_type {
                    FontType::Ui => div.font_ui(cx),
                    FontType::Monospace => div.font_buffer(cx),
                })
                .into_any_element(),
        );

        // Remaining columns: actual CSV data
        for col in 0..(COLS - 1) {
            let cell_content: SharedString = row.get(col).cloned().unwrap_or_else(|| "".into());

            // Check if this cell is selected using display coordinates
            let ordered_indices = generate_ordered_indices(this.ordering, &this.contents);
            let display_to_data_converter = |dr: usize| ordered_indices.get(dr).copied();
            let is_selected =
                this.selection
                    .is_cell_selected(display_index, col, display_to_data_converter);

            elements.push(TableSelection::create_selectable_cell(
                display_index,
                col,
                cell_content,
                cx.entity(),
                selected_bg,
                is_selected,
                this.settings.vertical_alignment,
                this.settings.font_type,
                cx,
            ));
        }

        let elements_array: [AnyElement; COLS] = elements.try_into().ok()?;
        Some(elements_array)
    }

    fn render_table_row_for_variable_list<const COLS: usize>(
        this: &CsvPreviewView,
        display_index: usize,
        row_identifier_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &Context<CsvPreviewView>,
    ) -> [AnyElement; COLS] {
        Self::render_single_table_row(
            this,
            display_index,
            row_identifier_text_color,
            selected_bg,
            cx,
        )
        .unwrap_or_else(|| std::array::from_fn(|_| div().into_any_element()))
    }

    /// Render multiple rows for uniform_list (uniform heights only)
    fn render_table_rows_for_uniform_list<const COLS: usize>(
        this: &CsvPreviewView,
        display_indices: Range<usize>,
        row_identifier_text_color: gpui::Hsla,
        selected_bg: gpui::Hsla,
        cx: &Context<CsvPreviewView>,
    ) -> Vec<[AnyElement; COLS]> {
        display_indices
            .filter_map(|display_index| {
                Self::render_single_table_row(
                    this,
                    display_index,
                    row_identifier_text_color,
                    selected_bg,
                    cx,
                )
            })
            .collect()
    }
}
