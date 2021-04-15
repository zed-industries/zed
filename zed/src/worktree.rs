use crate::sum_tree::{self, Edit, SumTree};
use gpui::{scoped_pool, Entity, ModelContext};
use ignore::dir::{Ignore, IgnoreBuilder};
use parking_lot::Mutex;
use smol::{channel::Sender, Timer};
use std::{
    ffi::{OsStr, OsString},
    fmt, fs, io,
    ops::AddAssign,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::{
        atomic::{self, AtomicU64},
        Arc,
    },
    time::{Duration, Instant},
};

#[derive(Debug)]
enum ScanState {
    Idle,
    Scanning,
    Err(io::Error),
}

pub struct Worktree {
    path: Arc<Path>,
    entries: SumTree<Entry>,
    scanner: BackgroundScanner,
    scan_state: ScanState,
    will_poll_entries: bool,
}

impl Worktree {
    fn new(path: impl Into<Arc<Path>>, ctx: &mut ModelContext<Self>) -> Self {
        let path = path.into();
        let scan_state = smol::channel::unbounded();
        let scanner = BackgroundScanner::new(path.clone(), scan_state.0, ctx.thread_pool().clone());
        let tree = Self {
            path,
            entries: Default::default(),
            scanner,
            scan_state: ScanState::Idle,
            will_poll_entries: false,
        };

        let scanner = tree.scanner.clone();
        std::thread::spawn(move || scanner.run());

        ctx.spawn_stream(scan_state.1, Self::observe_scan_state, |_, _| {})
            .detach();

        tree
    }

    fn observe_scan_state(&mut self, scan_state: ScanState, ctx: &mut ModelContext<Self>) {
        self.scan_state = scan_state;
        self.poll_entries(ctx);
        ctx.notify();
    }

    fn poll_entries(&mut self, ctx: &mut ModelContext<Self>) {
        self.entries = self.scanner.snapshot();
        if self.is_scanning() && !self.will_poll_entries {
            self.will_poll_entries = true;
            ctx.spawn(Timer::after(Duration::from_millis(100)), |this, _, ctx| {
                this.will_poll_entries = false;
                this.poll_entries(ctx);
            })
            .detach();
        }
    }

