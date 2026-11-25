use editor::{EditorSettings, ui_scrollbar_settings_from_raw};
use gpui::{App, Pixels};
use language::SymbolKind;
pub use settings::{DockSide, FolderIndicator, OutlineSymbolKind, Settings, ShowIndentGuides};
use settings::{IntoGpui, RegisterSetting};
use ui::scrollbars::{ScrollbarVisibility, ShowScrollbar};

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct OutlinePanelSettings {
    pub button: bool,
    pub default_width: Pixels,
    pub dock: DockSide,
    pub file_icons: bool,
    pub folder_indicator: FolderIndicator,
    pub git_status: bool,
    pub indent_size: f32,
    pub indent_guides: IndentGuidesSettings,
    pub auto_reveal_entries: bool,
    pub auto_fold_dirs: bool,
    pub scrollbar: ScrollbarSettings,
    pub expand_outlines_with_depth: usize,
    pub multi_buffer_hide_symbols: bool,
}

pub(crate) trait OutlineSymbolKindExt {
    fn to_symbol_kind(self) -> SymbolKind;
}

impl OutlineSymbolKindExt for OutlineSymbolKind {
    fn to_symbol_kind(self) -> SymbolKind {
        match self {
            Self::File => SymbolKind::File,
            Self::Module => SymbolKind::Module,
            Self::Namespace => SymbolKind::Namespace,
            Self::Package => SymbolKind::Package,
            Self::Class => SymbolKind::Class,
            Self::Method => SymbolKind::Method,
            Self::Property => SymbolKind::Property,
            Self::Field => SymbolKind::Field,
            Self::Constructor => SymbolKind::Constructor,
            Self::Enum => SymbolKind::Enum,
            Self::Interface => SymbolKind::Interface,
            Self::Function => SymbolKind::Function,
            Self::Variable => SymbolKind::Variable,
            Self::Constant => SymbolKind::Constant,
            Self::String => SymbolKind::String,
            Self::Number => SymbolKind::Number,
            Self::Boolean => SymbolKind::Boolean,
            Self::Array => SymbolKind::Array,
            Self::Object => SymbolKind::Object,
            Self::Key => SymbolKind::Key,
            Self::Null => SymbolKind::Null,
            Self::EnumMember => SymbolKind::EnumMember,
            Self::Struct => SymbolKind::Struct,
            Self::Event => SymbolKind::Event,
            Self::Operator => SymbolKind::Operator,
            Self::TypeParameter => SymbolKind::TypeParameter,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IndentGuidesSettings {
    pub show: ShowIndentGuides,
}

#[derive(Default)]
pub(crate) struct OutlinePanelSettingsScrollbarProxy;

impl ScrollbarVisibility for OutlinePanelSettingsScrollbarProxy {
    fn visibility(&self, cx: &App) -> ShowScrollbar {
        OutlinePanelSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }
}

impl Settings for OutlinePanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let panel = content.outline_panel.as_ref().unwrap();
        Self {
            button: panel.button.unwrap(),
            default_width: panel.default_width.unwrap().into_gpui(),
            dock: panel.dock.unwrap(),
            file_icons: panel.file_icons.unwrap(),
            folder_indicator: panel.folder_indicator.unwrap(),
            git_status: panel.git_status.unwrap()
                && content
                    .git
                    .as_ref()
                    .unwrap()
                    .enabled
                    .unwrap()
                    .is_git_status_enabled(),
            indent_size: *panel.indent_size.unwrap(),
            indent_guides: IndentGuidesSettings {
                show: panel.indent_guides.unwrap().show.unwrap(),
            },
            auto_reveal_entries: panel.auto_reveal_entries.unwrap(),
            auto_fold_dirs: panel.auto_fold_dirs.unwrap(),
            scrollbar: ScrollbarSettings {
                show: panel
                    .scrollbar
                    .unwrap()
                    .show
                    .map(ui_scrollbar_settings_from_raw),
            },
            expand_outlines_with_depth: panel.expand_outlines_with_depth.unwrap(),
            multi_buffer_hide_symbols: panel.multi_buffer_hide_symbols.unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::VariantArray;

    #[test]
    fn outline_symbol_kind_names_match_language_symbol_kinds() {
        for kind in OutlineSymbolKind::VARIANTS {
            let language_kind = kind.to_symbol_kind();
            assert_eq!(
                kind.as_ref(),
                language_kind.as_ref(),
                "settings and language symbol kind names should match for {kind:?}"
            );
            assert_eq!(
                kind.as_ref().parse::<SymbolKind>().unwrap(),
                language_kind,
                "outline grammar capture names should parse to the same symbol kind"
            );
        }
    }
}
