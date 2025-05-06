use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context as _;
use context_server::ContextServerId;
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    Animation, AnimationExt, App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task,
    TextStyle, TextStyleRefinement, Transformation, UnderlineStyle, WeakEntity, percentage,
};
use language::{Language, LanguageRegistry};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::{
    context_server_store::{ContextServerStatus, ContextServerStore},
    project_settings::{ContextServerConfiguration, ProjectSettings},
};
use settings::{Settings as _, update_settings_file};
use theme::ThemeSettings;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, Tooltip, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

pub(crate) struct ConfigureContextServerModal {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    context_servers_to_setup: Vec<ContextServerSetup>,
    context_server_store: Entity<ContextServerStore>,
}

enum Configuration {
    NotAvailable,
    Required(ConfigurationRequiredState),
}

struct ConfigurationRequiredState {
    installation_instructions: Entity<markdown::Markdown>,
    settings_validator: Option<jsonschema::Validator>,
    settings_editor: Entity<Editor>,
    last_error: Option<SharedString>,
    waiting_for_context_server: bool,
}

struct ContextServerSetup {
    id: ContextServerId,
    repository_url: Option<SharedString>,
    configuration: Configuration,
}

impl ConfigureContextServerModal {
    pub fn new(
        configurations: impl Iterator<Item = crate::context_server_configuration::Configuration>,
        context_server_store: Entity<ContextServerStore>,
        jsonc_language: Option<Arc<Language>>,
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let context_servers_to_setup = configurations
            .map(|config| match config {
                crate::context_server_configuration::Configuration::NotAvailable(
                    context_server_id,
                    repository_url,
                ) => ContextServerSetup {
                    id: context_server_id,
                    repository_url,
                    configuration: Configuration::NotAvailable,
                },
                crate::context_server_configuration::Configuration::Required(
                    context_server_id,
                    repository_url,
                    config,
                ) => {
                    let jsonc_language = jsonc_language.clone();
                    let settings_validator = jsonschema::validator_for(&config.settings_schema)
                        .context("Failed to load JSON schema for context server settings")
                        .log_err();
                    let state = ConfigurationRequiredState {
                        installation_instructions: cx.new(|cx| {
                            Markdown::new(
                                config.installation_instructions.clone().into(),
                                Some(language_registry.clone()),
                                None,
                                cx,
                            )
                        }),
                        settings_validator,
                        settings_editor: cx.new(|cx| {
                            let mut editor = Editor::auto_height(16, window, cx);
                            editor.set_text(config.default_settings.trim(), window, cx);
                            editor.set_show_gutter(false, cx);
                            editor.set_soft_wrap_mode(
                                language::language_settings::SoftWrap::None,
                                cx,
                            );
                            if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                                buffer.update(cx, |buffer, cx| {
                                    buffer.set_language(jsonc_language, cx)
                                })
                            }
                            editor
                        }),
                        waiting_for_context_server: false,
                        last_error: None,
                    };
                    ContextServerSetup {
                        id: context_server_id,
                        repository_url,
                        configuration: Configuration::Required(state),
                    }
                }
            })
            .collect::<Vec<_>>();

        Self {
            workspace,
            focus_handle: cx.focus_handle(),
            context_servers_to_setup,
            context_server_store,
        }
    }
}

