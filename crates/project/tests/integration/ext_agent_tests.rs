use anyhow::Result;
use collections::HashMap;
use gpui::{AsyncApp, SharedString, Task};
use project::agent_server_store::*;
use std::{any::Any, collections::HashSet, fmt::Write as _, path::PathBuf};
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
    let mut store = AgentServerStore::collab();

    // Seed with a couple of agents that will be replaced by extensions
    store.external_agents.insert(
        ExternalAgentServerName(SharedString::from("foo-agent")),
        ExternalAgentEntry::new(
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
            ExternalAgentSource::Custom,
            None,
            None,
        ),
    );
    store.external_agents.insert(
        ExternalAgentServerName(SharedString::from("bar-agent")),
        ExternalAgentEntry::new(
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
            ExternalAgentSource::Custom,
            None,
            None,
        ),
    );
    store.external_agents.insert(
        ExternalAgentServerName(SharedString::from("custom")),
        ExternalAgentEntry::new(
            Box::new(NoopExternalAgent) as Box<dyn ExternalAgentServer>,
            ExternalAgentSource::Custom,
            None,
            None,
        ),
    );

    // Simulate the removal phase: if we're syncing extensions that provide
    // "foo-agent" and "bar-agent", those should be removed first
    let extension_agent_names: HashSet<String> = ["foo-agent".to_string(), "bar-agent".to_string()]
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
    let result = project::agent_server_store::resolve_extension_icon_path(
        extensions_dir,
        "my-extension",
        "icon.svg",
    );
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

    let result = project::agent_server_store::resolve_extension_icon_path(
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
    let result = project::agent_server_store::resolve_extension_icon_path(
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
    let result = project::agent_server_store::resolve_extension_icon_path(
        extensions_dir,
        "my-extension",
        "../outside.svg",
    );
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
    let result = project::agent_server_store::resolve_extension_icon_path(
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
    let result = project::agent_server_store::resolve_extension_icon_path(
        extensions_dir,
        "my-extension",
        "nonexistent.svg",
    );
    assert!(result.is_none(), "Nonexistent file should return None");
}
