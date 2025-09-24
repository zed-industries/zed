use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Context as _, Result};
use context_server::{ContextServerCommand, ContextServerId};
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    AsyncWindowContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task,
    TextStyle, TextStyleRefinement, UnderlineStyle, WeakEntity, prelude::*,
};
use language::{Language, LanguageRegistry};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::{
    context_server_store::{
        ContextServerStatus, ContextServerStore, registry::ContextServerDescriptorRegistry,
    },
    project_settings::{ContextServerSettings, ProjectSettings},
    worktree_store::WorktreeStore,
};
use settings::{Settings as _, update_settings_file};
use theme::ThemeSettings;
use ui::{
    CommonAnimationExt, KeyBinding, Modal, ModalFooter, ModalHeader, Section, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{ModalView, Workspace};

use crate::AddContextServer;

enum ConfigurationTarget {
    New,
    Existing {
        id: ContextServerId,
        command: ContextServerCommand,
    },
    Extension {
        id: ContextServerId,
        repository_url: Option<SharedString>,
        installation: Option<extension::ContextServerConfiguration>,
    },
}

enum ConfigurationSource {
    New {
        editor: Entity<Editor>,
    },
    Existing {
        editor: Entity<Editor>,
    },
    Extension {
        id: ContextServerId,
        editor: Option<Entity<Editor>>,
        repository_url: Option<SharedString>,
        installation_instructions: Option<Entity<markdown::Markdown>>,
        settings_validator: Option<jsonschema::Validator>,
    },
}

impl ConfigurationSource {
    fn has_configuration_options(&self) -> bool {
        !matches!(self, ConfigurationSource::Extension { editor: None, .. })
    }

    fn is_new(&self) -> bool {
        matches!(self, ConfigurationSource::New { .. })
    }

    fn from_target(
        target: ConfigurationTarget,
        language_registry: Arc<LanguageRegistry>,
        jsonc_language: Option<Arc<Language>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        fn create_editor(
            json: String,
            jsonc_language: Option<Arc<Language>>,
            window: &mut Window,
            cx: &mut App,
        ) -> Entity<Editor> {
            cx.new(|cx| {
                let mut editor = Editor::auto_height(4, 16, window, cx);
                editor.set_text(json, window, cx);
                editor.set_show_gutter(false, cx);
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
                if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                    buffer.update(cx, |buffer, cx| buffer.set_language(jsonc_language, cx))
                }
                editor
            })
        }

        match target {
            ConfigurationTarget::New => ConfigurationSource::New {
                editor: create_editor(context_server_input(None), jsonc_language, window, cx),
            },
            ConfigurationTarget::Existing { id, command } => ConfigurationSource::Existing {
                editor: create_editor(
                    context_server_input(Some((id, command))),
                    jsonc_language,
                    window,
                    cx,
                ),
            },
            ConfigurationTarget::Extension {
                id,
                repository_url,
                installation,
            } => {
                let settings_validator = installation.as_ref().and_then(|installation| {
                    jsonschema::validator_for(&installation.settings_schema)
                        .context("Failed to load JSON schema for context server settings")
                        .log_err()
                });
                let installation_instructions = installation.as_ref().map(|installation| {
                    cx.new(|cx| {
                        Markdown::new(
                            installation.installation_instructions.clone().into(),
                            Some(language_registry.clone()),
                            None,
                            cx,
                        )
                    })
                });
                ConfigurationSource::Extension {
                    id,
                    repository_url,
                    installation_instructions,
                    settings_validator,
                    editor: installation.map(|installation| {
                        create_editor(installation.default_settings, jsonc_language, window, cx)
                    }),
                }
            }
        }
    }

    fn output(&self, cx: &mut App) -> Result<(ContextServerId, ContextServerSettings)> {
        match self {
            ConfigurationSource::New { editor } | ConfigurationSource::Existing { editor } => {
                parse_input(&editor.read(cx).text(cx)).map(|(id, command)| {
                    (
                        id,
                        ContextServerSettings::Custom {
                            enabled: true,
                            command,
                        },
                    )
                })
            }
            ConfigurationSource::Extension {
                id,
                editor,
                settings_validator,
                ..
            } => {
                let text = editor
                    .as_ref()
                    .context("No output available")?
                    .read(cx)
                    .text(cx);
                let settings = serde_json_lenient::from_str::<serde_json::Value>(&text)?;
                if let Some(settings_validator) = settings_validator
                    && let Err(error) = settings_validator.validate(&settings)
                {
                    return Err(anyhow::anyhow!(error.to_string()));
                }
                Ok((
                    id.clone(),
                    ContextServerSettings::Extension {
                        enabled: true,
                        settings,
                    },
                ))
            }
        }
    }
}

