use gpui::{BackgroundExecutor, Task};
use notify::{Event, EventKind};
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    fs,
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
    registrations: Arc<Mutex<HashMap<WatchKey, FsWatcherRegistration>>>,
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
        let case_insensitive = case_insensitive_path(&path);
        let key = WatchKey::for_registration(SanitizedPath::new(&path), case_insensitive);
        if self.registrations.lock().contains_key(&key) {
            log::trace!("path to watch is already watched: {path:?}");
            return Ok(());
        }
        if let Some(registration) = register_existing_path(
            path,
            case_insensitive,
            self.tx.clone(),
            self.pending_path_events.clone(),
        )? {
            self.registrations.lock().insert(key, registration);
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

        let mut registrations = HashMap::new();
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

        let path: Arc<Path> = path.into();
        if path_covered_by_recursive_registration(
            &self.registrations.lock(),
            SanitizedPath::new(&path),
        ) {
            log::trace!("path to watch is covered by an existing registration: {path:?}");
            return Ok(());
        }

        if self
            .pending_registrations
            .lock()
            .contains_key(path.as_ref())
        {
            log::trace!("path to watch is already pending: {path:?}");
            return Ok(());
        }

        if fs::symlink_metadata(path.as_ref()).is_err() {
            self.add_pending_path(path);
            return Ok(());
        }

        self.add_existing_path(path)
    }

    fn remove(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("remove watched path: {path:?}");
        self.pending_registrations.lock().remove(path);

        let sanitized = SanitizedPath::new(path);
        let registration = {
            let mut registrations = self.registrations.lock();
            registrations
                .remove(&WatchKey::exact(sanitized))
                .or_else(|| registrations.remove(&WatchKey::folded(sanitized)))
        };
        if let Some(registration) = registration {
            global_watcher().remove(registration.id);
        }
        Ok(())
    }
}

