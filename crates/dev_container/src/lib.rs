use gpui::AppContext;
use gpui::Entity;
use gpui::Task;
use picker::Picker;
use picker::PickerDelegate;
use settings::DevContainerConnection;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use ui::ActiveTheme;
use ui::Button;
use ui::Clickable;
use ui::FluentBuilder;
use ui::KeyBinding;
use ui::Switch;
use ui::ToggleState;
use ui::h_flex;
use ui::rems_from_px;

use gpui::{
    Action, AsyncWindowContext, DismissEvent, EventEmitter, FocusHandle, Focusable, RenderOnce,
    WeakEntity,
};
use node_runtime::NodeRuntime;
use serde::Deserialize;
use smol::fs;
use ui::{
    AnyElement, App, Color, CommonAnimationExt, Context, Headline, HeadlineSize, Icon, IconName,
    InteractiveElement, IntoElement, Label, ListItem, ListSeparator, ModalHeader, Navigable,
    NavigableEntry, ParentElement, Render, Styled, StyledExt, Toggleable, Window, div, rems,
};
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::{ModalView, Workspace, with_active_or_new_workspace};

use futures::AsyncReadExt;
use http::Request;
use http_client::{AsyncBody, HttpClient};

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

pub async fn start_dev_container(
    cx: &mut AsyncWindowContext,
    node_runtime: NodeRuntime,
) -> Result<(DevContainerConnection, String), DevContainerError> {
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

            let connection = DevContainerConnection {
                name: project_name.into(),
                container_id: container_id.into(),
            };

            Ok((connection, remote_workspace_folder))
        }
        Err(err) => {
            let message = format!("Failed with nested error: {}", err);
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

#[derive(Debug)]
pub enum DevContainerError {
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
struct InitDevContainer;

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
struct TemplateEntry {
    template: DevContainerTemplate,
    options_selected: HashMap<String, String>,
    next_option: Option<TemplateOptionSelection>,
    features_selected: HashMap<String, DevContainerFeature>,
}

#[derive(Clone)]
struct FeatureEntry {
    feature: DevContainerFeature,
    toggle_state: ToggleState,
}

#[derive(Clone)]
struct TemplateOptionSelection {
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
enum DevContainerState {
    Initial,
    QueryingTemplates,
    TemplateQueryReturned(Result<Vec<TemplateEntry>, String>), // TODO, it's either a successful query manifest or an error
    QueryingFeatures(TemplateEntry),
    FeaturesQueryReturned(TemplateEntry),
    UserOptionsSpecifying(TemplateEntry),
    ConfirmingWriteDevContainer(TemplateEntry),
}

#[derive(Debug, Clone)]
enum DevContainerMessage {
    SearchTemplates,
    TemplatesRetrieved(Vec<DevContainerTemplate>),
    TemplateSelected(TemplateEntry),
    TemplateOptionsSpecified(TemplateEntry),
    TemplateOptionsCompleted(TemplateEntry),
    FeaturesRetrieved(Vec<DevContainerFeature>),
    FeaturesSelected(TemplateEntry),
    ConfirmWriteDevContainer(TemplateEntry),
    GoBack,
}

struct DevContainerModal {
    workspace: WeakEntity<Workspace>,
    picker: Option<Entity<Picker<TemplatePickerDelegate>>>,
    features_picker: Option<Entity<Picker<FeaturePickerDelegate>>>,
    focus_handle: FocusHandle,
    confirm_entry: NavigableEntry,
    back_entry: NavigableEntry,
    state: DevContainerState,
}

struct TemplatePickerDelegate {
    selected_index: usize,
    placeholder_text: String,
    stateful_modal: WeakEntity<DevContainerModal>,
    candidate_templates: Vec<TemplateEntry>,
    matching_indices: Vec<usize>,
    on_confirm: Box<
        dyn FnMut(
            TemplateEntry,
            &mut DevContainerModal,
            &mut Window,
            &mut Context<DevContainerModal>,
        ),
    >,
}

impl TemplatePickerDelegate {
    fn new(
        placeholder_text: String,
        stateful_modal: WeakEntity<DevContainerModal>,
        elements: Vec<TemplateEntry>,
        on_confirm: Box<
            dyn FnMut(
                TemplateEntry,
                &mut DevContainerModal,
                &mut Window,
                &mut Context<DevContainerModal>,
            ),
        >,
    ) -> Self {
        Self {
            selected_index: 0,
            placeholder_text,
            stateful_modal,
            candidate_templates: elements,
            matching_indices: Vec::new(),
            on_confirm,
        }
    }
}

impl PickerDelegate for TemplatePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.matching_indices.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder_text.clone().into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        self.matching_indices = self
            .candidate_templates
            .iter()
            .enumerate()
            .filter(|(_, template_entry)| {
                template_entry.template.id.contains(&query)
                    || template_entry.template.name.contains(&query)
            })
            .map(|(ix, _)| ix)
            .collect();

        self.selected_index = std::cmp::min(
            self.selected_index,
            self.matching_indices.len().saturating_sub(1),
        );
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        let fun = &mut self.on_confirm;

        self.stateful_modal
            .update(cx, |modal, cx| {
                fun(
                    self.candidate_templates[self.matching_indices[self.selected_index]].clone(),
                    modal,
                    window,
                    cx,
                );
            })
            .log_err();
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        self.stateful_modal
            .update(cx, |modal, cx| {
                modal.dismiss(&menu::Cancel, window, cx);
            })
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let Some(template_entry) = self.candidate_templates.get(self.matching_indices[ix]) else {
            return None;
        };
        Some(
            ListItem::new("li-todo")
                .inset(true)
                .spacing(ui::ListItemSpacing::Sparse)
                .start_slot(Icon::new(IconName::Box))
                .toggle_state(selected)
                .child(Label::new(template_entry.template.name.clone()))
                .into_any_element(),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .justify_start()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("run-action", "Continue")
                        .key_binding(
                            KeyBinding::for_action(&menu::Confirm, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .into_any_element(),
        )
    }
}

struct FeaturePickerDelegate {
    selected_index: usize,
    placeholder_text: String,
    stateful_modal: WeakEntity<DevContainerModal>,
    candidate_features: Vec<FeatureEntry>,
    template_entry: TemplateEntry,
    matching_indices: Vec<usize>,
    on_confirm: Box<
        dyn FnMut(
            TemplateEntry,
            &mut DevContainerModal,
            &mut Window,
            &mut Context<DevContainerModal>,
        ),
    >,
}

impl FeaturePickerDelegate {
    fn new(
        placeholder_text: String,
        stateful_modal: WeakEntity<DevContainerModal>,
        candidate_features: Vec<FeatureEntry>,
        template_entry: TemplateEntry,
        on_confirm: Box<
            dyn FnMut(
                TemplateEntry,
                &mut DevContainerModal,
                &mut Window,
                &mut Context<DevContainerModal>,
            ),
        >,
    ) -> Self {
        Self {
            selected_index: 0,
            placeholder_text,
            stateful_modal,
            candidate_features,
            template_entry,
            matching_indices: Vec::new(),
            on_confirm,
        }
    }
}

impl PickerDelegate for FeaturePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.matching_indices.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder_text.clone().into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.matching_indices = self
            .candidate_features
            .iter()
            .enumerate()
            .filter(|(_, feature_entry)| {
                feature_entry.feature.id.contains(&query)
                    || feature_entry.feature.name.contains(&query)
            })
            .map(|(ix, _)| ix)
            .collect();
        self.selected_index = std::cmp::min(
            self.selected_index,
            self.matching_indices.len().saturating_sub(1),
        );
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if secondary {
            self.stateful_modal
                .update(cx, |modal, cx| {
                    (self.on_confirm)(self.template_entry.clone(), modal, window, cx)
                })
                .log_err();
        } else {
            let current = &mut self.candidate_features[self.matching_indices[self.selected_index]];
            current.toggle_state = match current.toggle_state {
                ToggleState::Selected => {
                    self.template_entry
                        .features_selected
                        .remove(&current.feature.id);
                    ToggleState::Unselected
                }
                _ => {
                    self.template_entry
                        .features_selected
                        .insert(current.feature.id.clone(), current.feature.clone());
                    ToggleState::Selected
                }
            };
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.stateful_modal
            .update(cx, |modal, cx| {
                modal.dismiss(&menu::Cancel, window, cx);
            })
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let feature_entry = self.candidate_features[self.matching_indices[ix]].clone();

        Some(
            ListItem::new("li-what")
                .inset(true)
                .toggle_state(selected)
                .start_slot(Switch::new(
                    feature_entry.feature.id.clone(),
                    feature_entry.toggle_state,
                ))
                .child(Label::new(feature_entry.feature.name.clone()))
                .into_any_element(),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .justify_start()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("run-action", "Select Feature")
                        .key_binding(
                            KeyBinding::for_action(&menu::Confirm, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .child(
                    Button::new("run-action-secondary", "Confirm Selections")
                        .key_binding(
                            KeyBinding::for_action(&menu::SecondaryConfirm, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                        }),
                )
                .into_any_element(),
        )
    }
}

impl DevContainerModal {
    fn new(workspace: WeakEntity<Workspace>, _window: &mut Window, cx: &mut App) -> Self {
        DevContainerModal {
            workspace,
            picker: None,
            features_picker: None,
            state: DevContainerState::Initial,
            focus_handle: cx.focus_handle(),
            confirm_entry: NavigableEntry::focusable(cx),
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
                        .track_focus(&self.confirm_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.accept_message(DevContainerMessage::SearchTemplates, window, cx);
                        }))
                        .child(
                            ListItem::new("li-search-containers")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                .toggle_state(
                                    self.confirm_entry.focus_handle.contains_focused(window, cx),
                                )
                                .child(Label::new("Create dev container from template")),
                        ),
                )
                .into_any_element(),
        );
        view = view.entry(self.confirm_entry.clone());
        view.render(window, cx).into_any_element()
    }

    fn render_retrieved_templates(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if let Some(picker) = &self.picker {
            let picker_element = div()
                .track_focus(&self.focus_handle(cx))
                .child(picker.clone().into_any_element())
                .into_any_element();
            picker.focus_handle(cx).focus(window, cx);
            picker_element
        } else {
            div().into_any_element()
        }
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if let Some(picker) = &self.features_picker {
            let picker_element = div()
                .track_focus(&self.focus_handle(cx))
                .child(picker.clone().into_any_element())
                .into_any_element();
            picker.focus_handle(cx).focus(window, cx);
            picker_element
        } else {
            div().into_any_element()
        }
    }

    fn render_confirming_write_dev_container(
        &self,
        template_entry: TemplateEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
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
                    div()
                        .track_focus(&self.confirm_entry.focus_handle)
                        .on_action(cx.listener(move |this, _: &menu::Confirm, window, cx| {
                            this.accept_message(
                                DevContainerMessage::ConfirmWriteDevContainer(
                                    template_entry.clone(),
                                ),
                                window,
                                cx,
                            );
                        }))
                        .child(
                            ListItem::new("li-search-containers")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                .toggle_state(
                                    self.confirm_entry.focus_handle.contains_focused(window, cx),
                                )
                                .child(Label::new("Create dev container from template")),
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
        .entry(self.confirm_entry.clone())
        .entry(self.back_entry.clone())
        .render(window, cx)
        .into_any_element()
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
            DevContainerState::TemplateQueryReturned(Ok(_)) => {
                self.render_retrieved_templates(window, cx)
            }
            DevContainerState::UserOptionsSpecifying(template_entry) => {
                self.render_user_options_specifying(template_entry, window, cx)
            }
            DevContainerState::QueryingFeatures(_) => self.render_querying_features(window, cx),
            DevContainerState::FeaturesQueryReturned(_) => {
                self.render_features_query_returned(window, cx)
            }
            DevContainerState::ConfirmingWriteDevContainer(template_entry) => {
                self.render_confirming_write_dev_container(template_entry, window, cx)
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
                let items = items
                    .into_iter()
                    .map(|item| TemplateEntry {
                        template: item,
                        options_selected: HashMap::new(),
                        next_option: None,
                        features_selected: HashMap::new(),
                    })
                    .collect::<Vec<TemplateEntry>>();
                if self.state == DevContainerState::QueryingTemplates {
                    let delegate = TemplatePickerDelegate::new(
                        "Select a template".to_string(),
                        cx.weak_entity(),
                        items.clone(),
                        Box::new(|entry, this, window, cx| {
                            this.accept_message(
                                DevContainerMessage::TemplateSelected(entry),
                                window,
                                cx,
                            );
                        }),
                    );

                    let picker =
                        cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));
                    self.picker = Some(picker);
                    Some(DevContainerState::TemplateQueryReturned(Ok(items)))
                } else {
                    None
                }
            }
            DevContainerMessage::TemplateSelected(mut template_entry) => {
                let Some(options) = template_entry.template.clone().options else {
                    return self.accept_message(
                        DevContainerMessage::TemplateOptionsCompleted(template_entry),
                        window,
                        cx,
                    );
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
                if let DevContainerState::QueryingFeatures(template_entry) = self.state.clone() {
                    let features = features
                        .iter()
                        .map(|feature| FeatureEntry {
                            feature: feature.clone(),
                            toggle_state: ToggleState::Unselected,
                        })
                        .collect::<Vec<FeatureEntry>>();
                    let delegate = FeaturePickerDelegate::new(
                        "Select features to add".to_string(),
                        cx.weak_entity(),
                        features.clone(),
                        template_entry.clone(),
                        Box::new(|entry, this, window, cx| {
                            this.accept_message(
                                DevContainerMessage::FeaturesSelected(entry),
                                window,
                                cx,
                            );
                        }),
                    );

                    let picker =
                        cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));
                    self.features_picker = Some(picker);
                    Some(DevContainerState::FeaturesQueryReturned(template_entry))
                } else {
                    None
                }
            }
            DevContainerMessage::FeaturesSelected(template_entry) => {
                let workspace = self.workspace.upgrade().expect("TODO");

                // let found_existing_configuration = false;
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

                            // if dev_container_manifest_exists(
                            //     &path_to_devcontainer_cli,
                            //     found_in_path,
                            //     &node_runtime,
                            //     &root_path,
                            // )
                            // .await
                            // {
                            //     return;
                            // }

                            let files = apply_dev_container_template(
                                &template_entry,
                                &path_to_devcontainer_cli,
                                found_in_path,
                                &node_runtime,
                                &root_path,
                            )
                            .await
                            .unwrap();

                            if files
                                .files
                                .contains(&"./.devcontainer/devcontainer.json".to_string())
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
            DevContainerMessage::ConfirmWriteDevContainer(template_entry) => {
                // TODO
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

trait StatefulModal: ModalView + EventEmitter<DismissEvent> + Render {
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

async fn dev_container_manifest_exists(
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime: &NodeRuntime,
    path: &Arc<Path>,
) -> bool {
    true // TODO
}

async fn apply_dev_container_template(
    template_entry: &TemplateEntry,
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime: &NodeRuntime,
    path: &Arc<Path>,
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

    let Ok(serialized_options) = serde_json::to_string(&template_entry.options_selected) else {
        log::error!(
            "Unable to serialize options for {:?}",
            &template_entry.options_selected
        );
        return Err(DevContainerError::DevContainerParseFailed);
    };

    command.arg("templates");
    command.arg("apply");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());
    command.arg("--template-id");
    command.arg(format!(
        "{}/{}",
        template_entry
            .template
            .source_repository
            .as_ref()
            .unwrap_or(&String::from("")),
        template_entry.template.id
    ));
    command.arg("--template-args");
    command.arg(serialized_options);
    command.arg("--features");
    command.arg(template_features_to_json(&template_entry.features_selected));
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

fn template_features_to_json(features_selected: &HashMap<String, DevContainerFeature>) -> String {
    let things = features_selected
        .iter()
        .map(|(_, v)| {
            let mut map = HashMap::new();
            map.insert(
                "id",
                format!(
                    "{}/{}:{}",
                    v.source_repository.as_ref().unwrap_or(&String::from("")),
                    v.id,
                    v.major_version()
                ),
            );
            map
        })
        .collect::<Vec<HashMap<&str, String>>>();
    serde_json::to_string(&things).unwrap()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GithubTokenResponse {
    token: String,
}

fn ghcr_url() -> &'static str {
    "https://ghcr.io"
}

fn ghcr_domain() -> &'static str {
    "ghcr.io"
}

fn devcontainer_templates_repository() -> &'static str {
    "devcontainers/templates"
}

fn devcontainer_features_repository() -> &'static str {
    "devcontainers/features"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ManifestLayer {
    digest: String,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TemplateOptions {
    #[serde(rename = "type")]
    option_type: String,
    description: Option<String>,
    proposals: Option<Vec<String>>,
    #[serde(rename = "enum")]
    enum_values: Option<Vec<String>>,
    // Different repositories surface "default: 'true'" or "default: true",
    // so we need to be flexible in deserializing
    #[serde(deserialize_with = "deserialize_string_or_bool")]
    default: String,
}

fn deserialize_string_or_bool<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrBool {
        String(String),
        Bool(bool),
    }

    match StringOrBool::deserialize(deserializer)? {
        StringOrBool::String(s) => Ok(s),
        StringOrBool::Bool(b) => Ok(b.to_string()),
    }
}

impl TemplateOptions {
    // TODO put this under test
    fn possible_values(&self) -> Vec<String> {
        match self.option_type.as_str() {
            "string" => self
                .enum_values
                .clone()
                .or(self.proposals.clone().or(Some(vec![self.default.clone()])))
                .unwrap_or_default(),
            // If not string, must be boolean
            _ => {
                if self.default == "true" {
                    vec!["true".to_string(), "false".to_string()]
                } else {
                    vec!["false".to_string(), "true".to_string()]
                }
            }
        }
    }
}

// https://distribution.github.io/distribution/spec/api/#pulling-an-image-manifest
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DockerManifestsResponse {
    layers: Vec<ManifestLayer>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DevContainerFeature {
    id: String,
    version: String,
    name: String,
    source_repository: Option<String>,
}

impl DevContainerFeature {
    fn major_version(&self) -> String {
        let Some(mv) = self.version.get(..1) else {
            return "".to_string();
        };
        mv.to_string()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DevContainerTemplate {
    id: String,
    name: String,
    options: Option<HashMap<String, TemplateOptions>>,
    source_repository: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerFeaturesResponse {
    features: Vec<DevContainerFeature>,
}

// https://ghcr.io/v2/devcontainers/templates/blobs/sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerTemplatesResponse {
    templates: Vec<DevContainerTemplate>,
}

async fn get_templates(
    client: Arc<dyn HttpClient>,
) -> Result<DevContainerTemplatesResponse, String> {
    let token = get_ghcr_token(&client).await?;
    let manifest = get_latest_manifest(&token.token, &client).await?;

    let mut template_response =
        get_devcontainer_templates(&token.token, &manifest.layers[0].digest, &client).await?;

    for template in &mut template_response.templates {
        template.source_repository = Some(format!(
            "{}/{}",
            ghcr_domain(),
            devcontainer_templates_repository()
        ));
    }
    Ok(template_response)
}

async fn get_features(client: Arc<dyn HttpClient>) -> Result<DevContainerFeaturesResponse, String> {
    let token = get_ghcr_token(&client).await?;
    let manifest = get_latest_feature_manifest(&token.token, &client).await?;

    let mut features_response =
        get_devcontainer_features(&token.token, &manifest.layers[0].digest, &client).await?;

    for feature in &mut features_response.features {
        feature.source_repository = Some(format!(
            "{}/{}",
            ghcr_domain(),
            devcontainer_features_repository()
        ));
    }
    Ok(features_response)
}

// Once we get the list of templates, and select the ID, we need to
// Get the manifest of that specific template, e.g. https://ghcr.io/v2/devcontainers/templates/alpine/manifests/latest
// /// Layer mediatype:   "mediaType": "application/vnd.devcontainers.layer.v1+tar",
// As opposed to "application/vnd.devcontainers.collection.layer.v1+json" for the list of templates
// Get the content (sent as a tarball) for the layer, e.g. https://ghcr.io/v2/devcontainers/templates/alpine/blobs/sha256:723fb0b5fc6eedd76957710cd45b287ef31362f900ea61190c1472910317bcb1

async fn get_ghcr_token(client: &Arc<dyn HttpClient>) -> Result<GithubTokenResponse, String> {
    let url = format!(
        "{}/token?service=ghcr.io&scope=repository:{}:pull",
        ghcr_url(),
        devcontainer_templates_repository()
    );
    get_deserialized_response("", &url, client).await
}

async fn get_latest_feature_manifest(
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "{}/v2/{}/manifests/latest",
        ghcr_url(),
        devcontainer_features_repository()
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_latest_manifest(
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "{}/v2/{}/manifests/latest",
        ghcr_url(),
        devcontainer_templates_repository()
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_devcontainer_features(
    token: &str,
    blob_digest: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DevContainerFeaturesResponse, String> {
    let url = format!(
        "{}/v2/{}/blobs/{}",
        ghcr_url(),
        devcontainer_features_repository(),
        blob_digest
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_devcontainer_templates(
    token: &str,
    blob_digest: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DevContainerTemplatesResponse, String> {
    let url = format!(
        "{}/v2/{}/blobs/{}",
        ghcr_url(),
        devcontainer_templates_repository(),
        blob_digest
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_deserialized_response<T>(
    token: &str,
    url: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let request = Request::get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.oci.image.manifest.v1+json")
        .body(AsyncBody::default())
        .unwrap();
    // client.send(request).await.unwrap();
    let Ok(response) = client.send(request).await else {
        return Err("Failed get reponse - TODO fix error handling".to_string());
    };

    let mut output = String::new();

    let Ok(_) = response.into_body().read_to_string(&mut output).await else {
        return Err("Failed to read response body - TODO fix error handling".to_string());
    };

    let structured_response: T = serde_json::from_str(&output).unwrap(); // TODO
    Ok(structured_response)
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, anyhow};

    use crate::{
        DevContainerUp, GithubTokenResponse, devcontainer_templates_repository,
        get_deserialized_response, get_devcontainer_templates, get_ghcr_token, get_latest_manifest,
    };

    #[gpui::test]
    async fn test_get_deserialized_response(_cx: &mut TestAppContext) {
        let client = FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response =
            get_deserialized_response::<GithubTokenResponse>("", "https://ghcr.io/token", &client)
                .await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string())
    }

    #[gpui::test]
    async fn test_get_ghcr_token() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path != "/token" {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            let query = request.uri().query();
            if query.is_none()
                || query.unwrap()
                    != format!(
                        "service=ghcr.io&scope=repository:{}:pull",
                        devcontainer_templates_repository()
                    )
            {
                return Err(anyhow!("Unexpected query: {}", query.unwrap_or_default()));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response = get_ghcr_token(&client).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string());
    }

    #[gpui::test]
    async fn test_get_latest_manifests() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path
                != format!(
                    "/v2/{}/manifests/latest",
                    devcontainer_templates_repository()
                )
            {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{
                    \"schemaVersion\": 2,
                    \"mediaType\": \"application/vnd.oci.image.manifest.v1+json\",
                    \"config\": {
                        \"mediaType\": \"application/vnd.devcontainers\",
                        \"digest\": \"sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a\",
                        \"size\": 2
                    },
                    \"layers\": [
                        {
                            \"mediaType\": \"application/vnd.devcontainers.collection.layer.v1+json\",
                            \"digest\": \"sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09\",
                            \"size\": 65235,
                            \"annotations\": {
                                \"org.opencontainers.image.title\": \"devcontainer-collection.json\"
                            }
                        }
                    ],
                    \"annotations\": {
                        \"com.github.package.type\": \"devcontainer_collection\"
                    }
                }".into())
                .unwrap())
        });

        let response = get_latest_manifest("", &client).await;
        assert!(response.is_ok());
        let response = response.unwrap();

        assert_eq!(response.layers.len(), 1);
        assert_eq!(
            response.layers[0].digest,
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09"
        );
    }

    #[gpui::test]
    async fn test_get_devcontainer_templates() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path
                != format!(
                    "/v2/{}/blobs/sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09",
                    devcontainer_templates_repository()
                )
            {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{
                    \"sourceInformation\": {
                        \"source\": \"devcontainer-cli\"
                    },
                    \"templates\": [
                        {
                            \"id\": \"alpine\",
                            \"version\": \"3.4.0\",
                            \"name\": \"Alpine\",
                            \"description\": \"Simple Alpine container with Git installed.\",
                            \"documentationURL\": \"https://github.com/devcontainers/templates/tree/main/src/alpine\",
                            \"publisher\": \"Dev Container Spec Maintainers\",
                            \"licenseURL\": \"https://github.com/devcontainers/templates/blob/main/LICENSE\",
                            \"options\": {
                                \"imageVariant\": {
                                    \"type\": \"string\",
                                    \"description\": \"Alpine version:\",
                                    \"proposals\": [
                                        \"3.21\",
                                        \"3.20\",
                                        \"3.19\",
                                        \"3.18\"
                                    ],
                                    \"default\": \"3.20\"
                                }
                            },
                            \"platforms\": [
                                \"Any\"
                            ],
                            \"optionalPaths\": [
                                \".github/dependabot.yml\"
                            ],
                            \"type\": \"image\",
                            \"files\": [
                                \"NOTES.md\",
                                \"README.md\",
                                \"devcontainer-template.json\",
                                \".devcontainer/devcontainer.json\",
                                \".github/dependabot.yml\"
                            ],
                            \"fileCount\": 5,
                            \"featureIds\": []
                        }
                    ]
                }".into())
                .unwrap())
        });
        let response = get_devcontainer_templates(
            "",
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09",
            &client,
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.templates.len(), 1);
        assert_eq!(response.templates[0].name, "Alpine");
    }

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
