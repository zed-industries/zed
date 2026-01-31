use std::{path::PathBuf, sync::Arc};

use collections::{BTreeMap, HashMap};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_json::parse_json_with_comments;
use settings_macros::{MergeFrom, with_fallible_options};
use util::serde::default_true;

use crate::{
    AllLanguageSettingsContent, DelayMs, ExtendingVec, ParseStatus, ProjectTerminalSettingsContent,
    RootUserSettings, SlashCommandSettings, fallible_options,
};

#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LspSettingsMap(pub HashMap<Arc<str>, LspSettings>);

impl IntoIterator for LspSettingsMap {
    type Item = (Arc<str>, LspSettings);
    type IntoIter = std::collections::hash_map::IntoIter<Arc<str>, LspSettings>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl RootUserSettings for ProjectSettingsContent {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus) {
        fallible_options::parse_json(json)
    }
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self> {
        parse_json_with_comments(json)
    }
}

#[with_fallible_options]
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ProjectSettingsContent {
    #[serde(flatten)]
    pub all_languages: AllLanguageSettingsContent,

    #[serde(flatten)]
    pub worktree: WorktreeSettingsContent,

    /// Configuration for language servers.
    ///
    /// The following settings can be overridden for specific language servers:
    /// - initialization_options
    ///
    /// To override settings for a language, add an entry for that language server's
    /// name to the lsp value.
    /// Default: null
    #[serde(default)]
    pub lsp: LspSettingsMap,

    pub terminal: Option<ProjectTerminalSettingsContent>,

    /// Configuration for Debugger-related features
    #[serde(default)]
    pub dap: HashMap<Arc<str>, DapSettingsContent>,

    /// Settings for context servers used for AI-related features.
    #[serde(default)]
    pub context_servers: HashMap<Arc<str>, ContextServerSettingsContent>,

    /// Default timeout in seconds for context server tool calls.
    /// Can be overridden per-server in context_servers configuration.
    ///
    /// Default: 60
    pub context_server_timeout: Option<u64>,

    /// Configuration for how direnv configuration should be loaded
    pub load_direnv: Option<DirenvSettings>,

    /// Settings for slash commands.
    pub slash_commands: Option<SlashCommandSettings>,

    /// The list of custom Git hosting providers.
    pub git_hosting_providers: Option<ExtendingVec<GitHostingProviderConfig>>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WorktreeSettingsContent {
    /// The displayed name of this project. If not set or null, the root directory name
    /// will be displayed.
    ///
    /// Default: null
    pub project_name: Option<String>,

    /// Whether to prevent this project from being shared in public channels.
    ///
    /// Default: false
    #[serde(default)]
    pub prevent_sharing_in_public_channels: bool,

    /// Completely ignore files matching globs from `file_scan_exclusions`. Overrides
    /// `file_scan_inclusions`.
    ///
    /// Default: [
    ///   "**/.git",
    ///   "**/.svn",
    ///   "**/.hg",
    ///   "**/.jj",
    ///   "**/CVS",
    ///   "**/.DS_Store",
    ///   "**/Thumbs.db",
    ///   "**/.classpath",
    ///   "**/.settings"
    /// ]
    pub file_scan_exclusions: Option<Vec<String>>,

    /// Always include files that match these globs when scanning for files, even if they're
    /// ignored by git. This setting is overridden by `file_scan_exclusions`.
    /// Default: [
    ///  ".env*",
    ///  "docker-compose.*.yml",
    /// ]
    pub file_scan_inclusions: Option<Vec<String>>,

    /// Treat the files matching these globs as `.env` files.
    /// Default: ["**/.env*", "**/*.pem", "**/*.key", "**/*.cert", "**/*.crt", "**/secrets.yml"]
    pub private_files: Option<ExtendingVec<String>>,

    /// Treat the files matching these globs as hidden files. You can hide hidden files in the project panel.
    /// Default: ["**/.*"]
    pub hidden_files: Option<Vec<String>>,

    /// Treat the files matching these globs as read-only. These files can be opened and viewed,
    /// but cannot be edited. This is useful for generated files, build outputs, or files from
    /// external dependencies that should not be modified directly.
    /// Default: []
    pub read_only_files: Option<Vec<String>>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    pub binary: Option<BinarySettings>,
    /// Options passed to the language server at startup.
    ///
    /// Ref: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize
    ///
    /// Consult the documentation for the specific language server to see which settings are supported.
    pub initialization_options: Option<serde_json::Value>,
    /// Language server settings.
    ///
    /// Ref: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_configuration
    ///
    /// Consult the documentation for the specific language server to see which settings are supported.
    pub settings: Option<serde_json::Value>,
    /// If the server supports sending tasks over LSP extensions,
    /// this setting can be used to enable or disable them in Zed.
    /// Default: true
    #[serde(default = "default_true")]
    pub enable_lsp_tasks: bool,
    pub fetch: Option<FetchSettings>,
}

impl Default for LspSettings {
    fn default() -> Self {
        Self {
            binary: None,
            initialization_options: None,
            settings: None,
            enable_lsp_tasks: true,
            fetch: None,
        }
    }
}

#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
pub struct BinarySettings {
    pub path: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub env: Option<BTreeMap<String, String>>,
    pub ignore_system_version: Option<bool>,
}

#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
pub struct FetchSettings {
    // Whether to consider pre-releases for fetching
    pub pre_release: Option<bool>,
}

/// Common language server settings.
#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct GlobalLspSettingsContent {
    /// Whether to show the LSP servers button in the status bar.
    ///
    /// Default: `true`
    pub button: Option<bool>,
    /// Settings for language server notifications
    pub notifications: Option<LspNotificationSettingsContent>,
}

