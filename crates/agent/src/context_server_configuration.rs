use std::sync::Arc;

use anyhow::Context as _;
use context_server::ContextServerDescriptorRegistry;
use editor::{Editor, EditorElement, EditorStyle};
use extension::ContextServerConfiguration;
use gpui::{
    App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, TextStyle, WeakEntity,
};
use language::{Language, LanguageRegistry};
use settings::{Settings as _, update_settings_file};
use theme::ThemeSettings;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

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
                        let registry = ContextServerDescriptorRegistry::global(cx).read(cx);
                        let project = workspace.project().clone();
                        let configuration_tasks =
                            manifest
                                .context_servers
                                .keys()
                                .cloned()
                                .filter_map({
                                    |key| {
                                        let descriptor= registry.context_server_descriptor(&key)?;
                                        Some(cx.spawn({
                                            let project = project.clone();
                                            async move |_, cx| {
                                                descriptor.configuration(project, &cx)
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
                                    descriptors.into_iter().filter_map(|descriptor| descriptor),
                                    jsonc_language,
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

struct ConfigureContextServer {
    id: Arc<str>,
    installation_instructions: SharedString,
    settings_validator: Option<jsonschema::Validator>,
    settings_editor: Entity<Editor>,
    last_error: Option<SharedString>,
}

struct ConfigureContextServerModal {
    workspace: WeakEntity<Workspace>,
    context_servers_to_setup: Vec<ConfigureContextServer>,
}

impl ConfigureContextServerModal {
    pub fn from_configurations(
        configurations: impl Iterator<Item = (Arc<str>, ContextServerConfiguration)>,
        jsonc_language: Option<Arc<Language>>,
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
        })
    }
}

impl ConfigureContextServerModal {
    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut Context<Self>) {
        if self.context_servers_to_setup.is_empty() {
            return;
        }

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let configuration = &mut self.context_servers_to_setup[0];
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

        self.context_servers_to_setup.remove(0);

        update_settings_file::<context_server::ContextServerSettings>(
            workspace.read(cx).app_state().fs.clone(),
            cx,
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
            },
        );
    }
}

impl ModalView for ConfigureContextServerModal {}

impl Focusable for ConfigureContextServerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if let Some(current) = self.context_servers_to_setup.first() {
            current.settings_editor.read(cx).focus_handle(cx)
        } else {
            cx.focus_handle()
        }
    }
}

impl EventEmitter<DismissEvent> for ConfigureContextServerModal {}

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
            .on_action(cx.listener(|_, _: &menu::Cancel, _window, cx| cx.emit(DismissEvent)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .on_mouse_down_out(cx.listener(|_this, _, _, cx| cx.emit(DismissEvent)))
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
                                            div()
                                                .bg(cx.theme().colors().editor_background)
                                                .rounded_md()
                                                .child(
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
                                                ),
                                        )
                                    }),
                            ),
                    )
                    .footer(
                        ModalFooter::new()
                            .start_slot(
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
                                    .on_click(
                                        cx.listener(|_, _event, _window, cx| cx.emit(DismissEvent)),
                                    ),
                            )
                            .end_slot(
                                Button::new("configure-server", "Configure MCP")
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
                                        this.confirm(&menu::Confirm, cx)
                                    })),
                            ),
                    ),
            )
    }
}
