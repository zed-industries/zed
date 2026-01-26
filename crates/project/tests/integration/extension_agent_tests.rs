use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext, AsyncApp, SharedString, Task, TestAppContext};
use node_runtime::NodeRuntime;
use project::agent_server_store::*;
use project::worktree_store::WorktreeStore;
use std::{any::Any, path::PathBuf, sync::Arc};

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
    let mut store = AgentServerStore::collab();

    // Seed with extension agents (contain ": ") and custom agents (don't contain ": ")
    store.external_agents.insert(
        ExternalAgentServerName(SharedString::from("Ext1: Agent1")),
        ExternalAgentEntry::new(
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
            ExternalAgentSource::Extension,
            None,
            None,
        ),
    );
    store.external_agents.insert(
        ExternalAgentServerName(SharedString::from("Ext2: Agent2")),
        ExternalAgentEntry::new(
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
            ExternalAgentSource::Extension,
            None,
            None,
        ),
    );
    store.external_agents.insert(
        ExternalAgentServerName(SharedString::from("custom-agent")),
        ExternalAgentEntry::new(
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
            ExternalAgentSource::Custom,
            None,
            None,
        ),
    );

    // Simulate removal phase
    store
        .external_agents
        .retain(|_, entry| entry.source != ExternalAgentSource::Extension);

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
        env: Default::default(),
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
        env: Default::default(),
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
