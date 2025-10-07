use notify_debouncer_full::{new_debouncer, notify::{self, EventKind}, Debouncer, RecommendedCache};
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, HashMap},
    ops::DerefMut,
    sync::{Arc, OnceLock},
    time::Duration,
};
use util::{ResultExt, paths::SanitizedPath};

use crate::{PathEvent, PathEventKind, Watcher};

pub struct FsWatcher {
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Mutex<BTreeMap<Arc<std::path::Path>, WatcherRegistrationId>>,
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
        let tx = self.tx.clone();
        let pending_paths = self.pending_path_events.clone();

        #[cfg(target_os = "windows")]
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
                return Ok(());
            }
        }
        #[cfg(target_os = "linux")]
        {
            if self.registrations.lock().contains_key(path) {
                return Ok(());
            }
        }

        let root_path = SanitizedPath::new_arc(path);
        let path: Arc<std::path::Path> = path.into();

        #[cfg(target_os = "windows")]
        let mode = notify::RecursiveMode::Recursive;
        #[cfg(target_os = "linux")]
        let mode = notify::RecursiveMode::NonRecursive;

        let registration_id = global({
            let path = path.clone();
            |g| {
                g.add(
                    path,
                    mode,
                    move |events: &[notify::Event]| {
                        let mut path_events = Vec::new();

                        for event in events {
                            let kind = match event.kind {
                                EventKind::Create(_) => Some(PathEventKind::Created),
                                EventKind::Modify(_) => Some(PathEventKind::Changed),
                                EventKind::Remove(_) => Some(PathEventKind::Removed),
                                _ => None,
                            };

                            for event_path in &event.paths {
                                let event_path = SanitizedPath::new(event_path);
                                if event_path.starts_with(&root_path) {
                                    path_events.push(PathEvent {
                                        path: event_path.as_path().to_path_buf(),
                                        kind,
                                    });
                                }
                            }
                        }

                    if !path_events.is_empty() {
                        path_events.sort();
                        let mut pending_paths = pending_paths.lock();
                        if pending_paths.is_empty() {
                            tx.try_send(()).ok();
                        }
                        util::extend_sorted(
                            &mut *pending_paths,
                            path_events,
                            usize::MAX,
                            |a, b| a.path.cmp(&b.path),
                        );
                    }
                })
            }
        })??;

        self.registrations.lock().insert(path, registration_id);

        Ok(())
    }

    fn remove(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let Some(registration) = self.registrations.lock().remove(path) else {
            return Ok(());
        };

        global(|w| w.remove(registration))
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WatcherRegistrationId(u32);

struct WatcherRegistrationState {
    callback: Arc<dyn Fn(&[notify::Event]) + Send + Sync>,
    path: Arc<std::path::Path>,
}

struct WatcherState {
    watchers: HashMap<WatcherRegistrationId, WatcherRegistrationState>,
    path_registrations: HashMap<Arc<std::path::Path>, u32>,
    last_registration: WatcherRegistrationId,
}

pub struct GlobalWatcher {
    state: Mutex<WatcherState>,
    debouncer: Mutex<Debouncer<notify::RecommendedWatcher, RecommendedCache>>,
}

impl GlobalWatcher {
    #[must_use]
    fn add(
        &self,
        path: Arc<std::path::Path>,
        mode: notify::RecursiveMode,
        cb: impl Fn(&[notify::Event]) + Send + Sync + 'static,
    ) -> anyhow::Result<WatcherRegistrationId> {
        self.debouncer.lock().watch(&path, mode)?;

        let mut state = self.state.lock();

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
            self.debouncer
                .lock()
                .unwatch(&registration_state.path)
                .log_err();
        }
    }
}

static FS_WATCHER_INSTANCE: OnceLock<anyhow::Result<GlobalWatcher, String>> =
    OnceLock::new();

fn handle_debounced_events(
    result: notify_debouncer_full::DebounceEventResult,
) {
    let events = match result {
        Ok(events) => events,
        Err(errors) => {
            for error in errors {
                log::error!("File watcher error: {:?}", error);
            }
            return;
        }
    };

    // Convert debounced events to notify events and filter
    let notify_events: Vec<notify::Event> = events
        .into_iter()
        .filter_map(|debounced_event| {
            // Filter out access events
            if matches!(debounced_event.event.kind, EventKind::Access(_)) {
                return None;
            }
            Some(debounced_event.event)
        })
        .collect();

    if notify_events.is_empty() {
        return;
    }

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
            callback(&notify_events);
        }
    })
    .log_err();
}

pub fn global<T>(f: impl FnOnce(&GlobalWatcher) -> T) -> anyhow::Result<T> {
    let result = FS_WATCHER_INSTANCE.get_or_init(|| {
        let debounce_duration = Duration::from_millis(200);

        new_debouncer(debounce_duration, None, handle_debounced_events)
            .map(|debouncer| GlobalWatcher {
                state: Mutex::new(WatcherState {
                    watchers: Default::default(),
                    path_registrations: Default::default(),
                    last_registration: Default::default(),
                }),
                debouncer: Mutex::new(debouncer),
            })
            .map_err(|e| format!("Failed to create debouncer: {}", e))
    });
    match result {
        Ok(g) => Ok(f(g)),
        Err(e) => Err(anyhow::anyhow!("{e}")),
    }
}
