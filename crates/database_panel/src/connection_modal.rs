use std::sync::Arc;

use fs::Fs;
use gpui::{App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Window};
use settings::{DatabaseAdapter, DatabaseConnectionContent, update_settings_file};
use ui::{Modal, ModalFooter, ModalHeader, Section, Tooltip, prelude::*};
use ui_input::InputField;
use workspace::{ModalView, Workspace};

/// A modal form that appends a new connection to the `database_panel.connections`
/// list in the user settings. Editing or removing connections is done from the
/// panel or directly in the settings file.
pub struct ConnectionModal {
    fs: Arc<dyn Fs>,
    adapter: DatabaseAdapter,
    name: Entity<InputField>,
    path: Entity<InputField>,
    host: Entity<InputField>,
    port: Entity<InputField>,
    username: Entity<InputField>,
    password: Entity<InputField>,
    databases: Entity<InputField>,
    error: Option<SharedString>,
}

impl ConnectionModal {
    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let fs = workspace.project().read(cx).fs().clone();
        workspace.toggle_modal(window, cx, |window, cx| Self::new(fs, window, cx));
    }

    fn new(fs: Arc<dyn Fs>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| {
            InputField::new(window, cx, "Optional display name")
                .label("Name")
                .tab_index(0)
        });
        let path = cx.new(|cx| {
            InputField::new(window, cx, "/path/to/database.sqlite")
                .label("File path")
                .tab_index(1)
        });
        let host = cx.new(|cx| {
            InputField::new(window, cx, "localhost")
                .label("Host")
                .tab_index(1)
        });
        let port = cx.new(|cx| {
            InputField::new(window, cx, "3306")
                .label("Port")
                .tab_index(2)
        });
        let username = cx.new(|cx| {
            InputField::new(window, cx, "root")
                .label("User")
                .tab_index(3)
        });
        let password = cx.new(|cx| {
            InputField::new(window, cx, "Password")
                .label("Password")
                .masked(true)
                .tab_index(4)
        });
        let databases = cx.new(|cx| {
            InputField::new(window, cx, "Comma-separated; empty shows all")
                .label("Databases")
                .tab_index(5)
        });
        Self {
            fs,
            adapter: DatabaseAdapter::Sqlite,
            name,
            path,
            host,
            port,
            username,
            password,
            databases,
            error: None,
        }
    }

    fn set_adapter(&mut self, adapter: DatabaseAdapter, cx: &mut Context<Self>) {
        if self.adapter != adapter {
            self.adapter = adapter;
            self.error = None;
            cx.notify();
        }
    }

    fn field_handles(&self, cx: &App) -> Vec<FocusHandle> {
        let fields: Vec<&Entity<InputField>> = match self.adapter {
            DatabaseAdapter::Sqlite => vec![&self.name, &self.path],
            DatabaseAdapter::Mariadb => vec![
                &self.name,
                &self.host,
                &self.port,
                &self.username,
                &self.password,
                &self.databases,
            ],
        };
        fields
            .into_iter()
            .map(|field| field.focus_handle(cx))
            .collect()
    }

    fn focus_next(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_focus(1, window, cx);
    }

    fn focus_previous(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cycle_focus(-1, window, cx);
    }

    fn cycle_focus(&mut self, direction: isize, window: &mut Window, cx: &mut Context<Self>) {
        let handles = self.field_handles(cx);
        if handles.is_empty() {
            return;
        }
        let current = handles
            .iter()
            .position(|handle| handle.contains_focused(window, cx));
        let next = match current {
            Some(index) => (index as isize + direction).rem_euclid(handles.len() as isize) as usize,
            None => 0,
        };
        if let Some(handle) = handles.get(next) {
            handle.focus(window, cx);
        }
    }

    fn text_or_none(field: &Entity<InputField>, cx: &App) -> Option<String> {
        let text = field.read(cx).text(cx).trim().to_string();
        (!text.is_empty()).then_some(text)
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.save(window, cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn save(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let name = Self::text_or_none(&self.name, cx);
        let connection = match self.adapter {
            DatabaseAdapter::Sqlite => {
                let Some(path) = Self::text_or_none(&self.path, cx) else {
                    self.error = Some("A database file path is required.".into());
                    cx.notify();
                    return;
                };
                DatabaseConnectionContent {
                    name,
                    adapter: Some(DatabaseAdapter::Sqlite),
                    path: Some(path),
                    ..Default::default()
                }
            }
            DatabaseAdapter::Mariadb => {
                let port = match Self::text_or_none(&self.port, cx) {
                    Some(text) => match text.parse::<u16>() {
                        Ok(port) => Some(port),
                        Err(_) => {
                            self.error = Some("The port must be a number.".into());
                            cx.notify();
                            return;
                        }
                    },
                    None => None,
                };
                let databases = Self::text_or_none(&self.databases, cx)
                    .map(|text| {
                        text.split(',')
                            .map(|database| database.trim().to_string())
                            .filter(|database| !database.is_empty())
                            .collect()
                    })
                    .unwrap_or_default();
                DatabaseConnectionContent {
                    name,
                    adapter: Some(DatabaseAdapter::Mariadb),
                    host: Self::text_or_none(&self.host, cx),
                    port,
                    username: Self::text_or_none(&self.username, cx),
                    password: Self::text_or_none(&self.password, cx),
                    databases,
                    ..Default::default()
                }
            }
        };
        update_settings_file(self.fs.clone(), cx, move |content, _| {
            content
                .database_panel
                .get_or_insert_default()
                .connections
                .get_or_insert_default()
                .push(connection);
        });
        cx.emit(DismissEvent);
    }

    fn render_adapter_button(
        &self,
        adapter: DatabaseAdapter,
        label: &'static str,
        cx: &mut Context<Self>,
    ) -> Button {
        Button::new(label, label)
            .toggle_state(self.adapter == adapter)
            .on_click(cx.listener(move |this, _, _, cx| this.set_adapter(adapter, cx)))
    }
}

impl ModalView for ConnectionModal {}

impl EventEmitter<DismissEvent> for ConnectionModal {}

impl Focusable for ConnectionModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.name.focus_handle(cx)
    }
}

