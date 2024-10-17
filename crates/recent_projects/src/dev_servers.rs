use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use dev_server_projects::{DevServer, DevServerId, DevServerProjectId};
use editor::Editor;
use file_finder::OpenPathDelegate;
use futures::channel::oneshot;
use futures::future::Shared;
use futures::FutureExt;
use gpui::canvas;
use gpui::AsyncWindowContext;
use gpui::ClipboardItem;
use gpui::Task;
use gpui::WeakView;
use gpui::{
    AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, FontWeight,
    Model, PromptLevel, ScrollHandle, View, ViewContext,
};
use picker::Picker;
use project::terminals::wrap_for_ssh;
use project::terminals::SshCommand;
use project::Project;
use remote::SshConnectionOptions;
use rpc::proto::DevServerStatus;
use settings::update_settings_file;
use settings::Settings;
use task::HideStrategy;
use task::RevealStrategy;
use task::SpawnInTerminal;
use terminal_view::terminal_panel::TerminalPanel;
use ui::Scrollbar;
use ui::ScrollbarState;
use ui::Section;
use ui::{prelude::*, IconButtonShape, List, ListItem, ListSeparator, Modal, ModalHeader, Tooltip};
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::OpenOptions;
use workspace::Toast;
use workspace::{notifications::DetachAndPromptErr, ModalView, Workspace};

use crate::open_dev_server_project;
use crate::ssh_connections::connect_over_ssh;
use crate::ssh_connections::open_ssh_project;
use crate::ssh_connections::RemoteSettingsContent;
use crate::ssh_connections::SshConnection;
use crate::ssh_connections::SshConnectionHeader;
use crate::ssh_connections::SshConnectionModal;
use crate::ssh_connections::SshProject;
use crate::ssh_connections::SshPrompt;
use crate::ssh_connections::SshSettings;
use crate::OpenRemote;

pub struct DevServerProjects {
    mode: Mode,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    workspace: WeakView<Workspace>,
    selectable_items: SelectableItemList,
}

struct CreateDevServer {
    address_editor: View<Editor>,
    address_error: Option<SharedString>,
    ssh_prompt: Option<View<SshPrompt>>,
    _creating: Option<Task<Option<()>>>,
}

impl CreateDevServer {
    fn new(cx: &mut WindowContext<'_>) -> Self {
        let address_editor = cx.new_view(Editor::single_line);
        address_editor.update(cx, |this, cx| {
            this.focus_handle(cx).focus(cx);
        });
        Self {
            address_editor,
            address_error: None,
            ssh_prompt: None,
            _creating: None,
        }
    }
}

struct ProjectPicker {
    connection_string: SharedString,
    picker: View<Picker<OpenPathDelegate>>,
    _path_task: Shared<Task<Option<()>>>,
}

type SelectedItemCallback =
    Box<dyn Fn(&mut DevServerProjects, &mut ViewContext<DevServerProjects>) + 'static>;

/// Used to implement keyboard navigation for SSH modal.
#[derive(Default)]
struct SelectableItemList {
    items: Vec<SelectedItemCallback>,
    active_item: Option<usize>,
}

struct EditNicknameState {
    index: usize,
    editor: View<Editor>,
}

impl EditNicknameState {
    fn new(index: usize, cx: &mut WindowContext<'_>) -> Self {
        let this = Self {
            index,
            editor: cx.new_view(Editor::single_line),
        };
        let starting_text = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(index)
            .and_then(|state| state.nickname.clone())
            .filter(|text| !text.is_empty());
        this.editor.update(cx, |this, cx| {
            this.set_placeholder_text("Add a nickname for this server", cx);
            if let Some(starting_text) = starting_text {
                this.set_text(starting_text, cx);
            }
        });
        this.editor.focus_handle(cx).focus(cx);
        this
    }
}

impl SelectableItemList {
    fn reset(&mut self) {
        self.items.clear();
    }

    fn reset_selection(&mut self) {
        self.active_item.take();
    }

    fn prev(&mut self, _: &mut WindowContext<'_>) {
        match self.active_item.as_mut() {
            Some(active_index) => {
                *active_index = active_index.checked_sub(1).unwrap_or(self.items.len() - 1)
            }
            None => {
                self.active_item = Some(self.items.len() - 1);
            }
        }
    }

