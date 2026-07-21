//! Invisible checkpoint history for BreadPaper vaults (spec: v2-invisible-git).
//!
//! A background service that commits snapshots of the whole vault into a
//! hidden git repository whose git-dir is `<vault>/.breadpaper/history`.
//! Because that directory is never named `.git`, Zed's worktree scanner never
//! discovers it, so it never appears in the git UI; isolation from any
//! user-managed `.git` comes from driving every command with explicit
//! `GIT_DIR`/`GIT_WORK_TREE` scoping. There is intentionally no user-facing
//! surface here: no panel, no actions, no notifications.

use anyhow::{Context as _, Result};
use chrono::Utc;
use fs::Fs;
use git::repository::{GitRepositoryCheckpoint, RealGitRepository};
use gpui::{
    App, AppContext as _, BackgroundExecutor, Context, Entity, EntityId, Global, Subscription,
    Task,
};
use project::{Project, WorktreeId};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use util::ResultExt as _;
use workspace::Workspace;

use crate::vault::{HistoryConfig, VAULT_CONFIG_FILE, VAULT_MARKER_DIR, Vault, VaultStatus};

const HISTORY_DIR: &str = "history";
const CHECKPOINTS_BRANCH: &str = "checkpoints";
const CHECKPOINTS_REF: &str = "refs/heads/checkpoints";
const AUTHOR_NAME: &str = "BreadPaper";
const AUTHOR_EMAIL: &str = "history@breadpaper.local";
/// The history repo must always ignore its own git-dir — a snapshot would
/// otherwise include its own objects and grow without bound. `.git` is listed
/// for clarity even though git never tracks a directory with that name.
const BASE_EXCLUDES: &str = "/.breadpaper/history/\n/.git/\n";
const HEARTBEAT_POLL_INTERVAL: Duration = Duration::from_secs(60);

pub fn init(cx: &mut App) {
    // The same bundled-git resolution Zed itself uses; outside bundled macOS
    // builds the service falls back to the system `git`.
    let bundled_git_binary_path =
        if cfg!(target_os = "macos") && option_env!("ZED_BUNDLE").as_deref() == Some("true") {
            cx.path_for_auxiliary_executable("git").log_err()
        } else {
            None
        };
    cx.observe_new(move |workspace: &mut Workspace, _window, cx| {
        let project = workspace.project().clone();
        if !project.read(cx).is_local() {
            return;
        }
        let service =
            cx.new(|cx| HistoryService::new(project.clone(), bundled_git_binary_path.clone(), cx));
        let project_id = project.entity_id();
        cx.default_global::<GlobalHistoryServices>()
            .0
            .insert(project_id, service);
        cx.on_release(move |_, cx| {
            cx.default_global::<GlobalHistoryServices>()
                .0
                .remove(&project_id);
        })
        .detach();
    })
    .detach();
}

/// The designed-for pre-AI-write trigger (spec §6.2): first-party code that is
/// about to write into a vault calls this immediately beforehand so a clean
/// pre-mutation restore point exists. No first-party AI write path exists yet
/// in V2, so nothing calls it in production.
pub fn checkpoint_before_ai_write(project: &Entity<Project>, cx: &mut App) {
    let service = cx
        .default_global::<GlobalHistoryServices>()
        .0
        .get(&project.entity_id())
        .cloned();
    if let Some(service) = service {
        service.update(cx, |service, cx| {
            service.checkpoint_all(CheckpointTrigger::PreAiWrite, cx)
        });
    }
}

#[derive(Default)]
struct GlobalHistoryServices(HashMap<EntityId, Entity<HistoryService>>);

impl Global for GlobalHistoryServices {}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CheckpointTrigger {
    Initial,
    Idle,
    Heartbeat,
    PreAiWrite,
    Close,
}

impl CheckpointTrigger {
    fn label(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::Idle => "idle",
            Self::Heartbeat => "heartbeat",
            Self::PreAiWrite => "pre-ai-write",
            Self::Close => "close",
        }
    }
}

