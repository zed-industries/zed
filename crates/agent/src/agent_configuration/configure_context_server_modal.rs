use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context as _, Result};
use context_server::{ContextServerCommand, ContextServerId};
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    Animation, AnimationExt as _, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task,
    TextStyle, Transformation, WeakEntity, percentage, prelude::*,
};
use language::{Language, LanguageRegistry};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::{
    context_server_store::{ContextServerStatus, ContextServerStore},
    project_settings::{ContextServerConfiguration, ProjectSettings},
};
use settings::{Settings as _, update_settings_file};
use theme::ThemeSettings;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, prelude::*};
use util::ResultExt as _;
use workspace::{ModalView, Workspace};

use crate::AddContextServer;

fn context_server_input(existing: Option<(ContextServerId, ContextServerCommand)>) -> String {
    let (name, path, args, env) = match existing {
        Some((id, cmd)) => {
            let args = serde_json::to_string(&cmd.args).unwrap();
            let env = serde_json::to_string(&cmd.env.unwrap_or_default()).unwrap();
            (id.0.to_string(), cmd.path, args, env)
        }
        None => (
            "some-mcp-server".to_string(),
            "".to_string(),
            "[]".to_string(),
            "{}".to_string(),
        ),
    };

    format!(
        r#"{{
  /// The name of your MCP server
  "{name}": {{
    "command": {{
      /// The path to the executable
      "path": "{path}",
      /// The arguments to pass to the executable
      "args": {args},
      /// The environment variables to set for the executable
      "env": {env}
    }}
  }}
}}"#
    )
}

enum State {
    Idle,
    Waiting,
    Error(SharedString),
}

pub struct ConfigureContextServerModal {
    context_server_store: Entity<ContextServerStore>,
    workspace: WeakEntity<Workspace>,
    editor: Entity<Editor>,
    configuring_existing_server: bool,
    state: State,
}

impl ConfigureContextServerModal {
    pub fn register(
        workspace: &mut Workspace,
        language_registry: Arc<LanguageRegistry>,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action({
            let language_registry = language_registry.clone();
            move |_workspace, _: &AddContextServer, window, cx| {
                Self::show_modal(
                    cx.weak_entity(),
                    language_registry.clone(),
                    None,
                    window,
                    cx,
                );
            }
        });
    }

    pub fn for_existing_server(
        server_id: ContextServerId,
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(server_command) = ProjectSettings::get_global(cx)
            .context_servers
            .get(&server_id.0)
            .and_then(|config| config.command.clone())
        else {
            return;
        };

        Self::show_modal(
            workspace,
            language_registry,
            Some((server_id, server_command)),
            window,
            cx,
        )
    }

    pub fn show_modal(
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        existing_configuration: Option<(ContextServerId, ContextServerCommand)>,
        window: &mut Window,
        cx: &mut App,
    ) {
        window
            .spawn(cx, {
                async move |cx| {
                    let jsonc_language = language_registry.language_for_name("jsonc").await.ok();
                    workspace.update_in(cx, |workspace, window, cx| {
                        let workspace_handle = cx.weak_entity();
                        let context_server_store =
                            workspace.project().read(cx).context_server_store();
                        workspace.toggle_modal(window, cx, |window, cx| {
                            Self::new(
                                context_server_store,
                                workspace_handle,
                                jsonc_language,
                                existing_configuration,
                                window,
                                cx,
                            )
                        })
                    })
                }
            })
            .detach()
    }

