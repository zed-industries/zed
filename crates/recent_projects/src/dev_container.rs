use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use ::dev_container::get_templates;
use gpui::http_client::AsyncBody;
use gpui::{
    Action, AsyncWindowContext, DismissEvent, EventEmitter, FocusHandle, Focusable, RenderOnce,
};
use node_runtime::NodeRuntime;
use serde::Deserialize;
use settings::DevContainerConnection;
use smol::fs;
use smol::io::AsyncReadExt;
use ui::{
    AnyElement, App, Color, CommonAnimationExt, Context, Headline, HeadlineSize, Icon, IconName,
    InteractiveElement, IntoElement, Label, ListItem, ListSeparator, ModalHeader, Navigable,
    NavigableEntry, ParentElement, Render, Styled, StyledExt, Toggleable, Window, div, rems,
};
use util::ResultExt;
use workspace::{ModalView, Workspace, with_active_or_new_workspace};

use crate::dev_container;
use crate::remote_connections::Connection;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerUp {
    _outcome: String,
    container_id: String,
    _remote_user: String,
    remote_workspace_folder: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerConfiguration {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DevContainerConfigurationOutput {
    configuration: DevContainerConfiguration,
}

#[cfg(not(target_os = "windows"))]
fn dev_container_cli() -> String {
    "devcontainer".to_string()
}

#[cfg(target_os = "windows")]
fn dev_container_cli() -> String {
    "devcontainer.cmd".to_string()
}

async fn check_for_docker() -> Result<(), DevContainerError> {
    let mut command = util::command::new_smol_command("docker");
    command.arg("--version");

    match command.output().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Unable to find docker in $PATH: {:?}", e);
            Err(DevContainerError::DockerNotAvailable)
        }
    }
}

async fn ensure_devcontainer_cli(node_runtime: NodeRuntime) -> Result<PathBuf, DevContainerError> {
    let mut command = util::command::new_smol_command(&dev_container_cli());
    command.arg("--version");

    if let Err(e) = command.output().await {
        log::error!(
            "Unable to find devcontainer CLI in $PATH. Checking for a zed installed version. Error: {:?}",
            e
        );

        let datadir_cli_path = paths::devcontainer_dir()
            .join("node_modules")
            .join(".bin")
            .join(&dev_container_cli());

        let mut command =
            util::command::new_smol_command(&datadir_cli_path.as_os_str().display().to_string());
        command.arg("--version");

        if let Err(e) = command.output().await {
            log::error!(
                "Unable to find devcontainer CLI in Data dir. Will try to install. Error: {:?}",
                e
            );
        } else {
            log::info!("Found devcontainer CLI in Data dir");
            return Ok(datadir_cli_path.clone());
        }

        if let Err(e) = fs::create_dir_all(paths::devcontainer_dir()).await {
            log::error!("Unable to create devcontainer directory. Error: {:?}", e);
            return Err(DevContainerError::DevContainerCliNotAvailable);
        }

        if let Err(e) = node_runtime
            .npm_install_packages(
                &paths::devcontainer_dir(),
                &[("@devcontainers/cli", "latest")],
            )
            .await
        {
            log::error!(
                "Unable to install devcontainer CLI to data directory. Error: {:?}",
                e
            );
            return Err(DevContainerError::DevContainerCliNotAvailable);
        };

        let mut command = util::command::new_smol_command(&datadir_cli_path.display().to_string());
        command.arg("--version");
        if let Err(e) = command.output().await {
            log::error!(
                "Unable to find devcontainer cli after NPM install. Error: {:?}",
                e
            );
            Err(DevContainerError::DevContainerCliNotAvailable)
        } else {
            Ok(datadir_cli_path)
        }
    } else {
        log::info!("Found devcontainer cli on $PATH, using it");
        Ok(PathBuf::from(&dev_container_cli()))
    }
}

