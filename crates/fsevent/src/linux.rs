use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
    time::Duration,
};

use notify::{Config, RecommendedWatcher, Watcher};
use parking_lot::Mutex;

use crate::{Event, StreamFlags};

pub struct EventStream {
    watcher: Arc<Mutex<RecommendedWatcher>>,
    paths: Vec<PathBuf>,
    event_fn: Arc<OnceLock<Box<dyn Fn(Vec<Event>) -> bool + 'static + Send + Sync>>>,
}

pub struct Handle(Arc<Mutex<RecommendedWatcher>>);

impl EventStream {
    pub fn new(paths: &[&Path], latency: Duration) -> (Self, Handle) {
        let event_fn: Arc<OnceLock<Box<dyn Fn(Vec<Event>) -> bool + 'static + Send + Sync>>> =
            Arc::new(OnceLock::new());

        let paths: Vec<_> = paths.iter().map(|path| path.to_path_buf()).collect();

        let mut watcher = notify::recommended_watcher({
            let event_fn = event_fn.clone();
            let paths = paths.clone();
            move |res: Result<notify::Event, _>| {
                if let Ok(event) = res {
                    let flags = StreamFlags::empty();

                    // match event.kind {
                    //     EventKind::Access(access) => match access {
                    //         notify::event::AccessKind::Any => todo!(),
                    //         notify::event::AccessKind::Read => todo!(),
                    //         notify::event::AccessKind::Open(_) => todo!(),
                    //         notify::event::AccessKind::Close(_) => todo!(),
                    //         notify::event::AccessKind::Other => todo!(),
                    //     },
                    //     EventKind::Create(create) => match create {
                    //         notify::event::CreateKind::Any => todo!(),
                    //         notify::event::CreateKind::File => todo!(),
                    //         notify::event::CreateKind::Folder => todo!(),
                    //         notify::event::CreateKind::Other => todo!(),
                    //     },
                    //     EventKind::Modify(modify) => match modify {
                    //         notify::event::ModifyKind::Any => todo!(),
                    //         notify::event::ModifyKind::Data(_) => todo!(),
                    //         notify::event::ModifyKind::Metadata(_) => todo!(),
                    //         notify::event::ModifyKind::Name(_) => todo!(),
                    //         notify::event::ModifyKind::Other => todo!(),
                    //     },
                    //     EventKind::Remove(remove) => match remove {
                    //         notify::event::RemoveKind::Any => todo!(),
                    //         notify::event::RemoveKind::File => todo!(),
                    //         notify::event::RemoveKind::Folder => todo!(),
                    //         notify::event::RemoveKind::Other => todo!(),
                    //     },
                    //     EventKind::Other => todo!(),
                    //     EventKind::Any => todo!(),
                    // };

                    let events = event
                        .paths
                        .iter()
                        .filter(|evt_path| {
                            paths
                                .iter()
                                .any(|requested_path| evt_path.starts_with(requested_path))
                        })
                        .map(|path| Event {
                            event_id: 0,
                            flags,
                            path: path.to_path_buf(),
                        })
                        .collect::<Vec<_>>();

                    if !events.is_empty() {
                        event_fn
                            .get()
                            .expect("Watcher cannot produce events until paths are provided")(
                            events,
                        );
                    }
                }
            }
        })
        .expect("Failed to watch requested path");

        watcher
            .configure(Config::default().with_poll_interval(latency))
            .expect("Failed to watch requested path");

        let watcher = Arc::new(Mutex::new(watcher));
        (
            Self {
                watcher: watcher.clone(),
                event_fn: event_fn.clone(),
                paths,
            },
            Handle(watcher),
        )
    }

    pub fn run(self, f: impl Fn(Vec<Event>) -> bool + 'static + Send + Sync) {
        self.event_fn.get_or_init(|| Box::new(f));

        let mut watcher = self.watcher.lock();
        for path in self.paths {
            watcher
                .watch(
                    dbg!(path.parent().unwrap_or(&path)),
                    notify::RecursiveMode::Recursive,
                )
                .expect("Failed to watch requested path");
        }
    }
}
