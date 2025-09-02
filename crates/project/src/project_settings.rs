use anyhow::Context as _;
use collections::HashMap;
use context_server::ContextServerCommand;
use dap::adapters::DebugAdapterName;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AsyncApp, BorrowAppContext, Context, Entity, EventEmitter, Task};
use lsp::LanguageServerName;
use paths::{
    EDITORCONFIG_NAME, local_debug_file_relative_path, local_settings_file_relative_path,
    local_tasks_file_relative_path, local_vscode_launch_file_relative_path,
    local_vscode_tasks_file_relative_path, task_file_name,
};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, FromProto, ToProto},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    InvalidSettingsError, LocalSettingsKind, Settings, SettingsLocation, SettingsSources,
    SettingsStore, SettingsUi, parse_json_with_comments, watch_config_file,
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use task::{DebugTaskFile, TaskTemplates, VsCodeDebugTaskFile, VsCodeTaskFile};
use util::{ResultExt, serde::default_true};
use worktree::{PathChange, UpdatedEntriesSet, Worktree, WorktreeId};

use crate::{
    task_store::{TaskSettingsLocation, TaskStore},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi)]
pub struct ProjectSettings {
    /// Configuration for language servers.
    ///
    /// The following settings can be overridden for specific language servers:
    /// - initialization_options
    ///
    /// To override settings for a language, add an entry for that language server's
    /// name to the lsp value.
    /// Default: null
    #[serde(default)]
    pub lsp: HashMap<LanguageServerName, LspSettings>,

    /// Common language server settings.
    #[serde(default)]
    pub global_lsp_settings: GlobalLspSettings,

    /// Configuration for Debugger-related features
    #[serde(default)]
    pub dap: HashMap<DebugAdapterName, DapSettings>,

    /// Settings for context servers used for AI-related features.
    #[serde(default)]
    pub context_servers: HashMap<Arc<str>, ContextServerSettings>,

    /// Configuration for Diagnostics-related features.
    #[serde(default)]
    pub diagnostics: DiagnosticsSettings,

    /// Configuration for Git-related features
    #[serde(default)]
    pub git: GitSettings,

    /// Configuration for Node-related features
    #[serde(default)]
    pub node: NodeBinarySettings,

    /// Configuration for how direnv configuration should be loaded
    #[serde(default)]
    pub load_direnv: DirenvSettings,

    /// Configuration for session-related features
    #[serde(default)]
    pub session: SessionSettings,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DapSettings {
    pub binary: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ContextServerSettings {
    Custom {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,

        #[serde(flatten)]
        command: ContextServerCommand,
    },
    Extension {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// The settings for this context server specified by the extension.
        ///
        /// Consult the documentation for the context server to see what settings
        /// are supported.
        settings: serde_json::Value,
    },
}

/// Common language server settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GlobalLspSettings {
    /// Whether to show the LSP servers button in the status bar.
    ///
    /// Default: `true`
    #[serde(default = "default_true")]
    pub button: bool,
}

impl ContextServerSettings {
    pub fn default_extension() -> Self {
        Self::Extension {
            enabled: true,
            settings: serde_json::json!({}),
        }
    }