fn context_server_input(existing: Option<(ContextServerId, ContextServerCommand)>) -> String {
    let (name, command, args, env) = match existing {
        Some((id, cmd)) => {
            let args = serde_json::to_string(&cmd.args).unwrap();
            let env = serde_json::to_string(&cmd.env.unwrap_or_default()).unwrap();
            (id.0.to_string(), cmd.path, args, env)
        }
        None => (
            "some-mcp-server".to_string(),
            PathBuf::new(),
            "[]".to_string(),
            "{}".to_string(),
        ),
    };

    format!(
        r#"{{
  /// The name of your MCP server
  "{name}": {{
    /// The command which runs the MCP server
    "command": "{}",
    /// The arguments to pass to the MCP server
    "args": {args},
    /// The environment variables to set
    "env": {env}
  }}
}}"#,
        command.display()
    )
}

fn resolve_context_server_extension(
    id: ContextServerId,
    worktree_store: Entity<WorktreeStore>,
    cx: &mut App,
) -> Task<Option<ConfigurationTarget>> {
    let registry = ContextServerDescriptorRegistry::default_global(cx).read(cx);

    let Some(descriptor) = registry.context_server_descriptor(&id.0) else {
        return Task::ready(None);
    };

    let extension = crate::agent_configuration::resolve_extension_for_context_server(&id, cx);
    cx.spawn(async move |cx| {
        let installation = descriptor
            .configuration(worktree_store, cx)
            .await
            .context("Failed to resolve context server configuration")
            .log_err()
            .flatten();

        Some(ConfigurationTarget::Extension {
            id,
            repository_url: extension
                .and_then(|(_, manifest)| manifest.repository.clone().map(SharedString::from)),
            installation,
        })
    })
}

enum State {
    Idle,
    Waiting,
    Error(SharedString),
}

pub struct ConfigureContextServerModal {
    context_server_store: Entity<ContextServerStore>,
    workspace: WeakEntity<Workspace>,
    source: ConfigurationSource,
    state: State,
    original_server_id: Option<ContextServerId>,
}

impl ConfigureContextServerModal {
    pub fn register(
        workspace: &mut Workspace,
        language_registry: Arc<LanguageRegistry>,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action({
            move |_workspace, _: &AddContextServer, window, cx| {
                let workspace_handle = cx.weak_entity();
                let language_registry = language_registry.clone();
                window
                    .spawn(cx, async move |cx| {
                        Self::show_modal(
                            ConfigurationTarget::New,
                            language_registry,
                            workspace_handle,
                            cx,
                        )
                        .await
                    })
                    .detach_and_log_err(cx);
            }
        });
    }

    pub fn show_modal_for_existing_server(
        server_id: ContextServerId,
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(settings) = ProjectSettings::get_global(cx)
            .context_servers
            .get(&server_id.0)
            .cloned()
            .or_else(|| {
                ContextServerDescriptorRegistry::default_global(cx)
                    .read(cx)
                    .context_server_descriptor(&server_id.0)
                    .map(|_| ContextServerSettings::default_extension())
            })
        else {
            return Task::ready(Err(anyhow::anyhow!("Context server not found")));
        };

        window.spawn(cx, async move |cx| {
            let target = match settings {
                ContextServerSettings::Custom {
                    enabled: _,
                    command,
                } => Some(ConfigurationTarget::Existing {
                    id: server_id,
                    command,
                }),
                ContextServerSettings::Extension { .. } => {
                    match workspace
                        .update(cx, |workspace, cx| {
                            resolve_context_server_extension(
                                server_id,
                                workspace.project().read(cx).worktree_store(),
                                cx,
                            )
                        })
                        .ok()
                    {
                        Some(task) => task.await,
                        None => None,
                    }
                }
            };

            match target {
                Some(target) => Self::show_modal(target, language_registry, workspace, cx).await,
                None => Err(anyhow::anyhow!("Failed to resolve context server")),
            }
        })
    }

