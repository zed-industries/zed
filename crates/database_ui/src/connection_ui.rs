use gpui::{prelude::*, Entity};
use ui_input::InputField;

use database_core::ConnectionConfig;

pub enum AddConnectionFlow {
    PostgreSql,
    MySql,
}

pub struct ConnectionForm {
    pub pg_connection_string_field: Entity<InputField>,
    pub mysql_connection_string_field: Entity<InputField>,
    pub error: Option<String>,
    pub active_flow: Option<AddConnectionFlow>,
}

impl ConnectionForm {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::App) -> Self {
        let pg_connection_string_field = cx.new(|cx| {
            InputField::new(window, cx, "postgres://user:pass@host:5432/db")
                .tab_index(1)
        });

        let mysql_connection_string_field = cx.new(|cx| {
            InputField::new(window, cx, "mysql://user:pass@host:3306/db")
                .tab_index(1)
        });

        Self {
            pg_connection_string_field,
            mysql_connection_string_field,
            error: None,
            active_flow: None,
        }
    }

    pub fn build_postgres_config(&self, cx: &gpui::App) -> Result<ConnectionConfig, String> {
        let text = self.pg_connection_string_field.read(cx).text(cx);
        ConnectionConfig::from_postgres_url(&text)
    }

    pub fn build_mysql_config(&self, cx: &gpui::App) -> Result<ConnectionConfig, String> {
        let text = self.mysql_connection_string_field.read(cx).text(cx);
        ConnectionConfig::from_mysql_url(&text)
    }

    pub fn show_postgres_form(&mut self) {
        self.active_flow = Some(AddConnectionFlow::PostgreSql);
        self.error = None;
    }

    pub fn show_mysql_form(&mut self) {
        self.active_flow = Some(AddConnectionFlow::MySql);
        self.error = None;
    }

    pub fn dismiss(&mut self) {
        self.active_flow = None;
        self.error = None;
    }

    pub fn clear(&self, window: &mut gpui::Window, cx: &mut gpui::App) {
        self.pg_connection_string_field.update(cx, |field, cx| {
            field.clear(window, cx);
        });
        self.mysql_connection_string_field.update(cx, |field, cx| {
            field.clear(window, cx);
        });
    }
}
