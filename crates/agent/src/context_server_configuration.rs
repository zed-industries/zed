use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context as _;
use context_server::{ContextServerDescriptorRegistry, manager::ContextServerManager};
use editor::{Editor, EditorElement, EditorStyle};
use extension::{ContextServerConfiguration, ExtensionManifest};
use gpui::{
    Animation, AnimationExt, App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task,
    TextStyle, Transformation, WeakEntity, percentage,
};
use language::{Language, LanguageRegistry};
use notifications::status_toast::{StatusToast, ToastIcon};
use settings::{Settings as _, update_settings_file};
use theme::ThemeSettings;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

use crate::AssistantPanel;

pub(crate) fn init(language_registry: Arc<LanguageRegistry>, cx: &mut App) {
    cx.observe_new(move |_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        if let Some(extension_events) = extension::ExtensionEvents::try_global(cx).as_ref() {
            cx.subscribe_in(extension_events, window, {
                let language_registry = language_registry.clone();
                move |workspace, _, event, window, cx| match event {
                    extension::Event::ExtensionInstalled(manifest) => {
                        show_configure_mcp_modal(
                            language_registry.clone(),
                            manifest,
                            workspace,
                            window,
                            cx,
                        );
                    }
                    extension::Event::ConfigureExtensionRequested(manifest) => {
                        if !manifest.context_servers.is_empty() {
                            show_configure_mcp_modal(
                                language_registry.clone(),
                                manifest,
                                workspace,
                                window,
                                cx,
                            );
                        }
                    }
                    _ => {}
                }
            })
            .detach();
        } else {
            log::info!(
                "No extension events global found. Skipping context server configuration wizard"
            );
        }
    })
    .detach();
}

fn show_configure_mcp_modal(
    language_registry: Arc<LanguageRegistry>,
    manifest: &Arc<ExtensionManifest>,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<'_, Workspace>,
) {
    let Some(context_server_manager) = workspace.panel::<AssistantPanel>(cx).map(|panel| {
        panel
            .read(cx)
            .thread_store()
            .read(cx)
            .context_server_manager()
    }) else {
        return;
    };

    let registry = ContextServerDescriptorRegistry::global(cx).read(cx);
    let project = workspace.project().clone();
    let configuration_tasks = manifest
        .context_servers
        .keys()
        .cloned()
        .filter_map({
            |key| {
                let descriptor = registry.context_server_descriptor(&key)?;
                Some(cx.spawn({
                    let project = project.clone();
                    async move |_, cx| {
                        descriptor
                            .configuration(project, &cx)
                            .await
                            .context("Failed to resolve context server configuration")
                            .log_err()
                            .flatten()
                            .map(|config| (key, config))
                    }
                }))
            }
        })
        .collect::<Vec<_>>();

    let jsonc_language = language_registry.language_for_name("jsonc");

    cx.spawn_in(window, async move |this, cx| {
        let descriptors = futures::future::join_all(configuration_tasks).await;
        let jsonc_language = jsonc_language.await.ok();

        this.update_in(cx, |this, window, cx| {
            let modal = ConfigureContextServerModal::from_configurations(
                descriptors.into_iter().flatten(),
                jsonc_language,
                context_server_manager,
                cx.entity().downgrade(),
                window,
                cx,
            );
            if let Some(modal) = modal {
                this.toggle_modal(window, cx, |_, _| modal);
            }
        })
    })
    .detach();
}

struct ConfigureContextServer {
    id: Arc<str>,
    installation_instructions: SharedString,
    settings_validator: Option<jsonschema::Validator>,
    settings_editor: Entity<Editor>,
    last_error: Option<SharedString>,
    waiting_for_context_server: bool,
}

struct ConfigureContextServerModal {
    workspace: WeakEntity<Workspace>,
    context_servers_to_setup: Vec<ConfigureContextServer>,
    context_server_manager: Entity<ContextServerManager>,
}