    fn show_modal(
        target: ConfigurationTarget,
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
        cx: &mut AsyncWindowContext,
    ) -> Task<Result<()>> {
        cx.spawn(async move |cx| {
            let jsonc_language = language_registry.language_for_name("jsonc").await.ok();
            workspace.update_in(cx, |workspace, window, cx| {
                let workspace_handle = cx.weak_entity();
                let context_server_store = workspace.project().read(cx).context_server_store();
                workspace.toggle_modal(window, cx, |window, cx| Self {
                    context_server_store,
                    workspace: workspace_handle,
                    state: State::Idle,
                    original_server_id: match &target {
                        ConfigurationTarget::Existing { id, .. } => Some(id.clone()),
                        ConfigurationTarget::Extension { id, .. } => Some(id.clone()),
                        ConfigurationTarget::New => None,
                    },
                    source: ConfigurationSource::from_target(
                        target,
                        language_registry,
                        jsonc_language,
                        window,
                        cx,
                    ),
                })
            })
        })
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

        let (id, settings) = match self.source.output(cx) {
            Ok(val) => val,
            Err(error) => {
                self.set_error(error.to_string(), cx);
                return;
            }
        };

        self.state = State::Waiting;

        let existing_server = self.context_server_store.read(cx).get_running_server(&id);
        if existing_server.is_some() {
            self.context_server_store.update(cx, |store, cx| {
                store.stop_server(&id, cx).log_err();
            });
        }

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

        let settings_changed =
            ProjectSettings::get_global(cx).context_servers.get(&id.0) != Some(&settings);

        if settings_changed {
            // When we write the settings to the file, the context server will be restarted.
            workspace.update(cx, |workspace, cx| {
                let fs = workspace.app_state().fs.clone();
                let original_server_id = self.original_server_id.clone();
                update_settings_file(fs.clone(), cx, move |current, _| {
                    if let Some(original_id) = original_server_id {
                        if original_id != id {
                            current.project.context_servers.remove(&original_id.0);
                        }
                    }
                    current
                        .project
                        .context_servers
                        .insert(id.0, settings.into());
                });
            });
        } else if let Some(existing_server) = existing_server {
            self.context_server_store
                .update(cx, |store, cx| store.start_server(existing_server, cx));
        }
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
                            this.icon(ToastIcon::new(IconName::ToolHammer).color(Color::Muted))
                                .action("Dismiss", |_, _| {})
                        },
                    );

                    workspace.toggle_status_toast(status_toast, cx);
                }
            })
            .log_err();
    }
}

fn parse_input(text: &str) -> Result<(ContextServerId, ContextServerCommand)> {
    let value: serde_json::Value = serde_json_lenient::from_str(text)?;
    let object = value.as_object().context("Expected object")?;
    anyhow::ensure!(object.len() == 1, "Expected exactly one key-value pair");
    let (context_server_name, value) = object.into_iter().next().unwrap();
    let command: ContextServerCommand = serde_json::from_value(value.clone())?;
    Ok((ContextServerId(context_server_name.clone().into()), command))
}

impl ModalView for ConfigureContextServerModal {}

impl Focusable for ConfigureContextServerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.source {
            ConfigurationSource::New { editor } => editor.focus_handle(cx),
            ConfigurationSource::Existing { editor, .. } => editor.focus_handle(cx),
            ConfigurationSource::Extension { editor, .. } => editor
                .as_ref()
                .map(|editor| editor.focus_handle(cx))
                .unwrap_or_else(|| cx.focus_handle()),
        }
    }
}

impl EventEmitter<DismissEvent> for ConfigureContextServerModal {}

impl ConfigureContextServerModal {
    fn render_modal_header(&self) -> ModalHeader {
        let text: SharedString = match &self.source {
            ConfigurationSource::New { .. } => "Add MCP Server".into(),
            ConfigurationSource::Existing { .. } => "Configure MCP Server".into(),
            ConfigurationSource::Extension { id, .. } => format!("Configure {}", id.0).into(),
        };
        ModalHeader::new().headline(text)
    }