    fn is_scanning(&self) -> bool {
        if let ScanState::Scanning = self.scan_state {
            true
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        self.root_ino() == 0
    }

    fn root_ino(&self) -> u64 {
        self.scanner.root_ino.load(atomic::Ordering::SeqCst)
    }

    fn file_count(&self) -> usize {
        self.entries.summary().file_count
    }

    fn fmt_entry(&self, f: &mut fmt::Formatter<'_>, ino: u64, indent: usize) -> fmt::Result {
        match self.entries.get(&ino).unwrap() {
            Entry::Dir { name, children, .. } => {
                write!(
                    f,
                    "{}{}/ ({})\n",
                    " ".repeat(indent),
                    name.to_string_lossy(),
                    ino
                )?;
                for child_id in children.iter() {
                    self.fmt_entry(f, *child_id, indent + 2)?;
                }
                Ok(())
            }
            Entry::File { name, .. } => write!(
                f,
                "{}{} ({})\n",
                " ".repeat(indent),
                name.to_string_lossy(),
                ino
            ),
        }
    }
}

impl Entity for Worktree {
    type Event = ();
}

impl fmt::Debug for Worktree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "Empty tree\n")
        } else {
            self.fmt_entry(f, self.root_ino(), 0)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Entry {
    Dir {
        parent: Option<u64>,
        name: Arc<OsStr>,
        ino: u64,
        is_symlink: bool,
        is_ignored: bool,
        children: Arc<[u64]>,
        pending: bool,
    },
    File {
        parent: Option<u64>,
        name: Arc<OsStr>,
        ino: u64,
        is_symlink: bool,
        is_ignored: bool,
    },
}

impl Entry {
    fn ino(&self) -> u64 {
        match self {
            Entry::Dir { ino, .. } => *ino,
            Entry::File { ino, .. } => *ino,
        }
    }
}

impl sum_tree::Item for Entry {
    type Summary = EntrySummary;

    fn summary(&self) -> Self::Summary {
        EntrySummary {
            max_ino: self.ino(),
            file_count: if matches!(self, Self::File { .. }) {
                1
            } else {
                0
            },
        }
    }
}

impl sum_tree::KeyedItem for Entry {
    type Key = u64;

    fn key(&self) -> Self::Key {
        self.ino()
    }
}

#[derive(Clone, Debug, Default)]
pub struct EntrySummary {
    max_ino: u64,
    file_count: usize,
}

impl<'a> AddAssign<&'a EntrySummary> for EntrySummary {
    fn add_assign(&mut self, rhs: &'a EntrySummary) {
        self.max_ino = rhs.max_ino;
        self.file_count += rhs.file_count;
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for u64 {
    fn add_summary(&mut self, summary: &'a EntrySummary) {
        *self = summary.max_ino;
    }
}

#[derive(Clone)]
struct BackgroundScanner {
    path: Arc<Path>,
    root_ino: Arc<AtomicU64>,
    entries: Arc<Mutex<SumTree<Entry>>>,
    notify: Sender<ScanState>,
    thread_pool: scoped_pool::Pool,
}

impl BackgroundScanner {
    fn new(path: Arc<Path>, notify: Sender<ScanState>, thread_pool: scoped_pool::Pool) -> Self {
        Self {
            path,
            root_ino: Arc::new(AtomicU64::new(0)),
            entries: Default::default(),
            notify,
            thread_pool,
        }
    }

    fn snapshot(&self) -> SumTree<Entry> {
        self.entries.lock().clone()
    }

    fn run(&self) {
        if smol::block_on(self.notify.send(ScanState::Scanning)).is_err() {
            return;
        }

        if let Err(err) = self.scan_dirs() {
            if smol::block_on(self.notify.send(ScanState::Err(err))).is_err() {
                return;
            }
        }

        if smol::block_on(self.notify.send(ScanState::Idle)).is_err() {
            return;
        }

        // TODO: Update when dir changes
    }

    fn scan_dirs(&self) -> io::Result<()> {
        println!("Scanning dirs ;)");
        let metadata = fs::metadata(&self.path)?;
        let ino = metadata.ino();
        let is_symlink = fs::symlink_metadata(&self.path)?.file_type().is_symlink();
        let name = Arc::from(self.path.file_name().unwrap_or(OsStr::new("/")));
        let relative_path = PathBuf::from(&name);

        let mut ignore = IgnoreBuilder::new()
            .build()
            .add_parents(&self.path)
            .unwrap();
        if metadata.is_dir() {
            ignore = ignore.add_child(&self.path).unwrap();
        }
        let is_ignored = ignore.matched(&self.path, metadata.is_dir()).is_ignore();

        if metadata.file_type().is_dir() {
            let is_ignored = is_ignored || name.as_ref() == ".git";
            let dir_entry = Entry::Dir {
                parent: None,
                name,
                ino,
                is_symlink,
                is_ignored,
                children: Arc::from([]),
                pending: true,
            };
            self.insert_entries(Some(dir_entry.clone()));

            let (tx, rx) = crossbeam_channel::unbounded();

            tx.send(Ok(ScanJob {
                ino,
                path: self.path.clone(),
                relative_path,
                dir_entry,
                ignore: Some(ignore),
                scan_queue: tx.clone(),
            }))
            .unwrap();
            drop(tx);

            let mut results = Vec::new();
            results.resize_with(16, || Ok(()));
            self.thread_pool.scoped(|pool| {
                for result in &mut results {
                    pool.execute(|| {
                        let result = result;
                        while let Ok(job) = rx.recv() {
                            if let Err(err) = job.and_then(|job| self.scan_dir(job)) {
                                *result = Err(err);
                                break;
                            }
                        }
                    });
                }
            });
            results.into_iter().collect::<io::Result<()>>()?;
        } else {
            self.insert_entries(Some(Entry::File {
                parent: None,
                name,
                ino,
                is_symlink,
                is_ignored,
            }));
        }

        self.root_ino.store(ino, atomic::Ordering::SeqCst);

        Ok(())
    }

    fn scan_dir(&self, job: ScanJob) -> io::Result<()> {
        let scan_queue = job.scan_queue;
        let mut dir_entry = job.dir_entry;

        let mut new_children = Vec::new();
        let mut new_entries = Vec::new();
        let mut new_jobs = Vec::new();

        for child_entry in fs::read_dir(&job.path)? {
            let child_entry = child_entry?;
            let name: Arc<OsStr> = child_entry.file_name().into();
            let relative_path = job.relative_path.join(name.as_ref());
            let metadata = child_entry.metadata()?;
            let ino = metadata.ino();
            let is_symlink = metadata.file_type().is_symlink();
            let path = job.path.join(name.as_ref());

            new_children.push(ino);
            if metadata.is_dir() {
                let mut is_ignored = true;
                let mut ignore = None;

                if let Some(parent_ignore) = job.ignore.as_ref() {
                    let child_ignore = parent_ignore.add_child(&path).unwrap();
                    is_ignored =
                        child_ignore.matched(&path, true).is_ignore() || name.as_ref() == ".git";
                    if !is_ignored {
                        ignore = Some(child_ignore);
                    }
                }

                let dir_entry = Entry::Dir {
                    parent: Some(job.ino),
                    name,
                    ino,
                    is_symlink,
                    is_ignored,
                    children: Arc::from([]),
                    pending: true,
                };
                new_entries.push(dir_entry.clone());
                new_jobs.push(ScanJob {
                    ino,
                    path: Arc::from(path),
                    relative_path,
                    dir_entry,
                    ignore,
                    scan_queue: scan_queue.clone(),
                });
            } else {
                let is_ignored = job
                    .ignore
                    .as_ref()
                    .map_or(true, |i| i.matched(path, false).is_ignore());
                new_entries.push(Entry::File {
                    parent: Some(job.ino),
                    name,
                    ino,
                    is_symlink,
                    is_ignored,
                });
            };
        }

        if let Entry::Dir {
            children, pending, ..
        } = &mut dir_entry
        {
            *children = Arc::from(new_children);
            *pending = false;
        } else {
            unreachable!()
        }
        new_entries.push(dir_entry);

        self.insert_entries(new_entries);
        for new_job in new_jobs {
            let _ = scan_queue.send(Ok(new_job));
        }

        Ok(())
    }

    fn insert_entries(&self, entries: impl IntoIterator<Item = Entry>) {
        self.entries
            .lock()
            .edit(&mut entries.into_iter().map(Edit::Insert).collect::<Vec<_>>());
    }
}

struct ScanJob {
    ino: u64,
    path: Arc<Path>,
    relative_path: PathBuf,
    dir_entry: Entry,
    ignore: Option<Ignore>,
    scan_queue: crossbeam_channel::Sender<io::Result<ScanJob>>,
}

trait UnwrapIgnoreTuple {
    fn unwrap(self) -> Ignore;
}

impl UnwrapIgnoreTuple for (Ignore, Option<ignore::Error>) {
    fn unwrap(self) -> Ignore {
        if let Some(error) = self.1 {
            log::error!("error loading gitignore data: {}", error);
        }
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::Buffer;
    use crate::test::*;
    use anyhow::Result;
    use gpui::App;
    use serde_json::json;
    use std::os::unix;

    #[test]
    fn test_populate_and_search() {
        App::test_async((), |mut app| async move {
            let dir = temp_tree(json!({
                "root": {
                    "apple": "",
                    "banana": {
                        "carrot": {
                            "date": "",
                            "endive": "",
                        }
                    },
                    "fennel": {
                        "grape": "",
                    }
                }
            }));

            let root_link_path = dir.path().join("root_link");
            unix::fs::symlink(&dir.path().join("root"), &root_link_path).unwrap();

            let tree = app.add_model(|ctx| Worktree::new(root_link_path, ctx));
            assert_condition(1, 300, || app.read(|ctx| tree.read(ctx).file_count() == 4)).await;
            // app.read(|ctx| {
            //     let tree = tree.read(ctx);
            //     assert_eq!(tree.file_count(), 4);
            //     let results = match_paths(
            //         &[tree.clone()],
            //         "bna",
            //         false,
            //         false,
            //         10,
            //         ctx.thread_pool().clone(),
            //     )
            //     .iter()
            //     .map(|result| tree.entry_path(result.entry_id))
            //     .collect::<Result<Vec<PathBuf>, _>>()
            //     .unwrap();
            //     assert_eq!(
            //         results,
            //         vec![
            //             PathBuf::from("root_link/banana/carrot/date"),
            //             PathBuf::from("root_link/banana/carrot/endive"),
            //         ]
            //     );
            // })
        });
    }
}
