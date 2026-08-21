use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use collections::{BTreeMap, HashMap};
use gpui::Rgba;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_json::parse_json_with_comments;
use settings_macros::{MergeFrom, with_fallible_options};
use util::serde::default_true;

use crate::{
    AllLanguageSettingsContent, DelayMs, ExtendingVec, ParseStatus, ProjectTerminalSettingsContent,
    RootUserSettings, SaturatingBool, SplicingVec, fallible_options,
};

/// A map from language server name to its settings.
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

/// Settings that can be configured per project.
#[with_fallible_options]
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ProjectSettingsContent {
    /// Settings for languages, applied to all languages or per-language.
    #[serde(flatten)]
    pub all_languages: AllLanguageSettingsContent,

    /// Settings that control how Zed scans and shares worktrees.
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

    /// Settings specific to the terminal.
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

    /// The list of custom Git hosting providers.
    pub git_hosting_providers: Option<ExtendingVec<GitHostingProviderConfig>>,

    /// Whether to disable all AI features in Zed.
    ///
    /// Default: false
    pub disable_ai: Option<SaturatingBool>,
}

/// When to scan content of linked directories.
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum ScanSymlinksSetting {
    /// Always scan symlinked directories
    Always,
    /// Only scan symlinked directories when they've been expanded in the workspace
    #[default]
    Expanded,
}

/// Settings that control how Zed scans and shares worktrees.
#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WorktreeSettingsContent {
    /// Whether to prevent this project from being shared in public channels.
    ///
    /// Default: false
    #[serde(default)]
    pub prevent_sharing_in_public_channels: bool,

    /// Completely ignore files matching globs from `file_scan_exclusions`. Overrides
    /// `file_scan_inclusions`.
    ///
    /// A `"..."` entry expands to the value being overridden, so
    /// `["**/node_modules", "..."]` adds to the inherited globs instead of
    /// replacing them. Leave `"..."` out to replace them.
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
    pub file_scan_exclusions: Option<SplicingVec>,

    /// Always include files that match these globs when scanning for files, even if they're
    /// ignored by git. This setting is overridden by `file_scan_exclusions`.
    /// Default: [
    ///  ".env*",
    ///  "docker-compose.*.yml",
    /// ]
    pub file_scan_inclusions: Option<Vec<String>>,

    /// When to scan content of linked directories.
    ///
    /// Default: expanded
    pub scan_symlinks: Option<ScanSymlinksSetting>,

    /// Maximum directory depth to eagerly index outside of git repositories;
    /// contents of directories at this depth or deeper are indexed on demand.
    /// Repositories rooted shallower than this depth are always indexed fully.
    /// In projects that are not rooted at a git repository, repositories directly
    /// inside a root folder activate their git features immediately; deeper ones
    /// activate on first use.
    /// `0` means no limit and activates all git repositories immediately.
    ///
    /// Default: 5
    pub file_scan_depth: Option<u32>,

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

/// Settings for a specific language server.
#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    /// Settings for the language server binary.
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
    /// Settings for fetching the language server binary.
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
/// Settings for a language server binary.
pub struct BinarySettings {
    /// The path to the binary.
    pub path: Option<String>,
    /// The arguments to pass to the binary.
    pub arguments: Option<Vec<String>>,
    /// The environment variables to set when launching the binary.
    pub env: Option<BTreeMap<String, String>>,
    /// Whether to fetch the binary from the internet, or attempt to find locally.
    pub ignore_system_version: Option<bool>,
}

#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
/// Settings for fetching a language server binary.
pub struct FetchSettings {
    /// Whether to consider pre-releases for fetching
    pub pre_release: Option<bool>,
}

/// Common language server settings.
#[with_fallible_options]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct GlobalLspSettingsContent {
    /// Whether to show the LSP servers button in the status bar.
    ///
    /// Default: `true`
    pub button: Option<bool>,
    /// The maximum amount of time to wait for responses from language servers, in seconds.
    /// A value of `0` will result in no timeout being applied (causing all LSP responses to wait indefinitely until completed).
    ///
    /// Default: `120`
    pub request_timeout: Option<u64>,
    /// The maximum line length a buffer may contain before language server features are disabled for the entire buffer.
    ///
    /// Default: `20000`
    #[schemars(range(min = 1))]
    pub max_buffer_line_length: Option<u32>,
    /// Settings for language server notifications
    pub notifications: Option<LspNotificationSettingsContent>,
    /// Rules for rendering LSP semantic tokens.
    pub semantic_token_rules: Option<SemanticTokenRules>,
}

