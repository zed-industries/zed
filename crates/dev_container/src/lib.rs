use std::path::Path;

use fs::Fs;
use gpui::AppContext;
use gpui::Entity;
use gpui::Task;
use gpui::WeakEntity;
use http_client::anyhow;
use picker::Picker;
use picker::PickerDelegate;
use project::ProjectEnvironment;
use remote::{RemoteClient, RemoteConnection};
use settings::RegisterSetting;
use settings::Settings;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;
use ui::ActiveTheme;
use ui::Button;
use ui::Clickable;
use ui::FluentBuilder;
use ui::KeyBinding;
use ui::StatefulInteractiveElement;
use ui::Switch;
use ui::ToggleState;
use ui::Tooltip;
use ui::h_flex;
use ui::rems_from_px;
use ui::v_flex;
use util::shell::Shell;

use gpui::{Action, DismissEvent, EventEmitter, FocusHandle, Focusable, RenderOnce};
use serde::Deserialize;
use ui::{
    AnyElement, App, Color, CommonAnimationExt, Context, Headline, HeadlineSize, Icon, IconName,
    InteractiveElement, IntoElement, Label, ListItem, ListSeparator, ModalHeader, Navigable,
    NavigableEntry, ParentElement, Render, Styled, StyledExt, Toggleable, Window, div, rems,
};
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::{ModalView, Workspace, with_active_or_new_workspace};

use http_client::HttpClient;

mod command_json;
mod devcontainer_api;
mod devcontainer_json;
mod devcontainer_manifest;
mod docker;
mod features;
mod oci;

use devcontainer_api::read_default_devcontainer_configuration;

use crate::devcontainer_api::DevContainerError;
use crate::devcontainer_api::apply_devcontainer_template;
use crate::oci::get_deserializable_oci_blob;
use crate::oci::get_latest_oci_manifest;
use crate::oci::get_oci_token;

pub use devcontainer_api::{
    DevContainerConfig, find_configs_in_snapshot, find_devcontainer_configs,
    start_dev_container_with_config,
};

/// Converts a string to a safe environment variable name.
///
/// Mirrors the CLI's `getSafeId` in `containerFeatures.ts`:
/// replaces non-alphanumeric/underscore characters with `_`, replaces a
/// leading sequence of digits/underscores with a single `_`, and uppercases.
pub(crate) fn safe_id_lower(input: &str) -> String {
    get_safe_id(input).to_lowercase()
}
pub(crate) fn safe_id_upper(input: &str) -> String {
    get_safe_id(input).to_uppercase()
}
fn get_safe_id(input: &str) -> String {
    let replaced: String = input
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let without_leading = replaced.trim_start_matches(|c: char| c.is_ascii_digit() || c == '_');
    let result = if without_leading.len() < replaced.len() {
        format!("_{}", without_leading)
    } else {
        replaced
    };
    result
}

/// The machine whose container engine builds and runs a dev container, and on
/// whose filesystem the project being opened lives.
///
/// These are deliberately the same choice: the engine has to be able to bind
/// mount the project directory, so a container cannot be built by one machine
/// for a project that lives on another.
#[derive(Clone, Default)]
pub enum DevContainerHost {
    /// The machine running Zed.
    #[default]
    Local,
    /// A machine already reached by an open remote project, whose connection
    /// carries the engine invocations.
    Remote(Arc<dyn RemoteConnection>),
}

impl DevContainerHost {
    /// How a connection to a container on this host is addressed.
    ///
    /// Provisioning and connecting are separate steps that must agree on the
    /// machine: a container built here can only be reached by a connection
    /// that names the same host.
    pub fn docker_host(&self) -> Result<remote::DockerHost, DevContainerError> {
        let DevContainerHost::Remote(connection) = self else {
            return Ok(remote::DockerHost::Local);
        };
        match connection.connection_options() {
            remote::RemoteConnectionOptions::Ssh(options) => Ok(remote::DockerHost::Ssh(options)),
            remote::RemoteConnectionOptions::Wsl(options) => Ok(remote::DockerHost::Wsl(options)),
            #[cfg(any(test, feature = "test-support"))]
            remote::RemoteConnectionOptions::Mock(options) => Ok(remote::DockerHost::Mock(options)),
            other => Err(DevContainerError::UnsupportedHost(
                other.connection_type().to_string(),
            )),
        }
    }

    /// How the host writes paths.
    ///
    /// Every path that reaches an engine command, a bind mount, or a container
    /// label describes the host's filesystem, so it must be rendered in this
    /// style rather than the style of the machine running Zed.
    pub(crate) fn path_style(&self) -> util::paths::PathStyle {
        match self {
            DevContainerHost::Local => util::paths::PathStyle::local(),
            DevContainerHost::Remote(connection) => connection.path_style(),
        }
    }

