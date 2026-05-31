use gpui::{BackgroundExecutor, Task};
use notify::{Event, EventKind};
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, HashMap},
    ops::DerefMut,
    path::Path,
    sync::{Arc, LazyLock, OnceLock},
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
        path_events.retain(|path_event| path_event.path != watched_root);
        path_events.push(PathEvent {
            path: watched_root.to_path_buf(),
            kind: Some(PathEventKind::Rescan),
        });
    }
    log::trace!("path_events: {:?}", path_events);
    enqueue_path_events(tx, pending_path_events, path_events);
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
}

impl<T: notify::Watcher + Send> WatchBackend for T {
    fn watch(&mut self, path: &Path, mode: notify::RecursiveMode) -> notify::Result<()> {
        notify::Watcher::watch(self, path, mode)
    }

    fn unwatch(&mut self, path: &Path) -> notify::Result<()> {
        notify::Watcher::unwatch(self, path)
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

        let watcher = notify::recommended_watcher(handle_native_event)?;
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

    #[derive(Default)]
    struct FakeWatchBackend {
        watched_paths: HashSet<PathBuf>,
        watch_calls: Vec<PathBuf>,
        unwatch_calls: Vec<PathBuf>,
        fail_with_watch_limit: bool,
    }

    struct SharedFakeWatchBackend(Arc<Mutex<FakeWatchBackend>>);

    impl WatchBackend for SharedFakeWatchBackend {
        fn watch(&mut self, path: &Path, _mode: notify::RecursiveMode) -> notify::Result<()> {
            let path = path.to_path_buf();
            let mut backend = self.0.lock();
            backend.watch_calls.push(path.clone());
            if backend.fail_with_watch_limit {
                return Err(notify::Error::new(notify::ErrorKind::MaxFilesWatch));
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
    }

    fn test_watcher(poll_watcher: Arc<Mutex<FakeWatchBackend>>) -> GlobalWatcher {
        test_watcher_with_backends(None, Some(poll_watcher))
    }

    fn test_watcher_with_backends(
        native_watcher: Option<Arc<Mutex<FakeWatchBackend>>>,
        poll_watcher: Option<Arc<Mutex<FakeWatchBackend>>>,
    ) -> GlobalWatcher {
        GlobalWatcher {
            state: Mutex::new(WatcherState {
                watchers: Default::default(),
                native_path_registrations: Default::default(),
                poll_path_registrations: Default::default(),
                cooldown_until: None,
                last_registration: Default::default(),
            }),
            native_watcher: Mutex::new(
                native_watcher.map(|watcher| {
                    Box::new(SharedFakeWatchBackend(watcher)) as Box<dyn WatchBackend>
                }),
            ),
            poll_watcher: Mutex::new(
                poll_watcher.map(|watcher| {
                    Box::new(SharedFakeWatchBackend(watcher)) as Box<dyn WatchBackend>
                }),
            ),
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
    fn covered_child_registration_is_not_unwatched_after_parent_is_removed() {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let watcher = test_watcher(backend.clone());
        let parent = Arc::<Path>::from(Path::new("/repo"));
        let child = Arc::<Path>::from(Path::new("/repo/foo.csproj"));

        let parent_registration = watcher
            .add(parent.as_ref().into(), WatcherMode::Poll, |_| {})
            .expect("add parent watch")
            .expect("parent watch registered");
        let child_registration = watcher
            .add(child.as_ref().into(), WatcherMode::Poll, |_| {})
            .expect("add covered child watch")
            .expect("child watch registered");

        watcher.remove(parent_registration);
        watcher.remove(child_registration);

        let backend = backend.lock();
        assert_eq!(backend.watch_calls, &[parent.to_path_buf()]);
        assert_eq!(backend.unwatch_calls, &[parent.to_path_buf()]);
    }

    #[test]
    fn native_watch_limit_cools_down_subsequent_native_registrations() {
        let native_backend = Arc::new(Mutex::new(FakeWatchBackend {
            fail_with_watch_limit: true,
            ..Default::default()
        }));
        let poll_backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let watcher = test_watcher_with_backends(Some(native_backend.clone()), Some(poll_backend));
        let first_path = Arc::<Path>::from(Path::new("/repo/first"));
        let second_path = Arc::<Path>::from(Path::new("/repo/second"));

        let first_registration = watcher
            .add(first_path.clone(), WatcherMode::Native, |_| {})
            .expect("native watch limit is handled");
        let second_registration = watcher
            .add(second_path, WatcherMode::Native, |_| {})
            .expect("native watch limit backoff is handled");

        assert!(first_registration.is_none());
        assert!(second_registration.is_none());

        let native_backend = native_backend.lock();
        assert_eq!(native_backend.watch_calls, &[first_path.to_path_buf()]);
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
}

pub fn global<T>(f: impl FnOnce(&GlobalWatcher) -> T) -> anyhow::Result<T> {
    let global_watcher = global_watcher();
    global_watcher.ensure_native_watcher()?;
    Ok(f(global_watcher))
}
