use std::time::Duration;

use anyhow::Context;
use dev_server_projects::{DevServer, DevServerId, DevServerProject, DevServerProjectId};
use editor::Editor;
use feature_flags::FeatureFlagAppExt;
use feature_flags::FeatureFlagViewExt;
use gpui::Subscription;
use gpui::WeakView;
use gpui::{
    percentage, Action, Animation, AnimationExt, AnyElement, AppContext, ClipboardItem,
    DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, ScrollHandle, Transformation,
    View, ViewContext,
};
use markdown::Markdown;
use markdown::MarkdownStyle;
use rpc::{
    proto::{CreateDevServerResponse, DevServerStatus, RegenerateDevServerTokenResponse},
    ErrorCode, ErrorExt,
};
use task::RevealStrategy;
use task::SpawnInTerminal;
use task::TerminalWorkDir;
use terminal_view::terminal_panel::TerminalPanel;
use ui::Elevation;
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
    rename_dev_server_input: View<TextField>,
    markdown: View<Markdown>,
    _dev_server_subscription: Subscription,
}

#[derive(Default, Clone)]
struct CreateDevServer {
    creating: bool,
    dev_server: Option<CreateDevServerResponse>,
    manual_setup: bool,
}

#[derive(Clone)]
struct EditDevServer {
    dev_server_id: DevServerId,
    state: EditDevServerState,
}

#[derive(Clone, PartialEq)]
enum EditDevServerState {
    Default,
    RenamingDevServer,
    RegeneratingToken,
    RegeneratedToken(RegenerateDevServerTokenResponse),
}

struct CreateDevServerProject {
    dev_server_id: DevServerId,
    creating: bool,
    _opening: Option<Subscription>,
}

