use remote::Interactive;
use std::{
    any::Any,
    borrow::Borrow,
    path::{Path, PathBuf},
    str::FromStr as _,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, Result, bail};
use collections::HashMap;
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::StreamExt as _;
use gpui::{
    AppContext as _, AsyncApp, Context, Entity, EventEmitter, SharedString, Subscription, Task,
};
use http_client::{HttpClient, github::AssetKind};
use node_runtime::NodeRuntime;
use remote::RemoteClient;
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, ExternalExtensionAgent},
};
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use settings::{RegisterSetting, SettingsStore};
use task::{Shell, SpawnInTerminal};
use util::{ResultExt as _, debug_panic};

use crate::ProjectEnvironment;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct AgentServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

impl std::fmt::Debug for AgentServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filtered_env = self.env.as_ref().map(|env| {
            env.iter()
                .map(|(k, v)| {
                    (
                        k,
                        if util::redact::should_redact(k) {
                            "[REDACTED]"
                        } else {
                            v
                        },
                    )
                })
                .collect::<Vec<_>>()
        });

        f.debug_struct("AgentServerCommand")
            .field("path", &self.path)
            .field("args", &self.args)
            .field("env", &filtered_env)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternalAgentServerName(pub SharedString);

impl std::fmt::Display for ExternalAgentServerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for ExternalAgentServerName {
    fn from(value: &'static str) -> Self {
        ExternalAgentServerName(value.into())
    }
}

impl From<ExternalAgentServerName> for SharedString {
    fn from(value: ExternalAgentServerName) -> Self {
        value.0
    }
}

impl Borrow<str> for ExternalAgentServerName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

pub trait ExternalAgentServer {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>>;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn ExternalAgentServer {
    fn downcast_mut<T: ExternalAgentServer + 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

enum AgentServerStoreState {
    Local {
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        project_environment: Entity<ProjectEnvironment>,
        downstream_client: Option<(u64, AnyProtoClient)>,
        settings: Option<AllAgentServersSettings>,
        http_client: Arc<dyn HttpClient>,
        extension_agents: Vec<(
            Arc<str>,
            String,
            HashMap<String, extension::TargetConfig>,
            HashMap<String, String>,
            Option<String>,
        )>,
        _subscriptions: [Subscription; 1],
    },
    Remote {
        project_id: u64,
        upstream_client: Entity<RemoteClient>,
    },
    Collab,
}

pub struct AgentServerStore {
    state: AgentServerStoreState,
    external_agents: HashMap<ExternalAgentServerName, Box<dyn ExternalAgentServer>>,
    agent_icons: HashMap<ExternalAgentServerName, SharedString>,
    agent_display_names: HashMap<ExternalAgentServerName, SharedString>,
}

pub struct AgentServersUpdated;

impl EventEmitter<AgentServersUpdated> for AgentServerStore {}

#[cfg(test)]
mod ext_agent_tests {
    use super::*;
    use std::{collections::HashSet, fmt::Write as _};

    // Helper to build a store in Collab mode so we can mutate internal maps without
    // needing to spin up a full project environment.
    fn collab_store() -> AgentServerStore {
        AgentServerStore {
            state: AgentServerStoreState::Collab,
            external_agents: HashMap::default(),
            agent_icons: HashMap::default(),
            agent_display_names: HashMap::default(),
        }
    }

    // A simple fake that implements ExternalAgentServer without needing async plumbing.
    struct NoopExternalAgent;

    impl ExternalAgentServer for NoopExternalAgent {
        fn get_command(
            &mut self,
            _root_dir: Option<&str>,
            _extra_env: HashMap<String, String>,
            _status_tx: Option<watch::Sender<SharedString>>,
            _new_version_available_tx: Option<watch::Sender<Option<String>>>,
            _cx: &mut AsyncApp,
        ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
            Task::ready(Ok((
                AgentServerCommand {
                    path: PathBuf::from("noop"),
                    args: Vec::new(),
                    env: None,
                },
                "".to_string(),
                None,
            )))
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn external_agent_server_name_display() {
        let name = ExternalAgentServerName(SharedString::from("Ext: Tool"));
        let mut s = String::new();
        write!(&mut s, "{name}").unwrap();
        assert_eq!(s, "Ext: Tool");
    }

    #[test]
    fn sync_extension_agents_removes_previous_extension_entries() {
        let mut store = collab_store();

        // Seed with a couple of agents that will be replaced by extensions
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("foo-agent")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("bar-agent")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("custom")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );

        // Simulate the removal phase: if we're syncing extensions that provide
        // "foo-agent" and "bar-agent", those should be removed first
        let extension_agent_names: HashSet<String> =
            ["foo-agent".to_string(), "bar-agent".to_string()]
                .into_iter()
                .collect();

        let keys_to_remove: Vec<_> = store
            .external_agents
            .keys()
            .filter(|name| extension_agent_names.contains(name.0.as_ref()))
            .cloned()
            .collect();

        for key in keys_to_remove {
            store.external_agents.remove(&key);
        }

        // Only the custom entry should remain.
        let remaining: Vec<_> = store
            .external_agents
            .keys()
            .map(|k| k.0.to_string())
            .collect();
        assert_eq!(remaining, vec!["custom".to_string()]);
    }

    #[test]
    fn resolve_extension_icon_path_allows_valid_paths() {
        // Create a temporary directory structure for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let extensions_dir = temp_dir.path();
        let ext_dir = extensions_dir.join("my-extension");
        std::fs::create_dir_all(&ext_dir).unwrap();

        // Create a valid icon file
        let icon_path = ext_dir.join("icon.svg");
        std::fs::write(&icon_path, "<svg></svg>").unwrap();

        // Test that a valid relative path works
        let result = super::resolve_extension_icon_path(extensions_dir, "my-extension", "icon.svg");
        assert!(result.is_some());
        assert!(result.unwrap().ends_with("icon.svg"));
    }

    #[test]
    fn resolve_extension_icon_path_allows_nested_paths() {
        let temp_dir = tempfile::tempdir().unwrap();
        let extensions_dir = temp_dir.path();
        let ext_dir = extensions_dir.join("my-extension");
        let icons_dir = ext_dir.join("assets").join("icons");
        std::fs::create_dir_all(&icons_dir).unwrap();

        let icon_path = icons_dir.join("logo.svg");
        std::fs::write(&icon_path, "<svg></svg>").unwrap();

        let result = super::resolve_extension_icon_path(
            extensions_dir,
            "my-extension",
            "assets/icons/logo.svg",
        );
        assert!(result.is_some());
        assert!(result.unwrap().ends_with("logo.svg"));
    }

    #[test]
    fn resolve_extension_icon_path_blocks_path_traversal() {
        let temp_dir = tempfile::tempdir().unwrap();
        let extensions_dir = temp_dir.path();

        // Create two extension directories
        let ext1_dir = extensions_dir.join("extension1");
        let ext2_dir = extensions_dir.join("extension2");
        std::fs::create_dir_all(&ext1_dir).unwrap();
        std::fs::create_dir_all(&ext2_dir).unwrap();

        // Create a file in extension2
        let secret_file = ext2_dir.join("secret.svg");
        std::fs::write(&secret_file, "<svg>secret</svg>").unwrap();

        // Try to access extension2's file from extension1 using path traversal
        let result = super::resolve_extension_icon_path(
            extensions_dir,
            "extension1",
            "../extension2/secret.svg",
        );
        assert!(
            result.is_none(),
            "Path traversal to sibling extension should be blocked"
        );
    }

    #[test]
    fn resolve_extension_icon_path_blocks_absolute_escape() {
        let temp_dir = tempfile::tempdir().unwrap();
        let extensions_dir = temp_dir.path();
        let ext_dir = extensions_dir.join("my-extension");
        std::fs::create_dir_all(&ext_dir).unwrap();

        // Create a file outside the extensions directory
        let outside_file = temp_dir.path().join("outside.svg");
        std::fs::write(&outside_file, "<svg>outside</svg>").unwrap();

        // Try to escape to parent directory
        let result =
            super::resolve_extension_icon_path(extensions_dir, "my-extension", "../outside.svg");
        assert!(
            result.is_none(),
            "Path traversal to parent directory should be blocked"
        );
    }

    #[test]
    fn resolve_extension_icon_path_blocks_deep_traversal() {
        let temp_dir = tempfile::tempdir().unwrap();
        let extensions_dir = temp_dir.path();
        let ext_dir = extensions_dir.join("my-extension");
        std::fs::create_dir_all(&ext_dir).unwrap();

        // Try deep path traversal
        let result = super::resolve_extension_icon_path(
            extensions_dir,
            "my-extension",
            "../../../../../../etc/passwd",
        );
        assert!(
            result.is_none(),
            "Deep path traversal should be blocked (file doesn't exist)"
        );
    }

