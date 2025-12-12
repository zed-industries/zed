use ui::{
    ActiveTheme as _, AnyElement, ButtonSize, Context, ContextMenu, DropdownMenu, ElementId,
    IntoElement as _, ParentElement as _, Styled as _, Tooltip, Window, div, h_flex,
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

#[derive(Default)]
pub(crate) struct CsvPreviewSettings {
    pub(crate) rendering_with: RowRenderMechanism,
    pub(crate) vertical_alignment: VerticalAlignment,
    pub(crate) font_type: FontType,
    pub(crate) numbering_type: RowIdentifiers,
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
}
