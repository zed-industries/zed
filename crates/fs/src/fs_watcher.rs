use futures::{FutureExt as _, channel::oneshot, select_biased};
use gpui::{BackgroundExecutor, Task};
use notify::{Event, EventKind};
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{
        Arc, LazyLock,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, Instant},
};
use util::paths::SanitizedPath;

use crate::{PathEvent, PathEventKind, Watcher};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WatcherMode {
    #[default]
    Native,
    Poll,
}

pub(crate) struct FsWatcher {
    global: GlobalWatcher,
    executor: BackgroundExecutor,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Arc<Mutex<BTreeMap<Arc<Path>, FsWatcherRegistration>>>,
}

enum FsWatcherRegistration {
    Pending {
        _task: Task<()>,
    },
    Registered {
        id: WatcherRegistrationId,
        mode: WatcherMode,
    },
}

impl FsWatcher {
    pub(crate) fn new(
        global: GlobalWatcher,
        executor: BackgroundExecutor,
        tx: async_channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        Self {
            global,
            executor,
            tx,
            pending_path_events,
            registrations: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    fn make_callback(&self, path: &Arc<Path>) -> Arc<dyn Fn(&notify::Event) + Send + Sync> {
        let tx = self.tx.clone();
        let pending_path_events = self.pending_path_events.clone();
        let root_path = SanitizedPath::new_arc(path.as_ref());
        let path = path.clone();
        Arc::new(move |event| {
            push_notify_event(&tx, &pending_path_events, &root_path, path.as_ref(), event)
        })
    }
}

impl Watcher for FsWatcher {
    fn add(&self, path: &Path) -> anyhow::Result<()> {
        log::trace!("watcher add: {path:?}");
        let mut paths = self.registrations.lock();

        let path_is_covered = path.ancestors().skip(1).any(|ancestor| {
            paths.get(ancestor).is_some_and(|entry| match entry {
                FsWatcherRegistration::Pending { .. } => false,
                FsWatcherRegistration::Registered { mode, .. } => {
                    platform_recursive_mode(*mode) == notify::RecursiveMode::Recursive
                }
            })
        });
        if path_is_covered || paths.contains_key(path) {
            log::trace!("path to watch is covered or already requested: {path:?}");
            return Ok(());
        }

        let path: Arc<Path> = path.into();
        if std::fs::symlink_metadata(path.as_ref()).is_err() {
            let task = self.executor.spawn(poll_path_until_created(
                self.global.clone(),
                self.executor.clone(),
                path.clone(),
                self.make_callback(&path),
                self.tx.clone(),
                self.pending_path_events.clone(),
                Arc::downgrade(&self.registrations),
            ));
            paths.insert(path, FsWatcherRegistration::Pending { _task: task });
            return Ok(());
        }

        let mode = if requires_poll_watcher(&path) {
            WatcherMode::Poll
        } else {
            WatcherMode::Native
        };
        let id = self
            .global
            .add(path.clone(), mode, self.make_callback(&path));
        paths.insert(path, FsWatcherRegistration::Registered { id, mode });
        Ok(())
    }

    fn remove(&self, path: &Path) -> anyhow::Result<()> {
        log::trace!("remove watched path: {path:?}");
        let entry = self.registrations.lock().remove(path);
        if let Some(FsWatcherRegistration::Registered { id, .. }) = entry {
            self.global.remove(id);
        }
        Ok(())
    }
}

impl Drop for FsWatcher {
    fn drop(&mut self) {
        let entries = std::mem::take(&mut *self.registrations.lock());
        for (_, entry) in entries {
            if let FsWatcherRegistration::Registered { id, .. } = entry {
                self.global.remove(id);
            }
        }
    }
}

/// Detect whether a path requires polling instead of native file watching.
///
/// Returns `true` for filesystem types where inotify/FSEvents/ReadDirectoryChanges
/// silently fail to deliver events: 9P (WSL drvfs), NFS, CIFS/SMB, FUSE (sshfs), etc.
///
/// Can be overridden with the `ZED_FILE_WATCHER_MODE` environment variable:
/// - `native` — always use native OS watcher
/// - `poll` — always use polling
/// - `auto` (default) — auto-detect based on filesystem type
pub fn requires_poll_watcher(path: &Path) -> bool {
    match std::env::var("ZED_FILE_WATCHER_MODE")
        .as_deref()
        .unwrap_or("auto")
    {
        "native" => return false,
        "poll" => return true,
        _ => {}
    }

    #[cfg(target_os = "linux")]
    {
        return detect_requires_poll_watcher_linux(path);
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = path;
        false
    }
}

#[cfg(target_os = "linux")]
fn detect_requires_poll_watcher_linux(path: &Path) -> bool {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let c_path = match CString::new(path.as_os_str().as_bytes()) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(c_path.as_ptr(), &mut stat) } != 0 {
        return false;
    }

    const V9FS_MAGIC: u64 = 0x0102_1997;
    const NFS_SUPER_MAGIC: u64 = 0x0000_6969;
    const CIFS_MAGIC: u64 = 0xFF53_4D42;
    const SMB_SUPER_MAGIC: u64 = 0x0000_517B;
    const SMB2_MAGIC: u64 = 0xFE53_4D42;
    const FUSE_SUPER_MAGIC: u64 = 0x6573_5546;

    let fs_type = (stat.f_type as u64) & 0xFFFF_FFFF;
    if fs_type == FUSE_SUPER_MAGIC && is_virtiofs(path) {
        return false;
    }

    if fs_type == V9FS_MAGIC
        || fs_type == NFS_SUPER_MAGIC
        || fs_type == CIFS_MAGIC
        || fs_type == SMB_SUPER_MAGIC
        || fs_type == SMB2_MAGIC
        || fs_type == FUSE_SUPER_MAGIC
    {
        log::info!(
            "Detected network/virtual filesystem (type 0x{:x}) at {}, using poll watcher",
            fs_type,
            path.display()
        );
        return true;
    }

    if is_wsl_drvfs_path(path) {
        log::info!(
            "Detected WSL drvfs mount at {}, using poll watcher",
            path.display()
        );
        return true;
    }

    false
}

#[cfg(target_os = "linux")]
fn is_virtiofs(path: &Path) -> bool {
    let Ok(mountinfo) = std::fs::read_to_string("/proc/self/mountinfo") else {
        return false;
    };

    let mut best_mount = None;
    for line in mountinfo.lines() {
        let fields = line.split(' ').collect::<Vec<_>>();
        let Some(separator) = fields.iter().position(|field| *field == "-") else {
            continue;
        };
        let (Some(mount_point), Some(fs_type)) = (fields.get(4), fields.get(separator + 1)) else {
            continue;
        };

        let mount_point = mount_point
            .replace("\\040", " ")
            .replace("\\011", "\t")
            .replace("\\012", "\n")
            .replace("\\134", "\\");
        if path.starts_with(&mount_point)
            && best_mount.is_none_or(|(length, _)| mount_point.len() > length)
        {
            best_mount = Some((mount_point.len(), *fs_type));
        }
    }

    best_mount.is_some_and(|(_, fs_type)| fs_type == "virtiofs" || fs_type == "fuse.virtiofs")
}

#[cfg(target_os = "linux")]
fn is_wsl_drvfs_path(path: &Path) -> bool {
    if std::env::var_os("WSL_DISTRO_NAME").is_none() {
        if let Ok(version) = std::fs::read_to_string("/proc/version") {
            let version = version.to_lowercase();
            if !version.contains("microsoft") && !version.contains("wsl") {
                return false;
            }
        } else {
            return false;
        }
    }

    let Some(path) = path.to_str() else {
        return false;
    };
    if !path.starts_with("/mnt/") || path.len() < 6 {
        return false;
    }
    let after_mnt = &path[5..];
    after_mnt.starts_with(|c: char| c.is_ascii_alphabetic())
        && (after_mnt.len() == 1 || after_mnt.as_bytes()[1] == b'/')
}

async fn poll_path_until_created(
    global: GlobalWatcher,
    executor: BackgroundExecutor,
    path: Arc<Path>,
    callback: Arc<dyn Fn(&notify::Event) + Send + Sync>,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: std::sync::Weak<Mutex<BTreeMap<Arc<Path>, FsWatcherRegistration>>>,
) {
    loop {
        executor.timer(poll_interval()).await;

        if std::fs::symlink_metadata(path.as_ref()).is_err() {
            continue;
        }

        let Some(registrations) = registrations.upgrade() else {
            return;
        };
        {
            let mut registrations = registrations.lock();
            let Some(entry) = registrations.get_mut(path.as_ref()) else {
                return;
            };
            if !matches!(entry, FsWatcherRegistration::Pending { .. }) {
                return;
            }
            let mode = if requires_poll_watcher(&path) {
                WatcherMode::Poll
            } else {
                WatcherMode::Native
            };
            let id = global.add(path.clone(), mode, callback.clone());
            *entry = FsWatcherRegistration::Registered { id, mode };
        }

        enqueue_path_events(
            &tx,
            &pending_path_events,
            vec![
                PathEvent {
                    path: path.to_path_buf(),
                    kind: Some(PathEventKind::Created),
                },
                PathEvent {
                    path: path.to_path_buf(),
                    kind: Some(PathEventKind::Rescan),
                },
            ],
        );
        return;
    }
}

fn enqueue_path_events(
    tx: &smol::channel::Sender<()>,
    pending_path_events: &Arc<Mutex<Vec<PathEvent>>>,
    mut path_events: Vec<PathEvent>,
) {
    if path_events.is_empty() {
        return;
    }

    path_events.sort();
    let mut pending_paths = pending_path_events.lock();
    if pending_paths.is_empty() {
        tx.try_send(()).ok();
    }
    coalesce_pending_rescans(&mut pending_paths, &mut path_events);
    util::extend_sorted(&mut *pending_paths, path_events, usize::MAX, |a, b| {
        a.path.cmp(&b.path)
    });
}

fn push_notify_event(
    tx: &smol::channel::Sender<()>,
    pending_path_events: &Arc<Mutex<Vec<PathEvent>>>,
    root_path: &SanitizedPath,
    watched_root: &Path,
    event: &notify::Event,
) {
    let kind = match event.kind {
        EventKind::Create(_) => Some(PathEventKind::Created),
        EventKind::Modify(_) => Some(PathEventKind::Changed),
        EventKind::Remove(_) => Some(PathEventKind::Removed),
        _ => None,
    };
    let mut path_events = event
        .paths
        .iter()
        .filter_map(|event_path| {
            let event_path = SanitizedPath::new(event_path);
            event_path.starts_with(root_path).then(|| PathEvent {
                path: event_path.as_path().to_path_buf(),
                kind,
            })
        })
        .collect::<Vec<_>>();

    if event.need_rescan() {
        if !watcher_logging_rate_limited() {
            log::warn!("filesystem watcher lost sync for {watched_root:?}; scheduling rescan");
        }

        path_events.retain(|path_event| path_event.path != watched_root);
        path_events.push(PathEvent {
            path: watched_root.to_path_buf(),
            kind: Some(PathEventKind::Rescan),
        });
    }
    log::trace!("path_events: {:?}", path_events);
    enqueue_path_events(tx, pending_path_events, path_events);
}

fn watcher_logging_rate_limited() -> bool {
    static LAST_WARN: Mutex<Option<(Instant, usize)>> = Mutex::new(None);
    let Some((ref mut started, ref mut emitted)) = *LAST_WARN.lock() else {
        *LAST_WARN.lock() = Some((Instant::now(), 0));
        return false;
    };

    if started.elapsed().as_secs() < 1 {
        if *emitted < 20 {
            log::warn!("filesystem watcher lost sync for many files, not logging more");
            return true;
        } else {
            *emitted += 1;
        }
    } else {
        *emitted = 0;
        *started = Instant::now()
    }

    true
}

fn coalesce_pending_rescans(pending_paths: &mut Vec<PathEvent>, path_events: &mut Vec<PathEvent>) {
    if !path_events
        .iter()
        .any(|event| event.kind == Some(PathEventKind::Rescan))
    {
        return;
    }

    let mut new_rescan_paths: Vec<std::path::PathBuf> = path_events
        .iter()
        .filter(|e| e.kind == Some(PathEventKind::Rescan))
        .map(|e| e.path.clone())
        .collect();
    new_rescan_paths.sort_unstable();

    let mut deduped_rescans: Vec<std::path::PathBuf> = Vec::with_capacity(new_rescan_paths.len());
    for path in new_rescan_paths {
        if deduped_rescans
            .iter()
            .any(|ancestor| path != *ancestor && path.starts_with(ancestor))
        {
            continue;
        }
        deduped_rescans.push(path);
    }

    deduped_rescans.retain(|new_path| {
        !pending_paths
            .iter()
            .any(|pending| is_covered_rescan(pending.kind, new_path, &pending.path))
    });

    if !deduped_rescans.is_empty() {
        pending_paths.retain(|pending| {
            !deduped_rescans.iter().any(|rescan_path| {
                pending.path == *rescan_path
                    || is_covered_rescan(pending.kind, &pending.path, rescan_path)
            })
        });
    }

    path_events.retain(|event| {
        event.kind != Some(PathEventKind::Rescan) || deduped_rescans.contains(&event.path)
    });
}

fn is_covered_rescan(kind: Option<PathEventKind>, path: &Path, ancestor: &Path) -> bool {
    kind == Some(PathEventKind::Rescan) && path != ancestor && path.starts_with(ancestor)
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WatcherRegistrationId(u32);

struct WatcherRegistrationState {
    callback: Arc<dyn Fn(&notify::Event) + Send + Sync>,
    path: Arc<std::path::Path>,
    mode: WatcherMode,
}

#[derive(Clone, Copy, Debug, Default)]
struct PathRegistrationState {
    count: u32,
}

struct WatcherState {
    watchers: HashMap<WatcherRegistrationId, WatcherRegistrationState>,
    /// Paths that some `FsWatcher` has asked us to watch using the native backend.
    ///
    /// The backend may or may not be watching them currently.
    native_path_registrations: BTreeMap<Arc<Path>, PathRegistrationState>,
    /// Paths that some `FsWatcher` has asked us to watch using the polling backend.
    ///
    /// The backend may or may not be watching them currently.
    poll_path_registrations: BTreeMap<Arc<Path>, PathRegistrationState>,
    /// Paths that have been registered or unregistered since the last reconciliation.
    ///
    /// We track this as an optimization, so reconciliation doesn't have to scan all the registrations every time.
    dirty_paths: Vec<(WatcherMode, Arc<Path>)>,
    pending_flushes: Vec<oneshot::Sender<()>>,
    pending_health_checks: Vec<oneshot::Sender<anyhow::Result<()>>>,
}

impl WatcherState {
    fn path_registrations_mut(
        &mut self,
        mode: WatcherMode,
    ) -> &mut BTreeMap<Arc<Path>, PathRegistrationState> {
        match mode {
            WatcherMode::Native => &mut self.native_path_registrations,
            WatcherMode::Poll => &mut self.poll_path_registrations,
        }
    }

    fn path_registrations(&self, mode: WatcherMode) -> &BTreeMap<Arc<Path>, PathRegistrationState> {
        match mode {
            WatcherMode::Native => &self.native_path_registrations,
            WatcherMode::Poll => &self.poll_path_registrations,
        }
    }
}

trait WatchBackend: Send {
    fn watch(&mut self, path: &Path, mode: notify::RecursiveMode) -> notify::Result<()>;
    fn unwatch(&mut self, path: &Path) -> notify::Result<()>;
    fn watched_paths(&self) -> notify::Result<Vec<(PathBuf, notify::RecursiveMode)>>;

    fn update_paths(&mut self, ops: Vec<notify::PathOp>) -> Result<(), notify::UpdatePathsError> {
        let mut ops = ops.into_iter();
        while let Some(op) = ops.next() {
            let result = match &op {
                notify::PathOp::Watch(path, config) => self.watch(path, config.recursive_mode()),
                notify::PathOp::Unwatch(path) => self.unwatch(path),
            };
            if let Err(source) = result {
                return Err(notify::UpdatePathsError {
                    source,
                    origin: Some(op),
                    remaining: ops.collect(),
                });
            }
        }
        Ok(())
    }
}

impl<T: notify::Watcher + Send> WatchBackend for T {
    fn watch(&mut self, path: &Path, mode: notify::RecursiveMode) -> notify::Result<()> {
        notify::Watcher::watch(self, path, mode)
    }

    fn unwatch(&mut self, path: &Path) -> notify::Result<()> {
        notify::Watcher::unwatch(self, path)
    }

    fn watched_paths(&self) -> notify::Result<Vec<(PathBuf, notify::RecursiveMode)>> {
        notify::Watcher::watched_paths(self)
    }

    fn update_paths(&mut self, ops: Vec<notify::PathOp>) -> Result<(), notify::UpdatePathsError> {
        notify::Watcher::update_paths(self, ops)
    }
}

/// `GlobalWatcher` is the low-level platform-agnostic manager of filesystem watching. There is one per `RealFs`.
///
/// `GlobalWatcher` talks to a `WatchBackend`, which handles the OS-specific details of filesystem watching.
/// In production, we use the `notify` crate for this. The job of `GlobalWatcher` is to convey watch and unwatch
/// requests from callers across Zed to the `WatchBackend` To do this, it tracks which paths are currently registered
/// for watching, and when this set changes, invokes the `Reconciler` to make appropriate calls into the `WatchBackend`.
///
/// The split between `GlobalWatcher` and `Reconciler` allows the `Reconciler` to run in the background,
/// and to make fewer calls into the backend by coalescing changes to the set of watched paths.
#[derive(Clone)]
pub(crate) struct GlobalWatcher {
    state: Arc<Mutex<WatcherState>>,
    wake_tx: async_channel::Sender<()>,
    next_registration_id: Arc<AtomicU32>,
}

impl GlobalWatcher {
    pub(crate) fn add(
        &self,
        path: Arc<Path>,
        mode: WatcherMode,
        callback: Arc<dyn Fn(&notify::Event) + Send + Sync>,
    ) -> WatcherRegistrationId {
        let id = WatcherRegistrationId(self.next_registration_id.fetch_add(1, Ordering::Relaxed));
        {
            let mut state = self.state.lock();
            state.watchers.insert(
                id,
                WatcherRegistrationState {
                    callback,
                    path: path.clone(),
                    mode,
                },
            );
            state
                .path_registrations_mut(mode)
                .entry(path.clone())
                .or_default()
                .count += 1;
            state.dirty_paths.push((mode, path));
        }
        self.request_reconciliation();
        id
    }

    pub(crate) fn remove(&self, id: WatcherRegistrationId) {
        {
            let mut state = self.state.lock();
            let Some(registration) = state.watchers.remove(&id) else {
                return;
            };
            let desired = state.path_registrations_mut(registration.mode);
            if let Some(registration_state) = desired.get_mut(&registration.path) {
                registration_state.count -= 1;
                if registration_state.count == 0 {
                    desired.remove(&registration.path);
                }
            }
            state
                .dirty_paths
                .push((registration.mode, registration.path.clone()));
        }
        self.request_reconciliation();
    }

    /// The returned channel will resolve once all add/remove requests preceding the `flush` call have been processed by the reconciler.
    ///
    /// Note that reconciliation will happen in any case; calling `flush` is not required.
    pub(crate) fn flush(&self) -> oneshot::Receiver<()> {
        let (flush_tx, flush_rx) = oneshot::channel();
        self.state.lock().pending_flushes.push(flush_tx);
        self.request_reconciliation();
        flush_rx
    }

    /// Asks the reconciler to ensure the native backend exists, reporting back
    /// whatever the backend's creation produced. Used at startup to surface OS
    /// watcher initialization failures (e.g. the inotify instance limit on
    /// Linux) to the user.
    pub(crate) fn check_health(&self) -> oneshot::Receiver<anyhow::Result<()>> {
        let (health_tx, health_rx) = oneshot::channel();
        self.state.lock().pending_health_checks.push(health_tx);
        self.request_reconciliation();
        health_rx
    }

    fn request_reconciliation(&self) {
        match self.wake_tx.try_send(()) {
            Ok(()) => {}
            Err(async_channel::TrySendError::Full(())) => {}
            Err(async_channel::TrySendError::Closed(())) => {
                log::warn!("file watcher reconciler is gone; dropping sync request");
            }
        }
    }

    pub(crate) fn new(executor: &BackgroundExecutor) -> Self {
        Self::with_backends(
            executor,
            (None, platform_recursive_mode(WatcherMode::Native)),
            (None, platform_recursive_mode(WatcherMode::Poll)),
        )
    }

    fn with_backends(
        executor: &BackgroundExecutor,
        native: (Option<Box<dyn WatchBackend>>, notify::RecursiveMode),
        poll: (Option<Box<dyn WatchBackend>>, notify::RecursiveMode),
    ) -> Self {
        let state = Arc::new(Mutex::new(WatcherState {
            watchers: HashMap::new(),
            native_path_registrations: BTreeMap::new(),
            poll_path_registrations: BTreeMap::new(),
            dirty_paths: Vec::new(),
            pending_flushes: Vec::new(),
            pending_health_checks: Vec::new(),
        }));
        let (wake_tx, wake_rx) = async_channel::bounded(1);
        executor
            .spawn(
                Reconciler {
                    watcher_state: state.clone(),
                    executor: executor.clone(),
                    native: BackendState::new(
                        WatcherMode::Native,
                        native.1,
                        executor.clone(),
                        native.0,
                    ),
                    poll: BackendState::new(WatcherMode::Poll, poll.1, executor.clone(), poll.0),
                }
                .run(wake_rx),
            )
            .detach();
        Self {
            state,
            wake_tx,
            next_registration_id: Arc::new(AtomicU32::new(0)),
        }
    }
}

struct BackendState {
    mode: WatcherMode,
    // Whether this backend watches recursively. Determined once at construction
    // (from the platform for the real backend, or chosen explicitly in tests) so
    // that coverage reasoning works even before the backend has been created.
    recursive_mode: notify::RecursiveMode,
    executor: BackgroundExecutor,
    backend: Option<Box<dyn WatchBackend>>,
    /// Paths that we have successfully asked the backend to watch.
    applied_paths: HashSet<Arc<Path>>,
    /// Paths that we asked the backend to watch at some point, that now might or might not be watched because of a backend error.
    suspect_paths: HashSet<Arc<Path>>,
    /// Paths that were registered and that we intend to ask the backend to watch in the future.
    deferred_paths: HashSet<Arc<Path>>,
    stream_restart_rescan_pending: bool,
    cooldown_until: Option<Instant>,
}

struct Reconciler {
    watcher_state: Arc<Mutex<WatcherState>>,
    executor: BackgroundExecutor,
    native: BackendState,
    poll: BackendState,
}

fn earliest(a: Option<Instant>, b: Option<Instant>) -> Option<Instant> {
    a.into_iter().chain(b).min()
}

impl Reconciler {
    async fn run(mut self, wake_rx: async_channel::Receiver<()>) {
        let mut wake_at: Option<Instant> = None;
        loop {
            if let Some(wake_at) = wake_at {
                let timeout = wake_at.saturating_duration_since(self.executor.now());
                select_biased! {
                    wake = wake_rx.recv().fuse() => {
                        if wake.is_err() {
                            break;
                        }
                    }
                    _ = self.executor.timer(timeout).fuse() => {}
                }
            } else if wake_rx.recv().await.is_err() {
                break;
            }

            let flushes;
            (flushes, wake_at) = self.reconcile();

            for flush in flushes {
                flush.send(()).ok();
            }
        }
    }

    fn reconcile(&mut self) -> (Vec<oneshot::Sender<()>>, Option<Instant>) {
        let mut native_affected_paths = HashSet::new();
        let mut poll_affected_paths = HashSet::new();
        let flushes;
        let health_checks;
        {
            let state = self.watcher_state.clone();
            let mut state = state.lock();
            // Taking the flushes and the dirty list in one lock acquisition is what
            // guarantees a flush ack covers every edit that preceded the flush call.
            flushes = std::mem::take(&mut state.pending_flushes);
            health_checks = std::mem::take(&mut state.pending_health_checks);
            let dirty = std::mem::take(&mut state.dirty_paths);
            for (mode, path) in dirty {
                let (affected_paths, is_recursive) = match mode {
                    WatcherMode::Native => (&mut native_affected_paths, self.native.is_recursive()),
                    WatcherMode::Poll => (&mut poll_affected_paths, self.poll.is_recursive()),
                };
                if is_recursive {
                    for (descendant, _) in state.path_registrations(mode).range::<Path, _>((
                        std::ops::Bound::Excluded(path.as_ref()),
                        std::ops::Bound::Unbounded,
                    )) {
                        if !descendant.starts_with(&path) {
                            break;
                        }
                        affected_paths.insert(descendant.clone());
                    }
                }
                affected_paths.insert(path);
            }
        }
        native_affected_paths.extend(self.native.deferred_paths.iter().cloned());
        poll_affected_paths.extend(self.poll.deferred_paths.iter().cloned());

        let wake_at = earliest(
            self.poll
                .reconcile(poll_affected_paths, &self.watcher_state),
            self.native
                .reconcile(native_affected_paths, &self.watcher_state),
        );

        for health_check in health_checks {
            // Re-attempt per request: backend creation is idempotent, so once it
            // succeeds further requests resolve immediately, and a failure leaves
            // the backend unset so each request gets its own fresh error (which
            // is not `Clone`).
            let result = self.native.ensure_backend(&self.watcher_state).map(|_| ());
            health_check.send(result).ok();
        }

        (flushes, wake_at)
    }
}

impl BackendState {
    fn new(
        mode: WatcherMode,
        recursive_mode: notify::RecursiveMode,
        executor: BackgroundExecutor,
        backend: Option<Box<dyn WatchBackend>>,
    ) -> Self {
        Self {
            mode,
            recursive_mode,
            executor,
            backend,
            applied_paths: HashSet::new(),
            suspect_paths: HashSet::new(),
            deferred_paths: HashSet::new(),
            stream_restart_rescan_pending: false,
            cooldown_until: None,
        }
    }

    fn is_recursive(&self) -> bool {
        self.recursive_mode == notify::RecursiveMode::Recursive
    }

    fn reconcile(
        &mut self,
        affected_paths: HashSet<Arc<Path>>,
        watcher_state: &Arc<Mutex<WatcherState>>,
    ) -> Option<Instant> {
        if affected_paths.is_empty() {
            return None;
        }
        let affected_count = affected_paths.len();
        let started_at = Instant::now();

        let mut to_watch = Vec::new();
        let mut to_unwatch = Vec::new();
        {
            let watcher_state = watcher_state.lock();
            let registrations = watcher_state.path_registrations(self.mode);
            for path in affected_paths {
                let is_desired = registrations.contains_key(&path);
                let is_covered = self.is_recursive()
                    && path.ancestors().skip(1).any(|ancestor| {
                        registrations.contains_key(ancestor)
                            && self.applied_paths.contains(ancestor)
                    });
                let should_watch = is_desired && !is_covered;
                if !should_watch {
                    self.deferred_paths.remove(&path);
                }
                if cfg!(any(test, debug_assertions)) {
                    assert!(
                        should_watch || !self.deferred_paths.contains(&path),
                        "{:?} keeps a covered or unregistered path queued for a watch: {:?}",
                        self.mode,
                        path,
                    )
                }
                let is_applied = self.applied_paths.contains(&path);
                if should_watch && !is_applied {
                    to_watch.push(path);
                } else if !should_watch && is_applied {
                    to_unwatch.push(path);
                }
            }
        }
        to_watch.sort();
        to_unwatch.sort();
        let watch_count = to_watch.len();
        let unwatch_count = to_unwatch.len();

        let wake_at = earliest(
            self.apply_unwatches(to_unwatch),
            self.apply_watches(to_watch, watcher_state),
        );
        if self.deferred_paths.is_empty() {
            self.stream_restart_rescan_pending = false;
        }

        self.verify_backend_watches();

        log::debug!(
            "fs watcher reconcile ({:?}): examined {affected_count}, watched {watch_count}, unwatched {unwatch_count}, {} applied total, took {:?}",
            self.mode,
            self.applied_paths.len(),
            started_at.elapsed(),
        );

        wake_at
    }

    fn verify_backend_watches(&self) {
        if !cfg!(any(test, debug_assertions)) {
            return;
        }

        let Some(backend) = self.backend.as_ref() else {
            return;
        };
        let backend_paths = match backend.watched_paths() {
            Ok(paths) => paths,
            Err(error) => {
                log::debug!(
                    "cannot verify watched paths for {:?} backend: {error}",
                    self.mode
                );
                return;
            }
        };
        let backend_paths = backend_paths
            .into_iter()
            .map(|(path, _)| path)
            .collect::<HashSet<PathBuf>>();

        // Check that the backend is in fact watching the paths we think it is definitely watching.
        let missing_paths = self
            .applied_paths
            .iter()
            .filter(|path| !backend_paths.contains(path.as_ref()))
            .collect::<Vec<_>>();
        assert!(
            missing_paths.is_empty(),
            "{:?} backend is missing paths we believe are watched: {missing_paths:?}",
            self.mode,
        );

        // Check that we haven't left any unexpected watches in the backend.
        //
        // We allow that the backend may be watching some paths for which `unwatch` failed,
        // or as a result of a stream restart error, in addition to the ones we successfully
        // asked it to watch.
        let unexpected_paths = backend_paths
            .iter()
            .filter(|path| {
                !self.applied_paths.contains(path.as_path())
                    && !self.suspect_paths.contains(path.as_path())
            })
            .collect::<Vec<_>>();
        assert!(
            unexpected_paths.is_empty(),
            "{:?} backend is watching paths we do not track: {unexpected_paths:?}",
            self.mode,
        );

        // Check that no successfully-watched path is also deferred.
        assert!(
            self.applied_paths.is_disjoint(&self.deferred_paths),
            "a path is both applied and deferred for {:?}",
            self.mode,
        );
    }

    fn apply_unwatches(&mut self, paths: Vec<Arc<Path>>) -> Option<Instant> {
        if paths.is_empty() {
            return None;
        }
        for path in &paths {
            self.applied_paths.remove(path);
        }
        let Some(backend) = self.backend.as_mut() else {
            return None;
        };

        let mut queue = paths;
        let mut any_succeeded = false;
        let mut restart_failure = None;
        let mut failed = Vec::new();
        while !queue.is_empty() {
            let ops = queue
                .iter()
                .map(|path| notify::PathOp::unwatch(path.as_ref()))
                .collect();
            match backend.update_paths(ops) {
                Ok(()) => {
                    any_succeeded = true;
                    break;
                }
                Err(error) => {
                    let Some(origin) = &error.origin else {
                        any_succeeded |= queue.len() > error.remaining.len();
                        restart_failure = Some(error.source);
                        break;
                    };
                    let applied_count = queue.len() - error.remaining.len() - 1;
                    any_succeeded |= applied_count > 0;
                    log::warn!(
                        "failed to unwatch {:?}: {:#}",
                        origin.as_path(),
                        error.source
                    );
                    if let Some(path) = queue.get(applied_count) {
                        failed.push(path.clone());
                    }
                    queue.drain(..queue.len() - error.remaining.len());
                }
            }
        }

        // A failed unwatch may have left the path watched in the backend, so record its state as indeterminate.
        self.suspect_paths.extend(failed);
        // If we unwatched at least one path, there might now be room to watch, so clear the cooldown.
        if any_succeeded {
            self.cooldown_until = None;
        }
        restart_failure.map(|source| self.handle_stream_restart_failure(Vec::new(), &source))
    }

    fn apply_watches(
        &mut self,
        paths: Vec<Arc<Path>>,
        watcher_state: &Arc<Mutex<WatcherState>>,
    ) -> Option<Instant> {
        let recursive_mode = self.recursive_mode;
        let mut queue = paths;
        let mut wake_at = None;
        while !queue.is_empty() {
            if self.is_cooldown_active() {
                self.deferred_paths.extend(queue);
                return earliest(wake_at, self.cooldown_until);
            }

            let backend = match self.ensure_backend(watcher_state) {
                Ok(backend) => backend,
                Err(error) => {
                    log::warn!(
                        "failed to create file watcher backend for {:?}: {error:#}",
                        self.mode,
                    );
                    return wake_at;
                }
            };

            let ops = queue
                .iter()
                .map(|path| match recursive_mode {
                    notify::RecursiveMode::Recursive => {
                        notify::PathOp::watch_recursive(path.as_ref())
                    }
                    notify::RecursiveMode::NonRecursive => {
                        notify::PathOp::watch_non_recursive(path.as_ref())
                    }
                })
                .collect();
            match backend.update_paths(ops) {
                Ok(()) => {
                    let mut applied_count = 0;
                    for path in queue {
                        applied_count += 1;
                        self.mark_watched(path);
                    }
                    self.emit_stream_restart_rescan_if_recovered(applied_count, watcher_state);
                    return wake_at;
                }
                Err(error) => {
                    if error.origin.is_none() {
                        let restart_at = self.handle_stream_restart_failure(queue, &error.source);
                        return earliest(wake_at, Some(restart_at));
                    }

                    let applied_count = queue.len() - error.remaining.len() - 1;
                    let mut rest = queue.split_off(applied_count);
                    for path in queue {
                        self.mark_watched(path);
                    }
                    self.emit_stream_restart_rescan_if_recovered(applied_count, watcher_state);

                    let failed = rest.remove(0);
                    self.deferred_paths.remove(&failed);
                    if self.mode == WatcherMode::Native
                        && matches!(error.source.kind, notify::ErrorKind::MaxFilesWatch)
                    {
                        self.start_cooldown(&failed);
                        self.deferred_paths.insert(failed);
                        wake_at = earliest(wake_at, self.cooldown_until);
                    } else {
                        log::warn!("failed to watch {failed:?}: {:#}", error.source);
                    }
                    queue = rest;
                }
            }
        }
        wake_at
    }

    fn mark_watched(&mut self, path: Arc<Path>) {
        self.deferred_paths.remove(&path);
        self.suspect_paths.remove(&path);
        self.applied_paths.insert(path);
    }

    // An error with no origin means the backend applied our path operations but
    // could not restart its event stream afterwards (seen with FSEvents in the
    // field), so every applied watch is suspect: defer them all and re-establish
    // them in one batch once the returned deadline fires, emitting rescans on
    // recovery so consumers learn about the gap.
    fn handle_stream_restart_failure(
        &mut self,
        pending: Vec<Arc<Path>>,
        source: &notify::Error,
    ) -> Instant {
        log::warn!(
            "file watcher backend for {:?} failed to restart its event stream: {source:#}; re-establishing all watches",
            self.mode,
        );
        // The ops were applied to the backend (it still reports these paths), but
        // the dead stream means they aren't delivering events. They're both
        // suspect-in-backend (`suspect`, so the upper bound covers them while they
        // sit in the backend) and pending re-establishment (`deferred`, which
        // drives the retry).
        let applied = std::mem::take(&mut self.applied_paths);
        self.suspect_paths.extend(applied.iter().cloned());
        self.suspect_paths.extend(pending.iter().cloned());
        self.deferred_paths.extend(applied);
        self.deferred_paths.extend(pending);
        self.stream_restart_rescan_pending = true;
        let retry_at = self.executor.now() + *FILE_WATCHER_RETRY_DELAY;
        self.cooldown_until = Some(retry_at);
        retry_at
    }

    fn is_cooldown_active(&self) -> bool {
        self.cooldown_until
            .is_some_and(|cooldown_until| cooldown_until > self.executor.now())
    }

    fn start_cooldown(&mut self, path: &Path) {
        let should_log = !self.is_cooldown_active();
        self.cooldown_until = Some(self.executor.now() + *NATIVE_WATCH_LIMIT_COOLDOWN);
        if should_log {
            log::warn!(
                "OS file watch limit reached while watching {path:?}; skipping new native file watcher registrations for {} seconds",
                NATIVE_WATCH_LIMIT_COOLDOWN.as_secs()
            );
        }
    }

    fn emit_stream_restart_rescan_if_recovered(
        &mut self,
        applied_count: usize,
        watcher_state: &Mutex<WatcherState>,
    ) {
        if self.stream_restart_rescan_pending && applied_count > 0 {
            self.emit_backend_rescan(watcher_state);
            self.stream_restart_rescan_pending = false;
        }
    }

    fn emit_backend_rescan(&self, watcher_state: &Mutex<WatcherState>) {
        let callbacks = {
            let watcher_state = watcher_state.lock();
            watcher_state
                .watchers
                .values()
                .filter(|registration| registration.mode == self.mode)
                .map(|registration| registration.callback.clone())
                .collect::<Vec<_>>()
        };
        let event = Event::new(EventKind::Other).set_flag(notify::event::Flag::Rescan);
        for callback in callbacks {
            callback(&event);
        }
    }

    fn ensure_backend(
        &mut self,
        watcher_state: &Arc<Mutex<WatcherState>>,
    ) -> anyhow::Result<&mut Box<dyn WatchBackend>> {
        if self.backend.is_none() {
            self.backend = Some(create_backend(self.mode, watcher_state.clone())?);
        }
        Ok(self.backend.as_mut().expect("backend was just initialized"))
    }
}

fn create_backend(
    mode: WatcherMode,
    watcher_state: Arc<Mutex<WatcherState>>,
) -> anyhow::Result<Box<dyn WatchBackend>> {
    match mode {
        WatcherMode::Native => {
            // CORE excludes Access events, which Zed discards anyway. Without this,
            // the default mask subscribes to inotify OPEN/CLOSE_* on Linux, so every
            // file read in a watched directory would queue events, increasing the
            // risk of queue overflows (and thus full rescans) under read-heavy
            // workloads like grep or language server indexing.
            let config = notify::Config::default().with_event_kinds(notify::EventKindMask::CORE);
            let watcher = <notify::RecommendedWatcher as notify::Watcher>::new(
                move |event: notify::Result<Event>| {
                    handle_event(WatcherMode::Native, &watcher_state, event);
                },
                config,
            )?;
            Ok(Box::new(watcher))
        }
        WatcherMode::Poll => {
            let config = notify::Config::default().with_poll_interval(*POLL_INTERVAL);
            let watcher = notify::PollWatcher::new(
                move |event: notify::Result<Event>| {
                    handle_event(WatcherMode::Poll, &watcher_state, event);
                },
                config,
            )?;
            Ok(Box::new(watcher))
        }
    }
}

fn platform_recursive_mode(mode: WatcherMode) -> notify::RecursiveMode {
    match mode {
        WatcherMode::Native => {
            if cfg!(any(target_os = "windows", target_os = "macos")) {
                notify::RecursiveMode::Recursive
            } else {
                notify::RecursiveMode::NonRecursive
            }
        }
        WatcherMode::Poll => notify::RecursiveMode::Recursive,
    }
}

static POLL_INTERVAL: LazyLock<Duration> = LazyLock::new(|| {
    let poll_ms: u64 = std::env::var("ZED_FILE_WATCHER_POLL_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(2000)
        .clamp(500, 30000);
    Duration::from_millis(poll_ms)
});

static NATIVE_WATCH_LIMIT_COOLDOWN: LazyLock<Duration> = LazyLock::new(|| {
    let cooldown_seconds: u64 = std::env::var("ZED_NATIVE_WATCH_LIMIT_COOLDOWN_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(5)
        .clamp(0, 300);
    Duration::from_secs(cooldown_seconds)
});

static FILE_WATCHER_RETRY_DELAY: LazyLock<Duration> = LazyLock::new(|| {
    let retry_seconds: u64 = std::env::var("ZED_FILE_WATCHER_RETRY_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(5)
        .clamp(0, 300);
    Duration::from_secs(retry_seconds)
});

pub fn poll_interval() -> Duration {
    *POLL_INTERVAL
}

fn handle_event(mode: WatcherMode, state: &Mutex<WatcherState>, event: notify::Result<Event>) {
    if matches!(
        event,
        Ok(Event {
            kind: EventKind::Access(_),
            ..
        })
    ) {
        return;
    }

    log::trace!("global handle event for {mode:?}: {event:?}");

    let callbacks = {
        let state = state.lock();
        state
            .watchers
            .values()
            .filter(|registration| registration.mode == mode)
            .map(|registration| registration.callback.clone())
            .collect::<Vec<_>>()
    };

    match event {
        Ok(event) => {
            if event.need_rescan() {
                log::warn!(
                    "filesystem watcher lost sync for {mode:?}; scheduling rescans for {} registrations",
                    callbacks.len()
                );
            }
            for callback in callbacks {
                callback(&event);
            }
        }
        Err(error) => {
            log::warn!("watcher error for {mode:?}: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::RecursiveMode;
    use std::{collections::HashSet, path::PathBuf};

    fn rescan(path: &str) -> PathEvent {
        PathEvent {
            path: PathBuf::from(path),
            kind: Some(PathEventKind::Rescan),
        }
    }

    fn changed(path: &str) -> PathEvent {
        PathEvent {
            path: PathBuf::from(path),
            kind: Some(PathEventKind::Changed),
        }
    }

    fn watch_limit_error() -> notify::Error {
        notify::Error::new(notify::ErrorKind::MaxFilesWatch)
    }

    fn generic_error() -> notify::Error {
        notify::Error::generic("watch failed")
    }

    fn stream_restart_error() -> notify::Error {
        notify::Error::generic("simulated stream restart failure")
    }

    #[derive(Default)]
    struct FakeWatchBackend {
        watched_paths: HashSet<PathBuf>,
        watch_calls: Vec<PathBuf>,
        unwatch_calls: Vec<PathBuf>,
        // Maps a path to the error its watch should report. A failed watch never
        // takes effect, so the backend never ends up watching such a path.
        watch_errors: HashMap<PathBuf, fn() -> notify::Error>,
        // Maps a path to the error its unwatch should report, and whether the
        // removal nonetheless takes effect in the backend. A failed unwatch
        // genuinely may or may not leave the path watched, so this lets a test pin
        // either side of that ambiguous failure.
        unwatch_errors: HashMap<PathBuf, (fn() -> notify::Error, bool)>,
        stream_restart_error: Option<fn() -> notify::Error>,
    }

    struct SharedFakeWatchBackend(Arc<Mutex<FakeWatchBackend>>);

    impl WatchBackend for SharedFakeWatchBackend {
        fn watch(&mut self, path: &Path, _mode: notify::RecursiveMode) -> notify::Result<()> {
            let path = path.to_path_buf();
            let mut backend = self.0.lock();
            backend.watch_calls.push(path.clone());
            if let Some(make_error) = backend.watch_errors.get(&path).copied() {
                return Err(make_error());
            }
            backend.watched_paths.insert(path);
            Ok(())
        }

        fn unwatch(&mut self, path: &Path) -> notify::Result<()> {
            let path = path.to_path_buf();
            let mut backend = self.0.lock();
            backend.unwatch_calls.push(path.clone());
            if let Some((make_error, took_effect)) = backend.unwatch_errors.get(&path).copied() {
                if took_effect {
                    backend.watched_paths.remove(&path);
                }
                return Err(make_error());
            }
            if backend.watched_paths.remove(&path) {
                Ok(())
            } else {
                Err(notify::Error::generic("path was not watched"))
            }
        }

        fn watched_paths(&self) -> notify::Result<Vec<(PathBuf, notify::RecursiveMode)>> {
            Ok(self
                .0
                .lock()
                .watched_paths
                .iter()
                .map(|path| (path.clone(), notify::RecursiveMode::Recursive))
                .collect())
        }

        fn update_paths(
            &mut self,
            ops: Vec<notify::PathOp>,
        ) -> Result<(), notify::UpdatePathsError> {
            let mut ops_iter = ops.into_iter();
            while let Some(op) = ops_iter.next() {
                let result = match &op {
                    notify::PathOp::Watch(path, config) => {
                        WatchBackend::watch(self, path, config.recursive_mode())
                    }
                    notify::PathOp::Unwatch(path) => WatchBackend::unwatch(self, path),
                };
                if let Err(source) = result {
                    return Err(notify::UpdatePathsError {
                        source,
                        origin: Some(op),
                        remaining: ops_iter.collect(),
                    });
                }
            }
            if let Some(make_error) = self.0.lock().stream_restart_error {
                return Err(notify::UpdatePathsError {
                    source: make_error(),
                    origin: None,
                    remaining: Vec::new(),
                });
            }
            Ok(())
        }
    }

    struct TestCase {
        name: &'static str,
        pending_paths: Vec<PathEvent>,
        path_events: Vec<PathEvent>,
        expected_pending_paths: Vec<PathEvent>,
        expected_path_events: Vec<PathEvent>,
    }

    #[test]
    fn test_coalesce_pending_rescans() {
        let test_cases = [
            TestCase {
                name: "coalesces descendant rescans under pending ancestor",
                pending_paths: vec![rescan("/root")],
                path_events: vec![rescan("/root/child"), rescan("/root/child/grandchild")],
                expected_pending_paths: vec![rescan("/root")],
                expected_path_events: vec![],
            },
            TestCase {
                name: "new ancestor rescan replaces pending descendant rescans",
                pending_paths: vec![
                    changed("/other"),
                    rescan("/root/child"),
                    rescan("/root/child/grandchild"),
                ],
                path_events: vec![rescan("/root")],
                expected_pending_paths: vec![changed("/other")],
                expected_path_events: vec![rescan("/root")],
            },
            TestCase {
                name: "same path rescan replaces pending non-rescan event",
                pending_paths: vec![changed("/root")],
                path_events: vec![rescan("/root")],
                expected_pending_paths: vec![],
                expected_path_events: vec![rescan("/root")],
            },
            TestCase {
                name: "unrelated rescans are preserved",
                pending_paths: vec![rescan("/root-a")],
                path_events: vec![rescan("/root-b")],
                expected_pending_paths: vec![rescan("/root-a")],
                expected_path_events: vec![rescan("/root-b")],
            },
            TestCase {
                name: "batch ancestor rescan replaces descendant rescan",
                pending_paths: vec![],
                path_events: vec![rescan("/root/child"), rescan("/root")],
                expected_pending_paths: vec![],
                expected_path_events: vec![rescan("/root")],
            },
        ];

        for test_case in test_cases {
            let mut pending_paths = test_case.pending_paths;
            let mut path_events = test_case.path_events;

            coalesce_pending_rescans(&mut pending_paths, &mut path_events);

            assert_eq!(
                pending_paths, test_case.expected_pending_paths,
                "pending_paths mismatch for case: {}",
                test_case.name
            );
            assert_eq!(
                path_events, test_case.expected_path_events,
                "path_events mismatch for case: {}",
                test_case.name
            );
        }
    }

    type FakeBackend = Option<(Arc<Mutex<FakeWatchBackend>>, notify::RecursiveMode)>;

    fn fake_backend(
        backend: FakeBackend,
        mode: WatcherMode,
    ) -> (Option<Box<dyn WatchBackend>>, notify::RecursiveMode) {
        match backend {
            Some((backend, recursive_mode)) => (
                Some(Box::new(SharedFakeWatchBackend(backend)) as Box<dyn WatchBackend>),
                recursive_mode,
            ),
            None => (None, platform_recursive_mode(mode)),
        }
    }

    fn test_global(
        executor: &BackgroundExecutor,
        native_backend: FakeBackend,
        poll_backend: FakeBackend,
    ) -> GlobalWatcher {
        GlobalWatcher::with_backends(
            executor,
            fake_backend(native_backend, WatcherMode::Native),
            fake_backend(poll_backend, WatcherMode::Poll),
        )
    }

    fn noop_callback() -> Arc<dyn Fn(&notify::Event) + Send + Sync> {
        Arc::new(|_| {})
    }

    fn collecting_callback() -> (
        Arc<Mutex<Vec<Event>>>,
        Arc<dyn Fn(&notify::Event) + Send + Sync>,
    ) {
        let events: Arc<Mutex<Vec<Event>>> = Default::default();
        let callback = {
            let events = events.clone();
            Arc::new(move |event: &notify::Event| events.lock().push(event.clone()))
                as Arc<dyn Fn(&notify::Event) + Send + Sync>
        };
        (events, callback)
    }

    #[gpui::test]
    async fn watch_and_unwatch_call_the_backend_once_per_path(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path = Arc::<Path>::from(Path::new("/repo"));

        let first = global.add(path.clone(), WatcherMode::Native, noop_callback());
        let second = global.add(path.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[PathBuf::from("/repo")]);

        global.remove(first);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, Vec::<PathBuf>::new());

        global.remove(second);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, &[PathBuf::from("/repo")]);
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn covered_child_is_promoted_when_parent_is_unregistered(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            None,
            Some((backend.clone(), RecursiveMode::Recursive)),
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/foo.csproj"));

        let parent_id = global.add(parent.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        let child_id = global.add(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[parent.to_path_buf()]);

        global.remove(parent_id);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, &[parent.to_path_buf()]);
        assert_eq!(
            backend.lock().watch_calls,
            &[parent.to_path_buf(), child.to_path_buf()],
            "covered child is promoted to its own watch once the parent goes away"
        );
        assert!(backend.lock().watched_paths.contains(child.as_ref()));

        global.remove(child_id);
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn existing_child_is_not_demoted_when_covering_parent_is_registered(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            None,
            Some((backend.clone(), RecursiveMode::Recursive)),
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/sub"));

        global.add(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[child.to_path_buf()]);

        global.add(parent.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();

        let backend = backend.lock();
        assert_eq!(
            backend.watch_calls,
            &[child.to_path_buf(), parent.to_path_buf()]
        );
        assert_eq!(backend.unwatch_calls, Vec::<PathBuf>::new());
        assert!(backend.watched_paths.contains(parent.as_ref()));
        assert!(backend.watched_paths.contains(child.as_ref()));
    }

    #[gpui::test]
    async fn failed_recursive_parent_does_not_cover_child(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            None,
            Some((backend.clone(), RecursiveMode::Recursive)),
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/sub"));
        backend
            .lock()
            .watch_errors
            .insert(parent.to_path_buf(), generic_error);

        global.add(parent.clone(), WatcherMode::Poll, noop_callback());
        global.add(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();

        let backend = backend.lock();
        assert_eq!(
            backend.watch_calls,
            &[parent.to_path_buf(), child.to_path_buf()]
        );
        assert!(!backend.watched_paths.contains(parent.as_ref()));
        assert!(backend.watched_paths.contains(child.as_ref()));
    }

    #[gpui::test]
    async fn non_recursive_backend_does_not_cover_children(executor: BackgroundExecutor) {
        // A non-recursive backend (e.g. inotify) does not deliver events for a
        // child via an ancestor's watch, so coverage must not kick in: every
        // registered path keeps its own watch, nothing is demoted, and removing an
        // ancestor leaves the descendant's watch untouched. This is the mirror
        // image of `covered_child_is_promoted_when_parent_is_unregistered`, where
        // a recursive ancestor covers the child and its removal promotes it. The
        // polymorphic fake lets us pin this on any host, independent of what
        // `platform_recursive_mode` would pick.
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((backend.clone(), RecursiveMode::NonRecursive)),
            None,
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/sub"));

        let (child_events, child_callback) = collecting_callback();
        global.add(child.clone(), WatcherMode::Native, child_callback);
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[child.to_path_buf()]);

        let parent_id = global.add(parent.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        {
            let backend = backend.lock();
            assert_eq!(
                backend.watch_calls,
                &[child.to_path_buf(), parent.to_path_buf()],
                "the parent gets its own watch without covering the child"
            );
            assert_eq!(
                backend.unwatch_calls,
                Vec::<PathBuf>::new(),
                "no demotion happens without recursive coverage"
            );
            assert!(backend.watched_paths.contains(parent.as_ref()));
            assert!(
                backend.watched_paths.contains(child.as_ref()),
                "the child keeps its own watch under a non-recursive backend"
            );
        }

        // Removing the ancestor must not disturb the child: with no coverage there
        // is nothing to promote, so the child is neither re-watched nor rescanned.
        global.remove(parent_id);
        executor.run_until_parked();
        {
            let backend = backend.lock();
            assert_eq!(
                backend.unwatch_calls,
                &[parent.to_path_buf()],
                "only the ancestor is unwatched"
            );
            assert_eq!(
                backend.watch_calls,
                &[child.to_path_buf(), parent.to_path_buf()],
                "the child is not re-watched when the ancestor goes away"
            );
            assert!(backend.watched_paths.contains(child.as_ref()));
        }
        assert!(
            child_events.lock().is_empty(),
            "the child gets no rescan since its watch never lapsed"
        );
    }

    #[gpui::test]
    async fn removing_a_covered_child_issues_no_unwatch(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            None,
            Some((backend.clone(), RecursiveMode::Recursive)),
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/foo.csproj"));

        let parent_id = global.add(parent.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        let child_id = global.add(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert_eq!(
            backend.lock().watch_calls,
            &[parent.to_path_buf()],
            "the covered child never gets its own OS watch"
        );

        global.remove(child_id);
        executor.run_until_parked();
        assert_eq!(
            backend.lock().unwatch_calls,
            Vec::<PathBuf>::new(),
            "removing a covered child issues no OS unwatch, since it never had one"
        );
        assert!(backend.lock().watched_paths.contains(parent.as_ref()));

        global.remove(parent_id);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, &[parent.to_path_buf()]);
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn existing_nested_registrations_can_leave_nested_applied_paths(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            None,
            Some((backend.clone(), RecursiveMode::Recursive)),
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/a"));
        let grandchild = Arc::<Path>::from(Path::new("/repo/a/b"));

        global.add(grandchild.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(grandchild.as_ref()));

        global.add(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(child.as_ref()));
        assert!(backend.lock().watched_paths.contains(grandchild.as_ref()));

        global.add(parent.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        let backend = backend.lock();
        assert!(backend.watched_paths.contains(parent.as_ref()));
        assert!(backend.watched_paths.contains(child.as_ref()));
        assert!(!backend.watched_paths.contains(grandchild.as_ref()));
        assert_eq!(backend.unwatch_calls, &[grandchild.to_path_buf()]);
    }

    #[gpui::test]
    async fn deferred_child_recovers_even_when_parent_is_registered(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            None,
            Some((backend.clone(), RecursiveMode::Recursive)),
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/sub"));

        // The child's initial watch applies, but the stream fails to restart, so
        // the child is deferred (its events aren't being delivered) behind a
        // cooldown rather than established.
        backend.lock().stream_restart_error = Some(stream_restart_error);
        let (child_events, child_callback) = collecting_callback();
        global.add(child.clone(), WatcherMode::Poll, child_callback);
        executor.run_until_parked();

        backend.lock().stream_restart_error = None;
        let parent_id = global.add(parent.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();

        executor.advance_clock(*FILE_WATCHER_RETRY_DELAY + Duration::from_secs(1));
        executor.run_until_parked();

        assert!(backend.lock().watched_paths.contains(parent.as_ref()));
        assert!(backend.lock().watched_paths.contains(child.as_ref()));
        assert_eq!(child_events.lock().len(), 1);
        assert!(child_events.lock()[0].need_rescan());

        global.remove(parent_id);
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(child.as_ref()));
    }

    #[gpui::test]
    async fn unregister_then_register_same_path_coalesces(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path = Arc::<Path>::from(Path::new("/repo"));

        let first = global.add(path.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[path.to_path_buf()]);

        global.remove(first);
        let second = global.add(path.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert!(backend.watched_paths.contains(path.as_ref()));
            assert_eq!(
                backend.watch_calls,
                &[path.to_path_buf()],
                "reconciler coalesces the remove/add pair into a no-op"
            );
            assert_eq!(backend.unwatch_calls, Vec::<PathBuf>::new());
        }

        global.remove(second);
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn failed_watch_is_not_retried_until_reregistered(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        backend
            .lock()
            .watch_errors
            .insert(path_a.to_path_buf(), generic_error);
        backend
            .lock()
            .watch_errors
            .insert(path_b.to_path_buf(), generic_error);

        let first = global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[path_a.to_path_buf()]);

        global.add(path_b.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(
            backend.lock().watch_calls,
            &[path_a.to_path_buf(), path_b.to_path_buf()],
            "the failed path is not retried on later passes"
        );

        backend.lock().watch_errors.clear();
        global.remove(first);
        executor.run_until_parked();

        global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert!(
            backend.lock().watched_paths.contains(path_a.as_ref()),
            "re-registering after the path left desired state retries the watch"
        );
    }

    #[gpui::test]
    async fn batched_watch_skips_only_the_path_with_a_generic_error(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        let path_c = Arc::<Path>::from(Path::new("/repo/c"));
        backend
            .lock()
            .watch_errors
            .insert(path_b.to_path_buf(), generic_error);

        // Registering all three before parking lands them in a single reconcile
        // batch, so they go to the backend in one `update_paths` call.
        global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        global.add(path_b.clone(), WatcherMode::Native, noop_callback());
        global.add(path_c.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert_eq!(
                backend.watch_calls,
                &[
                    path_a.to_path_buf(),
                    path_b.to_path_buf(),
                    path_c.to_path_buf(),
                ],
                "every path in the batch is attempted once, in sorted order"
            );
            assert!(backend.watched_paths.contains(path_a.as_ref()));
            assert!(
                !backend.watched_paths.contains(path_b.as_ref()),
                "the path with the generic error is dropped"
            );
            assert!(
                backend.watched_paths.contains(path_c.as_ref()),
                "the path after the error is still watched in the same pass"
            );
        }

        // A failed watch never registers in the backend, so it falls out of the
        // affected set and is not retried when a later pass runs for an unrelated
        // registration.
        global.add(
            Arc::<Path>::from(Path::new("/repo/d")),
            WatcherMode::Native,
            noop_callback(),
        );
        executor.run_until_parked();
        assert_eq!(
            backend.lock().watch_calls,
            &[
                path_a.to_path_buf(),
                path_b.to_path_buf(),
                path_c.to_path_buf(),
                PathBuf::from("/repo/d"),
            ],
            "the failed path is not retried on later passes"
        );
    }

    #[gpui::test]
    async fn batched_watch_limit_defers_the_failed_path_and_everything_after_it(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        let path_c = Arc::<Path>::from(Path::new("/repo/c"));
        backend
            .lock()
            .watch_errors
            .insert(path_b.to_path_buf(), watch_limit_error);

        let (events_a, callback_a) = collecting_callback();
        let (events_b, callback_b) = collecting_callback();
        let (events_c, callback_c) = collecting_callback();
        global.add(path_a.clone(), WatcherMode::Native, callback_a);
        global.add(path_b.clone(), WatcherMode::Native, callback_b);
        global.add(path_c.clone(), WatcherMode::Native, callback_c);
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert_eq!(
                backend.watch_calls,
                &[path_a.to_path_buf(), path_b.to_path_buf()],
                "the watch limit starts a cooldown, so the path after the failure is deferred without a syscall"
            );
            assert!(backend.watched_paths.contains(path_a.as_ref()));
            assert!(!backend.watched_paths.contains(path_b.as_ref()));
            assert!(!backend.watched_paths.contains(path_c.as_ref()));
        }

        backend.lock().watch_errors.clear();
        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert!(backend.watched_paths.contains(path_b.as_ref()));
            assert!(backend.watched_paths.contains(path_c.as_ref()));
        }
        assert!(
            events_a.lock().is_empty(),
            "the path watched on the first attempt does not get a rescan"
        );
        assert!(
            events_b.lock().is_empty(),
            "the deferred path is retried without a synthetic rescan"
        );
        assert!(
            events_c.lock().is_empty(),
            "the path deferred behind the failure is retried without a synthetic rescan"
        );
    }

    #[gpui::test]
    async fn watch_limit_deferred_child_stops_retrying_when_covered_by_parent(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((backend.clone(), RecursiveMode::Recursive)),
            None,
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/sub"));
        let (child_events, child_callback) = collecting_callback();
        backend
            .lock()
            .watch_errors
            .insert(child.to_path_buf(), watch_limit_error);

        global.add(child.clone(), WatcherMode::Native, child_callback);
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[child.to_path_buf()]);
        assert!(backend.lock().watched_paths.is_empty());

        global.add(parent.clone(), WatcherMode::Native, noop_callback());
        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
        executor.run_until_parked();

        assert!(backend.lock().watched_paths.contains(parent.as_ref()));
        assert!(!backend.lock().watched_paths.contains(child.as_ref()));
        assert!(child_events.lock().is_empty());

        backend.lock().watch_errors.clear();
        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
        executor.run_until_parked();

        assert!(backend.lock().watched_paths.contains(parent.as_ref()));
        assert!(!backend.lock().watched_paths.contains(child.as_ref()));
        assert!(child_events.lock().is_empty());
    }

    #[gpui::test]
    async fn events_are_dispatched_to_matching_mode_only(executor: BackgroundExecutor) {
        let native_backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let poll_backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((native_backend, platform_recursive_mode(WatcherMode::Native))),
            Some((poll_backend, platform_recursive_mode(WatcherMode::Poll))),
        );

        let (native_events, native_callback) = collecting_callback();
        let (poll_events, poll_callback) = collecting_callback();

        let native_id = global.add(
            Arc::<Path>::from(Path::new("/native")),
            WatcherMode::Native,
            native_callback,
        );
        global.add(
            Arc::<Path>::from(Path::new("/poll")),
            WatcherMode::Poll,
            poll_callback,
        );
        executor.run_until_parked();

        let event = Event::new(EventKind::Create(notify::event::CreateKind::File))
            .add_path(PathBuf::from("/native/file.txt"));
        handle_event(WatcherMode::Native, &global.state, Ok(event));

        assert_eq!(native_events.lock().len(), 1);
        assert_eq!(poll_events.lock().len(), 0);

        let access_event = Event::new(EventKind::Access(notify::event::AccessKind::Read))
            .add_path(PathBuf::from("/native/file.txt"));
        handle_event(WatcherMode::Native, &global.state, Ok(access_event));

        assert_eq!(native_events.lock().len(), 1);

        global.remove(native_id);
        let event = Event::new(EventKind::Create(notify::event::CreateKind::File))
            .add_path(PathBuf::from("/native/file.txt"));
        handle_event(WatcherMode::Native, &global.state, Ok(event));
        assert_eq!(native_events.lock().len(), 1);
    }

    #[gpui::test]
    async fn cooldown_defers_watches_without_further_syscalls(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        backend
            .lock()
            .watch_errors
            .insert(path_a.to_path_buf(), watch_limit_error);

        global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        global.add(path_b.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();

        assert_eq!(
            backend.lock().watch_calls,
            &[path_a.to_path_buf()],
            "the first failure starts the cooldown and the rest of the pass is skipped"
        );
    }

    #[gpui::test]
    async fn deferred_watches_recover_after_cooldown(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        backend
            .lock()
            .watch_errors
            .insert(path_a.to_path_buf(), watch_limit_error);

        let (events_a, callback_a) = collecting_callback();
        let (events_b, callback_b) = collecting_callback();
        global.add(path_a.clone(), WatcherMode::Native, callback_a);
        global.add(path_b.clone(), WatcherMode::Native, callback_b);
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls.len(), 1);

        backend.lock().watch_errors.clear();

        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert!(backend.watched_paths.contains(path_a.as_ref()));
            assert!(backend.watched_paths.contains(path_b.as_ref()));
        }
        assert!(
            events_a.lock().is_empty(),
            "recovered path does not get a synthetic rescan"
        );
        assert!(
            events_b.lock().is_empty(),
            "recovered path does not get a synthetic rescan"
        );
    }

    #[gpui::test]
    async fn unwatch_clears_the_cooldown(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        let first = global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(path_a.as_ref()));

        backend
            .lock()
            .watch_errors
            .insert(path_b.to_path_buf(), watch_limit_error);
        global.add(path_b.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert!(!backend.lock().watched_paths.contains(path_b.as_ref()));

        backend.lock().watch_errors.clear();
        global.remove(first);
        executor.run_until_parked();

        let backend = backend.lock();
        assert_eq!(backend.unwatch_calls, &[path_a.to_path_buf()]);
        assert!(
            backend.watched_paths.contains(path_b.as_ref()),
            "freeing a watch slot clears the cooldown and the deferred path is watched in the same pass"
        );
    }

    #[gpui::test]
    async fn unregistering_a_deferred_path_clears_it(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path = Arc::<Path>::from(Path::new("/repo/a"));
        backend
            .lock()
            .watch_errors
            .insert(path.to_path_buf(), watch_limit_error);

        // The watch limit defers the path behind a cooldown without establishing it.
        let first = global.add(path.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[path.to_path_buf()]);
        assert!(backend.lock().watched_paths.is_empty());

        // Drop the only registration while the path is still deferred, then clear
        // the error and let the cooldown expire so a re-registration can be
        // established immediately (not deferred again).
        global.remove(first);
        backend.lock().watch_errors.clear();
        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
        executor.run_until_parked();

        let (events, callback) = collecting_callback();
        global.add(path.clone(), WatcherMode::Native, callback);
        executor.run_until_parked();

        assert!(backend.lock().watched_paths.contains(path.as_ref()));
        assert!(
            events.lock().is_empty(),
            "unregistering removed the stale deferral, so the re-registered path is established as a fresh watch rather than recovered, and gets no rescan"
        );
    }

    #[gpui::test]
    async fn stream_restart_failure_reestablishes_all_watches(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        let (events_a, callback_a) = collecting_callback();
        global.add(path_a.clone(), WatcherMode::Native, callback_a);
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(path_a.as_ref()));

        backend.lock().stream_restart_error = Some(stream_restart_error);
        let (events_b, callback_b) = collecting_callback();
        global.add(path_b.clone(), WatcherMode::Native, callback_b);
        executor.run_until_parked();

        backend.lock().stream_restart_error = None;
        executor.advance_clock(*FILE_WATCHER_RETRY_DELAY + Duration::from_secs(1));
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert!(backend.watched_paths.contains(path_a.as_ref()));
            assert!(backend.watched_paths.contains(path_b.as_ref()));
        }
        let events_a = events_a.lock();
        let events_b = events_b.lock();
        assert_eq!(
            events_a.len(),
            1,
            "watch that predated the stream failure got a rescan after re-establishment"
        );
        assert!(events_a[0].need_rescan());
        assert_eq!(events_b.len(), 1);
        assert!(events_b[0].need_rescan());
    }

    #[gpui::test]
    async fn stream_restart_backend_rescan_reaches_covered_descendant(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            None,
            Some((backend.clone(), RecursiveMode::Recursive)),
        );
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/sub"));
        let other = Arc::<Path>::from(Path::new("/other"));
        let (tx, _rx) = async_channel::unbounded();
        let pending_path_events: Arc<Mutex<Vec<PathEvent>>> = Default::default();
        let child_callback = {
            let tx = tx.clone();
            let pending_path_events = pending_path_events.clone();
            let root_path = SanitizedPath::new_arc(child.as_ref());
            let child = child.clone();
            Arc::new(move |event: &notify::Event| {
                push_notify_event(&tx, &pending_path_events, &root_path, child.as_ref(), event)
            }) as Arc<dyn Fn(&notify::Event) + Send + Sync>
        };

        global.add(parent.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        global.add(child.clone(), WatcherMode::Poll, child_callback);
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[parent.to_path_buf()]);

        backend.lock().stream_restart_error = Some(stream_restart_error);
        global.add(other.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();

        backend.lock().stream_restart_error = None;
        executor.advance_clock(*FILE_WATCHER_RETRY_DELAY + Duration::from_secs(1));
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert!(backend.watched_paths.contains(parent.as_ref()));
            assert!(!backend.watched_paths.contains(child.as_ref()));
            assert!(backend.watched_paths.contains(other.as_ref()));
        }
        assert_eq!(
            std::mem::take(&mut *pending_path_events.lock()),
            vec![PathEvent {
                path: child.to_path_buf(),
                kind: Some(PathEventKind::Rescan),
            }]
        );
    }

    #[gpui::test]
    async fn stream_restart_backend_rescan_is_not_repeated_by_watch_limit_recovery(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((backend.clone(), RecursiveMode::Recursive)),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        let path_c = Arc::<Path>::from(Path::new("/repo/c"));
        let (events_a, callback_a) = collecting_callback();
        let (events_b, callback_b) = collecting_callback();
        let (events_c, callback_c) = collecting_callback();

        global.add(path_a.clone(), WatcherMode::Native, callback_a);
        executor.run_until_parked();

        backend.lock().stream_restart_error = Some(stream_restart_error);
        global.add(path_b.clone(), WatcherMode::Native, callback_b);
        global.add(path_c.clone(), WatcherMode::Native, callback_c);
        executor.run_until_parked();

        backend.lock().stream_restart_error = None;
        backend
            .lock()
            .watch_errors
            .insert(path_b.to_path_buf(), watch_limit_error);
        executor.advance_clock(*FILE_WATCHER_RETRY_DELAY + Duration::from_secs(1));
        executor.run_until_parked();

        assert_eq!(events_a.lock().len(), 1);
        assert_eq!(events_b.lock().len(), 1);
        assert_eq!(events_c.lock().len(), 1);
        assert!(events_a.lock()[0].paths.is_empty());
        assert!(events_a.lock()[0].need_rescan());

        backend.lock().watch_errors.clear();
        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
        executor.run_until_parked();

        assert_eq!(events_a.lock().len(), 1);
        assert_eq!(events_b.lock().len(), 1);
        assert_eq!(events_c.lock().len(), 1);
    }

    #[gpui::test]
    async fn stream_restart_failure_retry_delay_is_not_bypassed_by_wake(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        let path_c = Arc::<Path>::from(Path::new("/repo/c"));

        global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();

        backend.lock().stream_restart_error = Some(stream_restart_error);
        global.add(path_b.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();

        backend.lock().stream_restart_error = None;
        let watch_calls_before_wake = backend.lock().watch_calls.clone();
        global.add(path_c.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, watch_calls_before_wake);

        executor.advance_clock(*FILE_WATCHER_RETRY_DELAY + Duration::from_secs(1));
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(path_a.as_ref()));
        assert!(backend.lock().watched_paths.contains(path_b.as_ref()));
        assert!(backend.lock().watched_paths.contains(path_c.as_ref()));
    }

    #[gpui::test]
    async fn unwatch_skips_the_failed_path_and_unwatches_the_rest(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));
        let path_c = Arc::<Path>::from(Path::new("/repo/c"));

        let first = global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        let second = global.add(path_b.clone(), WatcherMode::Native, noop_callback());
        let third = global.add(path_c.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watched_paths.len(), 3);

        backend
            .lock()
            .unwatch_errors
            .insert(path_b.to_path_buf(), (generic_error, false));
        global.remove(first);
        global.remove(second);
        global.remove(third);
        executor.run_until_parked();

        let backend = backend.lock();
        assert_eq!(
            backend.unwatch_calls,
            &[
                path_a.to_path_buf(),
                path_b.to_path_buf(),
                path_c.to_path_buf(),
            ],
            "every unwatch in the batch is attempted, including the ones after the failure"
        );
        assert!(!backend.watched_paths.contains(path_a.as_ref()));
        assert!(
            backend.watched_paths.contains(path_b.as_ref()),
            "the path whose unwatch failed stays watched in the backend"
        );
        assert!(!backend.watched_paths.contains(path_c.as_ref()));
    }

    #[gpui::test]
    async fn unwatch_stream_restart_failure_reestablishes_surviving_watches(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        let first = global.add(path_a.clone(), WatcherMode::Native, noop_callback());
        let (events_b, callback_b) = collecting_callback();
        global.add(path_b.clone(), WatcherMode::Native, callback_b);
        executor.run_until_parked();
        assert_eq!(backend.lock().watched_paths.len(), 2);

        // Removing path_a triggers an unwatch whose stream fails to restart, so
        // every surviving watch is suspect and gets re-established.
        backend.lock().stream_restart_error = Some(stream_restart_error);
        global.remove(first);
        executor.run_until_parked();
        assert!(!backend.lock().watched_paths.contains(path_a.as_ref()));

        backend.lock().stream_restart_error = None;
        executor.advance_clock(*FILE_WATCHER_RETRY_DELAY + Duration::from_secs(1));
        executor.run_until_parked();

        assert!(backend.lock().watched_paths.contains(path_b.as_ref()));
        let events_b = events_b.lock();
        assert_eq!(
            events_b.len(),
            1,
            "the surviving watch is re-established with a rescan after the stream restart failure"
        );
        assert!(events_b[0].need_rescan());
    }

    #[gpui::test]
    async fn unregistering_a_suspect_path_during_cooldown_keeps_the_upper_bound_honest(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path = Arc::<Path>::from(Path::new("/repo"));

        // The initial watch applies in the backend, but the stream fails to
        // restart, so the path is left suspect (still watched in the backend) and
        // deferred behind a cooldown.
        backend.lock().stream_restart_error = Some(stream_restart_error);
        let registration = global.add(path.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(path.as_ref()));

        // Unregistering the path before the cooldown lets it recover must not drop
        // it from the suspect set while it is still leaked in the backend: doing so
        // would leave a path in `B` that is in neither `applied` nor `suspect`,
        // violating the hard upper bound `B ⊆ applied ∪ suspect`. We accept the
        // leak and keep tracking it.
        backend.lock().stream_restart_error = None;
        global.remove(registration);
        executor.run_until_parked();
        assert!(
            backend.lock().watched_paths.contains(path.as_ref()),
            "the leaked watch is still tracked as suspect, keeping the upper bound honest"
        );
    }

    #[gpui::test]
    async fn flush_resolves_after_reconcile(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let path = Arc::<Path>::from(Path::new("/repo"));

        global.add(path.clone(), WatcherMode::Native, noop_callback());
        let mut flushed = global.flush();
        executor.run_until_parked();

        assert_eq!(flushed.try_recv(), Ok(Some(())));
        assert!(backend.lock().watched_paths.contains(path.as_ref()));
    }

    #[gpui::test]
    async fn check_health_resolves_ok_when_native_backend_is_available(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global(
            &executor,
            Some((backend, platform_recursive_mode(WatcherMode::Native))),
            None,
        );

        let mut health = global.check_health();
        executor.run_until_parked();

        assert!(matches!(health.try_recv(), Ok(Some(Ok(())))));
    }

    fn test_sink(
        executor: &BackgroundExecutor,
        native_backend: FakeBackend,
        poll_backend: FakeBackend,
    ) -> (
        FsWatcher,
        async_channel::Receiver<()>,
        Arc<Mutex<Vec<PathEvent>>>,
    ) {
        let global = GlobalWatcher::with_backends(
            executor,
            fake_backend(native_backend, WatcherMode::Native),
            fake_backend(poll_backend, WatcherMode::Poll),
        );
        let (tx, rx) = async_channel::unbounded();
        let pending_path_events: Arc<Mutex<Vec<PathEvent>>> = Default::default();
        let sink = FsWatcher::new(global, executor.clone(), tx, pending_path_events.clone());
        (sink, rx, pending_path_events)
    }

    #[gpui::test]
    async fn sink_establishes_watch_for_existing_path(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let (sink, _rx, _pending) = test_sink(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let dir = tempfile::TempDir::new().expect("create temp dir");

        sink.add(dir.path()).expect("add succeeds");
        sink.add(dir.path()).expect("duplicate add succeeds");
        executor.run_until_parked();

        assert_eq!(backend.lock().watch_calls, &[dir.path().to_path_buf()]);

        drop(sink);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, &[dir.path().to_path_buf()]);
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn sink_waits_for_path_creation(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let (sink, rx, pending_path_events) = test_sink(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let dir = tempfile::TempDir::new().expect("create temp dir");
        let file = dir.path().join("missing.txt");

        sink.add(&file).expect("add succeeds");
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, Vec::<PathBuf>::new());

        std::fs::write(&file, "hello").expect("create file");
        executor.advance_clock(poll_interval() + Duration::from_millis(1));
        executor.run_until_parked();

        assert_eq!(backend.lock().watch_calls, &[file.clone()]);
        assert!(rx.try_recv().is_ok(), "consumer was signalled");
        // The promotion emits Created + Rescan, but enqueue_path_events merges by
        // path alone, so only one event per path survives a single batch. This
        // matches the old FsWatcher's pending-path promotion behavior exactly.
        assert_eq!(
            std::mem::take(&mut *pending_path_events.lock()),
            vec![PathEvent {
                path: file.clone(),
                kind: Some(PathEventKind::Created),
            }]
        );
    }

    #[gpui::test]
    async fn sink_remove_cancels_pending_watch(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let (sink, _rx, _pending) = test_sink(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let dir = tempfile::TempDir::new().expect("create temp dir");
        let file = dir.path().join("missing.txt");

        sink.add(&file).expect("add succeeds");
        executor.run_until_parked();

        sink.remove(&file).expect("remove succeeds");
        std::fs::write(&file, "hello").expect("create file");
        executor.advance_clock(poll_interval() + Duration::from_millis(1));
        executor.run_until_parked();

        let backend = backend.lock();
        assert_eq!(backend.watch_calls, Vec::<PathBuf>::new());
        assert_eq!(backend.unwatch_calls, Vec::<PathBuf>::new());
    }

    #[gpui::test]
    async fn sink_recovers_deferred_watch_without_rescan(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let (sink, _rx, pending_path_events) = test_sink(
            &executor,
            Some((
                backend.clone(),
                platform_recursive_mode(WatcherMode::Native),
            )),
            None,
        );
        let dir = tempfile::TempDir::new().expect("create temp dir");
        backend
            .lock()
            .watch_errors
            .insert(dir.path().to_path_buf(), watch_limit_error);

        sink.add(dir.path()).expect("add succeeds");
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls.len(), 1);
        assert!(pending_path_events.lock().is_empty());

        backend.lock().watch_errors.clear();

        executor.advance_clock(Duration::from_secs(1));
        executor.run_until_parked();
        assert_eq!(
            backend.lock().watch_calls.len(),
            1,
            "no retry happens before the cooldown expires"
        );

        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN);
        executor.run_until_parked();

        assert_eq!(backend.lock().watch_calls.len(), 2);
        assert!(
            backend
                .lock()
                .watched_paths
                .contains(&dir.path().to_path_buf())
        );
        assert!(pending_path_events.lock().is_empty());
    }
}
