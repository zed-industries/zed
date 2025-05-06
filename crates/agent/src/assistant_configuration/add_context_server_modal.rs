use context_server::ContextServerCommand;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity, prelude::*};
use project::project_settings::{ContextServerConfiguration, ProjectSettings};
use serde_json::json;
use settings::update_settings_file;
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, Tooltip, prelude::*};
use ui_input::SingleLineInput;
use workspace::{ModalView, Workspace};

use crate::AddContextServer;

pub struct AddContextServerModal {
    workspace: WeakEntity<Workspace>,
    name_editor: Entity<SingleLineInput>,
    command_editor: Entity<SingleLineInput>,
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
        let name_editor =
            cx.new(|cx| SingleLineInput::new(window, cx, "my-custom-server").label("Name"));
        let command_editor = cx.new(|cx| {
            SingleLineInput::new(window, cx, "Command").label("Command to run the MCP server")
        });

        Self {
            name_editor,
            command_editor,
            workspace,
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut Context<Self>) {
        let name = self
            .name_editor
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let command = self
            .command_editor
            .read(cx)
            .editor()
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

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
                update_settings_file::<ProjectSettings>(fs.clone(), cx, |settings, _| {
                    settings.context_servers.insert(
                        name.into(),
                        ContextServerConfiguration {
                            command: Some(ContextServerCommand {
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

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut Context<Self>) {
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_name_empty = self.name_editor.read(cx).is_empty(cx);
        let is_command_empty = self.command_editor.read(cx).is_empty(cx);

        let focus_handle = self.focus_handle(cx);

        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("AddContextServerModal")
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
            .on_mouse_down_out(cx.listener(|_this, _, _, cx| cx.emit(DismissEvent)))
            .child(
                Modal::new("add-context-server", None)
                    .header(ModalHeader::new().headline("Add MCP Server"))
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_2()
                                .child(self.name_editor.clone())
                                .child(self.command_editor.clone()),
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
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.cancel(&menu::Cancel, cx)
                                    })),
                            )
                            .end_slot(
                                Button::new("add-server", "Add Server")
                                    .disabled(is_name_empty || is_command_empty)
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &menu::Confirm,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                    )
                                    .map(|button| {
                                        if is_name_empty {
                                            button.tooltip(Tooltip::text("Name is required"))
                                        } else if is_command_empty {
                                            button.tooltip(Tooltip::text("Command is required"))
                                        } else {
                                            button
                                        }
                                    })
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.confirm(&menu::Confirm, cx)
                                    })),
                            ),
                    ),
            )
    }
}