/// Settings for language server notifications.
#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LspNotificationSettingsContent {
    /// Timeout in milliseconds for automatically dismissing language server notifications.
    /// Set to 0 to disable auto-dismiss.
    ///
    /// Default: 5000
    pub dismiss_timeout_ms: Option<u64>,
}

/// Custom rules for rendering LSP semantic tokens.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(transparent)]
pub struct SemanticTokenRules {
    /// The list of semantic token rules.
    /// The first matching rule for a token is applied.
    pub rules: Vec<SemanticTokenRule>,
}

impl SemanticTokenRules {
    /// The name of the file that semantic token rules are loaded from.
    pub const FILE_NAME: &'static str = "semantic_token_rules.json";

    /// Loads semantic token rules from the given file path.
    pub fn load(file_path: &Path) -> anyhow::Result<Self> {
        let rules_content = std::fs::read(file_path).with_context(|| {
            anyhow::anyhow!(
                "Could not read semantic token rules from {}",
                file_path.display()
            )
        })?;

        serde_json_lenient::from_slice::<SemanticTokenRules>(&rules_content).with_context(|| {
            anyhow::anyhow!(
                "Failed to parse semantic token rules from {}",
                file_path.display()
            )
        })
    }
}

impl crate::merge_from::MergeFrom for SemanticTokenRules {
    fn merge_from(&mut self, other: &Self) {
        self.rules.splice(0..0, other.rules.iter().cloned());
    }
}

/// A rule for highlighting semantic tokens.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SemanticTokenRule {
    /// The LSP semantic token type to customize. If omitted, the rule matches all token types.
    pub token_type: Option<String>,
    /// A list of LSP semantic token modifiers to match. All modifiers must be present
    /// to match.
    #[serde(default)]
    pub token_modifiers: Vec<String>,
    /// A list of styles from the current syntax theme to use. The first style found is used.
    /// The other style settings in this rule override that style.
    #[serde(default)]
    pub style: Vec<String>,
    /// The foreground color to use for the token type, in hex format (e.g., "#ff0000").
    pub foreground_color: Option<Rgba>,
    /// The background color to use for the token type, in hex format.
    pub background_color: Option<Rgba>,
    /// A boolean or color to underline with, in hex format. If `true`, then the token will be underlined with the text color.
    pub underline: Option<SemanticTokenColorOverride>,
    /// A boolean or color to strikethrough with, in hex format. If `true`, then the token have a strikethrough with the text color.
    pub strikethrough: Option<SemanticTokenColorOverride>,
    /// One of "normal", "bold".
    pub font_weight: Option<SemanticTokenFontWeight>,
    /// One of "normal", "italic".
    pub font_style: Option<SemanticTokenFontStyle>,
}

impl SemanticTokenRule {
    /// Returns whether the rule defines no styling.
    pub fn no_style_defined(&self) -> bool {
        self.style.is_empty()
            && self.foreground_color.is_none()
            && self.background_color.is_none()
            && self.underline.is_none()
            && self.strikethrough.is_none()
            && self.font_weight.is_none()
            && self.font_style.is_none()
    }
}

/// A color override for a semantic token decoration, such as underline or strikethrough.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
#[serde(untagged)]
pub enum SemanticTokenColorOverride {
    /// Enable or disable the decoration, using the text color when enabled.
    InheritForeground(bool),
    /// Use the given color for the decoration.
    Replace(Rgba),
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
/// The font weight to render a semantic token with.
pub enum SemanticTokenFontWeight {
    /// Normal font weight.
    #[default]
    Normal,
    /// Bold font weight.
    Bold,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
/// The font style to render a semantic token with.
pub enum SemanticTokenFontStyle {
    /// Normal font style.
    #[default]
    Normal,
    /// Italic font style.
    Italic,
}

/// Settings for a specific debug adapter.
#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub struct DapSettingsContent {
    /// The path to the debug adapter binary.
    pub binary: Option<String>,
    /// The arguments to pass to the debug adapter binary.
    pub args: Option<Vec<String>>,
    /// The environment variables to set when launching the debug adapter.
    pub env: Option<HashMap<String, String>>,
}

#[with_fallible_options]
#[derive(
    Default, Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema, MergeFrom,
)]
/// Configuration for session-related features.
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