impl ConfigureContextServerModal {
    pub fn confirm(&mut self, cx: &mut Context<Self>) {
        if self.context_servers_to_setup.is_empty() {
            self.dismiss(cx);
            return;
        }

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let id = self.context_servers_to_setup[0].id.clone();
        let configuration = match &mut self.context_servers_to_setup[0].configuration {
            Configuration::NotAvailable => {
                self.context_servers_to_setup.remove(0);
                if self.context_servers_to_setup.is_empty() {
                    self.dismiss(cx);
                }
                return;
            }
            Configuration::Required(state) => state,
        };

        configuration.last_error.take();
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
        let id = id.clone();

        let settings_changed = ProjectSettings::get_global(cx)
            .context_servers
            .get(&id.0)
            .map_or(true, |config| {
                config.settings.as_ref() != Some(&settings_value)
            });

        let is_running = self.context_server_store.read(cx).status_for_server(&id)
            == Some(ContextServerStatus::Running);

        if !settings_changed && is_running {
            self.complete_setup(id, cx);
            return;
        }

        configuration.waiting_for_context_server = true;

        let task = wait_for_context_server(&self.context_server_store, id.clone(), cx);
        cx.spawn({
            let id = id.clone();
            async move |this, cx| {
                let result = task.await;
                this.update(cx, |this, cx| match result {
                    Ok(_) => {
                        this.complete_setup(id, cx);
                    }
                    Err(err) => {
                        if let Some(setup) = this.context_servers_to_setup.get_mut(0) {
                            match &mut setup.configuration {
                                Configuration::NotAvailable => {}
                                Configuration::Required(state) => {
                                    state.last_error = Some(err.into());
                                    state.waiting_for_context_server = false;
                                }
                            }
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
        update_settings_file::<ProjectSettings>(workspace.read(cx).app_state().fs.clone(), cx, {
            let id = id.clone();
            |settings, _| {
                if let Some(server_config) = settings.context_servers.get_mut(&id.0) {
                    server_config.settings = Some(settings_value);
                } else {
                    settings.context_servers.insert(
                        id.0,
                        ContextServerConfiguration {
                            settings: Some(settings_value),
                            ..Default::default()
                        },
                    );
                }
            }
        });
    }

    fn complete_setup(&mut self, id: ContextServerId, cx: &mut Context<Self>) {
        self.context_servers_to_setup.remove(0);
        cx.notify();

        if !self.context_servers_to_setup.is_empty() {
            return;
        }

        self.workspace
            .update(cx, {
                |workspace, cx| {
                    let status_toast = StatusToast::new(
                        format!("{} configured successfully.", id),
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

        self.dismiss(cx);
    }

    fn dismiss(&self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
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

impl Render for ConfigureContextServerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(setup) = self.context_servers_to_setup.first() else {
            return div().into_any_element();
        };

        let focus_handle = self.focus_handle(cx);

        div()
            .elevation_3(cx)
            .w(rems(42.))
            .key_context("ConfigureContextServerModal")
            .track_focus(&focus_handle)
            .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| this.confirm(cx)))
            .on_action(cx.listener(|this, _: &menu::Cancel, _window, cx| this.dismiss(cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .child(
                Modal::new("configure-context-server", None)
                    .header(ModalHeader::new().headline(format!("Configure {}", setup.id)))
                    .section(match &setup.configuration {
                        Configuration::NotAvailable => Section::new().child(
                            Label::new(
                                "No configuration options available for this context server. Visit the Repository for any further instructions.",
                            )
                            .color(Color::Muted),
                        ),
                        Configuration::Required(configuration) => Section::new()
                            .child(div().pb_2().text_sm().child(MarkdownElement::new(
                                configuration.installation_instructions.clone(),
                                default_markdown_style(window, cx),
                            )))
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
                    })
                    .footer(
                        ModalFooter::new()
                            .when_some(setup.repository_url.clone(), |this, repository_url| {
                                this.start_slot(
                                    h_flex().w_full().child(
                                        Button::new("open-repository", "Open Repository")
                                            .icon(IconName::ArrowUpRight)
                                            .icon_color(Color::Muted)
                                            .icon_size(IconSize::XSmall)
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
                                            .on_click(move |_, _, cx| cx.open_url(&repository_url)),
                                    ),
                                )
                            })
                            .end_slot(match &setup.configuration {
                                Configuration::NotAvailable => Button::new("dismiss", "Dismiss")
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
                                        cx.listener(|this, _event, _window, cx| this.dismiss(cx)),
                                    )
                                    .into_any_element(),
                                Configuration::Required(state) => h_flex()
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
                                                this.dismiss(cx)
                                            })),
                                    )
                                    .child(
                                        Button::new("configure-server", "Configure MCP")
                                            .disabled(state.waiting_for_context_server)
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
                                    )
                                    .into_any_element(),
                            }),
                    ),
            ).into_any_element()
    }
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
        selection_background_color: cx.theme().players().local().selection,
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

impl ModalView for ConfigureContextServerModal {}
impl EventEmitter<DismissEvent> for ConfigureContextServerModal {}
impl Focusable for ConfigureContextServerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if let Some(current) = self.context_servers_to_setup.first() {
            match &current.configuration {
                Configuration::NotAvailable => self.focus_handle.clone(),
                Configuration::Required(configuration) => {
                    configuration.settings_editor.read(cx).focus_handle(cx)
                }
            }
        } else {
            self.focus_handle.clone()
        }
    }
}