    fn render_modal_description(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        const MODAL_DESCRIPTION: &str = "Visit the MCP server configuration docs to find all necessary arguments and environment variables.";

        if let ConfigurationSource::Extension {
            installation_instructions: Some(installation_instructions),
            ..
        } = &self.source
        {
            div()
                .pb_2()
                .text_sm()
                .child(MarkdownElement::new(
                    installation_instructions.clone(),
                    default_markdown_style(window, cx),
                ))
                .into_any_element()
        } else {
            Label::new(MODAL_DESCRIPTION)
                .color(Color::Muted)
                .into_any_element()
        }
    }

    fn render_modal_content(&self, cx: &App) -> AnyElement {
        let editor = match &self.source {
            ConfigurationSource::New { editor } => editor,
            ConfigurationSource::Existing { editor } => editor,
            ConfigurationSource::Extension { editor, .. } => {
                let Some(editor) = editor else {
                    return div().into_any_element();
                };
                editor
            }
        };

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
                    editor,
                    EditorStyle {
                        background: cx.theme().colors().editor_background,
                        local_player: cx.theme().players().local(),
                        text: text_style,
                        syntax: cx.theme().syntax().clone(),
                        ..Default::default()
                    },
                )
            })
            .into_any_element()
    }

    fn render_modal_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> ModalFooter {
        let focus_handle = self.focus_handle(cx);
        let is_connecting = matches!(self.state, State::Waiting);

        ModalFooter::new()
            .start_slot::<Button>(
                if let ConfigurationSource::Extension {
                    repository_url: Some(repository_url),
                    ..
                } = &self.source
                {
                    Some(
                        Button::new("open-repository", "Open Repository")
                            .icon(IconName::ArrowUpRight)
                            .icon_color(Color::Muted)
                            .icon_size(IconSize::Small)
                            .tooltip({
                                let repository_url = repository_url.clone();
                                move |window, cx| {
                                    Tooltip::with_meta(
                                        "Open Repository",
                                        None,
                                        repository_url.clone(),
                                        window,
                                        cx,
                                    )
                                }
                            })
                            .on_click({
                                let repository_url = repository_url.clone();
                                move |_, _, cx| cx.open_url(&repository_url)
                            }),
                    )
                } else {
                    None
                },
            )
            .end_slot(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new(
                            "cancel",
                            if self.source.has_configuration_options() {
                                "Cancel"
                            } else {
                                "Dismiss"
                            },
                        )
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Cancel, &focus_handle, window, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(
                            cx.listener(|this, _event, _window, cx| this.cancel(&menu::Cancel, cx)),
                        ),
                    )
                    .children(self.source.has_configuration_options().then(|| {
                        Button::new(
                            "add-server",
                            if self.source.is_new() {
                                "Add Server"
                            } else {
                                "Configure Server"
                            },
                        )
                        .disabled(is_connecting)
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Confirm, &focus_handle, window, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(
                            cx.listener(|this, _event, _window, cx| {
                                this.confirm(&menu::Confirm, cx)
                            }),
                        )
                    })),
            )
    }

    fn render_waiting_for_context_server() -> Div {
        h_flex()
            .gap_2()
            .child(
                Icon::new(IconName::ArrowCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Info)
                    .with_rotate_animation(2)
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
                            .child(self.render_modal_description(window, cx))
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
                    if server_id == &context_server_id
                        && let Some(tx) = tx.lock().unwrap().take()
                    {
                        let _ = tx.send(Ok(()));
                    }
                }
                ContextServerStatus::Stopped => {
                    if server_id == &context_server_id
                        && let Some(tx) = tx.lock().unwrap().take()
                    {
                        let _ = tx.send(Err("Context server stopped running".into()));
                    }
                }
                ContextServerStatus::Error(error) => {
                    if server_id == &context_server_id
                        && let Some(tx) = tx.lock().unwrap().take()
                    {
                        let _ = tx.send(Err(error.clone()));
                    }
                }
                _ => {}
            }
        }
    });

    cx.spawn(async move |_cx| {
        let result = rx
            .await
            .map_err(|_| Arc::from("Context server store was dropped"))?;
        drop(subscription);
        result
    })
}

pub(crate) fn default_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();
    let mut text_style = window.text_style();
    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(TextSize::XSmall.rems(cx).into()),
        color: Some(colors.text_muted),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        selection_background_color: colors.element_selection_background,
        link: TextStyleRefinement {
            background_color: Some(colors.editor_foreground.opacity(0.025)),
            underline: Some(UnderlineStyle {
                color: Some(colors.text_accent.opacity(0.5)),
                thickness: px(1.),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}
