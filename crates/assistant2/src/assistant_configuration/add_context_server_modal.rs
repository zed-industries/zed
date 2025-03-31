use context_server::{ContextServerSettings, ServerCommand, ServerConfig};
use editor::Editor;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity, prelude::*};
use serde_json::json;
use settings::update_settings_file;
use ui::{Modal, ModalFooter, ModalHeader, Section, Tooltip, prelude::*};
use workspace::{ModalView, Workspace};

use crate::AddContextServer;

pub struct AddContextServerModal {
    workspace: WeakEntity<Workspace>,
    name_editor: Entity<Editor>,
    command_editor: Entity<Editor>,
}

impl AddContextServerModal {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &AddContextServer, window, cx| {
            let workspace_handle = cx.entity().downgrade();
            workspace.toggle_modal(window, cx, |window, cx| {
                Self::new(workspace_handle, window, cx)
            })
        });
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_editor = cx.new(|cx| Editor::single_line(window, cx));
        let command_editor = cx.new(|cx| Editor::single_line(window, cx));

        name_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Context server name", cx);
        });

        command_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Command to run the context server", cx);
        });

        Self {
            name_editor,
            command_editor,
            workspace,
        }
    }

    fn confirm(&mut self, cx: &mut Context<Self>) {
        let name = self.name_editor.read(cx).text(cx).trim().to_string();
        let command = self.command_editor.read(cx).text(cx).trim().to_string();

        if name.is_empty() || command.is_empty() {
            return;
        }

        let mut command_parts = command.split(' ').map(|part| part.trim().to_string());
        let Some(path) = command_parts.next() else {
            return;
        };
        let args = command_parts.collect::<Vec<_>>();

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let fs = workspace.app_state().fs.clone();
                update_settings_file::<ContextServerSettings>(fs.clone(), cx, |settings, _| {
                    settings.context_servers.insert(
                        name.into(),
                        ServerConfig {
                            command: Some(ServerCommand {
                                path,
                                args,
                                env: None,
                            }),
                            settings: Some(json!({})),
                        },
                    );
                });
            });
        }

        cx.emit(DismissEvent);
    }

    fn cancel(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl ModalView for AddContextServerModal {}

impl Focusable for AddContextServerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.name_editor.focus_handle(cx).clone()
    }
}

impl EventEmitter<DismissEvent> for AddContextServerModal {}

impl Render for AddContextServerModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_name_empty = self.name_editor.read(cx).text(cx).trim().is_empty();
        let is_command_empty = self.command_editor.read(cx).text(cx).trim().is_empty();

        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("AddContextServerModal")
            .on_action(cx.listener(|this, _: &menu::Cancel, _window, cx| this.cancel(cx)))
            .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| this.confirm(cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .on_mouse_down_out(cx.listener(|_this, _, _, cx| cx.emit(DismissEvent)))
            .child(
                Modal::new("add-context-server", None)
                    .header(ModalHeader::new().headline("Add Context Server"))
                    .section(
                        Section::new()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(Label::new("Name"))
                                    .child(self.name_editor.clone()),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(Label::new("Command"))
                                    .child(self.command_editor.clone()),
                            ),
                    )
                    .footer(
                        ModalFooter::new()
                            .start_slot(
                                Button::new("cancel", "Cancel").on_click(
                                    cx.listener(|this, _event, _window, cx| this.cancel(cx)),
                                ),
                            )
                            .end_slot(
                                Button::new("add-server", "Add Server")
                                    .disabled(is_name_empty || is_command_empty)
                                    .map(|button| {
                                        if is_name_empty {
                                            button.tooltip(Tooltip::text("Name is required"))
                                        } else if is_command_empty {
                                            button.tooltip(Tooltip::text("Command is required"))
                                        } else {
                                            button
                                        }
                                    })
                                    .on_click(
                                        cx.listener(|this, _event, _window, cx| this.confirm(cx)),
                                    ),
                            ),
                    ),
            )
    }
}
