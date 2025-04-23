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
    pub slash_commands: BTreeMap<Arc<str>, SlashCommandManifestEntry>,
    #[serde(default)]
    pub indexed_docs_providers: BTreeMap<Arc<str>, IndexedDocsProviderEntry>,
    #[serde(default)]
    pub snippets: Option<PathBuf>,
    #[serde(default)]
    pub capabilities: Vec<ExtensionCapability>,
}

impl ExtensionManifest {
    pub fn allow_exec(
        &self,
        desired_command: &str,
        desired_args: &[impl AsRef<str> + std::fmt::Debug],
    ) -> Result<()> {
        let is_allowed = self.capabilities.iter().any(|capability| match capability {
            ExtensionCapability::ProcessExec { command, args } if command == desired_command => {
                for (ix, arg) in args.iter().enumerate() {
                    if arg == "**" {
                        return true;
                    }

                    if ix >= desired_args.len() {
                        return false;
                    }

                    if arg != "*" && arg != desired_args[ix].as_ref() {
                        return false;
                    }
                }
                if args.len() < desired_args.len() {
                    return false;
                }
                true
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
}

/// A capability for an extension.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ExtensionCapability {
    #[serde(rename = "process:exec")]
    ProcessExec {
        /// The command to execute.
        command: String,
        /// The arguments to pass to the command. Use `*` for a single wildcard argument.
        /// If the last element is `**`, then any trailing arguments are allowed.
        args: Vec<String>,
    },
}

#[derive(Clone, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct LibManifestEntry {
    pub kind: Option<ExtensionLibraryKind>,
    pub version: Option<SemanticVersion>,
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
pub struct LanguageServerManifestEntry {
    /// Deprecated in favor of `languages`.
    #[serde(default)]
    language: Option<LanguageName>,
    /// The list of languages this language server should work with.
    #[serde(default)]
    languages: Vec<LanguageName>,
    #[serde(default)]
    pub language_ids: HashMap<String, String>,
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
pub struct ContextServerManifestEntry {
    #[serde(default)]
    pub installation_instructions: String,
    #[serde(default)]
    pub settings_hint: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct SlashCommandManifestEntry {
    pub description: String,
    pub requires_argument: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct IndexedDocsProviderEntry {}

impl ExtensionManifest {
    pub async fn load(fs: Arc<dyn Fs>, extension_dir: &Path) -> Result<Self> {
        let extension_name = extension_dir
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow!("invalid extension name"))?;

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
            toml::from_str(&manifest_content)
                .with_context(|| format!("invalid extension.toml for extension {extension_name}"))
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
        slash_commands: BTreeMap::default(),
        indexed_docs_providers: BTreeMap::default(),
        snippets: None,
        capabilities: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
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
            slash_commands: BTreeMap::default(),
            indexed_docs_providers: BTreeMap::default(),
            snippets: None,
            capabilities: vec![],
        }
    }

    #[test]
    fn test_allow_exact_match() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec {
                command: "ls".to_string(),
                args: vec!["-la".to_string()],
            }],
            ..extension_manifest()
        };

        assert!(manifest.allow_exec("ls", &["-la"]).is_ok());
        assert!(manifest.allow_exec("ls", &["-l"]).is_err());
        assert!(manifest.allow_exec("pwd", &[] as &[&str]).is_err());
    }

    #[test]
    fn test_allow_wildcard_arg() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec {
                command: "git".to_string(),
                args: vec!["*".to_string()],
            }],
            ..extension_manifest()
        };

        assert!(manifest.allow_exec("git", &["status"]).is_ok());
        assert!(manifest.allow_exec("git", &["commit"]).is_ok());
        assert!(manifest.allow_exec("git", &["status", "-s"]).is_err()); // too many args
        assert!(manifest.allow_exec("npm", &["install"]).is_err()); // wrong command
    }

    #[test]
    fn test_allow_double_wildcard() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec {
                command: "cargo".to_string(),
                args: vec!["test".to_string(), "**".to_string()],
            }],
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
    fn test_allow_mixed_wildcards() {
        let manifest = ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec {
                command: "docker".to_string(),
                args: vec!["run".to_string(), "*".to_string(), "**".to_string()],
            }],
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
}