#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LspNotificationSettingsContent {
    /// Timeout in milliseconds for automatically dismissing language server notifications.
    /// Set to 0 to disable auto-dismiss.
    ///
    /// Default: 5000
    pub dismiss_timeout_ms: Option<u64>,
}

#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct DapSettingsContent {
    pub binary: Option<String>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

#[with_fallible_options]
#[derive(
    Default, Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema, MergeFrom,
)]
pub struct SessionSettingsContent {
    /// Whether or not to restore unsaved buffers on restart.
    ///
    /// If this is true, user won't be prompted whether to save/discard
    /// dirty files when closing the application.
    ///
    /// Default: true
    pub restore_unsaved_buffers: Option<bool>,
    /// Whether or not to skip worktree trust checks.
    /// When trusted, project settings are synchronized automatically,
    /// language and MCP servers are downloaded and started automatically.
    ///
    /// Default: false
    pub trust_all_worktrees: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom, Debug)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ContextServerSettingsContent {
    Stdio {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// Whether to run the context server on the remote server when using remote development.
        ///
        /// If this is false, the context server will always run on the local machine.
        ///
        /// Default: false
        #[serde(default)]
        remote: bool,
        #[serde(flatten)]
        command: ContextServerCommand,
    },
    Http {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// The URL of the remote context server.
        url: String,
        /// Optional headers to send.
        #[serde(skip_serializing_if = "HashMap::is_empty", default)]
        headers: HashMap<String, String>,
        /// Timeout for tool calls in seconds. Defaults to global context_server_timeout if not specified.
        timeout: Option<u64>,
    },
    Extension {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// Whether to run the context server on the remote server when using remote development.
        ///
        /// If this is false, the context server will always run on the local machine.
        ///
        /// Default: false
        #[serde(default)]
        remote: bool,
        /// The settings for this context server specified by the extension.
        ///
        /// Consult the documentation for the context server to see what settings
        /// are supported.
        settings: serde_json::Value,
    },
}

impl ContextServerSettingsContent {
    pub fn set_enabled(&mut self, enabled: bool) {
        match self {
            ContextServerSettingsContent::Stdio {
                enabled: custom_enabled,
                ..
            } => {
                *custom_enabled = enabled;
            }
            ContextServerSettingsContent::Extension {
                enabled: ext_enabled,
                ..
            } => *ext_enabled = enabled,
            ContextServerSettingsContent::Http {
                enabled: remote_enabled,
                ..
            } => *remote_enabled = enabled,
        }
    }
}

#[with_fallible_options]
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom)]
pub struct ContextServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    /// Timeout for tool calls in seconds. Defaults to 60 if not specified.
    pub timeout: Option<u64>,
}

impl std::fmt::Debug for ContextServerCommand {
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

        f.debug_struct("ContextServerCommand")
            .field("path", &self.path)
            .field("args", &self.args)
            .field("env", &filtered_env)
            .finish()
    }
}

#[with_fallible_options]
#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct GitSettings {
    /// Whether or not to enable git integration.
    ///
    /// Default: true
    #[serde(flatten)]
    pub enabled: Option<GitEnabledSettings>,
    /// Whether or not to show the git gutter.
    ///
    /// Default: tracked_files
    pub git_gutter: Option<GitGutterSetting>,
    /// Sets the debounce threshold (in milliseconds) after which changes are reflected in the git gutter.
    ///
    /// Default: 0
    pub gutter_debounce: Option<u64>,
    /// Whether or not to show git blame data inline in
    /// the currently focused line.
    ///
    /// Default: on
    pub inline_blame: Option<InlineBlameSettings>,
    /// Git blame settings.
    pub blame: Option<BlameSettings>,
    /// Which information to show in the branch picker.
    ///
    /// Default: on
    pub branch_picker: Option<BranchPickerSettingsContent>,
    /// How hunks are displayed visually in the editor.
    ///
    /// Default: staged_hollow
    pub hunk_style: Option<GitHunkStyleSetting>,
    /// How file paths are displayed in the git gutter.
    ///
    /// Default: file_name_first
    pub path_style: Option<GitPathStyle>,
}

#[with_fallible_options]
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct GitEnabledSettings {
    pub disable_git: Option<bool>,
    pub enable_status: Option<bool>,
    pub enable_diff: Option<bool>,
}