    /// Joins a relative path onto a host path.
    ///
    /// `Path::join` and `Path::parent` split on the separators of the machine
    /// running Zed, which mangles a host path written in the other style.
    pub(crate) fn join(&self, base: &Path, relative: &Path) -> std::path::PathBuf {
        let DevContainerHost::Remote(_) = self else {
            return base.join(relative);
        };
        let path_style = self.path_style();
        match path_style.join(base, relative) {
            Some(joined) => std::path::PathBuf::from(path_style.normalize(&joined)),
            None => base.join(relative),
        }
    }

    /// The directory containing `path`, split in the host's style.
    pub(crate) fn parent(&self, path: &Path) -> Option<std::path::PathBuf> {
        let DevContainerHost::Remote(_) = self else {
            return path.parent().map(Path::to_path_buf);
        };
        let path = path.display().to_string();
        let index = path.rfind(self.path_style().separators_ch())?;
        if index == 0 {
            return Some(std::path::PathBuf::from(&path[..1]));
        }
        Some(std::path::PathBuf::from(&path[..index]))
    }

    /// Builds the process Zed spawns in order to run `program args` on this
    /// host, with `working_dir` interpreted as a path on the host.
    ///
    /// For a remote host the returned command is the transport's, wrapping the
    /// requested one; the transport owns quoting, so callers must pass
    /// arguments unquoted and must not join them into a shell string.
    pub(crate) fn command(
        &self,
        program: &str,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<&Path>,
    ) -> Result<util::command::Command, DevContainerError> {
        match self {
            DevContainerHost::Local => {
                let mut command = util::command::Command::new(program);
                command.args(args);
                command.envs(env);
                if let Some(working_dir) = working_dir {
                    command.current_dir(working_dir);
                }
                Ok(command)
            }
            DevContainerHost::Remote(connection) => {
                let template = connection
                    .build_command(
                        Some(program.to_string()),
                        args,
                        &env.iter()
                            .map(|(key, value)| (key.clone(), value.clone()))
                            .collect(),
                        working_dir.map(|dir| dir.display().to_string()),
                        None,
                        remote::Interactive::No,
                    )
                    .map_err(|e| {
                        log::error!("Failed to build a remote `{program}` invocation: {e}");
                        DevContainerError::CommandFailed(program.to_string())
                    })?;
                let mut command = util::command::Command::new(&template.program);
                command.args(&template.args);
                command.envs(&template.env);
                Ok(command)
            }
        }
    }
}

/// Stands in for a connected host. Only command construction matters here,
/// and it mimics the POSIX SSH transport: a `cd` into the working directory
/// followed by single-quoted arguments, so a test can see whether quoting and
/// the working directory were applied by the transport rather than by the
/// caller.
#[cfg(test)]
pub(crate) struct FakeRemoteConnection {
    /// Every `(source, destination)` pair handed to [`RemoteConnection::upload_directory`].
    pub(crate) uploads: std::sync::Mutex<Vec<(std::path::PathBuf, String)>>,
    path_style: util::paths::PathStyle,
    connection_options: remote::RemoteConnectionOptions,
}

#[cfg(test)]
impl Default for FakeRemoteConnection {
    fn default() -> Self {
        Self::with_path_style(util::paths::PathStyle::Unix)
    }
}

#[cfg(test)]
impl FakeRemoteConnection {
    pub(crate) fn with_path_style(path_style: util::paths::PathStyle) -> Self {
        Self {
            uploads: std::sync::Mutex::new(Vec::new()),
            path_style,
            connection_options: remote::RemoteConnectionOptions::Ssh(Default::default()),
        }
    }

    pub(crate) fn with_connection_options(options: remote::RemoteConnectionOptions) -> Self {
        Self {
            connection_options: options,
            ..Self::default()
        }
    }
}

#[cfg(test)]
#[async_trait::async_trait(?Send)]
impl RemoteConnection for FakeRemoteConnection {
    fn start_proxy(
        &self,
        _unique_identifier: String,
        _reconnect: bool,
        _incoming_tx: futures::channel::mpsc::UnboundedSender<rpc::proto::Envelope>,
        _outgoing_rx: futures::channel::mpsc::UnboundedReceiver<rpc::proto::Envelope>,
        _connection_activity_tx: futures::channel::mpsc::Sender<()>,
        _delegate: Arc<dyn remote::RemoteClientDelegate>,
        _cx: &mut gpui::AsyncApp,
    ) -> gpui::Task<anyhow::Result<i32>> {
        gpui::Task::ready(Err(anyhow::anyhow!("not supported in tests")))
    }

