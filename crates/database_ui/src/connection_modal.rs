use std::sync::Arc;

use database_client::ConnectionConfig;
use fs::Fs;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ParentElement,
    Render, Styled, Window, rems,
};
use settings::update_settings_file;
use ui::{Headline, HeadlineSize, prelude::*};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::ModalView;

use crate::connection_store::{ClientFactory, credentials_url};

/// Identifies which form field a validation error belongs to, so the modal can
/// attach the message to the right [`InputField`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormField {
    Name,
    Host,
    Port,
    Database,
    User,
}

/// A validation failure carrying the offending field and a human-readable message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldError {
    pub field: FormField,
    pub message: String,
}

/// Raw text values read from the form fields, before validation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FormValues {
    pub name: String,
    pub host: String,
    pub port: String,
    pub database: String,
    pub user: String,
}

/// The default PostgreSQL port, used when the port field is left blank.
const DEFAULT_PORT: u16 = 5432;

/// Validates and normalizes the raw form values into a [`ConnectionConfig`].
///
/// `existing_names` is the set of connection names the new config must not
/// collide with. When editing, the caller excludes the connection's own
/// original name from this list so keeping the same name is allowed.
///
/// An empty port defaults to [`DEFAULT_PORT`]; a non-numeric or out-of-range
/// port is a [`FormField::Port`] error.
fn validate(
    values: &FormValues,
    existing_names: &[String],
) -> Result<ConnectionConfig, FieldError> {
    let name = values.name.trim();
    if name.is_empty() {
        return Err(FieldError {
            field: FormField::Name,
            message: "Name is required".into(),
        });
    }
    if existing_names.iter().any(|existing| existing == name) {
        return Err(FieldError {
            field: FormField::Name,
            message: "A connection with this name already exists".into(),
        });
    }

    let host = values.host.trim();
    if host.is_empty() {
        return Err(FieldError {
            field: FormField::Host,
            message: "Host is required".into(),
        });
    }

    let port_text = values.port.trim();
    let port = if port_text.is_empty() {
        DEFAULT_PORT
    } else {
        match port_text.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                return Err(FieldError {
                    field: FormField::Port,
                    message: "Port must be a number between 1 and 65535".into(),
                });
            }
        }
    };

    let database = values.database.trim();
    if database.is_empty() {
        return Err(FieldError {
            field: FormField::Database,
            message: "Database is required".into(),
        });
    }

    let user = values.user.trim();
    if user.is_empty() {
        return Err(FieldError {
            field: FormField::User,
            message: "User is required".into(),
        });
    }

    Ok(ConnectionConfig {
        name: name.to_string(),
        host: host.to_string(),
        port,
        database: database.to_string(),
        user: user.to_string(),
    })
}

/// The result of a Test Connection attempt, rendered as a status label.
enum TestStatus {
    Idle,
    Testing,
    Ok,
    Failed(String),
}

/// Modal dialog for creating or editing a database connection.
///
/// On save it writes the connection into settings and stores the password in
/// the system keychain. Test Connection builds a throwaway client via the
/// injected [`ClientFactory`] and calls `test_connection` without persisting
/// anything.
pub struct ConnectionModal {
    name_field: Entity<InputField>,
    host_field: Entity<InputField>,
    port_field: Entity<InputField>,
    database_field: Entity<InputField>,
    user_field: Entity<InputField>,
    password_field: Entity<InputField>,
    /// The connection being edited, if any. Used to exclude its own name from
    /// the duplicate check and to rewrite it in-place on save.
    existing: Option<ConnectionConfig>,
    /// Names of already-configured connections, for the duplicate-name check.
    existing_names: Vec<String>,
    client_factory: ClientFactory,
    fs: Arc<dyn Fs>,
    test_status: TestStatus,
}