enum HistoryPhase {
    Initializing,
    Ready(Arc<RealGitRepository>),
    Disabled,
}

struct VaultHistory {
    root: PathBuf,
    config: HistoryConfig,
    phase: HistoryPhase,
    /// Vault files changed since the last checkpoint attempt finished.
    dirty: bool,
    in_flight: bool,
    pending_trigger: Option<CheckpointTrigger>,
    last_checkpoint_at: Option<Instant>,
    /// Re-armed on every edit; fires the idle-debounce checkpoint. Replacing
    /// the task cancels the previous timer.
    idle_task: Option<Task<()>>,
    _tasks: Vec<Task<()>>,
}

pub struct HistoryService {
    project: Entity<Project>,
    bundled_git_binary_path: Option<PathBuf>,
    executor: BackgroundExecutor,
    vaults: HashMap<WorktreeId, VaultHistory>,
    _subscriptions: Vec<Subscription>,
}

impl HistoryService {
    fn new(
        project: Entity<Project>,
        bundled_git_binary_path: Option<PathBuf>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.subscribe(&project, Self::handle_project_event),
            cx.on_app_quit(Self::checkpoint_on_quit),
        ];
        let mut this = Self {
            project,
            bundled_git_binary_path,
            executor: cx.background_executor().clone(),
            vaults: HashMap::new(),
            _subscriptions: subscriptions,
        };
        this.sync_vaults(cx);
        this
    }

    fn handle_project_event(
        &mut self,
        _: Entity<Project>,
        event: &project::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::Event::WorktreeAdded(_) | project::Event::WorktreeRemoved(_) => {
                self.sync_vaults(cx)
            }
            project::Event::WorktreeUpdatedEntries(worktree_id, changes) => {
                if changes
                    .iter()
                    .any(|(path, _, _)| path.as_unix_str() == vault_config_rel_path())
                {
                    self.sync_vaults(cx);
                }
                // Never treat our own repo's writes as edits, or every
                // checkpoint would schedule the next one forever.
                if changes
                    .iter()
                    .any(|(path, _, _)| !is_history_repo_path(path.as_unix_str()))
                {
                    self.note_edit(*worktree_id, cx);
                }
            }
            _ => {}
        }
    }

    /// Aligns the tracked vaults with the project's current visible worktrees,
    /// starting history for new vaults and dropping ones that disappeared or
    /// were disabled. Cheap enough to run on any worktree/config change.
    fn sync_vaults(&mut self, cx: &mut Context<Self>) {
        let worktrees: Vec<(WorktreeId, PathBuf)> = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (worktree.id(), worktree.abs_path().to_path_buf())
            })
            .collect();

        self.vaults
            .retain(|worktree_id, _| worktrees.iter().any(|(id, _)| id == worktree_id));

        for (worktree_id, root) in worktrees {
            let config = match Vault::detect(&root) {
                VaultStatus::Valid(vault) if vault.config.history.enabled => vault.config.history,
                _ => {
                    self.vaults.remove(&worktree_id);
                    continue;
                }
            };
            match self.vaults.get_mut(&worktree_id) {
                Some(state) if state.root == root => state.config = config,
                _ => {
                    let state = self.start_vault(worktree_id, root, config, cx);
                    self.vaults.insert(worktree_id, state);
                }
            }
        }
    }

    fn start_vault(
        &self,
        worktree_id: WorktreeId,
        root: PathBuf,
        config: HistoryConfig,
        cx: &mut Context<Self>,
    ) -> VaultHistory {
        let git_dir = root.join(VAULT_MARKER_DIR).join(HISTORY_DIR);
        let fs = self.project.read(cx).fs().clone();
        let bundled_git_binary_path = self.bundled_git_binary_path.clone();
        let executor = self.executor.clone();

        let init_task = cx.spawn({
            let root = root.clone();
            async move |this, cx| {
                let result = cx
                    .background_spawn(open_or_init_repo(
                        fs,
                        git_dir,
                        root.clone(),
                        bundled_git_binary_path,
                        executor,
                    ))
                    .await;
                this.update(cx, |this, cx| {
                    let Some(state) = this.vaults.get_mut(&worktree_id) else {
                        return;
                    };
                    match result {
                        Ok(repository) => {
                            state.phase = HistoryPhase::Ready(repository);
                            this.try_checkpoint(worktree_id, CheckpointTrigger::Initial, cx);
                        }
                        Err(error) => {
                            log::warn!(
                                "BreadPaper history disabled for {}: {error:#}",
                                root.display()
                            );
                            state.phase = HistoryPhase::Disabled;
                        }
                    }
                })
                .ok();
            }
        });

        let heartbeat_task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(HEARTBEAT_POLL_INTERVAL).await;
                let keep_going = this.update(cx, |this, cx| {
                    let Some(state) = this.vaults.get_mut(&worktree_id) else {
                        return false;
                    };
                    let due = state.dirty
                        && !state.in_flight
                        && matches!(state.phase, HistoryPhase::Ready(_))
                        && state
                            .last_checkpoint_at
                            .is_none_or(|at| at.elapsed() >= state.config.heartbeat);
                    if due {
                        this.try_checkpoint(worktree_id, CheckpointTrigger::Heartbeat, cx);
                    }
                    true
                });
                if !keep_going.unwrap_or(false) {
                    break;
                }
            }
        });

        VaultHistory {
            root,
            config,
            phase: HistoryPhase::Initializing,
            dirty: false,
            in_flight: false,
            pending_trigger: None,
            last_checkpoint_at: None,
            idle_task: None,
            _tasks: vec![init_task, heartbeat_task],
        }
    }

    fn note_edit(&mut self, worktree_id: WorktreeId, cx: &mut Context<Self>) {
        let Some(state) = self.vaults.get_mut(&worktree_id) else {
            return;
        };
        state.dirty = true;
        let debounce = state.config.idle_debounce;
        state.idle_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(debounce).await;
            this.update(cx, |this, cx| {
                this.try_checkpoint(worktree_id, CheckpointTrigger::Idle, cx)
            })
            .ok();
        }));
    }

    fn checkpoint_all(&mut self, trigger: CheckpointTrigger, cx: &mut Context<Self>) {
        let worktree_ids: Vec<WorktreeId> = self.vaults.keys().copied().collect();
        for worktree_id in worktree_ids {
            self.try_checkpoint(worktree_id, trigger, cx);
        }
    }

    fn try_checkpoint(
        &mut self,
        worktree_id: WorktreeId,
        trigger: CheckpointTrigger,
        cx: &mut Context<Self>,
    ) {
        let Some(state) = self.vaults.get_mut(&worktree_id) else {
            return;
        };
        let HistoryPhase::Ready(repository) = &state.phase else {
            return;
        };
        if state.in_flight {
            // Coalesce: at most one in-flight checkpoint plus one pending.
            state.pending_trigger = Some(trigger);
            return;
        }
        state.in_flight = true;
        state.dirty = false;
        state.idle_task = None;
        let checkpoint = checkpoint_future(repository, trigger, state.config.max_file_bytes);
        cx.spawn(async move |this, cx| {
            let result = checkpoint.await;
            this.update(cx, |this, cx| {
                this.finish_checkpoint(worktree_id, result, cx)
            })
            .ok();
        })
        .detach();
    }

    fn finish_checkpoint(
        &mut self,
        worktree_id: WorktreeId,
        result: Result<Option<GitRepositoryCheckpoint>>,
        cx: &mut Context<Self>,
    ) {
        let Some(state) = self.vaults.get_mut(&worktree_id) else {
            return;
        };
        state.in_flight = false;
        match result {
            // A no-op (unchanged tree) still counts: the vault is known clean.
            Ok(_) => state.last_checkpoint_at = Some(Instant::now()),
            Err(error) => {
                log::debug!(
                    "BreadPaper history: checkpoint of {} failed (will retry on the next trigger): {error:#}",
                    state.root.display()
                );
                // Prior history is intact; mark dirty so the heartbeat retries.
                state.dirty = true;
            }
        }
        if let Some(trigger) = state.pending_trigger.take() {
            self.try_checkpoint(worktree_id, trigger, cx);
        }
    }

    fn checkpoint_on_quit(&mut self, _: &mut Context<Self>) -> impl Future<Output = ()> + use<> {
        let checkpoints: Vec<_> = self
            .vaults
            .values()
            .filter_map(|state| {
                if !state.dirty {
                    return None;
                }
                let HistoryPhase::Ready(repository) = &state.phase else {
                    return None;
                };
                Some(checkpoint_future(
                    repository,
                    CheckpointTrigger::Close,
                    state.config.max_file_bytes,
                ))
            })
            .collect();
        async move {
            for checkpoint in checkpoints {
                checkpoint.await.log_err();
            }
        }
    }
}

