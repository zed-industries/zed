use anyhow::Context as _;
use collections::HashMap;
use context_server::ContextServerCommand;
use dap::adapters::DebugAdapterName;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{AsyncApp, BorrowAppContext, Context, Entity, EventEmitter, Subscription, Task};
use lsp::LanguageServerName;
use paths::{
    EDITORCONFIG_NAME, local_debug_file_relative_path, local_settings_file_relative_path,
    local_tasks_file_relative_path, local_vscode_launch_file_relative_path,
    local_vscode_tasks_file_relative_path, task_file_name,
};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, REMOTE_SERVER_PROJECT_ID},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use settings::DirenvSettings;
pub use settings::LspSettings;
use settings::{
    DapSettingsContent, EditorconfigEvent, InvalidSettingsError, LocalSettingsKind,
    LocalSettingsPath, RegisterSetting, Settings, SettingsLocation, SettingsStore,
    parse_json_with_comments, watch_config_file,
};
use std::{cell::OnceCell, collections::BTreeMap, path::PathBuf, sync::Arc, time::Duration};
use task::{DebugTaskFile, TaskTemplates, VsCodeDebugTaskFile, VsCodeTaskFile};
use util::{ResultExt, rel_path::RelPath, serde::default_true};
use worktree::{PathChange, UpdatedEntriesSet, Worktree, WorktreeId};