impl ConnectionModal {
    pub fn new(
        existing: Option<ConnectionConfig>,
        existing_names: Vec<String>,
        client_factory: ClientFactory,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "Connection name")
                .label("Name")
                .tab_index(1);
            if let Some(existing) = existing.as_ref() {
                field.set_text(&existing.name, window, cx);
                // The name is the connection's identity in settings and the
                // keychain, so it cannot change while editing.
                field.editor().set_read_only(true, cx);
            }
            field
        });
        let host_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "localhost")
                .label("Host")
                .tab_index(2);
            if let Some(existing) = existing.as_ref() {
                field.set_text(&existing.host, window, cx);
            }
            field
        });
        let port_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "5432")
                .label("Port")
                .tab_index(3);
            if let Some(existing) = existing.as_ref() {
                field.set_text(&existing.port.to_string(), window, cx);
            }
            field
        });
        let database_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "postgres")
                .label("Database")
                .tab_index(4);
            if let Some(existing) = existing.as_ref() {
                field.set_text(&existing.database, window, cx);
            }
            field
        });
        let user_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "postgres")
                .label("User")
                .tab_index(5);
            if let Some(existing) = existing.as_ref() {
                field.set_text(&existing.user, window, cx);
            }
            field
        });
        let password_field = cx.new(|cx| {
            InputField::new(window, cx, "Password")
                .label("Password")
                .masked(true)
                .tab_index(6)
        });

        Self {
            name_field,
            host_field,
            port_field,
            database_field,
            user_field,
            password_field,
            existing,
            existing_names,
            client_factory,
            fs,
            test_status: TestStatus::Idle,
        }
    }

    /// Reads the raw text from every field into a [`FormValues`].
    fn form_values(&self, cx: &App) -> FormValues {
        FormValues {
            name: self.name_field.read(cx).text(cx),
            host: self.host_field.read(cx).text(cx),
            port: self.port_field.read(cx).text(cx),
            database: self.database_field.read(cx).text(cx),
            user: self.user_field.read(cx).text(cx),
        }
    }

    /// The connection names the new/edited config must not collide with. When
    /// editing, the connection's own original name is excluded.
    fn names_to_check(&self) -> Vec<String> {
        match &self.existing {
            Some(existing) => self
                .existing_names
                .iter()
                .filter(|name| *name != &existing.name)
                .cloned()
                .collect(),
            None => self.existing_names.clone(),
        }
    }

    fn clear_field_errors(&mut self, cx: &mut Context<Self>) {
        for field in [
            &self.name_field,
            &self.host_field,
            &self.port_field,
            &self.database_field,
            &self.user_field,
        ] {
            field.update(cx, |field, cx| field.set_error(None::<SharedString>, cx));
        }
    }

    fn set_field_error(&mut self, error: &FieldError, cx: &mut Context<Self>) {
        let field = match error.field {
            FormField::Name => &self.name_field,
            FormField::Host => &self.host_field,
            FormField::Port => &self.port_field,
            FormField::Database => &self.database_field,
            FormField::User => &self.user_field,
        };
        let message = error.message.clone();
        field.update(cx, |field, cx| field.set_error(Some(message), cx));
    }

    fn on_tab(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn on_tab_prev(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        self.save(cx);
    }

    fn test_connection(&mut self, cx: &mut Context<Self>) {
        if matches!(self.test_status, TestStatus::Testing) {
            return;
        }

        self.clear_field_errors(cx);
        let values = self.form_values(cx);
        let config = match validate(&values, &self.names_to_check()) {
            Ok(config) => config,
            Err(error) => {
                self.set_field_error(&error, cx);
                return;
            }
        };

        let password = self.password_field.read(cx).text(cx);
        // The factory builds the client with the configured statement timeout
        // (see `default_client_factory`), so the probe is bounded there.
        let client = (self.client_factory)(&config, &password);

        self.test_status = TestStatus::Testing;
        cx.notify();

        let task =
            gpui_tokio::Tokio::spawn_result(cx, async move { client.test_connection().await });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                this.test_status = match result {
                    Ok(()) => TestStatus::Ok,
                    Err(error) => TestStatus::Failed(error.to_string()),
                };
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        self.clear_field_errors(cx);
        let values = self.form_values(cx);
        let config = match validate(&values, &self.names_to_check()) {
            Ok(config) => config,
            Err(error) => {
                self.set_field_error(&error, cx);
                return;
            }
        };

        let password = self.password_field.read(cx).text(cx);
        // On edit, the name field is read-only, so `config.name` always equals
        // the original name and we simply replace that entry.
        let original_name = self.existing.as_ref().map(|existing| existing.name.clone());

        let content = settings::DatabaseConnectionContent {
            name: config.name.clone(),
            host: config.host.clone(),
            port: config.port,
            database: config.database.clone(),
            user: config.user.clone(),
        };
        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            let connections = settings
                .database
                .get_or_insert_default()
                .connections
                .get_or_insert_default();
            if let Some(slot) = connections
                .iter_mut()
                .find(|entry| Some(&entry.name) == original_name.as_ref())
            {
                *slot = content;
            } else {
                connections.push(content);
            }
        });

        // Password persistence:
        // - Editing with an empty field leaves the stored password untouched.
        // - Creating with an empty field writes an empty password, so that a
        //   trust-auth server (which needs no password) still connects rather
        //   than failing on `Ok(None)` from the keychain.
        let is_edit = self.existing.is_some();
        if !(is_edit && password.is_empty()) {
            let url = credentials_url(&config.name);
            let user = config.user;
            let provider = zed_credentials_provider::global(cx);
            cx.spawn(async move |_, cx| {
                provider
                    .write_credentials(&url, &user, password.as_bytes(), cx)
                    .await
                    .log_err();
            })
            .detach();
        }

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ConnectionModal {}
impl ModalView for ConnectionModal {}

impl Focusable for ConnectionModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        // The name field is the first tab stop; focusing it opens the modal on
        // the top field.
        self.name_field.focus_handle(cx)
    }
}

