use futures::{FutureExt as _, channel::oneshot, select_biased};
use gpui::{BackgroundExecutor, Task};
use notify::{Event, EventKind};
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::DerefMut,
    path::{Path, PathBuf},
    sync::{
        Arc, LazyLock, OnceLock,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, Instant},
};
use util::{ResultExt, paths::SanitizedPath};

use crate::{PathEvent, PathEventKind, Watcher};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WatcherMode {
    #[default]
    Native,
    Poll,
}

pub struct FsWatcher {
    executor: BackgroundExecutor,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Arc<Mutex<BTreeMap<Arc<std::path::Path>, FsWatcherRegistration>>>,
    pending_registrations: Arc<Mutex<HashMap<Arc<std::path::Path>, Task<()>>>>,
}

#[derive(Clone, Copy)]
struct FsWatcherRegistration {
    id: WatcherRegistrationId,
    mode: WatcherMode,
}

impl FsWatcher {
    pub fn new(
        executor: BackgroundExecutor,
        tx: async_channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        Self {
            executor,
            tx,
            pending_path_events,
            registrations: Default::default(),
            pending_registrations: Default::default(),
        }
    }

    fn add_existing_path(&self, path: Arc<Path>) -> anyhow::Result<()> {
        let registration_path = path.clone();
        if let Some(registration) =
            register_existing_path(path, self.tx.clone(), self.pending_path_events.clone())?
        {
            self.registrations
                .lock()
                .insert(registration_path, registration);
        }
        Ok(())
    }

    fn add_pending_path(&self, path: Arc<Path>) {
        let mut pending_registrations = self.pending_registrations.lock();
        if pending_registrations.contains_key(path.as_ref()) {
            return;
        }

        let task = self.executor.spawn(poll_path_until_created(
            self.executor.clone(),
            path.clone(),
            self.tx.clone(),
            self.pending_path_events.clone(),
            self.registrations.clone(),
            self.pending_registrations.clone(),
        ));
        pending_registrations.insert(path, task);
    }
}

impl Drop for FsWatcher {
    fn drop(&mut self) {
        self.pending_registrations.lock().clear();

        let mut registrations = BTreeMap::new();
        {
            let old = &mut self.registrations.lock();
            std::mem::swap(old.deref_mut(), &mut registrations);
        }

        let global_watcher = global_watcher();
        for (_, registration) in registrations {
            global_watcher.remove(registration.id);
        }
    }
}

impl Watcher for FsWatcher {
    fn add(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("watcher add: {path:?}");

        let (path_is_covered_by_recursive_registration, path_is_already_watched) = {
            let registrations = self.registrations.lock();
            (
                path.ancestors().skip(1).any(|ancestor| {
                    registrations.get(ancestor).is_some_and(|registration| {
                        registration.mode == WatcherMode::Poll
                            || cfg!(any(target_os = "windows", target_os = "macos"))
                    })
                }),
                registrations.contains_key(path),
            )
        };

        if path_is_covered_by_recursive_registration {
            log::trace!("path to watch is covered by existing registration: {path:?}");
            return Ok(());
        }

        if path_is_already_watched {
            log::trace!("path to watch is already watched: {path:?}");
            return Ok(());
        }

        if self.pending_registrations.lock().contains_key(path) {
            log::trace!("path to watch is already pending: {path:?}");
            return Ok(());
        }

        let path: Arc<std::path::Path> = path.into();
        if std::fs::symlink_metadata(path.as_ref()).is_err() {
            self.add_pending_path(path);
            return Ok(());
        }

        self.add_existing_path(path)
    }

