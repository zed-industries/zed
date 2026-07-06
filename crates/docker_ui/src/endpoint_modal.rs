use std::sync::Arc;

use docker_client::{DockerEndpoint, EndpointKind};
use fs::Fs;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ParentElement,
    Render, Styled, Window, rems,
};
use settings::{DockerConnectionContent, DockerEndpointKindContent, update_settings_file};
use ui::{Checkbox, Headline, HeadlineSize, ToggleState, prelude::*};
use ui_input::InputField;
use workspace::ModalView;

use crate::endpoint_store::ClientFactory;

/// Which of the two mutually-exclusive endpoint kinds the form currently has
/// selected. Mirrors [`DockerEndpointKindContent`] but lives in the UI layer
/// so the modal doesn't need to round-trip through settings just to flip the
/// toggle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormKind {
    Local,
    Ssh,
}

/// Identifies which form field a validation error belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormField {
    Name,
    SshHost,
}

/// A validation failure carrying the offending field and a human-readable message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldError {
    pub field: FormField,
    pub message: String,
}

/// Raw values read from the form fields, before validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormValues {
    pub name: String,
    pub kind: FormKind,
    pub ssh_host: String,
    pub read_only: bool,
}

/// Validates the raw form values and builds the [`DockerConnectionContent`]
/// that will be written to settings.
///
/// `existing_names` is the set of endpoint names the new/edited entry must
/// not collide with. When editing, the caller excludes the endpoint's own
/// original name from this list so keeping the same name is allowed.
fn build_content(
    values: &FormValues,
    existing_names: &[String],
) -> Result<DockerConnectionContent, FieldError> {
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
            message: "An endpoint with this name already exists".into(),
        });
    }

    let ssh_host = values.ssh_host.trim();
    if values.kind == FormKind::Ssh && ssh_host.is_empty() {
        return Err(FieldError {
            field: FormField::SshHost,
            message: "SSH host is required".into(),
        });
    }

    let kind = match values.kind {
        FormKind::Local => DockerEndpointKindContent::Local,
        FormKind::Ssh => DockerEndpointKindContent::Ssh,
    };

    Ok(DockerConnectionContent {
        name: name.to_string(),
        kind,
        ssh_host: (values.kind == FormKind::Ssh).then(|| ssh_host.to_string()),
        read_only: Some(values.read_only),
    })
}

/// The result of a Test Connection attempt, rendered as a status label.
enum TestStatus {
    Idle,
    Testing,
    Ok,
    Failed(String),
}

/// Modal dialog for creating or editing a Docker endpoint.
///
/// Unlike `database_ui::ConnectionModal`, this modal has no password/keychain
/// step: SSH authentication is expected to come from the user's own SSH
/// config/agent, so nothing secret is ever read from or written to this form.
pub struct DockerEndpointModal {
    name_field: Entity<InputField>,
    ssh_host_field: Entity<InputField>,
    kind: FormKind,
    read_only: bool,
    /// The endpoint being edited, if any. Used to exclude its own name from
    /// the duplicate check and to rewrite it in-place on save.
    existing: Option<DockerConnectionContent>,
    /// Names of already-configured endpoints, for the duplicate-name check.
    existing_names: Vec<String>,
    client_factory: ClientFactory,
    fs: Arc<dyn Fs>,
    test_status: TestStatus,
}