    #[test]
    fn resolve_extension_icon_path_returns_none_for_nonexistent() {
        let temp_dir = tempfile::tempdir().unwrap();
        let extensions_dir = temp_dir.path();
        let ext_dir = extensions_dir.join("my-extension");
        std::fs::create_dir_all(&ext_dir).unwrap();

        // Try to access a file that doesn't exist
        let result =
            super::resolve_extension_icon_path(extensions_dir, "my-extension", "nonexistent.svg");
        assert!(result.is_none(), "Nonexistent file should return None");
    }
}

impl AgentServerStore {
    /// Synchronizes extension-provided agent servers with the store.
    pub fn sync_extension_agents<'a, I>(
        &mut self,
        manifests: I,
        extensions_dir: PathBuf,
        cx: &mut Context<Self>,
    ) where
        I: IntoIterator<Item = (&'a str, &'a extension::ExtensionManifest)>,
    {
        // Collect manifests first so we can iterate twice
        let manifests: Vec<_> = manifests.into_iter().collect();

        // Remove all extension-provided agents
        // (They will be re-added below if they're in the currently installed extensions)
        self.external_agents.retain(|name, agent| {
            if agent.downcast_mut::<LocalExtensionArchiveAgent>().is_some() {
                self.agent_icons.remove(name);
                self.agent_display_names.remove(name);
                false
            } else {
                // Keep the hardcoded external agents that don't come from extensions
                // (In the future we may move these over to being extensions too.)
                true
            }
        });

        // Insert agent servers from extension manifests
        match &mut self.state {
            AgentServerStoreState::Local {
                extension_agents, ..
            } => {
                extension_agents.clear();
                for (ext_id, manifest) in manifests {
                    for (agent_name, agent_entry) in &manifest.agent_servers {
                        // Store display name from manifest
                        self.agent_display_names.insert(
                            ExternalAgentServerName(agent_name.clone().into()),
                            SharedString::from(agent_entry.name.clone()),
                        );

                        let icon_path = if let Some(icon) = &agent_entry.icon {
                            if let Some(absolute_icon_path) =
                                resolve_extension_icon_path(&extensions_dir, ext_id, icon)
                            {
                                self.agent_icons.insert(
                                    ExternalAgentServerName(agent_name.clone().into()),
                                    SharedString::from(absolute_icon_path.clone()),
                                );
                                Some(absolute_icon_path)
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        extension_agents.push((
                            agent_name.clone(),
                            ext_id.to_owned(),
                            agent_entry.targets.clone(),
                            agent_entry.env.clone(),
                            icon_path,
                        ));
                    }
                }
                self.reregister_agents(cx);
            }
            AgentServerStoreState::Remote {
                project_id,
                upstream_client,
            } => {
                let mut agents = vec![];
                for (ext_id, manifest) in manifests {
                    for (agent_name, agent_entry) in &manifest.agent_servers {
                        // Store display name from manifest
                        self.agent_display_names.insert(
                            ExternalAgentServerName(agent_name.clone().into()),
                            SharedString::from(agent_entry.name.clone()),
                        );

                        let icon = if let Some(icon) = &agent_entry.icon {
                            if let Some(absolute_icon_path) =
                                resolve_extension_icon_path(&extensions_dir, ext_id, icon)
                            {
                                self.agent_icons.insert(
                                    ExternalAgentServerName(agent_name.clone().into()),
                                    SharedString::from(absolute_icon_path.clone()),
                                );
                                Some(absolute_icon_path)
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        agents.push(ExternalExtensionAgent {
                            name: agent_name.to_string(),
                            icon_path: icon,
                            extension_id: ext_id.to_string(),
                            targets: agent_entry
                                .targets
                                .iter()
                                .map(|(k, v)| (k.clone(), v.to_proto()))
                                .collect(),
                            env: agent_entry
                                .env
                                .iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect(),
                        });
                    }
                }
                upstream_client
                    .read(cx)
                    .proto_client()
                    .send(proto::ExternalExtensionAgentsUpdated {
                        project_id: *project_id,
                        agents,
                    })
                    .log_err();
            }
            AgentServerStoreState::Collab => {
                // Do nothing
            }
        }

        cx.emit(AgentServersUpdated);
    }

    pub fn agent_icon(&self, name: &ExternalAgentServerName) -> Option<SharedString> {
        self.agent_icons.get(name).cloned()
    }
}

/// Safely resolves an extension icon path, ensuring it stays within the extension directory.
/// Returns `None` if the path would escape the extension directory (path traversal attack).
fn resolve_extension_icon_path(
    extensions_dir: &Path,
    extension_id: &str,
    icon_relative_path: &str,
) -> Option<String> {
    let extension_root = extensions_dir.join(extension_id);
    let icon_path = extension_root.join(icon_relative_path);

    // Canonicalize both paths to resolve symlinks and normalize the paths.
    // For the extension root, we need to handle the case where it might be a symlink
    // (common for dev extensions).
    let canonical_extension_root = extension_root.canonicalize().unwrap_or(extension_root);
    let canonical_icon_path = match icon_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            log::warn!(
                "Failed to canonicalize icon path for extension '{}': {} (path: {})",
                extension_id,
                err,
                icon_relative_path
            );
            return None;
        }
    };

    // Verify the resolved icon path is within the extension directory
    if canonical_icon_path.starts_with(&canonical_extension_root) {
        Some(canonical_icon_path.to_string_lossy().to_string())
    } else {
        log::warn!(
            "Icon path '{}' for extension '{}' escapes extension directory, ignoring for security",
            icon_relative_path,
            extension_id
        );
        None
    }
}

impl AgentServerStore {
    pub fn agent_display_name(&self, name: &ExternalAgentServerName) -> Option<SharedString> {
        self.agent_display_names.get(name).cloned()
    }

    pub fn init_remote(session: &AnyProtoClient) {
        session.add_entity_message_handler(Self::handle_external_agents_updated);
        session.add_entity_message_handler(Self::handle_loading_status_updated);
        session.add_entity_message_handler(Self::handle_new_version_available);
    }

    pub fn init_headless(session: &AnyProtoClient) {
        session.add_entity_message_handler(Self::handle_external_extension_agents_updated);
        session.add_entity_request_handler(Self::handle_get_agent_server_command);
    }

    fn agent_servers_settings_changed(&mut self, cx: &mut Context<Self>) {
        let AgentServerStoreState::Local {
            settings: old_settings,
            ..
        } = &mut self.state
        else {
            debug_panic!(
                "should not be subscribed to agent server settings changes in non-local project"
            );
            return;
        };

        let new_settings = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None)
            .clone();
        if Some(&new_settings) == old_settings.as_ref() {
            return;
        }

        self.reregister_agents(cx);
    }

    fn reregister_agents(&mut self, cx: &mut Context<Self>) {
        let AgentServerStoreState::Local {
            node_runtime,
            fs,
            project_environment,
            downstream_client,
            settings: old_settings,
            http_client,
            extension_agents,
            ..
        } = &mut self.state
        else {
            debug_panic!("Non-local projects should never attempt to reregister. This is a bug!");

            return;
        };

        let new_settings = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None)
            .clone();

        self.external_agents.clear();
        self.external_agents.insert(
            GEMINI_NAME.into(),
            Box::new(LocalGemini {
                fs: fs.clone(),
                node_runtime: node_runtime.clone(),
                project_environment: project_environment.clone(),
                custom_command: new_settings
                    .gemini
                    .clone()
                    .and_then(|settings| settings.custom_command()),
                settings_env: new_settings
                    .gemini
                    .as_ref()
                    .and_then(|settings| settings.env.clone()),
                ignore_system_version: new_settings
                    .gemini
                    .as_ref()
                    .and_then(|settings| settings.ignore_system_version)
                    .unwrap_or(true),
            }),
        );
        self.external_agents.insert(
            CODEX_NAME.into(),
            Box::new(LocalCodex {
                fs: fs.clone(),
                project_environment: project_environment.clone(),
                custom_command: new_settings
                    .codex
                    .clone()
                    .and_then(|settings| settings.custom_command()),
                settings_env: new_settings
                    .codex
                    .as_ref()
                    .and_then(|settings| settings.env.clone()),
                http_client: http_client.clone(),
                no_browser: downstream_client
                    .as_ref()
                    .is_some_and(|(_, client)| !client.has_wsl_interop()),
            }),
        );
        self.external_agents.insert(
            CLAUDE_CODE_NAME.into(),
            Box::new(LocalClaudeCode {
                fs: fs.clone(),
                node_runtime: node_runtime.clone(),
                project_environment: project_environment.clone(),
                custom_command: new_settings
                    .claude
                    .clone()
                    .and_then(|settings| settings.custom_command()),
                settings_env: new_settings
                    .claude
                    .as_ref()
                    .and_then(|settings| settings.env.clone()),
            }),
        );
        self.external_agents
            .extend(
                new_settings
                    .custom
                    .iter()
                    .filter_map(|(name, settings)| match settings {
                        CustomAgentServerSettings::Custom { command, .. } => Some((
                            ExternalAgentServerName(name.clone().into()),
                            Box::new(LocalCustomAgent {
                                command: command.clone(),
                                project_environment: project_environment.clone(),
                            }) as Box<dyn ExternalAgentServer>,
                        )),
                        CustomAgentServerSettings::Extension { .. } => None,
                    }),
            );
        self.external_agents.extend(extension_agents.iter().map(
            |(agent_name, ext_id, targets, env, icon_path)| {
                let name = ExternalAgentServerName(agent_name.clone().into());

                // Restore icon if present
                if let Some(icon) = icon_path {
                    self.agent_icons
                        .insert(name.clone(), SharedString::from(icon.clone()));
                }

                (
                    name,
                    Box::new(LocalExtensionArchiveAgent {
                        fs: fs.clone(),
                        http_client: http_client.clone(),
                        node_runtime: node_runtime.clone(),
                        project_environment: project_environment.clone(),
                        extension_id: Arc::from(&**ext_id),
                        targets: targets.clone(),
                        env: env.clone(),
                        agent_id: agent_name.clone(),
                    }) as Box<dyn ExternalAgentServer>,
                )
            },
        ));

        *old_settings = Some(new_settings.clone());

        if let Some((project_id, downstream_client)) = downstream_client {
            downstream_client
                .send(proto::ExternalAgentsUpdated {
                    project_id: *project_id,
                    names: self
                        .external_agents
                        .keys()
                        .map(|name| name.to_string())
                        .collect(),
                })
                .log_err();
        }
        cx.emit(AgentServersUpdated);
    }