    fn remove(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("remove watched path: {path:?}");
        self.pending_registrations.lock().remove(path);

        let Some(registration) = self.registrations.lock().remove(path) else {
            return Ok(());
        };

        global_watcher().remove(registration.id);
        Ok(())
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

fn register_existing_path(
    path: Arc<Path>,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
) -> anyhow::Result<Option<FsWatcherRegistration>> {
    let mode = if requires_poll_watcher(path.as_ref()) {
        log::info!(
            "Using poll watcher ({}ms interval) for {}",
            poll_interval().as_millis(),
            path.display()
        );
        telemetry::event!("fs_watcher_poll", path = path.display().to_string());
        WatcherMode::Poll
    } else {
        WatcherMode::Native
    };
    let root_path = SanitizedPath::new_arc(path.as_ref());
    let path_for_callback = path.clone();
    let Some(registration_id) =
        global_watcher().add(path, mode, move |event: &notify::Event| {
            log::trace!("watcher received event: {event:?}");
            push_notify_event(
                &tx,
                &pending_path_events,
                &root_path,
                path_for_callback.as_ref(),
                event,
            );
        })?
    else {
        return Ok(None);
    };
    Ok(Some(FsWatcherRegistration {
        id: registration_id,
        mode,
    }))
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
    executor: BackgroundExecutor,
    path: Arc<Path>,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Arc<Mutex<BTreeMap<Arc<Path>, FsWatcherRegistration>>>,
    pending_registrations: Arc<Mutex<HashMap<Arc<Path>, Task<()>>>>,
) {
    loop {
        executor.timer(poll_interval()).await;

        if !pending_registrations.lock().contains_key(path.as_ref()) {
            return;
        }

        if smol::fs::symlink_metadata(path.as_ref()).await.is_err() {
            continue;
        }

        if registrations.lock().contains_key(path.as_ref()) {
            pending_registrations.lock().remove(path.as_ref());
            return;
        }

        match register_existing_path(path.clone(), tx.clone(), pending_path_events.clone()) {
            Ok(Some(registration)) => {
                {
                    let mut pending_registrations = pending_registrations.lock();
                    if pending_registrations.remove(path.as_ref()).is_none() {
                        global_watcher().remove(registration.id);
                        return;
                    }
                    registrations.lock().insert(path.clone(), registration);
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
            Ok(None) => {}
            Err(error) => {
                log::warn!("failed to watch newly-created path {path:?}: {error}; retrying");
            }
        }
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

struct PathRegistrationState {
    count: u32,
    has_os_watcher: bool,
}

struct WatcherState {
    watchers: HashMap<WatcherRegistrationId, WatcherRegistrationState>,
    native_path_registrations: HashMap<Arc<std::path::Path>, PathRegistrationState>,
    poll_path_registrations: HashMap<Arc<std::path::Path>, PathRegistrationState>,
    cooldown_until: Option<Instant>,
    last_registration: WatcherRegistrationId,
}

impl WatcherState {
    fn is_native_watch_limit_cooldown_active(&self) -> bool {
        self.cooldown_until
            .is_some_and(|cooldown_until| cooldown_until > Instant::now())
    }

    fn path_registrations(
        &mut self,
        mode: WatcherMode,
    ) -> &mut HashMap<Arc<std::path::Path>, PathRegistrationState> {
        match mode {
            WatcherMode::Native => &mut self.native_path_registrations,
            WatcherMode::Poll => &mut self.poll_path_registrations,
        }
    }

    fn remove_registration(
        &mut self,
        id: WatcherRegistrationId,
    ) -> Option<(Arc<std::path::Path>, WatcherMode)> {
        let registration_state = self.watchers.remove(&id)?;
        let path_registrations = self.path_registrations(registration_state.mode);
        let count = path_registrations.get_mut(&registration_state.path)?;
        count.count -= 1;
        if count.count != 0 {
            return None;
        }

        let was_actually_watched = count.has_os_watcher;
        path_registrations.remove(&registration_state.path);

        was_actually_watched.then_some((registration_state.path, registration_state.mode))
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

pub struct GlobalWatcher {
    state: Mutex<WatcherState>,

    // DANGER: never keep state lock while holding watcher lock
    // two mutexes because calling watcher.add triggers watcher.event, which needs watchers.
    native_watcher: Mutex<Option<Box<dyn WatchBackend>>>,
    poll_watcher: Mutex<Option<Box<dyn WatchBackend>>>,
}

impl GlobalWatcher {
    #[must_use]
    fn add(
        &self,
        path: Arc<std::path::Path>,
        mode: WatcherMode,
        cb: impl Fn(&notify::Event) + Send + Sync + 'static,
    ) -> anyhow::Result<Option<WatcherRegistrationId>> {
        let mut state = self.state.lock();
        let (path_already_covered, path_already_registered) = {
            let registrations_for_mode = state.path_registrations(mode);
            (
                path_already_covered(path.as_ref(), registrations_for_mode, mode),
                registrations_for_mode.contains_key(&path),
            )
        };

        if !path_already_covered && !path_already_registered {
            if mode == WatcherMode::Native && state.is_native_watch_limit_cooldown_active() {
                return Ok(None);
            }

            drop(state);
            match self.watch(&path, mode) {
                Ok(()) => {}
                Err(error) if mode == WatcherMode::Native && is_max_files_watch_error(&error) => {
                    self.start_native_watch_limit_cooldown(&path);
                    return Ok(None);
                }
                Err(error) => return Err(error),
            }
            state = self.state.lock();
        }

        let id = state.last_registration;
        state.last_registration = WatcherRegistrationId(id.0 + 1);

        let registration_state = WatcherRegistrationState {
            callback: Arc::new(cb),
            path: path.clone(),
            mode,
        };
        state.watchers.insert(id, registration_state);
        state
            .path_registrations(mode)
            .entry(path)
            .and_modify(|registration| registration.count += 1)
            .or_insert(PathRegistrationState {
                count: 1,
                has_os_watcher: !path_already_covered,
            });

        Ok(Some(id))
    }

    fn start_native_watch_limit_cooldown(&self, path: &Path) {
        let mut state = self.state.lock();
        let now = Instant::now();
        let should_log = !state.is_native_watch_limit_cooldown_active();
        state.cooldown_until = Some(now + *NATIVE_WATCH_LIMIT_COOLDOWN);
        if should_log {
            log::warn!(
                "OS file watch limit reached while watching {path:?}; skipping new native file watcher registrations for {} seconds",
                NATIVE_WATCH_LIMIT_COOLDOWN.as_secs()
            );
        }
    }

    pub fn remove(&self, id: WatcherRegistrationId) {
        let mut state = self.state.lock();
        let Some((path, mode)) = state.remove_registration(id) else {
            return;
        };
        drop(state);
        self.unwatch(&path, mode).log_err();
    }

    fn watch(&self, path: &Path, mode: WatcherMode) -> anyhow::Result<()> {
        match mode {
            WatcherMode::Native => {
                self.ensure_native_watcher()?;
                self.native_watcher
                    .lock()
                    .as_mut()
                    .expect("native watcher initialized")
                    .watch(
                        path,
                        if cfg!(any(target_os = "windows", target_os = "macos")) {
                            notify::RecursiveMode::Recursive
                        } else {
                            notify::RecursiveMode::NonRecursive
                        },
                    )?;
            }
            WatcherMode::Poll => {
                self.ensure_poll_watcher()?;
                self.poll_watcher
                    .lock()
                    .as_mut()
                    .expect("poll watcher initialized")
                    .watch(path, notify::RecursiveMode::Recursive)?;
            }
        }

        Ok(())
    }

    fn unwatch(&self, path: &Path, mode: WatcherMode) -> anyhow::Result<()> {
        match mode {
            WatcherMode::Native => {
                if let Some(watcher) = self.native_watcher.lock().as_mut() {
                    watcher.unwatch(path)?;
                }
            }
            WatcherMode::Poll => {
                if let Some(watcher) = self.poll_watcher.lock().as_mut() {
                    watcher.unwatch(path)?;
                }
            }
        }

        Ok(())
    }

    fn ensure_native_watcher(&self) -> anyhow::Result<()> {
        if self.native_watcher.lock().is_some() {
            return Ok(());
        }

        // CORE excludes Access events, which Zed discards anyway. Without this,
        // the default mask subscribes to inotify OPEN/CLOSE_* on Linux, so every
        // file read in a watched directory would queue events, increasing the
        // risk of queue overflows (and thus full rescans) under read-heavy
        // workloads like grep or language server indexing.
        let config = notify::Config::default().with_event_kinds(notify::EventKindMask::CORE);
        let watcher =
            <notify::RecommendedWatcher as notify::Watcher>::new(handle_native_event, config)?;
        *self.native_watcher.lock() = Some(Box::new(watcher));
        Ok(())
    }

    fn ensure_poll_watcher(&self) -> anyhow::Result<()> {
        if self.poll_watcher.lock().is_some() {
            return Ok(());
        }

        let config = notify::Config::default().with_poll_interval(*POLL_INTERVAL);
        let watcher = notify::PollWatcher::new(handle_poll_event, config)?;
        *self.poll_watcher.lock() = Some(Box::new(watcher));
        Ok(())
    }
}

fn path_already_covered(
    path: &Path,
    path_registrations: &HashMap<Arc<std::path::Path>, PathRegistrationState>,
    mode: WatcherMode,
) -> bool {
    (mode == WatcherMode::Poll || cfg!(any(target_os = "windows", target_os = "macos")))
        && path
            .ancestors()
            .skip(1)
            .any(|ancestor| path_registrations.contains_key(ancestor))
}

fn is_max_files_watch_error(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<notify::Error>()
        .is_some_and(|error| matches!(&error.kind, notify::ErrorKind::MaxFilesWatch))
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

pub fn poll_interval() -> Duration {
    *POLL_INTERVAL
}

static FS_WATCHER_INSTANCE: OnceLock<GlobalWatcher> = OnceLock::new();

fn global_watcher() -> &'static GlobalWatcher {
    FS_WATCHER_INSTANCE.get_or_init(|| GlobalWatcher {
        state: Mutex::new(WatcherState {
            watchers: Default::default(),
            native_path_registrations: Default::default(),
            poll_path_registrations: Default::default(),
            cooldown_until: None,
            last_registration: Default::default(),
        }),
        native_watcher: Mutex::new(None),
        poll_watcher: Mutex::new(None),
    })
}

fn handle_native_event(event: Result<notify::Event, notify::Error>) {
    handle_event(WatcherMode::Native, event);
}

fn handle_poll_event(event: Result<notify::Event, notify::Error>) {
    handle_event(WatcherMode::Poll, event);
}

fn handle_event(mode: WatcherMode, event: Result<notify::Event, notify::Error>) {
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
        let state = global_watcher().state.lock();
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

#[derive(Clone)]
pub(crate) struct GlobalWatcher2 {
    state: Arc<Mutex<WatcherState2>>,
    wake_tx: async_channel::Sender<()>,
    next_registration_id: Arc<AtomicU32>,
}

impl GlobalWatcher2 {
    pub(crate) fn new(executor: &BackgroundExecutor) -> Self {
        Self::with_backends(executor, None, None)
    }

    fn with_backends(
        executor: &BackgroundExecutor,
        native_backend: Option<Box<dyn WatchBackend>>,
        poll_backend: Option<Box<dyn WatchBackend>>,
    ) -> Self {
        let state = Arc::new(Mutex::new(WatcherState2 {
            registrations: HashMap::new(),
            native_desired_paths: BTreeMap::new(),
            poll_desired_paths: BTreeMap::new(),
            dirty_paths: Vec::new(),
            pending_flushes: Vec::new(),
        }));
        let (wake_tx, wake_rx) = async_channel::bounded(1);
        executor
            .spawn(
                Reconciler {
                    watcher_state: state.clone(),
                    executor: executor.clone(),
                    native: BackendState::new(
                        WatcherMode::Native,
                        executor.clone(),
                        native_backend,
                    ),
                    poll: BackendState::new(WatcherMode::Poll, executor.clone(), poll_backend),
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

    pub(crate) fn register(
        &self,
        path: Arc<Path>,
        mode: WatcherMode,
        callback: Arc<dyn Fn(&notify::Event) + Send + Sync>,
    ) -> WatcherRegistrationId {
        let id = WatcherRegistrationId(self.next_registration_id.fetch_add(1, Ordering::Relaxed));
        {
            let mut state = self.state.lock();
            state.registrations.insert(
                id,
                WatcherRegistrationState {
                    callback,
                    path: path.clone(),
                    mode,
                },
            );
            *state
                .desired_paths_mut(mode)
                .entry(path.clone())
                .or_insert(0) += 1;
            state.dirty_paths.push((mode, path));
        }
        self.request_sync();
        id
    }

    pub(crate) fn unregister(&self, id: WatcherRegistrationId) {
        {
            let mut state = self.state.lock();
            let Some(registration) = state.registrations.remove(&id) else {
                return;
            };
            let desired = state.desired_paths_mut(registration.mode);
            if let Some(count) = desired.get_mut(&registration.path) {
                *count -= 1;
                if *count == 0 {
                    desired.remove(&registration.path);
                }
            }
            state
                .dirty_paths
                .push((registration.mode, registration.path.clone()));
        }
        self.request_sync();
    }

    pub(crate) fn flush(&self) -> oneshot::Receiver<()> {
        let (flush_tx, flush_rx) = oneshot::channel();
        self.state.lock().pending_flushes.push(flush_tx);
        self.request_sync();
        flush_rx
    }

    fn request_sync(&self) {
        match self.wake_tx.try_send(()) {
            Ok(()) => {}
            Err(async_channel::TrySendError::Full(())) => {}
            Err(async_channel::TrySendError::Closed(())) => {
                log::warn!("file watcher reconciler is gone; dropping sync request");
            }
        }
    }
}

struct WatcherState2 {
    registrations: HashMap<WatcherRegistrationId, WatcherRegistrationState>,
    native_desired_paths: BTreeMap<Arc<Path>, u32>,
    poll_desired_paths: BTreeMap<Arc<Path>, u32>,
    dirty_paths: Vec<(WatcherMode, Arc<Path>)>,
    pending_flushes: Vec<oneshot::Sender<()>>,
}

impl WatcherState2 {
    fn desired_paths_mut(&mut self, mode: WatcherMode) -> &mut BTreeMap<Arc<Path>, u32> {
        match mode {
            WatcherMode::Native => &mut self.native_desired_paths,
            WatcherMode::Poll => &mut self.poll_desired_paths,
        }
    }

    fn desired_paths(&self, mode: WatcherMode) -> &BTreeMap<Arc<Path>, u32> {
        match mode {
            WatcherMode::Native => &self.native_desired_paths,
            WatcherMode::Poll => &self.poll_desired_paths,
        }
    }
}

struct BackendState {
    mode: WatcherMode,
    executor: BackgroundExecutor,
    backend: Option<Box<dyn WatchBackend>>,
    applied_paths: HashSet<Arc<Path>>,
    errored_paths: HashSet<Arc<Path>>,
    deferred_paths: HashSet<Arc<Path>>,
    cooldown_until: Option<Instant>,
}

struct Reconciler {
    watcher_state: Arc<Mutex<WatcherState2>>,
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
        {
            let state = self.watcher_state.clone();
            let mut state = state.lock();
            // Taking the flushes and the dirty list in one lock acquisition is what
            // guarantees a flush ack covers every edit that preceded the flush call.
            flushes = std::mem::take(&mut state.pending_flushes);
            let dirty = std::mem::take(&mut state.dirty_paths);
            for (mode, path) in dirty {
                let examine = match mode {
                    WatcherMode::Native => &mut native_affected_paths,
                    WatcherMode::Poll => &mut poll_affected_paths,
                };
                if recursive_mode(mode) == notify::RecursiveMode::Recursive {
                    for (descendant, _) in state.desired_paths(mode).range::<Path, _>((
                        std::ops::Bound::Excluded(path.as_ref()),
                        std::ops::Bound::Unbounded,
                    )) {
                        if !descendant.starts_with(&path) {
                            break;
                        }
                        examine.insert(descendant.clone());
                    }
                }
                examine.insert(path);
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

        (flushes, wake_at)
    }
}

impl BackendState {
    fn new(
        mode: WatcherMode,
        executor: BackgroundExecutor,
        backend: Option<Box<dyn WatchBackend>>,
    ) -> Self {
        Self {
            mode,
            executor,
            backend,
            applied_paths: HashSet::new(),
            errored_paths: HashSet::new(),
            deferred_paths: HashSet::new(),
            cooldown_until: None,
        }
    }

    fn reconcile(
        &mut self,
        affected_paths: HashSet<Arc<Path>>,
        watcher_state: &Arc<Mutex<WatcherState2>>,
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
            let desired = watcher_state.desired_paths(self.mode);
            for path in affected_paths {
                let in_desired = desired.contains_key(&path);
                if !in_desired {
                    self.errored_paths.remove(&path);
                    self.deferred_paths.remove(&path);
                }
                let covered = recursive_mode(self.mode) == notify::RecursiveMode::Recursive
                    && path
                        .ancestors()
                        .skip(1)
                        .any(|ancestor| desired.contains_key(ancestor));
                let should_watch = in_desired && !covered;
                let applied = self.applied_paths.contains(&path);
                if should_watch && !applied {
                    if !self.errored_paths.contains(&path) {
                        to_watch.push(path);
                    }
                } else if !should_watch && applied {
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

        self.verify_backend_watches();

        log::debug!(
            "fs watcher reconcile ({:?}): examined {affected_count}, watched {watch_count}, unwatched {unwatch_count}, {} applied total, took {:?}",
            self.mode,
            self.applied_paths.len(),
            started_at.elapsed(),
        );

        wake_at
    }

    // The backend may legitimately retain paths we have demoted to deferred:
    // after a stream-restart failure the path operations were applied even
    // though the error made us schedule re-establishment. Anything else is a
    // divergence bug in the reconciler's bookkeeping.
    fn verify_backend_watches(&self) {
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
        let missing = self
            .applied_paths
            .iter()
            .filter(|path| !backend_paths.contains(path.as_ref()))
            .collect::<Vec<_>>();
        let unexpected = backend_paths
            .iter()
            .filter(|path| {
                !self.applied_paths.contains(path.as_path())
                    && !self.deferred_paths.contains(path.as_path())
            })
            .collect::<Vec<_>>();
        if !missing.is_empty() || !unexpected.is_empty() {
            if cfg!(test) {
                panic!(
                    "fs watcher state diverged from {:?} backend: missing from backend: {missing:?}, unexpected in backend: {unexpected:?}",
                    self.mode,
                )
            } else {
                log::warn!(
                    "fs watcher state diverged from {:?} backend: missing from backend: {missing:?}, unexpected in backend: {unexpected:?}",
                    self.mode,
                );
            }
        }
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
                    let applied_count =
                        queue.len() - error.remaining.len() - usize::from(error.origin.is_some());
                    any_succeeded |= applied_count > 0;
                    if error.origin.is_none() {
                        restart_failure = Some(error.source);
                        break;
                    }
                    log::warn!(
                        "failed to unwatch {:?}: {:#}",
                        error.origin.as_ref().map(|op| op.as_path()),
                        error.source
                    );
                    queue.drain(..queue.len() - error.remaining.len());
                }
            }
        }

        if any_succeeded {
            self.cooldown_until = None;
        }
        restart_failure.map(|source| self.handle_stream_restart_failure(Vec::new(), &source))
    }

    fn apply_watches(
        &mut self,
        paths: Vec<Arc<Path>>,
        watcher_state: &Arc<Mutex<WatcherState2>>,
    ) -> Option<Instant> {
        let recursive_mode = recursive_mode(self.mode);
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
                        "failed to create file watcher backend for {:?}: {error:#}; will retry",
                        self.mode,
                    );
                    let retry_at = self.executor.now() + *NATIVE_WATCH_LIMIT_COOLDOWN;
                    self.deferred_paths.extend(queue);
                    return earliest(wake_at, Some(retry_at));
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
                    for path in queue {
                        self.mark_watched(path, watcher_state);
                    }
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
                        self.mark_watched(path, watcher_state);
                    }

                    let failed = rest.remove(0);
                    if self.mode == WatcherMode::Native
                        && matches!(error.source.kind, notify::ErrorKind::MaxFilesWatch)
                    {
                        self.start_cooldown(&failed);
                        self.deferred_paths.insert(failed);
                        wake_at = earliest(wake_at, self.cooldown_until);
                    } else {
                        log::warn!("failed to watch {failed:?}: {:#}", error.source);
                        self.errored_paths.insert(failed);
                    }
                    queue = rest;
                }
            }
        }
        wake_at
    }

    fn mark_watched(&mut self, path: Arc<Path>, watcher_state: &Mutex<WatcherState2>) {
        let recovered = self.deferred_paths.remove(&path);
        self.applied_paths.insert(path.clone());
        if recovered {
            self.emit_rescan(&path, watcher_state);
        }
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
        let applied = std::mem::take(&mut self.applied_paths);
        self.deferred_paths.extend(applied);
        self.deferred_paths.extend(pending);
        self.executor.now() + *NATIVE_WATCH_LIMIT_COOLDOWN
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

    fn emit_rescan(&self, path: &Arc<Path>, watcher_state: &Mutex<WatcherState2>) {
        let callbacks = {
            let watcher_state = watcher_state.lock();
            watcher_state
                .registrations
                .values()
                .filter(|registration| registration.mode == self.mode && registration.path == *path)
                .map(|registration| registration.callback.clone())
                .collect::<Vec<_>>()
        };
        let event = Event::new(EventKind::Other)
            .add_path(path.to_path_buf())
            .set_flag(notify::event::Flag::Rescan);
        for callback in callbacks {
            callback(&event);
        }
    }

    fn ensure_backend(
        &mut self,
        watcher_state: &Arc<Mutex<WatcherState2>>,
    ) -> anyhow::Result<&mut Box<dyn WatchBackend>> {
        if self.backend.is_none() {
            self.backend = Some(create_backend(self.mode, watcher_state.clone())?);
        }
        Ok(self.backend.as_mut().expect("backend was just initialized"))
    }
}

fn create_backend(
    mode: WatcherMode,
    watcher_state: Arc<Mutex<WatcherState2>>,
) -> anyhow::Result<Box<dyn WatchBackend>> {
    match mode {
        WatcherMode::Native => {
            let config = notify::Config::default().with_event_kinds(notify::EventKindMask::CORE);
            let watcher = <notify::RecommendedWatcher as notify::Watcher>::new(
                move |event: notify::Result<Event>| {
                    handle_event2(WatcherMode::Native, &watcher_state, event);
                },
                config,
            )?;
            Ok(Box::new(watcher))
        }
        WatcherMode::Poll => {
            let config = notify::Config::default().with_poll_interval(*POLL_INTERVAL);
            let watcher = notify::PollWatcher::new(
                move |event: notify::Result<Event>| {
                    handle_event2(WatcherMode::Poll, &watcher_state, event);
                },
                config,
            )?;
            Ok(Box::new(watcher))
        }
    }
}

fn handle_event2(mode: WatcherMode, shared: &Mutex<WatcherState2>, event: notify::Result<Event>) {
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
        let shared = shared.lock();
        shared
            .registrations
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

fn recursive_mode(mode: WatcherMode) -> notify::RecursiveMode {
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

pub(crate) struct FsWatcher2 {
    global: GlobalWatcher2,
    executor: BackgroundExecutor,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    paths: Arc<Mutex<BTreeMap<Arc<Path>, PathState>>>,
}

enum PathState {
    Pending {
        _task: Task<()>,
    },
    Registered {
        id: WatcherRegistrationId,
        mode: WatcherMode,
    },
}

impl FsWatcher2 {
    pub(crate) fn new(
        global: GlobalWatcher2,
        executor: BackgroundExecutor,
        tx: async_channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        Self {
            global,
            executor,
            tx,
            pending_path_events,
            paths: Arc::new(Mutex::new(BTreeMap::new())),
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

impl Watcher for FsWatcher2 {
    fn add(&self, path: &Path) -> anyhow::Result<()> {
        log::trace!("watcher add: {path:?}");
        let mut paths = self.paths.lock();

        let path_is_covered = path.ancestors().skip(1).any(|ancestor| {
            paths.get(ancestor).is_some_and(|entry| match entry {
                PathState::Pending { .. } => false,
                PathState::Registered { mode, .. } => {
                    recursive_mode(*mode) == notify::RecursiveMode::Recursive
                }
            })
        });
        if path_is_covered || paths.contains_key(path) {
            log::trace!("path to watch is covered or already requested: {path:?}");
            return Ok(());
        }

        let path: Arc<Path> = path.into();
        if std::fs::symlink_metadata(path.as_ref()).is_err() {
            let task = self.executor.spawn(watch_path_when_created(
                self.global.clone(),
                self.executor.clone(),
                path.clone(),
                self.make_callback(&path),
                Arc::downgrade(&self.paths),
                self.tx.clone(),
                self.pending_path_events.clone(),
            ));
            paths.insert(path, PathState::Pending { _task: task });
            return Ok(());
        }

        let mode = if requires_poll_watcher(&path) {
            WatcherMode::Poll
        } else {
            WatcherMode::Native
        };
        let id = self
            .global
            .register(path.clone(), mode, self.make_callback(&path));
        paths.insert(path, PathState::Registered { id, mode });
        Ok(())
    }

    fn remove(&self, path: &Path) -> anyhow::Result<()> {
        log::trace!("remove watched path: {path:?}");
        let entry = self.paths.lock().remove(path);
        if let Some(PathState::Registered { id, .. }) = entry {
            self.global.unregister(id);
        }
        Ok(())
    }
}

impl Drop for FsWatcher2 {
    fn drop(&mut self) {
        let entries = std::mem::take(&mut *self.paths.lock());
        for (_, entry) in entries {
            if let PathState::Registered { id, .. } = entry {
                self.global.unregister(id);
            }
        }
    }
}

async fn watch_path_when_created(
    global: GlobalWatcher2,
    executor: BackgroundExecutor,
    path: Arc<Path>,
    callback: Arc<dyn Fn(&notify::Event) + Send + Sync>,
    entries: std::sync::Weak<Mutex<BTreeMap<Arc<Path>, PathState>>>,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
) {
    loop {
        executor.timer(poll_interval()).await;

        if std::fs::symlink_metadata(path.as_ref()).is_err() {
            continue;
        }

        let Some(entries) = entries.upgrade() else {
            return;
        };
        {
            let mut entries = entries.lock();
            let Some(entry) = entries.get_mut(path.as_ref()) else {
                return;
            };
            if !matches!(entry, PathState::Pending { .. }) {
                return;
            }
            let mode = if requires_poll_watcher(&path) {
                WatcherMode::Poll
            } else {
                WatcherMode::Native
            };
            let id = global.register(path.clone(), mode, callback.clone());
            *entry = PathState::Registered { id, mode };
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

#[cfg(test)]
mod tests {
    use super::*;
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
        watch_error: Option<fn() -> notify::Error>,
        stream_restart_error: Option<fn() -> notify::Error>,
    }

    struct SharedFakeWatchBackend(Arc<Mutex<FakeWatchBackend>>);

    impl WatchBackend for SharedFakeWatchBackend {
        fn watch(&mut self, path: &Path, _mode: notify::RecursiveMode) -> notify::Result<()> {
            let path = path.to_path_buf();
            let mut backend = self.0.lock();
            backend.watch_calls.push(path.clone());
            if let Some(make_error) = backend.watch_error {
                return Err(make_error());
            }
            backend.watched_paths.insert(path);
            Ok(())
        }

        fn unwatch(&mut self, path: &Path) -> notify::Result<()> {
            let path = path.to_path_buf();
            let mut backend = self.0.lock();
            backend.unwatch_calls.push(path.clone());
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

    fn test_global_v2(
        executor: &BackgroundExecutor,
        native_backend: Option<Arc<Mutex<FakeWatchBackend>>>,
        poll_backend: Option<Arc<Mutex<FakeWatchBackend>>>,
    ) -> GlobalWatcher2 {
        GlobalWatcher2::with_backends(
            executor,
            native_backend
                .map(|backend| Box::new(SharedFakeWatchBackend(backend)) as Box<dyn WatchBackend>),
            poll_backend
                .map(|backend| Box::new(SharedFakeWatchBackend(backend)) as Box<dyn WatchBackend>),
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
    async fn v2_watch_and_unwatch_call_the_backend_once_per_path(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path = Arc::<Path>::from(Path::new("/repo"));

        let first = global.register(path.clone(), WatcherMode::Native, noop_callback());
        let second = global.register(path.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[PathBuf::from("/repo")]);

        global.unregister(first);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, Vec::<PathBuf>::new());

        global.unregister(second);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, &[PathBuf::from("/repo")]);
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn v2_covered_child_is_promoted_when_parent_is_unregistered(
        executor: BackgroundExecutor,
    ) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, None, Some(backend.clone()));
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/foo.csproj"));

        let parent_id = global.register(parent.clone(), WatcherMode::Poll, noop_callback());
        let child_id = global.register(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[parent.to_path_buf()]);

        global.unregister(parent_id);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, &[parent.to_path_buf()]);
        assert_eq!(
            backend.lock().watch_calls,
            &[parent.to_path_buf(), child.to_path_buf()],
            "covered child is promoted to its own watch once the parent goes away"
        );
        assert!(backend.lock().watched_paths.contains(child.as_ref()));

        global.unregister(child_id);
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn v2_child_is_demoted_when_covering_parent_is_registered(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, None, Some(backend.clone()));
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/sub"));

        global.register(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[child.to_path_buf()]);

        global.register(parent.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();

        let backend = backend.lock();
        assert_eq!(
            backend.watch_calls,
            &[child.to_path_buf(), parent.to_path_buf()]
        );
        assert_eq!(
            backend.unwatch_calls,
            &[child.to_path_buf()],
            "the child's own watch is removed once the recursive parent covers it"
        );
        assert!(backend.watched_paths.contains(parent.as_ref()));
        assert!(!backend.watched_paths.contains(child.as_ref()));
    }

    #[gpui::test]
    async fn v2_removing_a_covered_child_issues_no_unwatch(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, None, Some(backend.clone()));
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/foo.csproj"));

        let parent_id = global.register(parent.clone(), WatcherMode::Poll, noop_callback());
        let child_id = global.register(child.clone(), WatcherMode::Poll, noop_callback());
        executor.run_until_parked();
        assert_eq!(
            backend.lock().watch_calls,
            &[parent.to_path_buf()],
            "the covered child never gets its own OS watch"
        );

        global.unregister(child_id);
        executor.run_until_parked();
        assert_eq!(
            backend.lock().unwatch_calls,
            Vec::<PathBuf>::new(),
            "removing a covered child issues no OS unwatch, since it never had one"
        );
        assert!(backend.lock().watched_paths.contains(parent.as_ref()));

        global.unregister(parent_id);
        executor.run_until_parked();
        assert_eq!(backend.lock().unwatch_calls, &[parent.to_path_buf()]);
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn v2_unregister_then_register_same_path_coalesces(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path = Arc::<Path>::from(Path::new("/repo"));

        let first = global.register(path.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[path.to_path_buf()]);

        global.unregister(first);
        let second = global.register(path.clone(), WatcherMode::Native, noop_callback());
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

        global.unregister(second);
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.is_empty());
    }

    #[gpui::test]
    async fn v2_failed_watch_is_abandoned_until_reregistered(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend {
            watch_error: Some(generic_error),
            ..Default::default()
        }));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        let first = global.register(path_a.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls, &[path_a.to_path_buf()]);

        global.register(path_b.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert_eq!(
            backend.lock().watch_calls,
            &[path_a.to_path_buf(), path_b.to_path_buf()],
            "the abandoned path is not retried on later passes"
        );

        backend.lock().watch_error = None;
        global.unregister(first);
        executor.run_until_parked();

        global.register(path_a.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert!(
            backend.lock().watched_paths.contains(path_a.as_ref()),
            "re-registering after the path left desired state retries the watch"
        );
    }

    #[gpui::test]
    async fn v2_events_are_dispatched_to_matching_mode_only(executor: BackgroundExecutor) {
        let native_backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let poll_backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, Some(native_backend), Some(poll_backend));

        let (native_events, native_callback) = collecting_callback();
        let (poll_events, poll_callback) = collecting_callback();

        let native_id = global.register(
            Arc::<Path>::from(Path::new("/native")),
            WatcherMode::Native,
            native_callback,
        );
        global.register(
            Arc::<Path>::from(Path::new("/poll")),
            WatcherMode::Poll,
            poll_callback,
        );
        executor.run_until_parked();

        let event = Event::new(EventKind::Create(notify::event::CreateKind::File))
            .add_path(PathBuf::from("/native/file.txt"));
        handle_event2(WatcherMode::Native, &global.state, Ok(event));

        assert_eq!(native_events.lock().len(), 1);
        assert_eq!(poll_events.lock().len(), 0);

        let access_event = Event::new(EventKind::Access(notify::event::AccessKind::Read))
            .add_path(PathBuf::from("/native/file.txt"));
        handle_event2(WatcherMode::Native, &global.state, Ok(access_event));

        assert_eq!(native_events.lock().len(), 1);

        global.unregister(native_id);
        let event = Event::new(EventKind::Create(notify::event::CreateKind::File))
            .add_path(PathBuf::from("/native/file.txt"));
        handle_event2(WatcherMode::Native, &global.state, Ok(event));
        assert_eq!(native_events.lock().len(), 1);
    }

    #[gpui::test]
    async fn v2_cooldown_defers_watches_without_further_syscalls(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend {
            watch_error: Some(watch_limit_error),
            ..Default::default()
        }));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        global.register(path_a.clone(), WatcherMode::Native, noop_callback());
        global.register(path_b.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();

        assert_eq!(
            backend.lock().watch_calls,
            &[path_a.to_path_buf()],
            "the first failure starts the cooldown and the rest of the pass is skipped"
        );
    }

    #[gpui::test]
    async fn v2_deferred_watches_recover_after_cooldown(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend {
            watch_error: Some(watch_limit_error),
            ..Default::default()
        }));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        let (events_a, callback_a) = collecting_callback();
        let (events_b, callback_b) = collecting_callback();
        global.register(path_a.clone(), WatcherMode::Native, callback_a);
        global.register(path_b.clone(), WatcherMode::Native, callback_b);
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls.len(), 1);

        backend.lock().watch_error = None;

        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
        executor.run_until_parked();

        {
            let backend = backend.lock();
            assert!(backend.watched_paths.contains(path_a.as_ref()));
            assert!(backend.watched_paths.contains(path_b.as_ref()));
        }
        let events_a = events_a.lock();
        let events_b = events_b.lock();
        assert_eq!(events_a.len(), 1, "recovered path got a rescan event");
        assert!(events_a[0].need_rescan());
        assert_eq!(events_b.len(), 1, "recovered path got a rescan event");
        assert!(events_b[0].need_rescan());
    }

    #[gpui::test]
    async fn v2_unwatch_clears_the_cooldown(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        let first = global.register(path_a.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(path_a.as_ref()));

        backend.lock().watch_error = Some(watch_limit_error);
        global.register(path_b.clone(), WatcherMode::Native, noop_callback());
        executor.run_until_parked();
        assert!(!backend.lock().watched_paths.contains(path_b.as_ref()));

        backend.lock().watch_error = None;
        global.unregister(first);
        executor.run_until_parked();

        let backend = backend.lock();
        assert_eq!(backend.unwatch_calls, &[path_a.to_path_buf()]);
        assert!(
            backend.watched_paths.contains(path_b.as_ref()),
            "freeing a watch slot clears the cooldown and the deferred path is watched in the same pass"
        );
    }

    #[gpui::test]
    async fn v2_stream_restart_failure_reestablishes_all_watches(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path_a = Arc::<Path>::from(Path::new("/repo/a"));
        let path_b = Arc::<Path>::from(Path::new("/repo/b"));

        let (events_a, callback_a) = collecting_callback();
        global.register(path_a.clone(), WatcherMode::Native, callback_a);
        executor.run_until_parked();
        assert!(backend.lock().watched_paths.contains(path_a.as_ref()));

        backend.lock().stream_restart_error = Some(stream_restart_error);
        let (events_b, callback_b) = collecting_callback();
        global.register(path_b.clone(), WatcherMode::Native, callback_b);
        executor.run_until_parked();

        backend.lock().stream_restart_error = None;
        executor.advance_clock(*NATIVE_WATCH_LIMIT_COOLDOWN + Duration::from_secs(1));
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
    async fn v2_flush_resolves_after_reconcile(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let global = test_global_v2(&executor, Some(backend.clone()), None);
        let path = Arc::<Path>::from(Path::new("/repo"));

        global.register(path.clone(), WatcherMode::Native, noop_callback());
        let mut flushed = global.flush();
        executor.run_until_parked();

        assert_eq!(flushed.try_recv(), Ok(Some(())));
        assert!(backend.lock().watched_paths.contains(path.as_ref()));
    }

    fn test_sink(
        executor: &BackgroundExecutor,
        native_backend: Option<Arc<Mutex<FakeWatchBackend>>>,
        poll_backend: Option<Arc<Mutex<FakeWatchBackend>>>,
    ) -> (
        FsWatcher2,
        async_channel::Receiver<()>,
        Arc<Mutex<Vec<PathEvent>>>,
    ) {
        let global = GlobalWatcher2::with_backends(
            executor,
            native_backend
                .map(|backend| Box::new(SharedFakeWatchBackend(backend)) as Box<dyn WatchBackend>),
            poll_backend
                .map(|backend| Box::new(SharedFakeWatchBackend(backend)) as Box<dyn WatchBackend>),
        );
        let (tx, rx) = async_channel::unbounded();
        let pending_path_events: Arc<Mutex<Vec<PathEvent>>> = Default::default();
        let sink = FsWatcher2::new(global, executor.clone(), tx, pending_path_events.clone());
        (sink, rx, pending_path_events)
    }

    #[gpui::test]
    async fn v2_sink_establishes_watch_for_existing_path(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let (sink, _rx, _pending) = test_sink(&executor, Some(backend.clone()), None);
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
    async fn v2_sink_waits_for_path_creation(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let (sink, rx, pending_path_events) = test_sink(&executor, Some(backend.clone()), None);
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
    async fn v2_sink_remove_cancels_pending_watch(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let (sink, _rx, _pending) = test_sink(&executor, Some(backend.clone()), None);
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
    async fn v2_sink_recovers_deferred_watch_and_emits_rescan(executor: BackgroundExecutor) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend {
            watch_error: Some(watch_limit_error),
            ..Default::default()
        }));
        let (sink, _rx, pending_path_events) = test_sink(&executor, Some(backend.clone()), None);
        let dir = tempfile::TempDir::new().expect("create temp dir");

        sink.add(dir.path()).expect("add succeeds");
        executor.run_until_parked();
        assert_eq!(backend.lock().watch_calls.len(), 1);
        assert!(pending_path_events.lock().is_empty());

        backend.lock().watch_error = None;

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
        assert_eq!(
            std::mem::take(&mut *pending_path_events.lock()),
            vec![PathEvent {
                path: dir.path().to_path_buf(),
                kind: Some(PathEventKind::Rescan),
            }]
        );
    }
}

pub fn global<T>(f: impl FnOnce(&GlobalWatcher) -> T) -> anyhow::Result<T> {
    let global_watcher = global_watcher();
    global_watcher.ensure_native_watcher()?;
    Ok(f(global_watcher))
}