/// Settings for a context server used for AI-related features.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom, Debug)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ContextServerSettingsContent {
    /// A context server that is launched with a custom command and communicates over stdio.
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
        /// The command used to launch the context server.
        #[serde(flatten)]
        command: ContextServerCommand,
    },
    /// A context server that is reachable over HTTP.
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
        /// Pre-registered OAuth client credentials for authorization servers that
        /// require out-of-band client registration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        oauth: Option<OAuthClientSettings>,
    },
    /// A context server that is provided by an extension.
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
    /// Sets whether the context server is enabled.
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

/// Pre-registered OAuth client credentials for MCP servers that don't support
/// Dynamic Client Registration.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom, Debug)]
pub struct OAuthClientSettings {
    /// The OAuth client ID obtained from out-of-band registration with the
    /// authorization server.
    pub client_id: String,
    /// The OAuth client secret, if this is a confidential client. For security,
    /// prefer providing this interactively; we will prompt and store it in
    /// the system keychain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

/// The command used to launch a stdio context server.
#[with_fallible_options]
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom)]
pub struct ContextServerCommand {
    /// The path of the executable to run.
    #[serde(rename = "command")]
    pub path: PathBuf,
    /// The arguments to pass to the executable.
    #[serde(default)]
    pub args: Vec<String>,
    /// The environment variables to set when launching the executable.
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

/// Configuration for git-related features.
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
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
    /// File diff settings.
    pub file_diff: Option<FileDiffSettingsContent>,
    /// How hunks are displayed visually in the editor.
    ///
    /// Default: staged_hollow
    pub hunk_style: Option<GitHunkStyleSetting>,
    /// Which base git features (gutter, file colors, git::Diff) diff against.
    ///
    /// Default: head
    pub diff_base: Option<GitDiffBaseSetting>,
    /// How file paths are displayed in the git gutter.
    ///
    /// Default: file_name_first
    pub path_style: Option<GitPathStyle>,
    /// Whether to show the stage and restore buttons on diff hunks.
    ///
    /// Default: true
    pub show_stage_restore_buttons: Option<bool>,
    /// Directory where git worktrees are created, relative to the repository
    /// working directory.
    ///
    /// When the resolved directory is outside the project root, the
    /// project's directory name is automatically appended so that
    /// sibling repos don't collide. For example, with the default
    /// `"../worktrees"` and a project at `~/code/zed`, worktrees are
    /// created under `~/code/worktrees/zed/`.
    ///
    /// When the resolved directory is inside the project root, no
    /// extra component is added (it's already project-scoped).
    ///
    /// Examples:
    /// - `"../worktrees"` — `~/code/worktrees/<project>/` (default)
    /// - `".git/zed-worktrees"` — `<project>/.git/zed-worktrees/`
    /// - `"my-worktrees"` — `<project>/my-worktrees/`
    ///
    /// Trailing slashes are ignored.
    ///
    /// Default: ../worktrees
    pub worktree_directory: Option<String>,
}

#[with_fallible_options]
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
/// Whether or not to enable git integration features.
pub struct GitEnabledSettings {
    /// If set to true, disables all git integration features.
    /// If set to false, individual git integration features below will be independently enabled or disabled.
    ///
    /// Default: false
    pub disable_git: Option<bool>,
    /// Whether to enable git status tracking.
    ///
    /// Default: true
    pub enable_status: Option<bool>,
    /// Whether to enable git diff display.
    ///
    /// Default: true
    pub enable_diff: Option<bool>,
}

impl GitEnabledSettings {
    /// Returns whether git status tracking is enabled.
    pub fn is_git_status_enabled(&self) -> bool {
        !self.disable_git.unwrap_or(false) && self.enable_status.unwrap_or(true)
    }