impl Drop for HistoryService {
    fn drop(&mut self) {
        // Workspace-close safety net (spec §6.2): best-effort and
        // fire-and-forget. The ref only advances on success, so dying
        // mid-write cannot damage existing history.
        for state in self.vaults.values() {
            if !state.dirty {
                continue;
            }
            let HistoryPhase::Ready(repository) = &state.phase else {
                continue;
            };
            let checkpoint =
                checkpoint_future(repository, CheckpointTrigger::Close, state.config.max_file_bytes);
            self.executor
                .spawn(async move {
                    checkpoint.await.log_err();
                })
                .detach();
        }
    }
}

fn checkpoint_future(
    repository: &RealGitRepository,
    trigger: CheckpointTrigger,
    max_file_bytes: u64,
) -> impl Future<Output = Result<Option<GitRepositoryCheckpoint>>> + use<> {
    repository.checkpoint_onto_ref(
        CHECKPOINTS_REF.to_string(),
        format!(
            "checkpoint: {} {}",
            trigger.label(),
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
        ),
        AUTHOR_NAME.to_string(),
        AUTHOR_EMAIL.to_string(),
        max_file_bytes,
    )
}

async fn open_or_init_repo(
    fs: Arc<dyn Fs>,
    git_dir: PathBuf,
    work_tree: PathBuf,
    bundled_git_binary_path: Option<PathBuf>,
    executor: BackgroundExecutor,
) -> Result<Arc<RealGitRepository>> {
    let system_git_binary_path = which::which("git").ok();
    if !fs.is_file(&git_dir.join("HEAD")).await {
        let git_binary_path = system_git_binary_path
            .as_deref()
            .or(bundled_git_binary_path.as_deref())
            .context("no git binary available")?;
        RealGitRepository::init_separate_git_dir(
            &git_dir,
            &work_tree,
            git_binary_path,
            CHECKPOINTS_BRANCH,
            executor.clone(),
        )
        .await?;
    }
    // Rewritten on every activation so the mandatory excludes are self-healing.
    let info_dir = git_dir.join("info");
    fs.create_dir(&info_dir).await?;
    fs.atomic_write(info_dir.join("exclude"), BASE_EXCLUDES.to_string())
        .await?;

    let repository = RealGitRepository::new_with_separate_git_dir(
        git_dir,
        work_tree,
        bundled_git_binary_path,
        system_git_binary_path,
        executor,
    )?;
    Ok(Arc::new(repository))
}

fn vault_config_rel_path() -> String {
    format!("{VAULT_MARKER_DIR}/{VAULT_CONFIG_FILE}")
}

fn is_history_repo_path(unix_path: &str) -> bool {
    let history_prefix = ".breadpaper/history";
    unix_path == history_prefix
        || unix_path
            .strip_prefix(history_prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_repo_paths_are_filtered() {
        assert!(is_history_repo_path(".breadpaper/history"));
        assert!(is_history_repo_path(".breadpaper/history/objects/ab/cd"));
        assert!(!is_history_repo_path(".breadpaper/config.toml"));
        assert!(!is_history_repo_path(".breadpaper/history-notes.md"));
        assert!(!is_history_repo_path("daily/2026-07-21.md"));
    }
}
