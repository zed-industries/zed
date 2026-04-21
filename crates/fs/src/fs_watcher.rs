use notify::EventKind;
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, HashMap},
    ops::DerefMut,
    path::Path,
    sync::{Arc, OnceLock},
    time::Duration,
};
use util::{ResultExt, paths::SanitizedPath};

use crate::{PathEvent, PathEventKind, Watcher};

/// Determines how file changes are detected.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WatcherMode {
    /// Use the OS-native file watcher (inotify on Linux, FSEvents on macOS,
    /// ReadDirectoryChanges on Windows). Most efficient but doesn't work on
    /// network filesystems, WSL drvfs mounts, or FUSE mounts.
    #[default]
    Native,
    /// Use polling to detect file changes. Works on all filesystems but uses more CPU.
    Poll,
}

pub struct FsWatcher {
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Mutex<BTreeMap<Arc<std::path::Path>, WatcherRegistrationId>>,
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
    watched_root: &Path,
    event: &notify::Event,
) {
    let kind = match &event.kind {
        // `need_rescan` is handled below, so unknown event kinds can still trigger recovery.
        EventKind::Create(_) => Some(PathEventKind::Created),
        EventKind::Modify(_) => Some(PathEventKind::Changed),
        EventKind::Remove(_) => Some(PathEventKind::Removed),
        _ => None,
    };
    let root_path = SanitizedPath::new_arc(watched_root);
    let mut path_events = event
        .paths
        .iter()
        .filter_map(|event_path| {
            let event_path = SanitizedPath::new(event_path);
            event_path.starts_with(&root_path).then(|| PathEvent {
                path: event_path.as_path().to_path_buf(),
                kind,
            })
        })
        .collect::<Vec<_>>();

    if event.need_rescan() {
        log::warn!("filesystem watcher lost sync for {watched_root:?}; scheduling rescan");
        path_events.retain(|path_event| path_event.path != watched_root);
        path_events.push(PathEvent {
            path: watched_root.to_path_buf(),
            kind: Some(PathEventKind::Rescan),
        });
    }

    enqueue_path_events(tx, pending_path_events, path_events);
}

impl FsWatcher {
    pub fn new(
        tx: smol::channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        Self {
            tx,
            pending_path_events,
            registrations: Default::default(),
        }
    }
}

impl Drop for FsWatcher {
    fn drop(&mut self) {
        let mut registrations = BTreeMap::new();
        {
            let old = &mut self.registrations.lock();
            std::mem::swap(old.deref_mut(), &mut registrations);
        }

        let _ = global(|g| {
            for (_, registration) in registrations {
                g.remove(registration);
            }
        });
    }
}

impl Watcher for FsWatcher {
    fn add(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("watcher add: {path:?}");
        let tx = self.tx.clone();
        let pending_paths = self.pending_path_events.clone();

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            // Return early if an ancestor of this path was already being watched.
            // saves a huge amount of memory
            if let Some((watched_path, _)) = self
                .registrations
                .lock()
                .range::<std::path::Path, _>((
                    std::ops::Bound::Unbounded,
                    std::ops::Bound::Included(path),
                ))
                .next_back()
                && path.starts_with(watched_path.as_ref())
            {
                log::trace!(
                    "path to watch is covered by existing registration: {path:?}, {watched_path:?}"
                );
                return Ok(());
            }
        }
        #[cfg(target_os = "linux")]
        {
            if self.registrations.lock().contains_key(path) {
                log::trace!("path to watch is already watched: {path:?}");
                return Ok(());
            }
        }

        let path: Arc<std::path::Path> = path.into();

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        let mode = notify::RecursiveMode::Recursive;
        #[cfg(target_os = "linux")]
        let mode = notify::RecursiveMode::NonRecursive;

        let registration_path = path.clone();
        let registration_id = global({
            let watch_path = path.clone();
            let callback_path = path;
            |g| {
                g.add(watch_path, mode, move |event: &notify::Event| {
                    log::trace!("watcher received event: {event:?}");
                    push_notify_event(&tx, &pending_paths, callback_path.as_ref(), event);
                })
            }
        })??;