    /// Returns whether git diff display is enabled.
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
/// Whether or not to show the git gutter.
pub enum GitGutterSetting {
    /// Show git gutter in tracked files.
    #[default]
    TrackedFiles,
    /// Hide git gutter
    Hide,
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
/// Where to render inline git blame information.
pub enum InlineBlameLocation {
    /// Show git blame inline at the current line.
    #[default]
    Inline,
    /// Show git blame in the status bar at the bottom of the window.
    StatusBar,
}

#[with_fallible_options]
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
/// Settings for inline git blame.
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
    /// Where to render the blame information when enabled.
    ///
    /// Default: inline
    pub location: Option<InlineBlameLocation>,
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
/// Git blame settings.
pub struct BlameSettings {
    /// Whether to show the avatar of the author of the commit.
    ///
    /// Default: true
    pub show_avatar: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
/// Settings for the branch picker.
pub struct BranchPickerSettingsContent {
    /// Whether to show author name as part of the commit information.
    ///
    /// Default: false
    pub show_author_name: Option<bool>,
}

#[with_fallible_options]
#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
/// File diff settings.
pub struct FileDiffSettingsContent {
    /// Whether newly opened file diffs show the full file instead of changes only.
    ///
    /// Default: true
    pub show_full_file: Option<bool>,
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
/// How hunks are displayed visually in the editor.
pub enum GitHunkStyleSetting {
    /// Show unstaged hunks with a filled background and staged hunks hollow.
    #[default]
    StagedHollow,
    /// Show unstaged hunks hollow and staged hunks with a filled background.
    UnstagedHollow,
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
/// Which base git features (gutter, file colors, git::Diff) diff against.
pub enum GitDiffBaseSetting {
    /// Diff against HEAD: show working (uncommitted) changes.
    #[default]
    Head,
    /// Diff against the merge base between HEAD and the repository's
    /// default branch: show all changes on the branch.
    ///
    /// Repositories where no default branch can be resolved fall back
    /// to `head` behavior.
    DefaultBranch,
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
/// How file paths are displayed in git-related UI.
pub enum GitPathStyle {
    /// Show file name first, then path
    #[default]
    FileNameFirst,
    /// Show full path first
    FilePathFirst,
}

/// Diagnostics-related settings.
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
/// Settings for using LSP pull diagnostics mechanism in Zed.
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
/// Settings for showing inline diagnostics.
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

    /// The minimum severity of the diagnostics to show inline.
    /// Inherits editor's diagnostics' max severity settings when `null`.
    ///
    /// Default: null
    pub max_severity: Option<DiagnosticSeverityContent>,
}

/// Configuration for Node.js integration.
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

/// Configuration for how direnv configuration should be loaded.
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
/// Which level to use to filter out diagnostics displayed in the editor.
pub enum DiagnosticSeverityContent {
    /// No diagnostics are shown.
    Off,
    /// Show only errors.
    Error,
    /// Show errors and warnings.
    Warning,
    /// Show errors, warnings, and information.
    Info,
    /// Show all including hints.
    Hint,
    /// Allow all diagnostics.
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

/// The kind of a Git hosting provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum GitHostingProviderKind {
    /// A GitHub-compatible provider.
    Github,
    /// A GitLab-compatible provider.
    Gitlab,
    /// A Bitbucket-compatible provider.
    Bitbucket,
    /// A Gitea-compatible provider.
    Gitea,
    /// A Forgejo-compatible provider.
    Forgejo,
    /// A SourceHut-compatible provider.
    SourceHut,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{REST_OF_FILE_SCAN_EXCLUSIONS, merge_from::MergeFrom};

    fn exclusions(globs: &[&str]) -> WorktreeSettingsContent {
        WorktreeSettingsContent {
            file_scan_exclusions: Some(SplicingVec::from(
                globs
                    .iter()
                    .map(|glob| glob.to_string())
                    .collect::<Vec<_>>(),
            )),
            ..Default::default()
        }
    }