impl ConfigureContextServerModal {
    pub fn from_configurations(
        configurations: impl Iterator<Item = (Arc<str>, ContextServerConfiguration)>,
        jsonc_language: Option<Arc<Language>>,
        context_server_manager: Entity<ContextServerManager>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Self> {
        let context_servers_to_setup = configurations
            .map(|(id, manifest)| {
                let jsonc_language = jsonc_language.clone();
                let settings_validator = jsonschema::validator_for(&manifest.settings_schema)
                    .context("Failed to load JSON schema for context server settings")
                    .log_err();
                ConfigureContextServer {
                    id: id.clone(),
                    installation_instructions: manifest.installation_instructions.clone().into(),
                    settings_validator,
                    settings_editor: cx.new(|cx| {
                        let mut editor = Editor::auto_height(16, window, cx);
                        editor.set_text(manifest.default_settings, window, cx);
                        if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                            buffer.update(cx, |buffer, cx| buffer.set_language(jsonc_language, cx))
                        }
                        editor
                    }),
                    waiting_for_context_server: false,
                    last_error: None,
                }
            })
            .collect::<Vec<_>>();

        if context_servers_to_setup.is_empty() {
            return None;
        }

        Some(Self {
            workspace,
            context_servers_to_setup,
            context_server_manager,
        })
    }
}

impl ConfigureContextServerModal {
    pub fn confirm(&mut self, cx: &mut Context<Self>) {
        if self.context_servers_to_setup.is_empty() {
            return;
        }

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let configuration = &mut self.context_servers_to_setup[0];
        if configuration.waiting_for_context_server {
            return;
        }

        let settings_value = match serde_json_lenient::from_str::<serde_json::Value>(
            &configuration.settings_editor.read(cx).text(cx),
        ) {
            Ok(value) => value,
            Err(error) => {
                configuration.last_error = Some(error.to_string().into());
                cx.notify();
                return;
            }
        };

        if let Some(validator) = configuration.settings_validator.as_ref() {
            if let Err(error) = validator.validate(&settings_value) {
                configuration.last_error = Some(error.to_string().into());
                cx.notify();
                return;
            }
        }
        let id = configuration.id.clone();

        configuration.waiting_for_context_server = true;

        let task = wait_for_context_server(&self.context_server_manager, id.clone(), cx);
        cx.spawn({
            let id = id.clone();
            async move |this, cx| {
                let result = task.await;
                this.update(cx, |this, cx| match result {
                    Ok(_) => {
                        this.context_servers_to_setup.remove(0);
                        if this.context_servers_to_setup.is_empty() {
                            this.workspace
                                .update(cx, {
                                    |workspace, cx| {
                                        let status_toast = StatusToast::new(
                                            format!("{} MCP configured successfully", id.clone()),
                                            cx,
                                            |this, _cx| {
                                                this.icon(
                                                    ToastIcon::new(IconName::DatabaseZap)
                                                        .color(Color::Muted),
                                                )
                                                .action("Dismiss", |_, _| {})
                                            },
                                        );

                                        workspace.toggle_status_toast(status_toast, cx);
                                    }
                                })
                                .log_err();

                            this.dismiss(cx);
                        }
                        cx.notify();
                    }
                    Err(err) => {
                        if let Some(configuration) = this.context_servers_to_setup.get_mut(0) {
                            configuration.last_error = Some(err.into());
                            configuration.waiting_for_context_server = false;
                        } else {
                            this.dismiss(cx);
                        }
                        cx.notify();
                    }
                })
            }
        })
        .detach();

        // When we write the settings to the file, the context server will be restarted.
        update_settings_file::<context_server::ContextServerSettings>(
            workspace.read(cx).app_state().fs.clone(),
            cx,
            {
                let id = id.clone();
                |settings, _| {
                    if let Some(server_config) = settings.context_servers.get_mut(&id) {
                        server_config.settings = Some(settings_value);
                    } else {
                        settings.context_servers.insert(
                            id,
                            context_server::ServerConfig {
                                settings: Some(settings_value),
                                ..Default::default()
                            },
                        );
                    }
                }
            },
        );
    }

