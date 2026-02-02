#[derive(Default, Clone, Copy)]
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

// TODO: Finish implementation of column driven width first
// #[derive(Default, Clone, Copy, PartialEq)]
// pub(crate) enum TableWidthMode {
//     /// Table width adjusts to container (fractional column resizing)
//     #[default]
//     Responsive,
//     /// Table width grows with columns (absolute column resizing)
//     ColumnDriven,
// }

// impl From<TableWidthMode> for TableWidth {
//     fn from(mode: TableWidthMode) -> Self {
//         match mode {
//             TableWidthMode::Responsive => TableWidth::Unset,
//             TableWidthMode::ColumnDriven => TableWidth::ColumnDriven,
//         }
//     }
// }

#[derive(Clone, Default)]
pub(crate) struct CsvPreviewSettings {
    pub(crate) rendering_with: RowRenderMechanism,
    pub(crate) vertical_alignment: VerticalAlignment,
    pub(crate) font_type: FontType,
    pub(crate) numbering_type: RowIdentifiers,
    // pub(crate) table_width_mode: TableWidthMode,
    pub(crate) show_debug_info: bool,
    pub(crate) show_perf_metrics_overlay: bool,
    pub(crate) multiline_cells_enabled: bool,
}