    #[test]
    fn test_file_scan_exclusions_splice_rest_of_list() {
        let defaults = exclusions(&["**/.git", "**/.DS_Store"]);

        let mut extended = defaults.clone();
        extended.merge_from(&exclusions(&[
            "**/node_modules",
            REST_OF_FILE_SCAN_EXCLUSIONS,
        ]));
        assert_eq!(
            extended.file_scan_exclusions.unwrap().0,
            vec!["**/node_modules", "**/.git", "**/.DS_Store"]
        );

        let mut replaced = defaults;
        replaced.merge_from(&exclusions(&["**/node_modules"]));
        assert_eq!(
            replaced.file_scan_exclusions.unwrap().0,
            vec!["**/node_modules"]
        );
    }

    #[test]
    fn test_file_scan_exclusions_splice_each_layer() {
        let mut settings = exclusions(&["**/.git"]);
        settings.merge_from(&exclusions(&[REST_OF_FILE_SCAN_EXCLUSIONS, "**/target"]));
        settings.merge_from(&exclusions(&[REST_OF_FILE_SCAN_EXCLUSIONS, "**/dist"]));

        assert_eq!(
            settings.file_scan_exclusions.unwrap().0,
            vec!["**/.git", "**/target", "**/dist"]
        );
    }

    #[test]
    fn test_file_scan_exclusions_splice_edge_cases() {
        let mut repeated = exclusions(&["**/.git"]);
        repeated.merge_from(&exclusions(&[
            REST_OF_FILE_SCAN_EXCLUSIONS,
            REST_OF_FILE_SCAN_EXCLUSIONS,
        ]));
        assert_eq!(repeated.file_scan_exclusions.unwrap().0, vec!["**/.git"]);

        let mut relisted = exclusions(&["**/.git", "**/.DS_Store"]);
        relisted.merge_from(&exclusions(&["**/.git", REST_OF_FILE_SCAN_EXCLUSIONS]));
        assert_eq!(
            relisted.file_scan_exclusions.unwrap().0,
            vec!["**/.git", "**/.DS_Store"]
        );

        let mut cleared = exclusions(&["**/.git"]);
        cleared.merge_from(&exclusions(&[]));
        assert!(cleared.file_scan_exclusions.unwrap().0.is_empty());

        let mut unchanged = exclusions(&["**/.git", "**/.DS_Store"]);
        unchanged.merge_from(&exclusions(&[REST_OF_FILE_SCAN_EXCLUSIONS]));
        assert_eq!(
            unchanged.file_scan_exclusions.unwrap().0,
            vec!["**/.git", "**/.DS_Store"]
        );
    }

    #[test]
    fn test_file_scan_exclusions_splice_without_a_base_layer() {
        let mut settings = WorktreeSettingsContent::default();
        settings.merge_from(&exclusions(&[REST_OF_FILE_SCAN_EXCLUSIONS, "**/target"]));

        // `Option::merge_from` replaces a `None` base outright rather than
        // splicing, so the sentinel survives here. `assets/settings/default.json`
        // always populates this field, and `WorktreeSettings::from_settings`
        // unwraps it, so no glob is ever compiled from this state.
        assert_eq!(
            settings.file_scan_exclusions.unwrap().0,
            vec![REST_OF_FILE_SCAN_EXCLUSIONS, "**/target"]
        );
    }

