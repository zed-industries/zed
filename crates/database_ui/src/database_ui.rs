mod connection_dialog;
mod connection_manager;
mod connection_ui;
mod database_panel;
mod database_panel_settings;
mod export_dialog;
mod import_dialog;
mod query_editor;
mod result_grid;
mod results_table;
mod schema_tree;
mod value_editor;

pub use database_panel::DatabasePanel;

use gpui::App;

pub fn init(cx: &mut App) {
    database_panel_settings::init(cx);
    database_panel::init(cx);
    query_editor::init(cx);
}
