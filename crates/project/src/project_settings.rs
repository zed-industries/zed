use anyhow::Context;
use collections::HashMap;
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, BorrowAppContext, EventEmitter, Model, ModelContext};
use language::LanguageServerName;
use paths::local_settings_file_relative_path;
use rpc::{proto, AnyProtoClient, TypedEnvelope};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{InvalidSettingsError, LocalSettingsKind, Settings, SettingsSources, SettingsStore};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use util::ResultExt;
use worktree::{PathChange, UpdatedEntriesSet, Worktree, WorktreeId};

use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};

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
}

const fn true_value() -> bool {
    true
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

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}

pub enum SettingsObserverMode {
    Local(Arc<dyn Fs>),
    Ssh(AnyProtoClient),
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
    worktree_store: Model<WorktreeStore>,
    project_id: u64,
}

/// SettingsObserver observers changes to .zed/settings.json files in local worktrees
/// (or the equivalent protobuf messages from upstream) and updates local settings
/// and sends notifications downstream.
/// In ssh mode it also monitors ~/.config/zed/settings.json and sends the content
/// upstream.
impl SettingsObserver {
    pub fn init(client: &AnyProtoClient) {
        client.add_model_message_handler(Self::handle_update_worktree_settings);
        client.add_model_message_handler(Self::handle_update_user_settings)
    }

    pub fn new_local(
        fs: Arc<dyn Fs>,
        worktree_store: Model<WorktreeStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();

        Self {
            worktree_store,
            mode: SettingsObserverMode::Local(fs),
            downstream_client: None,
            project_id: 0,
        }
    }

    pub fn new_ssh(
        client: AnyProtoClient,
        worktree_store: Model<WorktreeStore>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let this = Self {
            worktree_store,
            mode: SettingsObserverMode::Ssh(client.clone()),
            downstream_client: None,
            project_id: 0,
        };
        this.maintain_ssh_settings(client, cx);
        this
    }

    pub fn new_remote(worktree_store: Model<WorktreeStore>, _: &mut ModelContext<Self>) -> Self {
        Self {
            worktree_store,
            mode: SettingsObserverMode::Remote,
            downstream_client: None,
            project_id: 0,
        }
    }

    pub fn shared(
        &mut self,
        project_id: u64,
        downstream_client: AnyProtoClient,
        cx: &mut ModelContext<Self>,
    ) {
        self.project_id = project_id;
        self.downstream_client = Some(downstream_client.clone());

        let store = cx.global::<SettingsStore>();
        for worktree in self.worktree_store.read(cx).worktrees() {
            let worktree_id = worktree.read(cx).id().to_proto();
            for (path, kind, content) in store.local_settings(worktree.read(cx).id()) {
                downstream_client
                    .send(proto::UpdateWorktreeSettings {
                        project_id,
                        worktree_id,
                        path: path.to_string_lossy().into(),
                        content: Some(content),
                        kind: Some(local_settings_kind_to_proto(kind).into()),
                    })
                    .log_err();
            }
        }
    }

    pub fn unshared(&mut self, _: &mut ModelContext<Self>) {
        self.downstream_client = None;
    }

    async fn handle_update_worktree_settings(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateWorktreeSettings>,
        mut cx: AsyncAppContext,
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
                    PathBuf::from(&envelope.payload.path).into(),
                    local_settings_kind_from_proto(kind),
                    envelope.payload.content,
                )],
                cx,
            );
        })?;
        Ok(())
    }

    pub async fn handle_update_user_settings(
        _: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateUserSettings>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<()> {
        cx.update_global(move |settings_store: &mut SettingsStore, cx| {
            settings_store.set_user_settings(&envelope.payload.content, cx)
        })??;

        Ok(())
    }

    pub fn maintain_ssh_settings(&self, ssh: AnyProtoClient, cx: &mut ModelContext<Self>) {
        let mut settings = cx.global::<SettingsStore>().raw_user_settings().clone();
        if let Some(content) = serde_json::to_string(&settings).log_err() {
            ssh.send(proto::UpdateUserSettings {
                project_id: 0,
                content,
                kind: Some(proto::LocalSettingsKind::Settings.into()),
            })
            .log_err();
        }

        let weak_client = ssh.downgrade();
        cx.observe_global::<SettingsStore>(move |_, cx| {
            let new_settings = cx.global::<SettingsStore>().raw_user_settings();
            if &settings != new_settings {
                settings = new_settings.clone()
            }
            if let Some(content) = serde_json::to_string(&settings).log_err() {
                if let Some(ssh) = weak_client.upgrade() {
                    ssh.send(proto::UpdateUserSettings {
                        project_id: 0,
                        content,
                        kind: Some(proto::LocalSettingsKind::Settings.into()),
                    })
                    .log_err();
                }
            }
        })
        .detach();
    }

    fn on_worktree_store_event(
        &mut self,
        _: Model<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut ModelContext<Self>,
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
        worktree: &Model<Worktree>,
        changes: &UpdatedEntriesSet,
        cx: &mut ModelContext<Self>,
    ) {
        let SettingsObserverMode::Local(fs) = &self.mode else {
            return;
        };

        let mut settings_contents = Vec::new();
        for (path, _, change) in changes.iter() {
            let removed = change == &PathChange::Removed;
            let abs_path = match worktree.read(cx).absolutize(path) {
                Ok(abs_path) => abs_path,
                Err(e) => {
                    log::warn!("Cannot absolutize {path:?} received as {change:?} FS change: {e}");
                    continue;
                }
            };

            if path.ends_with(local_settings_file_relative_path()) {
                let settings_dir = Arc::from(
                    path.ancestors()
                        .nth(local_settings_file_relative_path().components().count())
                        .unwrap(),
                );
                let fs = fs.clone();
                settings_contents.push(async move {
                    (
                        settings_dir,
                        LocalSettingsKind::Settings,
                        if removed {
                            None
                        } else {
                            Some(async move { fs.load(&abs_path).await }.await)
                        },
                    )
                });
            }
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
        worktree: Model<Worktree>,
        settings_contents: impl IntoIterator<Item = (Arc<Path>, LocalSettingsKind, Option<String>)>,
        cx: &mut ModelContext<Self>,
    ) {
        let worktree_id = worktree.read(cx).id();
        let remote_worktree_id = worktree.read(cx).id();

        let result = cx.update_global::<SettingsStore, anyhow::Result<()>>(|store, cx| {
            for (directory, kind, file_content) in settings_contents {
                store.set_local_settings(
                    worktree_id,
                    directory.clone(),
                    kind,
                    file_content.as_deref(),
                    cx,
                )?;

                if let Some(downstream_client) = &self.downstream_client {
                    downstream_client
                        .send(proto::UpdateWorktreeSettings {
                            project_id: self.project_id,
                            worktree_id: remote_worktree_id.to_proto(),
                            path: directory.to_string_lossy().into_owned(),
                            content: file_content,
                            kind: Some(local_settings_kind_to_proto(kind).into()),
                        })
                        .log_err();
                }
            }
            anyhow::Ok(())
        });

        match result {
            Err(error) => {
                if let Ok(error) = error.downcast::<InvalidSettingsError>() {
                    if let InvalidSettingsError::LocalSettings {
                        ref path,
                        ref message,
                    } = error
                    {
                        log::error!("Failed to set local settings in {:?}: {:?}", path, message);
                        cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Err(error)));
                    }
                }
            }
            Ok(()) => {
                cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Ok(())));
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