    pub fn enabled(&self) -> bool {
        match self {
            ContextServerSettings::Custom { enabled, .. } => *enabled,
            ContextServerSettings::Extension { enabled, .. } => *enabled,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        match self {
            ContextServerSettings::Custom { enabled: e, .. } => *e = enabled,
            ContextServerSettings::Extension { enabled: e, .. } => *e = enabled,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NodeBinarySettings {
    /// The path to the Node binary.
    pub path: Option<String>,
    /// The path to the npm binary Zed should use (defaults to `.path/../npm`).
    pub npm_path: Option<String>,
    /// If enabled, Zed will download its own copy of Node.
    #[serde(default)]
    pub ignore_system_version: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DirenvSettings {
    /// Load direnv configuration through a shell hook
    ShellHook,
    /// Load direnv configuration directly using `direnv export json`
    #[default]
    Direct,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct LspPullDiagnosticsSettings {
    /// Whether to pull for diagnostics or not.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Minimum time to wait before pulling diagnostics from the language server(s).
    /// 0 turns the debounce off.
    ///
    /// Default: 50
    #[serde(default = "default_lsp_diagnostics_pull_debounce_ms")]
    pub debounce_ms: u64,
}

fn default_lsp_diagnostics_pull_debounce_ms() -> u64 {
    50
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct InlineDiagnosticsSettings {
    /// Whether or not to show inline diagnostics
    ///
    /// Default: false
    pub enabled: bool,
    /// Whether to only show the inline diagnostics after a delay after the
    /// last editor event.
    ///
    /// Default: 150
    #[serde(default = "default_inline_diagnostics_update_debounce_ms")]
    pub update_debounce_ms: u64,
    /// The amount of padding between the end of the source line and the start
    /// of the inline diagnostic in units of columns.
    ///
    /// Default: 4
    #[serde(default = "default_inline_diagnostics_padding")]
    pub padding: u32,
    /// The minimum column to display inline diagnostics. This setting can be
    /// used to horizontally align inline diagnostics at some position. Lines
    /// longer than this value will still push diagnostics further to the right.
    ///
    /// Default: 0
    pub min_column: u32,

    pub max_severity: Option<DiagnosticSeverity>,
}

fn default_inline_diagnostics_update_debounce_ms() -> u64 {
    150
}

fn default_inline_diagnostics_padding() -> u32 {
    4
}

impl Default for DiagnosticsSettings {
    fn default() -> Self {
        Self {
            button: true,
            include_warnings: true,
            lsp_pull_diagnostics: LspPullDiagnosticsSettings::default(),
            inline: InlineDiagnosticsSettings::default(),
        }
    }
}

impl Default for LspPullDiagnosticsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            debounce_ms: default_lsp_diagnostics_pull_debounce_ms(),
        }
    }
}

impl Default for InlineDiagnosticsSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            update_debounce_ms: default_inline_diagnostics_update_debounce_ms(),
            padding: default_inline_diagnostics_padding(),
            min_column: 0,
            max_severity: None,
        }
    }
}

impl Default for GlobalLspSettings {
    fn default() -> Self {
        Self {
            button: default_true(),
        }
    }
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
    SettingsUi,
)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    // No diagnostics are shown.
    Off,
    Error,
    Warning,
    Info,
    #[serde(alias = "all")]
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

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct GitSettings {
    /// Whether or not to show the git gutter.
    ///
    /// Default: tracked_files
    pub git_gutter: Option<GitGutterSetting>,
    /// Sets the debounce threshold (in milliseconds) after which changes are reflected in the git gutter.
    ///
    /// Default: null
    pub gutter_debounce: Option<u64>,
    /// Whether or not to show git blame data inline in
    /// the currently focused line.
    ///
    /// Default: on
    pub inline_blame: Option<InlineBlameSettings>,
    /// How hunks are displayed visually in the editor.
    ///
    /// Default: staged_hollow
    pub hunk_style: Option<GitHunkStyleSetting>,
}

impl GitSettings {
    pub fn inline_blame_enabled(&self) -> bool {
        #[allow(unknown_lints, clippy::manual_unwrap_or_default)]
        match self.inline_blame {
            Some(InlineBlameSettings { enabled, .. }) => enabled,
            _ => false,
        }
    }

    pub fn inline_blame_delay(&self) -> Option<Duration> {
        match self.inline_blame {
            Some(InlineBlameSettings { delay_ms, .. }) if delay_ms > 0 => {
                Some(Duration::from_millis(delay_ms))
            }
            _ => None,
        }
    }

