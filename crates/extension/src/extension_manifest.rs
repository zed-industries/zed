use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow, bail};
use cloud_api_types::ExtensionProvides;
use collections::{BTreeMap, BTreeSet, HashMap};
use fs::Fs;
use language::LanguageName;
use lsp::LanguageServerName;
use semver::Version;
use serde::{Deserialize, Serialize};
use util::rel_path::{PathExt, RelPathBuf};

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
    pub themes: BTreeMap<Arc<str>, RelPathBuf>,
    #[serde(default)]
    pub languages: BTreeMap<Arc<str>, RelPathBuf>,
    #[serde(default)]
    pub grammars: BTreeMap<Arc<str>, RelPathBuf>,
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

// TODO: We should change this to just always be a Vec<PathBuf> once we bump the
// extension.toml schema version to 2
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtensionSnippets {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

impl ExtensionSnippets {
    pub fn paths(&self) -> impl Iterator<Item = &PathBuf> {
        match self {
            ExtensionSnippets::Single(path) => std::slice::from_ref(path).iter(),
            ExtensionSnippets::Multiple(paths) => paths.iter(),
        }
    }
}

impl From<&str> for ExtensionSnippets {
    fn from(value: &str) -> Self {
        ExtensionSnippets::Single(value.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(try_from = "RawExtensionManifest")]
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
    pub themes: Vec<RelPathBuf>,
    #[serde(default)]
    pub icon_themes: Vec<RelPathBuf>,
    #[serde(default)]
    pub languages: Vec<RelPathBuf>,
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
    pub snippets: Option<ExtensionSnippets>,
    #[serde(default)]
    pub capabilities: Vec<ExtensionCapability>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub debug_adapters: BTreeMap<Arc<str>, DebugAdapterManifestEntry>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub debug_locators: BTreeMap<Arc<str>, DebugLocatorManifestEntry>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub language_model_providers: BTreeMap<Arc<str>, LanguageModelProviderManifestEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<DevManifestEntry>,
}

pub enum GrammarSource<'a> {
    Dev(&'a DevGrammarManifestEntry),
    Published(&'a GrammarManifestEntry),
}

#[derive(Deserialize)]
struct RawExtensionManifest {
    id: Arc<str>,
    name: String,
    version: Arc<str>,
    schema_version: SchemaVersion,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    repository: Option<String>,
    #[serde(default)]
    authors: Vec<String>,
    #[serde(default)]
    lib: LibManifestEntry,
    #[serde(default)]
    themes: Vec<PathBuf>,
    #[serde(default)]
    icon_themes: Vec<PathBuf>,
    #[serde(default)]
    languages: Vec<PathBuf>,
    #[serde(default)]
    grammars: BTreeMap<Arc<str>, GrammarManifestEntry>,
    #[serde(default)]
    language_servers: BTreeMap<LanguageServerName, LanguageServerManifestEntry>,
    #[serde(default)]
    context_servers: BTreeMap<Arc<str>, ContextServerManifestEntry>,
    #[serde(default)]
    agent_servers: BTreeMap<Arc<str>, AgentServerManifestEntry>,
    #[serde(default)]
    slash_commands: BTreeMap<Arc<str>, SlashCommandManifestEntry>,
    #[serde(default)]
    snippets: Option<ExtensionSnippets>,
    #[serde(default)]
    capabilities: Vec<ExtensionCapability>,
    #[serde(default)]
    debug_adapters: BTreeMap<Arc<str>, DebugAdapterManifestEntry>,
    #[serde(default)]
    debug_locators: BTreeMap<Arc<str>, DebugLocatorManifestEntry>,
    #[serde(default)]
    language_model_providers: BTreeMap<Arc<str>, LanguageModelProviderManifestEntry>,
    #[serde(default)]
    dev: Option<DevManifestEntry>,
}

impl TryFrom<RawExtensionManifest> for ExtensionManifest {
    type Error = String;

    fn try_from(raw: RawExtensionManifest) -> std::result::Result<Self, Self::Error> {
        if !raw.grammars.is_empty() && raw.dev.as_ref().is_some_and(|d| !d.grammars.is_empty()) {
            return Err(
                "manifest cannot specify both [grammars] and [dev.grammars]; \
                 use [grammars] for published extensions or [dev.grammars] for dev extensions"
                    .to_string(),
            );
        }

        if raw.dev.as_ref().is_some_and(|d| d.grammars.is_empty()) && raw.grammars.is_empty() {
            return Err(
                "manifest with a [dev] section must specify at least one grammar in \
                 [dev.grammars] or [grammars]"
                    .to_string(),
            );
        }

        Ok(ExtensionManifest {
            id: raw.id,
            name: raw.name,
            version: raw.version,
            schema_version: raw.schema_version,
            description: raw.description,
            repository: raw.repository,
            authors: raw.authors,
            lib: raw.lib,
            themes: raw.themes,
            icon_themes: raw.icon_themes,
            languages: raw.languages,
            grammars: raw.grammars,
            language_servers: raw.language_servers,
            context_servers: raw.context_servers,
            agent_servers: raw.agent_servers,
            slash_commands: raw.slash_commands,
            snippets: raw.snippets,
            capabilities: raw.capabilities,
            debug_adapters: raw.debug_adapters,
            debug_locators: raw.debug_locators,
            language_model_providers: raw.language_model_providers,
            dev: raw.dev,
        })
    }
}

impl ExtensionManifest {
    pub fn is_dev(&self) -> bool {
        self.dev.is_some()
    }

    pub fn get_dev(&self) -> &DevManifestEntry {
        let dev = self
            .dev
            .as_ref()
            .expect("extension manifest does not contain a [dev] section");

        assert!(
            self.grammars.is_empty() || dev.grammars.is_empty(),
            "extension manifest cannot contain both grammars and dev.grammars"
        );

        dev
    }

    pub fn grammar_source(&self, grammar_name: &str) -> Option<GrammarSource<'_>> {
        if let Some(dev_grammar) = self
            .dev
            .as_ref()
            .and_then(|dev| dev.grammars.get(grammar_name))
        {
            return Some(GrammarSource::Dev(dev_grammar));
        }

        self.grammars
            .get(grammar_name)
            .map(GrammarSource::Published)
    }

    pub fn grammar_names(&self) -> Box<dyn Iterator<Item = &Arc<str>> + '_> {
        if self
            .dev
            .as_ref()
            .is_some_and(|dev| !dev.grammars.is_empty())
        {
            let dev = self.get_dev();
            return Box::new(dev.grammars.keys());
        }

        Box::new(self.grammars.keys())
    }

    /// Returns the set of features provided by the extension.
    pub fn provides(&self) -> BTreeSet<ExtensionProvides> {
        let mut provides = BTreeSet::default();
        if !self.themes.is_empty() {
            provides.insert(ExtensionProvides::Themes);
        }

        if !self.icon_themes.is_empty() {
            provides.insert(ExtensionProvides::IconThemes);
        }

        if !self.languages.is_empty() {
            provides.insert(ExtensionProvides::Languages);
        }

        if self.grammar_names().next().is_some() {
            provides.insert(ExtensionProvides::Grammars);
        }

        if !self.language_servers.is_empty() {
            provides.insert(ExtensionProvides::LanguageServers);
        }

        if !self.context_servers.is_empty() {
            provides.insert(ExtensionProvides::ContextServers);
        }

        if !self.agent_servers.is_empty() {
            provides.insert(ExtensionProvides::AgentServers);
        }

        if self.snippets.is_some() {
            provides.insert(ExtensionProvides::Snippets);
        }

        if !self.debug_adapters.is_empty() {
            provides.insert(ExtensionProvides::DebugAdapters);
        }

        provides
    }

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
) -> anyhow::Result<RelPathBuf> {
    match &meta.schema_path {
        Some(path) => Ok(path.clone()),
        None => Path::new("debug_adapter_schemas")
            .join(Path::new(adapter_name.as_ref()).with_extension("json"))
            .to_rel_path_buf(),
    }
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LibManifestEntry {
    pub kind: Option<ExtensionLibraryKind>,
    pub version: Option<Version>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct AgentServerManifestEntry {
    /// Display name for the agent (shown in menus).
    pub name: String,
    /// Environment variables to set when launching the agent server.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Optional icon path (relative to extension root, e.g., "ai.svg").
    /// Should be a small SVG icon for display in menus.
    #[serde(default)]
    pub icon: Option<String>,
    /// Per-target configuration for archive-based installation.
    /// The key format is "{os}-{arch}" where:
    /// - os: "darwin" (macOS), "linux", "windows"
    /// - arch: "aarch64" (arm64), "x86_64"
    ///
    /// Example:
    /// ```toml
    /// [agent_servers.myagent.targets.darwin-aarch64]
    /// archive = "https://example.com/myagent-darwin-arm64.zip"
    /// cmd = "./myagent"
    /// args = ["--serve"]
    /// sha256 = "abc123..."  # optional
    /// ```
    ///
    /// For Node.js-based agents, you can use "node" as the cmd to automatically
    /// use Zed's managed Node.js runtime instead of relying on the user's PATH:
    /// ```toml
    /// [agent_servers.nodeagent.targets.darwin-aarch64]
    /// archive = "https://example.com/nodeagent.zip"
    /// cmd = "node"
    /// args = ["index.js", "--port", "3000"]
    /// ```
    ///
    /// Note: All commands are executed with the archive extraction directory as the
    /// working directory, so relative paths in args (like "index.js") will resolve
    /// relative to the extracted archive contents.
    pub targets: HashMap<String, TargetConfig>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct TargetConfig {
    /// URL to download the archive from (e.g., "https://github.com/owner/repo/releases/download/v1.0.0/myagent-darwin-arm64.zip")
    pub archive: String,
    /// Command to run (e.g., "./myagent" or "./myagent.exe")
    pub cmd: String,
    /// Command-line arguments to pass to the agent server.
    #[serde(default)]
    pub args: Vec<String>,
    /// Optional SHA-256 hash of the archive for verification.
    /// If not provided and the URL is a GitHub release, we'll attempt to fetch it from GitHub.
    #[serde(default)]
    pub sha256: Option<String>,
    /// Environment variables to set when launching the agent server.
    /// These target-specific env vars will override any env vars set at the agent level.
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl TargetConfig {
    pub fn from_proto(proto: proto::ExternalExtensionAgentTarget) -> Self {
        Self {
            archive: proto.archive,
            cmd: proto.cmd,
            args: proto.args,
            sha256: proto.sha256,
            env: proto.env.into_iter().collect(),
        }
    }

    pub fn to_proto(&self) -> proto::ExternalExtensionAgentTarget {
        proto::ExternalExtensionAgentTarget {
            archive: self.archive.clone(),
            cmd: self.cmd.clone(),
            args: self.args.clone(),
            sha256: self.sha256.clone(),
            env: self
                .env
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }
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

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct DevGrammarManifestEntry {
    pub path: String,
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct DevManifestEntry {
    #[serde(default)]
    pub grammars: BTreeMap<Arc<str>, DevGrammarManifestEntry>,
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
    pub schema_path: Option<RelPathBuf>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct DebugLocatorManifestEntry {}

/// Manifest entry for a language model provider.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LanguageModelProviderManifestEntry {
    /// Display name for the provider.
    pub name: String,
    /// Path to an SVG icon file relative to the extension root (e.g., "icons/provider.svg").
    #[serde(default)]
    pub icon: Option<String>,
}

impl ExtensionManifest {
    pub async fn load(fs: Arc<dyn Fs>, extension_dir: &Path) -> Result<Self> {
        let extension_name = extension_dir
            .file_name()
            .and_then(OsStr::to_str)
            .context("invalid extension name")?;

        let extension_manifest_path = extension_dir.join("extension.toml");
        if fs.is_file(&extension_manifest_path).await {
            let manifest_content = fs.load(&extension_manifest_path).await.with_context(|| {
                format!("loading {extension_name} extension.toml, {extension_manifest_path:?}")
            })?;
            toml::from_str(&manifest_content).map_err(|err| {
                anyhow!("Invalid extension.toml for extension {extension_name}:\n{err}")
            })
        } else if let extension_manifest_path = extension_manifest_path.with_extension("json")
            && fs.is_file(&extension_manifest_path).await
        {
            let manifest_content = fs.load(&extension_manifest_path).await.with_context(|| {
                format!("loading {extension_name} extension.json, {extension_manifest_path:?}")
            })?;

            serde_json::from_str::<OldExtensionManifest>(&manifest_content)
                .with_context(|| format!("invalid extension.json for extension {extension_name}"))
                .map(|manifest_json| manifest_from_old_manifest(manifest_json, extension_name))
        } else {
            anyhow::bail!("No extension manifest found for extension {extension_name}")
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
        language_model_providers: Default::default(),
        dev: None,
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use util::rel_path::rel_path_buf;

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
            language_model_providers: BTreeMap::default(),
            dev: None,
        }
    }

    #[test]
    fn test_build_adapter_schema_path_with_schema_path() {
        let adapter_name = Arc::from("my_adapter");
        let entry = DebugAdapterManifestEntry {
            schema_path: Some(rel_path_buf("foo/bar")),
        };

        let path = build_debug_adapter_schema_path(&adapter_name, &entry).unwrap();
        assert_eq!(path, rel_path_buf("foo/bar"));
    }

    #[test]
    fn test_build_adapter_schema_path_without_schema_path() {
        let adapter_name = Arc::from("my_adapter");
        let entry = DebugAdapterManifestEntry { schema_path: None };

        let path = build_debug_adapter_schema_path(&adapter_name, &entry).unwrap();
        assert_eq!(path, rel_path_buf("debug_adapter_schemas/my_adapter.json"));
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
    #[cfg(target_os = "windows")]
    fn test_deserialize_manifest_with_windows_separators() {
        let content = indoc! {r#"
            id = "test-manifest"
            name = "Test Manifest"
            version = "0.0.1"
            schema_version = 0
            languages = ["foo\\bar"]
        "#};
        let manifest: ExtensionManifest = toml::from_str(&content).expect("manifest should parse");
        assert_eq!(manifest.languages, vec![rel_path_buf("foo/bar")]);
    }

    #[test]
    fn parse_manifest_with_agent_server_archive_launcher() {
        let toml_src = indoc! {r#"
            id = "example.agent-server-ext"
            name = "Agent Server Example"
            version = "1.0.0"
            schema_version = 0

            [agent_servers.foo]
            name = "Foo Agent"

            [agent_servers.foo.targets.linux-x86_64]
            archive = "https://example.com/agent-linux-x64.tar.gz"
            cmd = "./agent"
            args = ["--serve"]
        "#};

        let manifest: ExtensionManifest = toml::from_str(toml_src).expect("manifest should parse");
        assert_eq!(manifest.id.as_ref(), "example.agent-server-ext");
        assert!(manifest.agent_servers.contains_key("foo"));
        let entry = manifest.agent_servers.get("foo").unwrap();
        assert!(entry.targets.contains_key("linux-x86_64"));
        let target = entry.targets.get("linux-x86_64").unwrap();
        assert_eq!(target.archive, "https://example.com/agent-linux-x64.tar.gz");
        assert_eq!(target.cmd, "./agent");
        assert_eq!(target.args, vec!["--serve"]);
    }

    #[test]
    fn parse_manifest_with_dev_grammars() {
        let toml_src = indoc! {r#"
            id = "example.dev-extension"
            name = "Dev Extension Example"
            version = "1.0.0"
            schema_version = 0

            [dev.grammars.example]
            path = "../tree-sitter-example"
        "#};

        let manifest: ExtensionManifest = toml::from_str(toml_src).expect("manifest should parse");
        assert_eq!(manifest.id.as_ref(), "example.dev-extension");
        assert!(manifest.grammars.is_empty());

        let dev = manifest
            .dev
            .as_ref()
            .expect("dev section should be present");
        assert!(dev.grammars.contains_key("example"));
        let entry = dev
            .grammars
            .get("example")
            .expect("grammar entry should exist");
        assert_eq!(entry.path, "../tree-sitter-example");

        let grammar_names: Vec<&str> = manifest.grammar_names().map(|n| n.as_ref()).collect();
        assert_eq!(grammar_names, vec!["example"]);
    }

    #[test]
    fn parse_manifest_rejects_both_grammars_and_dev_grammars() {
        let toml_src = indoc! {r#"
            id = "example.invalid"
            name = "Invalid Extension"
            version = "1.0.0"
            schema_version = 0

            [grammars.html]
            repository = "https://github.com/tree-sitter/tree-sitter-html"
            commit = "abc123"

            [dev.grammars.html]
            path = "../tree-sitter-html"
        "#};

        let result: std::result::Result<ExtensionManifest, _> = toml::from_str(toml_src);
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("cannot specify both"),
            "unexpected error message: {error_message}"
        );
    }

    #[test]
    fn parse_manifest_with_standard_grammars() {
        let toml_src = indoc! {r#"
            id = "example.published"
            name = "Published Extension"
            version = "1.0.0"
            schema_version = 0

            [grammars.html]
            repository = "https://github.com/tree-sitter/tree-sitter-html"
            commit = "abc123"
        "#};

        let manifest: ExtensionManifest = toml::from_str(toml_src).expect("manifest should parse");
        assert!(manifest.dev.is_none());
        assert!(manifest.grammars.contains_key("html"));

        let entry = manifest
            .grammars
            .get("html")
            .expect("grammar entry should exist");
        assert_eq!(
            entry.repository,
            "https://github.com/tree-sitter/tree-sitter-html"
        );
        assert_eq!(entry.rev, "abc123");

        let grammar_names: Vec<&str> = manifest.grammar_names().map(|n| n.as_ref()).collect();
        assert_eq!(grammar_names, vec!["html"]);
    }

    #[test]
    fn get_dev_returns_dev_section_for_dev_manifest() {
        let manifest: ExtensionManifest = toml::from_str(indoc! {r#"
            id = "example.dev-extension"
            name = "Dev Extension Example"
            version = "1.0.0"
            schema_version = 0

            [dev.grammars.example]
            path = "../tree-sitter-example"
        "#})
        .expect("manifest should parse");

        assert!(manifest.is_dev());
        let dev = manifest.get_dev();
        assert!(dev.grammars.contains_key("example"));
    }

    #[test]
    #[should_panic(expected = "extension manifest does not contain a [dev] section")]
    fn get_dev_panics_without_dev_section() {
        let manifest = extension_manifest();

        let _ = manifest.get_dev();
    }

    #[test]
    fn grammar_names_fall_back_to_published_grammars_when_dev_grammars_are_empty() {
        let manifest: ExtensionManifest = toml::from_str(indoc! {r#"
            id = "example.fallback"
            name = "Fallback Example"
            version = "1.0.0"
            schema_version = 0

            [grammars.html]
            repository = "https://github.com/tree-sitter/tree-sitter-html"
            commit = "abc123"

            [dev]
        "#})
        .expect("manifest should parse");

        let grammar_names: Vec<&str> = manifest.grammar_names().map(|name| name.as_ref()).collect();
        assert_eq!(grammar_names, vec!["html"]);
    }

    #[test]
    fn grammar_source_falls_back_to_published_grammar_when_dev_grammars_are_empty() {
        let manifest: ExtensionManifest = toml::from_str(indoc! {r#"
            id = "example.fallback"
            name = "Fallback Example"
            version = "1.0.0"
            schema_version = 0

            [grammars.html]
            repository = "https://github.com/tree-sitter/tree-sitter-html"
            commit = "abc123"
            path = "vendor/html"

            [dev]
        "#})
        .expect("manifest should parse");

        match manifest.grammar_source("html") {
            Some(GrammarSource::Published(grammar)) => {
                assert_eq!(grammar.path.as_deref(), Some("vendor/html"));
            }
            Some(GrammarSource::Dev(_)) => panic!("expected published grammar source"),
            None => panic!("expected grammar source"),
        }
    }

    #[test]
    fn parse_manifest_rejects_empty_dev_section_without_any_grammars() {
        let toml_src = indoc! {r#"
            id = "example.invalid-dev"
            name = "Invalid Dev Extension"
            version = "1.0.0"
            schema_version = 0

            [dev]
        "#};

        let result: std::result::Result<ExtensionManifest, _> = toml::from_str(toml_src);
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("must specify at least one grammar"),
            "unexpected error message: {error_message}"
        );
    }

    #[test]
    fn grammar_names_empty_when_no_grammars() {
        let manifest = extension_manifest();
        assert!(manifest.grammar_names().next().is_none());
    }
}
