use notify::EventKind;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
use util::{ResultExt, paths::SanitizedPath};

use crate::{PathEvent, PathEventKind, Watcher};

pub struct FsWatcher {
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    registrations: Mutex<HashMap<Arc<std::path::Path>, WatcherRegistrationId>>,
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
        let mut registrations = self.registrations.lock();
        let registrations = registrations.drain();

        let _ = global(|g| {
            for (_, registration) in registrations {
                g.remove(registration);
            }
        });
    }
}

impl Watcher for FsWatcher {
    fn add(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let root_path = SanitizedPath::new_arc(path);

        let tx = self.tx.clone();
        let pending_paths = self.pending_path_events.clone();

        let path: Arc<std::path::Path> = path.into();

        if self.registrations.lock().contains_key(&path) {
            return Ok(());
        }

        let registration_id = global({
            let path = path.clone();
            |g| {
                g.add(
                    path,
                    notify::RecursiveMode::NonRecursive,
                    move |event: &notify::Event| {
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
                                event_path.starts_with(&root_path).then(|| PathEvent {
                                    path: event_path.as_path().to_path_buf(),
                                    kind,
                                })
                            })
                            .collect::<Vec<_>>();

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
                    },
                )
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

        self.watcher.lock().watch(&path, mode)?;

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

fn handle_event(event: Result<notify::Event, notify::Error>) {
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