use crate::{
    task_store::{TaskSettingsLocation, TaskStore},
    trusted_worktrees::{PathTrust, TrustedWorktrees, TrustedWorktreesEvent},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

#[derive(Debug, Clone, RegisterSetting)]
pub struct ProjectSettings {
    /// Configuration for language servers.
    ///
    /// The following settings can be overridden for specific language servers:
    /// - initialization_options
    ///
    /// To override settings for a language, add an entry for that language server's
    /// name to the lsp value.
    /// Default: null
    // todo(settings-follow-up)
    // We should change to use a non content type (settings::LspSettings is a content type)
    // Note: Will either require merging with defaults, which also requires deciding where the defaults come from,
    //       or case by case deciding which fields are optional and which are actually required.
    pub lsp: HashMap<LanguageServerName, settings::LspSettings>,

    /// Common language server settings.
    pub global_lsp_settings: GlobalLspSettings,

    /// Configuration for Debugger-related features
    pub dap: HashMap<DebugAdapterName, DapSettings>,

    /// Settings for context servers used for AI-related features.
    pub context_servers: HashMap<Arc<str>, ContextServerSettings>,

    /// Default timeout for context server requests in seconds.
    pub context_server_timeout: u64,

    /// Configuration for Diagnostics-related features.
    pub diagnostics: DiagnosticsSettings,

    /// Configuration for Git-related features
    pub git: GitSettings,

    /// Configuration for Node-related features
    pub node: NodeBinarySettings,

    /// Configuration for how direnv configuration should be loaded
    pub load_direnv: DirenvSettings,

    /// Configuration for session-related features
    pub session: SessionSettings,
}

#[derive(Copy, Clone, Debug)]
pub struct SessionSettings {
    /// Whether or not to restore unsaved buffers on restart.
    ///
    /// If this is true, user won't be prompted whether to save/discard
    /// dirty files when closing the application.
    ///
    /// Default: true
    pub restore_unsaved_buffers: bool,
    /// Whether or not to skip worktree trust checks.
    /// When trusted, project settings are synchronized automatically,
    /// language and MCP servers are downloaded and started automatically.
    ///
    /// Default: false
    pub trust_all_worktrees: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NodeBinarySettings {
    /// The path to the Node binary.
    pub path: Option<String>,
    /// The path to the npm binary Zed should use (defaults to `.path/../npm`).
    pub npm_path: Option<String>,
    /// If enabled, Zed will download its own copy of Node.
    pub ignore_system_version: bool,
}

impl From<settings::NodeBinarySettings> for NodeBinarySettings {
    fn from(settings: settings::NodeBinarySettings) -> Self {
        Self {
            path: settings.path,
            npm_path: settings.npm_path,
            ignore_system_version: settings.ignore_system_version.unwrap_or(false),
        }
    }
}

/// Common language server settings.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalLspSettings {
    /// Whether to show the LSP servers button in the status bar.
    ///
    /// Default: `true`
    pub button: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ContextServerSettings {
    Stdio {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// If true, run this server on the remote server when using remote development.
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
        /// Optional authentication configuration for the remote server.
        #[serde(skip_serializing_if = "HashMap::is_empty", default)]
        headers: HashMap<String, String>,
        /// Timeout for tool calls in milliseconds.
        timeout: Option<u64>,
    },
    Extension {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// If true, run this server on the remote server when using remote development.
        #[serde(default)]
        remote: bool,
        /// The settings for this context server specified by the extension.
        ///
        /// Consult the documentation for the context server to see what settings
        /// are supported.
        settings: serde_json::Value,
    },
}

impl From<settings::ContextServerSettingsContent> for ContextServerSettings {
    fn from(value: settings::ContextServerSettingsContent) -> Self {
        match value {
            settings::ContextServerSettingsContent::Stdio {
                enabled,
                remote,
                command,
            } => ContextServerSettings::Stdio {
                enabled,
                remote,
                command,
            },
            settings::ContextServerSettingsContent::Extension {
                enabled,
                remote,
                settings,
            } => ContextServerSettings::Extension {
                enabled,
                remote,
                settings,
            },
            settings::ContextServerSettingsContent::Http {
                enabled,
                url,
                headers,
                timeout,
            } => ContextServerSettings::Http {
                enabled,
                url,
                headers,
                timeout,
            },
        }
    }
}
impl Into<settings::ContextServerSettingsContent> for ContextServerSettings {
    fn into(self) -> settings::ContextServerSettingsContent {
        match self {
            ContextServerSettings::Stdio {
                enabled,
                remote,
                command,
            } => settings::ContextServerSettingsContent::Stdio {
                enabled,
                remote,
                command,
            },
            ContextServerSettings::Extension {
                enabled,
                remote,
                settings,
            } => settings::ContextServerSettingsContent::Extension {
                enabled,
                remote,
                settings,
            },
            ContextServerSettings::Http {
                enabled,
                url,
                headers,
                timeout,
            } => settings::ContextServerSettingsContent::Http {
                enabled,
                url,
                headers,
                timeout,
            },
        }
    }
}

impl ContextServerSettings {
    pub fn default_extension() -> Self {
        Self::Extension {
            enabled: true,
            remote: false,
            settings: serde_json::json!({}),
        }
    }

    pub fn enabled(&self) -> bool {
        match self {
            ContextServerSettings::Stdio { enabled, .. } => *enabled,
            ContextServerSettings::Http { enabled, .. } => *enabled,
            ContextServerSettings::Extension { enabled, .. } => *enabled,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        match self {
            ContextServerSettings::Stdio { enabled: e, .. } => *e = enabled,
            ContextServerSettings::Http { enabled: e, .. } => *e = enabled,
            ContextServerSettings::Extension { enabled: e, .. } => *e = enabled,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DiagnosticSeverity {
    // No diagnostics are shown.
    Off,
    Error,
    Warning,
    Info,
    Hint,
}

impl DiagnosticSeverity {
    pub fn into_lsp(self) -> Option<lsp::DiagnosticSeverity> {
        match self {
            DiagnosticSeverity::Off => None,
            DiagnosticSeverity::Error => Some(lsp::DiagnosticSeverity::ERROR),
            DiagnosticSeverity::Warning => Some(lsp::DiagnosticSeverity::WARNING),
            DiagnosticSeverity::Info => Some(lsp::DiagnosticSeverity::INFORMATION),
            DiagnosticSeverity::Hint => Some(lsp::DiagnosticSeverity::HINT),
        }
    }
}

impl From<settings::DiagnosticSeverityContent> for DiagnosticSeverity {
    fn from(severity: settings::DiagnosticSeverityContent) -> Self {
        match severity {
            settings::DiagnosticSeverityContent::Off => DiagnosticSeverity::Off,
            settings::DiagnosticSeverityContent::Error => DiagnosticSeverity::Error,
            settings::DiagnosticSeverityContent::Warning => DiagnosticSeverity::Warning,
            settings::DiagnosticSeverityContent::Info => DiagnosticSeverity::Info,
            settings::DiagnosticSeverityContent::Hint => DiagnosticSeverity::Hint,
            settings::DiagnosticSeverityContent::All => DiagnosticSeverity::Hint,
        }
    }
}

/// Determines the severity of the diagnostic that should be moved to.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GoToDiagnosticSeverity {
    /// Errors
    Error = 3,
    /// Warnings
    Warning = 2,
    /// Information
    Information = 1,
    /// Hints
    Hint = 0,
}

impl From<lsp::DiagnosticSeverity> for GoToDiagnosticSeverity {
    fn from(severity: lsp::DiagnosticSeverity) -> Self {
        match severity {
            lsp::DiagnosticSeverity::ERROR => Self::Error,
            lsp::DiagnosticSeverity::WARNING => Self::Warning,
            lsp::DiagnosticSeverity::INFORMATION => Self::Information,
            lsp::DiagnosticSeverity::HINT => Self::Hint,
            _ => Self::Error,
        }
    }
}

impl GoToDiagnosticSeverity {
    pub fn min() -> Self {
        Self::Hint
    }

    pub fn max() -> Self {
        Self::Error
    }
}

/// Allows filtering diagnostics that should be moved to.
#[derive(PartialEq, Clone, Copy, Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum GoToDiagnosticSeverityFilter {
    /// Move to diagnostics of a specific severity.
    Only(GoToDiagnosticSeverity),

    /// Specify a range of severities to include.
    Range {
        /// Minimum severity to move to. Defaults no "error".
        #[serde(default = "GoToDiagnosticSeverity::min")]
        min: GoToDiagnosticSeverity,
        /// Maximum severity to move to. Defaults to "hint".
        #[serde(default = "GoToDiagnosticSeverity::max")]
        max: GoToDiagnosticSeverity,
    },
}

impl Default for GoToDiagnosticSeverityFilter {
    fn default() -> Self {
        Self::Range {
            min: GoToDiagnosticSeverity::min(),
            max: GoToDiagnosticSeverity::max(),
        }
    }
}

impl GoToDiagnosticSeverityFilter {
    pub fn matches(&self, severity: lsp::DiagnosticSeverity) -> bool {
        let severity: GoToDiagnosticSeverity = severity.into();
        match self {
            Self::Only(target) => *target == severity,
            Self::Range { min, max } => severity >= *min && severity <= *max,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GitSettings {
    /// Whether or not git integration is enabled.
    ///
    /// Default: true
    pub enabled: GitEnabledSettings,
    /// Whether or not to show the git gutter.
    ///
    /// Default: tracked_files
    pub git_gutter: settings::GitGutterSetting,
    /// Sets the debounce threshold (in milliseconds) after which changes are reflected in the git gutter.
    ///
    /// Default: 0
    pub gutter_debounce: u64,
    /// Whether or not to show git blame data inline in
    /// the currently focused line.
    ///
    /// Default: on
    pub inline_blame: InlineBlameSettings,
    /// Git blame settings.
    pub blame: BlameSettings,
    /// Which information to show in the branch picker.
    ///
    /// Default: on
    pub branch_picker: BranchPickerSettings,
    /// How hunks are displayed visually in the editor.
    ///
    /// Default: staged_hollow
    pub hunk_style: settings::GitHunkStyleSetting,
    /// How file paths are displayed in the git gutter.
    ///
    /// Default: file_name_first
    pub path_style: GitPathStyle,
}

#[derive(Clone, Copy, Debug)]
pub struct GitEnabledSettings {
    /// Whether git integration is enabled for showing git status.
    ///
    /// Default: true
    pub status: bool,
    /// Whether git integration is enabled for showing diffs.
    ///
    /// Default: true
    pub diff: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum GitPathStyle {
    #[default]
    FileNameFirst,
    FilePathFirst,
}

impl From<settings::GitPathStyle> for GitPathStyle {
    fn from(style: settings::GitPathStyle) -> Self {
        match style {
            settings::GitPathStyle::FileNameFirst => GitPathStyle::FileNameFirst,
            settings::GitPathStyle::FilePathFirst => GitPathStyle::FilePathFirst,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InlineBlameSettings {
    /// Whether or not to show git blame data inline in
    /// the currently focused line.
    ///
    /// Default: true
    pub enabled: bool,
    /// Whether to only show the inline blame information
    /// after a delay once the cursor stops moving.
    ///
    /// Default: 0
    pub delay_ms: settings::DelayMs,
    /// The amount of padding between the end of the source line and the start
    /// of the inline blame in units of columns.
    ///
    /// Default: 7
    pub padding: u32,
    /// The minimum column number to show the inline blame information at
    ///
    /// Default: 0
    pub min_column: u32,
    /// Whether to show commit summary as part of the inline blame.
    ///
    /// Default: false
    pub show_commit_summary: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct BlameSettings {
    /// Whether to show the avatar of the author of the commit.
    ///
    /// Default: true
    pub show_avatar: bool,
}

impl GitSettings {
    pub fn inline_blame_delay(&self) -> Option<Duration> {
        if self.inline_blame.delay_ms.0 > 0 {
            Some(Duration::from_millis(self.inline_blame.delay_ms.0))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BranchPickerSettings {
    /// Whether to show author name as part of the commit information.
    ///
    /// Default: false
    #[serde(default)]
    pub show_author_name: bool,
}

impl Default for BranchPickerSettings {
    fn default() -> Self {
        Self {
            show_author_name: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiagnosticsSettings {
    /// Whether to show the project diagnostics button in the status bar.
    pub button: bool,

    /// Whether or not to include warning diagnostics.
    pub include_warnings: bool,

    /// Settings for using LSP pull diagnostics mechanism in Zed.
    pub lsp_pull_diagnostics: LspPullDiagnosticsSettings,

    /// Settings for showing inline diagnostics.
    pub inline: InlineDiagnosticsSettings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InlineDiagnosticsSettings {
    /// Whether or not to show inline diagnostics
    ///
    /// Default: false
    pub enabled: bool,
    /// Whether to only show the inline diagnostics after a delay after the
    /// last editor event.
    ///
    /// Default: 150
    pub update_debounce_ms: u64,
    /// The amount of padding between the end of the source line and the start
    /// of the inline diagnostic in units of columns.
    ///
    /// Default: 4
    pub padding: u32,
    /// The minimum column to display inline diagnostics. This setting can be
    /// used to horizontally align inline diagnostics at some position. Lines
    /// longer than this value will still push diagnostics further to the right.
    ///
    /// Default: 0
    pub min_column: u32,

    pub max_severity: Option<DiagnosticSeverity>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct LspPullDiagnosticsSettings {
    /// Whether to pull for diagnostics or not.
    ///
    /// Default: true
    pub enabled: bool,
    /// Minimum time to wait before pulling diagnostics from the language server(s).
    /// 0 turns the debounce off.
    ///
    /// Default: 50
    pub debounce_ms: u64,
}

impl Settings for ProjectSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let project = &content.project.clone();
        let diagnostics = content.diagnostics.as_ref().unwrap();
        let lsp_pull_diagnostics = diagnostics.lsp_pull_diagnostics.as_ref().unwrap();
        let inline_diagnostics = diagnostics.inline.as_ref().unwrap();

        let git = content.git.as_ref().unwrap();
        let git_enabled = {
            GitEnabledSettings {
                status: git.enabled.as_ref().unwrap().is_git_status_enabled(),
                diff: git.enabled.as_ref().unwrap().is_git_diff_enabled(),
            }
        };
        let git_settings = GitSettings {
            enabled: git_enabled,
            git_gutter: git.git_gutter.unwrap(),
            gutter_debounce: git.gutter_debounce.unwrap_or_default(),
            inline_blame: {
                let inline = git.inline_blame.unwrap();
                InlineBlameSettings {
                    enabled: inline.enabled.unwrap(),
                    delay_ms: inline.delay_ms.unwrap(),
                    padding: inline.padding.unwrap(),
                    min_column: inline.min_column.unwrap(),
                    show_commit_summary: inline.show_commit_summary.unwrap(),
                }
            },
            blame: {
                let blame = git.blame.unwrap();
                BlameSettings {
                    show_avatar: blame.show_avatar.unwrap(),
                }
            },
            branch_picker: {
                let branch_picker = git.branch_picker.unwrap();
                BranchPickerSettings {
                    show_author_name: branch_picker.show_author_name.unwrap(),
                }
            },
            hunk_style: git.hunk_style.unwrap(),
            path_style: git.path_style.unwrap().into(),
        };
        Self {
            context_servers: project
                .context_servers
                .clone()
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            context_server_timeout: project.context_server_timeout.unwrap_or(60),
            lsp: project
                .lsp
                .clone()
                .into_iter()
                .map(|(key, value)| (LanguageServerName(key.into()), value))
                .collect(),
            global_lsp_settings: GlobalLspSettings {
                button: content
                    .global_lsp_settings
                    .as_ref()
                    .unwrap()
                    .button
                    .unwrap(),
            },
            dap: project
                .dap
                .clone()
                .into_iter()
                .map(|(key, value)| (DebugAdapterName(key.into()), DapSettings::from(value)))
                .collect(),
            diagnostics: DiagnosticsSettings {
                button: diagnostics.button.unwrap(),
                include_warnings: diagnostics.include_warnings.unwrap(),
                lsp_pull_diagnostics: LspPullDiagnosticsSettings {
                    enabled: lsp_pull_diagnostics.enabled.unwrap(),
                    debounce_ms: lsp_pull_diagnostics.debounce_ms.unwrap().0,
                },
                inline: InlineDiagnosticsSettings {
                    enabled: inline_diagnostics.enabled.unwrap(),
                    update_debounce_ms: inline_diagnostics.update_debounce_ms.unwrap().0,
                    padding: inline_diagnostics.padding.unwrap(),
                    min_column: inline_diagnostics.min_column.unwrap(),
                    max_severity: inline_diagnostics.max_severity.map(Into::into),
                },
            },
            git: git_settings,
            node: content.node.clone().unwrap().into(),
            load_direnv: project.load_direnv.clone().unwrap(),
            session: SessionSettings {
                restore_unsaved_buffers: content.session.unwrap().restore_unsaved_buffers.unwrap(),
                trust_all_worktrees: content.session.unwrap().trust_all_worktrees.unwrap(),
            },
        }
    }
}

pub enum SettingsObserverMode {
    Local(Arc<dyn Fs>),
    Remote { via_collab: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub enum SettingsObserverEvent {
    LocalSettingsUpdated(Result<PathBuf, InvalidSettingsError>),
    LocalTasksUpdated(Result<PathBuf, InvalidSettingsError>),
    LocalDebugScenariosUpdated(Result<PathBuf, InvalidSettingsError>),
}

impl EventEmitter<SettingsObserverEvent> for SettingsObserver {}

pub struct SettingsObserver {
    mode: SettingsObserverMode,
    downstream_client: Option<AnyProtoClient>,
    worktree_store: Entity<WorktreeStore>,
    project_id: u64,
    task_store: Entity<TaskStore>,
    pending_local_settings:
        HashMap<PathTrust, BTreeMap<(WorktreeId, Arc<RelPath>), Option<String>>>,
    _trusted_worktrees_watcher: Option<Subscription>,
    _user_settings_watcher: Option<Subscription>,
    _editorconfig_watcher: Option<Subscription>,
    _global_task_config_watcher: Task<()>,
    _global_debug_config_watcher: Task<()>,
}

/// SettingsObserver observers changes to .zed/{settings, task}.json files in local worktrees
/// (or the equivalent protobuf messages from upstream) and updates local settings
/// and sends notifications downstream.
/// In ssh mode it also monitors ~/.config/zed/{settings, task}.json and sends the content
/// upstream.
impl SettingsObserver {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_message_handler(Self::handle_update_worktree_settings);
        client.add_entity_message_handler(Self::handle_update_user_settings);
    }

    pub fn new_local(
        fs: Arc<dyn Fs>,
        worktree_store: Entity<WorktreeStore>,
        task_store: Entity<TaskStore>,
        watch_global_configs: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();

        let _trusted_worktrees_watcher =
            TrustedWorktrees::try_get_global(cx).map(|trusted_worktrees| {
                cx.subscribe(
                    &trusted_worktrees,
                    move |settings_observer, _, e, cx| match e {
                        TrustedWorktreesEvent::Trusted(_, trusted_paths) => {
                            for trusted_path in trusted_paths {
                                if let Some(pending_local_settings) = settings_observer
                                    .pending_local_settings
                                    .remove(trusted_path)
                                {
                                    for ((worktree_id, directory_path), settings_contents) in
                                        pending_local_settings
                                    {
                                        let path =
                                            LocalSettingsPath::InWorktree(directory_path.clone());
                                        apply_local_settings(
                                            worktree_id,
                                            path.clone(),
                                            LocalSettingsKind::Settings,
                                            &settings_contents,
                                            cx,
                                        );
                                        if let Some(downstream_client) =
                                            &settings_observer.downstream_client
                                        {
                                            downstream_client
                                                .send(proto::UpdateWorktreeSettings {
                                                    project_id: settings_observer.project_id,
                                                    worktree_id: worktree_id.to_proto(),
                                                    path: path.to_proto(),
                                                    content: settings_contents,
                                                    kind: Some(
                                                        local_settings_kind_to_proto(
                                                            LocalSettingsKind::Settings,
                                                        )
                                                        .into(),
                                                    ),
                                                    outside_worktree: Some(false),
                                                })
                                                .log_err();
                                        }
                                    }
                                }
                            }
                        }
                        TrustedWorktreesEvent::Restricted(..) => {}
                    },
                )
            });

        let editorconfig_store = cx.global::<SettingsStore>().editorconfig_store.clone();
        let _editorconfig_watcher = cx.subscribe(
            &editorconfig_store,
            |this, _, event: &EditorconfigEvent, cx| {
                let EditorconfigEvent::ExternalConfigChanged {
                    path,
                    content,
                    affected_worktree_ids,
                } = event;
                for worktree_id in affected_worktree_ids {
                    if let Some(worktree) = this
                        .worktree_store
                        .read(cx)
                        .worktree_for_id(*worktree_id, cx)
                    {
                        this.update_settings(
                            worktree,
                            [(
                                path.clone(),
                                LocalSettingsKind::Editorconfig,
                                content.clone(),
                            )],
                            false,
                            cx,
                        );
                    }
                }
            },
        );

        Self {
            worktree_store,
            task_store,
            mode: SettingsObserverMode::Local(fs.clone()),
            downstream_client: None,
            _trusted_worktrees_watcher,
            pending_local_settings: HashMap::default(),
            _user_settings_watcher: None,
            _editorconfig_watcher: Some(_editorconfig_watcher),
            project_id: REMOTE_SERVER_PROJECT_ID,
            _global_task_config_watcher: if watch_global_configs {
                Self::subscribe_to_global_task_file_changes(
                    fs.clone(),
                    paths::tasks_file().clone(),
                    cx,
                )
            } else {
                Task::ready(())
            },
            _global_debug_config_watcher: if watch_global_configs {
                Self::subscribe_to_global_debug_scenarios_changes(
                    fs.clone(),
                    paths::debug_scenarios_file().clone(),
                    cx,
                )
            } else {
                Task::ready(())
            },
        }
    }

    pub fn new_remote(
        fs: Arc<dyn Fs>,
        worktree_store: Entity<WorktreeStore>,
        task_store: Entity<TaskStore>,
        upstream_client: Option<AnyProtoClient>,
        via_collab: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut user_settings_watcher = None;
        if cx.try_global::<SettingsStore>().is_some() {
            if let Some(upstream_client) = upstream_client {
                let mut user_settings = None;
                user_settings_watcher = Some(cx.observe_global::<SettingsStore>(move |_, cx| {
                    if let Some(new_settings) = cx.global::<SettingsStore>().raw_user_settings() {
                        if Some(new_settings) != user_settings.as_ref() {
                            if let Some(new_settings_string) =
                                serde_json::to_string(new_settings).ok()
                            {
                                user_settings = Some(new_settings.clone());
                                upstream_client
                                    .send(proto::UpdateUserSettings {
                                        project_id: REMOTE_SERVER_PROJECT_ID,
                                        contents: new_settings_string,
                                    })
                                    .log_err();
                            }
                        }
                    }
                }));
            }
        };

        Self {
            worktree_store,
            task_store,
            mode: SettingsObserverMode::Remote { via_collab },
            downstream_client: None,
            project_id: REMOTE_SERVER_PROJECT_ID,
            _trusted_worktrees_watcher: None,
            pending_local_settings: HashMap::default(),
            _user_settings_watcher: user_settings_watcher,
            _editorconfig_watcher: None,
            _global_task_config_watcher: Self::subscribe_to_global_task_file_changes(
                fs.clone(),
                paths::tasks_file().clone(),
                cx,
            ),
            _global_debug_config_watcher: Self::subscribe_to_global_debug_scenarios_changes(
                fs.clone(),
                paths::debug_scenarios_file().clone(),
                cx,
            ),
        }
    }

    pub fn shared(
        &mut self,
        project_id: u64,
        downstream_client: AnyProtoClient,
        cx: &mut Context<Self>,
    ) {
        self.project_id = project_id;
        self.downstream_client = Some(downstream_client.clone());

        let store = cx.global::<SettingsStore>();
        for worktree in self.worktree_store.read(cx).worktrees() {
            let worktree_id = worktree.read(cx).id().to_proto();
            for (path, content) in store.local_settings(worktree.read(cx).id()) {
                let content = serde_json::to_string(&content).unwrap();
                downstream_client
                    .send(proto::UpdateWorktreeSettings {
                        project_id,
                        worktree_id,
                        path: path.to_proto(),
                        content: Some(content),
                        kind: Some(
                            local_settings_kind_to_proto(LocalSettingsKind::Settings).into(),
                        ),
                        outside_worktree: Some(false),
                    })
                    .log_err();
            }
            for (path, content, _) in store
                .editorconfig_store
                .read(cx)
                .local_editorconfig_settings(worktree.read(cx).id())
            {
                downstream_client
                    .send(proto::UpdateWorktreeSettings {
                        project_id,
                        worktree_id,
                        path: path.to_proto(),
                        content: Some(content.to_owned()),
                        kind: Some(
                            local_settings_kind_to_proto(LocalSettingsKind::Editorconfig).into(),
                        ),
                        outside_worktree: Some(path.is_outside_worktree()),
                    })
                    .log_err();
            }
        }
    }

    pub fn unshared(&mut self, _: &mut Context<Self>) {
        self.downstream_client = None;
    }

    async fn handle_update_worktree_settings(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateWorktreeSettings>,
        mut cx: AsyncApp,
    ) -> anyhow::Result<()> {
        let kind = match envelope.payload.kind {
            Some(kind) => proto::LocalSettingsKind::from_i32(kind)
                .with_context(|| format!("unknown kind {kind}"))?,
            None => proto::LocalSettingsKind::Settings,
        };

        let path = LocalSettingsPath::from_proto(
            &envelope.payload.path,
            envelope.payload.outside_worktree.unwrap_or(false),
        )?;

        this.update(&mut cx, |this, cx| {
            let is_via_collab = match &this.mode {
                SettingsObserverMode::Local(..) => false,
                SettingsObserverMode::Remote { via_collab } => *via_collab,
            };
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            let Some(worktree) = this
                .worktree_store
                .read(cx)
                .worktree_for_id(worktree_id, cx)
            else {
                return;
            };

            this.update_settings(
                worktree,
                [(
                    path,
                    local_settings_kind_from_proto(kind),
                    envelope.payload.content,
                )],
                is_via_collab,
                cx,
            );
        });
        Ok(())
    }

    async fn handle_update_user_settings(
        _: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateUserSettings>,
        cx: AsyncApp,
    ) -> anyhow::Result<()> {
        cx.update_global(|settings_store: &mut SettingsStore, cx| {
            settings_store
                .set_user_settings(&envelope.payload.contents, cx)
                .result()
                .context("setting new user settings")?;
            anyhow::Ok(())
        })?;
        Ok(())
    }

    fn on_worktree_store_event(
        &mut self,
        _: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(worktree) => cx
                .subscribe(worktree, |this, worktree, event, cx| {
                    if let worktree::Event::UpdatedEntries(changes) = event {
                        this.update_local_worktree_settings(&worktree, changes, cx)
                    }
                })
                .detach(),
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id) => {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    store.clear_local_settings(*worktree_id, cx).log_err();
                });
            }
            _ => {}
        }
    }

    fn update_local_worktree_settings(
        &mut self,
        worktree: &Entity<Worktree>,
        changes: &UpdatedEntriesSet,
        cx: &mut Context<Self>,
    ) {
        let SettingsObserverMode::Local(fs) = &self.mode else {
            return;
        };

        let mut settings_contents = Vec::new();
        for (path, _, change) in changes.iter() {
            let (settings_dir, kind) = if path.ends_with(local_settings_file_relative_path()) {
                let settings_dir = path
                    .ancestors()
                    .nth(local_settings_file_relative_path().components().count())
                    .unwrap()
                    .into();
                (settings_dir, LocalSettingsKind::Settings)
            } else if path.ends_with(local_tasks_file_relative_path()) {
                let settings_dir = path
                    .ancestors()
                    .nth(
                        local_tasks_file_relative_path()
                            .components()
                            .count()
                            .saturating_sub(1),
                    )
                    .unwrap()
                    .into();
                (settings_dir, LocalSettingsKind::Tasks)
            } else if path.ends_with(local_vscode_tasks_file_relative_path()) {
                let settings_dir = path
                    .ancestors()
                    .nth(
                        local_vscode_tasks_file_relative_path()
                            .components()
                            .count()
                            .saturating_sub(1),
                    )
                    .unwrap()
                    .into();
                (settings_dir, LocalSettingsKind::Tasks)
            } else if path.ends_with(local_debug_file_relative_path()) {
                let settings_dir = path
                    .ancestors()
                    .nth(
                        local_debug_file_relative_path()
                            .components()
                            .count()
                            .saturating_sub(1),
                    )
                    .unwrap()
                    .into();
                (settings_dir, LocalSettingsKind::Debug)
            } else if path.ends_with(local_vscode_launch_file_relative_path()) {
                let settings_dir = path
                    .ancestors()
                    .nth(
                        local_vscode_tasks_file_relative_path()
                            .components()
                            .count()
                            .saturating_sub(1),
                    )
                    .unwrap()
                    .into();
                (settings_dir, LocalSettingsKind::Debug)
            } else if path.ends_with(RelPath::unix(EDITORCONFIG_NAME).unwrap()) {
                let Some(settings_dir) = path.parent().map(Arc::from) else {
                    continue;
                };
                if matches!(change, PathChange::Loaded) || matches!(change, PathChange::Added) {
                    let worktree_id = worktree.read(cx).id();
                    let worktree_path = worktree.read(cx).abs_path();
                    let fs = fs.clone();
                    cx.update_global::<SettingsStore, _>(|store, cx| {
                        store
                            .editorconfig_store
                            .update(cx, |editorconfig_store, cx| {
                                editorconfig_store.discover_local_external_configs_chain(
                                    worktree_id,
                                    worktree_path,
                                    fs,
                                    cx,
                                );
                            });
                    });
                }
                (settings_dir, LocalSettingsKind::Editorconfig)
            } else {
                continue;
            };

            let removed = change == &PathChange::Removed;
            let fs = fs.clone();
            let abs_path = worktree.read(cx).absolutize(path);
            settings_contents.push(async move {
                (
                    settings_dir,
                    kind,
                    if removed {
                        None
                    } else {
                        Some(
                            async move {
                                let content = fs.load(&abs_path).await?;
                                if abs_path.ends_with(local_vscode_tasks_file_relative_path().as_std_path()) {
                                    let vscode_tasks =
                                        parse_json_with_comments::<VsCodeTaskFile>(&content)
                                            .with_context(|| {
                                                format!("parsing VSCode tasks, file {abs_path:?}")
                                            })?;
                                    let zed_tasks = TaskTemplates::try_from(vscode_tasks)
                                        .with_context(|| {
                                            format!(
                                        "converting VSCode tasks into Zed ones, file {abs_path:?}"
                                    )
                                        })?;
                                    serde_json::to_string(&zed_tasks).with_context(|| {
                                        format!(
                                            "serializing Zed tasks into JSON, file {abs_path:?}"
                                        )
                                    })
                                } else if abs_path.ends_with(local_vscode_launch_file_relative_path().as_std_path()) {
                                    let vscode_tasks =
                                        parse_json_with_comments::<VsCodeDebugTaskFile>(&content)
                                            .with_context(|| {
                                                format!("parsing VSCode debug tasks, file {abs_path:?}")
                                            })?;
                                    let zed_tasks = DebugTaskFile::try_from(vscode_tasks)
                                        .with_context(|| {
                                            format!(
                                        "converting VSCode debug tasks into Zed ones, file {abs_path:?}"
                                    )
                                        })?;
                                    serde_json::to_string(&zed_tasks).with_context(|| {
                                        format!(
                                            "serializing Zed tasks into JSON, file {abs_path:?}"
                                        )
                                    })
                                } else {
                                    Ok(content)
                                }
                            }
                            .await,
                        )
                    },
                )
            });
        }

        if settings_contents.is_empty() {
            return;
        }

        let worktree = worktree.clone();
        cx.spawn(async move |this, cx| {
            let settings_contents: Vec<(Arc<RelPath>, _, _)> =
                futures::future::join_all(settings_contents).await;
            cx.update(|cx| {
                this.update(cx, |this, cx| {
                    this.update_settings(
                        worktree,
                        settings_contents.into_iter().map(|(path, kind, content)| {
                            (
                                LocalSettingsPath::InWorktree(path),
                                kind,
                                content.and_then(|c| c.log_err()),
                            )
                        }),
                        false,
                        cx,
                    )
                })
            })
        })
        .detach();
    }

    fn update_settings(
        &mut self,
        worktree: Entity<Worktree>,
        settings_contents: impl IntoIterator<
            Item = (LocalSettingsPath, LocalSettingsKind, Option<String>),
        >,
        is_via_collab: bool,
        cx: &mut Context<Self>,
    ) {
        let worktree_id = worktree.read(cx).id();
        let remote_worktree_id = worktree.read(cx).id();
        let task_store = self.task_store.clone();
        let can_trust_worktree = if is_via_collab {
            OnceCell::from(true)
        } else {
            OnceCell::new()
        };
        for (directory_path, kind, file_content) in settings_contents {
            let mut applied = true;
            match (&directory_path, kind) {
                (LocalSettingsPath::InWorktree(directory), LocalSettingsKind::Settings) => {
                    if *can_trust_worktree.get_or_init(|| {
                        if let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) {
                            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                                trusted_worktrees.can_trust(&self.worktree_store, worktree_id, cx)
                            })
                        } else {
                            true
                        }
                    }) {
                        apply_local_settings(
                            worktree_id,
                            LocalSettingsPath::InWorktree(directory.clone()),
                            kind,
                            &file_content,
                            cx,
                        )
                    } else {
                        applied = false;
                        self.pending_local_settings
                            .entry(PathTrust::Worktree(worktree_id))
                            .or_default()
                            .insert((worktree_id, directory.clone()), file_content.clone());
                    }
                }
                (LocalSettingsPath::InWorktree(directory), LocalSettingsKind::Tasks) => {
                    let result = task_store.update(cx, |task_store, cx| {
                        task_store.update_user_tasks(
                            TaskSettingsLocation::Worktree(SettingsLocation {
                                worktree_id,
                                path: directory.as_ref(),
                            }),
                            file_content.as_deref(),
                            cx,
                        )
                    });

                    match result {
                        Err(InvalidSettingsError::Tasks { path, message }) => {
                            log::error!("Failed to set local tasks in {path:?}: {message:?}");
                            cx.emit(SettingsObserverEvent::LocalTasksUpdated(Err(
                                InvalidSettingsError::Tasks { path, message },
                            )));
                        }
                        Err(e) => {
                            log::error!("Failed to set local tasks: {e}");
                        }
                        Ok(()) => {
                            cx.emit(SettingsObserverEvent::LocalTasksUpdated(Ok(directory
                                .as_std_path()
                                .join(task_file_name()))));
                        }
                    }
                }
                (LocalSettingsPath::InWorktree(directory), LocalSettingsKind::Debug) => {
                    let result = task_store.update(cx, |task_store, cx| {
                        task_store.update_user_debug_scenarios(
                            TaskSettingsLocation::Worktree(SettingsLocation {
                                worktree_id,
                                path: directory.as_ref(),
                            }),
                            file_content.as_deref(),
                            cx,
                        )
                    });

                    match result {
                        Err(InvalidSettingsError::Debug { path, message }) => {
                            log::error!(
                                "Failed to set local debug scenarios in {path:?}: {message:?}"
                            );
                            cx.emit(SettingsObserverEvent::LocalTasksUpdated(Err(
                                InvalidSettingsError::Debug { path, message },
                            )));
                        }
                        Err(e) => {
                            log::error!("Failed to set local tasks: {e}");
                        }
                        Ok(()) => {
                            cx.emit(SettingsObserverEvent::LocalTasksUpdated(Ok(directory
                                .as_std_path()
                                .join(task_file_name()))));
                        }
                    }
                }
                (directory, LocalSettingsKind::Editorconfig) => {
                    apply_local_settings(worktree_id, directory.clone(), kind, &file_content, cx);
                }
                (LocalSettingsPath::OutsideWorktree(path), kind) => {
                    log::error!(
                        "OutsideWorktree path {:?} with kind {:?} is only supported by editorconfig",
                        path,
                        kind
                    );
                    continue;
                }
            };

            if applied {
                if let Some(downstream_client) = &self.downstream_client {
                    downstream_client
                        .send(proto::UpdateWorktreeSettings {
                            project_id: self.project_id,
                            worktree_id: remote_worktree_id.to_proto(),
                            path: directory_path.to_proto(),
                            content: file_content.clone(),
                            kind: Some(local_settings_kind_to_proto(kind).into()),
                            outside_worktree: Some(directory_path.is_outside_worktree()),
                        })
                        .log_err();
                }
            }
        }
    }

    fn subscribe_to_global_task_file_changes(
        fs: Arc<dyn Fs>,
        file_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let (mut user_tasks_file_rx, watcher_task) =
            watch_config_file(cx.background_executor(), fs, file_path.clone());
        let user_tasks_content = cx.foreground_executor().block_on(user_tasks_file_rx.next());
        let weak_entry = cx.weak_entity();
        cx.spawn(async move |settings_observer, cx| {
            let _watcher_task = watcher_task;
            let Ok(task_store) = settings_observer.read_with(cx, |settings_observer, _| {
                settings_observer.task_store.clone()
            }) else {
                return;
            };
            if let Some(user_tasks_content) = user_tasks_content {
                task_store.update(cx, |task_store, cx| {
                    task_store
                        .update_user_tasks(
                            TaskSettingsLocation::Global(&file_path),
                            Some(&user_tasks_content),
                            cx,
                        )
                        .log_err();
                });
            }
            while let Some(user_tasks_content) = user_tasks_file_rx.next().await {
                let result = task_store.update(cx, |task_store, cx| {
                    task_store.update_user_tasks(
                        TaskSettingsLocation::Global(&file_path),
                        Some(&user_tasks_content),
                        cx,
                    )
                });

                weak_entry
                    .update(cx, |_, cx| match result {
                        Ok(()) => cx.emit(SettingsObserverEvent::LocalTasksUpdated(Ok(
                            file_path.clone()
                        ))),
                        Err(err) => cx.emit(SettingsObserverEvent::LocalTasksUpdated(Err(
                            InvalidSettingsError::Tasks {
                                path: file_path.clone(),
                                message: err.to_string(),
                            },
                        ))),
                    })
                    .ok();
            }
        })
    }
    fn subscribe_to_global_debug_scenarios_changes(
        fs: Arc<dyn Fs>,
        file_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let (mut user_tasks_file_rx, watcher_task) =
            watch_config_file(cx.background_executor(), fs, file_path.clone());
        let user_tasks_content = cx.foreground_executor().block_on(user_tasks_file_rx.next());
        let weak_entry = cx.weak_entity();
        cx.spawn(async move |settings_observer, cx| {
            let _watcher_task = watcher_task;
            let Ok(task_store) = settings_observer.read_with(cx, |settings_observer, _| {
                settings_observer.task_store.clone()
            }) else {
                return;
            };
            if let Some(user_tasks_content) = user_tasks_content {
                task_store.update(cx, |task_store, cx| {
                    task_store
                        .update_user_debug_scenarios(
                            TaskSettingsLocation::Global(&file_path),
                            Some(&user_tasks_content),
                            cx,
                        )
                        .log_err();
                });
            }
            while let Some(user_tasks_content) = user_tasks_file_rx.next().await {
                let result = task_store.update(cx, |task_store, cx| {
                    task_store.update_user_debug_scenarios(
                        TaskSettingsLocation::Global(&file_path),
                        Some(&user_tasks_content),
                        cx,
                    )
                });

                weak_entry
                    .update(cx, |_, cx| match result {
                        Ok(()) => cx.emit(SettingsObserverEvent::LocalDebugScenariosUpdated(Ok(
                            file_path.clone(),
                        ))),
                        Err(err) => cx.emit(SettingsObserverEvent::LocalDebugScenariosUpdated(
                            Err(InvalidSettingsError::Tasks {
                                path: file_path.clone(),
                                message: err.to_string(),
                            }),
                        )),
                    })
                    .ok();
            }
        })
    }
}

fn apply_local_settings(
    worktree_id: WorktreeId,
    path: LocalSettingsPath,
    kind: LocalSettingsKind,
    file_content: &Option<String>,
    cx: &mut Context<'_, SettingsObserver>,
) {
    cx.update_global::<SettingsStore, _>(|store, cx| {
        let result =
            store.set_local_settings(worktree_id, path.clone(), kind, file_content.as_deref(), cx);

        match result {
            Err(InvalidSettingsError::LocalSettings { path, message }) => {
                log::error!("Failed to set local settings in {path:?}: {message}");
                cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Err(
                    InvalidSettingsError::LocalSettings { path, message },
                )));
            }
            Err(e) => log::error!("Failed to set local settings: {e}"),
            Ok(()) => {
                let settings_path = match &path {
                    LocalSettingsPath::InWorktree(rel_path) => rel_path
                        .as_std_path()
                        .join(local_settings_file_relative_path().as_std_path()),
                    LocalSettingsPath::OutsideWorktree(abs_path) => abs_path.to_path_buf(),
                };
                cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Ok(
                    settings_path,
                )))
            }
        }
    })
}

pub fn local_settings_kind_from_proto(kind: proto::LocalSettingsKind) -> LocalSettingsKind {
    match kind {
        proto::LocalSettingsKind::Settings => LocalSettingsKind::Settings,
        proto::LocalSettingsKind::Tasks => LocalSettingsKind::Tasks,
        proto::LocalSettingsKind::Editorconfig => LocalSettingsKind::Editorconfig,
        proto::LocalSettingsKind::Debug => LocalSettingsKind::Debug,
    }
}

pub fn local_settings_kind_to_proto(kind: LocalSettingsKind) -> proto::LocalSettingsKind {
    match kind {
        LocalSettingsKind::Settings => proto::LocalSettingsKind::Settings,
        LocalSettingsKind::Tasks => proto::LocalSettingsKind::Tasks,
        LocalSettingsKind::Editorconfig => proto::LocalSettingsKind::Editorconfig,
        LocalSettingsKind::Debug => proto::LocalSettingsKind::Debug,
    }
}

#[derive(Debug, Clone)]
pub struct DapSettings {
    pub binary: DapBinary,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

impl From<DapSettingsContent> for DapSettings {
    fn from(content: DapSettingsContent) -> Self {
        DapSettings {
            binary: content
                .binary
                .map_or_else(|| DapBinary::Default, |binary| DapBinary::Custom(binary)),
            args: content.args.unwrap_or_default(),
            env: content.env.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DapBinary {
    Default,
    Custom(String),
}