    fn upload_directory(
        &self,
        src_path: std::path::PathBuf,
        dest_path: util::paths::RemotePathBuf,
        _cx: &gpui::App,
    ) -> gpui::Task<anyhow::Result<()>> {
        match self.uploads.lock() {
            Ok(mut uploads) => {
                uploads.push((src_path, dest_path.to_string()));
                gpui::Task::ready(Ok(()))
            }
            Err(e) => gpui::Task::ready(Err(anyhow::anyhow!("uploads lock poisoned: {e}"))),
        }
    }

    async fn kill(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        false
    }

    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &collections::HashMap<String, String>,
        working_dir: Option<String>,
        _port_forward: Option<(u16, String, u16)>,
        _interactive: remote::Interactive,
    ) -> anyhow::Result<remote::CommandTemplate> {
        let mut wrapped = vec!["host".to_string(), "--".to_string()];
        if let Some(working_dir) = working_dir {
            wrapped.push(format!("cd {working_dir} &&"));
        }
        wrapped.extend(env.iter().map(|(key, value)| format!("{key}={value}")));
        if let Some(program) = program {
            wrapped.push(format!("'{program}'"));
        }
        wrapped.extend(args.iter().map(|arg| format!("'{arg}'")));
        Ok(remote::CommandTemplate {
            program: "ssh".to_string(),
            args: wrapped,
            env: Default::default(),
        })
    }

    fn build_forward_ports_command(
        &self,
        _forwards: Vec<(u16, String, u16)>,
    ) -> anyhow::Result<remote::CommandTemplate> {
        Err(anyhow::anyhow!("not supported in tests"))
    }

    fn connection_options(&self) -> remote::RemoteConnectionOptions {
        self.connection_options.clone()
    }

    fn path_style(&self) -> util::paths::PathStyle {
        self.path_style
    }

    fn remote_platform(&self) -> remote::RemotePlatform {
        remote::RemotePlatform {
            os: remote::RemoteOs::Linux,
            arch: remote::RemoteArch::X86_64,
        }
    }

    fn remote_os_version(&self) -> Option<String> {
        None
    }

    fn shell(&self) -> String {
        "sh".to_string()
    }

    fn default_system_shell(&self) -> String {
        "sh".to_string()
    }

    fn has_wsl_interop(&self) -> bool {
        false
    }
}

/// Which machine's shell environment applies to a dev container's
/// configuration — the environment that `${localEnv:...}` resolves against and
/// that lifecycle commands inherit.
#[derive(Debug, PartialEq, Eq)]
enum EnvironmentSource {
    /// The machine running Zed.
    Local,
    /// The machine whose engine builds the container.
    Host,
    /// The host's environment cannot be reached, so no environment applies.
    /// Falling back to this machine's would be wrong: it would resolve
    /// `${localEnv:PATH}` against a filesystem the container never sees.
    Unavailable,
}

/// `has_remote_client` is whether a proto connection to the host's Zed server
/// is available; reading the host's environment is an RPC, so a remote host
/// without one has no environment to offer.
fn environment_source(host: &DevContainerHost, has_remote_client: bool) -> EnvironmentSource {
    match (host, has_remote_client) {
        (DevContainerHost::Local, _) => EnvironmentSource::Local,
        (DevContainerHost::Remote(_), true) => EnvironmentSource::Host,
        (DevContainerHost::Remote(_), false) => EnvironmentSource::Unavailable,
    }
}

pub struct DevContainerContext {
    pub project_directory: Arc<Path>,
    pub host: DevContainerHost,
    /// The connection to the host's Zed server, when the project is remote.
    /// Used for host operations that are proto requests rather than commands.
    pub remote_client: Option<Entity<RemoteClient>>,
    pub use_podman: bool,
    pub use_buildkit: Option<bool>,
    pub fs: Arc<dyn Fs>,
    pub http_client: Arc<dyn HttpClient>,
    pub environment: WeakEntity<ProjectEnvironment>,
}

impl DevContainerContext {
    pub fn from_workspace(workspace: &Workspace, cx: &App) -> Option<Self> {
        let project_directory = workspace.project().read(cx).active_project_directory(cx)?;
        let settings = DevContainerSettings::get_global(cx);
        let use_podman = settings.use_podman;
        let use_buildkit = settings.use_buildkit;
        let http_client = cx.http_client().clone();
        let fs = workspace.app_state().fs.clone();
        let environment = workspace.project().read(cx).environment().downgrade();
        let remote_client = workspace.project().read(cx).remote_client();
        // A remote project's files only exist on its host, so that is the only
        // machine whose engine can bind mount them.
        let host = match remote_client
            .as_ref()
            .and_then(|client| client.read(cx).remote_connection())
        {
            Some(connection) => DevContainerHost::Remote(connection),
            None => DevContainerHost::Local,
        };
        Some(Self {
            project_directory,
            host,
            remote_client,
            use_podman,
            use_buildkit,
            fs,
            http_client,
            environment,
        })
    }