        self.registrations
            .lock()
            .insert(registration_path, registration_id);

        Ok(())
    }

    fn remove(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("remove watched path: {path:?}");
        let Some(registration) = self.registrations.lock().remove(path) else {
            return Ok(());
        };

        global(|w| w.remove(registration))
    }
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

/// A polling-based file watcher that works on any filesystem.
///
/// Unlike [`FsWatcher`] (which uses OS-native inotify/FSEvents/ReadDirectoryChanges),
/// this periodically polls the filesystem for changes. Use this for network filesystems,
/// WSL drvfs mounts, FUSE mounts, or other situations where native watchers silently
/// fail to deliver events.
pub struct PollFsWatcher {
    watcher: Mutex<notify::PollWatcher>,
    watched_paths: Arc<Mutex<BTreeMap<Arc<std::path::Path>, usize>>>,
}

impl PollFsWatcher {
    pub fn new(
        tx: smol::channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
        poll_interval: Duration,
    ) -> anyhow::Result<Self> {
        let config = notify::Config::default().with_poll_interval(poll_interval);

        let watched_paths = Arc::new(Mutex::new(BTreeMap::<Arc<std::path::Path>, usize>::new()));
        let callback_paths = watched_paths.clone();
        let watcher = notify::PollWatcher::new(
            move |result: Result<notify::Event, notify::Error>| {
                let watched_roots = callback_paths.lock().keys().cloned().collect::<Vec<_>>();
                match result {
                    Ok(event) => {
                        if matches!(event.kind, EventKind::Access(_)) {
                            return;
                        }
                        for watched_root in watched_roots {
                            push_notify_event(
                                &tx,
                                &pending_path_events,
                                watched_root.as_ref(),
                                &event,
                            );
                        }
                    }
                    Err(error) => {
                        for watched_root in watched_roots {
                            log::warn!("watcher error for {watched_root:?}: {error}");
                            enqueue_path_events(
                                &tx,
                                &pending_path_events,
                                vec![PathEvent {
                                    path: watched_root.to_path_buf(),
                                    kind: Some(PathEventKind::Rescan),
                                }],
                            );
                        }
                    }
                }
            },
            config,
        )?;

        Ok(Self {
            watcher: Mutex::new(watcher),
            watched_paths,
        })
    }
}

impl Watcher for PollFsWatcher {
    fn add(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("poll watcher add: {path:?}");

        let path: Arc<std::path::Path> = path.into();
        let mut path_counts = self.watched_paths.lock();
        let path_already_covered = path_counts
            .keys()
            .any(|watched_path| path.starts_with(watched_path.as_ref()) && path != *watched_path);

        if !path_already_covered && !path_counts.contains_key(&path) {
            drop(path_counts);
            use notify::Watcher as _;
            self.watcher
                .lock()
                .watch(&path, notify::RecursiveMode::Recursive)?;
            path_counts = self.watched_paths.lock();
        }

        *path_counts.entry(path).or_insert(0) += 1;
        Ok(())
    }