impl Render for ConnectionModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_sqlite = self.adapter == DatabaseAdapter::Sqlite;

        v_flex()
            .key_context("DatabaseConnectionModal")
            .w(rems(34.))
            .elevation_3(cx)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::focus_next))
            .on_action(cx.listener(Self::focus_previous))
            .child(
                Modal::new("database-connection-modal", None)
                    .header(ModalHeader::new().headline("Add Database Connection"))
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_2()
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(self.render_adapter_button(
                                            DatabaseAdapter::Sqlite,
                                            "SQLite",
                                            cx,
                                        ))
                                        .child(self.render_adapter_button(
                                            DatabaseAdapter::Mariadb,
                                            "MariaDB",
                                            cx,
                                        )),
                                )
                                .child(self.name.clone())
                                .when(is_sqlite, |this| this.child(self.path.clone()))
                                .when(!is_sqlite, |this| {
                                    this.child(
                                        h_flex()
                                            .gap_2()
                                            .child(div().flex_1().child(self.host.clone()))
                                            .child(div().w_24().child(self.port.clone())),
                                    )
                                    .child(self.username.clone())
                                    .child(self.password.clone())
                                    .child(self.databases.clone())
                                })
                                .when_some(self.error.clone(), |this, error| {
                                    this.child(
                                        Label::new(error)
                                            .size(LabelSize::Small)
                                            .color(Color::Error),
                                    )
                                }),
                        ),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_1()
                                .child(Button::new("cancel", "Cancel").on_click(cx.listener(
                                    |_, _, _, cx| {
                                        cx.emit(DismissEvent);
                                    },
                                )))
                                .child(
                                    Button::new("save", "Add")
                                        .tooltip(Tooltip::text(
                                            "Saved to the database_panel settings",
                                        ))
                                        .on_click(
                                            cx.listener(|this, _, window, cx| {
                                                this.save(window, cx)
                                            }),
                                        ),
                                ),
                        ),
                    ),
            )
    }
}