    fn next(&mut self, _: &mut WindowContext<'_>) {
        match self.active_item.as_mut() {
            Some(active_index) => {
                if *active_index + 1 < self.items.len() {
                    *active_index += 1;
                } else {
                    *active_index = 0;
                }
            }
            None => {
                self.active_item = Some(0);
            }
        }
    }

    fn add_item(&mut self, callback: SelectedItemCallback) {
        self.items.push(callback)
    }

    fn is_selected(&self) -> bool {
        self.active_item == self.items.len().checked_sub(1)
    }

    fn confirm(&self, dev_modal: &mut DevServerProjects, cx: &mut ViewContext<DevServerProjects>) {
        if let Some(active_item) = self.active_item.and_then(|ix| self.items.get(ix)) {
            active_item(dev_modal, cx);
        }
    }
}

impl ProjectPicker {
    fn new(
        ix: usize,
        connection_string: SharedString,
        project: Model<Project>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<DevServerProjects>,
    ) -> View<Self> {
        let (tx, rx) = oneshot::channel();
        let lister = project::DirectoryLister::Project(project.clone());
        let query = lister.default_query(cx);
        let delegate = file_finder::OpenPathDelegate::new(tx, lister);

        let picker = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx)
                .width(rems(34.))
                .modal(false);
            picker.set_query(query, cx);
            picker
        });
        cx.new_view(|cx| {
            let _path_task = cx
                .spawn({
                    let workspace = workspace.clone();
                    move |_, mut cx| async move {
                        let Ok(Some(paths)) = rx.await else {
                            workspace
                                .update(&mut cx, |workspace, cx| {
                                    let weak = cx.view().downgrade();
                                    workspace
                                        .toggle_modal(cx, |cx| DevServerProjects::new(cx, weak));
                                })
                                .log_err()?;
                            return None;
                        };

                        let app_state = workspace
                            .update(&mut cx, |workspace, _| workspace.app_state().clone())
                            .ok()?;
                        let options = cx
                            .update(|cx| (app_state.build_window_options)(None, cx))
                            .log_err()?;

                        cx.open_window(options, |cx| {
                            cx.activate_window();

                            let fs = app_state.fs.clone();
                            update_settings_file::<SshSettings>(fs, cx, {
                                let paths = paths
                                    .iter()
                                    .map(|path| path.to_string_lossy().to_string())
                                    .collect();
                                move |setting, _| {
                                    if let Some(server) = setting
                                        .ssh_connections
                                        .as_mut()
                                        .and_then(|connections| connections.get_mut(ix))
                                    {
                                        server.projects.push(SshProject { paths })
                                    }
                                }
                            });

                            let tasks = paths
                                .into_iter()
                                .map(|path| {
                                    project.update(cx, |project, cx| {
                                        project.find_or_create_worktree(&path, true, cx)
                                    })
                                })
                                .collect::<Vec<_>>();
                            cx.spawn(|_| async move {
                                for task in tasks {
                                    task.await?;
                                }
                                Ok(())
                            })
                            .detach_and_prompt_err(
                                "Failed to open path",
                                cx,
                                |_, _| None,
                            );

                            cx.new_view(|cx| {
                                let workspace =
                                    Workspace::new(None, project.clone(), app_state.clone(), cx);

                                workspace
                                    .client()
                                    .telemetry()
                                    .report_app_event("create ssh project".to_string());

                                workspace
                            })
                        })
                        .log_err();
                        Some(())
                    }
                })
                .shared();

            Self {
                _path_task,
                picker,
                connection_string,
            }
        })
    }
}

impl gpui::Render for ProjectPicker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .child(
                SshConnectionHeader {
                    connection_string: self.connection_string.clone(),
                    nickname: None,
                }
                .render(cx),
            )
            .child(self.picker.clone())
    }
}
enum Mode {
    Default(ScrollbarState),
    ViewServerOptions(usize, SshConnection),
    EditNickname(EditNicknameState),
    ProjectPicker(View<ProjectPicker>),
    CreateDevServer(CreateDevServer),
}

impl Mode {
    fn default_mode() -> Self {
        let handle = ScrollHandle::new();
        Self::Default(ScrollbarState::new(handle))
    }
}
impl DevServerProjects {
    pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
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
        let focus_handle = cx.focus_handle();

        let mut base_style = cx.text_style();
        base_style.refine(&gpui::TextStyleRefinement {
            color: Some(cx.theme().colors().editor_foreground),
            ..Default::default()
        });

