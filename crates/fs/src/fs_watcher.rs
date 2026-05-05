use notify::EventKind;
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, HashMap},
    ops::DerefMut,
    path::Path,
    sync::{Arc, LazyLock, OnceLock},
    time::Duration,
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
    tx: async_channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Mutex<BTreeMap<Arc<std::path::Path>, WatcherRegistrationId>>,
    mode: WatcherMode,
}

impl FsWatcher {
    pub fn new(
        tx: async_channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
        mode: WatcherMode,
    ) -> Self {
        Self {
            tx,
            pending_path_events,
            registrations: Default::default(),
            mode,
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

        let global_watcher = global_watcher();
        for (_, registration) in registrations {
            global_watcher.remove(registration);
        }
    }
}

impl Watcher for FsWatcher {
    fn add(&self, path: &std::path::Path) -> anyhow::Result<()> {
        log::trace!("watcher add: {path:?}");
        let tx = self.tx.clone();
        let pending_path_events = self.pending_path_events.clone();

        if (self.mode == WatcherMode::Poll || cfg!(any(target_os = "windows", target_os = "macos")))
            && let Some((watched_path, _)) = self
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

        if self.registrations.lock().contains_key(path) {
            log::trace!("path to watch is already watched: {path:?}");
            return Ok(());
        }

        let root_path = SanitizedPath::new_arc(path);
        let path: Arc<std::path::Path> = path.into();

        let registration_path = path.clone();
        let registration_id = global_watcher().add(
            path.clone(),
            self.mode,
            move |result: Result<&notify::Event, &notify::Error>| match result {
                Ok(event) => {
                    log::trace!("watcher received event: {event:?}");
                    push_notify_event(&tx, &pending_path_events, &root_path, path.as_ref(), event);
                }
                Err(error) => {
                    push_notify_error(&tx, &pending_path_events, path.as_ref(), error);
                }
            },
        )?;

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

        global_watcher().remove(registration);
        Ok(())
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
    callback: Arc<dyn for<'a> Fn(Result<&'a notify::Event, &'a notify::Error>) + Send + Sync>,
    path: Arc<std::path::Path>,
    mode: WatcherMode,
}

struct WatcherState {
    watchers: HashMap<WatcherRegistrationId, WatcherRegistrationState>,
    native_path_registrations: HashMap<Arc<std::path::Path>, u32>,
    poll_path_registrations: HashMap<Arc<std::path::Path>, u32>,
    last_registration: WatcherRegistrationId,
}

impl WatcherState {
    fn path_registrations(&mut self, mode: WatcherMode) -> &mut HashMap<Arc<std::path::Path>, u32> {
        match mode {
            WatcherMode::Native => &mut self.native_path_registrations,
            WatcherMode::Poll => &mut self.poll_path_registrations,
        }
    }
}

pub struct GlobalWatcher {
    state: Mutex<WatcherState>,

    // DANGER: never keep state lock while holding watcher lock
    // two mutexes because calling watcher.add triggers watcher.event, which needs watchers.
    native_watcher: Mutex<Option<notify::RecommendedWatcher>>,
    poll_watcher: Mutex<Option<notify::PollWatcher>>,
}

impl GlobalWatcher {
    #[must_use]
    fn add(
        &self,
        path: Arc<std::path::Path>,
        mode: WatcherMode,
        cb: impl for<'a> Fn(Result<&'a notify::Event, &'a notify::Error>) + Send + Sync + 'static,
    ) -> anyhow::Result<WatcherRegistrationId> {
        let mut state = self.state.lock();
        let registrations_for_mode = state.path_registrations(mode);
        let path_already_covered =
            path_already_covered(path.as_ref(), registrations_for_mode, mode);

        if !path_already_covered && !registrations_for_mode.contains_key(&path) {
            drop(state);
            self.watch(&path, mode)?;
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
        *state.path_registrations(mode).entry(path).or_insert(0) += 1;

        Ok(id)
    }

    pub fn remove(&self, id: WatcherRegistrationId) {
        let mut state = self.state.lock();
        let Some(registration_state) = state.watchers.remove(&id) else {
            return;
        };

        let path_registrations = state.path_registrations(registration_state.mode);
        let Some(count) = path_registrations.get_mut(&registration_state.path) else {
            return;
        };
        *count -= 1;
        if *count == 0 {
            path_registrations.remove(&registration_state.path);
            let path_still_covered = path_already_covered(
                registration_state.path.as_ref(),
                path_registrations,
                registration_state.mode,
            );

            if !path_still_covered {
                drop(state);
                self.unwatch(&registration_state.path, registration_state.mode)
                    .log_err();
            }
        }
    }

    fn watch(&self, path: &Path, mode: WatcherMode) -> anyhow::Result<()> {
        use notify::Watcher;

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
        use notify::Watcher;

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
        *self.native_watcher.lock() = Some(watcher);
        Ok(())
    }

    fn ensure_poll_watcher(&self) -> anyhow::Result<()> {
        if self.poll_watcher.lock().is_some() {
            return Ok(());
        }

        let config = notify::Config::default().with_poll_interval(*POLL_INTERVAL);
        let watcher = notify::PollWatcher::new(handle_poll_event, config)?;
        *self.poll_watcher.lock() = Some(watcher);
        Ok(())
    }
}

fn path_already_covered(
    path: &Path,
    path_registrations: &HashMap<Arc<std::path::Path>, u32>,
    mode: WatcherMode,
) -> bool {
    (mode == WatcherMode::Poll || cfg!(any(target_os = "windows", target_os = "macos")))
        && path_registrations
            .keys()
            .any(|existing| path.starts_with(existing.as_ref()) && path != existing.as_ref())
}

static POLL_INTERVAL: LazyLock<Duration> = LazyLock::new(|| {
    let poll_ms: u64 = std::env::var("ZED_FILE_WATCHER_POLL_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(2000)
        .clamp(500, 30000);
    Duration::from_millis(poll_ms)
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
            if matches!(event.kind, EventKind::Access(_)) {
                return;
            }
            for callback in callbacks {
                callback(Ok(&event));
            }
        }
        Err(error) => {
            for callback in callbacks {
                callback(Err(&error));
            }
        }
    }
}

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

pub fn global<T>(f: impl FnOnce(&GlobalWatcher) -> T) -> anyhow::Result<T> {
    let global_watcher = global_watcher();
    global_watcher.ensure_native_watcher()?;
    Ok(f(global_watcher))
}
