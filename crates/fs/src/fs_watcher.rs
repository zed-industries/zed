use notify::EventKind;
use parking_lot::Mutex;
use std::{
    collections::BTreeMap,
    path::Path,
    sync::{Arc, OnceLock},
    time::Duration,
};
use util::paths::SanitizedPath;

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

enum WatchBackend {
    Native(notify::RecommendedWatcher),
    Poll(notify::PollWatcher),
}

impl WatchBackend {
    fn watch(&mut self, path: &Path, recursive_mode: notify::RecursiveMode) -> notify::Result<()> {
        use notify::Watcher as _;

        match self {
            Self::Native(watcher) => watcher.watch(path, recursive_mode),
            Self::Poll(watcher) => watcher.watch(path, recursive_mode),
        }
    }

    fn unwatch(&mut self, path: &Path) -> notify::Result<()> {
        use notify::Watcher as _;

        match self {
            Self::Native(watcher) => watcher.unwatch(path),
            Self::Poll(watcher) => watcher.unwatch(path),
        }
    }
}

pub struct FsWatcher {
    backend: Mutex<Option<WatchBackend>>,
    mode: WatcherMode,
    poll_interval: Option<Duration>,
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    watched_paths: Arc<Mutex<BTreeMap<Arc<std::path::Path>, usize>>>,
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

fn push_notify_error(
    tx: &smol::channel::Sender<()>,
    pending_path_events: &Arc<Mutex<Vec<PathEvent>>>,
    watched_root: &Path,
    error: &notify::Error,
) {
    log::warn!("watcher error for {watched_root:?}: {error}");
    enqueue_path_events(
        tx,
        pending_path_events,
        vec![PathEvent {
            path: watched_root.to_path_buf(),
            kind: Some(PathEventKind::Rescan),
        }],
    );
}

impl FsWatcher {
    pub fn new(
        tx: smol::channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
        mode: WatcherMode,
        poll_interval: Option<Duration>,
    ) -> Self {
        Self {
            backend: Mutex::new(None),
            mode,
            poll_interval,
            tx,
            pending_path_events,
            watched_paths: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    fn recursive_mode(&self) -> notify::RecursiveMode {
        match self.mode {
            WatcherMode::Native => native_recursive_mode(),
            WatcherMode::Poll => notify::RecursiveMode::Recursive,
        }
    }

    fn ensure_backend(&self) -> anyhow::Result<()> {
        let mut backend = self.backend.lock();
        if backend.is_some() {
            return Ok(());
        }

        let callback_paths = self.watched_paths.clone();
        let callback_tx = self.tx.clone();
        let callback_pending_path_events = self.pending_path_events.clone();
        let callback = move |result: Result<notify::Event, notify::Error>| {
            let watched_roots = callback_paths.lock().keys().cloned().collect::<Vec<_>>();
            match result {
                Ok(event) => {
                    if matches!(event.kind, EventKind::Access(_)) {
                        return;
                    }
                    for watched_root in watched_roots {
                        push_notify_event(
                            &callback_tx,
                            &callback_pending_path_events,
                            watched_root.as_ref(),
                            &event,
                        );
                    }
                }
                Err(error) => {
                    for watched_root in watched_roots {
                        push_notify_error(
                            &callback_tx,
                            &callback_pending_path_events,
                            watched_root.as_ref(),
                            &error,
                        );
                    }
                }
            }
        };

        *backend = Some(match self.mode {
            WatcherMode::Native => WatchBackend::Native(notify::recommended_watcher(callback)?),
            WatcherMode::Poll => {
                let config = notify::Config::default()
                    .with_poll_interval(self.poll_interval.unwrap_or(Duration::from_secs(2)));
                WatchBackend::Poll(notify::PollWatcher::new(callback, config)?)
            }
        });
        Ok(())
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn native_recursive_mode() -> notify::RecursiveMode {
    notify::RecursiveMode::Recursive
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn native_recursive_mode() -> notify::RecursiveMode {
    notify::RecursiveMode::NonRecursive
}

impl Watcher for FsWatcher {
    fn add(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("watcher add: {path:?}");

        let path: Arc<std::path::Path> = path.into();
        let mut path_counts = self.watched_paths.lock();
        let path_already_covered = path_counts
            .keys()
            .any(|watched_path| path.starts_with(watched_path.as_ref()) && path != *watched_path);

        if !path_already_covered && !path_counts.contains_key(&path) {
            drop(path_counts);
            self.ensure_backend()?;
            self.backend
                .lock()
                .as_mut()
                .expect("backend initialized")
                .watch(&path, self.recursive_mode())?;
            path_counts = self.watched_paths.lock();
        }

        *path_counts.entry(path).or_insert(0) += 1;
        Ok(())
    }

    fn remove(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("remove watched path: {path:?}");

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
        if let Some(backend) = self.backend.lock().as_mut() {
            backend.unwatch(&path)?;
        }
        Ok(())
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

pub struct NativeWatcherAvailability;

static FS_WATCHER_INSTANCE: OnceLock<anyhow::Result<NativeWatcherAvailability, notify::Error>> =
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

pub fn global<T>(f: impl FnOnce(&NativeWatcherAvailability) -> T) -> anyhow::Result<T> {
    let result = FS_WATCHER_INSTANCE.get_or_init(|| {
        notify::recommended_watcher(|_: Result<notify::Event, notify::Error>| {})
            .map(|_| NativeWatcherAvailability)
    });
    match result {
        Ok(availability) => Ok(f(availability)),
        Err(error) => Err(anyhow::anyhow!("{error}")),
    }
}
