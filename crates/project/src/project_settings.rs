use anyhow::Context as _;
use collections::HashMap;
use fs::Fs;
use gpui::{App, AsyncApp, BorrowAppContext, Context, Entity, EventEmitter};
use lsp::LanguageServerName;
use paths::{
    local_settings_file_relative_path, local_tasks_file_relative_path,
    local_vscode_tasks_file_relative_path, EDITORCONFIG_NAME,
};
use rpc::{
    proto::{self, FromProto, ToProto},
    AnyProtoClient, TypedEnvelope,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    parse_json_with_comments, InvalidSettingsError, LocalSettingsKind, Settings, SettingsLocation,
    SettingsSources, SettingsStore,
};
use std::{path::Path, sync::Arc, time::Duration};
use task::{TaskTemplates, VsCodeTaskFile};
use util::ResultExt;
use worktree::{PathChange, UpdatedEntriesSet, Worktree, WorktreeId};

use crate::{
    task_store::TaskStore,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
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
pub struct NodeBinarySettings {
    /// The path to the node binary
    pub path: Option<String>,
    ///  The path to the npm binary Zed should use (defaults to .path/../npm)
    pub npm_path: Option<String>,
    /// If disabled, zed will download its own copy of node.
    #[serde(default)]
    pub ignore_system_version: Option<bool>,
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
            Some(InlineBlameSettings {
                delay_ms: Some(delay_ms),
                ..
            }) if delay_ms > 0 => Some(Duration::from_millis(delay_ms)),
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
pub enum GitGutterSetting {
    /// Show git gutter in tracked files.
    #[default]
    TrackedFiles,
    /// Hide git gutter
    Hide,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InlineBlameSettings {
    /// Whether or not to show git blame data inline in
    /// the currently focused line.
    ///
    /// Default: true
    #[serde(default = "true_value")]
    pub enabled: bool,
    /// Whether to only show the inline blame information
    /// after a delay once the cursor stops moving.
    ///
    /// Default: 0
    pub delay_ms: Option<u64>,
    /// The minimum column number to show the inline blame information at
    ///
    /// Default: 0
    pub min_column: Option<u32>,
    /// Whether to show commit summary as part of the inline blame.
    ///
    /// Default: false
    #[serde(default = "false_value")]
    pub show_commit_summary: bool,
}

const fn true_value() -> bool {
    true
}

const fn false_value() -> bool {
    false
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct BinarySettings {
    pub path: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub ignore_system_version: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    pub binary: Option<BinarySettings>,
    pub initialization_options: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
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
}

pub enum SettingsObserverMode {
    Local(Arc<dyn Fs>),
    Remote,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SettingsObserverEvent {
    LocalSettingsUpdated(Result<(), InvalidSettingsError>),
}

impl EventEmitter<SettingsObserverEvent> for SettingsObserver {}

pub struct SettingsObserver {
    mode: SettingsObserverMode,
    downstream_client: Option<AnyProtoClient>,
    worktree_store: Entity<WorktreeStore>,
    project_id: u64,
    task_store: Entity<TaskStore>,
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
            mode: SettingsObserverMode::Local(fs),
            downstream_client: None,
            project_id: 0,
        }
    }

    pub fn new_remote(
        worktree_store: Entity<WorktreeStore>,
        task_store: Entity<TaskStore>,
        _: &mut Context<Self>,
    ) -> Self {
        Self {
            worktree_store,
            task_store,
            mode: SettingsObserverMode::Remote,
            downstream_client: None,
            project_id: 0,
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
        cx.spawn(move |this, cx| async move {
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
                                log::error!(
                                    "Failed to set local settings in {:?}: {:?}",
                                    path,
                                    message
                                );
                                cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Err(
                                    InvalidSettingsError::LocalSettings { path, message },
                                )));
                            }
                            Err(e) => {
                                log::error!("Failed to set local settings: {e}");
                            }
                            Ok(_) => {
                                cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Ok(())));
                            }
                        }
                    }),
                LocalSettingsKind::Tasks => task_store.update(cx, |task_store, cx| {
                    task_store
                        .update_user_tasks(
                            Some(SettingsLocation {
                                worktree_id,
                                path: directory.as_ref(),
                            }),
                            file_content.as_deref(),
                            cx,
                        )
                        .log_err();
                }),
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
}

pub fn local_settings_kind_from_proto(kind: proto::LocalSettingsKind) -> LocalSettingsKind {
    match kind {
        proto::LocalSettingsKind::Settings => LocalSettingsKind::Settings,
        proto::LocalSettingsKind::Tasks => LocalSettingsKind::Tasks,
        proto::LocalSettingsKind::Editorconfig => LocalSettingsKind::Editorconfig,
    }
}

pub fn local_settings_kind_to_proto(kind: LocalSettingsKind) -> proto::LocalSettingsKind {
    match kind {
        LocalSettingsKind::Settings => proto::LocalSettingsKind::Settings,
        LocalSettingsKind::Tasks => proto::LocalSettingsKind::Tasks,
        LocalSettingsKind::Editorconfig => proto::LocalSettingsKind::Editorconfig,
    }
}
