use channel::ChannelStore;
use client::{ChannelId, DevServerId};
use editor::Editor;
use gpui::{
    AppContext, ClipboardItem, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, View,
    ViewContext,
};
use ui::{prelude::*, CheckboxWithLabel, Tooltip};
use workspace::{notifications::DetachAndPromptErr, ModalView};

pub struct DevServerModal {
    mode: Mode,
    focus_handle: FocusHandle,
    channel_store: Model<ChannelStore>,
    channel_id: ChannelId,
    remote_project_name_editor: View<Editor>,
    remote_project_path_editor: View<Editor>,
    dev_server_name_editor: View<Editor>,
    selected_dev_server_id: Option<DevServerId>,
    access_token: Option<String>,
}

#[derive(PartialEq)]
enum Mode {
    CreateRemoteProject,
    CreateDevServer,
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
            mode: Mode::CreateRemoteProject,
            focus_handle: cx.focus_handle(),
            channel_store,
            channel_id,
            remote_project_name_editor: name_editor,
            remote_project_path_editor: path_editor,
            dev_server_name_editor,
            selected_dev_server_id: None,
            access_token: None,
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

        cx.spawn(|this, mut cx| async move {
            let dev_server = dev_server.await?;
            let access_token = dev_server.access_token.clone();
            if let Some(view) = this.upgrade() {
                view.update(&mut cx, move |this, _| {
                    this.access_token = Some(access_token)
                })?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        self.create_remote_project(cx);
    }

    fn render_create_remote_project(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let dev_servers = channel_store.dev_servers_for_id(self.channel_id);

        v_flex()
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                v_flex()
                    .py_0p5()
                    .px_1()
                    .child(div().px_1().py_0p5().child("Add Remote Project")),
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
                CheckboxWithLabel::new(
                    ("selected-dev-server", dev_server.id.0),
                    Label::new(dev_server.name.clone()),
                    if Some(dev_server.id) == self.selected_dev_server_id {
                        Selection::Selected
                    } else {
                        Selection::Unselected
                    },
                    cx.listener(move |this, _, cx| {
                        if this.selected_dev_server_id == Some(dev_server_id) {
                            this.selected_dev_server_id = None;
                        } else {
                            this.selected_dev_server_id = Some(dev_server_id);
                        }
                        cx.notify();
                    }),
                )
            }))
            .child(
                Button::new("toggle-create-dev-server-button", "Create dev server").on_click(
                    cx.listener(|this, _, cx| {
                        this.mode = Mode::CreateDevServer;
                        this.access_token = None;
                        this.dev_server_name_editor
                            .read(cx)
                            .focus_handle(cx)
                            .focus(cx);
                        cx.notify();
                    }),
                ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child("Path")
                    .child(self.remote_project_path_editor.clone()),
            )
    }

    fn render_create_dev_server(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                v_flex()
                    .py_0p5()
                    .px_1()
                    .child(div().px_1().py_0p5().child("Add Dev Server")),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child("Name")
                    .child(self.dev_server_name_editor.clone()),
            )
            .when_some(self.access_token.clone(), |this, access_token| {
                this.child(
                    div()
                        .child("Server created!")
                        .child("Access token: ")
                        .child(access_token.clone())
                        .child(
                            IconButton::new("copy-access-token", IconName::Copy)
                                .on_click(cx.listener(move |_, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new(access_token.clone()))
                                }))
                                .icon_size(IconSize::Small)
                                .tooltip(|cx| Tooltip::text("Copy access token", cx)),
                        ),
                )
            })
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
        let modal_content = match self.mode {
            Mode::CreateRemoteProject => self.render_create_remote_project(cx).into_any_element(),
            Mode::CreateDevServer => self.render_create_dev_server(cx).into_any_element(),
        };

        div()
            .track_focus(&self.focus_handle)
            .elevation_2(cx)
            .key_context("DevServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .w_96()
            .child(modal_content)
            .child(
                div()
                    .flex()
                    .w_full()
                    .flex_row_reverse()
                    .child(Button::new("create-button", "Create").on_click(cx.listener(
                        |this, _event, cx| match this.mode {
                            Mode::CreateRemoteProject => this.create_remote_project(cx),
                            Mode::CreateDevServer => this.create_dev_server(cx),
                        },
                    )))
                    .when(self.mode == Mode::CreateDevServer, |this| {
                        this.child(Button::new("cancel-button", "Cancel").on_click(cx.listener(
                            |this, _, cx| {
                                this.access_token = None;
                                this.dev_server_name_editor
                                    .update(cx, |editor, cx| editor.set_text("", cx));
                                this.mode = Mode::CreateRemoteProject;
                                this.remote_project_name_editor
                                    .read(cx)
                                    .focus_handle(cx)
                                    .focus(cx);
                                cx.notify();
                            },
                        )))
                    }),
            )
    }
}