    #[test]
    fn test_stdio_context_server_without_args() {
        let settings: ContextServerSettingsContent =
            serde_json::from_str(r#"{ "command": "echo" }"#)
                .expect("stdio context server without `args` should parse");
        let ContextServerSettingsContent::Stdio { command, .. } = settings else {
            panic!("expected Stdio variant, got {settings:?}");
        };
        assert_eq!(command.path, PathBuf::from("echo"));
        assert!(command.args.is_empty());

        let settings: ContextServerSettingsContent =
            serde_json::from_str(r#"{ "command": "echo", "args": ["hello"] }"#).unwrap();
        let ContextServerSettingsContent::Stdio { command, .. } = settings else {
            panic!("expected Stdio variant, got {settings:?}");
        };
        assert_eq!(command.args, vec!["hello".to_string()]);
    }
}

impl ProjectSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            all_languages: AllLanguageSettingsContent::defaults(),
            worktree: WorktreeSettingsContent::defaults(),
            lsp: LspSettingsMap(HashMap::default()),
            terminal: None,
            dap: HashMap::from_iter([(
                Arc::from("CodeLLDB"),
                DapSettingsContent {
                    binary: None,
                    args: None,
                    env: Some(HashMap::from_iter([(
                        String::from("RUST_LOG"),
                        String::from("info"),
                    )])),
                },
            )]),
            context_servers: HashMap::default(),
            context_server_timeout: Some(60),
            load_direnv: Some(DirenvSettings::Direct),
            git_hosting_providers: Some(ExtendingVec(Vec::new())),
            disable_ai: Some(SaturatingBool(false)),
        }
    }
}

impl WorktreeSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            prevent_sharing_in_public_channels: false,
            file_scan_exclusions: Some(SplicingVec(vec![
                String::from("**/.git"),
                String::from("**/.svn"),
                String::from("**/.hg"),
                String::from("**/.jj"),
                String::from("**/.sl"),
                String::from("**/.repo"),
                String::from("**/CVS"),
                String::from("**/.DS_Store"),
                String::from("**/Thumbs.db"),
                String::from("**/.classpath"),
                String::from("**/.settings"),
            ])),
            file_scan_inclusions: Some(vec![String::from(".env*")]),
            scan_symlinks: Some(ScanSymlinksSetting::Expanded),
            file_scan_depth: Some(5),
            private_files: Some(ExtendingVec(vec![
                String::from("**/.env*"),
                String::from("**/*.pem"),
                String::from("**/*.key"),
                String::from("**/*.cert"),
                String::from("**/*.crt"),
                String::from("**/secrets.yml"),
            ])),
            hidden_files: Some(vec![String::from("**/.*")]),
            read_only_files: Some(Vec::new()),
        }
    }
}

impl DiagnosticsSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            button: Some(true),
            include_warnings: Some(true),
            lsp_pull_diagnostics: Some(LspPullDiagnosticsSettingsContent::defaults()),
            inline: Some(InlineDiagnosticsSettingsContent::defaults()),
        }
    }
}

impl LspPullDiagnosticsSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            enabled: Some(true),
            debounce_ms: Some(DelayMs(50)),
        }
    }
}

impl InlineDiagnosticsSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            enabled: Some(false),
            update_debounce_ms: Some(DelayMs(150)),
            padding: Some(4),
            min_column: Some(0),
            max_severity: None,
        }
    }
}

impl GitSettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            enabled: Some(GitEnabledSettings::defaults()),
            git_gutter: Some(GitGutterSetting::TrackedFiles),
            gutter_debounce: Some(0),
            inline_blame: Some(InlineBlameSettings::defaults()),
            blame: Some(BlameSettings::defaults()),
            branch_picker: Some(BranchPickerSettingsContent::defaults()),
            file_diff: Some(FileDiffSettingsContent::defaults()),
            hunk_style: Some(GitHunkStyleSetting::StagedHollow),
            diff_base: Some(GitDiffBaseSetting::Head),
            path_style: Some(GitPathStyle::FileNameFirst),
            show_stage_restore_buttons: Some(true),
            worktree_directory: Some(String::from("../worktrees")),
        }
    }
}

impl GitEnabledSettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            disable_git: Some(false),
            enable_status: Some(true),
            enable_diff: Some(true),
        }
    }
}

impl InlineBlameSettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            enabled: Some(true),
            delay_ms: Some(DelayMs(0)),
            location: Some(InlineBlameLocation::Inline),
            padding: Some(7),
            min_column: Some(0),
            show_commit_summary: Some(false),
        }
    }
}

impl BlameSettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            show_avatar: Some(true),
        }
    }
}

impl BranchPickerSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            show_author_name: Some(true),
        }
    }
}

impl FileDiffSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            show_full_file: Some(true),
        }
    }
}

impl GlobalLspSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            button: Some(true),
            request_timeout: Some(120),
            max_buffer_line_length: Some(20000),
            notifications: Some(LspNotificationSettingsContent::defaults()),
            semantic_token_rules: Some(SemanticTokenRules { rules: Vec::new() }),
        }
    }
}

impl LspNotificationSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            dismiss_timeout_ms: Some(5000),
        }
    }
}

impl NodeBinarySettings {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            path: None,
            npm_path: None,
            ignore_system_version: Some(false),
        }
    }
}

impl SessionSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            restore_unsaved_buffers: Some(true),
            trust_all_worktrees: Some(false),
        }
    }
}
