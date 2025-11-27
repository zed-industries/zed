use editor::EditorSettings;
use gpui::{App, Pixels};
use settings::RegisterSetting;
pub use settings::{DockSide, Settings, ShowIndentGuides};
use ui::scrollbars::{ScrollbarVisibility, ShowScrollbar};

#[derive(Debug, Clone, Copy, PartialEq, RegisterSetting)]
pub struct CallHierarchyPanelSettings {
    pub button: bool,
    pub default_width: Pixels,
    pub dock: DockSide,
    pub indent_size: Pixels,
    pub indent_guides: IndentGuidesSettings,
    pub scrollbar: ScrollbarSettings,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScrollbarSettings {
    pub show: Option<ShowScrollbar>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IndentGuidesSettings {
    pub show: ShowIndentGuides,
}

impl ScrollbarVisibility for CallHierarchyPanelSettings {
    fn visibility(&self, cx: &App) -> ShowScrollbar {
        self.scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }
}

impl Settings for CallHierarchyPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let panel = content.call_hierarchy_panel.as_ref().unwrap();
        Self {
            button: panel.button.unwrap(),
            default_width: panel.default_width.map(gpui::px).unwrap(),
            dock: panel.dock.unwrap(),
            indent_size: panel.indent_size.map(gpui::px).unwrap(),
            indent_guides: IndentGuidesSettings {
                show: panel
                    .indent_guides
                    .as_ref()
                    .and_then(|g| g.show)
                    .unwrap_or(ShowIndentGuides::Always),
            },
            scrollbar: ScrollbarSettings {
                show: panel
                    .scrollbar
                    .as_ref()
                    .and_then(|s| s.show)
                    .map(Into::into),
            },
        }
    }
}