    /// The shell environment the configuration is resolved against.
    ///
    /// It has to come from the machine the container is built on: a remote dev
    /// container's `${localEnv:...}` references and lifecycle commands see the
    /// host's environment, not this machine's.
    pub async fn environment(&self, cx: &mut impl AppContext) -> HashMap<String, String> {
        let task = match environment_source(&self.host, self.remote_client.is_some()) {
            EnvironmentSource::Local => self.environment.update(cx, |this, cx| {
                this.local_directory_environment(&Shell::System, self.project_directory.clone(), cx)
            }),
            EnvironmentSource::Host => {
                let Some(remote_client) = self.remote_client.clone() else {
                    return HashMap::default();
                };
                self.environment.update(cx, |this, cx| {
                    this.remote_directory_environment(
                        &Shell::System,
                        self.project_directory.clone(),
                        remote_client,
                        cx,
                    )
                })
            }
            EnvironmentSource::Unavailable => {
                log::warn!(
                    "No connection to the dev container host, so its shell environment is unavailable"
                );
                return HashMap::default();
            }
        };
        let Ok(task) = task else {
            return HashMap::default();
        };
        task.await
            .map(|env| env.into_iter().collect::<std::collections::HashMap<_, _>>())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod environment_source_tests {
    use super::{DevContainerHost, EnvironmentSource, FakeRemoteConnection, environment_source};
    use std::sync::Arc;

    /// A remote dev container's configuration must resolve against the host's
    /// environment. Falling back to this machine's would silently substitute
    /// paths and versions that do not exist over there.
    #[test]
    fn environment_follows_the_host() {
        assert_eq!(
            environment_source(&DevContainerHost::Local, false),
            EnvironmentSource::Local
        );
        assert_eq!(
            environment_source(&DevContainerHost::Local, true),
            EnvironmentSource::Local,
            "an open remote project does not move a local container's environment"
        );

        let remote = DevContainerHost::Remote(Arc::new(FakeRemoteConnection::default()));
        assert_eq!(environment_source(&remote, true), EnvironmentSource::Host);
        assert_eq!(
            environment_source(&remote, false),
            EnvironmentSource::Unavailable,
            "reading the host environment is an RPC, so it needs the server connection"
        );
    }

    /// Provisioning and connecting are separate steps. If the connection does
    /// not name the machine the container was built on, it reaches this
    /// machine's daemon and finds nothing.
    #[test]
    fn the_connection_names_the_machine_the_container_was_built_on() {
        assert_eq!(
            DevContainerHost::Local.docker_host().unwrap(),
            remote::DockerHost::Local
        );

        let host = DevContainerHost::Remote(Arc::new(FakeRemoteConnection::default()));
        assert!(matches!(
            host.docker_host().unwrap(),
            remote::DockerHost::Ssh(_)
        ));

        let distro = remote::WslConnectionOptions {
            distro_name: "ubuntu".to_string(),
            user: Some("zed".to_string()),
        };
        let host =
            DevContainerHost::Remote(Arc::new(FakeRemoteConnection::with_connection_options(
                remote::RemoteConnectionOptions::Wsl(distro.clone()),
            )));
        assert_eq!(
            host.docker_host().unwrap(),
            remote::DockerHost::Wsl(distro),
            "a project in a WSL distro is built by that distro's engine"
        );
    }

    /// The engine is on whichever machine will build the container, so the
    /// probe for it has to travel the same path every other engine command
    /// does rather than running here.
    #[test]
    fn the_engine_is_probed_on_the_host() {
        let host = DevContainerHost::Remote(Arc::new(FakeRemoteConnection::default()));
        let command = host
            .command(
                "docker",
                &["--version".to_string()],
                &std::collections::HashMap::default(),
                None,
            )
            .expect("a remote host can build a command");

        assert_ne!(
            command.get_program(),
            "docker",
            "the probe must be wrapped by the transport, not run locally"
        );
    }
}

#[derive(RegisterSetting)]
struct DevContainerSettings {
    use_podman: bool,
    use_buildkit: Option<bool>,
}

pub fn use_podman(cx: &App) -> bool {
    DevContainerSettings::get_global(cx).use_podman
}

impl Settings for DevContainerSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            use_podman: content.remote.use_podman.unwrap_or(false),
            use_buildkit: content.remote.dev_container_use_buildkit,
        }
    }
}

#[derive(PartialEq, Clone, Deserialize, Default, Action)]
#[action(namespace = projects)]
#[serde(deny_unknown_fields)]
struct InitializeDevContainer;

