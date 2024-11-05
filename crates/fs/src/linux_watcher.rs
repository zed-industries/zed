use std::sync::Arc;

use notify::EventKind;
use parking_lot::Mutex;

use crate::{PathEvent, PathEventKind, Watcher};

pub struct LinuxWatcher {
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
}

impl LinuxWatcher {
    pub(crate) fn new(
        tx: smol::channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        Self {
            tx,
            pending_path_events,
        }
    }
}

impl Watcher for LinuxWatcher {
    fn add(&self, path: &std::path::Path) -> gpui::Result<()> {
        let root_path = path.to_path_buf();

        let tx = self.tx.clone();
        let pending_paths = self.pending_path_events.clone();

        use notify::Watcher;

        watcher::global({
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
                            if event_path.starts_with(&root_path) {
                                Some(PathEvent {
                                    path: event_path.clone(),
                                    kind,
                                })
                            } else {
                                None
                            }
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

        watcher::global(|g| {
            g.inotify
                .lock()
                .watch(path, notify::RecursiveMode::NonRecursive)
        })??;

        Ok(())
    }

    fn remove(&self, path: &std::path::Path) -> gpui::Result<()> {
        use notify::Watcher;
        Ok(watcher::global(|w| w.inotify.lock().unwatch(path))??)
    }
}

#[cfg(target_os = "linux")]
pub mod watcher {
    use std::sync::OnceLock;

    use parking_lot::Mutex;
    use util::ResultExt;

    pub struct GlobalWatcher {
        // two mutexes because calling inotify.add triggers an inotify.event, which needs watchers.
        pub(super) inotify: Mutex<notify::INotifyWatcher>,
        pub(super) watchers: Mutex<Vec<Box<dyn Fn(&notify::Event) + Send + Sync>>>,
    }

    impl GlobalWatcher {
        pub(super) fn add(&self, cb: impl Fn(&notify::Event) + Send + Sync + 'static) {
            self.watchers.lock().push(Box::new(cb))
        }
    }

    static INOTIFY_INSTANCE: OnceLock<anyhow::Result<GlobalWatcher, notify::Error>> =
        OnceLock::new();

    fn handle_event(event: Result<notify::Event, notify::Error>) {
        let Some(event) = event.log_err() else { return };
        global::<()>(move |watcher| {
            for f in watcher.watchers.lock().iter() {
                f(&event)
            }
        })
        .log_err();
    }

    pub fn global<T>(f: impl FnOnce(&GlobalWatcher) -> T) -> anyhow::Result<T> {
        let result = INOTIFY_INSTANCE.get_or_init(|| {
            notify::recommended_watcher(handle_event).map(|file_watcher| GlobalWatcher {
                inotify: Mutex::new(file_watcher),
                watchers: Default::default(),
            })
        });
        match result {
            Ok(g) => Ok(f(g)),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
}