/// Whether a recursive registration on a strict ancestor of `path` already covers
/// it. Both key spellings are probed so a folded registration still matches; only
/// poll watches and native macOS/Windows watches are recursive.
fn path_covered_by_recursive_registration(
    registrations: &HashMap<WatchKey, FsWatcherRegistration>,
    path: &SanitizedPath,
) -> bool {
    path.as_path().ancestors().skip(1).any(|ancestor| {
        let ancestor = SanitizedPath::unchecked_new(ancestor);
        [WatchKey::exact(ancestor), WatchKey::folded(ancestor)]
            .iter()
            .any(|key| {
                registrations.get(key).is_some_and(|registration| {
                    registration.mode == WatcherMode::Poll
                        || cfg!(any(target_os = "windows", target_os = "macos"))
                })
            })
    })
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
    case_insensitive: bool,
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
    let Some(registration_id) = global_watcher().add(
        path,
        mode,
        case_insensitive,
        move |event: &notify::Event| {
            log::trace!("watcher received event: {event:?}");
            push_notify_event(
                &tx,
                &pending_path_events,
                &root_path,
                case_insensitive,
                path_for_callback.as_ref(),
                event,
            );
        },
    )?
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

/// Whether the volume backing `path` does case-insensitive name lookups, used to
/// pick exact vs. folded matching.
#[cfg(target_os = "macos")]
fn case_insensitive_path(path: &Path) -> bool {
    use std::os::unix::ffi::OsStrExt as _;

    // `pathconf(_PC_CASE_SENSITIVE)` returns 1 (sensitive), 0 (insensitive), or -1
    // on error; default errors to insensitive (the APFS/HFS+ default).
    let Ok(c_path) = std::ffi::CString::new(path.as_os_str().as_bytes()) else {
        return true;
    };
    // SAFETY: We just initialized c_path, so it's a valid pointer
    unsafe { libc::pathconf(c_path.as_ptr(), libc::_PC_CASE_SENSITIVE) == 0 }
}

#[cfg(target_os = "linux")]
fn case_insensitive_path(_path: &Path) -> bool {
    // use std::os::unix::ffi::OsStrExt as _;

    // // Only ext4/f2fs casefold (`+F`) dirs are insensitive, reported by `statx` via
    // // STATX_ATTR_CASEFOLD; any failure (e.g. pre-4.11 ENOSYS) means case-sensitive.
    // const STATX_ATTR_CASEFOLD: u64 = 0x0000_2000;
    // let Ok(c_path) = std::ffi::CString::new(path.as_os_str().as_bytes()) else {
    //     return false;
    // };
    // let mut buf = std::mem::MaybeUninit::<libc::statx>::zeroed();

    // // SAFETY: c_path is still valid, buffer has been zeroed
    // if unsafe { libc::statx(libc::AT_FDCWD, c_path.as_ptr(), 0, 0, buf.as_mut_ptr()) } != 0 {
    //     return false;
    // }

    // // SAFETY: libc statx initialized this buffer, otherwise we would've returned on a error
    // // in that function call
    // let buf = unsafe { buf.assume_init() };
    // buf.stx_attributes_mask & STATX_ATTR_CASEFOLD != 0
    //     && buf.stx_attributes & STATX_ATTR_CASEFOLD != 0
    false
}

#[cfg(target_os = "windows")]
fn case_insensitive_path(_path: &Path) -> bool {
    // todo(windows): Windows defaults to case in sensitive, but
    // they can mark specific directories as case sensitive. Mainly
    // for WSL use cases
    true
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn case_insensitive_path(_path: &Path) -> bool {
    // Other BSDs default to case-sensitive local filesystems.
    false
}

/// Whether `path` is `root` or sits beneath it, folding case on case-insensitive
/// volumes so a differently-cased spelling still matches.
fn path_is_under(path: &SanitizedPath, root: &SanitizedPath, case_insensitive: bool) -> bool {
    if case_insensitive {
        let path = path.as_path().to_string_lossy().to_lowercase();
        let root = root.as_path().to_string_lossy().to_lowercase();
        Path::new(&path).starts_with(Path::new(&root))
    } else {
        path.starts_with(root)
    }
}

/// Lookup key for a watch path, shared by add, remove, and dispatch so they all
/// agree on whether two spellings denote the same directory.
///
/// On case-sensitive volumes the exact (sanitized) path is the key, so genuinely
/// distinct directories stay distinct. On case-insensitive volumes the folded
/// (lowercased) spelling is the key, so any casing of a directory collides.
///
/// The two variants are distinct map keys, so a case-sensitive registration can
/// never be hit by a folded lookup (or vice versa); dispatch can therefore probe
/// both forms of an event path without risking a cross-rule false match.
///
/// NOTE: folding only normalizes case. macOS (APFS/HFS+) is also Unicode
/// normalization-insensitive (NFC vs NFD); that normalization is intentionally
/// centralized here so it can be added in one place later without touching the
/// add/remove/dispatch call sites.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum WatchKey {
    Exact(Arc<Path>),
    Folded(Arc<str>),
}

impl WatchKey {
    fn exact(path: &SanitizedPath) -> Self {
        Self::Exact(Arc::from(path.as_path()))
    }

    fn folded(path: &SanitizedPath) -> Self {
        let lossy = path.as_path().to_string_lossy();
        // macOS (APFS/HFS+) compares names normalization-insensitively (NFC vs
        // NFD), and FSEvents can report NFD while a config/LSP supplies NFC, so
        // normalize before folding case. Windows (NTFS) and Linux are
        // normalization-sensitive, so there we only fold case.
        #[cfg(target_os = "macos")]
        let folded = {
            use unicode_normalization::UnicodeNormalization as _;
            lossy.chars().nfc().collect::<String>().to_lowercase()
        };
        #[cfg(not(target_os = "macos"))]
        let folded = lossy.to_lowercase();
        Self::Folded(folded.into())
    }

    fn for_registration(path: &SanitizedPath, case_insensitive: bool) -> Self {
        if case_insensitive {
            Self::folded(path)
        } else {
            Self::exact(path)
        }
    }
}

async fn poll_path_until_created(
    executor: BackgroundExecutor,
    path: Arc<Path>,
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Arc<Mutex<HashMap<WatchKey, FsWatcherRegistration>>>,
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

        // Probe case sensitivity now that the path exists, rather than at add
        // time when it didn't.
        let case_insensitive = case_insensitive_path(path.as_ref());
        let key = WatchKey::for_registration(SanitizedPath::new(&path), case_insensitive);

        if registrations.lock().contains_key(&key) {
            pending_registrations.lock().remove(path.as_ref());
            return;
        }

        match register_existing_path(
            path.clone(),
            case_insensitive,
            tx.clone(),
            pending_path_events.clone(),
        ) {
            Ok(Some(registration)) => {
                {
                    let mut pending_registrations = pending_registrations.lock();
                    if pending_registrations.remove(path.as_ref()).is_none() {
                        global_watcher().remove(registration.id);
                        return;
                    }
                    registrations.lock().insert(key, registration);
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
    case_insensitive: bool,
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
            path_is_under(event_path, root_path, case_insensitive).then(|| PathEvent {
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
    key: WatchKey,
    path: Arc<SanitizedPath>,
    mode: WatcherMode,
}

struct PathRegistrationState {
    watcher_ids: Vec<WatcherRegistrationId>,
    has_os_watcher: bool,
}

/// The registered watch paths for one watcher mode, keyed by [`WatchKey`] so that
/// add (dedup), remove, and dispatch share a single notion of path identity.
#[derive(Default)]
struct WatchPaths(HashMap<WatchKey, PathRegistrationState>);

impl WatchPaths {
    fn contains(&self, key: &WatchKey) -> bool {
        self.0.contains_key(key)
    }

    fn get_mut(&mut self, key: &WatchKey) -> Option<&mut PathRegistrationState> {
        self.0.get_mut(key)
    }

    fn entry(
        &mut self,
        key: WatchKey,
    ) -> std::collections::hash_map::Entry<'_, WatchKey, PathRegistrationState> {
        self.0.entry(key)
    }

    fn remove(&mut self, key: &WatchKey) {
        self.0.remove(key);
    }

    /// True if a recursive registration on a strict ancestor already covers
    /// `path`. Only poll watches and native macOS/Windows watches are recursive.
    fn covered_by_recursive_ancestor(&self, path: &SanitizedPath, mode: WatcherMode) -> bool {
        if mode != WatcherMode::Poll && !cfg!(any(target_os = "windows", target_os = "macos")) {
            return false;
        }
        path.as_path().ancestors().skip(1).any(|ancestor| {
            let ancestor = SanitizedPath::unchecked_new(ancestor);
            self.0.contains_key(&WatchKey::exact(ancestor))
                || self.0.contains_key(&WatchKey::folded(ancestor))
        })
    }

    /// Collects the watcher ids of every registration whose directory is an
    /// ancestor of (or equal to) `path`. Both exact and folded keys are probed,
    /// so a real-cased event path matches a folded registration and vice versa.
    fn watcher_ids_covering(&self, path: &SanitizedPath, ids: &mut Vec<WatcherRegistrationId>) {
        for ancestor in path.as_path().ancestors() {
            let ancestor = SanitizedPath::unchecked_new(ancestor);
            if let Some(registration) = self.0.get(&WatchKey::exact(ancestor)) {
                ids.extend_from_slice(&registration.watcher_ids);
            }
            if let Some(registration) = self.0.get(&WatchKey::folded(ancestor)) {
                ids.extend_from_slice(&registration.watcher_ids);
            }
        }
    }
}

struct WatcherState {
    watchers: HashMap<WatcherRegistrationId, WatcherRegistrationState>,
    native_path_registrations: WatchPaths,
    poll_path_registrations: WatchPaths,
    cooldown_until: Option<Instant>,
    last_registration: WatcherRegistrationId,
}

impl WatcherState {
    fn is_native_watch_limit_cooldown_active(&self) -> bool {
        self.cooldown_until
            .is_some_and(|cooldown_until| cooldown_until > Instant::now())
    }

    fn path_registrations(&mut self, mode: WatcherMode) -> &mut WatchPaths {
        match mode {
            WatcherMode::Native => &mut self.native_path_registrations,
            WatcherMode::Poll => &mut self.poll_path_registrations,
        }
    }

    fn remove_registration(
        &mut self,
        id: WatcherRegistrationId,
    ) -> Option<(Arc<SanitizedPath>, WatcherMode)> {
        let registration_state = self.watchers.remove(&id)?;
        let path_registrations = self.path_registrations(registration_state.mode);
        let path_state = path_registrations.get_mut(&registration_state.key)?;
        path_state.watcher_ids.retain(|&existing| existing != id);
        if !path_state.watcher_ids.is_empty() {
            return None;
        }

        let was_actually_watched = path_state.has_os_watcher;
        path_registrations.remove(&registration_state.key);

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

type DispatchEvent = (WatcherMode, Result<notify::Event, notify::Error>);

pub struct GlobalWatcher {
    state: Mutex<WatcherState>,

    // DANGER: never keep state lock while holding watcher lock
    // two mutexes because calling watcher.add triggers watcher.event, which needs watchers.
    native_watcher: Mutex<Option<Box<dyn WatchBackend>>>,
    poll_watcher: Mutex<Option<Box<dyn WatchBackend>>>,
    event_tx: async_channel::Sender<DispatchEvent>,
}

impl GlobalWatcher {
    #[must_use]
    fn add(
        &self,
        path: Arc<std::path::Path>,
        mode: WatcherMode,
        case_insensitive: bool,
        cb: impl Fn(&notify::Event) + Send + Sync + 'static,
    ) -> anyhow::Result<Option<WatcherRegistrationId>> {
        let path = SanitizedPath::from_arc(path);
        let key = WatchKey::for_registration(&path, case_insensitive);
        let mut state = self.state.lock();
        let (path_already_covered, path_already_registered) = {
            let registrations_for_mode = state.path_registrations(mode);
            (
                registrations_for_mode.covered_by_recursive_ancestor(&path, mode),
                registrations_for_mode.contains(&key),
            )
        };

        if !path_already_covered && !path_already_registered {
            if mode == WatcherMode::Native && state.is_native_watch_limit_cooldown_active() {
                return Ok(None);
            }

            drop(state);
            match self.watch(path.as_path(), mode) {
                Ok(()) => {}
                Err(error) if mode == WatcherMode::Native && is_max_files_watch_error(&error) => {
                    self.start_native_watch_limit_cooldown(path.as_path());
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
            key: key.clone(),
            path,
            mode,
        };
        state.watchers.insert(id, registration_state);
        state
            .path_registrations(mode)
            .entry(key)
            .and_modify(|registration| registration.watcher_ids.push(id))
            .or_insert_with(|| PathRegistrationState {
                watcher_ids: vec![id],
                has_os_watcher: !path_already_covered,
            });

        Ok(Some(id))
    }

    fn enqueue(&self, mode: WatcherMode, event: Result<notify::Event, notify::Error>) {
        if matches!(
            event,
            Ok(Event {
                kind: EventKind::Access(_),
                ..
            })
        ) {
            return;
        }

        // A failed send only happens once the dispatch thread has shut down, at
        // which point there's nothing left to dispatch to.
        self.event_tx.try_send((mode, event)).ok();
    }

    fn dispatch(&self, mode: WatcherMode, event: Result<notify::Event, notify::Error>) {
        let event = match event {
            Ok(event) => event,
            Err(error) => {
                log::warn!("watcher error for {mode:?}: {error}");
                return;
            }
        };

        log::trace!("global handle event for {mode:?}: {event:?}");

        let callbacks = {
            let state = self.state.lock();
            if event.need_rescan() {
                let callbacks = state
                    .watchers
                    .values()
                    .filter(|registration| registration.mode == mode)
                    .map(|registration| registration.callback.clone())
                    .collect::<Vec<_>>();
                log::warn!(
                    "filesystem watcher lost sync for {mode:?}; scheduling rescans for {} registrations",
                    callbacks.len()
                );
                callbacks
            } else {
                let path_registrations = match mode {
                    WatcherMode::Native => &state.native_path_registrations,
                    WatcherMode::Poll => &state.poll_path_registrations,
                };
                let mut ids = Vec::new();
                for path in &event.paths {
                    let sanitized = SanitizedPath::new(path);
                    path_registrations.watcher_ids_covering(sanitized, &mut ids);
                }
                ids.sort_unstable_by_key(|id| id.0);
                ids.dedup();
                ids.into_iter()
                    .filter_map(|id| state.watchers.get(&id))
                    .map(|registration| registration.callback.clone())
                    .collect::<Vec<_>>()
            }
        };

        for callback in callbacks {
            callback(&event);
        }
    }

    fn dispatch_batch(
        &self,
        first: DispatchEvent,
        event_rx: &async_channel::Receiver<DispatchEvent>,
    ) {
        // A single backend overflow can enqueue many rescan markers. One rescan
        // per mode covers the entire drained batch; ordinary events still run.
        let mut native_rescan_dispatched = false;
        let mut poll_rescan_dispatched = false;

        for (mode, event) in
            std::iter::once(first).chain(std::iter::from_fn(|| event_rx.try_recv().ok()))
        {
            let rescan_dispatched = match mode {
                WatcherMode::Native => &mut native_rescan_dispatched,
                WatcherMode::Poll => &mut poll_rescan_dispatched,
            };
            if event.as_ref().is_ok_and(notify::Event::need_rescan) {
                if *rescan_dispatched {
                    continue;
                }
                *rescan_dispatched = true;
            }

            self.dispatch(mode, event);
        }
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
        self.unwatch(path.as_path(), mode).log_err();
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
        let watcher = match mode {
            WatcherMode::Native => self
                .native_watcher
                .lock()
                .as_mut()
                .map(|watcher| watcher.unwatch(path)),
            WatcherMode::Poll => self
                .poll_watcher
                .lock()
                .as_mut()
                .map(|watcher| watcher.unwatch(path)),
        };

        match watcher {
            // inotify auto-removes a watch when its directory is deleted, so a
            // later unwatch races that and fails with a benign error. Either way
            // the path is no longer watched, which is all we wanted.
            Some(Err(error)) if !matches!(error.kind, notify::ErrorKind::WatchNotFound) => {
                Err(error.into())
            }
            _ => Ok(()),
        }
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
        let watcher = <notify::RecommendedWatcher as notify::Watcher>::new(
            |event| global_watcher().enqueue(WatcherMode::Native, event),
            config,
        )?;
        *self.native_watcher.lock() = Some(Box::new(watcher));
        Ok(())
    }

    fn ensure_poll_watcher(&self) -> anyhow::Result<()> {
        if self.poll_watcher.lock().is_some() {
            return Ok(());
        }

        let config = notify::Config::default().with_poll_interval(*POLL_INTERVAL);
        let watcher = notify::PollWatcher::new(
            |event| global_watcher().enqueue(WatcherMode::Poll, event),
            config,
        )?;
        *self.poll_watcher.lock() = Some(Box::new(watcher));
        Ok(())
    }
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
    FS_WATCHER_INSTANCE.get_or_init(|| {
        let (event_tx, event_rx) = async_channel::unbounded::<DispatchEvent>();
        std::thread::Builder::new()
            .name("fs-watcher-dispatch".to_owned())
            .spawn(move || {
                while let Ok(first) = event_rx.recv_blocking() {
                    global_watcher().dispatch_batch(first, &event_rx);
                }
            })
            .expect("failed to spawn fs watcher dispatch thread");
        GlobalWatcher {
            state: Mutex::new(WatcherState {
                watchers: Default::default(),
                native_path_registrations: Default::default(),
                poll_path_registrations: Default::default(),
                cooldown_until: None,
                last_registration: Default::default(),
            }),
            native_watcher: Mutex::new(None),
            poll_watcher: Mutex::new(None),
            event_tx,
        }
    })
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
        // Tests call `handle_event` directly to exercise dispatch synchronously,
        // rather than going through the OS watcher callbacks, so nothing is ever
        // sent on this channel; the receiver can just be dropped.
        let (event_tx, _event_rx) = async_channel::unbounded();
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
            event_tx,
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
            .add(parent.as_ref().into(), WatcherMode::Poll, false, |_| {})
            .expect("add parent watch")
            .expect("parent watch registered");
        let child_registration = watcher
            .add(child.as_ref().into(), WatcherMode::Poll, false, |_| {})
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
            .add(first_path.clone(), WatcherMode::Native, false, |_| {})
            .expect("native watch limit is handled");
        let second_registration = watcher
            .add(second_path, WatcherMode::Native, false, |_| {})
            .expect("native watch limit backoff is handled");

        assert!(first_registration.is_none());
        assert!(second_registration.is_none());

        let native_backend = native_backend.lock();
        assert_eq!(native_backend.watch_calls, &[first_path.to_path_buf()]);
    }

    fn modify_event(path: &str) -> notify::Event {
        notify::Event {
            paths: vec![PathBuf::from(path)],
            ..notify::Event::new(EventKind::Modify(notify::event::ModifyKind::Any))
        }
    }

    fn recording_watcher() -> (GlobalWatcher, Arc<Mutex<Vec<String>>>) {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let watcher = test_watcher_with_backends(Some(backend), None);
        let fired = Arc::new(Mutex::new(Vec::new()));
        for dir in ["/repo/a", "/repo/a/nested", "/repo/b"] {
            let fired = fired.clone();
            let label = dir.to_owned();
            watcher
                .add(
                    Arc::<Path>::from(Path::new(dir)),
                    WatcherMode::Native,
                    false,
                    move |_| {
                        fired.lock().push(label.clone());
                    },
                )
                .expect("add watch")
                .expect("watch registered");
        }
        (watcher, fired)
    }

    #[test]
    fn event_dispatches_only_to_registrations_covering_its_path() {
        let (watcher, fired) = recording_watcher();

        watcher.dispatch(WatcherMode::Native, Ok(modify_event("/repo/a/file.txt")));

        // Only the directory containing the file resolves; siblings stay untouched.
        assert_eq!(*fired.lock(), vec!["/repo/a".to_owned()]);
    }

    #[test]
    fn event_dispatches_to_every_ancestor_registration() {
        let (watcher, fired) = recording_watcher();

        watcher.dispatch(
            WatcherMode::Native,
            Ok(modify_event("/repo/a/nested/file.txt")),
        );

        // Both the directory containing the file and the ancestor watching it
        // resync, each exactly once, matching the previous broadcast-and-filter
        // behavior.
        let mut got = fired.lock().clone();
        got.sort();
        assert_eq!(got, vec!["/repo/a".to_owned(), "/repo/a/nested".to_owned()]);
    }

    fn fired_count() -> (
        Arc<Mutex<usize>>,
        impl Fn(&notify::Event) + Send + Sync + 'static,
    ) {
        let fired = Arc::new(Mutex::new(0usize));
        let cb = {
            let fired = fired.clone();
            move |_: &notify::Event| *fired.lock() += 1
        };
        (fired, cb)
    }

    #[test]
    fn watch_key_folds_case_but_keeps_exact_distinct() {
        let mixed = SanitizedPath::new(Path::new("/Repo/Proj"));
        let lower = SanitizedPath::new(Path::new("/repo/proj"));

        // Folded keys collide regardless of casing; exact keys do not.
        assert_eq!(WatchKey::folded(mixed), WatchKey::folded(lower));
        assert_ne!(WatchKey::exact(mixed), WatchKey::exact(lower));
        // Exact and folded live in different key spaces even for the same path.
        assert_ne!(WatchKey::exact(mixed), WatchKey::folded(mixed));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn watch_key_folds_unicode_normalization_on_macos() {
        // "Café" precomposed (NFC) vs decomposed (NFD) are different byte
        // sequences but the same directory on a normalization-insensitive volume.
        let nfc = Path::new("/repo/Caf\u{00e9}");
        let nfd = Path::new("/repo/Cafe\u{0301}");
        assert_ne!(nfc, nfd);
        assert_eq!(
            WatchKey::folded(SanitizedPath::new(nfc)),
            WatchKey::folded(SanitizedPath::new(nfd)),
        );
    }

    #[test]
    fn case_insensitive_registration_matches_differently_cased_event() {
        let (fired, cb) = fired_count();
        let watcher = test_watcher_with_backends(Some(Default::default()), None);
        watcher
            .add(
                Path::new("/Repo/Project").into(),
                WatcherMode::Native,
                true,
                cb,
            )
            .expect("add")
            .expect("registered");

        // Event arrives lowercased (as TSGO/macOS may report it).
        watcher.dispatch(
            WatcherMode::Native,
            Ok(modify_event("/repo/project/file.txt")),
        );
        assert_eq!(*fired.lock(), 1);
    }

    #[test]
    fn case_insensitive_registration_survives_case_only_rename() {
        let (fired, cb) = fired_count();
        let watcher = test_watcher_with_backends(Some(Default::default()), None);
        watcher
            .add(
                Path::new("/Repo/Proj").into(),
                WatcherMode::Native,
                true,
                cb,
            )
            .expect("add")
            .expect("registered");

        // The watched directory was renamed to a different casing; events now
        // arrive under the new spelling.
        watcher.dispatch(WatcherMode::Native, Ok(modify_event("/Repo/PROJ/file.txt")));
        assert_eq!(*fired.lock(), 1);
    }

    #[test]
    fn case_sensitive_registration_ignores_differently_cased_event() {
        let (fired, cb) = fired_count();
        let watcher = test_watcher_with_backends(Some(Default::default()), None);
        watcher
            .add(
                Path::new("/Repo/proj").into(),
                WatcherMode::Native,
                false,
                cb,
            )
            .expect("add")
            .expect("registered");

        // On a case-sensitive volume these are genuinely different directories.
        watcher.dispatch(WatcherMode::Native, Ok(modify_event("/Repo/PROJ/file.txt")));
        assert_eq!(*fired.lock(), 0);
    }

    #[test]
    fn differently_cased_adds_dedupe_on_case_insensitive_volume() {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let watcher = test_watcher_with_backends(Some(backend.clone()), None);
        watcher
            .add(
                Path::new("/Repo/Proj").into(),
                WatcherMode::Native,
                true,
                |_| {},
            )
            .expect("add")
            .expect("registered");
        watcher
            .add(
                Path::new("/repo/proj").into(),
                WatcherMode::Native,
                true,
                |_| {},
            )
            .expect("add")
            .expect("registered");

        // The second, differently-cased spelling reuses the same OS watch.
        assert_eq!(backend.lock().watch_calls.len(), 1);
    }

    #[test]
    fn recursive_parent_covers_differently_cased_child() {
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let watcher = test_watcher(backend.clone());
        watcher
            .add(Path::new("/Repo").into(), WatcherMode::Poll, true, |_| {})
            .expect("add")
            .expect("registered");
        watcher
            .add(
                Path::new("/repo/child").into(),
                WatcherMode::Poll,
                true,
                |_| {},
            )
            .expect("add");

        // The child is covered by the recursive parent despite the case mismatch.
        assert_eq!(backend.lock().watch_calls, vec![PathBuf::from("/Repo")]);
    }

    #[test]
    fn rescan_event_broadcasts_to_all_registrations_of_the_same_mode() {
        let (watcher, fired) = recording_watcher();

        let rescan = notify::Event::new(EventKind::Other).set_flag(notify::event::Flag::Rescan);
        watcher.dispatch(WatcherMode::Native, Ok(rescan));

        // A pathless rescan may have missed events anywhere, so every registration
        // of that mode resyncs, regardless of which directory it watches.
        let mut got = fired.lock().clone();
        got.sort();
        assert_eq!(
            got,
            vec![
                "/repo/a".to_owned(),
                "/repo/a/nested".to_owned(),
                "/repo/b".to_owned(),
            ]
        );
    }

    #[test]
    fn queued_rescans_are_coalesced_without_dropping_normal_events() {
        let (watcher, fired) = recording_watcher();
        let (event_tx, event_rx) = async_channel::unbounded();
        let rescan = || notify::Event::new(EventKind::Other).set_flag(notify::event::Flag::Rescan);

        event_tx
            .try_send((WatcherMode::Native, Ok(rescan())))
            .unwrap();
        event_tx
            .try_send((WatcherMode::Native, Ok(modify_event("/repo/a/file.txt"))))
            .unwrap();
        watcher.dispatch_batch((WatcherMode::Native, Ok(rescan())), &event_rx);

        let mut got = fired.lock().clone();
        got.sort();
        assert_eq!(
            got,
            vec![
                "/repo/a".to_owned(),
                "/repo/a".to_owned(),
                "/repo/a/nested".to_owned(),
                "/repo/b".to_owned(),
            ]
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn event_dispatches_when_reported_path_has_verbatim_prefix() {
        // `notify` (and Windows APIs generally) can report a changed path using the
        // verbatim `\\?\` long-path form even when the directory was registered
        // without it.
        let backend = Arc::new(Mutex::new(FakeWatchBackend::default()));
        let watcher = test_watcher_with_backends(Some(backend), None);
        let fired = Arc::new(Mutex::new(Vec::new()));
        {
            let fired = fired.clone();
            watcher
                .add(
                    Arc::<Path>::from(Path::new("C:\\repo\\src")),
                    WatcherMode::Native,
                    false,
                    move |_| fired.lock().push(()),
                )
                .expect("add watch")
                .expect("watch registered");
        }

        watcher.dispatch(
            WatcherMode::Native,
            Ok(modify_event("\\\\?\\C:\\repo\\src\\main.rs")),
        );

        assert_eq!(fired.lock().len(), 1);
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
