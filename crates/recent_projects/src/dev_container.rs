use regex::Regex;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use ::dev_container::{DevContainerTemplate, get_template_text, get_templates};
use editor::{Editor, MultiBufferOffset};
use gpui::{
    Action, AsyncWindowContext, DismissEvent, EventEmitter, FocusHandle, Focusable, RenderOnce,
    WeakEntity,
};
use node_runtime::NodeRuntime;
use serde::Deserialize;
use settings::DevContainerConnection;
use smallvec::SmallVec;
use smol::fs;
use snippet::{Snippet, TabStop};
use ui::{
    AnyElement, App, Color, CommonAnimationExt, Context, Headline, HeadlineSize, Icon, IconName,
    InteractiveElement, IntoElement, Label, ListItem, ListSeparator, ModalHeader, Navigable,
    NavigableEntry, ParentElement, Render, Styled, StyledExt, Toggleable, Window, div, rems,
};
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::{ModalView, Workspace, with_active_or_new_workspace};

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

async fn ensure_devcontainer_cli(
    node_runtime: &NodeRuntime,
) -> Result<(PathBuf, bool), DevContainerError> {
    let mut command = util::command::new_smol_command(&dev_container_cli());
    command.arg("--version");

    if let Err(e) = command.output().await {
        log::error!(
            "Unable to find devcontainer CLI in $PATH. Checking for a zed installed version. Error: {:?}",
            e
        );

        let Ok(node_runtime_path) = node_runtime.binary_path().await else {
            return Err(DevContainerError::NodeRuntimeNotAvailable);
        };

        let datadir_cli_path = paths::devcontainer_dir()
            .join("node_modules")
            .join("@devcontainers")
            .join("cli")
            .join(format!("{}.js", &dev_container_cli()));

        log::debug!(
            "devcontainer not found in path, using local location: ${}",
            datadir_cli_path.display()
        );

        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(datadir_cli_path.display().to_string());
        command.arg("--version");

        match command.output().await {
            Err(e) => log::error!(
                "Unable to find devcontainer CLI in Data dir. Will try to install. Error: {:?}",
                e
            ),
            Ok(output) => {
                if output.status.success() {
                    log::info!("Found devcontainer CLI in Data dir");
                    return Ok((datadir_cli_path.clone(), false));
                } else {
                    log::error!(
                        "Could not run devcontainer CLI from data_dir. Will try once more to install. Output: {:?}",
                        output
                    );
                }
            }
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

        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(datadir_cli_path.display().to_string());
        command.arg("--version");
        if let Err(e) = command.output().await {
            log::error!(
                "Unable to find devcontainer cli after NPM install. Error: {:?}",
                e
            );
            Err(DevContainerError::DevContainerCliNotAvailable)
        } else {
            Ok((datadir_cli_path, false))
        }
    } else {
        log::info!("Found devcontainer cli on $PATH, using it");
        Ok((PathBuf::from(&dev_container_cli()), true))
    }
}

async fn devcontainer_up(
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime: &NodeRuntime,
    path: Arc<Path>,
) -> Result<DevContainerUp, DevContainerError> {
    let Ok(node_runtime_path) = node_runtime.binary_path().await else {
        log::error!("Unable to find node runtime path");
        return Err(DevContainerError::NodeRuntimeNotAvailable);
    };

    let mut command = if found_in_path {
        let mut command = util::command::new_smol_command(path_to_cli.display().to_string());
        command.arg("up");
        command.arg("--workspace-folder");
        command.arg(path.display().to_string());
        command
    } else {
        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(path_to_cli.display().to_string());
        command.arg("up");
        command.arg("--workspace-folder");
        command.arg(path.display().to_string());
        command
    };

    log::debug!("Running full devcontainer up command: {:?}", command);

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
                let message = format!(
                    "Non-success status running devcontainer up for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );

                log::error!("{}", &message);
                Err(DevContainerError::DevContainerUpFailed(message))
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer up: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerUpFailed(message))
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
                let message = format!(
                    "Non-success status running devcontainer read-configuration for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                log::error!("{}", &message);
                Err(DevContainerError::DevContainerUpFailed(message))
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer read-configuration: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerUpFailed(message))
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

    let (path_to_devcontainer_cli, found_in_path) = ensure_devcontainer_cli(&node_runtime).await?;

    let Some(directory) = project_directory(cx) else {
        return Err(DevContainerError::DevContainerNotFound);
    };

    match devcontainer_up(
        &path_to_devcontainer_cli,
        found_in_path,
        &node_runtime,
        directory.clone(),
    )
    .await
    {
        Ok(DevContainerUp {
            container_id,
            remote_workspace_folder,
            ..
        }) => {
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
        }
        Err(err) => {
            let message = format!("Failed with nested error: {}", err);
            Err(DevContainerError::DevContainerUpFailed(message))
        }
    }
}

#[derive(Debug)]
pub(crate) enum DevContainerError {
    DockerNotAvailable,
    DevContainerCliNotAvailable,
    DevContainerUpFailed(String),
    DevContainerNotFound,
    DevContainerParseFailed,
    NodeRuntimeNotAvailable,
}

impl Display for DevContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DevContainerError::DockerNotAvailable =>
                    "Docker CLI not found on $PATH".to_string(),
                DevContainerError::DevContainerCliNotAvailable =>
                    "Docker not found on path".to_string(),
                DevContainerError::DevContainerUpFailed(message) => {
                    format!("DevContainer creation failed with error: {}", message)
                }
                DevContainerError::DevContainerNotFound => "TODO what".to_string(),
                DevContainerError::DevContainerParseFailed =>
                    "Failed to parse file .devcontainer/devcontainer.json".to_string(),
                DevContainerError::NodeRuntimeNotAvailable =>
                    "Cannot find a valid node runtime".to_string(),
            }
        )
    }
}

