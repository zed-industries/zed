use dev_container::DevContainerFeature;
use dev_container::TemplateOptions;
use dev_container::get_features;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use ui::SwitchField;
use ui::ToggleState;

use ::dev_container::{DevContainerTemplate, get_templates};
use gpui::{
    Action, AsyncWindowContext, DismissEvent, EventEmitter, FocusHandle, Focusable, RenderOnce,
    WeakEntity,
};
use node_runtime::NodeRuntime;
use serde::Deserialize;
use settings::DevContainerConnection;
use smol::fs;
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
struct DevContainerApply {
    files: Vec<String>,
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
    options_selected: HashMap<String, String>,
    next_option: Option<TemplateOptionSelection>,
    features: Vec<FeatureEntry>,
    features_selected: HashMap<String, DevContainerFeature>,
}

#[derive(Clone)]
pub struct FeatureEntry {
    feature: DevContainerFeature,
    toggle_state: ToggleState,
    entry: NavigableEntry,
}

#[derive(Clone)]
pub struct TemplateOptionSelection {
    option_name: String,
    navigable_options: Vec<(String, NavigableEntry)>,
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

impl Eq for FeatureEntry {}
impl PartialEq for FeatureEntry {
    fn eq(&self, other: &Self) -> bool {
        self.feature.id == other.feature.id
    }
}

impl Debug for FeatureEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureEntry")
            .field("feature", &self.feature)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevContainerState {
    Initial,
    QueryingTemplates,
    TemplateQueryReturned(Result<Vec<TemplateEntry>, String>), // TODO, it's either a successful query manifest or an error
    QueryingFeatures(TemplateEntry),
    FeaturesQueryReturned(TemplateEntry),
    UserOptionsSpecifying(TemplateEntry),
    ConfirmingWriteDevContainer,
}

#[derive(Debug, Clone)]
pub enum DevContainerMessage {
    SearchTemplates,
    TemplatesRetrieved(Vec<DevContainerTemplate>),
    TemplateSelected(TemplateEntry),
    TemplateOptionsSpecified(TemplateEntry),
    TemplateOptionsCompleted(TemplateEntry),
    FeaturesRetrieved(Vec<DevContainerFeature>),
    FeaturesSelecting(FeatureEntry),
    FeaturesSelected(TemplateEntry),
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
        items: Vec<TemplateEntry>,
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
                                let template = template_entry.clone();
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

