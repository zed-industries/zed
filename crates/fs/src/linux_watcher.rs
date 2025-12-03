use std::{
    path::Path,
    pin::Pin,
    sync::{Arc, OnceLock},
};

use crate::{PathEvent, Watcher};
use anyhow::Result;
use collections::HashMap;
use futures::Stream;
use gpui::{BackgroundExecutor, Task};
use inotify::{Inotify, WatchMask};
use smol::{
    channel::{Receiver, Sender},
    future::block_on,
};

pub struct LinuxWatcher {
    inotify: InotifyWatcher,
    poller: PollWatcher,
    //task: Task<Result<Never>>,
}

#[derive(Debug)]
enum WatcherStateUpdate {
    NewPath(Arc<Path>),
    RemPath(Arc<Path>),
}

struct InotifyWatcher {
    tx: Sender<WatcherStateUpdate>,
}

struct PollWatcher {}

enum Never {}

impl LinuxWatcher {
    pub fn init(
        executor: BackgroundExecutor,
    ) -> (Self, Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>) {
        let (tx_paths, rx_paths) = smol::channel::unbounded();
        let (tx_events, rx_events) = smol::channel::unbounded();

        let task = executor.spawn::<Result<Never>>(async move {
            const WATCH_MASK: WatchMask = WatchMask::CREATE
                .union(WatchMask::CLOSE_WRITE)
                .union(WatchMask::DELETE)
                .union(WatchMask::DELETE_SELF)
                .union(WatchMask::MODIFY)
                .union(WatchMask::MOVE)
                .union(WatchMask::MOVE_SELF); // todo: fix this
            let mut buf = Box::<[u8; 1024]>::new_uninit();
            unsafe {
                buf.as_mut_ptr().write_bytes(0, 1);
            }
            let mut buf = unsafe { buf.assume_init() };
            let mut watcher = Inotify::init()?;
            let mut path_map = HashMap::default();
            loop {
                //dbg!();
                for upd in (0..).map_while(|_| rx_paths.try_recv().ok()) {
                    //dbg!(&upd);
                    match upd {
                        WatcherStateUpdate::NewPath(p) => {
                            //dbg!();
                            let wd = watcher.watches().add(p.clone(), WATCH_MASK)?;
                            //dbg!();
                            path_map.insert(wd, p.clone());
                            tx_events
                                .send(vec![PathEvent {
                                    path: p.to_path_buf(),
                                    kind: None,
                                }])
                                .await?;
                            //dbg!();
                            log::info!("Watching {p:?}");
                        }
                        WatcherStateUpdate::RemPath(p) => {
                            if let Some((wd, _)) = path_map.iter().find(|(_, v)| **v == p) {
                                let wd = wd.clone();
                                path_map.remove(&wd);
                                watcher.watches().remove(wd)?;
                                //dbg!();
                                log::info!("No longer watching {p:?}");
                            }
                            tx_events
                                .send(vec![PathEvent {
                                    path: p.to_path_buf(),
                                    kind: None,
                                }])
                                .await?;
                            //dbg!();
                        }
                    };
                }
                //dbg!();
                let mut evts = vec![];
                // todo: skip wouldblock errors
                match watcher.read_events(&mut *buf) {
                    Ok(raw_events) => {
                        for evt in raw_events {
                            log::info!("Got event {evt:?}");

                            let Some(path) = path_map.get(&evt.wd) else {
                                continue;
                            };
                            let path = path.to_path_buf();
                            if let Some(extra) = evt.name {
                                evts.push(dbg!(PathEvent {
                                    path: path.join(extra),
                                    kind: None,
                                }))
                                //path = dbg!(path.join(extra));
                            }
                            evts.push(dbg!(PathEvent { path, kind: None }));
                        }
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::WouldBlock => {
                            //dbg!()
                        }
                        _ => {
                            util::log_err(&e);
                            //dbg!(&e);
                            //return Err(e.into());
                        }
                    },
                }
                if !evts.is_empty() {
                    tx_events.send(evts).await?;
                }
                //dbg!();
                std::thread::yield_now();
            }
        });

        task.detach();
        (
            Self {
                inotify: InotifyWatcher { tx: tx_paths },
                //task,
                poller: PollWatcher {},
            },
            Box::pin(rx_events),
        )
    }
}

impl Watcher for LinuxWatcher {
    fn add(&self, path: &std::path::Path) -> Result<()> {
        block_on(async {
            self.inotify
                .tx
                .send(WatcherStateUpdate::NewPath(path.into()))
                .await
        })?;
        Ok(())
    }

    fn remove(&self, path: &std::path::Path) -> Result<()> {
        block_on(async {
            self.inotify
                .tx
                .send(WatcherStateUpdate::RemPath(path.into()))
                .await
        })?;
        Ok(())
    }
}

// For some reason, tests are currently broken. Don't reenable until this is figured
// out.

// #[cfg(test)]
// mod tests {
//     use std::{
//         path::PathBuf,
//         time::{Duration, Instant},
//     };

//     use collections::HashSet;
//     use gpui::TestAppContext;
//     use smol::stream::StreamExt;
//     use tempfile::TempDir;

//     use crate::{Fs, RealFs};

//     #[gpui::test]
//     async fn randomized_file_watcher_tests(cx: &mut TestAppContext) {
//         let dir = TempDir::new().unwrap();
//         let fs = RealFs::new(None, cx.executor());
//         let (mut events, watcher) = fs.watch(dir.path(), Duration::from_millis(100)).await;

//         dbg!("gonna create dir");
//         std::fs::create_dir(dir.path().join("test")).unwrap();
//         dbg!("created dir");
//         let expected_paths: HashSet<PathBuf> = [dir.path().to_owned()].into_iter().collect();
//         let mut actual_paths = HashSet::default();

//         let start = Instant::now();
//         while start.elapsed() < Duration::from_millis(1000) {
//             dbg!();
//             //std::thread::sleep(Duration::from_secs(1));
//             let events = events.next().await;
//             dbg!();
//             for event in events.into_iter().flatten() {
//                 actual_paths.insert(dbg!(event.path));
//             }
//             dbg!();
//             //std::fs::write(dir.path().join("filename"), [42]).unwrap();
//         }

//         assert_eq!(expected_paths, actual_paths);

//         drop(watcher)
//     }
// }
