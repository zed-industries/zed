use channel::ChannelStore;
use client::{ChannelId, DevServerId};
use editor::Editor;
use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, View, ViewContext,
};
use ui::{prelude::*, Checkbox};
use workspace::{notifications::DetachAndPromptErr, ModalView};

pub struct DevServerModal {
    focus_handle: FocusHandle,
    channel_store: Model<ChannelStore>,
    channel_id: ChannelId,
    remote_project_name_editor: View<Editor>,
    remote_project_path_editor: View<Editor>,
    dev_server_name_editor: View<Editor>,
    selected_dev_server_id: Option<DevServerId>,
    creating_dev_server: bool,
}

impl DevServerModal {
    pub fn new(
        channel_store: Model<ChannelStore>,
        channel_id: ChannelId,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let name_editor = cx.new_view(|cx| Editor::single_line(cx));
        let path_editor = cx.new_view(|cx| Editor::single_line(cx));
        let dev_server_name_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Dev server name", cx);
            editor
        });

        Self {
            focus_handle: cx.focus_handle(),
            channel_store,
            channel_id,
            remote_project_name_editor: name_editor,
            remote_project_path_editor: path_editor,
            dev_server_name_editor,
            selected_dev_server_id: None,
            creating_dev_server: false,
        }
    }

    pub fn create_remote_project(&self, cx: &mut ViewContext<Self>) {
        let channel_id = self.channel_id;
        let name = self
            .remote_project_name_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let path = self
            .remote_project_path_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

        let Some(dev_server_id) = self.selected_dev_server_id else {
            return;
        };

        if name == "" {
            return;
        }
        if path == "" {
            return;
        }

        let task = self.channel_store.update(cx, |store, cx| {
            store.create_remote_project(channel_id, dev_server_id, name, path, cx)
        });

        task.detach_and_prompt_err("Failed to create remote project", cx, |_, _| None);
    }

    pub fn create_dev_server(&self, cx: &mut ViewContext<Self>) {
        let name = self
            .dev_server_name_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();

        if name == "" {
            return;
        }

        let dev_server = self.channel_store.update(cx, |store, cx| {
            store.create_dev_server(self.channel_id, name, cx)
        });

        cx.spawn(|_, _| async move {
            let dev_server = dev_server.await?;
            dbg!(dev_server.access_token, dev_server.name);
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }
}
impl ModalView for DevServerModal {}

impl FocusableView for DevServerModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for DevServerModal {}

impl Render for DevServerModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let dev_servers = channel_store.dev_servers_for_id(self.channel_id);
        div()
            .track_focus(&self.focus_handle)
            .elevation_2(cx)
            .key_context("DevServerModal")
            // .on_action(cx.listener(Self::cancel))
            // .on_action(cx.listener(Self::confirm))
            .w_96()
            .child(
                v_flex()
                    .px_1()
                    .pt_0p5()
                    .gap_px()
                    .child(
                        v_flex()
                            .py_0p5()
                            .px_1()
                            .child(div().px_1().py_0p5().child("Add Remote Project:")),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child("Name")
                            .child(self.remote_project_name_editor.clone()),
                    )
                    .child("Dev Server")
                    .children(dev_servers.iter().map(|dev_server| {
                        let dev_server_id = dev_server.id;
                        h_flex()
                            .gap_2()
                            .child(Checkbox::new(
                                ("selected-dev-server", dev_server.id.0),
                                if Some(dev_server.id) == self.selected_dev_server_id {
                                    Selection::Selected
                                } else {
                                    Selection::Unselected
                                },
                            ))
                            .child(dev_server.name.clone())
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |this, _, cx| {
                                    this.selected_dev_server_id = Some(dev_server_id);
                                    cx.notify();
                                }),
                            )
                    }))
                    .when(!self.creating_dev_server, |container| {
                        container.child(
                            Button::new("toggle-create-dev-server-button", "Create dev server")
                                .on_click(cx.listener(|this, _, cx| {
                                    this.creating_dev_server = true;
                                    this.dev_server_name_editor
                                        .read(cx)
                                        .focus_handle(cx)
                                        .focus(cx);
                                })),
                        )
                    })
                    .when(self.creating_dev_server, |container| {
                        container.child(
                            div()
                                .flex()
                                .flex_row()
                                .w_full()
                                .gap_2()
                                .child(self.dev_server_name_editor.clone())
                                .child(Button::new("create-dev-server-button", "Create").on_click(
                                    cx.listener(|this, _, cx| {
                                        this.create_dev_server(cx);
                                        this.creating_dev_server = false;
                                        this.dev_server_name_editor
                                            .update(cx, |editor, cx| editor.set_text("", cx));
                                    }),
                                ))
                                .child(
                                    Button::new("cancel-create-dev-server-button", "Cancel")
                                        .on_click(cx.listener(|this, _, cx| {
                                            this.creating_dev_server = false;
                                            this.dev_server_name_editor
                                                .update(cx, |editor, cx| editor.set_text("", cx))
                                        })),
                                ),
                        )
                    })
                    .child(
                        h_flex()
                            .gap_2()
                            .child("Path")
                            .child(self.remote_project_path_editor.clone()),
                    )
                    .child(
                        Button::new("create-remote-project-button", "Create remote project")
                            .on_click(
                                cx.listener(|this, _event, cx| this.create_remote_project(cx)),
                            ),
                    ),
            )
    }
}