impl GitEnabledSettings {
    pub fn is_git_status_enabled(&self) -> bool {
        !self.disable_git.unwrap_or(false) && self.enable_status.unwrap_or(true)
    }

    pub fn is_git_diff_enabled(&self) -> bool {
        !self.disable_git.unwrap_or(false) && self.enable_diff.unwrap_or(true)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum GitGutterSetting {
    /// Show git gutter in tracked files.
    #[default]
    TrackedFiles,
    /// Hide git gutter
    Hide,
}

#[with_fallible_options]
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct InlineBlameSettings {
    /// Whether or not to show git blame data inline in
    /// the currently focused line.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// Whether to only show the inline blame information
    /// after a delay once the cursor stops moving.
    ///
    /// Default: 0
    pub delay_ms: Option<DelayMs>,
    /// The amount of padding between the end of the source line and the start
    /// of the inline blame in units of columns.
    ///
    /// Default: 7
    pub padding: Option<u32>,
    /// The minimum column number to show the inline blame information at
    ///
    /// Default: 0
    pub min_column: Option<u32>,
    /// Whether to show commit summary as part of the inline blame.
    ///
    /// Default: false
    pub show_commit_summary: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct BlameSettings {
    /// Whether to show the avatar of the author of the commit.
    ///
    /// Default: true
    pub show_avatar: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct BranchPickerSettingsContent {
    /// Whether to show author name as part of the commit information.
    ///
    /// Default: false
    pub show_author_name: Option<bool>,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Debug,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum GitHunkStyleSetting {
    /// Show unstaged hunks with a filled background and staged hunks hollow.
    #[default]
    StagedHollow,
    /// Show unstaged hunks hollow and staged hunks with a filled background.
    UnstagedHollow,
}

#[with_fallible_options]
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum GitPathStyle {
    /// Show file name first, then path
    #[default]
    FileNameFirst,
    /// Show full path first
    FilePathFirst,
}

#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct DiagnosticsSettingsContent {
    /// Whether to show the project diagnostics button in the status bar.
    pub button: Option<bool>,

    /// Whether or not to include warning diagnostics.
    ///
    /// Default: true
    pub include_warnings: Option<bool>,

    /// Settings for using LSP pull diagnostics mechanism in Zed.
    pub lsp_pull_diagnostics: Option<LspPullDiagnosticsSettingsContent>,

    /// Settings for showing inline diagnostics.
    pub inline: Option<InlineDiagnosticsSettingsContent>,
}

#[with_fallible_options]
#[derive(
    Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq,
)]
pub struct LspPullDiagnosticsSettingsContent {
    /// Whether to pull for diagnostics or not.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// Minimum time to wait before pulling diagnostics from the language server(s).
    /// 0 turns the debounce off.
    ///
    /// Default: 50
    pub debounce_ms: Option<DelayMs>,
}

#[with_fallible_options]
#[derive(
    Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Eq,
)]
pub struct InlineDiagnosticsSettingsContent {
    /// Whether or not to show inline diagnostics
    ///
    /// Default: false
    pub enabled: Option<bool>,
    /// Whether to only show the inline diagnostics after a delay after the
    /// last editor event.
    ///
    /// Default: 150
    pub update_debounce_ms: Option<DelayMs>,
    /// The amount of padding between the end of the source line and the start
    /// of the inline diagnostic in units of columns.
    ///
    /// Default: 4
    pub padding: Option<u32>,
    /// The minimum column to display inline diagnostics. This setting can be
    /// used to horizontally align inline diagnostics at some position. Lines
    /// longer than this value will still push diagnostics further to the right.
    ///
    /// Default: 0
    pub min_column: Option<u32>,

    pub max_severity: Option<DiagnosticSeverityContent>,
}

#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct NodeBinarySettings {
    /// The path to the Node binary.
    pub path: Option<String>,
    /// The path to the npm binary Zed should use (defaults to `.path/../npm`).
    pub npm_path: Option<String>,
    /// If enabled, Zed will download its own copy of Node.
    pub ignore_system_version: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum DirenvSettings {
    /// Load direnv configuration through a shell hook
    ShellHook,
    /// Load direnv configuration directly using `direnv export json`
    #[default]
    Direct,
    /// Do not load direnv configuration
    Disabled,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverityContent {
    // No diagnostics are shown.
    Off,
    Error,
    Warning,
    Info,
    Hint,
    All,
}

/// A custom Git hosting provider.
#[with_fallible_options]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct GitHostingProviderConfig {
    /// The type of the provider.
    ///
    /// Must be one of `github`, `gitlab`, `bitbucket`, `gitea`, `forgejo`, or `source_hut`.
    pub provider: GitHostingProviderKind,

    /// The base URL for the provider (e.g., "https://code.corp.big.com").
    pub base_url: String,

    /// The display name for the provider (e.g., "BigCorp GitHub").
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum GitHostingProviderKind {
    Github,
    Gitlab,
    Bitbucket,
    Gitea,
    Forgejo,
    SourceHut,
}