async fn devcontainer_up(
    path_to_cli: &PathBuf,
    path: Arc<Path>,
) -> Result<DevContainerUp, DevContainerError> {
    let mut command = util::command::new_smol_command(path_to_cli.display().to_string());
    command.arg("up");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                serde_json::from_str::<DevContainerUp>(&raw).map_err(|e| {
                    log::error!(
                        "Unable to parse response from 'devcontainer up' command, error: {:?}",
                        e
                    );
                    DevContainerError::DevContainerParseFailed
                })
            } else {
                log::error!(
                    "Non-success status running devcontainer up for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                Err(DevContainerError::DevContainerUpFailed)
            }
        }
        Err(e) => {
            log::error!("Error running devcontainer up: {:?}", e);
            Err(DevContainerError::DevContainerUpFailed)
        }
    }
}

async fn devcontainer_read_configuration(
    path_to_cli: &PathBuf,
    path: Arc<Path>,
) -> Result<DevContainerConfigurationOutput, DevContainerError> {
    let mut command = util::command::new_smol_command(path_to_cli.display().to_string());
    command.arg("read-configuration");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());
    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                serde_json::from_str::<DevContainerConfigurationOutput>(&raw).map_err(|e| {
                    log::error!(
                        "Unable to parse response from 'devcontainer read-configuration' command, error: {:?}",
                        e
                    );
                    DevContainerError::DevContainerParseFailed
                })
            } else {
                log::error!(
                    "Non-success status running devcontainer read-configuration for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                Err(DevContainerError::DevContainerUpFailed)
            }
        }
        Err(e) => {
            log::error!("Error running devcontainer read-configuration: {:?}", e);
            Err(DevContainerError::DevContainerUpFailed)
        }
    }
}

// Name the project with two fallbacks
async fn get_project_name(
    path_to_cli: &PathBuf,
    path: Arc<Path>,
    remote_workspace_folder: String,
    container_id: String,
) -> Result<String, DevContainerError> {
    if let Ok(dev_container_configuration) =
        devcontainer_read_configuration(path_to_cli, path).await
        && let Some(name) = dev_container_configuration.configuration.name
    {
        // Ideally, name the project after the name defined in devcontainer.json
        Ok(name)
    } else {
        // Otherwise, name the project after the remote workspace folder name
        Ok(Path::new(&remote_workspace_folder)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|string| string.into())
            // Finally, name the project after the container ID as a last resort
            .unwrap_or_else(|| container_id.clone()))
    }
}

fn project_directory(cx: &mut AsyncWindowContext) -> Option<Arc<Path>> {
    let Some(workspace) = cx.window_handle().downcast::<Workspace>() else {
        return None;
    };

    match workspace.update(cx, |workspace, _, cx| {
        workspace.project().read(cx).active_project_directory(cx)
    }) {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("Error getting project directory from workspace: {:?}", e);
            None
        }
    }
}

pub(crate) async fn start_dev_container(
    cx: &mut AsyncWindowContext,
    node_runtime: NodeRuntime,
) -> Result<(Connection, String), DevContainerError> {
    check_for_docker().await?;

    let path_to_devcontainer_cli = ensure_devcontainer_cli(node_runtime).await?;

    let Some(directory) = project_directory(cx) else {
        return Err(DevContainerError::DevContainerNotFound);
    };

    if let Ok(DevContainerUp {
        container_id,
        remote_workspace_folder,
        ..
    }) = devcontainer_up(&path_to_devcontainer_cli, directory.clone()).await
    {
        let project_name = get_project_name(
            &path_to_devcontainer_cli,
            directory,
            remote_workspace_folder.clone(),
            container_id.clone(),
        )
        .await?;

        let connection = Connection::DevContainer(DevContainerConnection {
            name: project_name.into(),
            container_id: container_id.into(),
        });

        Ok((connection, remote_workspace_folder))
    } else {
        Err(DevContainerError::DevContainerUpFailed)
    }
}

#[derive(Debug)]
pub(crate) enum DevContainerError {
    DockerNotAvailable,
    DevContainerCliNotAvailable,
    DevContainerUpFailed,
    DevContainerNotFound,
    DevContainerParseFailed,
}

#[derive(PartialEq, Clone, Deserialize, Default, Action)]
#[action(namespace = containers)]
#[serde(deny_unknown_fields)]
pub struct InitDevContainer;

pub fn init(cx: &mut App) {
    cx.on_action(|_: &InitDevContainer, cx| {
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |window, cx| DevContainerModal::new(window, cx));
        });
    });
}