enum Mode {
    Default(Option<CreateDevServerProject>),
    CreateDevServer(CreateDevServer),
    EditDevServer(EditDevServer),
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
        let rename_dev_server_input = cx.new_view(|cx| {
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
            link: Default::default(),
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
            rename_dev_server_input,
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
        let path = self.project_path_input.read(cx).text(cx).trim().to_string();

        if path == "" {
            return;
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

    pub fn create_dev_server(&mut self, manual_setup: bool, cx: &mut ViewContext<Self>) {
        let name = get_text(&self.dev_server_name_input, cx);
        if name.is_empty() {
            return;
        }

        let ssh_connection_string = if manual_setup {
            None
        } else {
            Some(name.clone())
        };

        let dev_server = self.dev_server_store.update(cx, |store, cx| {
            store.create_dev_server(name, ssh_connection_string.clone(), cx)
        });

        let workspace = self.workspace.clone();

        cx.spawn(|this, mut cx| async move {
            let result = dev_server.await;

            match result {
                Ok(dev_server) => {
                    if let Some(ssh_connection_string) =  ssh_connection_string {
                    let terminal_panel = workspace
                        .update(&mut cx, |workspace, cx| workspace.panel::<TerminalPanel>(cx))
                        .ok()
                        .flatten()
                        .with_context(|| anyhow::anyhow!("No terminal panel"))?;

                        let command = "sh".to_string();
                        let args = vec!["-c".to_string(), "-x".to_string(),
                            format!(r#"(curl -sSL https://zed.dev/install.sh || wget -qO- https://zed.dev/install.sh) | bash && ~/.local/bin/zed --dev-server-token {}"#, dev_server.access_token)];

                        let terminal = terminal_panel.update(&mut cx, |terminal_panel, cx| {
                            terminal_panel.spawn_in_new_terminal(
                                SpawnInTerminal {
                                    id: task::TaskId("ssh-remote".into()),
                                    full_label: "Installing zed over ssh".into(),
                                    label: "Installing Zed over ssh".into(),
                                    command,
                                    args,
                                    command_label: ssh_connection_string.clone(),
                                    cwd: Some(TerminalWorkDir::Ssh { ssh_command: ssh_connection_string, path: None }),
                                    env: Default::default(),
                                    use_new_terminal: true,
                                    allow_concurrent_runs: false,
                                    reveal: RevealStrategy::Always,
                                },
                                cx,
                            )
                        })?.await?;

                        terminal.update(&mut cx, |terminal, cx| {
                            terminal.wait_for_completed_task(cx)
                        })?.await;

                    this.update(&mut cx, |this, cx| {
                            this.focus_handle.focus(cx);
                            this.mode = Mode::CreateDevServer(CreateDevServer {
                                creating: false,
                                dev_server: Some(dev_server.clone()),
                                manual_setup: false,
                        });
                    })?;
                }
                Ok(())
            }
            Err(e) => {
                this.update(&mut cx, |this, cx| {
                    this.mode = Mode::CreateDevServer(CreateDevServer { creating:false, dev_server: None, manual_setup });
                })
                .log_err();

                return Err(e)
            }
            }
        })
        .detach_and_prompt_err("Failed to create server", cx, |_, _| None);

        self.mode = Mode::CreateDevServer(CreateDevServer {
            creating: true,
            dev_server: None,
            manual_setup,
        });
        cx.notify()
    }

    fn rename_dev_server(&mut self, id: DevServerId, cx: &mut ViewContext<Self>) {
        let name = get_text(&self.rename_dev_server_input, cx);

        let Some(dev_server) = self.dev_server_store.read(cx).dev_server(id) else {
            return;
        };

        if name.is_empty() || dev_server.name == name {
            return;
        }

        let request = self
            .dev_server_store
            .update(cx, |store, cx| store.rename_dev_server(id, name, cx));

        self.mode = Mode::EditDevServer(EditDevServer {
            dev_server_id: id,
            state: EditDevServerState::RenamingDevServer,
        });

        cx.spawn(|this, mut cx| async move {
            request.await?;
            this.update(&mut cx, move |this, cx| {
                this.mode = Mode::EditDevServer(EditDevServer {
                    dev_server_id: id,
                    state: EditDevServerState::Default,
                });
                cx.notify();
            })
        })
        .detach_and_prompt_err("Failed to rename dev server", cx, |_, _| None);
    }

    fn refresh_dev_server_token(&mut self, id: DevServerId, cx: &mut ViewContext<Self>) {
        let answer = cx.prompt(
            gpui::PromptLevel::Warning,
            "Are you sure?",
            Some("This will invalidate the existing dev server token."),
            &["Generate", "Cancel"],
        );
        cx.spawn(|this, mut cx| async move {
            let answer = answer.await?;

            if answer != 0 {
                return Ok(());
            }

            let response = this
                .update(&mut cx, move |this, cx| {
                    let request = this
                        .dev_server_store
                        .update(cx, |store, cx| store.regenerate_dev_server_token(id, cx));
                    this.mode = Mode::EditDevServer(EditDevServer {
                        dev_server_id: id,
                        state: EditDevServerState::RegeneratingToken,
                    });
                    cx.notify();
                    request
                })?
                .await?;

            this.update(&mut cx, move |this, cx| {
                this.mode = Mode::EditDevServer(EditDevServer {
                    dev_server_id: id,
                    state: EditDevServerState::RegeneratedToken(response),
                });
                cx.notify();
            })
            .log_err();

            Ok(())
        })
        .detach_and_prompt_err("Failed to delete dev server", cx, |_, _| None);
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
                if !state.creating && state.dev_server.is_none() {
                    self.create_dev_server(state.manual_setup, cx);
                }
            }
            Mode::EditDevServer(edit_dev_server) => {
                if self
                    .rename_dev_server_input
                    .read(cx)
                    .editor()
                    .read(cx)
                    .is_focused(cx)
                {
                    self.rename_dev_server(edit_dev_server.dev_server_id, cx);
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
                        .child(dev_server_name.clone())
                        .child(
                            h_flex()
                                .visible_on_hover("dev-server")
                                .gap_1()
                                .child(
                                    IconButton::new("edit-dev-server", IconName::Pencil)
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.mode = Mode::EditDevServer(EditDevServer {
                                                dev_server_id,
                                                state: EditDevServerState::Default,
                                            });
                                            let dev_server_name = dev_server_name.clone();
                                            this.rename_dev_server_input.update(
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
                    .bg(cx.theme().colors().title_bar_background) // todo: this should be distinct
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
        &mut self,
        state: CreateDevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let CreateDevServer {
            creating,
            dev_server,
            manual_setup,
        } = state;

        let can_submit = true;

        self.dev_server_name_input.update(cx, |input, cx| {
            input.set_disabled(creating || dev_server.is_some(), cx);
        });

        Modal::new("create-dev-server", Some(self.scroll_handle.clone()))
            .header(
                ModalHeader::new()
                    .headline("Create Dev Server")
                    .show_back_button(true),
            )
            .section(
                Section::new()
                    .header("Server Name".into())
                    .child(
                        div()
                            .max_w(rems(16.))
                            .child(self.dev_server_name_input.clone())
                    ),
            )
            .section(
                Section::new_contained()
                    .header("Connection Method".into())
                    // .meta("Select the type of server you want to create.")
                    .child(
                        v_flex()
                            .w_full()
                            .gap_y(Spacing::Large.rems(cx))
                            .child(v_flex().child(RadioWithLabel::new(
                                "use-server-name-in-ssh",
                                Label::new("Connect via SSH (default)"),
                                !manual_setup,
                                cx.listener(|this, _, cx| {
                                    this.mode = Mode::CreateDevServer(CreateDevServer {
                                        creating: false,
                                        dev_server: None,
                                        manual_setup: false,
                                    });
                                    cx.notify()
                                }),
                            ))
                            .child(RadioWithLabel::new(
                                "use-server-name-in-ssh",
                                Label::new("Manual Setup"),
                                manual_setup,
                                cx.listener(|this, _, cx| {
                                    this.mode = Mode::CreateDevServer(CreateDevServer {
                                        creating: false,
                                        dev_server: None,
                                        manual_setup: true,
                                    });
                                    cx.notify()
                                }),
                            )))
                            .when(dev_server.is_none(), |div| {
                                let server_name = get_text(&self.dev_server_name_input, cx);
                                let server_name_trimmed = server_name.trim();
                                let ssh_host_name = if server_name_trimmed.is_empty() {
                                    "user@host"
                                } else {
                                    server_name_trimmed
                                };
                                div.child(
                                    if manual_setup {
                                        Label::new(format!(
                                            "Once you have created a dev server, you will be given a command to run on the server to register it.\n\
                                            If you enable SSH, then the terminal will automatically `ssh {ssh_host_name}` on open."
                                        ))
                                    } else {
                                        Label::new(format!(
                                            "Once you have created a dev server, you will be given a command to run on the server to register it.\n\
                                            If you enable SSH, then the terminal will automatically `ssh {ssh_host_name}` on open."
                                        ))
                                }.size(LabelSize::Small).color(Color::Muted))
                            }),
                    ),
            )
            // .child(content)
            .footer(ModalFooter::new().end_slot(
                Button::new("create-dev-server", "Create")
                    .style(ButtonStyle::Filled)
                    .layer(ElevationIndex::ModalSurface)
                    .disabled(!can_submit)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.create_dev_server(manual_setup, cx);
                    })),
            ))
    }

    fn render_dev_server_token_instructions(
        &self,
        access_token: &str,
        dev_server_name: &str,
        status: DevServerStatus,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let instructions = SharedString::from(format!("zed --dev-server-token {}", access_token));
        self.markdown.update(cx, |markdown, cx| {
            if !markdown.source().contains(access_token) {
                markdown.reset(format!("```\n{}\n```", instructions), cx);
            }
        });

        v_flex()
            .pl_2()
            .pt_2()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .w_full()
                    .child(Label::new(format!(
                        "Please log into `{}` and run:",
                        dev_server_name
                    )))
                    .child(
                        Button::new("copy-access-token", "Copy Instructions")
                            .icon(Some(IconName::Copy))
                            .icon_size(IconSize::Small)
                            .on_click({
                                let instructions = instructions.clone();
                                cx.listener(move |_, _, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new(
                                        instructions.to_string(),
                                    ))
                                })
                            }),
                    ),
            )
            .child(v_flex().w_full().child(self.markdown.clone()))
            .when(status == DevServerStatus::Offline, |this| {
                this.child(Self::render_loading_spinner("Waiting for connection…"))
            })
            .when(status == DevServerStatus::Online, |this| {
                this.child(Label::new("🎊 Connection established!")).child(
                    h_flex()
                        .justify_end()
                        .child(Button::new("done", "Done").on_click(
                            cx.listener(|_, _, cx| cx.dispatch_action(menu::Cancel.boxed_clone())),
                        )),
                )
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

    fn render_edit_dev_server(
        &mut self,
        edit_dev_server: EditDevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let dev_server_id = edit_dev_server.dev_server_id;
        let dev_server = self
            .dev_server_store
            .read(cx)
            .dev_server(dev_server_id)
            .cloned();

        let dev_server_name = dev_server
            .as_ref()
            .map(|dev_server| dev_server.name.clone())
            .unwrap_or_default();

        let dev_server_status = dev_server
            .map(|dev_server| dev_server.status)
            .unwrap_or(DevServerStatus::Offline);

        let disabled = matches!(
            edit_dev_server.state,
            EditDevServerState::RenamingDevServer | EditDevServerState::RegeneratingToken
        );
        self.rename_dev_server_input.update(cx, |input, cx| {
            input.set_disabled(disabled, cx);
        });

        let rename_dev_server_input_text = self
            .rename_dev_server_input
            .read(cx)
            .editor()
            .read(cx)
            .text(cx);

        let content = v_flex().w_full().gap_2().child(
            h_flex()
                .pb_2()
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .items_end()
                .w_full()
                .px_2()
                .child(
                    div()
                        .pl_2()
                        .max_w(rems(16.))
                        .child(self.rename_dev_server_input.clone()),
                )
                .child(
                    div()
                        .pl_1()
                        .pb(px(3.))
                        .when(
                            edit_dev_server.state != EditDevServerState::RenamingDevServer,
                            |div| {
                                div.child(
                                    Button::new("rename-dev-server", "Rename")
                                        .disabled(
                                            rename_dev_server_input_text.trim().is_empty()
                                                || rename_dev_server_input_text == dev_server_name,
                                        )
                                        .on_click(cx.listener(move |this, _, cx| {
                                            this.rename_dev_server(dev_server_id, cx);
                                            cx.notify();
                                        })),
                                )
                            },
                        )
                        .when(
                            edit_dev_server.state == EditDevServerState::RenamingDevServer,
                            |div| {
                                div.child(
                                    Button::new("rename-dev-server", "Renaming...").disabled(true),
                                )
                            },
                        ),
                ),
        );

        let content = content.child(match edit_dev_server.state {
            EditDevServerState::RegeneratingToken => {
                Self::render_loading_spinner("Generating token...")
            }
            EditDevServerState::RegeneratedToken(response) => self
                .render_dev_server_token_instructions(
                    &response.access_token,
                    &dev_server_name,
                    dev_server_status,
                    cx,
                ),
            _ => h_flex().items_end().w_full().child(
                Button::new("regenerate-dev-server-token", "Generate new access token")
                    .icon(IconName::Update)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.refresh_dev_server_token(dev_server_id, cx);
                        cx.notify();
                    })),
            ),
        });

        v_flex()
            .id("scroll-container")
            .h_full()
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(ModalHeader::new().show_back_button(true).child(
                Headline::new(format!("Edit {}", &dev_server_name)).size(HeadlineSize::Small),
            ))
            .child(div().child(v_flex().w_full().child(content)))
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

        v_flex()
            .id("scroll-container")
            .h_full()
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .px_1()
            .pt_0p5()
            .gap_px()
            .child(
                ModalHeader::new()
                    .show_dismiss_button(true)
                    .child(Headline::new("Remote Projects").size(HeadlineSize::Small)),
            )
            .child(
                div().child(
                    List::new()
                        .empty_message("No dev servers registered.")
                        .header(Some(
                            ListHeader::new("Dev Servers").end_slot(
                                Button::new("register-dev-server-button", "New Server")
                                    .icon(IconName::Plus)
                                    .icon_position(IconPosition::Start)
                                    .tooltip(|cx| Tooltip::text("Register a new dev server", cx))
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.mode =
                                            Mode::CreateDevServer(CreateDevServer::default());
                                        this.dev_server_name_input.update(cx, |text_field, cx| {
                                            text_field.editor().update(cx, |editor, cx| {
                                                editor.set_text("", cx);
                                            });
                                        });
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
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                if matches!(this.mode, Mode::Default(None)) {
                    cx.emit(DismissEvent)
                } else {
                    this.focus_handle(cx).focus(cx);
                    cx.stop_propagation()
                }
            }))
            .pb_4()
            .w(rems(34.))
            .min_h(rems(20.))
            .max_h(rems(40.))
            .child(match &self.mode {
                Mode::Default(_) => self.render_default(cx).into_any_element(),
                Mode::CreateDevServer(state) => self
                    .render_create_dev_server(state.clone(), cx)
                    .into_any_element(),
                Mode::EditDevServer(state) => self
                    .render_edit_dev_server(state.clone(), cx)
                    .into_any_element(),
            })
    }
}