impl DockerEndpointModal {
    pub fn new(
        existing: Option<DockerConnectionContent>,
        existing_names: Vec<String>,
        client_factory: ClientFactory,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "Endpoint name")
                .label("Name")
                .tab_index(1);
            if let Some(existing) = existing.as_ref() {
                field.set_text(&existing.name, window, cx);
                // The name is the endpoint's identity in settings, so it
                // cannot change while editing.
                field.editor().set_read_only(true, cx);
            }
            field
        });
        let ssh_host_field = cx.new(|cx| {
            let field = InputField::new(window, cx, "user@host")
                .label("SSH host")
                .tab_index(2);
            if let Some(host) = existing
                .as_ref()
                .and_then(|existing| existing.ssh_host.as_ref())
            {
                field.set_text(host, window, cx);
            }
            field
        });

        // Default to SSH for the "add remote" use case, but preserve the
        // existing endpoint's kind when editing.
        let kind = match existing.as_ref().map(|existing| &existing.kind) {
            Some(DockerEndpointKindContent::Local) => FormKind::Local,
            Some(DockerEndpointKindContent::Ssh) | None => FormKind::Ssh,
        };
        let read_only = existing
            .as_ref()
            .and_then(|existing| existing.read_only)
            .unwrap_or(false);

        Self {
            name_field,
            ssh_host_field,
            kind,
            read_only,
            existing,
            existing_names,
            client_factory,
            fs,
            test_status: TestStatus::Idle,
        }
    }

    fn form_values(&self, cx: &App) -> FormValues {
        FormValues {
            name: self.name_field.read(cx).text(cx),
            kind: self.kind,
            ssh_host: self.ssh_host_field.read(cx).text(cx),
            read_only: self.read_only,
        }
    }

    /// The endpoint names the new/edited entry must not collide with. When
    /// editing, the endpoint's own original name is excluded.
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
        self.name_field
            .update(cx, |field, cx| field.set_error(None::<SharedString>, cx));
        self.ssh_host_field
            .update(cx, |field, cx| field.set_error(None::<SharedString>, cx));
    }

    fn set_field_error(&mut self, error: &FieldError, cx: &mut Context<Self>) {
        let field = match error.field {
            FormField::Name => &self.name_field,
            FormField::SshHost => &self.ssh_host_field,
        };
        let message = error.message.clone();
        field.update(cx, |field, cx| field.set_error(Some(message), cx));
    }

    fn set_kind(&mut self, kind: FormKind, cx: &mut Context<Self>) {
        self.kind = kind;
        cx.notify();
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
        let content = match build_content(&values, &self.names_to_check()) {
            Ok(content) => content,
            Err(error) => {
                self.set_field_error(&error, cx);
                return;
            }
        };

        let endpoint = DockerEndpoint {
            name: content.name,
            kind: match content.kind {
                DockerEndpointKindContent::Local => EndpointKind::Local,
                DockerEndpointKindContent::Ssh => EndpointKind::Ssh {
                    host: content.ssh_host.unwrap_or_default(),
                },
            },
            read_only: content.read_only.unwrap_or(false),
        };
        let client = (self.client_factory)();

        self.test_status = TestStatus::Testing;
        cx.notify();

        let task =
            gpui_tokio::Tokio::spawn_result(
                cx,
                async move { client.test_endpoint(&endpoint).await },
            );
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                this.test_status = match result {
                    Ok(()) => TestStatus::Ok,
                    Err(error) => TestStatus::Failed(format!("{error:#}")),
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
        let content = match build_content(&values, &self.names_to_check()) {
            Ok(content) => content,
            Err(error) => {
                self.set_field_error(&error, cx);
                return;
            }
        };

        // On edit, the name field is read-only, so `content.name` always
        // equals the original name and we simply replace that entry.
        let original_name = self.existing.as_ref().map(|existing| existing.name.clone());

        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            let connections = settings
                .docker
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

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for DockerEndpointModal {}
impl ModalView for DockerEndpointModal {}

impl Focusable for DockerEndpointModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        // The name field is the first tab stop; focusing it opens the modal
        // on the top field.
        self.name_field.focus_handle(cx)
    }
}