#[derive(Clone)]
pub struct TemplateEntry {
    name: String,
    entry: NavigableEntry,
}

impl Eq for TemplateEntry {}
impl PartialEq for TemplateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Debug for TemplateEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateEntry")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevContainerState {
    Initial,
    QueryingTemplates,
    TemplateQueryReturned(Result<Vec<TemplateEntry>, String>), // TODO, it's either a successful query manifest or an error
    UserOptionsSpecifying,
    ConfirmingWriteDevContainer,
}

#[derive(Debug, Clone)]
pub enum DevContainerMessage {
    SearchTemplates,
    TemplatesRetrieved(Vec<String>),
    GoBack,
}

pub struct DevContainerModal {
    focus_handle: FocusHandle,
    search_navigable_entry: NavigableEntry,
    back_entry: NavigableEntry,
    state: DevContainerState,
}

impl DevContainerModal {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        DevContainerModal {
            state: DevContainerState::Initial,
            focus_handle: cx.focus_handle(),
            search_navigable_entry: NavigableEntry::focusable(cx),
            back_entry: NavigableEntry::focusable(cx),
        }
    }

    fn render_initial(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let mut view = Navigable::new(
            div()
                .child(
                    div().track_focus(&self.focus_handle).child(
                        ModalHeader::new().child(
                            Headline::new("Create Dev Container").size(HeadlineSize::XSmall),
                        ),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div()
                        .track_focus(&self.search_navigable_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.accept_message(DevContainerMessage::SearchTemplates, window, cx);
                        }))
                        .child(
                            ListItem::new("li-search-containers")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                .toggle_state(
                                    self.search_navigable_entry
                                        .focus_handle
                                        .contains_focused(window, cx),
                                )
                                .child(Label::new("Create dev container from template")),
                        ),
                )
                .into_any_element(),
        );
        view = view.entry(self.search_navigable_entry.clone());
        view.render(window, cx).into_any_element()
    }

    fn render_retrieved_templates(
        &self,
        items: &Vec<TemplateEntry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let mut view =
            Navigable::new(
                div()
                    .child(div().track_focus(&self.focus_handle).child(
                        ModalHeader::new().child(
                            Headline::new("Create Dev Container").size(HeadlineSize::XSmall),
                        ),
                    ))
                    .child(ListSeparator)
                    .children(items.iter().map(|template_entry| {
                        div()
                            .track_focus(&template_entry.entry.focus_handle)
                            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                                // TODO
                                this.accept_message(DevContainerMessage::GoBack, window, cx);
                            }))
                            .child(
                                ListItem::new("li-todo")
                                    .inset(true)
                                    .spacing(ui::ListItemSpacing::Sparse)
                                    .start_slot(Icon::new(IconName::Box))
                                    .toggle_state(
                                        template_entry
                                            .entry
                                            .focus_handle
                                            .contains_focused(window, cx),
                                    )
                                    .child(Label::new(template_entry.name.clone())),
                            )
                    }))
                    .child(ListSeparator)
                    .child(
                        div()
                            .track_focus(&self.back_entry.focus_handle)
                            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                                this.accept_message(DevContainerMessage::GoBack, window, cx);
                            }))
                            .child(
                                ListItem::new("li-goback")
                                    .inset(true)
                                    .spacing(ui::ListItemSpacing::Sparse)
                                    .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                    .toggle_state(
                                        self.back_entry.focus_handle.contains_focused(window, cx),
                                    )
                                    .child(Label::new("Go Back")),
                            ),
                    )
                    .into_any_element(),
            )
            .entry(self.back_entry.clone());

        for item in items {
            view = view.entry(item.entry.clone());
        }
        view.render(window, cx).into_any_element()
    }

    fn render_querying_templates(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        Navigable::new(
            div()
                .child(
                    div().track_focus(&self.focus_handle).child(
                        ModalHeader::new().child(
                            Headline::new("Create Dev Container").size(HeadlineSize::XSmall),
                        ),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div().child(
                        ListItem::new("li-querying")
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(
                                Icon::new(IconName::ArrowCircle)
                                    .color(Color::Muted)
                                    .with_rotate_animation(2),
                            )
                            .child(Label::new("Querying template registry...")),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div()
                        .track_focus(&self.back_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.accept_message(DevContainerMessage::GoBack, window, cx);
                        }))
                        .child(
                            ListItem::new("li-goback")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                .toggle_state(
                                    self.back_entry.focus_handle.contains_focused(window, cx),
                                )
                                .child(Label::new("Go Back")),
                        ),
                )
                .into_any_element(),
        )
        .entry(self.back_entry.clone())
        .render(window, cx)
        .into_any_element()
    }
}