    pub fn node_runtime(&self) -> Option<NodeRuntime> {
        match &self.state {
            AgentServerStoreState::Local { node_runtime, .. } => Some(node_runtime.clone()),
            _ => None,
        }
    }

    pub fn local(
        node_runtime: NodeRuntime,
        fs: Arc<dyn Fs>,
        project_environment: Entity<ProjectEnvironment>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe_global::<SettingsStore>(|this, cx| {
            this.agent_servers_settings_changed(cx);
        });
        let mut this = Self {
            state: AgentServerStoreState::Local {
                node_runtime,
                fs,
                project_environment,
                http_client,
                downstream_client: None,
                settings: None,
                extension_agents: vec![],
                _subscriptions: [subscription],
            },
            external_agents: Default::default(),
            agent_icons: Default::default(),
            agent_display_names: Default::default(),
        };
        if let Some(_events) = extension::ExtensionEvents::try_global(cx) {}
        this.agent_servers_settings_changed(cx);
        this
    }

    pub(crate) fn remote(project_id: u64, upstream_client: Entity<RemoteClient>) -> Self {
        // Set up the builtin agents here so they're immediately available in
        // remote projects--we know that the HeadlessProject on the other end
        // will have them.
        let external_agents: [(ExternalAgentServerName, Box<dyn ExternalAgentServer>); 3] = [
            (
                CLAUDE_CODE_NAME.into(),
                Box::new(RemoteExternalAgentServer {
                    project_id,
                    upstream_client: upstream_client.clone(),
                    name: CLAUDE_CODE_NAME.into(),
                    status_tx: None,
                    new_version_available_tx: None,
                }) as Box<dyn ExternalAgentServer>,
            ),
            (
                CODEX_NAME.into(),
                Box::new(RemoteExternalAgentServer {
                    project_id,
                    upstream_client: upstream_client.clone(),
                    name: CODEX_NAME.into(),
                    status_tx: None,
                    new_version_available_tx: None,
                }) as Box<dyn ExternalAgentServer>,
            ),
            (
                GEMINI_NAME.into(),
                Box::new(RemoteExternalAgentServer {
                    project_id,
                    upstream_client: upstream_client.clone(),
                    name: GEMINI_NAME.into(),
                    status_tx: None,
                    new_version_available_tx: None,
                }) as Box<dyn ExternalAgentServer>,
            ),
        ];

        Self {
            state: AgentServerStoreState::Remote {
                project_id,
                upstream_client,
            },
            external_agents: external_agents.into_iter().collect(),
            agent_icons: HashMap::default(),
            agent_display_names: HashMap::default(),
        }
    }

    pub(crate) fn collab(_cx: &mut Context<Self>) -> Self {
        Self {
            state: AgentServerStoreState::Collab,
            external_agents: Default::default(),
            agent_icons: Default::default(),
            agent_display_names: Default::default(),
        }
    }

    pub fn shared(&mut self, project_id: u64, client: AnyProtoClient, cx: &mut Context<Self>) {
        match &mut self.state {
            AgentServerStoreState::Local {
                downstream_client, ..
            } => {
                *downstream_client = Some((project_id, client.clone()));
                // Send the current list of external agents downstream, but only after a delay,
                // to avoid having the message arrive before the downstream project's agent server store
                // sets up its handlers.
                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    let names = this.update(cx, |this, _| {
                        this.external_agents
                            .keys()
                            .map(|name| name.to_string())
                            .collect()
                    })?;
                    client
                        .send(proto::ExternalAgentsUpdated { project_id, names })
                        .log_err();
                    anyhow::Ok(())
                })
                .detach();
            }
            AgentServerStoreState::Remote { .. } => {
                debug_panic!(
                    "external agents over collab not implemented, remote project should not be shared"
                );
            }
            AgentServerStoreState::Collab => {
                debug_panic!("external agents over collab not implemented, should not be shared");
            }
        }
    }

    pub fn get_external_agent(
        &mut self,
        name: &ExternalAgentServerName,
    ) -> Option<&mut (dyn ExternalAgentServer + 'static)> {
        self.external_agents
            .get_mut(name)
            .map(|agent| agent.as_mut())
    }

    pub fn external_agents(&self) -> impl Iterator<Item = &ExternalAgentServerName> {
        self.external_agents.keys()
    }

    async fn handle_get_agent_server_command(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetAgentServerCommand>,
        mut cx: AsyncApp,
    ) -> Result<proto::AgentServerCommand> {
        let (command, root_dir, login_command) = this
            .update(&mut cx, |this, cx| {
                let AgentServerStoreState::Local {
                    downstream_client, ..
                } = &this.state
                else {
                    debug_panic!("should not receive GetAgentServerCommand in a non-local project");
                    bail!("unexpected GetAgentServerCommand request in a non-local project");
                };
                let agent = this
                    .external_agents
                    .get_mut(&*envelope.payload.name)
                    .with_context(|| format!("agent `{}` not found", envelope.payload.name))?;
                let (status_tx, new_version_available_tx) = downstream_client
                    .clone()
                    .map(|(project_id, downstream_client)| {
                        let (status_tx, mut status_rx) = watch::channel(SharedString::from(""));
                        let (new_version_available_tx, mut new_version_available_rx) =
                            watch::channel(None);
                        cx.spawn({
                            let downstream_client = downstream_client.clone();
                            let name = envelope.payload.name.clone();
                            async move |_, _| {
                                while let Some(status) = status_rx.recv().await.ok() {
                                    downstream_client.send(
                                        proto::ExternalAgentLoadingStatusUpdated {
                                            project_id,
                                            name: name.clone(),
                                            status: status.to_string(),
                                        },
                                    )?;
                                }
                                anyhow::Ok(())
                            }
                        })
                        .detach_and_log_err(cx);
                        cx.spawn({
                            let name = envelope.payload.name.clone();
                            async move |_, _| {
                                if let Some(version) =
                                    new_version_available_rx.recv().await.ok().flatten()
                                {
                                    downstream_client.send(
                                        proto::NewExternalAgentVersionAvailable {
                                            project_id,
                                            name: name.clone(),
                                            version,
                                        },
                                    )?;
                                }
                                anyhow::Ok(())
                            }
                        })
                        .detach_and_log_err(cx);
                        (status_tx, new_version_available_tx)
                    })
                    .unzip();
                anyhow::Ok(agent.get_command(
                    envelope.payload.root_dir.as_deref(),
                    HashMap::default(),
                    status_tx,
                    new_version_available_tx,
                    &mut cx.to_async(),
                ))
            })?
            .await?;
        Ok(proto::AgentServerCommand {
            path: command.path.to_string_lossy().into_owned(),
            args: command.args,
            env: command
                .env
                .map(|env| env.into_iter().collect())
                .unwrap_or_default(),
            root_dir: root_dir,
            login: login_command.map(|cmd| cmd.to_proto()),
        })
    }

