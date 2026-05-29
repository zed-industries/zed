//! Tracks the effective `allow_binary_downloads` setting per worktree and emits
//! events when it flips, so that subsystems that manage tool binaries
//! (language servers, prettier, debug adapters, MCP servers, copilot, npm
//! packages) can react and restart their bookkeeping without each subsystem
//! polling the [`settings::SettingsStore`] on every change.
//!
//! Modelled after [`crate::trusted_worktrees`]: there's a single global
//! [`BinaryDownloads`] entity, and each [`crate::Project`] (or `HeadlessProject`)
//! registers its [`WorktreeStore`] via [`track_binary_downloads`].

use collections::{HashMap, HashSet};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, SharedString, Subscription,
    WeakEntity,
};
use postage::{sink::Sink as _, watch};
use settings::{Settings as _, SettingsLocation, SettingsStore, WorktreeId};
use util::rel_path::RelPath;

use crate::{
    project_settings::ProjectSettings,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};

pub fn init(cx: &mut App) {
    if BinaryDownloads::try_get_global(cx).is_none() {
        let entity = cx.new(BinaryDownloadsStore::new);
        cx.set_global(BinaryDownloads(entity));
    }
}

/// Registers `worktree_store` so that subsequent setting changes that flip
/// `allow_binary_downloads` for any of its worktrees emit a
/// [`BinaryDownloadsEvent`]. Safe to call on hosts where [`init`] was not
/// invoked: it just becomes a no-op.
pub fn track_binary_downloads(worktree_store: Entity<WorktreeStore>, cx: &mut App) {
    let Some(entity) = BinaryDownloads::try_get_global(cx) else {
        return;
    };
    entity.update(cx, |store, cx| {
        store.add_worktree_store(worktree_store, cx);
    });
}

pub struct BinaryDownloads(pub Entity<BinaryDownloadsStore>);
impl Global for BinaryDownloads {}
impl BinaryDownloads {
    pub fn try_get_global(cx: &App) -> Option<Entity<BinaryDownloadsStore>> {
        cx.try_global::<Self>().map(|this| this.0.clone())
    }
}

/// Identifies a single one-off install prompt: a tool needed by a worktree
/// while downloads are disabled. `worktree_id` is `None` for tools that aren't
/// worktree-scoped.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ToolInstall {
    pub worktree_id: Option<WorktreeId>,
    pub tool: SharedString,
}

#[derive(Debug)]
pub enum BinaryDownloadsEvent {
    /// `allow_binary_downloads` flipped from `false` to `true` for the listed
    /// worktrees of the given store.
    Allowed(WeakEntity<WorktreeStore>, HashSet<WorktreeId>),
    /// `allow_binary_downloads` flipped from `true` to `false` for the listed
    /// worktrees of the given store.
    Disallowed(WeakEntity<WorktreeStore>, HashSet<WorktreeId>),
    /// A subsystem needs to install `tool` while downloads are disabled and no
    /// local copy was found. The UI should prompt the user and call back into
    /// [`BinaryDownloadsStore::resolve_tool_install`]. Emitted at most once per
    /// [`ToolInstall`] for the lifetime of the store.
    InstallRequested(ToolInstall),
}

impl EventEmitter<BinaryDownloadsEvent> for BinaryDownloadsStore {}

pub struct BinaryDownloadsStore {
    snapshots: HashMap<WeakEntity<WorktreeStore>, HashMap<WorktreeId, bool>>,
    global_snapshot: bool,
    /// Senders for in-flight "wait until downloads are allowed" channels,
    /// keyed by worktree. A waiter exists only while the worktree's effective
    /// value is `false`; once it flips to `true` the channel is signalled and
    /// dropped. A subsequent flip back to `false` will create a fresh channel
    /// on the next call to [`Self::wait_until_allowed`].
    waiters: HashMap<WorktreeId, watch::Sender<bool>>,
    /// Parallel waiter for callers that don't have a worktree in hand (e.g. a
    /// buffer with no file backing it).
    global_waiter: Option<watch::Sender<bool>>,
    /// One-off per-tool install decisions made through the install prompt.
    /// `true` means "allow installing this tool even though downloads are off";
    /// `false` means the user declined. Either way we never prompt again for
    /// the same [`ToolInstall`].
    tool_decisions: HashMap<ToolInstall, bool>,
    /// In-flight per-tool install waiters. Each fires `true` once the user
    /// approves the prompt or the effective setting flips back on, letting the
    /// awaiting subsystem proceed with the download.
    tool_waiters: HashMap<ToolInstall, watch::Sender<bool>>,
    _worktree_subscriptions: HashMap<WeakEntity<WorktreeStore>, Subscription>,
    _settings_subscription: Subscription,
}