        Self {
            mode: Mode::default_mode(),
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            workspace,
            selectable_items: Default::default(),
        }
    }

    fn next_item(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        if !matches!(self.mode, Mode::Default(_) | Mode::ViewServerOptions(_, _)) {
            return;
        }
        self.selectable_items.next(cx);
    }
    fn prev_item(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        if !matches!(self.mode, Mode::Default(_) | Mode::ViewServerOptions(_, _)) {
            return;
        }
        self.selectable_items.prev(cx);
    }
    pub fn project_picker(
        ix: usize,
        connection_options: remote::SshConnectionOptions,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
        workspace: WeakView<Workspace>,
    ) -> Self {
        let mut this = Self::new(cx, workspace.clone());
        this.mode = Mode::ProjectPicker(ProjectPicker::new(
            ix,
            connection_options.connection_string().into(),
            project,
            workspace,
            cx,
        ));

        this
    }

    fn create_ssh_server(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        let input = get_text(&editor, cx);
        if input.is_empty() {
            return;
        }

        let connection_options = match SshConnectionOptions::parse_command_line(&input) {
            Ok(c) => c,
            Err(e) => {
                self.mode = Mode::CreateDevServer(CreateDevServer {
                    address_editor: editor,
                    address_error: Some(format!("could not parse: {:?}", e).into()),
                    ssh_prompt: None,
                    _creating: None,
                });
                return;
            }
        };
        let ssh_prompt = cx.new_view(|cx| SshPrompt::new(&connection_options, cx));

        let connection = connect_over_ssh(
            connection_options.dev_server_identifier(),
            connection_options.clone(),
            ssh_prompt.clone(),
            cx,
        )
        .prompt_err("Failed to connect", cx, |_, _| None);

        let address_editor = editor.clone();
        let creating = cx.spawn(move |this, mut cx| async move {
            match connection.await {
                Some(_) => this
                    .update(&mut cx, |this, cx| {
                        let _ = this.workspace.update(cx, |workspace, _| {
                            workspace
                                .client()
                                .telemetry()
                                .report_app_event("create ssh server".to_string())
                        });

                        this.add_ssh_server(connection_options, cx);
                        this.mode = Mode::default_mode();
                        this.selectable_items.reset_selection();
                        cx.notify()
                    })
                    .log_err(),
                None => this
                    .update(&mut cx, |this, cx| {
                        address_editor.update(cx, |this, _| {
                            this.set_read_only(false);
                        });
                        this.mode = Mode::CreateDevServer(CreateDevServer {
                            address_editor,
                            address_error: None,
                            ssh_prompt: None,
                            _creating: None,
                        });
                        cx.notify()
                    })
                    .log_err(),
            };
            None
        });

        editor.update(cx, |this, _| {
            this.set_read_only(true);
        });
        self.mode = Mode::CreateDevServer(CreateDevServer {
            address_editor: editor,
            address_error: None,
            ssh_prompt: Some(ssh_prompt.clone()),
            _creating: Some(creating),
        });
    }

    fn view_server_options(
        &mut self,
        (index, connection): (usize, SshConnection),
        cx: &mut ViewContext<Self>,
    ) {
        self.selectable_items.reset_selection();
        self.mode = Mode::ViewServerOptions(index, connection);
        cx.notify();
    }

    fn create_ssh_project(
        &mut self,
        ix: usize,
        ssh_connection: SshConnection,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let connection_options = ssh_connection.into();
        workspace.update(cx, |_, cx| {
            cx.defer(move |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    SshConnectionModal::new(&connection_options, false, cx)
                });
                let prompt = workspace
                    .active_modal::<SshConnectionModal>(cx)
                    .unwrap()
                    .read(cx)
                    .prompt
                    .clone();

                let connect = connect_over_ssh(
                    connection_options.dev_server_identifier(),
                    connection_options.clone(),
                    prompt,
                    cx,
                )
                .prompt_err("Failed to connect", cx, |_, _| None);
                cx.spawn(move |workspace, mut cx| async move {
                    let Some(session) = connect.await else {
                        workspace
                            .update(&mut cx, |workspace, cx| {
                                let weak = cx.view().downgrade();
                                workspace.toggle_modal(cx, |cx| DevServerProjects::new(cx, weak));
                            })
                            .log_err();
                        return;
                    };

                    workspace
                        .update(&mut cx, |workspace, cx| {
                            let app_state = workspace.app_state().clone();
                            let weak = cx.view().downgrade();
                            let project = project::Project::ssh(
                                session,
                                app_state.client.clone(),
                                app_state.node_runtime.clone(),
                                app_state.user_store.clone(),
                                app_state.languages.clone(),
                                app_state.fs.clone(),
                                cx,
                            );
                            workspace.toggle_modal(cx, |cx| {
                                DevServerProjects::project_picker(
                                    ix,
                                    connection_options,
                                    project,
                                    cx,
                                    weak,
                                )
                            });
                        })
                        .ok();
                })
                .detach()
            })
        })
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match &self.mode {
            Mode::Default(_) | Mode::ViewServerOptions(_, _) => {
                let items = std::mem::take(&mut self.selectable_items);
                items.confirm(self, cx);
                self.selectable_items = items;
            }
            Mode::ProjectPicker(_) => {}
            Mode::CreateDevServer(state) => {
                if let Some(prompt) = state.ssh_prompt.as_ref() {
                    prompt.update(cx, |prompt, cx| {
                        prompt.confirm(cx);
                    });
                    return;
                }

                self.create_ssh_server(state.address_editor.clone(), cx);
            }
            Mode::EditNickname(state) => {
                let text = Some(state.editor.read(cx).text(cx))
                    .filter(|text| !text.is_empty())
                    .map(SharedString::from);
                let index = state.index;
                self.update_settings_file(cx, move |setting, _| {
                    if let Some(connections) = setting.ssh_connections.as_mut() {
                        if let Some(connection) = connections.get_mut(index) {
                            connection.nickname = text;
                        }
                    }
                });
                self.mode = Mode::default_mode();
                self.selectable_items.reset_selection();
                self.focus_handle.focus(cx);
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        match &self.mode {
            Mode::Default(_) => cx.emit(DismissEvent),
            Mode::CreateDevServer(state) if state.ssh_prompt.is_some() => {
                self.mode = Mode::CreateDevServer(CreateDevServer::new(cx));
                self.selectable_items.reset_selection();
                cx.notify();
            }
            _ => {
                self.mode = Mode::default_mode();
                self.selectable_items.reset_selection();
                self.focus_handle(cx).focus(cx);
                cx.notify();
            }
        }
    }

    fn render_ssh_connection(
        &mut self,
        ix: usize,
        ssh_connection: SshConnection,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let (main_label, aux_label) = if let Some(nickname) = ssh_connection.nickname.clone() {
            let aux_label = SharedString::from(format!("({})", ssh_connection.host));
            (nickname, Some(aux_label))
        } else {
            (ssh_connection.host.clone(), None)
        };
        v_flex()
            .w_full()
            .child(ListSeparator)
            .child(
                h_flex()
                    .group("ssh-server")
                    .w_full()
                    .pt_0p5()
                    .px_3()
                    .gap_1()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(
                        Label::new(main_label)
                            .size(LabelSize::Small)
                            .weight(FontWeight::SEMIBOLD)
                            .color(Color::Muted),
                    )
                    .children(
                        aux_label.map(|label| {
                            Label::new(label).size(LabelSize::Small).color(Color::Muted)
                        }),
                    ),
            )
            .child(
                List::new()
                    .empty_message("No projects.")
                    .children(ssh_connection.projects.iter().enumerate().map(|(pix, p)| {
                        v_flex().gap_0p5().child(self.render_ssh_project(
                            ix,
                            &ssh_connection,
                            pix,
                            p,
                            cx,
                        ))
                    }))
                    .child(h_flex().map(|this| {
                        self.selectable_items.add_item(Box::new({
                            let ssh_connection = ssh_connection.clone();
                            move |this, cx| {
                                this.create_ssh_project(ix, ssh_connection.clone(), cx);
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        this.child(
                            ListItem::new(("new-remote-project", ix))
                                .selected(is_selected)
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                                .child(Label::new("Open Folder"))
                                .on_click(cx.listener({
                                    let ssh_connection = ssh_connection.clone();
                                    move |this, _, cx| {
                                        this.create_ssh_project(ix, ssh_connection.clone(), cx);
                                    }
                                })),
                        )
                    }))
                    .child(h_flex().map(|this| {
                        self.selectable_items.add_item(Box::new({
                            let ssh_connection = ssh_connection.clone();
                            move |this, cx| {
                                this.view_server_options((ix, ssh_connection.clone()), cx);
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        this.child(
                            ListItem::new(("server-options", ix))
                                .selected(is_selected)
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Settings).color(Color::Muted))
                                .child(Label::new("View Server Options"))
                                .on_click(cx.listener({
                                    let ssh_connection = ssh_connection.clone();
                                    move |this, _, cx| {
                                        this.view_server_options((ix, ssh_connection.clone()), cx);
                                    }
                                })),
                        )
                    })),
            )
    }

    fn render_ssh_project(
        &mut self,
        server_ix: usize,
        server: &SshConnection,
        ix: usize,
        project: &SshProject,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let server = server.clone();

        let element_id_base = SharedString::from(format!("remote-project-{server_ix}"));
        let callback = Arc::new({
            let project = project.clone();
            move |this: &mut Self, cx: &mut ViewContext<Self>| {
                let Some(app_state) = this
                    .workspace
                    .update(cx, |workspace, _| workspace.app_state().clone())
                    .log_err()
                else {
                    return;
                };
                let project = project.clone();
                let server = server.clone();
                cx.spawn(|_, mut cx| async move {
                    let result = open_ssh_project(
                        server.into(),
                        project.paths.into_iter().map(PathBuf::from).collect(),
                        app_state,
                        OpenOptions::default(),
                        &mut cx,
                    )
                    .await;
                    if let Err(e) = result {
                        log::error!("Failed to connect: {:?}", e);
                        cx.prompt(
                            gpui::PromptLevel::Critical,
                            "Failed to connect",
                            Some(&e.to_string()),
                            &["Ok"],
                        )
                        .await
                        .ok();
                    }
                })
                .detach();
            }
        });
        self.selectable_items.add_item(Box::new({
            let callback = callback.clone();
            move |this, cx| callback(this, cx)
        }));
        let is_selected = self.selectable_items.is_selected();

        ListItem::new((element_id_base, ix))
            .inset(true)
            .selected(is_selected)
            .spacing(ui::ListItemSpacing::Sparse)
            .start_slot(
                Icon::new(IconName::Folder)
                    .color(Color::Muted)
                    .size(IconSize::Small),
            )
            .child(Label::new(project.paths.join(", ")))
            .on_click(cx.listener(move |this, _, cx| callback(this, cx)))
            .end_hover_slot::<AnyElement>(Some(
                IconButton::new("remove-remote-project", IconName::TrashAlt)
                    .icon_size(IconSize::Small)
                    .shape(IconButtonShape::Square)
                    .on_click(
                        cx.listener(move |this, _, cx| this.delete_ssh_project(server_ix, ix, cx)),
                    )
                    .size(ButtonSize::Large)
                    .tooltip(|cx| Tooltip::text("Delete Remote Project", cx))
                    .into_any_element(),
            ))
    }

    fn update_settings_file(
        &mut self,
        cx: &mut ViewContext<Self>,
        f: impl FnOnce(&mut RemoteSettingsContent, &AppContext) + Send + Sync + 'static,
    ) {
        let Some(fs) = self
            .workspace
            .update(cx, |workspace, _| workspace.app_state().fs.clone())
            .log_err()
        else {
            return;
        };
        update_settings_file::<SshSettings>(fs, cx, move |setting, cx| f(setting, cx));
    }

    fn delete_ssh_server(&mut self, server: usize, cx: &mut ViewContext<Self>) {
        self.update_settings_file(cx, move |setting, _| {
            if let Some(connections) = setting.ssh_connections.as_mut() {
                connections.remove(server);
            }
        });
    }

    fn delete_ssh_project(&mut self, server: usize, project: usize, cx: &mut ViewContext<Self>) {
        self.update_settings_file(cx, move |setting, _| {
            if let Some(server) = setting
                .ssh_connections
                .as_mut()
                .and_then(|connections| connections.get_mut(server))
            {
                server.projects.remove(project);
            }
        });
    }

    fn add_ssh_server(
        &mut self,
        connection_options: remote::SshConnectionOptions,
        cx: &mut ViewContext<Self>,
    ) {
        self.update_settings_file(cx, move |setting, _| {
            setting
                .ssh_connections
                .get_or_insert(Default::default())
                .push(SshConnection {
                    host: SharedString::from(connection_options.host),
                    username: connection_options.username,
                    port: connection_options.port,
                    projects: vec![],
                    nickname: None,
                    args: connection_options.args.unwrap_or_default(),
                })
        });
    }

    fn render_create_dev_server(
        &self,
        state: &CreateDevServer,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let ssh_prompt = state.ssh_prompt.clone();

        state.address_editor.update(cx, |editor, cx| {
            if editor.text(cx).is_empty() {
                editor.set_placeholder_text("ssh user@example -p 2222", cx);
            }
        });

        let theme = cx.theme();

        v_flex()
            .id("create-dev-server")
            .overflow_hidden()
            .size_full()
            .flex_1()
            .child(
                div()
                    .p_2()
                    .border_b_1()
                    .border_color(theme.colors().border_variant)
                    .child(state.address_editor.clone()),
            )
            .child(
                h_flex()
                    .bg(theme.colors().editor_background)
                    .rounded_b_md()
                    .w_full()
                    .map(|this| {
                        if let Some(ssh_prompt) = ssh_prompt {
                            this.child(h_flex().w_full().child(ssh_prompt))
                        } else if let Some(address_error) = &state.address_error {
                            this.child(
                                h_flex().p_2().w_full().gap_2().child(
                                    Label::new(address_error.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Error),
                                ),
                            )
                        } else {
                            this.child(
                                h_flex()
                                    .p_2()
                                    .w_full()
                                    .gap_1()
                                    .child(
                                        Label::new(
                                            "Enter the command you use to SSH into this server.",
                                        )
                                        .color(Color::Muted)
                                        .size(LabelSize::Small),
                                    )
                                    .child(
                                        Button::new("learn-more", "Learn more…")
                                            .label_size(LabelSize::Small)
                                            .size(ButtonSize::None)
                                            .color(Color::Accent)
                                            .style(ButtonStyle::Transparent)
                                            .on_click(|_, cx| {
                                                cx.open_url(
                                                    "https://zed.dev/docs/remote-development",
                                                );
                                            }),
                                    ),
                            )
                        }
                    }),
            )
    }

    fn render_view_options(
        &mut self,
        index: usize,
        connection: SshConnection,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let connection_string = connection.host.clone();

        div()
            .size_full()
            .child(
                SshConnectionHeader {
                    connection_string: connection_string.clone(),
                    nickname: connection.nickname.clone(),
                }
                .render(cx),
            )
            .child(
                v_flex()
                    .py_1()
                    .child({
                        self.selectable_items.add_item(Box::new({
                            move |this, cx| {
                                this.mode = Mode::EditNickname(EditNicknameState::new(index, cx));
                                cx.notify();
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        let label = if connection.nickname.is_some() {
                            "Edit Nickname"
                        } else {
                            "Add Nickname to Server"
                        };
                        ListItem::new("add-nickname")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                            .child(Label::new(label))
                            .on_click(cx.listener(move |this, _, cx| {
                                this.mode = Mode::EditNickname(EditNicknameState::new(index, cx));
                                cx.notify();
                            }))
                    })
                    .child({
                        let workspace = self.workspace.clone();
                        fn callback(
                            workspace: WeakView<Workspace>,
                            connection_string: SharedString,
                            cx: &mut WindowContext<'_>,
                        ) {
                            cx.write_to_clipboard(ClipboardItem::new_string(
                                connection_string.to_string(),
                            ));
                            workspace
                                .update(cx, |this, cx| {
                                    struct SshServerAddressCopiedToClipboard;
                                    let notification = format!(
                                        "Copied server address ({}) to clipboard",
                                        connection_string
                                    );

                                    this.show_toast(
                                        Toast::new(
                                            NotificationId::composite::<
                                                SshServerAddressCopiedToClipboard,
                                            >(
                                                connection_string.clone()
                                            ),
                                            notification,
                                        )
                                        .autohide(),
                                        cx,
                                    );
                                })
                                .ok();
                        }
                        self.selectable_items.add_item(Box::new({
                            let workspace = workspace.clone();
                            let connection_string = connection_string.clone();
                            move |_, cx| {
                                callback(workspace.clone(), connection_string.clone(), cx);
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        ListItem::new("copy-server-address")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Copy).color(Color::Muted))
                            .child(Label::new("Copy Server Address"))
                            .end_hover_slot(
                                Label::new(connection_string.clone()).color(Color::Muted),
                            )
                            .on_click({
                                let connection_string = connection_string.clone();
                                move |_, cx| {
                                    callback(workspace.clone(), connection_string.clone(), cx);
                                }
                            })
                    })
                    .child({
                        fn remove_ssh_server(
                            dev_servers: View<DevServerProjects>,
                            index: usize,
                            connection_string: SharedString,
                            cx: &mut WindowContext<'_>,
                        ) {
                            let prompt_message = format!("Remove server `{}`?", connection_string);

                            let confirmation = cx.prompt(
                                PromptLevel::Warning,
                                &prompt_message,
                                None,
                                &["Yes, remove it", "No, keep it"],
                            );

                            cx.spawn(|mut cx| async move {
                                if confirmation.await.ok() == Some(0) {
                                    dev_servers
                                        .update(&mut cx, |this, cx| {
                                            this.delete_ssh_server(index, cx);
                                            this.mode = Mode::default_mode();
                                            cx.notify();
                                        })
                                        .ok();
                                }
                                anyhow::Ok(())
                            })
                            .detach_and_log_err(cx);
                        }
                        self.selectable_items.add_item(Box::new({
                            let connection_string = connection_string.clone();
                            move |_, cx| {
                                remove_ssh_server(
                                    cx.view().clone(),
                                    index,
                                    connection_string.clone(),
                                    cx,
                                );
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        ListItem::new("remove-server")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::Trash).color(Color::Error))
                            .child(Label::new("Remove Server").color(Color::Error))
                            .on_click(cx.listener(move |_, _, cx| {
                                remove_ssh_server(
                                    cx.view().clone(),
                                    index,
                                    connection_string.clone(),
                                    cx,
                                );
                            }))
                    })
                    .child(ListSeparator)
                    .child({
                        self.selectable_items.add_item(Box::new({
                            move |this, cx| {
                                this.mode = Mode::default_mode();
                                cx.notify();
                            }
                        }));
                        let is_selected = self.selectable_items.is_selected();
                        ListItem::new("go-back")
                            .selected(is_selected)
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(Icon::new(IconName::ArrowLeft).color(Color::Muted))
                            .child(Label::new("Go Back"))
                            .on_click(cx.listener(|this, _, cx| {
                                this.mode = Mode::default_mode();
                                cx.notify()
                            }))
                    }),
            )
    }

    fn render_edit_nickname(
        &self,
        state: &EditNicknameState,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let Some(connection) = SshSettings::get_global(cx)
            .ssh_connections()
            .nth(state.index)
        else {
            return v_flex();
        };

        let connection_string = connection.host.clone();

        v_flex()
            .child(
                SshConnectionHeader {
                    connection_string,
                    nickname: connection.nickname.clone(),
                }
                .render(cx),
            )
            .child(h_flex().p_2().child(state.editor.clone()))
    }

    fn render_default(
        &mut self,
        scroll_state: ScrollbarState,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let scroll_state = scroll_state.parent_view(cx.view());
        let ssh_connections = SshSettings::get_global(cx)
            .ssh_connections()
            .collect::<Vec<_>>();
        self.selectable_items.add_item(Box::new(|this, cx| {
            this.mode = Mode::CreateDevServer(CreateDevServer::new(cx));
            cx.notify();
        }));

        let is_selected = self.selectable_items.is_selected();

        let connect_button = ListItem::new("register-dev-server-button")
            .selected(is_selected)
            .inset(true)
            .spacing(ui::ListItemSpacing::Sparse)
            .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
            .child(Label::new("Connect New Server"))
            .on_click(cx.listener(|this, _, cx| {
                let state = CreateDevServer::new(cx);
                this.mode = Mode::CreateDevServer(state);

                cx.notify();
            }));

        let ui::ScrollableHandle::NonUniform(scroll_handle) = scroll_state.scroll_handle() else {
            unreachable!()
        };

        let mut modal_section = v_flex()
            .id("ssh-server-list")
            .overflow_y_scroll()
            .track_scroll(&scroll_handle)
            .size_full()
            .child(connect_button)
            .child(
                h_flex().child(
                    List::new()
                        .empty_message(
                            v_flex()
                                .child(ListSeparator)
                                .child(
                                    div().px_3().child(
                                        Label::new("No dev servers registered yet.")
                                            .color(Color::Muted),
                                    ),
                                )
                                .into_any_element(),
                        )
                        .children(ssh_connections.iter().cloned().enumerate().map(
                            |(ix, connection)| {
                                self.render_ssh_connection(ix, connection, cx)
                                    .into_any_element()
                            },
                        )),
                ),
            )
            .into_any_element();

        Modal::new("remote-projects", Some(self.scroll_handle.clone()))
            .header(
                ModalHeader::new()
                    .child(Headline::new("Remote Projects (alpha)").size(HeadlineSize::XSmall)),
            )
            .section(
                Section::new().padded(false).child(
                    h_flex()
                        .min_h(rems(20.))
                        .size_full()
                        .child(
                            v_flex().size_full().child(ListSeparator).child(
                                canvas(
                                    |bounds, cx| {
                                        modal_section.prepaint_as_root(
                                            bounds.origin,
                                            bounds.size.into(),
                                            cx,
                                        );
                                        modal_section
                                    },
                                    |_, mut modal_section, cx| {
                                        modal_section.paint(cx);
                                    },
                                )
                                .size_full(),
                            ),
                        )
                        .child(
                            div()
                                .occlude()
                                .h_full()
                                .absolute()
                                .right_1()
                                .top_1()
                                .bottom_1()
                                .w(px(12.))
                                .children(Scrollbar::vertical(scroll_state)),
                        ),
                ),
            )
    }
}

fn get_text(element: &View<Editor>, cx: &mut WindowContext) -> String {
    element.read(cx).text(cx).trim().to_string()
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
        self.selectable_items.reset();
        div()
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .key_context("DevServerModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::prev_item))
            .on_action(cx.listener(Self::next_item))
            .capture_any_mouse_down(cx.listener(|this, _, cx| {
                this.focus_handle(cx).focus(cx);
            }))
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                if matches!(this.mode, Mode::Default(_)) {
                    cx.emit(DismissEvent)
                }
            }))
            .w(rems(34.))
            .child(match &self.mode {
                Mode::Default(state) => self.render_default(state.clone(), cx).into_any_element(),
                Mode::ViewServerOptions(index, connection) => self
                    .render_view_options(*index, connection.clone(), cx)
                    .into_any_element(),
                Mode::ProjectPicker(element) => element.clone().into_any_element(),
                Mode::CreateDevServer(state) => {
                    self.render_create_dev_server(state, cx).into_any_element()
                }
                Mode::EditNickname(state) => {
                    self.render_edit_nickname(state, cx).into_any_element()
                }
            })
    }
}

pub fn reconnect_to_dev_server_project(
    workspace: View<Workspace>,
    dev_server: DevServer,
    dev_server_project_id: DevServerProjectId,
    replace_current_window: bool,
    cx: &mut WindowContext,
) -> Task<Result<()>> {
    let store = dev_server_projects::Store::global(cx);
    let reconnect = reconnect_to_dev_server(workspace.clone(), dev_server, cx);
    cx.spawn(|mut cx| async move {
        reconnect.await?;

        cx.background_executor()
            .timer(Duration::from_millis(1000))
            .await;

        if let Some(project_id) = store.update(&mut cx, |store, _| {
            store
                .dev_server_project(dev_server_project_id)
                .and_then(|p| p.project_id)
        })? {
            workspace
                .update(&mut cx, move |_, cx| {
                    open_dev_server_project(
                        replace_current_window,
                        dev_server_project_id,
                        project_id,
                        cx,
                    )
                })?
                .await?;
        }

        Ok(())
    })
}

pub fn reconnect_to_dev_server(
    workspace: View<Workspace>,
    dev_server: DevServer,
    cx: &mut WindowContext,
) -> Task<Result<()>> {
    let Some(ssh_connection_string) = dev_server.ssh_connection_string else {
        return Task::ready(Err(anyhow!("Can't reconnect, no ssh_connection_string")));
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
) -> Result<()> {
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
            r#"~/.local/bin/zed -v >/dev/stderr || (curl -f https://zed.dev/install.sh || wget -qO- https://zed.dev/install.sh) | sh && ZED_HEADLESS=1 ~/.local/bin/zed --dev-server-token {}"#,
            access_token
        ),
    ];

    let ssh_connection_string = ssh_connection_string.to_string();
    let (command, args) = wrap_for_ssh(
        &SshCommand::DevServer(ssh_connection_string.clone()),
        Some((&command, &args)),
        None,
        HashMap::default(),
        None,
    );

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
                    cwd: None,
                    use_new_terminal: true,
                    allow_concurrent_runs: false,
                    reveal: RevealStrategy::Always,
                    hide: HideStrategy::Never,
                    env: Default::default(),
                    shell: Default::default(),
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