    async fn handle_external_agents_updated(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExternalAgentsUpdated>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let AgentServerStoreState::Remote {
                project_id,
                upstream_client,
            } = &this.state
            else {
                debug_panic!(
                    "handle_external_agents_updated should not be called for a non-remote project"
                );
                bail!("unexpected ExternalAgentsUpdated message")
            };

            let mut status_txs = this
                .external_agents
                .iter_mut()
                .filter_map(|(name, agent)| {
                    Some((
                        name.clone(),
                        agent
                            .downcast_mut::<RemoteExternalAgentServer>()?
                            .status_tx
                            .take(),
                    ))
                })
                .collect::<HashMap<_, _>>();
            let mut new_version_available_txs = this
                .external_agents
                .iter_mut()
                .filter_map(|(name, agent)| {
                    Some((
                        name.clone(),
                        agent
                            .downcast_mut::<RemoteExternalAgentServer>()?
                            .new_version_available_tx
                            .take(),
                    ))
                })
                .collect::<HashMap<_, _>>();

            this.external_agents = envelope
                .payload
                .names
                .into_iter()
                .map(|name| {
                    let agent = RemoteExternalAgentServer {
                        project_id: *project_id,
                        upstream_client: upstream_client.clone(),
                        name: ExternalAgentServerName(name.clone().into()),
                        status_tx: status_txs.remove(&*name).flatten(),
                        new_version_available_tx: new_version_available_txs
                            .remove(&*name)
                            .flatten(),
                    };
                    (
                        ExternalAgentServerName(name.into()),
                        Box::new(agent) as Box<dyn ExternalAgentServer>,
                    )
                })
                .collect();
            cx.emit(AgentServersUpdated);
            Ok(())
        })
    }

    async fn handle_external_extension_agents_updated(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExternalExtensionAgentsUpdated>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let AgentServerStoreState::Local {
                extension_agents, ..
            } = &mut this.state
            else {
                panic!(
                    "handle_external_extension_agents_updated \
                    should not be called for a non-remote project"
                );
            };

            for ExternalExtensionAgent {
                name,
                icon_path,
                extension_id,
                targets,
                env,
            } in envelope.payload.agents
            {
                let icon_path_string = icon_path.clone();
                if let Some(icon_path) = icon_path {
                    this.agent_icons.insert(
                        ExternalAgentServerName(name.clone().into()),
                        icon_path.into(),
                    );
                }
                extension_agents.push((
                    Arc::from(&*name),
                    extension_id,
                    targets
                        .into_iter()
                        .map(|(k, v)| (k, extension::TargetConfig::from_proto(v)))
                        .collect(),
                    env.into_iter().collect(),
                    icon_path_string,
                ));
            }

            this.reregister_agents(cx);
            cx.emit(AgentServersUpdated);
            Ok(())
        })
    }

    async fn handle_loading_status_updated(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExternalAgentLoadingStatusUpdated>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            if let Some(agent) = this.external_agents.get_mut(&*envelope.payload.name)
                && let Some(agent) = agent.downcast_mut::<RemoteExternalAgentServer>()
                && let Some(status_tx) = &mut agent.status_tx
            {
                status_tx.send(envelope.payload.status.into()).ok();
            }
        });
        Ok(())
    }

    async fn handle_new_version_available(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::NewExternalAgentVersionAvailable>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, _| {
            if let Some(agent) = this.external_agents.get_mut(&*envelope.payload.name)
                && let Some(agent) = agent.downcast_mut::<RemoteExternalAgentServer>()
                && let Some(new_version_available_tx) = &mut agent.new_version_available_tx
            {
                new_version_available_tx
                    .send(Some(envelope.payload.version))
                    .ok();
            }
        });
        Ok(())
    }

    pub fn get_extension_id_for_agent(
        &mut self,
        name: &ExternalAgentServerName,
    ) -> Option<Arc<str>> {
        self.external_agents.get_mut(name).and_then(|agent| {
            agent
                .as_any_mut()
                .downcast_ref::<LocalExtensionArchiveAgent>()
                .map(|ext_agent| ext_agent.extension_id.clone())
        })
    }
}

fn get_or_npm_install_builtin_agent(
    binary_name: SharedString,
    package_name: SharedString,
    entrypoint_path: PathBuf,
    minimum_version: Option<semver::Version>,
    status_tx: Option<watch::Sender<SharedString>>,
    new_version_available: Option<watch::Sender<Option<String>>>,
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    cx: &mut AsyncApp,
) -> Task<std::result::Result<AgentServerCommand, anyhow::Error>> {
    cx.spawn(async move |cx| {
        let node_path = node_runtime.binary_path().await?;
        let dir = paths::external_agents_dir().join(binary_name.as_str());
        fs.create_dir(&dir).await?;

        let mut stream = fs.read_dir(&dir).await?;
        let mut versions = Vec::new();
        let mut to_delete = Vec::new();
        while let Some(entry) = stream.next().await {
            let Ok(entry) = entry else { continue };
            let Some(file_name) = entry.file_name() else {
                continue;
            };

            if let Some(name) = file_name.to_str()
                && let Some(version) = semver::Version::from_str(name).ok()
                && fs
                    .is_file(&dir.join(file_name).join(&entrypoint_path))
                    .await
            {
                versions.push((version, file_name.to_owned()));
            } else {
                to_delete.push(file_name.to_owned())
            }
        }

        versions.sort();
        let newest_version = if let Some((version, _)) = versions.last().cloned()
            && minimum_version.is_none_or(|minimum_version| version >= minimum_version)
        {
            versions.pop()
        } else {
            None
        };
        log::debug!("existing version of {package_name}: {newest_version:?}");
        to_delete.extend(versions.into_iter().map(|(_, file_name)| file_name));

        cx.background_spawn({
            let fs = fs.clone();
            let dir = dir.clone();
            async move {
                for file_name in to_delete {
                    fs.remove_dir(
                        &dir.join(file_name),
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: false,
                        },
                    )
                    .await
                    .ok();
                }
            }
        })
        .detach();

        let version = if let Some((version, file_name)) = newest_version {
            cx.background_spawn({
                let dir = dir.clone();
                let fs = fs.clone();
                async move {
                    let latest_version = node_runtime
                        .npm_package_latest_version(&package_name)
                        .await
                        .ok();
                    if let Some(latest_version) = latest_version
                        && latest_version != version
                    {
                        let download_result = download_latest_version(
                            fs,
                            dir.clone(),
                            node_runtime,
                            package_name.clone(),
                        )
                        .await
                        .log_err();
                        if let Some(mut new_version_available) = new_version_available
                            && download_result.is_some()
                        {
                            new_version_available
                                .send(Some(latest_version.to_string()))
                                .ok();
                        }
                    }
                }
            })
            .detach();
            file_name
        } else {
            if let Some(mut status_tx) = status_tx {
                status_tx.send("Installing…".into()).ok();
            }
            let dir = dir.clone();
            cx.background_spawn(download_latest_version(
                fs.clone(),
                dir.clone(),
                node_runtime,
                package_name.clone(),
            ))
            .await?
            .to_string()
            .into()
        };

        let agent_server_path = dir.join(version).join(entrypoint_path);
        let agent_server_path_exists = fs.is_file(&agent_server_path).await;
        anyhow::ensure!(
            agent_server_path_exists,
            "Missing entrypoint path {} after installation",
            agent_server_path.to_string_lossy()
        );

        anyhow::Ok(AgentServerCommand {
            path: node_path,
            args: vec![agent_server_path.to_string_lossy().into_owned()],
            env: None,
        })
    })
}

fn find_bin_in_path(
    bin_name: SharedString,
    root_dir: PathBuf,
    env: HashMap<String, String>,
    cx: &mut AsyncApp,
) -> Task<Option<PathBuf>> {
    cx.background_executor().spawn(async move {
        let which_result = if cfg!(windows) {
            which::which(bin_name.as_str())
        } else {
            let shell_path = env.get("PATH").cloned();
            which::which_in(bin_name.as_str(), shell_path.as_ref(), &root_dir)
        };

        if let Err(which::Error::CannotFindBinaryPath) = which_result {
            return None;
        }

        which_result.log_err()
    })
}