    pub fn new(
        context_server_store: Entity<ContextServerStore>,
        workspace: WeakEntity<Workspace>,
        jsonc_language: Option<Arc<Language>>,
        existing_configuration: Option<(ContextServerId, ContextServerCommand)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let configuring_existing_server = existing_configuration.is_some();
        let json = context_server_input(existing_configuration);
        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(16, window, cx);
            editor.set_text(json, window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
            if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                buffer.update(cx, |buffer, cx| buffer.set_language(jsonc_language, cx))
            }
            editor
        });

        Self {
            editor,
            context_server_store,
            workspace,
            configuring_existing_server,
            state: State::Idle,
        }
    }

    fn set_error(&mut self, err: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.state = State::Error(err.into());
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut Context<Self>) {
        self.state = State::Idle;
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let (id, config) = match parse_input(&self.editor.read(cx).text(cx)) {
            Ok((name, config)) => (ContextServerId(name.into()), config),
            Err(error) => {
                self.set_error(error.to_string(), cx);
                return;
            }
        };

        self.state = State::Waiting;
        let wait_for_context_server_task =
            wait_for_context_server(&self.context_server_store, id.clone(), cx);
        cx.spawn({
            let id = id.clone();
            async move |this, cx| {
                let result = wait_for_context_server_task.await;
                this.update(cx, |this, cx| match result {
                    Ok(_) => {
                        this.state = State::Idle;
                        this.show_configured_context_server_toast(id, cx);
                        cx.emit(DismissEvent);
                    }
                    Err(err) => {
                        this.set_error(err, cx);
                    }
                })
            }
        })
        .detach();

        // When we write the settings to the file, the context server will be restarted.
        workspace.update(cx, |workspace, cx| {
            let fs = workspace.app_state().fs.clone();
            update_settings_file::<ProjectSettings>(fs.clone(), cx, |settings, _| {
                settings.context_servers.insert(id.0, config);
            });
        });
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn show_configured_context_server_toast(&self, id: ContextServerId, cx: &mut App) {
        self.workspace
            .update(cx, {
                |workspace, cx| {
                    let status_toast = StatusToast::new(
                        format!("{} configured successfully.", id.0),
                        cx,
                        |this, _cx| {
                            this.icon(ToastIcon::new(IconName::Hammer).color(Color::Muted))
                                .action("Dismiss", |_, _| {})
                        },
                    );

                    workspace.toggle_status_toast(status_toast, cx);
                }
            })
            .log_err();
    }
}

fn parse_input(text: &str) -> Result<(String, ContextServerConfiguration)> {
    let value: serde_json::Value = serde_json_lenient::from_str(text)?;
    let object = value.as_object().context("Expected object")?;
    anyhow::ensure!(object.len() == 1, "Expected exactly one key-value pair");
    let (context_server_name, value) = object.into_iter().next().unwrap();
    let config: ContextServerConfiguration = serde_json::from_value(value.clone())?;
    Ok((context_server_name.clone(), config))
}

impl ModalView for ConfigureContextServerModal {}

impl Focusable for ConfigureContextServerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx).clone()
    }
}

impl EventEmitter<DismissEvent> for ConfigureContextServerModal {}

impl ConfigureContextServerModal {
    fn render_modal_header(&self) -> ModalHeader {
        ModalHeader::new().headline(if self.configuring_existing_server {
            "Configure MCP Server"
        } else {
            "Add MCP Server"
        })
    }

    fn render_modal_description() -> Label {
        const MODAL_DESCRIPTION: &'static str = "Visit the MCP server configuration docs to find all necessary arguments and environment variables.";

        Label::new(MODAL_DESCRIPTION).color(Color::Muted)
    }

