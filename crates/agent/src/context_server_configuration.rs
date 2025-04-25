use std::sync::Arc;

use editor::Editor;
use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity};
use settings::update_settings_file;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, prelude::*};
use workspace::{ModalView, Workspace};

pub(crate) fn init(cx: &mut App) {
    cx.observe_new(|_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        if let Some(extension_events) = extension::ExtensionEvents::try_global(cx).as_ref() {
            cx.subscribe_in(
                extension_events,
                window,
                |workspace, _, event, window, cx| match event {
                    extension::Event::ExtensionInstalled(manifest) => {
                        let context_servers_to_setup = manifest
                            .context_servers
                            .iter()
                            .filter_map(|(id, manifest)| {
                                Some(ContextServerConfiguration {
                                    id: id.clone(),
                                    installation_instructions: manifest
                                        .installation_instructions
                                        .clone()?
                                        .into(),
                                    settings_hint: manifest.settings_hint.clone()?.into(),
                                })
                            })
                            .collect::<Vec<_>>();

                        if !context_servers_to_setup.is_empty() {
                            workspace.toggle_modal(window, cx, |_, cx| {
                                ConfigureContextServerModal {
                                    context_servers_to_setup,
                                    focus_handle: cx.focus_handle(),
                                }
                            });
                        }
                    }
                    _ => {}
                },
            )
            .detach();
        } else {
            log::info!(
                "No extension events global found. Skipping context server configuration wizard"
            );
        }
    })
    .detach();
}

struct ContextServerConfiguration {
    id: Arc<str>,
    installation_instructions: SharedString,
    settings_hint: SharedString,

    settings_editor: Entity<Editor>,
}

struct ConfigureContextServerModal {
    context_servers_to_setup: Vec<ContextServerConfiguration>,
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
}

impl ConfigureContextServerModal {
    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut Context<Self>) {
        if self.context_servers_to_setup.is_empty() {
            return;
        }

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let configuration = self.context_servers_to_setup.remove(0);
        let settings = configuration.settings_editor.read(cx).text(cx);

        update_settings_file::<context_server::ContextServerSettings>(
            workspace.read(cx).app_state().fs.clone(),
            cx,
            |settings, cx| {
                settings.context_servers.insert(configuration.)
            },
        );
    }
}

impl Render for ConfigureContextServerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(current) = self.context_servers_to_setup.first() else {
            return div().child("No context servers to setup");
        };

        let focus_handle = self.focus_handle.clone();

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
                    .header(ModalHeader::new().headline(format!("Configure {}", current.id)))
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_2()
                                .child(current.installation_instructions.clone())
                                .child(current.settings_hint.clone()),
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
                                Button::new("configure-server", "Configure")
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

impl ModalView for ConfigureContextServerModal {}
impl EventEmitter<DismissEvent> for ConfigureContextServerModal {}
impl Focusable for ConfigureContextServerModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
