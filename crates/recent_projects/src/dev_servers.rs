use std::time::Duration;

use anyhow::anyhow;
use anyhow::Context;
use dev_server_projects::{DevServer, DevServerId, DevServerProject, DevServerProjectId};
use editor::Editor;
use feature_flags::FeatureFlagAppExt;
use feature_flags::FeatureFlagViewExt;
use gpui::AsyncWindowContext;
use gpui::Subscription;
use gpui::Task;
use gpui::WeakView;
use gpui::{
    percentage, Animation, AnimationExt, AnyElement, AppContext, DismissEvent, EventEmitter,
    FocusHandle, FocusableView, Model, ScrollHandle, Transformation, View, ViewContext,
};
use markdown::Markdown;
use markdown::MarkdownStyle;
use rpc::proto::RegenerateDevServerTokenResponse;
use rpc::{
    proto::{CreateDevServerResponse, DevServerStatus},
    ErrorCode, ErrorExt,
};
use task::RevealStrategy;
use task::SpawnInTerminal;
use task::TerminalWorkDir;
use terminal_view::terminal_panel::TerminalPanel;
use ui::ElevationIndex;
use ui::Section;
use ui::{
    prelude::*, Indicator, List, ListHeader, ListItem, Modal, ModalFooter, ModalHeader,
    RadioWithLabel, Tooltip,
};
use ui_text_field::{FieldLabelLayout, TextField};
use util::ResultExt;
use workspace::{notifications::DetachAndPromptErr, AppState, ModalView, Workspace, WORKSPACE_DB};

use crate::OpenRemote;

pub struct DevServerProjects {
    mode: Mode,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    dev_server_store: Model<dev_server_projects::Store>,
    workspace: WeakView<Workspace>,
    project_path_input: View<Editor>,
    dev_server_name_input: View<TextField>,
    markdown: View<Markdown>,
    _dev_server_subscription: Subscription,
}

#[derive(Default)]
struct CreateDevServer {
    creating: Option<Task<()>>,
    dev_server_id: Option<DevServerId>,
    access_token: Option<String>,
    manual_setup: bool,
}

struct CreateDevServerProject {
    dev_server_id: DevServerId,
    creating: bool,
    _opening: Option<Subscription>,
}

enum Mode {
    Default(Option<CreateDevServerProject>),
    CreateDevServer(CreateDevServer),
}

impl DevServerProjects {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        cx.observe_flag::<feature_flags::Remoting, _>(|enabled, workspace, _| {
            if enabled {
                Self::register_open_remote_action(workspace);
            }
        })
        .detach();