    fn render_user_options_specifying(
        &self,
        template_entry: TemplateEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(next_option_entries) = &template_entry.next_option else {
            return div().into_any_element();
        };
        let mut view =
            Navigable::new(
                div()
                    .child(
                        div().track_focus(&self.focus_handle).child(
                            ModalHeader::new().child(
                                Headline::new(&next_option_entries.option_name)
                                    .size(HeadlineSize::XSmall),
                            ),
                        ),
                    )
                    .child(ListSeparator)
                    .children(next_option_entries.navigable_options.iter().map(
                        |(option, entry)| {
                            div()
                                .track_focus(&entry.focus_handle)
                                .on_action({
                                    let mut template = template_entry.clone();
                                    template.options_selected.insert(
                                        next_option_entries.option_name.clone(),
                                        option.clone(),
                                    );
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.accept_message(
                                            DevContainerMessage::TemplateOptionsSpecified(
                                                template.clone(),
                                            ),
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
                                            entry.focus_handle.contains_focused(window, cx),
                                        )
                                        .child(Label::new(option)),
                                )
                        },
                    ))
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
        for (_, entry) in &next_option_entries.navigable_options {
            view = view.entry(entry.clone());
        }
        view = view.entry(self.back_entry.clone());
        view.render(window, cx).into_any_element()
    }

    fn render_features_query_returned(
        &self,
        template_entry: TemplateEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let mut view = Navigable::new(
            div()
                .child(
                    div()
                        .track_focus(&self.focus_handle)
                        .child(
                            ModalHeader::new().child(
                                Headline::new("Selected additional features")
                                    .size(HeadlineSize::XSmall),
                            ),
                        ),
                )
                .child(ListSeparator)
                .children(template_entry.features.iter().map(|feature_entry| {
                    SwitchField::new(
                        feature_entry.feature.id.clone(),
                        Some(feature_entry.feature.name.clone()),
                        None,
                        feature_entry.toggle_state,
                        {
                            let _template = template_entry.clone();
                            let feature = feature_entry.clone();
                            let feature = feature.clone();
                            cx.listener(move |this, state: &ToggleState, window, cx| {
                                let mut feature = feature.clone();
                                feature.toggle_state = state.clone();
                                this.accept_message(
                                    DevContainerMessage::FeaturesSelecting(feature),
                                    window,
                                    cx,
                                );
                            })
                        },
                    )
                }))
                .child(ListSeparator)
                .child(
                    div()
                        .track_focus(&self.search_navigable_entry.focus_handle) // TODO
                        .on_action({
                            let template_entry = template_entry.clone();
                            cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                this.accept_message(
                                    DevContainerMessage::FeaturesSelected(template_entry.clone()),
                                    window,
                                    cx,
                                );
                            })
                        })
                        .child(
                            ListItem::new("li-goback")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                .toggle_state(
                                    self.search_navigable_entry
                                        .focus_handle
                                        .contains_focused(window, cx),
                                )
                                .child(Label::new("Confirm")),
                        ),
                )
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

        for feature in template_entry.features {
            view = view.entry(feature.entry.clone());
        }
        view = view.entry(self.search_navigable_entry.clone());
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
    fn render_querying_features(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
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
                            .child(Label::new("Querying features...")),
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
        state: Self::State,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match state {
            DevContainerState::Initial => self.render_initial(window, cx),
            DevContainerState::QueryingTemplates => self.render_querying_templates(window, cx),
            DevContainerState::TemplateQueryReturned(Ok(items)) => {
                self.render_retrieved_templates(items, window, cx)
            }
            DevContainerState::UserOptionsSpecifying(template_entry) => {
                self.render_user_options_specifying(template_entry, window, cx)
            }
            DevContainerState::QueryingFeatures(_) => self.render_querying_features(window, cx),
            DevContainerState::FeaturesQueryReturned(template_entry) => {
                self.render_features_query_returned(template_entry, window, cx)
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
        let new_state = match message {
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
                        // .filter(|item| item.id == "docker-in-docker".to_string()) // TODO just for simplicity, we'll keep it to one element
                        .map(|item| TemplateEntry {
                            template: item,
                            entry: NavigableEntry::focusable(cx),
                            options_selected: HashMap::new(),
                            next_option: None,
                            features: Vec::new(),
                            features_selected: HashMap::new(),
                        })
                        .collect())))
                } else {
                    None
                }
            }
            DevContainerMessage::TemplateSelected(mut template_entry) => {
                let Some(options) = template_entry.template.clone().options else {
                    panic!("not ready yet");
                };

                let options = options
                    .iter()
                    .collect::<Vec<(&String, &TemplateOptions)>>()
                    .clone();

                let Some((first_option_name, first_option)) = options.get(0) else {
                    return self.accept_message(
                        DevContainerMessage::TemplateOptionsCompleted(template_entry),
                        window,
                        cx,
                    );
                };

                let next_option_entries = first_option
                    .possible_values()
                    .into_iter()
                    .map(|option| (option, NavigableEntry::focusable(cx)))
                    .collect();

                template_entry.next_option = Some(TemplateOptionSelection {
                    option_name: (*first_option_name).clone(),
                    navigable_options: next_option_entries,
                });

                Some(DevContainerState::UserOptionsSpecifying(template_entry))
            }
            DevContainerMessage::TemplateOptionsSpecified(mut template_entry) => {
                let Some(options) = template_entry.template.clone().options else {
                    panic!("not ready yet");
                };
                let options = options
                    .iter()
                    // This has to be better, we're iterating over all for no reason. We really just want the first
                    .filter(|(k, _)| !&template_entry.options_selected.contains_key(*k))
                    .collect::<Vec<(&String, &TemplateOptions)>>();

                let Some((next_option_name, next_option)) = options.get(0) else {
                    return self.accept_message(
                        DevContainerMessage::TemplateOptionsCompleted(template_entry),
                        window,
                        cx,
                    );
                };

                let next_option_entries = next_option
                    .possible_values()
                    .into_iter()
                    .map(|option| (option, NavigableEntry::focusable(cx)))
                    .collect();

                template_entry.next_option = Some(TemplateOptionSelection {
                    option_name: (*next_option_name).clone(),
                    navigable_options: next_option_entries,
                });

                Some(DevContainerState::UserOptionsSpecifying(template_entry))
            }
            DevContainerMessage::TemplateOptionsCompleted(template_entry) => {
                cx.spawn_in(window, async move |this, cx| {
                    let client = cx.update(|_, cx| cx.http_client()).unwrap();
                    let Some(features) = get_features(client).await.log_err() else {
                        return;
                    };
                    let message = DevContainerMessage::FeaturesRetrieved(features.features);
                    this.update_in(cx, |this, window, cx| {
                        this.accept_message(message, window, cx);
                    })
                    .log_err();
                })
                .detach();
                Some(DevContainerState::QueryingFeatures(template_entry))
            }
            DevContainerMessage::FeaturesRetrieved(features) => {
                if let DevContainerState::QueryingFeatures(mut template_entry) = self.state.clone()
                {
                    template_entry.features = features
                        .iter()
                        .map(|feature| FeatureEntry {
                            feature: feature.clone(),
                            toggle_state: ToggleState::Unselected,
                            entry: NavigableEntry::focusable(cx),
                        })
                        .collect();
                    Some(DevContainerState::FeaturesQueryReturned(template_entry))
                } else {
                    None
                }
            }
            DevContainerMessage::FeaturesSelecting(feature_entry) => {
                if let DevContainerState::FeaturesQueryReturned(mut template_entry) =
                    self.state.clone()
                {
                    for feature in &mut template_entry.features {
                        if feature == &feature_entry {
                            *feature = feature_entry.clone();
                            template_entry.features_selected.insert(
                                feature_entry.feature.name.clone(),
                                feature_entry.feature.clone(),
                            );
                        }
                    }
                    Some(DevContainerState::FeaturesQueryReturned(template_entry))
                } else {
                    None
                }
            }
            DevContainerMessage::FeaturesSelected(template_entry) => {
                let workspace = self.workspace.upgrade().expect("TODO");

                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project().clone();

                    let worktree = project
                        .read(cx)
                        .visible_worktrees(cx)
                        .find_map(|tree| tree.read(cx).root_entry()?.is_dir().then_some(tree));

                    if let Some(worktree) = worktree {
                        let tree_id = worktree.read(cx).id();
                        let root_path = worktree.read(cx).abs_path();
                        cx.spawn_in(window, async move |workspace, cx| {
                            let node_runtime = workspace
                                .read_with(cx, |workspace, _| {
                                    workspace.app_state().node_runtime.clone()
                                })
                                .unwrap();
                            let (path_to_devcontainer_cli, found_in_path) =
                                ensure_devcontainer_cli(&node_runtime).await.unwrap();
                            let files = apply_dev_container_template(
                                template_entry,
                                &path_to_devcontainer_cli,
                                found_in_path,
                                node_runtime,
                                root_path,
                            )
                            .await
                            .unwrap();

                            if files
                                .files
                                .contains(&".devcontainer/devcontainer.json".to_string())
                            {
                                workspace
                                    .update_in(cx, |workspace, window, cx| {
                                        let path = RelPath::unix(".devcontainer/devcontainer.json")
                                            .unwrap();
                                        workspace.open_path((tree_id, path), None, true, window, cx)
                                    })
                                    // TODO handle this better
                                    .unwrap()
                                    .await
                                    .unwrap();
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
        if let Some(state) = new_state {
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
        state: Self::State,
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
        let element = self.render_for_state(self.state(), window, cx);
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ContainerModal")
            .on_action(cx.listener(Self::dismiss))
            .child(element)
    }
}

async fn apply_dev_container_template(
    template: TemplateEntry,
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime: NodeRuntime,
    path: Arc<Path>,
) -> Result<DevContainerApply, DevContainerError> {
    let Ok(node_runtime_path) = node_runtime.binary_path().await else {
        log::error!("Unable to find node runtime path");
        return Err(DevContainerError::NodeRuntimeNotAvailable);
    };

    let mut command = if found_in_path {
        util::command::new_smol_command(path_to_cli.display().to_string())
    } else {
        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(path_to_cli.display().to_string());
        command
    };

    command.arg("templates");
    command.arg("apply");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());
    command.arg("--template-id");
    command.arg(format!(
        "ghcr.io/devcontainers/templates/{}",
        template.template.id
    )); // TODO
    command.arg("--template-args");
    command.arg(template_args_to_json(template.options_selected));
    command.arg("--features");
    command.arg(template_features_to_json(template.features_selected));
    log::debug!("Running full devcontainer apply command: {:?}", command);

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                serde_json::from_str::<DevContainerApply>(&raw).map_err(|e| {
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

fn template_features_to_json(features_selected: HashMap<String, DevContainerFeature>) -> String {
    let things = features_selected
        .iter()
        .map(|(_, v)| {
            let mut map = HashMap::new();
            map.insert(
                "id",
                format!(
                    "ghcr.io/devcontainers/features/{}:{}",
                    v.id,
                    v.major_version()
                ),
            );
            map
        }) // TODO
        .collect::<Vec<HashMap<&str, String>>>();
    serde_json::to_string(&things).unwrap()
}

fn template_args_to_json(option_selected: HashMap<String, String>) -> String {
    // TODO this should probably be inlined, I'm wrong
    serde_json::to_string(&option_selected).unwrap()
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
