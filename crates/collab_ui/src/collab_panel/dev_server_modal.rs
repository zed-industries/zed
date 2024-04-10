use channel::{ChannelStore, DevServer, RemoteProject};
use client::{ChannelId, DevServerId};
use editor::Editor;
use gpui::{
    AppContext, ClipboardItem, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, Task,
    View, ViewContext,
};
use rpc::proto::{CreateDevServerResponse, DevServerStatus};
use ui::{prelude::*, Indicator, Tooltip};
use util::ResultExt;
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

#[derive(Default)]
struct CreateDevServer {
    name: Option<String>,
    creating: Option<Task<()>>,
    dev_server: Option<CreateDevServerResponse>,
}

enum Mode {
    Default,
    CreateRemoteProject(DevServerId),
    CreateDevServer(CreateDevServer),
}

/*
 ┌────────────────────────────────────────────────────────┐
 │  Manage Remote Projects                             X  │
 │                                                        │
 │  * dev-server-1 [online]                               │
 │                                                        │
 │      * zed : /Users/eg/code/zed/zed  [DELETE]          │
 │      * [NEW PROJECT]                                   │
 │                                                        │
 │  * dev-server-2 [offline]                              │
 │                                                        │
 │      * treesitter: /Users/maxbrunsfeld/code/treesitter │
 │      * docs: /Users/conrad/code/docs                   │
 │      * [new project]                                   │
 │                                                        │
 │  * [REGISTER DEV SERVER]                               │
 │                                                        │
 └────────────────────────────────────────────────────────┘


  ┌───────────────────────────────────────┐
  │  Register Dev Server:               X │
  │                                       │
  │  Name: [_______]  [SUBMIT]            │
  │                                       │
  │                                       │
  │                                       │
  │                                       │
  │                                       │
  │                                       │
  │                                       │
  └───────────────────────────────────────┘

  ┌───────────────────────────────────────┐
  │  Register Dev Server:               X │
  │                                       │
  │  Name: [_zeus__]                      │
  │                                       │
  │  Log into your server and run:        │
  │                                       │
  │    zed --dev-server-token  XXXXXXXXX  │
  │                                       │
  │   -- waiting for connection... --     │
  │                                       │
  └───────────────────────────────────────┘

  ┌───────────────────────────────────────┐
  │  Register Dev Server:               X │
  │                                       │
  │  Name: [_zeus__]                      │
  │                                       │
  │  Log into your server and run:        │
  │                                       │
  │    zed --dev-server-token  XXXXXXXXX  │
  │                                       │
  │   Connected! :)          [CLOSE]      │
  │                                       │
  └───────────────────────────────────────┘

┌───────────────────────────────────────┐
│  Add project on `zeus`:             X │
│                                       │
│  Path: [_______]                      │
│                                       │
│  Name: [_______]                      │
│                                       │
│                               [SUBMIT]│
└───────────────────────────────────────┘

*/

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
            mode: Mode::Default,
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

    pub fn create_dev_server(&mut self, cx: &mut ViewContext<Self>) {
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
            store.create_dev_server(self.channel_id, name.clone(), cx)
        });

        let task = cx.spawn(|this, mut cx| async move {
            match dev_server.await {
                Ok(dev_server) => {
                    this.update(&mut cx, |this, _| {
                        this.mode = Mode::CreateDevServer(CreateDevServer {
                            name: Some(dev_server.name.clone()),
                            creating: None,
                            dev_server: Some(dev_server),
                        });
                    })
                    .log_err();
                }
                Err(e) => {
                    cx.prompt(
                        gpui::PromptLevel::Critical,
                        "Failed to create server",
                        Some(&format!("{:?}. Please try again.", e)),
                        &["Ok"],
                    )
                    .await
                    .log_err();
                    this.update(&mut cx, |this, _| {
                        this.mode = Mode::CreateDevServer(Default::default());
                    })
                    .log_err();
                }
            }
        });

        self.mode = Mode::CreateDevServer(CreateDevServer {
            name: Some(name),
            creating: Some(task),
            dev_server: None,
        });
        cx.notify()
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        self.create_remote_project(cx);
    }

    fn render_dev_server(
        &mut self,
        dev_server: &DevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let dev_server_id = dev_server.id;

        v_flex()
            .ml_5()
            .child(
                h_flex()
                    .gap_2()
                    .child(Icon::new(IconName::Server))
                    .child(dev_server.name.clone())
                    .child(Indicator::dot().color(match dev_server.status {
                        DevServerStatus::Online => Color::Created,
                        DevServerStatus::Offline => Color::Deleted,
                    })),
            )
            .children(
                channel_store
                    .remote_projects_for_id(dev_server.channel_id)
                    .iter()
                    .filter_map(|remote_project| {
                        if remote_project.dev_server_id == dev_server.id {
                            Some(self.render_remote_project(remote_project, cx))
                        } else {
                            None
                        }
                    }),
            )
            .child(div().ml_8().child(
                Button::new(("add-project", dev_server_id.0), "Add Project").on_click(cx.listener(
                    move |this, _, cx| {
                        this.mode = Mode::CreateRemoteProject(dev_server_id);
                        cx.notify();
                    },
                )),
            ))
    }

    fn render_remote_project(
        &mut self,
        project: &RemoteProject,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        h_flex()
            .ml_8()
            .gap_2()
            .child(Icon::new(IconName::FileTree))
            .child(project.name.clone())
            .child(project.path.clone())
            .child("DELETE")
    }

    fn render_create_dev_server(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Mode::CreateDevServer(CreateDevServer {
            name,
            creating: _,
            dev_server,
        }) = &self.mode
        else {
            unreachable!()
        };

        self.dev_server_name_editor
            .update(cx, |editor, _| editor.set_read_only(name.is_some()));
        v_flex()
            .on_action(cx.listener(|this, _: &menu::Confirm, cx| this.create_dev_server(cx)))
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                v_flex()
                    .py_0p5()
                    .px_1()
                    .child(div().px_1().py_0p5().child("Manage Remote Projects")),
            )
            .child(
                h_flex()
                    .py_0p5()
                    .px_1()
                    .child(div().px_1().py_0p5().child(Icon::new(IconName::ArrowLeft)))
                    .child("Add Dev Server"),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child("Name")
                    .child(self.dev_server_name_editor.clone())
                    .when(name.is_none(), |div| {
                        div.child(
                            Button::new("create-dev-server", "Create").on_click(cx.listener(
                                move |this, _, cx| {
                                    this.create_dev_server(cx);
                                },
                            )),
                        )
                    })
                    .when(name.is_some() && dev_server.is_none(), |div| {
                        div.child(Button::new("create-dev-server", "Creating...").disabled(true))
                    }),
            )
            .when_some(dev_server.clone(), |this, dev_server| {
                let instructions = SharedString::from(format!(
                    "zed --dev-server-token {}",
                    dev_server.access_token
                ));
                this.child(
                    v_flex()
                        .ml_8()
                        .gap_2()
                        .child(Label::new(format!(
                            "Please log into `{}` and run:",
                            dev_server.name
                        )))
                        .child(instructions.clone())
                        .child(
                            IconButton::new("copy-access-token", IconName::Copy)
                                .on_click(cx.listener(move |_, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new(
                                        instructions.to_string(),
                                    ))
                                }))
                                .icon_size(IconSize::Small)
                                .tooltip(|cx| Tooltip::text("Copy access token", cx)),
                        ),
                )
            })
    }

    fn render_default(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                    .child(div().px_1().py_0p5().child("Manage Remote Projects")),
            )
            .children(
                dev_servers
                    .iter()
                    .map(|dev_server| self.render_dev_server(dev_server, cx)),
            )
            .child(
                Button::new("toggle-create-dev-server-button", "Create dev server").on_click(
                    cx.listener(|this, _, cx| {
                        this.mode = Mode::CreateDevServer(Default::default());
                        this.access_token = None;
                        this.dev_server_name_editor
                            .read(cx)
                            .focus_handle(cx)
                            .focus(cx);
                        cx.notify();
                    }),
                ),
            )
    }

    fn render_create_project(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
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
        div()
            .track_focus(&self.focus_handle)
            .elevation_2(cx)
            .key_context("DevServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .w(rems(44.))
            .h(rems(40.))
            .child(match &self.mode {
                Mode::Default => self.render_default(cx).into_any_element(),
                Mode::CreateRemoteProject(_) => self.render_create_project(cx).into_any_element(),
                Mode::CreateDevServer(_) => self.render_create_dev_server(cx).into_any_element(),
            })
    }
}