        if cx.has_flag::<feature_flags::Remoting>() {
            Self::register_open_remote_action(workspace);
        }
    }

    fn register_open_remote_action(workspace: &mut Workspace) {
        workspace.register_action(|workspace, _: &OpenRemote, cx| {
            let handle = cx.view().downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle))
        });
    }

    pub fn open(workspace: View<Workspace>, cx: &mut WindowContext) {
        workspace.update(cx, |workspace, cx| {
            let handle = cx.view().downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle))
        })
    }

    pub fn new(cx: &mut ViewContext<Self>, workspace: WeakView<Workspace>) -> Self {
        let project_path_input = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Project path (~/work/zed, /workspace/zed, …)", cx);
            editor
        });
        let dev_server_name_input = cx.new_view(|cx| {
            TextField::new(cx, "Name", "192.168.0.1").with_label(FieldLabelLayout::Hidden)
        });

        let focus_handle = cx.focus_handle();
        let dev_server_store = dev_server_projects::Store::global(cx);

        let subscription = cx.observe(&dev_server_store, |_, _, cx| {
            cx.notify();
        });

        let markdown_style = MarkdownStyle {
            code_block: gpui::TextStyleRefinement {
                font_family: Some("Zed Mono".into()),
                color: Some(cx.theme().colors().editor_foreground),
                background_color: Some(cx.theme().colors().editor_background),
                ..Default::default()
            },
            inline_code: Default::default(),
            block_quote: Default::default(),
            link: gpui::TextStyleRefinement {
                color: Some(Color::Accent.color(cx)),
                ..Default::default()
            },
            rule_color: Default::default(),
            block_quote_border_color: Default::default(),
            syntax: cx.theme().syntax().clone(),
            selection_background_color: cx.theme().players().local().selection,
        };
        let markdown = cx.new_view(|cx| Markdown::new("".to_string(), markdown_style, None, cx));

        Self {
            mode: Mode::Default(None),
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            dev_server_store,
            project_path_input,
            dev_server_name_input,
            markdown,
            workspace,
            _dev_server_subscription: subscription,
        }
    }

    pub fn create_dev_server_project(
        &mut self,
        dev_server_id: DevServerId,
        cx: &mut ViewContext<Self>,
    ) {
        let mut path = self.project_path_input.read(cx).text(cx).trim().to_string();

        if path == "" {
            return;
        }

        if !path.starts_with('/') && !path.starts_with('~') {
            path = format!("~/{}", path);
        }

        if self
            .dev_server_store
            .read(cx)
            .projects_for_server(dev_server_id)
            .iter()
            .any(|p| p.path == path)
        {
            cx.spawn(|_, mut cx| async move {
                cx.prompt(
                    gpui::PromptLevel::Critical,
                    "Failed to create project",
                    Some(&format!(
                        "Project {} already exists for this dev server.",
                        path
                    )),
                    &["Ok"],
                )
                .await
            })
            .detach_and_log_err(cx);
            return;
        }

        let create = {
            let path = path.clone();
            self.dev_server_store.update(cx, |store, cx| {
                store.create_dev_server_project(dev_server_id, path, cx)
            })
        };

        cx.spawn(|this, mut cx| async move {
            let result = create.await;
            this.update(&mut cx, |this, cx| {
                if let Ok(result) = &result {
                    if let Some(dev_server_project_id) =
                        result.dev_server_project.as_ref().map(|p| p.id)
                    {
                        let subscription =
                            cx.observe(&this.dev_server_store, move |this, store, cx| {
                                if let Some(project_id) = store
                                    .read(cx)
                                    .dev_server_project(DevServerProjectId(dev_server_project_id))
                                    .and_then(|p| p.project_id)
                                {
                                    this.project_path_input.update(cx, |editor, cx| {
                                        editor.set_text("", cx);
                                    });
                                    this.mode = Mode::Default(None);
                                    if let Some(app_state) = AppState::global(cx).upgrade() {
                                        workspace::join_dev_server_project(
                                            project_id, app_state, None, cx,
                                        )
                                        .detach_and_prompt_err(
                                            "Could not join project",
                                            cx,
                                            |_, _| None,
                                        )
                                    }
                                }
                            });

                        this.mode = Mode::Default(Some(CreateDevServerProject {
                            dev_server_id,
                            creating: true,
                            _opening: Some(subscription),
                        }));
                    }
                } else {
                    this.mode = Mode::Default(Some(CreateDevServerProject {
                        dev_server_id,
                        creating: false,
                        _opening: None,
                    }));
                }
            })
            .log_err();
            result
        })
        .detach_and_prompt_err("Failed to create project", cx, move |e, _| {
            match e.error_code() {
                ErrorCode::DevServerOffline => Some(
                    "The dev server is offline. Please log in and check it is connected."
                        .to_string(),
                ),
                ErrorCode::DevServerProjectPathDoesNotExist => {
                    Some(format!("The path `{}` does not exist on the server.", path))
                }
                _ => None,
            }
        });

        self.mode = Mode::Default(Some(CreateDevServerProject {
            dev_server_id,
            creating: true,
            _opening: None,
        }));
    }

    pub fn create_or_update_dev_server(
        &mut self,
        manual_setup: bool,
        existing_id: Option<DevServerId>,
        access_token: Option<String>,
        cx: &mut ViewContext<Self>,
    ) {
        let name = get_text(&self.dev_server_name_input, cx);
        if name.is_empty() {
            return;
        }

        let ssh_connection_string = if manual_setup {
            None
        } else if name.contains(' ') {
            Some(name.clone())
        } else {
            Some(format!("ssh {}", name))
        };

        let dev_server = self.dev_server_store.update(cx, {
            let access_token = access_token.clone();
            |store, cx| {
                let ssh_connection_string = ssh_connection_string.clone();
                if let Some(dev_server_id) = existing_id {
                    let rename = store.rename_dev_server(
                        dev_server_id,
                        name.clone(),
                        ssh_connection_string,
                        cx,
                    );
                    let token = if let Some(access_token) = access_token {
                        Task::ready(Ok(RegenerateDevServerTokenResponse {
                            dev_server_id: dev_server_id.0,
                            access_token,
                        }))
                    } else {
                        store.regenerate_dev_server_token(dev_server_id, cx)
                    };
                    cx.spawn(|_, _| async move {
                        rename.await?;
                        let response = token.await?;
                        Ok(CreateDevServerResponse {
                            dev_server_id: dev_server_id.0,
                            name,
                            access_token: response.access_token,
                        })
                    })
                } else {
                    store.create_dev_server(name, ssh_connection_string.clone(), cx)
                }
            }
        });

        let workspace = self.workspace.clone();
        let store = dev_server_projects::Store::global(cx);

        let task = cx
            .spawn({
                |this, mut cx| async move {
                    let result = dev_server.await;

                    match result {
                        Ok(dev_server) => {
                            if let Some(ssh_connection_string) = ssh_connection_string {
                                this.update(&mut cx, |this, cx| {
                                    if let Mode::CreateDevServer(CreateDevServer {
                                        access_token,
                                        dev_server_id,
                                        ..
                                    }) = &mut this.mode
                                    {
                                        access_token.replace(dev_server.access_token.clone());
                                        dev_server_id
                                            .replace(DevServerId(dev_server.dev_server_id));
                                    }
                                    cx.notify();
                                })?;

                                spawn_ssh_task(
                                    workspace
                                        .upgrade()
                                        .ok_or_else(|| anyhow!("workspace dropped"))?,
                                    store,
                                    DevServerId(dev_server.dev_server_id),
                                    ssh_connection_string,
                                    dev_server.access_token.clone(),
                                    &mut cx,
                                )
                                .await
                                .log_err();
                            }

                            this.update(&mut cx, |this, cx| {
                                this.focus_handle.focus(cx);
                                this.mode = Mode::CreateDevServer(CreateDevServer {
                                    creating: None,
                                    dev_server_id: Some(DevServerId(dev_server.dev_server_id)),
                                    access_token: Some(dev_server.access_token),
                                    manual_setup,
                                });
                                cx.notify();
                            })?;
                            Ok(())
                        }
                        Err(e) => {
                            this.update(&mut cx, |this, cx| {
                                this.mode = Mode::CreateDevServer(CreateDevServer {
                                    creating: None,
                                    dev_server_id: existing_id,
                                    access_token: None,
                                    manual_setup,
                                });
                                cx.notify()
                            })
                            .log_err();

                            return Err(e);
                        }
                    }
                }
            })
            .prompt_err("Failed to create server", cx, |_, _| None);

        self.mode = Mode::CreateDevServer(CreateDevServer {
            creating: Some(task),
            dev_server_id: existing_id,
            access_token,
            manual_setup,
        });
        cx.notify()
    }

    fn delete_dev_server(&mut self, id: DevServerId, cx: &mut ViewContext<Self>) {
        let store = self.dev_server_store.read(cx);
        let prompt = if store.projects_for_server(id).is_empty()
            && store
                .dev_server(id)
                .is_some_and(|server| server.status == DevServerStatus::Offline)
        {
            None
        } else {
            Some(cx.prompt(
                gpui::PromptLevel::Warning,
                "Are you sure?",
                Some("This will delete the dev server and all of its remote projects."),
                &["Delete", "Cancel"],
            ))
        };

        cx.spawn(|this, mut cx| async move {
            if let Some(prompt) = prompt {
                if prompt.await? != 0 {
                    return Ok(());
                }
            }

            let project_ids: Vec<DevServerProjectId> = this.update(&mut cx, |this, cx| {
                this.dev_server_store.update(cx, |store, _| {
                    store
                        .projects_for_server(id)
                        .into_iter()
                        .map(|project| project.id)
                        .collect()
                })
            })?;

            this.update(&mut cx, |this, cx| {
                this.dev_server_store
                    .update(cx, |store, cx| store.delete_dev_server(id, cx))
            })?
            .await?;

            for id in project_ids {
                WORKSPACE_DB
                    .delete_workspace_by_dev_server_project_id(id)
                    .await
                    .log_err();
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to delete dev server", cx, |_, _| None);
    }

    fn delete_dev_server_project(
        &mut self,
        id: DevServerProjectId,
        path: &str,
        cx: &mut ViewContext<Self>,
    ) {
        let answer = cx.prompt(
            gpui::PromptLevel::Warning,
            format!("Delete \"{}\"?", path).as_str(),
            Some("This will delete the remote project. You can always re-add it later."),
            &["Delete", "Cancel"],
        );

        cx.spawn(|this, mut cx| async move {
            let answer = answer.await?;

            if answer != 0 {
                return Ok(());
            }

            this.update(&mut cx, |this, cx| {
                this.dev_server_store
                    .update(cx, |store, cx| store.delete_dev_server_project(id, cx))
            })?
            .await?;

            WORKSPACE_DB
                .delete_workspace_by_dev_server_project_id(id)
                .await
                .log_err();

            Ok(())
        })
        .detach_and_prompt_err("Failed to delete dev server project", cx, |_, _| None);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match &self.mode {
            Mode::Default(None) => {}
            Mode::Default(Some(create_project)) => {
                self.create_dev_server_project(create_project.dev_server_id, cx);
            }
            Mode::CreateDevServer(state) => {
                if state.creating.is_none() || state.dev_server_id.is_some() {
                    self.create_or_update_dev_server(
                        state.manual_setup,
                        state.dev_server_id,
                        state.access_token.clone(),
                        cx,
                    );
                }
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        match self.mode {
            Mode::Default(None) => cx.emit(DismissEvent),
            _ => {
                self.mode = Mode::Default(None);
                self.focus_handle(cx).focus(cx);
                cx.notify();
            }
        }
    }

    fn render_dev_server(
        &mut self,
        dev_server: &DevServer,
        create_project: Option<bool>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let dev_server_id = dev_server.id;
        let status = dev_server.status;
        let dev_server_name = dev_server.name.clone();
        let manual_setup = dev_server.ssh_connection_string.is_none();

        v_flex()
            .w_full()
            .child(
                h_flex().group("dev-server").justify_between().child(
                    h_flex()
                        .gap_2()
                        .child(
                            div()
                                .id(("status", dev_server.id.0))
                                .relative()
                                .child(Icon::new(IconName::Server).size(IconSize::Small))
                                .child(div().absolute().bottom_0().left(rems_from_px(8.0)).child(
                                    Indicator::dot().color(match status {
                                        DevServerStatus::Online => Color::Created,
                                        DevServerStatus::Offline => Color::Hidden,
                                    }),
                                ))
                                .tooltip(move |cx| {
                                    Tooltip::text(
                                        match status {
                                            DevServerStatus::Online => "Online",
                                            DevServerStatus::Offline => "Offline",
                                        },
                                        cx,
                                    )
                                }),
                        )
                        .child(
                            div()
                                .max_w(rems(26.))
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .child(Label::new(dev_server_name.clone())),
                        )
                        .child(
                            h_flex()
                                .visible_on_hover("dev-server")
                                .gap_1()
                                .child(
                                    IconButton::new("edit-dev-server", IconName::Pencil)
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.mode = Mode::CreateDevServer(CreateDevServer {
                                                dev_server_id: Some(dev_server_id),
                                                creating: None,
                                                access_token: None,
                                                manual_setup,
                                            });
                                            let dev_server_name = dev_server_name.clone();
                                            this.dev_server_name_input.update(
                                                cx,
                                                move |input, cx| {
                                                    input.editor().update(cx, move |editor, cx| {
                                                        editor.set_text(dev_server_name, cx)
                                                    })
                                                },
                                            )
                                        }))
                                        .tooltip(|cx| Tooltip::text("Edit dev server", cx)),
                                )
                                .child({
                                    let dev_server_id = dev_server.id;
                                    IconButton::new("remove-dev-server", IconName::Trash)
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.delete_dev_server(dev_server_id, cx)
                                        }))
                                        .tooltip(|cx| Tooltip::text("Remove dev server", cx))
                                }),
                        ),
                ),
            )
            .child(
                v_flex()
                    .w_full()
                    .bg(cx.theme().colors().background)
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_md()
                    .my_1()
                    .py_0p5()
                    .px_3()
                    .child(
                        List::new()
                            .empty_message("No projects.")
                            .children(
                                self.dev_server_store
                                    .read(cx)
                                    .projects_for_server(dev_server.id)
                                    .iter()
                                    .map(|p| self.render_dev_server_project(p, cx)),
                            )
                            .when(
                                create_project.is_none()
                                    && dev_server.status == DevServerStatus::Online,
                                |el| {
                                    el.child(
                                        ListItem::new("new-remote_project")
                                            .start_slot(Icon::new(IconName::Plus))
                                            .child(Label::new("Open folder…"))
                                            .on_click(cx.listener(move |this, _, cx| {
                                                this.mode =
                                                    Mode::Default(Some(CreateDevServerProject {
                                                        dev_server_id,
                                                        creating: false,
                                                        _opening: None,
                                                    }));
                                                this.project_path_input
                                                    .read(cx)
                                                    .focus_handle(cx)
                                                    .focus(cx);
                                                cx.notify();
                                            })),
                                    )
                                },
                            )
                            .when_some(create_project, |el, creating| {
                                el.child(self.render_create_new_project(creating, cx))
                            }),
                    ),
            )
    }

    fn render_create_new_project(
        &mut self,
        creating: bool,
        _: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        ListItem::new("create-remote-project")
            .disabled(true)
            .start_slot(Icon::new(IconName::FileTree).color(Color::Muted))
            .child(self.project_path_input.clone())
            .child(div().w(IconSize::Medium.rems()).when(creating, |el| {
                el.child(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::Medium)
                        .with_animation(
                            "arrow-circle",
                            Animation::new(Duration::from_secs(2)).repeat(),
                            |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                        ),
                )
            }))
    }

    fn render_dev_server_project(
        &mut self,
        project: &DevServerProject,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let dev_server_project_id = project.id;
        let project_id = project.project_id;
        let is_online = project_id.is_some();
        let project_path = project.path.clone();

        ListItem::new(("remote-project", dev_server_project_id.0))
            .start_slot(Icon::new(IconName::FileTree).when(!is_online, |icon| icon.color(Color::Muted)))
            .child(
                    Label::new(project.path.clone())
            )
            .on_click(cx.listener(move |_, _, cx| {
                if let Some(project_id) = project_id {
                    if let Some(app_state) = AppState::global(cx).upgrade() {
                        workspace::join_dev_server_project(project_id, app_state, None, cx)
                            .detach_and_prompt_err("Could not join project", cx, |_, _| None)
                    }
                } else {
                    cx.spawn(|_, mut cx| async move {
                        cx.prompt(gpui::PromptLevel::Critical, "This project is offline", Some("The `zed` instance running on this dev server is not connected. You will have to restart it."), &["Ok"]).await.log_err();
                    }).detach();
                }
            }))
            .end_hover_slot::<AnyElement>(Some(IconButton::new("remove-remote-project", IconName::Trash)
                .on_click(cx.listener(move |this, _, cx| {
                    this.delete_dev_server_project(dev_server_project_id, &project_path, cx)
                }))
                .tooltip(|cx| Tooltip::text("Delete remote project", cx)).into_any_element()))
    }

    fn render_create_dev_server(
        &self,
        state: &CreateDevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let creating = state.creating.is_some();
        let dev_server_id = state.dev_server_id;
        let access_token = state.access_token.clone();
        let manual_setup = state.manual_setup;

        let status = dev_server_id
            .map(|id| self.dev_server_store.read(cx).dev_server_status(id))
            .unwrap_or_default();

        let name = self.dev_server_name_input.update(cx, |input, cx| {
            input.editor().update(cx, |editor, cx| {
                if editor.text(cx).is_empty() {
                    if manual_setup {
                        editor.set_placeholder_text("example-server", cx)
                    } else {
                        editor.set_placeholder_text("ssh host", cx)
                    }
                }
                editor.text(cx)
            })
        });

        const MANUAL_SETUP_MESSAGE: &str = "Click create to generate a token for this server. The next step will provide instructions for setting zed up on that machine.";
        const SSH_SETUP_MESSAGE: &str = "Enter the command you use to ssh into this server.\nFor example: `ssh me@my.server` or `gh cs ssh -c example`.";

        Modal::new("create-dev-server", Some(self.scroll_handle.clone()))
            .header(
                ModalHeader::new()
                    .headline("Create Dev Server")
                    .show_back_button(true),
            )
            .section(
                Section::new()
                    .header(if manual_setup {
                        "Server Name".into()
                    } else {
                        "SSH arguments".into()
                    })
                    .child(
                        div()
                            .max_w(rems(16.))
                            .child(self.dev_server_name_input.clone()),
                    ),
            )
            .section(
                Section::new_contained()
                    .header("Connection Method".into())
                    .child(
                        v_flex()
                            .w_full()
                            .gap_y(Spacing::Large.rems(cx))
                            .child(
                                v_flex()
                                    .child(RadioWithLabel::new(
                                        "use-server-name-in-ssh",
                                        Label::new("Connect via SSH (default)"),
                                        !manual_setup,
                                        cx.listener({
                                            move |this, _, cx| {
                                                if let Mode::CreateDevServer(CreateDevServer {
                                                    manual_setup,
                                                    ..
                                                }) = &mut this.mode
                                                {
                                                    *manual_setup = false;
                                                }
                                                cx.notify()
                                            }
                                        }),
                                    ))
                                    .child(RadioWithLabel::new(
                                        "use-server-name-in-ssh",
                                        Label::new("Manual Setup"),
                                        manual_setup,
                                        cx.listener({
                                            move |this, _, cx| {
                                                if let Mode::CreateDevServer(CreateDevServer {
                                                    manual_setup,
                                                    ..
                                                }) = &mut this.mode
                                                {
                                                    *manual_setup = true;
                                                }
                                                cx.notify()
                                            }
                                        }),
                                    )),
                            )
                            .when(dev_server_id.is_none(), |el| {
                                el.child(
                                    if manual_setup {
                                        Label::new(MANUAL_SETUP_MESSAGE)
                                    } else {
                                        Label::new(SSH_SETUP_MESSAGE)
                                    }
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                )
                            })
                            .when(dev_server_id.is_some() && access_token.is_none(), |el| {
                                el.child(
                                    if manual_setup {
                                        Label::new(
                                            "Note: updating the dev server generate a new token",
                                        )
                                    } else {
                                        Label::new(
                                            "Enter the command you use to ssh into this server.\n\
                                        For example: `ssh me@my.server` or `gh cs ssh -c example`.",
                                        )
                                    }
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                )
                            })
                            .when_some(access_token.clone(), {
                                |el, access_token| {
                                    el.child(self.render_dev_server_token_creating(
                                        access_token,
                                        name,
                                        manual_setup,
                                        status,
                                        creating,
                                        cx,
                                    ))
                                }
                            }),
                    ),
            )
            .footer(
                ModalFooter::new().end_slot(if status == DevServerStatus::Online {
                    Button::new("create-dev-server", "Done")
                        .style(ButtonStyle::Filled)
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(cx.listener(move |this, _, cx| {
                            cx.focus(&this.focus_handle);
                            this.mode = Mode::Default(None);
                            cx.notify();
                        }))
                } else {
                    Button::new(
                        "create-dev-server",
                        if manual_setup {
                            if dev_server_id.is_some() {
                                "Update"
                            } else {
                                "Create"
                            }
                        } else {
                            if dev_server_id.is_some() {
                                "Reconnect"
                            } else {
                                "Connect"
                            }
                        },
                    )
                    .style(ButtonStyle::Filled)
                    .layer(ElevationIndex::ModalSurface)
                    .disabled(creating && dev_server_id.is_none())
                    .on_click(cx.listener({
                        let access_token = access_token.clone();
                        move |this, _, cx| {
                            this.create_or_update_dev_server(
                                manual_setup,
                                dev_server_id,
                                access_token.clone(),
                                cx,
                            );
                        }
                    }))
                }),
            )
    }

    fn render_dev_server_token_creating(
        &self,
        access_token: String,
        dev_server_name: String,
        manual_setup: bool,
        status: DevServerStatus,
        creating: bool,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        self.markdown.update(cx, |markdown, cx| {
            if manual_setup {
                markdown.reset(format!("Please log into '{}'. If you don't yet have zed installed, run:\n```\ncurl https://zed.dev/install.sh | bash\n```\nThen to start zed in headless mode:\n```\nzed --dev-server-token {}\n```", dev_server_name, access_token), cx);
            } else {
                markdown.reset("Please wait while we connect over SSH.\n\nIf you run into problems, please [file a bug](https://github.com/zed-industries/zed), and in the meantime try using manual setup.".to_string(), cx);
            }
        });

        v_flex()
            .pl_2()
            .pt_2()
            .gap_2()
            .child(v_flex().w_full().text_sm().child(self.markdown.clone()))
            .map(|el| {
                if status == DevServerStatus::Offline && !manual_setup && !creating {
                    el.child(
                        h_flex()
                            .gap_2()
                            .child(Icon::new(IconName::Disconnected).size(IconSize::Medium))
                            .child(Label::new("Not connected")),
                    )
                } else if status == DevServerStatus::Offline {
                    el.child(Self::render_loading_spinner("Waiting for connection…"))
                } else {
                    el.child(Label::new("🎊 Connection established!"))
                }
            })
    }

    fn render_loading_spinner(label: impl Into<SharedString>) -> Div {
        h_flex()
            .gap_2()
            .child(
                Icon::new(IconName::ArrowCircle)
                    .size(IconSize::Medium)
                    .with_animation(
                        "arrow-circle",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    ),
            )
            .child(Label::new(label))
    }

    fn render_default(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let dev_servers = self.dev_server_store.read(cx).dev_servers();

        let Mode::Default(create_dev_server_project) = &self.mode else {
            unreachable!()
        };

        let mut is_creating = None;
        let mut creating_dev_server = None;
        if let Some(CreateDevServerProject {
            creating,
            dev_server_id,
            ..
        }) = create_dev_server_project
        {
            is_creating = Some(*creating);
            creating_dev_server = Some(*dev_server_id);
        };

        Modal::new("remote-projects", Some(self.scroll_handle.clone()))
            .header(
                ModalHeader::new()
                    .show_dismiss_button(true)
                    .child(Headline::new("Remote Projects").size(HeadlineSize::Small)),
            )
            .section(
                Section::new().child(
                    div().mb_4().child(
                        List::new()
                            .empty_message("No dev servers registered.")
                            .header(Some(
                                ListHeader::new("Dev Servers").end_slot(
                                    Button::new("register-dev-server-button", "New Server")
                                        .icon(IconName::Plus)
                                        .icon_position(IconPosition::Start)
                                        .tooltip(|cx| {
                                            Tooltip::text("Register a new dev server", cx)
                                        })
                                        .on_click(cx.listener(|this, _, cx| {
                                            this.mode =
                                                Mode::CreateDevServer(CreateDevServer::default());
                                            this.dev_server_name_input.update(
                                                cx,
                                                |text_field, cx| {
                                                    text_field.editor().update(cx, |editor, cx| {
                                                        editor.set_text("", cx);
                                                    });
                                                },
                                            );
                                            cx.notify();
                                        })),
                                ),
                            ))
                            .children(dev_servers.iter().map(|dev_server| {
                                let creating = if creating_dev_server == Some(dev_server.id) {
                                    is_creating
                                } else {
                                    None
                                };
                                self.render_dev_server(dev_server, creating, cx)
                                    .into_any_element()
                            })),
                    ),
                ),
            )
    }
}