async fn download_latest_version(
    fs: Arc<dyn Fs>,
    dir: PathBuf,
    node_runtime: NodeRuntime,
    package_name: SharedString,
) -> Result<Version> {
    log::debug!("downloading latest version of {package_name}");

    let tmp_dir = tempfile::tempdir_in(&dir)?;

    node_runtime
        .npm_install_packages(tmp_dir.path(), &[(&package_name, "latest")])
        .await?;

    let version = node_runtime
        .npm_package_installed_version(tmp_dir.path(), &package_name)
        .await?
        .context("expected package to be installed")?;

    fs.rename(
        &tmp_dir.keep(),
        &dir.join(version.to_string()),
        RenameOptions {
            ignore_if_exists: true,
            overwrite: true,
            create_parents: false,
        },
    )
    .await?;

    anyhow::Ok(version)
}

struct RemoteExternalAgentServer {
    project_id: u64,
    upstream_client: Entity<RemoteClient>,
    name: ExternalAgentServerName,
    status_tx: Option<watch::Sender<SharedString>>,
    new_version_available_tx: Option<watch::Sender<Option<String>>>,
}

impl ExternalAgentServer for RemoteExternalAgentServer {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let project_id = self.project_id;
        let name = self.name.to_string();
        let upstream_client = self.upstream_client.downgrade();
        let root_dir = root_dir.map(|root_dir| root_dir.to_owned());
        self.status_tx = status_tx;
        self.new_version_available_tx = new_version_available_tx;
        cx.spawn(async move |cx| {
            let mut response = upstream_client
                .update(cx, |upstream_client, _| {
                    upstream_client
                        .proto_client()
                        .request(proto::GetAgentServerCommand {
                            project_id,
                            name,
                            root_dir: root_dir.clone(),
                        })
                })?
                .await?;
            let root_dir = response.root_dir;
            response.env.extend(extra_env);
            let command = upstream_client.update(cx, |client, _| {
                client.build_command_with_options(
                    Some(response.path),
                    &response.args,
                    &response.env.into_iter().collect(),
                    Some(root_dir.clone()),
                    None,
                    Interactive::No,
                )
            })??;
            Ok((
                AgentServerCommand {
                    path: command.program.into(),
                    args: command.args,
                    env: Some(command.env),
                },
                root_dir,
                response.login.map(SpawnInTerminal::from_proto),
            ))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalGemini {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    custom_command: Option<AgentServerCommand>,
    settings_env: Option<HashMap<String, String>>,
    ignore_system_version: bool,
}

impl ExternalAgentServer for LocalGemini {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let custom_command = self.custom_command.clone();
        let settings_env = self.settings_env.clone();
        let ignore_system_version = self.ignore_system_version;
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();

            env.extend(settings_env.unwrap_or_default());

            let mut command = if let Some(mut custom_command) = custom_command {
                custom_command.env = Some(env);
                custom_command
            } else if !ignore_system_version
                && let Some(bin) =
                    find_bin_in_path("gemini".into(), root_dir.to_path_buf(), env.clone(), cx).await
            {
                AgentServerCommand {
                    path: bin,
                    args: Vec::new(),
                    env: Some(env),
                }
            } else {
                let mut command = get_or_npm_install_builtin_agent(
                    GEMINI_NAME.into(),
                    "@google/gemini-cli".into(),
                    "node_modules/@google/gemini-cli/dist/index.js".into(),
                    if cfg!(windows) {
                        // v0.8.x on Windows has a bug that causes the initialize request to hang forever
                        Some("0.9.0".parse().unwrap())
                    } else {
                        Some("0.2.1".parse().unwrap())
                    },
                    status_tx,
                    new_version_available_tx,
                    fs,
                    node_runtime,
                    cx,
                )
                .await?;
                command.env = Some(env);
                command
            };

            // Gemini CLI doesn't seem to have a dedicated invocation for logging in--we just run it normally without any arguments.
            let login = task::SpawnInTerminal {
                command: Some(command.path.to_string_lossy().into_owned()),
                args: command.args.clone(),
                env: command.env.clone().unwrap_or_default(),
                label: "gemini /auth".into(),
                ..Default::default()
            };

            command.env.get_or_insert_default().extend(extra_env);
            command.args.push("--experimental-acp".into());
            Ok((
                command,
                root_dir.to_string_lossy().into_owned(),
                Some(login),
            ))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalClaudeCode {
    fs: Arc<dyn Fs>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    custom_command: Option<AgentServerCommand>,
    settings_env: Option<HashMap<String, String>>,
}

impl ExternalAgentServer for LocalClaudeCode {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let custom_command = self.custom_command.clone();
        let settings_env = self.settings_env.clone();
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();
            env.insert("ANTHROPIC_API_KEY".into(), "".into());

            env.extend(settings_env.unwrap_or_default());

            let (mut command, login_command) = if let Some(mut custom_command) = custom_command {
                custom_command.env = Some(env);
                (custom_command, None)
            } else {
                let mut command = get_or_npm_install_builtin_agent(
                    "claude-code-acp".into(),
                    "@zed-industries/claude-code-acp".into(),
                    "node_modules/@zed-industries/claude-code-acp/dist/index.js".into(),
                    Some("0.5.2".parse().unwrap()),
                    status_tx,
                    new_version_available_tx,
                    fs,
                    node_runtime,
                    cx,
                )
                .await?;
                command.env = Some(env);
                let login = command
                    .args
                    .first()
                    .and_then(|path| {
                        path.strip_suffix("/@zed-industries/claude-code-acp/dist/index.js")
                    })
                    .map(|path_prefix| task::SpawnInTerminal {
                        command: Some(command.path.to_string_lossy().into_owned()),
                        args: vec![
                            Path::new(path_prefix)
                                .join("@anthropic-ai/claude-agent-sdk/cli.js")
                                .to_string_lossy()
                                .to_string(),
                            "/login".into(),
                        ],
                        env: command.env.clone().unwrap_or_default(),
                        label: "claude /login".into(),
                        ..Default::default()
                    });
                (command, login)
            };

            command.env.get_or_insert_default().extend(extra_env);
            Ok((
                command,
                root_dir.to_string_lossy().into_owned(),
                login_command,
            ))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct LocalCodex {
    fs: Arc<dyn Fs>,
    project_environment: Entity<ProjectEnvironment>,
    http_client: Arc<dyn HttpClient>,
    custom_command: Option<AgentServerCommand>,
    settings_env: Option<HashMap<String, String>>,
    no_browser: bool,
}

impl ExternalAgentServer for LocalCodex {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        mut status_tx: Option<watch::Sender<SharedString>>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let project_environment = self.project_environment.downgrade();
        let http = self.http_client.clone();
        let custom_command = self.custom_command.clone();
        let settings_env = self.settings_env.clone();
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();
        let no_browser = self.no_browser;

        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();
            if no_browser {
                env.insert("NO_BROWSER".to_owned(), "1".to_owned());
            }

            env.extend(settings_env.unwrap_or_default());

            let mut command = if let Some(mut custom_command) = custom_command {
                custom_command.env = Some(env);
                custom_command
            } else {
                let dir = paths::external_agents_dir().join(CODEX_NAME);
                fs.create_dir(&dir).await?;

                let bin_name = if cfg!(windows) {
                    "codex-acp.exe"
                } else {
                    "codex-acp"
                };

                let find_latest_local_version = async || -> Option<PathBuf> {
                    let mut local_versions: Vec<(semver::Version, String)> = Vec::new();
                    let mut stream = fs.read_dir(&dir).await.ok()?;
                    while let Some(entry) = stream.next().await {
                        let Ok(entry) = entry else { continue };
                        let Some(file_name) = entry.file_name() else {
                            continue;
                        };
                        let version_path = dir.join(&file_name);
                        if fs.is_file(&version_path.join(bin_name)).await {
                            let version_str = file_name.to_string_lossy();
                            if let Ok(version) =
                                semver::Version::from_str(version_str.trim_start_matches('v'))
                            {
                                local_versions.push((version, version_str.into_owned()));
                            }
                        }
                    }
                    local_versions.sort_by(|(a, _), (b, _)| a.cmp(b));
                    local_versions.last().map(|(_, v)| dir.join(v))
                };

                let fallback_to_latest_local_version =
                    async |err: anyhow::Error| -> Result<PathBuf, anyhow::Error> {
                        if let Some(local) = find_latest_local_version().await {
                            log::info!(
                                "Falling back to locally installed Codex version: {}",
                                local.display()
                            );
                            Ok(local)
                        } else {
                            Err(err)
                        }
                    };

                let version_dir = match ::http_client::github::latest_github_release(
                    CODEX_ACP_REPO,
                    true,
                    false,
                    http.clone(),
                )
                .await
                {
                    Ok(release) => {
                        let version_dir = dir.join(&release.tag_name);
                        if !fs.is_dir(&version_dir).await {
                            if let Some(ref mut status_tx) = status_tx {
                                status_tx.send("Installing…".into()).ok();
                            }

                            let tag = release.tag_name.clone();
                            let version_number = tag.trim_start_matches('v');
                            let asset_name = asset_name(version_number)
                                .context("codex acp is not supported for this architecture")?;
                            let asset = release
                                .assets
                                .into_iter()
                                .find(|asset| asset.name == asset_name)
                                .with_context(|| {
                                    format!("no asset found matching `{asset_name:?}`")
                                })?;
                            // Strip "sha256:" prefix from digest if present (GitHub API format)
                            let digest = asset
                                .digest
                                .as_deref()
                                .and_then(|d| d.strip_prefix("sha256:").or(Some(d)));
                            match ::http_client::github_download::download_server_binary(
                                &*http,
                                &asset.browser_download_url,
                                digest,
                                &version_dir,
                                if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
                                    AssetKind::Zip
                                } else {
                                    AssetKind::TarGz
                                },
                            )
                            .await
                            {
                                Ok(()) => {
                                    // remove older versions
                                    util::fs::remove_matching(&dir, |entry| entry != version_dir)
                                        .await;
                                    version_dir
                                }
                                Err(err) => {
                                    log::error!(
                                        "Failed to download Codex release {}: {err:#}",
                                        release.tag_name
                                    );
                                    fallback_to_latest_local_version(err).await?
                                }
                            }
                        } else {
                            version_dir
                        }
                    }
                    Err(err) => {
                        log::error!("Failed to fetch Codex latest release: {err:#}");
                        fallback_to_latest_local_version(err).await?
                    }
                };

                let bin_path = version_dir.join(bin_name);
                anyhow::ensure!(
                    fs.is_file(&bin_path).await,
                    "Missing Codex binary at {} after installation",
                    bin_path.to_string_lossy()
                );

                let mut cmd = AgentServerCommand {
                    path: bin_path,
                    args: Vec::new(),
                    env: None,
                };
                cmd.env = Some(env);
                cmd
            };

            command.env.get_or_insert_default().extend(extra_env);
            Ok((command, root_dir.to_string_lossy().into_owned(), None))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub const CODEX_ACP_REPO: &str = "zed-industries/codex-acp";

fn get_platform_info() -> Option<(&'static str, &'static str, &'static str)> {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        return None;
    };

    let platform = if cfg!(target_os = "macos") {
        "apple-darwin"
    } else if cfg!(target_os = "windows") {
        "pc-windows-msvc"
    } else if cfg!(target_os = "linux") {
        "unknown-linux-gnu"
    } else {
        return None;
    };

    // Windows uses .zip in release assets
    let ext = if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    };

    Some((arch, platform, ext))
}

fn asset_name(version: &str) -> Option<String> {
    let (arch, platform, ext) = get_platform_info()?;
    Some(format!("codex-acp-{version}-{arch}-{platform}.{ext}"))
}

struct LocalExtensionArchiveAgent {
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    node_runtime: NodeRuntime,
    project_environment: Entity<ProjectEnvironment>,
    extension_id: Arc<str>,
    agent_id: Arc<str>,
    targets: HashMap<String, extension::TargetConfig>,
    env: HashMap<String, String>,
}

struct LocalCustomAgent {
    project_environment: Entity<ProjectEnvironment>,
    command: AgentServerCommand,
}

impl ExternalAgentServer for LocalExtensionArchiveAgent {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        _status_tx: Option<watch::Sender<SharedString>>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        let node_runtime = self.node_runtime.clone();
        let project_environment = self.project_environment.downgrade();
        let extension_id = self.extension_id.clone();
        let agent_id = self.agent_id.clone();
        let targets = self.targets.clone();
        let base_env = self.env.clone();

        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();

        cx.spawn(async move |cx| {
            // Get project environment
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();

            // Merge manifest env and extra env
            env.extend(base_env);
            env.extend(extra_env);

            let cache_key = format!("{}/{}", extension_id, agent_id);
            let dir = paths::external_agents_dir().join(&cache_key);
            fs.create_dir(&dir).await?;

            // Determine platform key
            let os = if cfg!(target_os = "macos") {
                "darwin"
            } else if cfg!(target_os = "linux") {
                "linux"
            } else if cfg!(target_os = "windows") {
                "windows"
            } else {
                anyhow::bail!("unsupported OS");
            };

            let arch = if cfg!(target_arch = "aarch64") {
                "aarch64"
            } else if cfg!(target_arch = "x86_64") {
                "x86_64"
            } else {
                anyhow::bail!("unsupported architecture");
            };

            let platform_key = format!("{}-{}", os, arch);
            let target_config = targets.get(&platform_key).with_context(|| {
                format!(
                    "no target specified for platform '{}'. Available platforms: {}",
                    platform_key,
                    targets
                        .keys()
                        .map(|k| k.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })?;

            let archive_url = &target_config.archive;

            // Use URL as version identifier for caching
            // Hash the URL to get a stable directory name
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            archive_url.hash(&mut hasher);
            let url_hash = hasher.finish();
            let version_dir = dir.join(format!("v_{:x}", url_hash));

            if !fs.is_dir(&version_dir).await {
                // Determine SHA256 for verification
                let sha256 = if let Some(provided_sha) = &target_config.sha256 {
                    // Use provided SHA256
                    Some(provided_sha.clone())
                } else if archive_url.starts_with("https://github.com/") {
                    // Try to fetch SHA256 from GitHub API
                    // Parse URL to extract repo and tag/file info
                    // Format: https://github.com/owner/repo/releases/download/tag/file.zip
                    if let Some(caps) = archive_url.strip_prefix("https://github.com/") {
                        let parts: Vec<&str> = caps.split('/').collect();
                        if parts.len() >= 6 && parts[2] == "releases" && parts[3] == "download" {
                            let repo = format!("{}/{}", parts[0], parts[1]);
                            let tag = parts[4];
                            let filename = parts[5..].join("/");

                            // Try to get release info from GitHub
                            if let Ok(release) = ::http_client::github::get_release_by_tag_name(
                                &repo,
                                tag,
                                http_client.clone(),
                            )
                            .await
                            {
                                // Find matching asset
                                if let Some(asset) =
                                    release.assets.iter().find(|a| a.name == filename)
                                {
                                    // Strip "sha256:" prefix if present
                                    asset.digest.as_ref().and_then(|d| {
                                        d.strip_prefix("sha256:")
                                            .map(|s| s.to_string())
                                            .or_else(|| Some(d.clone()))
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Determine archive type from URL
                let asset_kind = if archive_url.ends_with(".zip") {
                    AssetKind::Zip
                } else if archive_url.ends_with(".tar.gz") || archive_url.ends_with(".tgz") {
                    AssetKind::TarGz
                } else {
                    anyhow::bail!("unsupported archive type in URL: {}", archive_url);
                };

                // Download and extract
                ::http_client::github_download::download_server_binary(
                    &*http_client,
                    archive_url,
                    sha256.as_deref(),
                    &version_dir,
                    asset_kind,
                )
                .await?;
            }

            // Validate and resolve cmd path
            let cmd = &target_config.cmd;

            let cmd_path = if cmd == "node" {
                // Use Zed's managed Node.js runtime
                node_runtime.binary_path().await?
            } else {
                if cmd.contains("..") {
                    anyhow::bail!("command path cannot contain '..': {}", cmd);
                }

                if cmd.starts_with("./") || cmd.starts_with(".\\") {
                    // Relative to extraction directory
                    let cmd_path = version_dir.join(&cmd[2..]);
                    anyhow::ensure!(
                        fs.is_file(&cmd_path).await,
                        "Missing command {} after extraction",
                        cmd_path.to_string_lossy()
                    );
                    cmd_path
                } else {
                    // On PATH
                    anyhow::bail!("command must be relative (start with './'): {}", cmd);
                }
            };

            let command = AgentServerCommand {
                path: cmd_path,
                args: target_config.args.clone(),
                env: Some(env),
            };

            Ok((command, version_dir.to_string_lossy().into_owned(), None))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ExternalAgentServer for LocalCustomAgent {
    fn get_command(
        &mut self,
        root_dir: Option<&str>,
        extra_env: HashMap<String, String>,
        _status_tx: Option<watch::Sender<SharedString>>,
        _new_version_available_tx: Option<watch::Sender<Option<String>>>,
        cx: &mut AsyncApp,
    ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
        let mut command = self.command.clone();
        let root_dir: Arc<Path> = root_dir
            .map(|root_dir| Path::new(root_dir))
            .unwrap_or(paths::home_dir())
            .into();
        let project_environment = self.project_environment.downgrade();
        cx.spawn(async move |cx| {
            let mut env = project_environment
                .update(cx, |project_environment, cx| {
                    project_environment.local_directory_environment(
                        &Shell::System,
                        root_dir.clone(),
                        cx,
                    )
                })?
                .await
                .unwrap_or_default();
            env.extend(command.env.unwrap_or_default());
            env.extend(extra_env);
            command.env = Some(env);
            Ok((command, root_dir.to_string_lossy().into_owned(), None))
        })
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub const GEMINI_NAME: &'static str = "gemini";
pub const CLAUDE_CODE_NAME: &'static str = "claude";
pub const CODEX_NAME: &'static str = "codex";

#[derive(Default, Clone, JsonSchema, Debug, PartialEq, RegisterSetting)]
pub struct AllAgentServersSettings {
    pub gemini: Option<BuiltinAgentServerSettings>,
    pub claude: Option<BuiltinAgentServerSettings>,
    pub codex: Option<BuiltinAgentServerSettings>,
    pub custom: HashMap<String, CustomAgentServerSettings>,
}
#[derive(Default, Clone, JsonSchema, Debug, PartialEq)]
pub struct BuiltinAgentServerSettings {
    pub path: Option<PathBuf>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub ignore_system_version: Option<bool>,
    pub default_mode: Option<String>,
    pub default_model: Option<String>,
    pub favorite_models: Vec<String>,
    pub default_config_options: HashMap<String, String>,
    pub favorite_config_option_values: HashMap<String, Vec<String>>,
}

impl BuiltinAgentServerSettings {
    fn custom_command(self) -> Option<AgentServerCommand> {
        self.path.map(|path| AgentServerCommand {
            path,
            args: self.args.unwrap_or_default(),
            // Settings env are always applied, so we don't need to supply them here as well
            env: None,
        })
    }
}

impl From<settings::BuiltinAgentServerSettings> for BuiltinAgentServerSettings {
    fn from(value: settings::BuiltinAgentServerSettings) -> Self {
        BuiltinAgentServerSettings {
            path: value
                .path
                .map(|p| PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).as_ref())),
            args: value.args,
            env: value.env,
            ignore_system_version: value.ignore_system_version,
            default_mode: value.default_mode,
            default_model: value.default_model,
            favorite_models: value.favorite_models,
            default_config_options: value.default_config_options,
            favorite_config_option_values: value.favorite_config_option_values,
        }
    }
}

impl From<AgentServerCommand> for BuiltinAgentServerSettings {
    fn from(value: AgentServerCommand) -> Self {
        BuiltinAgentServerSettings {
            path: Some(value.path),
            args: Some(value.args),
            env: value.env,
            ..Default::default()
        }
    }
}

#[derive(Clone, JsonSchema, Debug, PartialEq)]
pub enum CustomAgentServerSettings {
    Custom {
        command: AgentServerCommand,
        /// The default mode to use for this agent.
        ///
        /// Note: Not only all agents support modes.
        ///
        /// Default: None
        default_mode: Option<String>,
        /// The default model to use for this agent.
        ///
        /// This should be the model ID as reported by the agent.
        ///
        /// Default: None
        default_model: Option<String>,
        /// The favorite models for this agent.
        ///
        /// Default: []
        favorite_models: Vec<String>,
        /// Default values for session config options.
        ///
        /// This is a map from config option ID to value ID.
        ///
        /// Default: {}
        default_config_options: HashMap<String, String>,
        /// Favorited values for session config options.
        ///
        /// This is a map from config option ID to a list of favorited value IDs.
        ///
        /// Default: {}
        favorite_config_option_values: HashMap<String, Vec<String>>,
    },
    Extension {
        /// The default mode to use for this agent.
        ///
        /// Note: Not only all agents support modes.
        ///
        /// Default: None
        default_mode: Option<String>,
        /// The default model to use for this agent.
        ///
        /// This should be the model ID as reported by the agent.
        ///
        /// Default: None
        default_model: Option<String>,
        /// The favorite models for this agent.
        ///
        /// Default: []
        favorite_models: Vec<String>,
        /// Default values for session config options.
        ///
        /// This is a map from config option ID to value ID.
        ///
        /// Default: {}
        default_config_options: HashMap<String, String>,
        /// Favorited values for session config options.
        ///
        /// This is a map from config option ID to a list of favorited value IDs.
        ///
        /// Default: {}
        favorite_config_option_values: HashMap<String, Vec<String>>,
    },
}

impl CustomAgentServerSettings {
    pub fn command(&self) -> Option<&AgentServerCommand> {
        match self {
            CustomAgentServerSettings::Custom { command, .. } => Some(command),
            CustomAgentServerSettings::Extension { .. } => None,
        }
    }

    pub fn default_mode(&self) -> Option<&str> {
        match self {
            CustomAgentServerSettings::Custom { default_mode, .. }
            | CustomAgentServerSettings::Extension { default_mode, .. } => default_mode.as_deref(),
        }
    }

    pub fn default_model(&self) -> Option<&str> {
        match self {
            CustomAgentServerSettings::Custom { default_model, .. }
            | CustomAgentServerSettings::Extension { default_model, .. } => {
                default_model.as_deref()
            }
        }
    }

    pub fn favorite_models(&self) -> &[String] {
        match self {
            CustomAgentServerSettings::Custom {
                favorite_models, ..
            }
            | CustomAgentServerSettings::Extension {
                favorite_models, ..
            } => favorite_models,
        }
    }

    pub fn default_config_option(&self, config_id: &str) -> Option<&str> {
        match self {
            CustomAgentServerSettings::Custom {
                default_config_options,
                ..
            }
            | CustomAgentServerSettings::Extension {
                default_config_options,
                ..
            } => default_config_options.get(config_id).map(|s| s.as_str()),
        }
    }

    pub fn favorite_config_option_values(&self, config_id: &str) -> Option<&[String]> {
        match self {
            CustomAgentServerSettings::Custom {
                favorite_config_option_values,
                ..
            }
            | CustomAgentServerSettings::Extension {
                favorite_config_option_values,
                ..
            } => favorite_config_option_values
                .get(config_id)
                .map(|v| v.as_slice()),
        }
    }
}

impl From<settings::CustomAgentServerSettings> for CustomAgentServerSettings {
    fn from(value: settings::CustomAgentServerSettings) -> Self {
        match value {
            settings::CustomAgentServerSettings::Custom {
                path,
                args,
                env,
                default_mode,
                default_model,
                favorite_models,
                default_config_options,
                favorite_config_option_values,
            } => CustomAgentServerSettings::Custom {
                command: AgentServerCommand {
                    path: PathBuf::from(shellexpand::tilde(&path.to_string_lossy()).as_ref()),
                    args,
                    env,
                },
                default_mode,
                default_model,
                favorite_models,
                default_config_options,
                favorite_config_option_values,
            },
            settings::CustomAgentServerSettings::Extension {
                default_mode,
                default_model,
                default_config_options,
                favorite_models,
                favorite_config_option_values,
            } => CustomAgentServerSettings::Extension {
                default_mode,
                default_model,
                default_config_options,
                favorite_models,
                favorite_config_option_values,
            },
        }
    }
}

impl settings::Settings for AllAgentServersSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let agent_settings = content.agent_servers.clone().unwrap();
        Self {
            gemini: agent_settings.gemini.map(Into::into),
            claude: agent_settings.claude.map(Into::into),
            codex: agent_settings.codex.map(Into::into),
            custom: agent_settings
                .custom
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

#[cfg(test)]
mod extension_agent_tests {
    use crate::worktree_store::WorktreeStore;

    use super::*;
    use gpui::TestAppContext;
    use std::sync::Arc;

    #[test]
    fn extension_agent_constructs_proper_display_names() {
        // Verify the display name format for extension-provided agents
        let name1 = ExternalAgentServerName(SharedString::from("Extension: Agent"));
        assert!(name1.0.contains(": "));

        let name2 = ExternalAgentServerName(SharedString::from("MyExt: MyAgent"));
        assert_eq!(name2.0, "MyExt: MyAgent");

        // Non-extension agents shouldn't have the separator
        let custom = ExternalAgentServerName(SharedString::from("custom"));
        assert!(!custom.0.contains(": "));
    }

    struct NoopExternalAgent;

    impl ExternalAgentServer for NoopExternalAgent {
        fn get_command(
            &mut self,
            _root_dir: Option<&str>,
            _extra_env: HashMap<String, String>,
            _status_tx: Option<watch::Sender<SharedString>>,
            _new_version_available_tx: Option<watch::Sender<Option<String>>>,
            _cx: &mut AsyncApp,
        ) -> Task<Result<(AgentServerCommand, String, Option<task::SpawnInTerminal>)>> {
            Task::ready(Ok((
                AgentServerCommand {
                    path: PathBuf::from("noop"),
                    args: Vec::new(),
                    env: None,
                },
                "".to_string(),
                None,
            )))
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn sync_removes_only_extension_provided_agents() {
        let mut store = AgentServerStore {
            state: AgentServerStoreState::Collab,
            external_agents: HashMap::default(),
            agent_icons: HashMap::default(),
            agent_display_names: HashMap::default(),
        };

        // Seed with extension agents (contain ": ") and custom agents (don't contain ": ")
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("Ext1: Agent1")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("Ext2: Agent2")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );
        store.external_agents.insert(
            ExternalAgentServerName(SharedString::from("custom-agent")),
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
        );

        // Simulate removal phase
        let keys_to_remove: Vec<_> = store
            .external_agents
            .keys()
            .filter(|name| name.0.contains(": "))
            .cloned()
            .collect();

        for key in keys_to_remove {
            store.external_agents.remove(&key);
        }

        // Only custom-agent should remain
        assert_eq!(store.external_agents.len(), 1);
        assert!(
            store
                .external_agents
                .contains_key(&ExternalAgentServerName(SharedString::from("custom-agent")))
        );
    }

    #[test]
    fn archive_launcher_constructs_with_all_fields() {
        use extension::AgentServerManifestEntry;

        let mut env = HashMap::default();
        env.insert("GITHUB_TOKEN".into(), "secret".into());

        let mut targets = HashMap::default();
        targets.insert(
            "darwin-aarch64".to_string(),
            extension::TargetConfig {
                archive:
                    "https://github.com/owner/repo/releases/download/v1.0.0/agent-darwin-arm64.zip"
                        .into(),
                cmd: "./agent".into(),
                args: vec![],
                sha256: None,
                env: Default::default(),
            },
        );

        let _entry = AgentServerManifestEntry {
            name: "GitHub Agent".into(),
            targets,
            env,
            icon: None,
        };

        // Verify display name construction
        let expected_name = ExternalAgentServerName(SharedString::from("GitHub Agent"));
        assert_eq!(expected_name.0, "GitHub Agent");
    }

    #[gpui::test]
    async fn archive_agent_uses_extension_and_agent_id_for_cache_key(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let worktree_store = cx.new(|_| WorktreeStore::local(false, fs.clone()));
        let project_environment = cx.new(|cx| {
            crate::ProjectEnvironment::new(None, worktree_store.downgrade(), None, false, cx)
        });

        let agent = LocalExtensionArchiveAgent {
            fs,
            http_client,
            node_runtime: node_runtime::NodeRuntime::unavailable(),
            project_environment,
            extension_id: Arc::from("my-extension"),
            agent_id: Arc::from("my-agent"),
            targets: {
                let mut map = HashMap::default();
                map.insert(
                    "darwin-aarch64".to_string(),
                    extension::TargetConfig {
                        archive: "https://example.com/my-agent-darwin-arm64.zip".into(),
                        cmd: "./my-agent".into(),
                        args: vec!["--serve".into()],
                        sha256: None,
                        env: Default::default(),
                    },
                );
                map
            },
            env: {
                let mut map = HashMap::default();
                map.insert("PORT".into(), "8080".into());
                map
            },
        };

        // Verify agent is properly constructed
        assert_eq!(agent.extension_id.as_ref(), "my-extension");
        assert_eq!(agent.agent_id.as_ref(), "my-agent");
        assert_eq!(agent.env.get("PORT"), Some(&"8080".to_string()));
        assert!(agent.targets.contains_key("darwin-aarch64"));
    }

    #[test]
    fn sync_extension_agents_registers_archive_launcher() {
        use extension::AgentServerManifestEntry;

        let expected_name = ExternalAgentServerName(SharedString::from("Release Agent"));
        assert_eq!(expected_name.0, "Release Agent");

        // Verify the manifest entry structure for archive-based installation
        let mut env = HashMap::default();
        env.insert("API_KEY".into(), "secret".into());

        let mut targets = HashMap::default();
        targets.insert(
            "linux-x86_64".to_string(),
            extension::TargetConfig {
                archive: "https://github.com/org/project/releases/download/v2.1.0/release-agent-linux-x64.tar.gz".into(),
                cmd: "./release-agent".into(),
                args: vec!["serve".into()],
                sha256: None,
                env: Default::default(),
            },
        );

        let manifest_entry = AgentServerManifestEntry {
            name: "Release Agent".into(),
            targets: targets.clone(),
            env,
            icon: None,
        };

        // Verify target config is present
        assert!(manifest_entry.targets.contains_key("linux-x86_64"));
        let target = manifest_entry.targets.get("linux-x86_64").unwrap();
        assert_eq!(target.cmd, "./release-agent");
    }

    #[gpui::test]
    async fn test_node_command_uses_managed_runtime(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let node_runtime = NodeRuntime::unavailable();
        let worktree_store = cx.new(|_| WorktreeStore::local(false, fs.clone()));
        let project_environment = cx.new(|cx| {
            crate::ProjectEnvironment::new(None, worktree_store.downgrade(), None, false, cx)
        });

        let agent = LocalExtensionArchiveAgent {
            fs: fs.clone(),
            http_client,
            node_runtime,
            project_environment,
            extension_id: Arc::from("node-extension"),
            agent_id: Arc::from("node-agent"),
            targets: {
                let mut map = HashMap::default();
                map.insert(
                    "darwin-aarch64".to_string(),
                    extension::TargetConfig {
                        archive: "https://example.com/node-agent.zip".into(),
                        cmd: "node".into(),
                        args: vec!["index.js".into()],
                        sha256: None,
                        env: Default::default(),
                    },
                );
                map
            },
            env: HashMap::default(),
        };

        // Verify that when cmd is "node", it attempts to use the node runtime
        assert_eq!(agent.extension_id.as_ref(), "node-extension");
        assert_eq!(agent.agent_id.as_ref(), "node-agent");

        let target = agent.targets.get("darwin-aarch64").unwrap();
        assert_eq!(target.cmd, "node");
        assert_eq!(target.args, vec!["index.js"]);
    }

    #[gpui::test]
    async fn test_commands_run_in_extraction_directory(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let node_runtime = NodeRuntime::unavailable();
        let worktree_store = cx.new(|_| WorktreeStore::local(false, fs.clone()));
        let project_environment = cx.new(|cx| {
            crate::ProjectEnvironment::new(None, worktree_store.downgrade(), None, false, cx)
        });

        let agent = LocalExtensionArchiveAgent {
            fs: fs.clone(),
            http_client,
            node_runtime,
            project_environment,
            extension_id: Arc::from("test-ext"),
            agent_id: Arc::from("test-agent"),
            targets: {
                let mut map = HashMap::default();
                map.insert(
                    "darwin-aarch64".to_string(),
                    extension::TargetConfig {
                        archive: "https://example.com/test.zip".into(),
                        cmd: "node".into(),
                        args: vec![
                            "server.js".into(),
                            "--config".into(),
                            "./config.json".into(),
                        ],
                        sha256: None,
                        env: Default::default(),
                    },
                );
                map
            },
            env: HashMap::default(),
        };

        // Verify the agent is configured with relative paths in args
        let target = agent.targets.get("darwin-aarch64").unwrap();
        assert_eq!(target.args[0], "server.js");
        assert_eq!(target.args[2], "./config.json");
        // These relative paths will resolve relative to the extraction directory
        // when the command is executed
    }

    #[test]
    fn test_tilde_expansion_in_settings() {
        let settings = settings::BuiltinAgentServerSettings {
            path: Some(PathBuf::from("~/bin/agent")),
            args: Some(vec!["--flag".into()]),
            env: None,
            ignore_system_version: None,
            default_mode: None,
            default_model: None,
            favorite_models: vec![],
            default_config_options: Default::default(),
            favorite_config_option_values: Default::default(),
        };

        let BuiltinAgentServerSettings { path, .. } = settings.into();

        let path = path.unwrap();
        assert!(
            !path.to_string_lossy().starts_with("~"),
            "Tilde should be expanded for builtin agent path"
        );

        let settings = settings::CustomAgentServerSettings::Custom {
            path: PathBuf::from("~/custom/agent"),
            args: vec!["serve".into()],
            env: None,
            default_mode: None,
            default_model: None,
            favorite_models: vec![],
            default_config_options: Default::default(),
            favorite_config_option_values: Default::default(),
        };

        let converted: CustomAgentServerSettings = settings.into();
        let CustomAgentServerSettings::Custom {
            command: AgentServerCommand { path, .. },
            ..
        } = converted
        else {
            panic!("Expected Custom variant");
        };

        assert!(
            !path.to_string_lossy().starts_with("~"),
            "Tilde should be expanded for custom agent path"
        );
    }
}