#[derive(PartialEq, Clone, Deserialize, Default, Action)]
#[action(namespace = containers)]
#[serde(deny_unknown_fields)]
pub struct InitDevContainer;

pub fn init(cx: &mut App) {
    cx.on_action(|_: &InitDevContainer, cx| {
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let weak_entity = cx.weak_entity();
            workspace.toggle_modal(window, cx, |window, cx| {
                DevContainerModal::new(weak_entity, window, cx)
            });
        });
    });
}

#[derive(Clone)]
pub struct TemplateEntry {
    template: DevContainerTemplate,
    entry: NavigableEntry,
}

// TODO this could be better
impl Eq for TemplateEntry {}
impl PartialEq for TemplateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.template.id == other.template.id
    }
}
impl Debug for TemplateEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateEntry")
            .field("template", &self.template)
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
    TemplatesRetrieved(Vec<DevContainerTemplate>),
    TemplateSelected(DevContainerTemplate),
    GoBack,
}

pub struct DevContainerModal {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    search_navigable_entry: NavigableEntry,
    back_entry: NavigableEntry,
    state: DevContainerState,
}

impl DevContainerModal {
    pub fn new(workspace: WeakEntity<Workspace>, _window: &mut Window, cx: &mut App) -> Self {
        DevContainerModal {
            workspace,
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
                            .on_action({
                                let template = template_entry.template.clone();
                                cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                    this.accept_message(
                                        DevContainerMessage::TemplateSelected(template.clone()),
                                        window,
                                        cx,
                                    );
                                })
                            })
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
                                    .child(Label::new(template_entry.template.name.clone())),
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
            );

        for item in items {
            view = view.entry(item.entry.clone());
        }
        view = view.entry(self.back_entry.clone());
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