pub fn init(cx: &mut App) {
    cx.on_action(|_: &InitializeDevContainer, cx| {
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
    current_option_index: usize,
    current_option: Option<TemplateOptionSelection>,
    features_selected: HashSet<DevContainerFeature>,
}

#[derive(Clone)]
struct FeatureEntry {
    feature: DevContainerFeature,
    toggle_state: ToggleState,
}

#[derive(Clone)]
struct TemplateOptionSelection {
    option_name: String,
    description: String,
    navigable_options: Vec<(String, NavigableEntry)>,
}

impl Eq for TemplateEntry {}
impl PartialEq for TemplateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.template == other.template
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
        self.feature == other.feature
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
    TemplateQueryReturned(Result<Vec<TemplateEntry>, String>),
    QueryingFeatures(TemplateEntry),
    FeaturesQueryReturned(TemplateEntry),
    UserOptionsSpecifying(TemplateEntry),
    ConfirmingWriteDevContainer(TemplateEntry),
    TemplateWriteFailed(DevContainerError),
}

#[derive(Debug, Clone)]
enum DevContainerMessage {
    SearchTemplates,
    TemplatesRetrieved(Vec<DevContainerTemplate>),
    ErrorRetrievingTemplates(String),
    TemplateSelected(TemplateEntry),
    TemplateOptionsSpecified(TemplateEntry),
    TemplateOptionsCompleted(TemplateEntry),
    FeaturesRetrieved(Vec<DevContainerFeature>),
    FeaturesSelected(TemplateEntry),
    NeedConfirmWriteDevContainer(TemplateEntry),
    ConfirmWriteDevContainer(TemplateEntry),
    FailedToWriteTemplate(DevContainerError),
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

    fn name() -> &'static str {
        "dev container template picker"
    }

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
                template_entry
                    .template
                    .id
                    .to_lowercase()
                    .contains(&query.to_lowercase())
                    || template_entry
                        .template
                        .name
                        .to_lowercase()
                        .contains(&query.to_lowercase())
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

        if self.matching_indices.is_empty() {
            return;
        }
        self.stateful_modal
            .update(cx, |modal, cx| {
                let Some(confirmed_entry) = self
                    .matching_indices
                    .get(self.selected_index)
                    .and_then(|ix| self.candidate_templates.get(*ix))
                else {
                    log::error!("Selected index not in range of known matches");
                    return;
                };
                fun(confirmed_entry.clone(), modal, window, cx);
            })
            .ok();
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        self.stateful_modal
            .update(cx, |modal, cx| {
                modal.dismiss(&menu::Cancel, window, cx);
            })
            .ok();
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
            ListItem::new("li-template-match")
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
                                .map(|kb| kb.size(rems_from_px(12_f32))),
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

    fn name() -> &'static str {
        "dev container feature picker"
    }

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
                feature_entry
                    .feature
                    .id
                    .to_lowercase()
                    .contains(&query.to_lowercase())
                    || feature_entry
                        .feature
                        .name
                        .to_lowercase()
                        .contains(&query.to_lowercase())
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
                .ok();
        } else {
            if self.matching_indices.is_empty() {
                return;
            }
            let Some(current) = self
                .matching_indices
                .get(self.selected_index)
                .and_then(|ix| self.candidate_features.get_mut(*ix))
            else {
                log::error!("Selected index not in range of matches");
                return;
            };
            current.toggle_state = match current.toggle_state {
                ToggleState::Selected => {
                    self.template_entry
                        .features_selected
                        .remove(&current.feature);
                    ToggleState::Unselected
                }
                _ => {
                    self.template_entry
                        .features_selected
                        .insert(current.feature.clone());
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
            .ok();
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
                .child(Label::new(feature_entry.feature.name))
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
                                .map(|kb| kb.size(rems_from_px(12_f32))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .child(
                    Button::new("run-action-secondary", "Confirm Selections")
                        .key_binding(
                            KeyBinding::for_action(&menu::SecondaryConfirm, cx)
                                .map(|kb| kb.size(rems_from_px(12_f32))),
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
                .p_1()
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
                                .start_slot(
                                    Icon::new(IconName::MagnifyingGlass).color(Color::Muted),
                                )
                                .toggle_state(
                                    self.confirm_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(
                                        DevContainerMessage::SearchTemplates,
                                        window,
                                        cx,
                                    );
                                    cx.notify();
                                }))
                                .child(Label::new("Search for Dev Container Templates")),
                        ),
                )
                .into_any_element(),
        );
        view = view.entry(self.confirm_entry.clone());
        view.render(window, cx).into_any_element()
    }

    fn render_error(
        &self,
        error_title: String,
        error: impl Display,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .p_1()
            .child(div().track_focus(&self.focus_handle).child(
                ModalHeader::new().child(Headline::new(error_title).size(HeadlineSize::XSmall)),
            ))
            .child(ListSeparator)
            .child(
                v_flex()
                    .child(Label::new(format!("{}", error)))
                    .whitespace_normal(),
            )
            .into_any_element()
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
        let Some(next_option_entries) = &template_entry.current_option else {
            return div().into_any_element();
        };
        let mut view = Navigable::new(
            div()
                .child(
                    div()
                        .id("title")
                        .tooltip(Tooltip::text(next_option_entries.description.clone()))
                        .track_focus(&self.focus_handle)
                        .child(
                            ModalHeader::new()
                                .child(
                                    Headline::new("Template Option: ").size(HeadlineSize::XSmall),
                                )
                                .child(
                                    Headline::new(&next_option_entries.option_name)
                                        .size(HeadlineSize::XSmall),
                                ),
                        ),
                )
                .child(ListSeparator)
                .children(
                    next_option_entries
                        .navigable_options
                        .iter()
                        .map(|(option, entry)| {
                            div()
                                .id(format!("li-parent-{}", option))
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
                                    ListItem::new(format!("li-option-{}", option))
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .toggle_state(
                                            entry.focus_handle.contains_focused(window, cx),
                                        )
                                        .on_click({
                                            let mut template = template_entry.clone();
                                            template.options_selected.insert(
                                                next_option_entries.option_name.clone(),
                                                option.clone(),
                                            );
                                            cx.listener(move |this, _, window, cx| {
                                                this.accept_message(
                                                    DevContainerMessage::TemplateOptionsSpecified(
                                                        template.clone(),
                                                    ),
                                                    window,
                                                    cx,
                                                );
                                                cx.notify();
                                            })
                                        })
                                        .child(Label::new(option)),
                                )
                        }),
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
                                .start_slot(Icon::new(IconName::Return).color(Color::Muted))
                                .toggle_state(
                                    self.back_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(DevContainerMessage::GoBack, window, cx);
                                    cx.notify();
                                }))
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
                        ModalHeader::new()
                            .icon(Icon::new(IconName::Warning).color(Color::Warning))
                            .child(
                                Headline::new("Overwrite Existing Configuration?")
                                    .size(HeadlineSize::XSmall),
                            ),
                    ),
                )
                .child(
                    div()
                        .track_focus(&self.confirm_entry.focus_handle)
                        .on_action({
                            let template = template_entry.clone();
                            cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                this.accept_message(
                                    DevContainerMessage::ConfirmWriteDevContainer(template.clone()),
                                    window,
                                    cx,
                                );
                            })
                        })
                        .child(
                            ListItem::new("li-search-containers")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Check).color(Color::Muted))
                                .toggle_state(
                                    self.confirm_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.accept_message(
                                        DevContainerMessage::ConfirmWriteDevContainer(
                                            template_entry.clone(),
                                        ),
                                        window,
                                        cx,
                                    );
                                    cx.notify();
                                }))
                                .child(Label::new("Overwrite")),
                        ),
                )
                .child(
                    div()
                        .track_focus(&self.back_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.dismiss(&menu::Cancel, window, cx);
                        }))
                        .child(
                            ListItem::new("li-goback")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::XCircle).color(Color::Muted))
                                .toggle_state(
                                    self.back_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.dismiss(&menu::Cancel, window, cx);
                                    cx.notify();
                                }))
                                .child(Label::new("Cancel")),
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
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(DevContainerMessage::GoBack, window, cx);
                                    cx.notify();
                                }))
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
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(DevContainerMessage::GoBack, window, cx);
                                    cx.notify();
                                }))
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
            DevContainerState::TemplateWriteFailed(dev_container_error) => self.render_error(
                "Error Creating Dev Container Definition".to_string(),
                dev_container_error,
                window,
                cx,
            ),
            DevContainerState::TemplateQueryReturned(Err(e)) => {
                self.render_error("Error Retrieving Templates".to_string(), e, window, cx)
            }
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
                    let Ok(client) = cx.update(|_, cx| cx.http_client()) else {
                        return;
                    };
                    match get_ghcr_templates(client).await {
                        Ok(templates) => {
                            let message =
                                DevContainerMessage::TemplatesRetrieved(templates.templates);
                            this.update_in(cx, |this, window, cx| {
                                this.accept_message(message, window, cx);
                            })
                            .ok();
                        }
                        Err(e) => {
                            let message = DevContainerMessage::ErrorRetrievingTemplates(e);
                            this.update_in(cx, |this, window, cx| {
                                this.accept_message(message, window, cx);
                            })
                            .ok();
                        }
                    }
                })
                .detach();
                Some(DevContainerState::QueryingTemplates)
            }
            DevContainerMessage::ErrorRetrievingTemplates(message) => {
                Some(DevContainerState::TemplateQueryReturned(Err(message)))
            }
            DevContainerMessage::GoBack => match &self.state {
                DevContainerState::Initial => Some(DevContainerState::Initial),
                DevContainerState::QueryingTemplates => Some(DevContainerState::Initial),
                DevContainerState::UserOptionsSpecifying(template_entry) => {
                    if template_entry.current_option_index <= 1 {
                        self.accept_message(DevContainerMessage::SearchTemplates, window, cx);
                    } else {
                        let mut template_entry = template_entry.clone();
                        template_entry.current_option_index =
                            template_entry.current_option_index.saturating_sub(2);
                        self.accept_message(
                            DevContainerMessage::TemplateOptionsSpecified(template_entry),
                            window,
                            cx,
                        );
                    }
                    None
                }
                _ => Some(DevContainerState::Initial),
            },
            DevContainerMessage::TemplatesRetrieved(items) => {
                let items = items
                    .into_iter()
                    .map(|item| TemplateEntry {
                        template: item,
                        options_selected: HashMap::new(),
                        current_option_index: 0,
                        current_option: None,
                        features_selected: HashSet::new(),
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

                    let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).embedded());
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

                let Some((first_option_name, first_option)) =
                    options.get(template_entry.current_option_index)
                else {
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

                template_entry.current_option_index += 1;
                template_entry.current_option = Some(TemplateOptionSelection {
                    option_name: (*first_option_name).clone(),
                    description: first_option
                        .description
                        .clone()
                        .unwrap_or_else(|| "".to_string()),
                    navigable_options: next_option_entries,
                });

                Some(DevContainerState::UserOptionsSpecifying(template_entry))
            }
            DevContainerMessage::TemplateOptionsSpecified(mut template_entry) => {
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

                let Some((next_option_name, next_option)) =
                    options.get(template_entry.current_option_index)
                else {
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

                template_entry.current_option_index += 1;
                template_entry.current_option = Some(TemplateOptionSelection {
                    option_name: (*next_option_name).clone(),
                    description: next_option
                        .description
                        .clone()
                        .unwrap_or_else(|| "".to_string()),
                    navigable_options: next_option_entries,
                });

                Some(DevContainerState::UserOptionsSpecifying(template_entry))
            }
            DevContainerMessage::TemplateOptionsCompleted(template_entry) => {
                cx.spawn_in(window, async move |this, cx| {
                    let Ok(client) = cx.update(|_, cx| cx.http_client()) else {
                        return;
                    };
                    let Some(features) = get_ghcr_features(client).await.log_err() else {
                        return;
                    };
                    let message = DevContainerMessage::FeaturesRetrieved(features.features);
                    this.update_in(cx, |this, window, cx| {
                        this.accept_message(message, window, cx);
                    })
                    .ok();
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
                        features,
                        template_entry.clone(),
                        Box::new(|entry, this, window, cx| {
                            this.accept_message(
                                DevContainerMessage::FeaturesSelected(entry),
                                window,
                                cx,
                            );
                        }),
                    );

                    let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).embedded());
                    self.features_picker = Some(picker);
                    Some(DevContainerState::FeaturesQueryReturned(template_entry))
                } else {
                    None
                }
            }
            DevContainerMessage::FeaturesSelected(template_entry) => {
                if let Some(workspace) = self.workspace.upgrade() {
                    dispatch_apply_templates(template_entry, workspace, window, true, cx);
                }

                None
            }
            DevContainerMessage::NeedConfirmWriteDevContainer(template_entry) => Some(
                DevContainerState::ConfirmingWriteDevContainer(template_entry),
            ),
            DevContainerMessage::ConfirmWriteDevContainer(template_entry) => {
                if let Some(workspace) = self.workspace.upgrade() {
                    dispatch_apply_templates(template_entry, workspace, window, false, cx);
                }
                None
            }
            DevContainerMessage::FailedToWriteTemplate(error) => {
                Some(DevContainerState::TemplateWriteFailed(error))
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

fn ghcr_registry() -> &'static str {
    "ghcr.io"
}

