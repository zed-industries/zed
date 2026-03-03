use gpui::{App, Pixels};
use serde::{Deserialize, Serialize};
use settings::{RegisterSetting, Settings, settings_content};
use ui::px;
use workspace::dock::DockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SortMode {
    #[default]
    Server,
    Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFormatSettings {
    pub decimal_separator: char,
    pub grouping_separator: Option<char>,
}

impl Default for NumberFormatSettings {
    fn default() -> Self {
        Self {
            decimal_separator: '.',
            grouping_separator: None,
        }
    }
}

#[derive(Debug, RegisterSetting)]
pub struct DatabasePanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
    pub max_cell_display_chars: usize,
    #[allow(dead_code)]
    pub max_query_history: usize,
    pub default_column_width: f32,
    pub row_height: f32,
    pub header_height: f32,
    #[allow(dead_code)]
    pub default_rows_per_page: usize,
    #[allow(dead_code)]
    pub auto_commit: bool,
    #[allow(dead_code)]
    pub show_null_indicator: bool,
    #[allow(dead_code)]
    pub grid_font_family: Option<String>,
    #[allow(dead_code)]
    pub grid_font_size: Option<f32>,
    #[allow(dead_code)]
    pub default_export_format: String,
    #[allow(dead_code)]
    pub sort_mode: SortMode,
    #[allow(dead_code)]
    pub number_format: NumberFormatSettings,
}

impl Settings for DatabasePanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        match content.database_panel.as_ref() {
            Some(panel) => Self {
                button: panel.button.unwrap_or(true),
                dock: panel
                    .dock
                    .map(Into::into)
                    .unwrap_or(DockPosition::Right),
                default_width: panel.default_width.map(px).unwrap_or(px(360.0)),
                max_cell_display_chars: panel.max_cell_display_chars.unwrap_or(500),
                max_query_history: panel.max_query_history.unwrap_or(100),
                default_column_width: panel.default_column_width.unwrap_or(200.0),
                row_height: panel.row_height.unwrap_or(26.0),
                header_height: panel.header_height.unwrap_or(28.0),
                default_rows_per_page: panel.default_rows_per_page.unwrap_or(50),
                auto_commit: panel.auto_commit.unwrap_or(false),
                show_null_indicator: panel.show_null_indicator.unwrap_or(true),
                grid_font_family: panel.grid_font_family.clone(),
                grid_font_size: panel.grid_font_size,
                default_export_format: panel
                    .default_export_format
                    .clone()
                    .unwrap_or_else(|| "csv".to_string()),
                sort_mode: match panel.sort_mode {
                    Some(settings_content::DatabasePanelSortMode::Client) => SortMode::Client,
                    _ => SortMode::Server,
                },
                number_format: panel
                    .number_format
                    .as_ref()
                    .map(|nf| NumberFormatSettings {
                        decimal_separator: nf
                            .decimal_separator
                            .as_ref()
                            .and_then(|s| s.chars().next())
                            .unwrap_or('.'),
                        grouping_separator: nf
                            .grouping_separator
                            .as_ref()
                            .and_then(|s| s.chars().next()),
                    })
                    .unwrap_or_default(),
            },
            None => Self::default(),
        }
    }
}

impl Default for DatabasePanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            dock: DockPosition::Right,
            default_width: px(360.0),
            max_cell_display_chars: 500,
            max_query_history: 100,
            default_column_width: 200.0,
            row_height: 26.0,
            header_height: 28.0,
            default_rows_per_page: 50,
            auto_commit: false,
            show_null_indicator: true,
            grid_font_family: None,
            grid_font_size: None,
            default_export_format: "csv".to_string(),
            sort_mode: SortMode::default(),
            number_format: NumberFormatSettings::default(),
        }
    }
}

pub fn init(cx: &mut App) {
    DatabasePanelSettings::register(cx);
}