    pub fn show_inline_commit_summary(&self) -> bool {
        match self.inline_blame {
            Some(InlineBlameSettings {
                show_commit_summary,
                ..
            }) => show_commit_summary,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitHunkStyleSetting {
    /// Show unstaged hunks with a filled background and staged hunks hollow.
    #[default]
    StagedHollow,
    /// Show unstaged hunks hollow and staged hunks with a filled background.
    UnstagedHollow,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitGutterSetting {
    /// Show git gutter in tracked files.
    #[default]
    TrackedFiles,
    /// Hide git gutter
    Hide,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InlineBlameSettings {
    /// Whether or not to show git blame data inline in
    /// the currently focused line.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Whether to only show the inline blame information
    /// after a delay once the cursor stops moving.
    ///
    /// Default: 0
    #[serde(default)]
    pub delay_ms: u64,
    /// The amount of padding between the end of the source line and the start
    /// of the inline blame in units of columns.
    ///
    /// Default: 7
    #[serde(default = "default_inline_blame_padding")]
    pub padding: u32,
    /// The minimum column number to show the inline blame information at
    ///
    /// Default: 0
    #[serde(default)]
    pub min_column: u32,
    /// Whether to show commit summary as part of the inline blame.
    ///
    /// Default: false
    #[serde(default)]
    pub show_commit_summary: bool,
}

fn default_inline_blame_padding() -> u32 {
    7
}

impl Default for InlineBlameSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            delay_ms: 0,
            padding: default_inline_blame_padding(),
            min_column: 0,
            show_commit_summary: false,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, Hash)]
pub struct BinarySettings {
    pub path: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub env: Option<BTreeMap<String, String>>,
    pub ignore_system_version: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, Hash)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    pub binary: Option<BinarySettings>,
    pub initialization_options: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    /// If the server supports sending tasks over LSP extensions,
    /// this setting can be used to enable or disable them in Zed.
    /// Default: true
    #[serde(default = "default_true")]
    pub enable_lsp_tasks: bool,
}

impl Default for LspSettings {
    fn default() -> Self {
        Self {
            binary: None,
            initialization_options: None,
            settings: None,
            enable_lsp_tasks: true,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SessionSettings {
    /// Whether or not to restore unsaved buffers on restart.
    ///
    /// If this is true, user won't be prompted whether to save/discard
    /// dirty files when closing the application.
    ///
    /// Default: true
    pub restore_unsaved_buffers: bool,
}

impl Default for SessionSettings {
    fn default() -> Self {
        Self {
            restore_unsaved_buffers: true,
        }
    }
}

impl Settings for ProjectSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        // this just sets the binary name instead of a full path so it relies on path lookup
        // resolving to the one you want
        vscode.enum_setting(
            "npm.packageManager",
            &mut current.node.npm_path,
            |s| match s {
                v @ ("npm" | "yarn" | "bun" | "pnpm") => Some(v.to_owned()),
                _ => None,
            },
        );

        if let Some(b) = vscode.read_bool("git.blame.editorDecoration.enabled") {
            if let Some(blame) = current.git.inline_blame.as_mut() {
                blame.enabled = b
            } else {
                current.git.inline_blame = Some(InlineBlameSettings {
                    enabled: b,
                    ..Default::default()
                })
            }
        }

        #[derive(Deserialize)]
        struct VsCodeContextServerCommand {
            command: PathBuf,
            args: Option<Vec<String>>,
            env: Option<HashMap<String, String>>,
            // note: we don't support envFile and type
        }
        impl From<VsCodeContextServerCommand> for ContextServerCommand {
            fn from(cmd: VsCodeContextServerCommand) -> Self {
                Self {
                    path: cmd.command,
                    args: cmd.args.unwrap_or_default(),
                    env: cmd.env,
                }
            }
        }
        if let Some(mcp) = vscode.read_value("mcp").and_then(|v| v.as_object()) {
            current
                .context_servers
                .extend(mcp.iter().filter_map(|(k, v)| {
                    Some((
                        k.clone().into(),
                        ContextServerSettings::Custom {
                            enabled: true,
                            command: serde_json::from_value::<VsCodeContextServerCommand>(
                                v.clone(),
                            )
                            .ok()?
                            .into(),
                        },
                    ))
                }));
        }

        // TODO: translate lsp settings for rust-analyzer and other popular ones to old.lsp
    }
}

pub enum SettingsObserverMode {
    Local(Arc<dyn Fs>),
    Remote,
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
    }

    pub fn new_local(
        fs: Arc<dyn Fs>,
        worktree_store: Entity<WorktreeStore>,
        task_store: Entity<TaskStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();

        Self {
            worktree_store,
            task_store,
            mode: SettingsObserverMode::Local(fs.clone()),
            downstream_client: None,
            project_id: 0,
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

    pub fn new_remote(
        fs: Arc<dyn Fs>,
        worktree_store: Entity<WorktreeStore>,
        task_store: Entity<TaskStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            worktree_store,
            task_store,
            mode: SettingsObserverMode::Remote,
            downstream_client: None,
            project_id: 0,
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
                downstream_client
                    .send(proto::UpdateWorktreeSettings {
                        project_id,
                        worktree_id,
                        path: path.to_proto(),
                        content: Some(content),
                        kind: Some(
                            local_settings_kind_to_proto(LocalSettingsKind::Settings).into(),
                        ),
                    })
                    .log_err();
            }
            for (path, content, _) in store.local_editorconfig_settings(worktree.read(cx).id()) {
                downstream_client
                    .send(proto::UpdateWorktreeSettings {
                        project_id,
                        worktree_id,
                        path: path.to_proto(),
                        content: Some(content),
                        kind: Some(
                            local_settings_kind_to_proto(LocalSettingsKind::Editorconfig).into(),
                        ),
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
        this.update(&mut cx, |this, cx| {
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
                    Arc::<Path>::from_proto(envelope.payload.path.clone()),
                    local_settings_kind_from_proto(kind),
                    envelope.payload.content,
                )],
                cx,
            );
        })?;
        Ok(())
    }

    fn on_worktree_store_event(
        &mut self,
        _: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        if let WorktreeStoreEvent::WorktreeAdded(worktree) = event {
            cx.subscribe(worktree, |this, worktree, event, cx| {
                if let worktree::Event::UpdatedEntries(changes) = event {
                    this.update_local_worktree_settings(&worktree, changes, cx)
                }
            })
            .detach()
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
                let settings_dir = Arc::<Path>::from(
                    path.ancestors()
                        .nth(local_settings_file_relative_path().components().count())
                        .unwrap(),
                );
                (settings_dir, LocalSettingsKind::Settings)
            } else if path.ends_with(local_tasks_file_relative_path()) {
                let settings_dir = Arc::<Path>::from(
                    path.ancestors()
                        .nth(
                            local_tasks_file_relative_path()
                                .components()
                                .count()
                                .saturating_sub(1),
                        )
                        .unwrap(),
                );
                (settings_dir, LocalSettingsKind::Tasks)
            } else if path.ends_with(local_vscode_tasks_file_relative_path()) {
                let settings_dir = Arc::<Path>::from(
                    path.ancestors()
                        .nth(
                            local_vscode_tasks_file_relative_path()
                                .components()
                                .count()
                                .saturating_sub(1),
                        )
                        .unwrap(),
                );
                (settings_dir, LocalSettingsKind::Tasks)
            } else if path.ends_with(local_debug_file_relative_path()) {
                let settings_dir = Arc::<Path>::from(
                    path.ancestors()
                        .nth(
                            local_debug_file_relative_path()
                                .components()
                                .count()
                                .saturating_sub(1),
                        )
                        .unwrap(),
                );
                (settings_dir, LocalSettingsKind::Debug)
            } else if path.ends_with(local_vscode_launch_file_relative_path()) {
                let settings_dir = Arc::<Path>::from(
                    path.ancestors()
                        .nth(
                            local_vscode_tasks_file_relative_path()
                                .components()
                                .count()
                                .saturating_sub(1),
                        )
                        .unwrap(),
                );
                (settings_dir, LocalSettingsKind::Debug)
            } else if path.ends_with(EDITORCONFIG_NAME) {
                let Some(settings_dir) = path.parent().map(Arc::from) else {
                    continue;
                };
                (settings_dir, LocalSettingsKind::Editorconfig)
            } else {
                continue;
            };

            let removed = change == &PathChange::Removed;
            let fs = fs.clone();
            let abs_path = match worktree.read(cx).absolutize(path) {
                Ok(abs_path) => abs_path,
                Err(e) => {
                    log::warn!("Cannot absolutize {path:?} received as {change:?} FS change: {e}");
                    continue;
                }
            };
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
                                if abs_path.ends_with(local_vscode_tasks_file_relative_path()) {
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
                                } else if abs_path.ends_with(local_vscode_launch_file_relative_path()) {
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
            let settings_contents: Vec<(Arc<Path>, _, _)> =
                futures::future::join_all(settings_contents).await;
            cx.update(|cx| {
                this.update(cx, |this, cx| {
                    this.update_settings(
                        worktree,
                        settings_contents.into_iter().map(|(path, kind, content)| {
                            (path, kind, content.and_then(|c| c.log_err()))
                        }),
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
        settings_contents: impl IntoIterator<Item = (Arc<Path>, LocalSettingsKind, Option<String>)>,
        cx: &mut Context<Self>,
    ) {
        let worktree_id = worktree.read(cx).id();
        let remote_worktree_id = worktree.read(cx).id();
        let task_store = self.task_store.clone();

        for (directory, kind, file_content) in settings_contents {
            match kind {
                LocalSettingsKind::Settings | LocalSettingsKind::Editorconfig => cx
                    .update_global::<SettingsStore, _>(|store, cx| {
                        let result = store.set_local_settings(
                            worktree_id,
                            directory.clone(),
                            kind,
                            file_content.as_deref(),
                            cx,
                        );

                        match result {
                            Err(InvalidSettingsError::LocalSettings { path, message }) => {
                                log::error!("Failed to set local settings in {path:?}: {message}");
                                cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Err(
                                    InvalidSettingsError::LocalSettings { path, message },
                                )));
                            }
                            Err(e) => {
                                log::error!("Failed to set local settings: {e}");
                            }
                            Ok(()) => {
                                cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Ok(
                                    directory.join(local_settings_file_relative_path())
                                )));
                            }
                        }
                    }),
                LocalSettingsKind::Tasks => {
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
                            cx.emit(SettingsObserverEvent::LocalTasksUpdated(Ok(
                                directory.join(task_file_name())
                            )));
                        }
                    }
                }
                LocalSettingsKind::Debug => {
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
                            cx.emit(SettingsObserverEvent::LocalTasksUpdated(Ok(
                                directory.join(task_file_name())
                            )));
                        }
                    }
                }
            };

            if let Some(downstream_client) = &self.downstream_client {
                downstream_client
                    .send(proto::UpdateWorktreeSettings {
                        project_id: self.project_id,
                        worktree_id: remote_worktree_id.to_proto(),
                        path: directory.to_proto(),
                        content: file_content,
                        kind: Some(local_settings_kind_to_proto(kind).into()),
                    })
                    .log_err();
            }
        }
    }

    fn subscribe_to_global_task_file_changes(
        fs: Arc<dyn Fs>,
        file_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let mut user_tasks_file_rx =
            watch_config_file(cx.background_executor(), fs, file_path.clone());
        let user_tasks_content = cx.background_executor().block(user_tasks_file_rx.next());
        let weak_entry = cx.weak_entity();
        cx.spawn(async move |settings_observer, cx| {
            let Ok(task_store) = settings_observer.read_with(cx, |settings_observer, _| {
                settings_observer.task_store.clone()
            }) else {
                return;
            };
            if let Some(user_tasks_content) = user_tasks_content {
                let Ok(()) = task_store.update(cx, |task_store, cx| {
                    task_store
                        .update_user_tasks(
                            TaskSettingsLocation::Global(&file_path),
                            Some(&user_tasks_content),
                            cx,
                        )
                        .log_err();
                }) else {
                    return;
                };
            }
            while let Some(user_tasks_content) = user_tasks_file_rx.next().await {
                let Ok(result) = task_store.update(cx, |task_store, cx| {
                    task_store.update_user_tasks(
                        TaskSettingsLocation::Global(&file_path),
                        Some(&user_tasks_content),
                        cx,
                    )
                }) else {
                    break;
                };

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
        let mut user_tasks_file_rx =
            watch_config_file(cx.background_executor(), fs, file_path.clone());
        let user_tasks_content = cx.background_executor().block(user_tasks_file_rx.next());
        let weak_entry = cx.weak_entity();
        cx.spawn(async move |settings_observer, cx| {
            let Ok(task_store) = settings_observer.read_with(cx, |settings_observer, _| {
                settings_observer.task_store.clone()
            }) else {
                return;
            };
            if let Some(user_tasks_content) = user_tasks_content {
                let Ok(()) = task_store.update(cx, |task_store, cx| {
                    task_store
                        .update_user_debug_scenarios(
                            TaskSettingsLocation::Global(&file_path),
                            Some(&user_tasks_content),
                            cx,
                        )
                        .log_err();
                }) else {
                    return;
                };
            }
            while let Some(user_tasks_content) = user_tasks_file_rx.next().await {
                let Ok(result) = task_store.update(cx, |task_store, cx| {
                    task_store.update_user_debug_scenarios(
                        TaskSettingsLocation::Global(&file_path),
                        Some(&user_tasks_content),
                        cx,
                    )
                }) else {
                    break;
                };

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