fn devcontainer_templates_repository() -> &'static str {
    "devcontainers/templates"
}

fn devcontainer_features_repository() -> &'static str {
    "devcontainers/features"
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerTemplatesResponse {
    templates: Vec<DevContainerTemplate>,
}

fn dispatch_apply_templates(
    template_entry: TemplateEntry,
    workspace: Entity<Workspace>,
    window: &mut Window,
    check_for_existing: bool,
    cx: &mut Context<DevContainerModal>,
) {
    cx.spawn_in(window, async move |this, cx| {
        let Some((tree_id, context)) = workspace.update(cx, |workspace, cx| {
            let worktree = workspace
                .project()
                .read(cx)
                .visible_worktrees(cx)
                .find_map(|tree| {
                    tree.read(cx)
                        .root_entry()?
                        .is_dir()
                        .then_some(tree.read(cx))
                });
            let tree_id = worktree.map(|w| w.id())?;
            let context = DevContainerContext::from_workspace(workspace, cx)?;
            Some((tree_id, context))
        }) else {
            return;
        };

        let environment = context.environment(cx).await;

        {
            if check_for_existing
                && read_default_devcontainer_configuration(&context, environment)
                    .await
                    .is_ok()
            {
                this.update_in(cx, |this, window, cx| {
                    this.accept_message(
                        DevContainerMessage::NeedConfirmWriteDevContainer(template_entry),
                        window,
                        cx,
                    );
                })
                .ok();
                return;
            }

            let worktree = workspace.read_with(cx, |workspace, cx| {
                workspace.project().read(cx).worktree_for_id(tree_id, cx)
            });

            let files = match apply_devcontainer_template(
                worktree.unwrap(),
                &template_entry.template,
                &template_entry.options_selected,
                &template_entry.features_selected,
                &context,
                cx,
            )
            .await
            {
                Ok(files) => files,
                Err(e) => {
                    this.update_in(cx, |this, window, cx| {
                        this.accept_message(
                            DevContainerMessage::FailedToWriteTemplate(
                                DevContainerError::DevContainerTemplateApplyFailed(e.to_string()),
                            ),
                            window,
                            cx,
                        );
                    })
                    .ok();
                    return;
                }
            };

            if files.project_files.contains(&Arc::from(
                RelPath::from_unix_str(".devcontainer/devcontainer.json").unwrap(),
            )) {
                let Some(workspace_task) = workspace
                    .update_in(cx, |workspace, window, cx| {
                        let Ok(path) = RelPath::from_unix_str(".devcontainer/devcontainer.json")
                        else {
                            return Task::ready(Err(anyhow!(
                                "Couldn't create path for .devcontainer/devcontainer.json"
                            )));
                        };
                        workspace.open_path((tree_id, path), None, true, window, cx)
                    })
                    .ok()
                else {
                    return;
                };

                workspace_task.await.log_err();
            }
            this.update_in(cx, |this, window, cx| {
                this.dismiss(&menu::Cancel, window, cx);
            })
            .ok();
        }
    })
    .detach();
}