impl BinaryDownloadsStore {
    fn new(cx: &mut Context<Self>) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(Self::on_settings_changed);
        let global_snapshot = Self::allow_binary_downloads(None, cx);
        Self {
            snapshots: HashMap::default(),
            global_snapshot,
            waiters: HashMap::default(),
            global_waiter: None,
            tool_decisions: HashMap::default(),
            tool_waiters: HashMap::default(),
            _worktree_subscriptions: HashMap::default(),
            _settings_subscription: settings_subscription,
        }
    }

    /// Returns a watch channel that yields `true` once `allow_binary_downloads`
    /// becomes `true` for the given scope. Pass `Some(worktree_id)` for a
    /// worktree-scoped wait or `None` to wait on the global default (for
    /// callers that have no worktree in hand, such as buffers backed by no
    /// file). Returns `None` when downloads are already allowed, so callers
    /// can early-out without spinning up a wait.
    ///
    /// This mirrors the worktree-trust wait so that subsystems that need to
    /// download a binary can `await` for approval instead of failing fast and
    /// being restarted later.
    pub fn wait_until_allowed(
        &mut self,
        worktree_id: Option<WorktreeId>,
        cx: &App,
    ) -> Option<watch::Receiver<bool>> {
        if Self::allow_binary_downloads(worktree_id, cx) {
            match worktree_id {
                Some(id) => {
                    self.waiters.remove(&id);
                }
                None => {
                    self.global_waiter = None;
                }
            }
            return None;
        }
        let sender = match worktree_id {
            Some(id) => self
                .waiters
                .entry(id)
                .or_insert_with(|| watch::channel::<bool>().0),
            None => self
                .global_waiter
                .get_or_insert_with(|| watch::channel::<bool>().0),
        };
        Some(sender.subscribe())
    }

    /// Returns the effective `allow_binary_downloads` value, optionally scoped
    /// to a particular worktree. Equivalent to reading [`ProjectSettings`]
    /// directly, but kept here so callers can route all binary-download trust
    /// queries through one place.
    pub fn allow_binary_downloads(worktree_id: Option<WorktreeId>, cx: &App) -> bool {
        let location = worktree_id.map(|worktree_id| SettingsLocation {
            worktree_id,
            path: RelPath::empty(),
        });
        ProjectSettings::get(location, cx).allow_binary_downloads
    }

    /// Returns the effective `prompt_to_install_binaries` value for the scope.
    pub fn prompt_to_install_binaries(worktree_id: Option<WorktreeId>, cx: &App) -> bool {
        let location = worktree_id.map(|worktree_id| SettingsLocation {
            worktree_id,
            path: RelPath::empty(),
        });
        ProjectSettings::get(location, cx).prompt_to_install_binaries
    }

    /// Like [`Self::wait_until_allowed`], but for the case where a subsystem
    /// found no local copy of `tool` and would have to download it. When
    /// `prompt_to_install_binaries` is enabled, the first such request for a
    /// given [`ToolInstall`] emits [`BinaryDownloadsEvent::InstallRequested`]
    /// so the UI can offer a one-off "install this tool" prompt; subsequent
    /// requests reuse the same pending waiter without re-prompting.
    ///
    /// Returns `None` when the download may proceed immediately (downloads are
    /// already allowed, or this tool was previously approved). Otherwise
    /// returns a receiver that yields `true` once the user approves or the
    /// effective setting flips on.
    pub fn request_tool_install(
        &mut self,
        worktree_id: Option<WorktreeId>,
        tool: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Option<watch::Receiver<bool>> {
        let key = ToolInstall {
            worktree_id,
            tool: tool.into(),
        };
        if Self::allow_binary_downloads(worktree_id, cx) {
            self.tool_waiters.remove(&key);
            return None;
        }
        if self.tool_decisions.get(&key).copied() == Some(true) {
            return None;
        }

        let already_pending = self.tool_waiters.contains_key(&key);
        let already_decided = self.tool_decisions.contains_key(&key);
        let sender = self
            .tool_waiters
            .entry(key.clone())
            .or_insert_with(|| watch::channel::<bool>().0);
        let receiver = sender.subscribe();

        if !already_pending && !already_decided && Self::prompt_to_install_binaries(worktree_id, cx)
        {
            cx.emit(BinaryDownloadsEvent::InstallRequested(key));
        }
        Some(receiver)
    }

    /// Records the user's answer to an install prompt. Approving fires the
    /// pending waiter so the awaiting download proceeds; declining keeps the
    /// waiter registered (so a later settings flip still unblocks it) but
    /// ensures we never prompt again for the same [`ToolInstall`].
    pub fn resolve_tool_install(
        &mut self,
        worktree_id: Option<WorktreeId>,
        tool: impl Into<SharedString>,
        install: bool,
    ) {
        let key = ToolInstall {
            worktree_id,
            tool: tool.into(),
        };
        self.tool_decisions.insert(key.clone(), install);
        if install && let Some(mut sender) = self.tool_waiters.remove(&key) {
            sender.blocking_send(true).ok();
        }
    }

    /// The tools that have requested a one-off install while downloads are
    /// disabled and are still waiting for a decision (pending or previously
    /// declined). Approved tools are removed once their download proceeds.
    pub fn pending_tool_installs(&self) -> Vec<ToolInstall> {
        self.tool_waiters.keys().cloned().collect()
    }

    fn flush_tool_waiters(&mut self, cx: &App) {
        let allowed_keys = self
            .tool_waiters
            .keys()
            .filter(|key| Self::allow_binary_downloads(key.worktree_id, cx))
            .cloned()
            .collect::<Vec<_>>();
        for key in allowed_keys {
            if let Some(mut sender) = self.tool_waiters.remove(&key) {
                sender.blocking_send(true).ok();
            }
        }
    }

    fn add_worktree_store(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) {
        self.snapshots.retain(|ws, _| ws.is_upgradable());
        self._worktree_subscriptions
            .retain(|ws, _| ws.is_upgradable());

        let weak = worktree_store.downgrade();
        let snapshot = compute_snapshot(&worktree_store, cx);
        self.snapshots.insert(weak.clone(), snapshot);

        let subscription = cx.subscribe(&worktree_store, Self::on_worktree_store_event);
        self._worktree_subscriptions.insert(weak, subscription);
    }

    fn on_settings_changed(&mut self, cx: &mut Context<Self>) {
        let new_global = Self::allow_binary_downloads(None, cx);
        if new_global != self.global_snapshot {
            self.global_snapshot = new_global;
            if new_global && let Some(mut sender) = self.global_waiter.take() {
                sender.blocking_send(true).ok();
            }
        }

        let keys = self.snapshots.keys().cloned().collect::<Vec<_>>();
        for weak in keys {
            self.refresh(weak, cx);
        }
        self.flush_tool_waiters(cx);
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(_)
            | WorktreeStoreEvent::WorktreeRemoved(..)
            | WorktreeStoreEvent::WorktreeReleased(..) => {
                self.refresh(worktree_store.downgrade(), cx);
            }
            _ => {}
        }
    }

    fn refresh(&mut self, weak: WeakEntity<WorktreeStore>, cx: &mut Context<Self>) {
        let Some(worktree_store) = weak.upgrade() else {
            self.snapshots.remove(&weak);
            self._worktree_subscriptions.remove(&weak);
            return;
        };

        let new = compute_snapshot(&worktree_store, cx);
        let previous = self
            .snapshots
            .insert(weak.clone(), new.clone())
            .unwrap_or_default();

        let mut allowed = HashSet::default();
        let mut disallowed = HashSet::default();
        for (&worktree_id, &current) in &new {
            // Newly added worktrees count as a transition from the inherited
            // global default; only emit when their value differs from what we
            // had recorded, to avoid spurious events on registration.
            if previous.get(&worktree_id).copied() != Some(current)
                && previous.contains_key(&worktree_id)
            {
                if current {
                    allowed.insert(worktree_id);
                } else {
                    disallowed.insert(worktree_id);
                }
            }
        }

        for worktree_id in &allowed {
            if let Some(mut sender) = self.waiters.remove(worktree_id) {
                sender.blocking_send(true).ok();
            }
        }

        if !allowed.is_empty() {
            cx.emit(BinaryDownloadsEvent::Allowed(weak.clone(), allowed));
        }
        if !disallowed.is_empty() {
            cx.emit(BinaryDownloadsEvent::Disallowed(weak, disallowed));
        }
    }
}

fn compute_snapshot(worktree_store: &Entity<WorktreeStore>, cx: &App) -> HashMap<WorktreeId, bool> {
    worktree_store
        .read(cx)
        .worktrees()
        .map(|worktree| {
            let id = worktree.read(cx).id();
            (
                id,
                BinaryDownloadsStore::allow_binary_downloads(Some(id), cx),
            )
        })
        .collect()
}