fn get_text(element: &View<TextField>, cx: &mut WindowContext) -> String {
    element
        .read(cx)
        .editor()
        .read(cx)
        .text(cx)
        .trim()
        .to_string()
}

impl ModalView for DevServerProjects {}

impl FocusableView for DevServerProjects {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for DevServerProjects {}

impl Render for DevServerProjects {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .key_context("DevServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .capture_any_mouse_down(cx.listener(|this, _, cx| {
                this.focus_handle(cx).focus(cx);
            }))
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                if matches!(this.mode, Mode::Default(None)) {
                    cx.emit(DismissEvent)
                }
            }))
            .w(rems(34.))
            .max_h(rems(40.))
            .child(match &self.mode {
                Mode::Default(_) => self.render_default(cx).into_any_element(),
                Mode::CreateDevServer(state) => {
                    self.render_create_dev_server(state, cx).into_any_element()
                }
            })
    }
}

pub fn reconnect_to_dev_server(
    workspace: View<Workspace>,
    dev_server: DevServer,
    cx: &mut WindowContext,
) -> Task<anyhow::Result<()>> {
    let Some(ssh_connection_string) = dev_server.ssh_connection_string else {
        return Task::ready(Err(anyhow!("can't reconnect, no ssh_connection_string")));
    };
    let dev_server_store = dev_server_projects::Store::global(cx);
    let get_access_token = dev_server_store.update(cx, |store, cx| {
        store.regenerate_dev_server_token(dev_server.id, cx)
    });

    cx.spawn(|mut cx| async move {
        let access_token = get_access_token.await?.access_token;

        spawn_ssh_task(
            workspace,
            dev_server_store,
            dev_server.id,
            ssh_connection_string.to_string(),
            access_token,
            &mut cx,
        )
        .await
    })
}

