use std::sync::Arc;

use anyhow::{Context as _, Result};
use context_server::{ContextServerCommand, ContextServerId};
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, TextStyle, WeakEntity, prelude::*,
};
use language::{Language, LanguageRegistry};
use project::project_settings::{ContextServerConfiguration, ProjectSettings};
use settings::{Settings as _, update_settings_file};
use theme::ThemeSettings;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, prelude::*};
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

pub struct ConfigureContextServerModal {
    workspace: WeakEntity<Workspace>,
    editor: Entity<Editor>,
    configuring_existing_server: bool,
    last_error: Option<SharedString>,
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
                cx.spawn_in(window, {
                    let language_registry = language_registry.clone();
                    async move |this, cx| {
                        let jsonc_language =
                            language_registry.language_for_name("jsonc").await.ok();
                        let workspace_handle = this.clone();
                        this.update_in(cx, |workspace, window, cx| {
                            workspace.toggle_modal(window, cx, |window, cx| {
                                Self::new(workspace_handle, jsonc_language, None, window, cx)
                            })
                        })
                    }
                })
                .detach()
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

        window
            .spawn(cx, {
                async move |cx| {
                    let jsonc_language = language_registry.language_for_name("jsonc").await.ok();
                    let workspace_handle = workspace.clone();
                    workspace.update_in(cx, |workspace, window, cx| {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            Self::new(
                                workspace_handle,
                                jsonc_language,
                                Some((server_id, server_command)),
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
            workspace,
            configuring_existing_server,
            last_error: None,
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut Context<Self>) {
        match parse_input(&self.editor.read(cx).text(cx)) {
            Ok((name, config)) => {
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        let fs = workspace.app_state().fs.clone();
                        update_settings_file::<ProjectSettings>(fs.clone(), cx, |settings, _| {
                            settings.context_servers.insert(name.into(), config);
                        });
                    });
                }
                cx.emit(DismissEvent);
            }
            Err(error) => {
                self.last_error = Some(error.to_string().into());
                cx.notify();
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
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

impl Render for ConfigureContextServerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const MODAL_DESCRIPTION: &'static str = "Visit the MCP server configuration docs to find all necessary arguments and environment variables.";

        let focus_handle = self.focus_handle(cx);

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
                Modal::new("add-context-server", None)
                    .header(
                        ModalHeader::new().headline(if self.configuring_existing_server {
                            "Configure MCP Server"
                        } else {
                            "Add MCP Server"
                        }),
                    )
                    .section(
                        Section::new()
                            .child(Label::new(MODAL_DESCRIPTION).color(Color::Muted))
                            .child(
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
                                            line_height: relative(
                                                settings.buffer_line_height.value(),
                                            ),
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
                                    }),
                            )
                            .when_some(self.last_error.clone(), |this, error| {
                                this.child(
                                    h_flex()
                                        .gap_2()
                                        .px_2()
                                        .py_1()
                                        .child(
                                            Icon::new(IconName::Warning)
                                                .size(IconSize::XSmall)
                                                .color(Color::Warning),
                                        )
                                        .child(
                                            div().w_full().child(
                                                Label::new(error)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            ),
                                        ),
                                )
                            }),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("cancel", "Cancel")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Cancel,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        )
                                        .on_click(cx.listener(|this, _event, _window, cx| {
                                            this.cancel(&menu::Cancel, cx)
                                        })),
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
                                        KeyBinding::for_action_in(
                                            &menu::Confirm,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                    )
                                    .on_click(cx.listener(
                                        |this, _event, _window, cx| {
                                            this.confirm(&menu::Confirm, cx)
                                        },
                                    )),
                                ),
                        ),
                    ),
            )
    }
}