impl StatefulModal for DevContainerModal {
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
                cx.spawn_in(window, async move |this, cx| {
                    let client = cx.update(|_, cx| cx.http_client()).unwrap();
                    let Some(templates) = get_templates(client).await.log_err() else {
                        return;
                    };
                    let message = DevContainerMessage::TemplatesRetrieved(templates.templates);
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
                        .filter(|item| item.id == "docker-in-docker".to_string()) // TODO just for simplicity, we'll keep it to one element
                        .map(|item| TemplateEntry {
                            template: item,
                            entry: NavigableEntry::focusable(cx),
                        })
                        .collect())))
                } else {
                    None
                }
            }
            // Dismiss, open a buffer with the template, do a template expand to fill in the values.
            DevContainerMessage::TemplateSelected(template) => {
                let workspace = self.workspace.upgrade().expect("TODO");
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project().clone();

                    let worktree = project
                        .read(cx)
                        .visible_worktrees(cx)
                        .find_map(|tree| tree.read(cx).root_entry()?.is_dir().then_some(tree));

                    if let Some(worktree) = worktree {
                        let tree_id = worktree.read(cx).id();
                        let devcontainer_path =
                            RelPath::unix(".devcontainer/devcontainer.json").unwrap();
                        cx.spawn_in(window, async move |workspace, cx| {
                            let template_text =
                                get_template_text(&template).await.expect("Hard-coded");

                            let Ok(open_task) = workspace.update_in(cx, |workspace, window, cx| {
                                workspace.open_path(
                                    (tree_id, devcontainer_path),
                                    None,
                                    true,
                                    window,
                                    cx,
                                )
                            }) else {
                                return;
                            };

                            // let our_important_data = (template.options, template_text); // How are we going to use this?

                            // let mut small_vec = SmallVec::<[Range<isize>; 2]>::new();
                            // small_vec.push(54_isize..82_isize);

                            // let mut other_small_vec = SmallVec::<[Range<isize>; 2]>::new();
                            // other_small_vec.push(83_isize..83_isize);

                            // let snippet = Snippet {
                            //     text: "\"image\": \"mcr.microsoft.com/devcontainers/base:alpine-${templateOption:imageVariant}\"".to_string(),
                            //     tabstops: vec![
                            //         TabStop {
                            //             ranges: small_vec,
                            //             choices: None,
                            //         },
                            //         TabStop {
                            //             ranges: other_small_vec,
                            //             choices: None,
                            //         },
                            //     ]
                            // };

                            let snippet = build_snippet_from_template(template, template_text);

                            if let Ok(item) = open_task.await {
                                if let Some(editor) = item.downcast::<Editor>() {
                                    editor
                                        .update_in(cx, |editor, window, cx| {
                                            // Things we want to do today:
                                            // - Make this a snippet-expansion workflow
                                            // - Set up a warning if the file already exists - do we want to overwrite?
                                            // - If time:
                                            //   - Dive deeper into the actual call to get the content
                                            editor.clear(window, cx);
                                            editor.insert_snippet(
                                                &vec![MultiBufferOffset(0)..MultiBufferOffset(0)],
                                                snippet,
                                                window,
                                                cx,
                                            )
                                            //editor.insert_snippet(Range<MultiBufferOffset>(0..0), snippet, window, cx)
                                        })
                                        .log_err();
                                };
                            }
                        })
                        .detach();
                    } else {
                        return;
                    }
                });

                self.dismiss(&menu::Cancel, window, cx);
                None
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

