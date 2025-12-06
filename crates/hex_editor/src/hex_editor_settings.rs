use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{HexEditorOffsetFormat, Settings, SettingsContent};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct HexEditorSettings {
    /// Number of bytes to display per row. Default is 16.
    pub bytes_per_row: usize,

    /// Whether to show the data inspector panel by default.
    pub show_data_inspector: bool,

    /// Whether to show the ASCII column.
    pub show_ascii: bool,

    /// Whether to show the offset column.
    pub show_offset: bool,

    /// Offset display format: "hex" or "decimal".
    pub offset_format: OffsetFormat,

    /// Whether to group bytes (adds extra spacing every 8 bytes).
    pub group_bytes: bool,

    /// Whether to highlight non-printable characters in ASCII view.
    pub highlight_non_printable: bool,

    /// Whether to highlight modified bytes.
    pub highlight_modified: bool,

    /// Maximum file size (in bytes) to open without warning.
    /// Files larger than this will show a confirmation dialog.
    pub max_file_size_without_warning: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OffsetFormat {
    #[default]
    Hex,
    Decimal,
}

impl From<HexEditorOffsetFormat> for OffsetFormat {
    fn from(format: HexEditorOffsetFormat) -> Self {
        match format {
            HexEditorOffsetFormat::Hex => OffsetFormat::Hex,
            HexEditorOffsetFormat::Decimal => OffsetFormat::Decimal,
        }
    }
}

impl Default for HexEditorSettings {
    fn default() -> Self {
        Self {
            bytes_per_row: 16,
            show_data_inspector: true,
            show_ascii: true,
            show_offset: true,
            offset_format: OffsetFormat::Hex,
            group_bytes: true,
            highlight_non_printable: true,
            highlight_modified: true,
            max_file_size_without_warning: 100 * 1024 * 1024, // 100 MB
        }
    }
}

impl Settings for HexEditorSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let defaults = HexEditorSettings::default();

        if let Some(hex_editor) = &content.hex_editor {
            Self {
                bytes_per_row: hex_editor.bytes_per_row.unwrap_or(defaults.bytes_per_row),
                show_data_inspector: hex_editor
                    .show_data_inspector
                    .unwrap_or(defaults.show_data_inspector),
                show_ascii: hex_editor.show_ascii.unwrap_or(defaults.show_ascii),
                show_offset: hex_editor.show_offset.unwrap_or(defaults.show_offset),
                offset_format: hex_editor
                    .offset_format
                    .map(Into::into)
                    .unwrap_or(defaults.offset_format),
                group_bytes: hex_editor.group_bytes.unwrap_or(defaults.group_bytes),
                highlight_non_printable: hex_editor
                    .highlight_non_printable
                    .unwrap_or(defaults.highlight_non_printable),
                highlight_modified: hex_editor
                    .highlight_modified
                    .unwrap_or(defaults.highlight_modified),
                max_file_size_without_warning: hex_editor
                    .max_file_size_without_warning
                    .unwrap_or(defaults.max_file_size_without_warning),
            }
        } else {
            defaults
        }
    }
}
