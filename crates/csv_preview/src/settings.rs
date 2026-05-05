#[derive(Default, Clone, Copy)]
pub enum RowRenderMechanism {
    /// More correct for multiline content, but slower.
    #[allow(dead_code)] // Will be used when settings ui is added
    VariableList,
    /// Default behaviour for now while resizable columns are being stabilized.
    #[default]
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

#[derive(Clone, Default)]
pub(crate) struct CsvPreviewSettings {
    pub(crate) rendering_with: RowRenderMechanism,
    pub(crate) vertical_alignment: VerticalAlignment,
    pub(crate) font_type: FontType,
    pub(crate) numbering_type: RowIdentifiers,
    pub(crate) show_debug_info: bool,
    pub(crate) multiline_cells_enabled: bool,
}