    fn remove(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("poll watcher remove: {path:?}");

        let path: Arc<std::path::Path> = path.into();
        let mut path_counts = self.watched_paths.lock();
        let Some(count) = path_counts.get_mut(&path) else {
            return Ok(());
        };

        *count -= 1;
        if *count > 0 {
            return Ok(());
        }

        path_counts.remove(&path);
        let path_is_still_covered = path_counts.keys().any(|watched_path| {
            path.starts_with(watched_path.as_ref()) && path.as_ref() != watched_path.as_ref()
        });
        if path_is_still_covered {
            return Ok(());
        }

        drop(path_counts);
        use notify::Watcher as _;
        self.watcher.lock().unwatch(&path)?;
        Ok(())
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WatcherRegistrationId(u32);

struct WatcherRegistrationState {
    callback: Arc<dyn Fn(&notify::Event) + Send + Sync>,
    path: Arc<std::path::Path>,
}

struct WatcherState {
    watchers: HashMap<WatcherRegistrationId, WatcherRegistrationState>,
    path_registrations: HashMap<Arc<std::path::Path>, u32>,
    last_registration: WatcherRegistrationId,
}

pub struct GlobalWatcher {
    state: Mutex<WatcherState>,

    // DANGER: never keep the state lock while holding the watcher lock
    // two mutexes because calling watcher.add triggers an watcher.event, which needs watchers.
    #[cfg(target_os = "linux")]
    watcher: Mutex<notify::INotifyWatcher>,
    #[cfg(target_os = "freebsd")]
    watcher: Mutex<notify::KqueueWatcher>,
    #[cfg(target_os = "windows")]
    watcher: Mutex<notify::ReadDirectoryChangesWatcher>,
    #[cfg(target_os = "macos")]
    watcher: Mutex<notify::FsEventWatcher>,
}

impl GlobalWatcher {
    #[must_use]
    fn add(
        &self,
        path: Arc<std::path::Path>,
        mode: notify::RecursiveMode,
        cb: impl Fn(&notify::Event) + Send + Sync + 'static,
    ) -> anyhow::Result<WatcherRegistrationId> {
        use notify::Watcher;

        let mut state = self.state.lock();

        // Check if this path is already covered by an existing watched ancestor path.
        // On macOS and Windows, watching is recursive, so we don't need to watch
        // child paths if an ancestor is already being watched.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        let path_already_covered = state.path_registrations.keys().any(|existing| {
            path.starts_with(existing.as_ref()) && path.as_ref() != existing.as_ref()
        });

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let path_already_covered = false;

        if !path_already_covered && !state.path_registrations.contains_key(&path) {
            drop(state);
            self.watcher.lock().watch(&path, mode)?;
            state = self.state.lock();
        }

        let id = state.last_registration;
        state.last_registration = WatcherRegistrationId(id.0 + 1);

        let registration_state = WatcherRegistrationState {
            callback: Arc::new(cb),
            path: path.clone(),
        };
        state.watchers.insert(id, registration_state);
        *state.path_registrations.entry(path).or_insert(0) += 1;

        Ok(id)
    }

    pub fn remove(&self, id: WatcherRegistrationId) {
        use notify::Watcher;
        let mut state = self.state.lock();
        let Some(registration_state) = state.watchers.remove(&id) else {
            return;
        };

        let Some(count) = state.path_registrations.get_mut(&registration_state.path) else {
            return;
        };
        *count -= 1;
        if *count == 0 {
            state.path_registrations.remove(&registration_state.path);

            drop(state);
            self.watcher
                .lock()
                .unwatch(&registration_state.path)
                .log_err();
        }
    }
}

static FS_WATCHER_INSTANCE: OnceLock<anyhow::Result<GlobalWatcher, notify::Error>> =
    OnceLock::new();

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
}

fn handle_event(event: Result<notify::Event, notify::Error>) {
    log::trace!("global handle event: {event:?}");
    // Filter out access events, which could lead to a weird bug on Linux after upgrading notify
    // https://github.com/zed-industries/zed/actions/runs/14085230504/job/39449448832
    let Some(event) = event
        .log_err()
        .filter(|event| !matches!(event.kind, EventKind::Access(_)))
    else {
        return;
    };
    global::<()>(move |watcher| {
        let callbacks = {
            let state = watcher.state.lock();
            state
                .watchers
                .values()
                .map(|r| r.callback.clone())
                .collect::<Vec<_>>()
        };
        for callback in callbacks {
            callback(&event);
        }
    })
    .log_err();
}

pub fn global<T>(f: impl FnOnce(&GlobalWatcher) -> T) -> anyhow::Result<T> {
    let result = FS_WATCHER_INSTANCE.get_or_init(|| {
        notify::recommended_watcher(handle_event).map(|file_watcher| GlobalWatcher {
            state: Mutex::new(WatcherState {
                watchers: Default::default(),
                path_registrations: Default::default(),
                last_registration: Default::default(),
            }),
            watcher: Mutex::new(file_watcher),
        })
    });
    match result {
        Ok(g) => Ok(f(g)),
        Err(e) => Err(anyhow::anyhow!("{e}")),
    }
}