pub trait StatefulModal: ModalView + EventEmitter<DismissEvent> + Render {
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

// Note that it looks like we will have to support Dockerfile and all other files in the .devcontainer directory
// Ok, we can re-use a bunch of this logic, but unfortunately we'll need to move towards that form-based UI because of the potential for multiple files in the .devcontainer directory.
// Next step will be to actually grab those files from the server and add them to the project. Then we can look at template expansion for each of them with this code.
fn build_snippet_from_template(template: DevContainerTemplate, template_text: String) -> Snippet {
    static TEMPLATE_OPTION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\$\{templateOption:([^\}]+)\}").expect("Failed to create REGEX")
    });

    let mut tabstops = TEMPLATE_OPTION_REGEX
        .captures_iter(&template_text)
        .filter(|c| c.get(1).is_some())
        .map(|c| {
            let full_match = c.get_match();
            let option_name_match = c.get(1).expect("Filter");
            let options = template.options.clone().and_then(|options| {
                // let thing = option_name_match.as_str();
                // dbg!(&thing);
                // dbg!(&c.get(1));
                let Some(value) = options.get(option_name_match.as_str()) else {
                    return None;
                };

                let mut options = vec![value.default.clone()];

                if value.option_type == "boolean" {
                    if value.default == "false" {
                        options = vec![String::from("false"), String::from("true")];
                    } else {
                        options = vec![String::from("true"), String::from("false")];
                    }
                } else if value.enum_values.is_some() {
                    options.append(
                        &mut value
                            .enum_values
                            .clone()
                            .expect("")
                            .into_iter()
                            .filter(|p| p != &value.default)
                            .collect(),
                    );
                } else if value.proposals.is_some() {
                    options.append(
                        &mut value
                            .proposals
                            .clone()
                            .expect("")
                            .into_iter()
                            .filter(|p| p != &value.default)
                            .collect(),
                    );
                }

                Some(options)
            });
            let mut vec: SmallVec<[Range<isize>; 2]> = SmallVec::new();
            vec.push(full_match.start() as isize..full_match.end() as isize);
            TabStop {
                ranges: vec,
                choices: options.clone(),
            }
        })
        .collect::<Vec<TabStop>>();

    let mut final_range: SmallVec<[Range<isize>; 2]> = SmallVec::new();
    final_range.push(template_text.len() as isize..template_text.len() as isize);
    tabstops.push(TabStop {
        ranges: final_range,
        choices: None,
    });

    Snippet {
        text: template_text,
        tabstops: tabstops,
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use dev_container::{DevContainerTemplate, TemplateOptions};

    use crate::dev_container::{DevContainerUp, build_snippet_from_template};

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

    /// Build snippet tests
    #[test]
    fn should_parse_template_text_and_add_string_tabstops() {
        let string_template_text =
            "mcr.microsoft.com/devcontainers/base:alpine-${templateOption:imageVariant}"
                .to_string();

        let boolean_template_text =
            "Some value which has a boolean, and its value is: ${templateOption:boolValue}"
                .to_string();

        let template_with_string_proposals = DevContainerTemplate {
            id: "test".to_string(),
            name: "test".to_string(),
            options: Some(HashMap::from([(
                "imageVariant".to_string(),
                TemplateOptions {
                    option_type: "string".to_string(),
                    description: "description".to_string(),
                    proposals: Some(vec!["proposal1".to_string(), "proposal2".to_string()]),
                    enum_values: None,
                    default: "proposal1".to_string(),
                },
            )])),
        };

        let snippet = build_snippet_from_template(
            template_with_string_proposals,
            string_template_text.clone(),
        );

        assert_eq!(snippet.text, string_template_text);
        assert_eq!(snippet.tabstops.len(), 2);
        assert_eq!(snippet.tabstops[0].ranges[0], 44..74);
        assert_eq!(
            snippet.tabstops[1].ranges[0],
            (string_template_text.len() - 1) as isize..(string_template_text.len() - 1) as isize
        );

        assert_eq!(
            snippet.tabstops[0].choices,
            Some(vec!["proposal1".to_string(), "proposal2".to_string()])
        );

        let template_with_string_enums = DevContainerTemplate {
            id: "test".to_string(),
            name: "test".to_string(),
            options: Some(HashMap::from([(
                "imageVariant".to_string(),
                TemplateOptions {
                    option_type: "string".to_string(),
                    description: "desc".to_string(),
                    default: "option1".to_string(),
                    proposals: None,
                    enum_values: Some(vec!["option1".to_string(), "option2".to_string()]),
                },
            )])),
        };

        let snippet =
            build_snippet_from_template(template_with_string_enums, string_template_text.clone());

        assert_eq!(snippet.text, string_template_text);
        assert_eq!(snippet.tabstops.len(), 2);
        assert_eq!(snippet.tabstops[0].ranges[0], 44..74);
        assert_eq!(
            snippet.tabstops[1].ranges[0],
            (string_template_text.len() - 1) as isize..(string_template_text.len() - 1) as isize
        );

        assert_eq!(
            snippet.tabstops[0].choices,
            Some(vec!["option1".to_string(), "option2".to_string()])
        );

        let template_with_boolean = DevContainerTemplate {
            id: "test".to_string(),
            name: "Test Template".to_string(),
            options: Some(HashMap::from([(
                "boolValue".to_string(),
                TemplateOptions {
                    option_type: "boolean".to_string(),
                    description: "desc".to_string(),
                    default: "true".to_string(),
                    proposals: None,
                    enum_values: None,
                },
            )])),
        };

        let snippet =
            build_snippet_from_template(template_with_boolean, boolean_template_text.clone());

        assert_eq!(snippet.text, boolean_template_text);
        assert_eq!(snippet.tabstops.len(), 2);
        assert_eq!(snippet.tabstops[0].ranges[0], 50..77);
        assert_eq!(
            snippet.tabstops[1].ranges[0],
            (boolean_template_text.len() - 1) as isize..(boolean_template_text.len() - 1) as isize
        );

        assert_eq!(
            snippet.tabstops[0].choices,
            Some(vec!["true".to_string(), "false".to_string()])
        );

        let template_with_boolean_default_false = DevContainerTemplate {
            id: "test".to_string(),
            name: "Test Template".to_string(),
            options: Some(HashMap::from([(
                "boolValue".to_string(),
                TemplateOptions {
                    option_type: "boolean".to_string(),
                    description: "desc".to_string(),
                    default: "false".to_string(),
                    proposals: None,
                    enum_values: None,
                },
            )])),
        };

        let snippet = build_snippet_from_template(
            template_with_boolean_default_false,
            boolean_template_text.clone(),
        );

        assert_eq!(snippet.text, boolean_template_text);
        assert_eq!(snippet.tabstops.len(), 2);
        assert_eq!(snippet.tabstops[0].ranges[0], 50..77);
        assert_eq!(
            snippet.tabstops[1].ranges[0],
            (boolean_template_text.len() - 1) as isize..(boolean_template_text.len() - 1) as isize
        );

        assert_eq!(
            snippet.tabstops[0].choices,
            Some(vec!["false".to_string(), "true".to_string()])
        );

        let template_with_string_proposals_out_of_order = DevContainerTemplate {
            id: "test".to_string(),
            name: "test".to_string(),
            options: Some(HashMap::from([(
                "imageVariant".to_string(),
                TemplateOptions {
                    option_type: "string".to_string(),
                    description: "description".to_string(),
                    proposals: Some(vec![
                        "proposal1".to_string(),
                        "proposal2".to_string(),
                        "proposal3".to_string(),
                    ]),
                    enum_values: None,
                    default: "proposal2".to_string(),
                },
            )])),
        };

        let snippet = build_snippet_from_template(
            template_with_string_proposals_out_of_order,
            string_template_text.clone(),
        );

        assert_eq!(snippet.text, string_template_text);
        assert_eq!(snippet.tabstops.len(), 2);
        assert_eq!(snippet.tabstops[0].ranges[0], 44..74);
        assert_eq!(
            snippet.tabstops[1].ranges[0],
            (string_template_text.len() - 1) as isize..(string_template_text.len() - 1) as isize
        );

        assert_eq!(
            snippet.tabstops[0].choices,
            Some(vec![
                "proposal2".to_string(),
                "proposal1".to_string(),
                "proposal3".to_string()
            ])
        );

        let template_with_string_enums_out_of_order = DevContainerTemplate {
            id: "test".to_string(),
            name: "test".to_string(),
            options: Some(HashMap::from([(
                "imageVariant".to_string(),
                TemplateOptions {
                    option_type: "string".to_string(),
                    description: "description".to_string(),
                    enum_values: Some(vec![
                        "enum1".to_string(),
                        "enum2".to_string(),
                        "enum3".to_string(),
                    ]),
                    proposals: None,
                    default: "enum2".to_string(),
                },
            )])),
        };

        let snippet = build_snippet_from_template(
            template_with_string_enums_out_of_order,
            string_template_text.clone(),
        );

        assert_eq!(snippet.text, string_template_text);
        assert_eq!(snippet.tabstops.len(), 2);
        assert_eq!(snippet.tabstops[0].ranges[0], 44..74);
        assert_eq!(
            snippet.tabstops[1].ranges[0],
            (string_template_text.len() - 1) as isize..(string_template_text.len() - 1) as isize
        );

        assert_eq!(
            snippet.tabstops[0].choices,
            Some(vec![
                "enum2".to_string(),
                "enum1".to_string(),
                "enum3".to_string()
            ])
        );

        let multi_value_template_text = "
            // For format details, see https://aka.ms/devcontainer.json. For config options, see the
            // README at: https://github.com/devcontainers/templates/tree/main/src/docker-in-docker
            {
	\"name\": \"Docker in Docker\",
	// Or use a Dockerfile or Docker Compose file. More info: https://containers.dev/guide/dockerfile
	\"image\": \"mcr.microsoft.com/devcontainers/base:bullseye\",

	\"features\": {
		\"ghcr.io/devcontainers/features/docker-in-docker:2\": {
			\"version\": \"${templateOption:dockerVersion}\",
			\"enableNonRootDocker\": \"${templateOption:enableNonRootDocker}\",
			\"moby\": \"${templateOption:moby}\",
			\"installZsh\": \"${templateOption:installZsh}\",
			\"upgradePackages\": \"${templateOption:upgradePackages}\"
		}
	}

	// Use 'forwardPorts' to make a list of ports inside the container available locally.
	// \"forwardPorts\": [],

	// Use 'postCreateCommand' to run commands after the container is created.
	// \"postCreateCommand\": \"docker --version\",

	// Configure tool-specific properties.
	// \"customizations\": {},

	// Uncomment to connect as root instead. More info: https://aka.ms/dev-containers-non-root.
	// \"remoteUser\": \"root\"
            }
            ";

        let multi_value_template = DevContainerTemplate {
            id: "test".to_string(),
            name: "Test Template".to_string(),
            options: Some(HashMap::from([
                ("installZsh".to_string(), TemplateOptions {
                    option_type: "boolean".to_string(),
                    description: "Install ZSH!?".to_string(),
                    default: "true".to_string(),
                    proposals: None,
                    enum_values: None,
                }),
                ("upgradePackages".to_string(), TemplateOptions {
                    option_type: "boolean".to_string(),
                    description: "Upgrade OS packages?".to_string(),
                    default: "false".to_string(),
                    proposals: None,
                    enum_values: None,
                }),
                ("dockerVersion".to_string(), TemplateOptions {
                    option_type: "string".to_string(),
                    description: "elect or enter a Docker/Moby CLI version. (Availability can vary by OS version.)".to_string(),
                    default: "latest".to_string(),
                    proposals: Some(vec!["latest".to_string(), "none".to_string(), "20.10".to_string()]),
                    enum_values: None,
                }),
                ("moby".to_string(), TemplateOptions {
                    option_type: "boolean".to_string(),
                    description: "Install OSS Moby build instead of Docker CE".to_string(),
                    proposals: None,
                    enum_values: None,
                    default: "true".to_string(),
                }),
                ("enableNonRootDocker".to_string(), TemplateOptions {
                    option_type: "boolean".to_string(),
                    description: "Enable non-root user to access Docker in container?".to_string(),
                    proposals: None,
                    enum_values: None,
                    default: "true".to_string(),
                })
            ]))
        };

        let snippet = build_snippet_from_template(
            multi_value_template,
            multi_value_template_text.to_string(),
        );

        assert_eq!(snippet.text, multi_value_template_text);
        assert_eq!(snippet.tabstops.len(), 6);
        assert_eq!(snippet.tabstops[0].ranges[0], 491..522);
        assert_eq!(snippet.tabstops[1].ranges[0], 552..589);
        assert_eq!(snippet.tabstops[2].ranges[0], 604..626);
        assert_eq!(snippet.tabstops[3].ranges[0], 647..675);
        assert_eq!(snippet.tabstops[4].ranges[0], 701..734);
        assert_eq!(
            snippet.tabstops[5].ranges[0],
            (multi_value_template_text.len() - 1) as isize
                ..(multi_value_template_text.len() - 1) as isize
        );

        assert_eq!(
            snippet.tabstops[0].choices,
            Some(vec![
                "latest".to_string(),
                "none".to_string(),
                "20.10".to_string()
            ])
        );
        assert_eq!(
            snippet.tabstops[1].choices,
            Some(vec!["true".to_string(), "false".to_string()])
        );
        assert_eq!(
            snippet.tabstops[2].choices,
            Some(vec!["true".to_string(), "false".to_string()])
        );
        assert_eq!(
            snippet.tabstops[3].choices,
            Some(vec!["true".to_string(), "false".to_string()])
        );
        assert_eq!(
            snippet.tabstops[4].choices,
            Some(vec!["false".to_string(), "true".to_string()])
        );
    }
}
