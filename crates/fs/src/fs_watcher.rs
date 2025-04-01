use notify::EventKind;
use parking_lot::Mutex;
use std::sync::{Arc, OnceLock};
use util::{ResultExt, paths::SanitizedPath};

use crate::{PathEvent, PathEventKind, Watcher};

pub struct FsWatcher {
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
}

impl FsWatcher {
    pub fn new(
        tx: smol::channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        Self {
            tx,
            pending_path_events,
        }
    }
}

impl Watcher for FsWatcher {
    fn add(&self, path: &std::path::Path) -> gpui::Result<()> {
        let root_path = SanitizedPath::from(path);

        let tx = self.tx.clone();
        let pending_paths = self.pending_path_events.clone();

        use notify::Watcher;

        global({
            |g| {
                g.add(move |event: &notify::Event| {
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
                            let event_path = SanitizedPath::from(event_path);
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
                })
            }
        })?;

        global(|g| {
            g.watcher
                .lock()
                .watch(path, notify::RecursiveMode::NonRecursive)
        })??;

        Ok(())
    }

    fn remove(&self, path: &std::path::Path) -> gpui::Result<()> {
        use notify::Watcher;
        Ok(global(|w| w.watcher.lock().unwatch(path))??)
    }
}

pub struct GlobalWatcher {
    // two mutexes because calling watcher.add triggers an watcher.event, which needs watchers.
    #[cfg(target_os = "linux")]
    pub(super) watcher: Mutex<notify::INotifyWatcher>,
    #[cfg(target_os = "freebsd")]
    pub(super) watcher: Mutex<notify::KqueueWatcher>,
    #[cfg(target_os = "windows")]
    pub(super) watcher: Mutex<notify::ReadDirectoryChangesWatcher>,
    pub(super) watchers: Mutex<Vec<Box<dyn Fn(&notify::Event) + Send + Sync>>>,
}

impl GlobalWatcher {
    pub(super) fn add(&self, cb: impl Fn(&notify::Event) + Send + Sync + 'static) {
        self.watchers.lock().push(Box::new(cb))
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
        for f in watcher.watchers.lock().iter() {
            f(&event)
        }
    })
    .log_err();
}

pub fn global<T>(f: impl FnOnce(&GlobalWatcher) -> T) -> anyhow::Result<T> {
    let result = FS_WATCHER_INSTANCE.get_or_init(|| {
        notify::recommended_watcher(handle_event).map(|file_watcher| GlobalWatcher {
            watcher: Mutex::new(file_watcher),
            watchers: Default::default(),
        })
    });
    match result {
        Ok(g) => Ok(f(g)),
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    }
}
