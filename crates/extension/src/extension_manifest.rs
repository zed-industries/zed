use anyhow::{Context as _, Result, anyhow, bail};
use collections::{BTreeMap, HashMap};
use fs::Fs;
use language::LanguageName;
use lsp::LanguageServerName;
use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::ExtensionCapability;

/// This is the old version of the extension manifest, from when it was `extension.json`.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OldExtensionManifest {
    pub name: String,
    pub version: Arc<str>,

    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,

    #[serde(default)]
    pub themes: BTreeMap<Arc<str>, PathBuf>,
    #[serde(default)]
    pub languages: BTreeMap<Arc<str>, PathBuf>,
    #[serde(default)]
    pub grammars: BTreeMap<Arc<str>, PathBuf>,
}

/// The schema version of the [`ExtensionManifest`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct SchemaVersion(pub i32);

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SchemaVersion {
    pub const ZERO: Self = Self(0);

    pub fn is_v0(&self) -> bool {
        self == &Self::ZERO
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: Arc<str>,
    pub name: String,
    pub version: Arc<str>,
    pub schema_version: SchemaVersion,

    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub lib: LibManifestEntry,

    #[serde(default)]
    pub themes: Vec<PathBuf>,
    #[serde(default)]
    pub icon_themes: Vec<PathBuf>,
    #[serde(default)]
    pub languages: Vec<PathBuf>,
    #[serde(default)]
    pub grammars: BTreeMap<Arc<str>, GrammarManifestEntry>,
    #[serde(default)]
    pub language_servers: BTreeMap<LanguageServerName, LanguageServerManifestEntry>,
    #[serde(default)]
    pub context_servers: BTreeMap<Arc<str>, ContextServerManifestEntry>,
    #[serde(default)]
    pub agent_servers: BTreeMap<Arc<str>, AgentServerManifestEntry>,
    #[serde(default)]
    pub slash_commands: BTreeMap<Arc<str>, SlashCommandManifestEntry>,
    #[serde(default)]
    pub snippets: Option<PathBuf>,
    #[serde(default)]
    pub capabilities: Vec<ExtensionCapability>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub debug_adapters: BTreeMap<Arc<str>, DebugAdapterManifestEntry>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub debug_locators: BTreeMap<Arc<str>, DebugLocatorManifestEntry>,
}

impl ExtensionManifest {
    pub fn allow_exec(
        &self,
        desired_command: &str,
        desired_args: &[impl AsRef<str> + std::fmt::Debug],
    ) -> Result<()> {
        let is_allowed = self.capabilities.iter().any(|capability| match capability {
            ExtensionCapability::ProcessExec(capability) => {
                capability.allows(desired_command, desired_args)
            }
            _ => false,
        });

        if !is_allowed {
            bail!(
                "capability for process:exec {desired_command} {desired_args:?} was not listed in the extension manifest",
            );
        }

        Ok(())
    }

    pub fn allow_remote_load(&self) -> bool {
        !self.language_servers.is_empty()
            || !self.debug_adapters.is_empty()
            || !self.debug_locators.is_empty()
    }
}

pub fn build_debug_adapter_schema_path(
    adapter_name: &Arc<str>,
    meta: &DebugAdapterManifestEntry,
) -> PathBuf {
    meta.schema_path.clone().unwrap_or_else(|| {
        Path::new("debug_adapter_schemas")
            .join(Path::new(adapter_name.as_ref()).with_extension("json"))
    })
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LibManifestEntry {
    pub kind: Option<ExtensionLibraryKind>,
    pub version: Option<SemanticVersion>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct AgentServerManifestEntry {
    /// How to install and launch the agent server.
    pub launcher: AgentServerLauncher,
    /// Environment variables to set when launching the agent server.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Command-line arguments to pass to the agent server.
    #[serde(default)]
    pub args: Vec<String>,
    /// Terminal-based authentication commands for specific auth methods.
    ///
    /// Each entry maps an ACP auth method ID (returned by the agent in its `InitializeResponse`)
    /// to the terminal command that should be run when that auth method requires interactive
    /// authentication. This is typically used for OAuth flows or credential entry that can't
    /// happen over the ACP protocol itself.
    ///
    /// When an auth method from this list is triggered (either by the user clicking it in the UI
    /// or when the agent requires authentication), Zed will spawn the specified terminal command
    /// instead of calling the ACP `authenticate` method.
    ///
    /// Multiple auth methods can be configured, each with their own terminal command.
    /// If an auth method doesn't appear in this list, Zed will use the standard ACP
    /// `authenticate` call for it.
    #[serde(default)]
    pub auth_commands: Vec<AgentServerAuthCommand>,
    /// Whether to skip checking for system-installed versions of this agent.
    /// When true, always uses the extension-installed version.
    #[serde(default)]
    pub ignore_system_version: Option<bool>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AgentServerLauncher {
    Npm {
        package: String,
        entrypoint: String,
        min_version: String,
    },
    GithubRelease {
        repo: String,
        asset_pattern: String,
        binary_name: String,
    },
    Binary {
        bin_name: String,
    },
}

/// Configuration for a terminal-based authentication command.
///
/// This defines a separate terminal command that runs for authentication purposes,
/// mapped to a specific ACP auth method ID. The command is executed outside of the
/// ACP protocol, typically in a terminal panel where the user can interact with an
/// OAuth flow or enter credentials.
///
/// # Relationship to ACP Auth Methods
///
/// The agent declares its available auth methods in the ACP `InitializeResponse`.
/// Each auth method has an ID (e.g., "oauth-personal", "claude-login") that the user
/// can select. Most auth methods use the standard ACP `authenticate` call. However,
/// some auth methods (particularly OAuth flows) need to run in a terminal for user
/// interaction. This struct maps those specific auth method IDs to their terminal commands.
///
/// # Examples
///
/// **Gemini-style (same binary, different args):**
/// ```toml
/// [[agent_servers.my-gemini.auth_commands]]
/// auth_method_id = "oauth-personal"
/// label = "gemini /auth"
/// # No command override - runs the same binary but without --experimental-acp
/// ```
///
/// **Claude-style (different script from same package):**
/// ```toml
/// [[agent_servers.my-claude.auth_commands]]
/// auth_method_id = "claude-login"
/// label = "claude /login"
/// args = ["node_modules/@anthropic-ai/claude-agent-sdk/cli.js", "/login"]
/// ```
///
/// **Separate auth binary:**
/// ```toml
/// [[agent_servers.my-agent.auth_commands]]
/// auth_method_id = "oauth"
/// label = "my-agent login"
/// command = "my-agent-auth"
/// args = ["--interactive"]
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct AgentServerAuthCommand {
    /// The ACP auth method ID this command is for (e.g., "oauth-personal", "claude-login").
    ///
    /// This must exactly match an auth method ID that the agent returns in its
    /// `InitializeResponse.auth_methods` array. When the user selects this auth method
    /// and it requires terminal interaction, Zed will execute this command instead of
    /// calling the ACP `authenticate` method.
    pub auth_method_id: String,
    /// Label displayed to the user (e.g., "gemini /auth" or "claude /login").
    pub label: String,
    /// Optional override for the command to run.
    /// If `None`, uses the same command as the main agent launcher.
    /// If `Some`, uses the specified command instead (e.g., a separate auth tool).
    #[serde(default)]
    pub command: Option<String>,
    /// Arguments to pass to the auth command.
    /// For npm agents without a custom command, this can specify a different JS file to run.
    #[serde(default)]
    pub args: Vec<String>,
    /// Additional environment variables for the auth command.
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum ExtensionLibraryKind {
    Rust,
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct GrammarManifestEntry {
    pub repository: String,
    #[serde(alias = "commit")]
    pub rev: String,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LanguageServerManifestEntry {
    /// Deprecated in favor of `languages`.
    #[serde(default)]
    language: Option<LanguageName>,
    /// The list of languages this language server should work with.
    #[serde(default)]
    languages: Vec<LanguageName>,
    #[serde(default)]
    pub language_ids: HashMap<LanguageName, String>,
    #[serde(default)]
    pub code_action_kinds: Option<Vec<lsp::CodeActionKind>>,
}

impl LanguageServerManifestEntry {
    /// Returns the list of languages for the language server.
    ///
    /// Prefer this over accessing the `language` or `languages` fields directly,
    /// as we currently support both.
    ///
    /// We can replace this with just field access for the `languages` field once
    /// we have removed `language`.
    pub fn languages(&self) -> impl IntoIterator<Item = LanguageName> + '_ {
        let language = if self.languages.is_empty() {
            self.language.clone()
        } else {
            None
        };
        self.languages.iter().cloned().chain(language)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ContextServerManifestEntry {}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct SlashCommandManifestEntry {
    pub description: String,
    pub requires_argument: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct DebugAdapterManifestEntry {
    pub schema_path: Option<PathBuf>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct DebugLocatorManifestEntry {}

impl ExtensionManifest {
    pub async fn load(fs: Arc<dyn Fs>, extension_dir: &Path) -> Result<Self> {
        let extension_name = extension_dir
            .file_name()
            .and_then(OsStr::to_str)
            .context("invalid extension name")?;

        let mut extension_manifest_path = extension_dir.join("extension.json");
        if fs.is_file(&extension_manifest_path).await {
            let manifest_content = fs
                .load(&extension_manifest_path)
                .await
                .with_context(|| format!("failed to load {extension_name} extension.json"))?;
            let manifest_json = serde_json::from_str::<OldExtensionManifest>(&manifest_content)
                .with_context(|| {
                    format!("invalid extension.json for extension {extension_name}")
                })?;

            Ok(manifest_from_old_manifest(manifest_json, extension_name))
        } else {
            extension_manifest_path.set_extension("toml");
            let manifest_content = fs
                .load(&extension_manifest_path)
                .await
                .with_context(|| format!("failed to load {extension_name} extension.toml"))?;
            toml::from_str(&manifest_content).map_err(|err| {
                anyhow!("Invalid extension.toml for extension {extension_name}:\n{err}")
            })
        }
    }
}

fn manifest_from_old_manifest(
    manifest_json: OldExtensionManifest,
    extension_id: &str,
) -> ExtensionManifest {
    ExtensionManifest {
        id: extension_id.into(),
        name: manifest_json.name,
        version: manifest_json.version,
        description: manifest_json.description,
        repository: manifest_json.repository,
        authors: manifest_json.authors,
        schema_version: SchemaVersion::ZERO,
        lib: Default::default(),
        themes: {
            let mut themes = manifest_json.themes.into_values().collect::<Vec<_>>();
            themes.sort();
            themes.dedup();
            themes
        },
        icon_themes: Vec::new(),
        languages: {
            let mut languages = manifest_json.languages.into_values().collect::<Vec<_>>();
            languages.sort();
            languages.dedup();
            languages
        },
        grammars: manifest_json
            .grammars
            .into_keys()
            .map(|grammar_name| (grammar_name, Default::default()))
            .collect(),
        language_servers: Default::default(),
        context_servers: BTreeMap::default(),
        agent_servers: BTreeMap::default(),
        slash_commands: BTreeMap::default(),
        snippets: None,
        capabilities: Vec::new(),
        debug_adapters: Default::default(),
        debug_locators: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::ProcessExecCapability;

    use super::*;

    fn extension_manifest() -> ExtensionManifest {
        ExtensionManifest {
            id: "test".into(),
            name: "Test".to_string(),
            version: "1.0.0".into(),
            schema_version: SchemaVersion::ZERO,
            description: None,
            repository: None,
            authors: vec![],
            lib: Default::default(),
            themes: vec![],
            icon_themes: vec![],
            languages: vec![],
            grammars: BTreeMap::default(),
            language_servers: BTreeMap::default(),
            context_servers: BTreeMap::default(),
            agent_servers: BTreeMap::default(),
            slash_commands: BTreeMap::default(),
            snippets: None,
            capabilities: vec![],
            debug_adapters: Default::default(),
            debug_locators: Default::default(),
        }
    }

    #[test]
    fn test_build_adapter_schema_path_with_schema_path() {
        let adapter_name = Arc::from("my_adapter");
        let entry = DebugAdapterManifestEntry {
            schema_path: Some(PathBuf::from("foo/bar")),
        };

        let path = build_debug_adapter_schema_path(&adapter_name, &entry);
        assert_eq!(path, PathBuf::from("foo/bar"));
    }

    #[test]
    fn test_build_adapter_schema_path_without_schema_path() {
        let adapter_name = Arc::from("my_adapter");
        let entry = DebugAdapterManifestEntry { schema_path: None };

        let path = build_debug_adapter_schema_path(&adapter_name, &entry);
        assert_eq!(
            path,
            PathBuf::from("debug_adapter_schemas").join("my_adapter.json")
        );
    }

    #[test]
    fn test_allow_exec_exact_match() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec(ProcessExecCapability {
                command: "ls".to_string(),
                args: vec!["-la".to_string()],
            })],
            ..extension_manifest()
        };

        assert!(manifest.allow_exec("ls", &["-la"]).is_ok());
        assert!(manifest.allow_exec("ls", &["-l"]).is_err());
        assert!(manifest.allow_exec("pwd", &[] as &[&str]).is_err());
    }

    #[test]
    fn test_allow_exec_wildcard_arg() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec(ProcessExecCapability {
                command: "git".to_string(),
                args: vec!["*".to_string()],
            })],
            ..extension_manifest()
        };

        assert!(manifest.allow_exec("git", &["status"]).is_ok());
        assert!(manifest.allow_exec("git", &["commit"]).is_ok());
        assert!(manifest.allow_exec("git", &["status", "-s"]).is_err()); // too many args
        assert!(manifest.allow_exec("npm", &["install"]).is_err()); // wrong command
    }

    #[test]
    fn test_allow_exec_double_wildcard() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec(ProcessExecCapability {
                command: "cargo".to_string(),
                args: vec!["test".to_string(), "**".to_string()],
            })],
            ..extension_manifest()
        };

        assert!(manifest.allow_exec("cargo", &["test"]).is_ok());
        assert!(manifest.allow_exec("cargo", &["test", "--all"]).is_ok());
        assert!(
            manifest
                .allow_exec("cargo", &["test", "--all", "--no-fail-fast"])
                .is_ok()
        );
        assert!(manifest.allow_exec("cargo", &["build"]).is_err()); // wrong first arg
    }

    #[test]
    fn test_allow_exec_mixed_wildcards() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec(ProcessExecCapability {
                command: "docker".to_string(),
                args: vec!["run".to_string(), "*".to_string(), "**".to_string()],
            })],
            ..extension_manifest()
        };

        assert!(manifest.allow_exec("docker", &["run", "nginx"]).is_ok());
        assert!(manifest.allow_exec("docker", &["run"]).is_err());
        assert!(
            manifest
                .allow_exec("docker", &["run", "ubuntu", "bash"])
                .is_ok()
        );
        assert!(
            manifest
                .allow_exec("docker", &["run", "alpine", "sh", "-c", "echo hello"])
                .is_ok()
        );
        assert!(manifest.allow_exec("docker", &["ps"]).is_err()); // wrong first arg
    }
    #[test]
    fn parse_manifest_with_agent_server_npm_launcher() {
        let toml_src = r#"
id = "example.agent-server-ext"
name = "Agent Server Example"
version = "1.0.0"
schema_version = 0

[agent_servers.foo]
[agent_servers.foo.launcher]
package = "@example/agent-server"
entrypoint = "node_modules/@example/agent-server/dist/index.js"
min_version = "1.0.0"
# removed: experimental flag should be provided by extension args if needed
"#;

        let manifest: ExtensionManifest = toml::from_str(toml_src).expect("manifest should parse");
        assert_eq!(manifest.id.as_ref(), "example.agent-server-ext");
        assert!(manifest.agent_servers.contains_key("foo"));
        let entry = manifest.agent_servers.get("foo").unwrap();
        match &entry.launcher {
            AgentServerLauncher::Npm {
                package,
                entrypoint,
                min_version,
            } => {
                assert_eq!(package, "@example/agent-server");
                assert_eq!(
                    entrypoint,
                    "node_modules/@example/agent-server/dist/index.js"
                );
                assert_eq!(min_version, "1.0.0");
                assert!(entry.args.is_empty());
            }
            _ => panic!("expected Npm launcher"),
        }
    }

    #[test]
    fn parse_manifest_with_agent_server_login() {
        let toml_src = r#"
id = "example.agent-with-login"
name = "Agent With Login"
version = "1.0.0"
schema_version = 0

[agent_servers.my-agent]
args = ["--acp"]

[agent_servers.my-agent.launcher]
bin_name = "my-agent"

[[agent_servers.my-agent.auth_commands]]
auth_method_id = "oauth"
label = "my-agent login"
command = "my-agent-auth"
args = ["--interactive"]

[agent_servers.my-agent.auth_commands.env]
AUTH_MODE = "oauth"

[agent_servers.gemini-style]
args = ["--experimental-acp"]

[agent_servers.gemini-style.launcher]
package = "@example/gemini-fork"
entrypoint = "node_modules/@example/gemini-fork/dist/index.js"
min_version = "0.9.0"

[[agent_servers.gemini-style.auth_commands]]
auth_method_id = "oauth-personal"
label = "gemini /auth"
# No command - uses same node + entrypoint but without --experimental-acp
"#;

        let manifest: ExtensionManifest = toml::from_str(toml_src).expect("manifest should parse");
        assert_eq!(manifest.id.as_ref(), "example.agent-with-login");

        // Test binary agent with custom auth command
        let binary_agent = manifest.agent_servers.get("my-agent").unwrap();
        assert_eq!(binary_agent.auth_commands.len(), 1);
        let auth_cmd = &binary_agent.auth_commands[0];
        assert_eq!(auth_cmd.auth_method_id, "oauth");
        assert_eq!(auth_cmd.label, "my-agent login");
        assert_eq!(auth_cmd.command, Some("my-agent-auth".to_string()));
        assert_eq!(auth_cmd.args, vec!["--interactive"]);
        assert_eq!(auth_cmd.env.get("AUTH_MODE"), Some(&"oauth".to_string()));

        // Test NPM agent with default auth command
        let npm_agent = manifest.agent_servers.get("gemini-style").unwrap();
        assert_eq!(npm_agent.auth_commands.len(), 1);
        let npm_auth_cmd = &npm_agent.auth_commands[0];
        assert_eq!(npm_auth_cmd.auth_method_id, "oauth-personal");
        assert_eq!(npm_auth_cmd.label, "gemini /auth");
        assert_eq!(npm_auth_cmd.command, None); // Uses main command
        assert!(npm_auth_cmd.args.is_empty());
    }
}