    fn dismiss(&self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

fn wait_for_context_server(
    context_server_manager: &Entity<ContextServerManager>,
    context_server_id: Arc<str>,
    cx: &mut App,
) -> Task<Result<(), Arc<str>>> {
    let (tx, rx) = futures::channel::oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let subscription = cx.subscribe(context_server_manager, move |_, event, _cx| match event {
        context_server::manager::Event::ServerStarted { server_id } => {
            if server_id == &context_server_id {
                if let Some(tx) = tx.lock().unwrap().take() {
                    let _ = tx.send(Ok(()));
                }
            }
        }
        context_server::manager::Event::ServerFailedToStart { server_id, error } => {
            if server_id == &context_server_id {
                if let Some(tx) = tx.lock().unwrap().take() {
                    let _ = tx.send(Err(error.clone()));
                }
            }
        }
        _ => {}
    });

    cx.spawn(async move |_cx| {
        let result = rx.await.unwrap();
        drop(subscription);
        result
    })
}

impl Render for ConfigureContextServerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(configuration) = self.context_servers_to_setup.first() else {
            return div().child("No context servers to setup");
        };

        let focus_handle = self.focus_handle(cx);

        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ConfigureContextServerModal")
            .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| this.confirm(cx)))
            .on_action(cx.listener(|this, _: &menu::Cancel, _window, cx| this.dismiss(cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .child(
                Modal::new("configure-context-server", None)
                    .header(ModalHeader::new().headline(format!("Configure {}", configuration.id)))
                    .section(
                        Section::new()
                            .child(
                                Label::new(configuration.installation_instructions.clone())
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .p_2()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .bg(cx.theme().colors().editor_background)
                                    .gap_1()
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
                                            &configuration.settings_editor,
                                            EditorStyle {
                                                background: cx.theme().colors().editor_background,
                                                local_player: cx.theme().players().local(),
                                                text: text_style,
                                                syntax: cx.theme().syntax().clone(),
                                                ..Default::default()
                                            },
                                        )
                                    })
                                    .when_some(configuration.last_error.clone(), |this, error| {
                                        this.child(
                                            h_flex()
                                                .gap_1()
                                                .px_2()
                                                .py_1()
                                                .child(
                                                    Icon::new(IconName::Warning)
                                                        .size(IconSize::XSmall)
                                                        .color(Color::Warning),
                                                )
                                                .child(
                                                    Label::new(error)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                ),
                                        )
                                    }),
                            )
                            .when(configuration.waiting_for_context_server, |this| {
                                this.child(
                                    h_flex()
                                        .gap_1p5()
                                        .child(
                                            Icon::new(IconName::ArrowCircle)
                                                .size(IconSize::XSmall)
                                                .color(Color::Info)
                                                .with_animation(
                                                    "arrow-circle",
                                                    Animation::new(Duration::from_secs(2)).repeat(),
                                                    |icon, delta| {
                                                        icon.transform(Transformation::rotate(
                                                            percentage(delta),
                                                        ))
                                                    },
                                                )
                                                .into_any_element(),
                                        )
                                        .child(
                                            Label::new("Waiting for Context Server")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        ),
                                )
                            }),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_1()
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
                                            this.dismiss(cx)
                                        })),
                                )
                                .child(
                                    Button::new("configure-server", "Configure MCP")
                                        .disabled(configuration.waiting_for_context_server)
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Confirm,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        )
                                        .on_click(cx.listener(|this, _event, _window, cx| {
                                            this.confirm(cx)
                                        })),
                                ),
                        ),
                    ),
            )
    }
}

impl ModalView for ConfigureContextServerModal {}

impl EventEmitter<DismissEvent> for ConfigureContextServerModal {}
impl Focusable for ConfigureContextServerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if let Some(current) = self.context_servers_to_setup.first() {
            current.settings_editor.read(cx).focus_handle(cx)
        } else {
            cx.focus_handle()
        }
    }
}
