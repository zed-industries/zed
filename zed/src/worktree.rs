use crate::sum_tree::{self, Edit, SumTree};
use gpui::{Entity, ModelContext};
use ignore::dir::{Ignore, IgnoreBuilder};
use parking_lot::Mutex;
use smol::channel::Sender;
use std::{
    ffi::{OsStr, OsString},
    fs, io,
    ops::AddAssign,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::Arc,
};

enum ScanState {
    Idle,
    Scanning,
}

pub struct Worktree {
    path: Arc<Path>,
    entries: SumTree<Entry>,
    scanner: BackgroundScanner,
    scan_state: ScanState,
}

impl Worktree {
    fn new(path: impl Into<Arc<Path>>, ctx: &mut ModelContext<Self>) -> Self {
        let path = path.into();
        let scan_state = smol::channel::unbounded();
        let scanner = BackgroundScanner::new(path.clone(), scan_state.0);
        let tree = Self {
            path,
            entries: Default::default(),
            scanner,
            scan_state: ScanState::Idle,
        };

        {
            let scanner = tree.scanner.clone();
            std::thread::spawn(move || scanner.run());
        }

        tree
    }
}

impl Entity for Worktree {
    type Event = ();
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
    entries: Arc<Mutex<SumTree<Entry>>>,
    notify: Sender<ScanState>,
}

impl BackgroundScanner {
    fn new(path: Arc<Path>, notify: Sender<ScanState>) -> Self {
        Self {
            path,
            entries: Default::default(),
            notify,
        }
    }

    fn run(&self) {
        if smol::block_on(self.notify.send(ScanState::Scanning)).is_err() {
            return;
        }

        self.scan_dirs();

        if smol::block_on(self.notify.send(ScanState::Idle)).is_err() {
            return;
        }

        // TODO: Update when dir changes
    }

    fn scan_dirs(&self) -> io::Result<()> {
        let metadata = fs::metadata(&self.path)?;
        let ino = metadata.ino();
        let is_symlink = fs::symlink_metadata(&self.path)?.file_type().is_symlink();
        let name = self.path.file_name().unwrap_or(OsStr::new("/")).into();
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
            let is_ignored = is_ignored || name == ".git";

            self.insert_entries(Some(Entry::Dir {
                parent: None,
                name,
                ino,
                is_symlink,
                is_ignored,
                children: Arc::from([]),
                pending: true,
            }));

            let (tx, rx) = crossbeam_channel::unbounded();

            tx.send(Ok(ScanJob {
                ino,
                path: path.into(),
                relative_path,
                ignore: Some(ignore),
                scan_queue: tx.clone(),
            }))
            .unwrap();
            drop(tx);

            Parallel::<io::Result<()>>::new()
                .each(0..16, |_| {
                    while let Ok(result) = rx.recv() {
                        self.scan_dir(result?)?;
                    }
                    Ok(())
                })
                .run()
                .into_iter()
                .collect::<io::Result<()>>()?;
        } else {
            self.insert_file(None, name, ino, is_symlink, is_ignored, relative_path);
        }
        self.0.write().root_ino = Some(ino);

        Ok(())
    }

    fn scan_dir(&self, to_scan: ScanJob) -> io::Result<()> {
        let mut new_children = Vec::new();

        for child_entry in fs::read_dir(&to_scan.path)? {
            let child_entry = child_entry?;
            let name = child_entry.file_name();
            let relative_path = to_scan.relative_path.join(&name);
            let metadata = child_entry.metadata()?;
            let ino = metadata.ino();
            let is_symlink = metadata.file_type().is_symlink();

            if metadata.is_dir() {
                let path = to_scan.path.join(&name);
                let mut is_ignored = true;
                let mut ignore = None;

                if let Some(parent_ignore) = to_scan.ignore.as_ref() {
                    let child_ignore = parent_ignore.add_child(&path).unwrap();
                    is_ignored = child_ignore.matched(&path, true).is_ignore() || name == ".git";
                    if !is_ignored {
                        ignore = Some(child_ignore);
                    }
                }

                self.insert_entries(
                    Some(Entry::Dir {
                        parent: (),
                        name: (),
                        ino: (),
                        is_symlink: (),
                        is_ignored: (),
                        children: (),
                        pending: (),
                    })
                    .into_iter(),
                );

                self.insert_dir(Some(to_scan.ino), name, ino, is_symlink, is_ignored);
                new_children.push(ino);

                let dirs_to_scan = to_scan.scan_queue.clone();
                let _ = to_scan.scan_queue.send(Ok(ScanJob {
                    ino,
                    path,
                    relative_path,
                    ignore,
                    scan_queue: dirs_to_scan,
                }));
            } else {
                let is_ignored = to_scan.ignore.as_ref().map_or(true, |i| {
                    i.matched(to_scan.path.join(&name), false).is_ignore()
                });

                self.insert_file(
                    Some(to_scan.ino),
                    name,
                    ino,
                    is_symlink,
                    is_ignored,
                    relative_path,
                );
                new_children.push(ino);
            };
        }

        if let Some(Entry::Dir { children, .. }) = &mut self.0.write().entries.get_mut(&to_scan.ino)
        {
            *children = new_children.clone();
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