impl Render for DockerEndpointModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_edit = self.existing.is_some();
        let title = if is_edit {
            "Edit Endpoint"
        } else {
            "Add Endpoint"
        };
        let testing = matches!(self.test_status, TestStatus::Testing);
        let is_ssh = self.kind == FormKind::Ssh;

        let status_label = match &self.test_status {
            TestStatus::Idle => None,
            TestStatus::Testing => Some(
                Label::new("Testing…")
                    .color(Color::Muted)
                    .into_any_element(),
            ),
            TestStatus::Ok => Some(
                Label::new("Connected")
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
            .key_context("DockerEndpointModal")
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
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Kind").size(LabelSize::Small))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("kind-local", "Local")
                                            .style(if is_ssh {
                                                ButtonStyle::Subtle
                                            } else {
                                                ButtonStyle::Filled
                                            })
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.set_kind(FormKind::Local, cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("kind-ssh", "SSH")
                                            .style(if is_ssh {
                                                ButtonStyle::Filled
                                            } else {
                                                ButtonStyle::Subtle
                                            })
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.set_kind(FormKind::Ssh, cx);
                                            })),
                                    ),
                            ),
                    )
                    .when(is_ssh, |this| this.child(self.ssh_host_field.clone()))
                    .child(
                        Checkbox::new("read-only", self.read_only.into())
                            .label("Read-only")
                            .on_click(cx.listener(|this, state: &ToggleState, _window, cx| {
                                this.read_only = *state == ToggleState::Selected;
                                cx.notify();
                            })),
                    )
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

    fn values(name: &str, kind: FormKind, ssh_host: &str, read_only: bool) -> FormValues {
        FormValues {
            name: name.into(),
            kind,
            ssh_host: ssh_host.into(),
            read_only,
        }
    }

    #[test]
    fn ssh_with_host_produces_ssh_content() {
        let content = build_content(&values("prod", FormKind::Ssh, "deploy@1.2.3.4", true), &[])
            .expect("valid SSH values should produce content");
        assert_eq!(content.name, "prod");
        assert!(matches!(content.kind, DockerEndpointKindContent::Ssh));
        assert_eq!(content.ssh_host.as_deref(), Some("deploy@1.2.3.4"));
        assert_eq!(content.read_only, Some(true));
    }

    #[test]
    fn local_without_host_produces_local_content_with_no_ssh_host() {
        let content = build_content(&values("home", FormKind::Local, "", false), &[])
            .expect("valid Local values should produce content");
        assert_eq!(content.name, "home");
        assert!(matches!(content.kind, DockerEndpointKindContent::Local));
        assert_eq!(content.ssh_host, None);
        assert_eq!(content.read_only, Some(false));
    }

    #[test]
    fn local_ignores_stray_ssh_host_text() {
        // Even if the SSH host field retains stale text (e.g. the user typed
        // it, then switched back to Local), the produced content must not
        // carry it: `ssh_host` is only meaningful for `Ssh`.
        let content = build_content(
            &values("home", FormKind::Local, "leftover@host", false),
            &[],
        )
        .expect("Local should not require ssh_host");
        assert_eq!(content.ssh_host, None);
    }

    #[test]
    fn empty_name_is_rejected() {
        let error = build_content(&values("  ", FormKind::Local, "", false), &[])
            .expect_err("empty name should be rejected");
        assert_eq!(error.field, FormField::Name);
    }

    #[test]
    fn ssh_without_host_is_rejected() {
        let error = build_content(&values("prod", FormKind::Ssh, "  ", false), &[])
            .expect_err("SSH without a host should be rejected");
        assert_eq!(error.field, FormField::SshHost);
    }

    #[test]
    fn duplicate_name_is_rejected_on_create() {
        let error = build_content(
            &values("prod", FormKind::Ssh, "deploy@host", false),
            &["prod".to_string()],
        )
        .expect_err("duplicate name should be rejected");
        assert_eq!(error.field, FormField::Name);
    }

    #[test]
    fn same_name_allowed_when_excluded_from_existing() {
        // Emulates editing: the endpoint's own name is excluded from the list.
        let content = build_content(&values("prod", FormKind::Ssh, "deploy@host", false), &[])
            .expect("same name is fine when it is not in the existing list");
        assert_eq!(content.name, "prod");
    }

    #[test]
    fn values_are_trimmed() {
        let content = build_content(
            &values("  prod  ", FormKind::Ssh, "  deploy@host  ", false),
            &[],
        )
        .expect("surrounding whitespace should be trimmed");
        assert_eq!(content.name, "prod");
        assert_eq!(content.ssh_host.as_deref(), Some("deploy@host"));
    }
}