pub async fn spawn_ssh_task(
    workspace: View<Workspace>,
    dev_server_store: Model<dev_server_projects::Store>,
    dev_server_id: DevServerId,
    ssh_connection_string: String,
    access_token: String,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<()> {
    let terminal_panel = workspace
        .update(cx, |workspace, cx| workspace.panel::<TerminalPanel>(cx))
        .ok()
        .flatten()
        .with_context(|| anyhow!("No terminal panel"))?;

    let command = "sh".to_string();
    let args = vec![
        "-x".to_string(),
        "-c".to_string(),
        format!(
            r#"~/.local/bin/zed -v >/dev/stderr || (curl -sSL https://zed.dev/install.sh || wget -qO- https://zed.dev/install.sh) | bash && ~/.local/bin/zed --dev-server-token {}"#,
            access_token
        ),
    ];

    let ssh_connection_string = ssh_connection_string.to_string();

    let terminal = terminal_panel
        .update(cx, |terminal_panel, cx| {
            terminal_panel.spawn_in_new_terminal(
                SpawnInTerminal {
                    id: task::TaskId("ssh-remote".into()),
                    full_label: "Install zed over ssh".into(),
                    label: "Install zed over ssh".into(),
                    command,
                    args,
                    command_label: ssh_connection_string.clone(),
                    cwd: Some(TerminalWorkDir::Ssh {
                        ssh_command: ssh_connection_string,
                        path: None,
                    }),
                    env: Default::default(),
                    use_new_terminal: true,
                    allow_concurrent_runs: false,
                    reveal: RevealStrategy::Always,
                },
                cx,
            )
        })?
        .await?;

    terminal
        .update(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
        .await;

    // There's a race-condition between the task completing successfully, and the server sending us the online status. Make it less likely we'll show the error state.
    if dev_server_store.update(cx, |this, _| this.dev_server_status(dev_server_id))?
        == DevServerStatus::Offline
    {
        cx.background_executor()
            .timer(Duration::from_millis(200))
            .await
    }

    if dev_server_store.update(cx, |this, _| this.dev_server_status(dev_server_id))?
        == DevServerStatus::Offline
    {
        return Err(anyhow!("couldn't reconnect"))?;
    }

    Ok(())
}