impl Render for ConnectionModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_edit = self.existing.is_some();
        let title = if is_edit {
            "Edit Connection"
        } else {
            "Add Connection"
        };
        let testing = matches!(self.test_status, TestStatus::Testing);

        let status_label = match &self.test_status {
            TestStatus::Idle => None,
            TestStatus::Testing => Some(
                Label::new("Testing…")
                    .color(Color::Muted)
                    .into_any_element(),
            ),
            TestStatus::Ok => Some(
                Label::new("Connection OK")
                    .color(Color::Success)
                    .into_any_element(),
            ),
            TestStatus::Failed(message) => Some(
                Label::new(message.clone())
                    .color(Color::Error)
                    .into_any_element(),
            ),
        };

        v_flex()
            .key_context("ConnectionModal")
            .elevation_3(cx)
            .w(rems(34.))
            .tab_group()
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .child(
                h_flex()
                    .p_3()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(Headline::new(title).size(HeadlineSize::XSmall)),
            )
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    .child(self.name_field.clone())
                    .child(self.host_field.clone())
                    .child(self.port_field.clone())
                    .child(self.database_field.clone())
                    .child(self.user_field.clone())
                    .child(self.password_field.clone())
                    .when_some(status_label, |this, label| this.child(label)),
            )
            .child(
                h_flex()
                    .p_3()
                    .gap_2()
                    .justify_end()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Button::new("test-connection", "Test Connection")
                            .disabled(testing)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.test_connection(cx);
                            })),
                    )
                    .child(Button::new("cancel", "Cancel").on_click(cx.listener(
                        |_, _, _window, cx| {
                            cx.emit(DismissEvent);
                        },
                    )))
                    .child(
                        Button::new("save", "Save")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.save(cx);
                            })),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(name: &str, host: &str, port: &str, database: &str, user: &str) -> FormValues {
        FormValues {
            name: name.into(),
            host: host.into(),
            port: port.into(),
            database: database.into(),
            user: user.into(),
        }
    }

    #[test]
    fn valid_values_produce_config() {
        let config = validate(
            &values("local", "127.0.0.1", "5432", "postgres", "postgres"),
            &[],
        )
        .expect("valid values should produce a config");
        assert_eq!(
            config,
            ConnectionConfig {
                name: "local".into(),
                host: "127.0.0.1".into(),
                port: 5432,
                database: "postgres".into(),
                user: "postgres".into(),
            }
        );
    }

    #[test]
    fn empty_name_is_rejected() {
        let error = validate(&values("  ", "host", "5432", "db", "user"), &[])
            .expect_err("empty name should be rejected");
        assert_eq!(error.field, FormField::Name);
    }

    #[test]
    fn empty_host_is_rejected() {
        let error = validate(&values("local", "", "5432", "db", "user"), &[])
            .expect_err("empty host should be rejected");
        assert_eq!(error.field, FormField::Host);
    }

    #[test]
    fn empty_database_is_rejected() {
        let error = validate(&values("local", "host", "5432", "", "user"), &[])
            .expect_err("empty database should be rejected");
        assert_eq!(error.field, FormField::Database);
    }

    #[test]
    fn empty_user_is_rejected() {
        let error = validate(&values("local", "host", "5432", "db", ""), &[])
            .expect_err("empty user should be rejected");
        assert_eq!(error.field, FormField::User);
    }

    #[test]
    fn non_numeric_port_is_rejected() {
        let error = validate(&values("local", "host", "abc", "db", "user"), &[])
            .expect_err("non-numeric port should be rejected");
        assert_eq!(error.field, FormField::Port);
    }

    #[test]
    fn out_of_range_port_is_rejected() {
        let error = validate(&values("local", "host", "70000", "db", "user"), &[])
            .expect_err("out-of-range port should be rejected");
        assert_eq!(error.field, FormField::Port);
    }

    #[test]
    fn empty_port_defaults_to_5432() {
        let config = validate(&values("local", "host", "  ", "db", "user"), &[])
            .expect("empty port should default");
        assert_eq!(config.port, 5432);
    }

    #[test]
    fn duplicate_name_is_rejected_on_create() {
        let error = validate(
            &values("local", "host", "5432", "db", "user"),
            &["local".to_string()],
        )
        .expect_err("duplicate name should be rejected");
        assert_eq!(error.field, FormField::Name);
    }

    #[test]
    fn same_name_allowed_when_excluded_from_existing() {
        // Emulates editing: the connection's own name is excluded from the list.
        let config = validate(&values("local", "host", "5432", "db", "user"), &[])
            .expect("same name is fine when it is not in the existing list");
        assert_eq!(config.name, "local");
    }

    #[test]
    fn values_are_trimmed() {
        let config = validate(
            &values("  local  ", "  host  ", " 5433 ", " db ", " user "),
            &[],
        )
        .expect("surrounding whitespace should be trimmed");
        assert_eq!(config.name, "local");
        assert_eq!(config.host, "host");
        assert_eq!(config.port, 5433);
        assert_eq!(config.database, "db");
        assert_eq!(config.user, "user");
    }
}
