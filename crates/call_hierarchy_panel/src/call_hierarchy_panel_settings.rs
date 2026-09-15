use editor::{EditorSettings, ui_scrollbar_settings_from_raw};
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

#[derive(Default)]
pub(crate) struct CallHierarchyPanelSettingsScrollbarProxy;

impl ScrollbarVisibility for CallHierarchyPanelSettingsScrollbarProxy {
    fn visibility(&self, cx: &App) -> ShowScrollbar {
        CallHierarchyPanelSettings::get_global(cx)
            .scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }
}

impl Settings for CallHierarchyPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let panel = content.call_hierarchy_panel.as_ref();
        Self {
            button: panel.and_then(|settings| settings.button).unwrap_or(true),
            default_width: panel
                .and_then(|settings| settings.default_width)
                .map(|width| gpui::px(width.0))
                .unwrap_or_else(|| gpui::px(320.)),
            dock: panel
                .and_then(|settings| settings.dock)
                .unwrap_or(DockSide::Left),
            indent_size: panel
                .and_then(|settings| settings.indent_size)
                .map(|size| gpui::px(size.0))
                .unwrap_or_else(|| gpui::px(20.)),
            indent_guides: IndentGuidesSettings {
                show: panel
                    .and_then(|settings| settings.indent_guides.as_ref())
                    .and_then(|g| g.show)
                    .unwrap_or(ShowIndentGuides::Always),
            },
            scrollbar: ScrollbarSettings {
                show: panel
                    .and_then(|settings| settings.scrollbar.as_ref())
                    .and_then(|s| s.show)
                    .map(ui_scrollbar_settings_from_raw),
            },
        }
    }
}