    fn render_modal_content(&self, cx: &App) -> Div {
        div()
            .p_2()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().editor_background)
            .child({
                let settings = ThemeSettings::get_global(cx);
                let text_style = TextStyle {
                    color: cx.theme().colors().text,
                    font_family: settings.buffer_font.family.clone(),
                    font_fallbacks: settings.buffer_font.fallbacks.clone(),
                    font_size: settings.buffer_font_size(cx).into(),
                    font_weight: settings.buffer_font.weight,
                    line_height: relative(settings.buffer_line_height.value()),
                    ..Default::default()
                };
                EditorElement::new(
                    &self.editor,
                    EditorStyle {
                        background: cx.theme().colors().editor_background,
                        local_player: cx.theme().players().local(),
                        text: text_style,
                        syntax: cx.theme().syntax().clone(),
                        ..Default::default()
                    },
                )
            })
    }

    fn render_modal_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> ModalFooter {
        let focus_handle = self.focus_handle(cx);

        ModalFooter::new().end_slot(
            h_flex()
                .gap_2()
                .child(
                    Button::new("cancel", "Cancel")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Cancel, &focus_handle, window, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(
                            cx.listener(|this, _event, _window, cx| this.cancel(&menu::Cancel, cx)),
                        ),
                )
                .child(
                    Button::new(
                        "add-server",
                        if self.configuring_existing_server {
                            "Configure Server"
                        } else {
                            "Add Server"
                        },
                    )
                    .key_binding(
                        KeyBinding::for_action_in(&menu::Confirm, &focus_handle, window, cx)
                            .map(|kb| kb.size(rems_from_px(12.))),
                    )
                    .on_click(
                        cx.listener(|this, _event, _window, cx| this.confirm(&menu::Confirm, cx)),
                    ),
                ),
        )
    }

    fn render_waiting_for_context_server() -> Div {
        h_flex()
            .gap_2()
            .child(
                Icon::new(IconName::ArrowCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Info)
                    .with_animation(
                        "arrow-circle",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    )
                    .into_any_element(),
            )
            .child(
                Label::new("Waiting for Context Server")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
    }

    fn render_modal_error(error: SharedString) -> Div {
        h_flex()
            .gap_2()
            .child(
                Icon::new(IconName::Warning)
                    .size(IconSize::XSmall)
                    .color(Color::Warning),
            )
            .child(
                div()
                    .w_full()
                    .child(Label::new(error).size(LabelSize::Small).color(Color::Muted)),
            )
    }
}

impl Render for ConfigureContextServerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ConfigureContextServerModal")
            .on_action(
                cx.listener(|this, _: &menu::Cancel, _window, cx| this.cancel(&menu::Cancel, cx)),
            )
            .on_action(
                cx.listener(|this, _: &menu::Confirm, _window, cx| {
                    this.confirm(&menu::Confirm, cx)
                }),
            )
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .child(
                Modal::new("configure-context-server", None)
                    .header(self.render_modal_header())
                    .section(
                        Section::new()
                            .child(Self::render_modal_description())
                            .child(self.render_modal_content(cx))
                            .child(match &self.state {
                                State::Idle => div(),
                                State::Waiting => Self::render_waiting_for_context_server(),
                                State::Error(error) => Self::render_modal_error(error.clone()),
                            }),
                    )
                    .footer(self.render_modal_footer(window, cx)),
            )
    }
}

fn wait_for_context_server(
    context_server_store: &Entity<ContextServerStore>,
    context_server_id: ContextServerId,
    cx: &mut App,
) -> Task<Result<(), Arc<str>>> {
    let (tx, rx) = futures::channel::oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let subscription = cx.subscribe(context_server_store, move |_, event, _cx| match event {
        project::context_server_store::Event::ServerStatusChanged { server_id, status } => {
            match status {
                ContextServerStatus::Running => {
                    if server_id == &context_server_id {
                        if let Some(tx) = tx.lock().unwrap().take() {
                            let _ = tx.send(Ok(()));
                        }
                    }
                }
                ContextServerStatus::Stopped => {
                    if server_id == &context_server_id {
                        if let Some(tx) = tx.lock().unwrap().take() {
                            let _ = tx.send(Err("Context server stopped running".into()));
                        }
                    }
                }
                ContextServerStatus::Error(error) => {
                    if server_id == &context_server_id {
                        if let Some(tx) = tx.lock().unwrap().take() {
                            let _ = tx.send(Err(error.clone()));
                        }
                    }
                }
                _ => {}
            }
        }
    });

    cx.spawn(async move |_cx| {
        let result = rx.await.unwrap();
        drop(subscription);
        result
    })
}