async fn get_ghcr_templates(
    client: Arc<dyn HttpClient>,
) -> Result<DevContainerTemplatesResponse, String> {
    let token = get_oci_token(
        ghcr_registry(),
        devcontainer_templates_repository(),
        &client,
    )
    .await?;
    let manifest = get_latest_oci_manifest(
        &token.token,
        ghcr_registry(),
        devcontainer_templates_repository(),
        &client,
        None,
    )
    .await?;

    let mut template_response: DevContainerTemplatesResponse = get_deserializable_oci_blob(
        &token.token,
        ghcr_registry(),
        devcontainer_templates_repository(),
        &manifest.layers[0].digest,
        &client,
    )
    .await?;

    for template in &mut template_response.templates {
        template.source_repository = Some(format!(
            "{}/{}",
            ghcr_registry(),
            devcontainer_templates_repository()
        ));
    }
    Ok(template_response)
}

async fn get_ghcr_features(
    client: Arc<dyn HttpClient>,
) -> Result<DevContainerFeaturesResponse, String> {
    let token = get_oci_token(
        ghcr_registry(),
        devcontainer_templates_repository(),
        &client,
    )
    .await?;

    let manifest = get_latest_oci_manifest(
        &token.token,
        ghcr_registry(),
        devcontainer_features_repository(),
        &client,
        None,
    )
    .await?;

    let mut features_response: DevContainerFeaturesResponse = get_deserializable_oci_blob(
        &token.token,
        ghcr_registry(),
        devcontainer_features_repository(),
        &manifest.layers[0].digest,
        &client,
    )
    .await?;

    for feature in &mut features_response.features {
        feature.source_repository = Some(format!(
            "{}/{}",
            ghcr_registry(),
            devcontainer_features_repository()
        ));
    }
    Ok(features_response)
}

#[cfg(test)]
mod tests {
    use http_client::{FakeHttpClient, anyhow};

    use crate::{
        DevContainerTemplatesResponse, devcontainer_templates_repository,
        get_deserializable_oci_blob, ghcr_registry,
    };

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
        let response: Result<DevContainerTemplatesResponse, String> = get_deserializable_oci_blob(
            "",
            ghcr_registry(),
            devcontainer_templates_repository(),
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09",
            &client,
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.templates.len(), 1);
        assert_eq!(response.templates[0].name, "Alpine");
    }
}