impl ElmLikeModalV2 for DevContainerModal {
    type State = DevContainerState;
    type Message = DevContainerMessage;

    fn state(&self) -> Self::State {
        self.state.clone()
    }

    fn render_for_state(
        &self,
        state: &Self::State,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match state {
            DevContainerState::Initial => self.render_initial(window, cx),
            DevContainerState::QueryingTemplates => self.render_querying_templates(window, cx),
            DevContainerState::TemplateQueryReturned(Ok(items)) => {
                self.render_retrieved_templates(items, window, cx)
            }
            _ => div().into_any_element(),
        }
    }

    fn accept_message(
        &mut self,
        message: Self::Message,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let message = match message {
            DevContainerMessage::SearchTemplates => {
                // Test for now, but basically demonstrating that a call must be made from the render side of things
                dbg!("spawning");
                cx.spawn_in(window, async move |this, cx| {
                    // let timer = smol::Timer::after(Duration::from_millis(5000));
                    // timer.await;
                    let client = cx.update(|_, cx| cx.http_client()).unwrap();
                    let Some(templates) = get_templates(client).await.log_err() else {
                        return;
                    };
                    let message = DevContainerMessage::TemplatesRetrieved(
                        templates.templates.iter().map(|t| t.name.clone()).collect(),
                    );
                    this.update_in(cx, |this, window, cx| {
                        this.accept_message(message, window, cx);
                    })
                    .log_err();
                })
                .detach();
                Some(DevContainerState::QueryingTemplates)
            }
            DevContainerMessage::GoBack => match self.state {
                DevContainerState::Initial => Some(DevContainerState::Initial),
                DevContainerState::QueryingTemplates => Some(DevContainerState::Initial),
                _ => Some(DevContainerState::Initial),
            },
            DevContainerMessage::TemplatesRetrieved(items) => {
                if self.state == DevContainerState::QueryingTemplates {
                    Some(DevContainerState::TemplateQueryReturned(Ok(items
                        .into_iter()
                        .map(|item| TemplateEntry {
                            name: item,
                            entry: NavigableEntry::focusable(cx),
                        })
                        .collect())))
                } else {
                    None
                }
            }
        };
        if let Some(state) = message {
            self.state = state;
            self.focus_handle.focus(window, cx);
        }
        cx.notify();
    }
}
impl EventEmitter<DismissEvent> for DevContainerModal {}
impl Focusable for DevContainerModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl ModalView for DevContainerModal {}

impl Render for DevContainerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_inner(window, cx)
    }
}

pub trait ElmLikeModalV2: ModalView + EventEmitter<DismissEvent> + Render {
    type State;
    type Message;

    fn state(&self) -> Self::State;

    fn render_for_state(
        &self,
        state: &Self::State,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement;

    fn accept_message(
        &mut self,
        message: Self::Message,
        window: &mut Window,
        cx: &mut Context<Self>,
    );

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    // Why can't I make this a default implementation of render?
    fn render_inner(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let element = self.render_for_state(&self.state(), window, cx);
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ContainerModal")
            .on_action(cx.listener(Self::dismiss))
            .child(element)
    }
}

#[cfg(test)]
mod test {

    use crate::dev_container::DevContainerUp;

    #[test]
    fn should_parse_from_devcontainer_json() {
        let json = r#"{"outcome":"success","containerId":"826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a","remoteUser":"vscode","remoteWorkspaceFolder":"/workspaces/zed"}"#;
        let up: DevContainerUp = serde_json::from_str(json).unwrap();
        assert_eq!(up._outcome, "success");
        assert_eq!(
            up.container_id,
            "826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a"
        );
        assert_eq!(up._remote_user, "vscode");
        assert_eq!(up.remote_workspace_folder, "/workspaces/zed");
    }
}
