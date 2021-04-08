pub use super::fuzzy::PathMatch;
use super::{
    char_bag::CharBag,
    fuzzy::{self, PathEntry},
};
use crate::{
    editor::{History, Snapshot},
    timer,
    util::post_inc,
};
use anyhow::{anyhow, Result};
use crossbeam_channel as channel;
use easy_parallel::Parallel;
use gpui::{scoped_pool, AppContext, Entity, ModelContext, ModelHandle, Task};
use ignore::dir::{Ignore, IgnoreBuilder};
use parking_lot::RwLock;
use smol::prelude::*;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    fmt, fs,
    io::{self, Write},
    os::unix::fs::MetadataExt,
    path::Path,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

#[derive(Clone)]
pub struct Worktree(Arc<RwLock<WorktreeState>>);

struct WorktreeState {
    id: usize,
    path: PathBuf,
    root_ino: Option<usize>,
    entries: HashMap<usize, Entry>,
    file_paths: Vec<PathEntry>,
    histories: HashMap<usize, History>,
    scanning: bool,
}

struct DirToScan {
    id: usize,
    path: PathBuf,
    relative_path: PathBuf,
    ignore: Option<Ignore>,
    dirs_to_scan: channel::Sender<io::Result<DirToScan>>,
}

impl Worktree {
    pub fn new<T>(id: usize, path: T, ctx: Option<&mut ModelContext<Self>>) -> Self
    where
        T: Into<PathBuf>,
    {
        let tree = Self(Arc::new(RwLock::new(WorktreeState {
            id,
            path: path.into(),
            root_ino: None,
            entries: HashMap::new(),
            file_paths: Vec::new(),
            histories: HashMap::new(),
            scanning: ctx.is_some(),
        })));

        if let Some(ctx) = ctx {
            tree.0.write().scanning = true;

            let tree = tree.clone();
            let task = ctx.background_executor().spawn(async move {
                tree.scan_dirs()?;
                Ok(())
            });

            ctx.spawn(task, Self::done_scanning).detach();

            ctx.spawn_stream(
                timer::repeat(Duration::from_millis(100)).map(|_| ()),
                Self::scanning,
                |_, _| {},
            )
            .detach();
        }

        tree
    }

    fn scan_dirs(&self) -> io::Result<()> {
        let path = self.0.read().path.clone();
        let metadata = fs::metadata(&path)?;
        let ino = metadata.ino();
        let is_symlink = fs::symlink_metadata(&path)?.file_type().is_symlink();
        let name = path
            .file_name()
            .map(|name| OsString::from(name))
            .unwrap_or(OsString::from("/"));
        let relative_path = PathBuf::from(&name);

        let mut ignore = IgnoreBuilder::new().build().add_parents(&path).unwrap();
        if metadata.is_dir() {
            ignore = ignore.add_child(&path).unwrap();
        }
        let is_ignored = ignore.matched(&path, metadata.is_dir()).is_ignore();

        self.0.write().root_ino = Some(0);
        if metadata.file_type().is_dir() {
            let is_ignored = is_ignored || name == ".git";
            let id = self.insert_dir(None, name, ino, is_symlink, is_ignored);
            let (tx, rx) = channel::unbounded();

            tx.send(Ok(DirToScan {
                id,
                path,
                relative_path,
                ignore: Some(ignore),
                dirs_to_scan: tx.clone(),
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

        Ok(())
    }

    fn scan_dir(&self, to_scan: DirToScan) -> io::Result<()> {
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

                let id = self.insert_dir(Some(to_scan.id), name, ino, is_symlink, is_ignored);
                new_children.push(id);

                let dirs_to_scan = to_scan.dirs_to_scan.clone();
                let _ = to_scan.dirs_to_scan.send(Ok(DirToScan {
                    id,
                    path,
                    relative_path,
                    ignore,
                    dirs_to_scan,
                }));
            } else {
                let is_ignored = to_scan.ignore.as_ref().map_or(true, |i| {
                    i.matched(to_scan.path.join(&name), false).is_ignore()
                });

                new_children.push(self.insert_file(
                    Some(to_scan.id),
                    name,
                    ino,
                    is_symlink,
                    is_ignored,
                    relative_path,
                ));
            };
        }

        if let Some(Entry::Dir { children, .. }) = &mut self.0.write().entries.get_mut(&to_scan.id)
        {
            *children = new_children.clone();
        }

        Ok(())
    }

    fn insert_dir(
        &self,
        parent: Option<usize>,
        name: OsString,
        ino: u64,
        is_symlink: bool,
        is_ignored: bool,
    ) -> usize {
        let entries = &mut self.0.write().entries;
        let dir_id = entries.len();
        entries.insert(
            dir_id,
            Entry::Dir {
                parent,
                name,
                ino,
                is_symlink,
                is_ignored,
                children: Vec::new(),
            },
        );
        dir_id
    }

    fn insert_file(
        &self,
        parent: Option<usize>,
        name: OsString,
        ino: u64,
        is_symlink: bool,
        is_ignored: bool,
        path: PathBuf,
    ) -> usize {
        let path = path.to_string_lossy();
        let lowercase_path = path.to_lowercase().chars().collect::<Vec<_>>();
        let path = path.chars().collect::<Vec<_>>();
        let path_chars = CharBag::from(&path[..]);

        let mut state = self.0.write();
        let entry_id = state.entries.len();
        state.entries.insert(
            entry_id,
            Entry::File {
                parent,
                name,
                ino,
                is_symlink,
                is_ignored,
            },
        );
        state.file_paths.push(PathEntry {
            entry_id,
            path_chars,
            path,
            lowercase_path,
            is_ignored,
        });
        entry_id
    }

    pub fn entry_path(&self, mut entry_id: usize) -> Result<PathBuf> {
        let state = self.0.read();

        if entry_id >= state.entries.len() {
            return Err(anyhow!("Entry does not exist in tree"));
        }

        let mut entries = Vec::new();
        loop {
            let entry = &state.entries[&entry_id];
            entries.push(entry);
            if let Some(parent_id) = entry.parent() {
                entry_id = parent_id;
            } else {
                break;
            }
        }

        let mut path = PathBuf::new();
        for entry in entries.into_iter().rev() {
            path.push(entry.name());
        }
        Ok(path)
    }

    pub fn abs_entry_path(&self, entry_id: usize) -> Result<PathBuf> {
        let mut path = self.0.read().path.clone();
        path.pop();
        Ok(path.join(self.entry_path(entry_id)?))
    }

    fn fmt_entry(&self, f: &mut fmt::Formatter<'_>, entry_id: usize, indent: usize) -> fmt::Result {
        match &self.0.read().entries[&entry_id] {
            Entry::Dir { name, children, .. } => {
                write!(
                    f,
                    "{}{}/ ({})\n",
                    " ".repeat(indent),
                    name.to_string_lossy(),
                    entry_id
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
                entry_id
            ),
        }
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from(&self.0.read().path)
    }

    pub fn contains_path(&self, path: &Path) -> bool {
        path.starts_with(self.path())
    }

    pub fn iter(&self) -> Iter {
        Iter {
            tree: self.clone(),
            stack: Vec::new(),
            started: false,
        }
    }

    pub fn files(&self) -> FilesIter {
        FilesIter {
            iter: self.iter(),
            path: PathBuf::new(),
        }
    }

    pub fn entry_count(&self) -> usize {
        self.0.read().entries.len()
    }

    pub fn file_count(&self) -> usize {
        self.0.read().file_paths.len()
    }

    pub fn load_history(&self, entry_id: usize) -> impl Future<Output = Result<History>> {
        let tree = self.clone();

        async move {
            if let Some(history) = tree.0.read().histories.get(&entry_id) {
                return Ok(history.clone());
            }

            let path = tree.abs_entry_path(entry_id)?;

            let mut file = smol::fs::File::open(&path).await?;
            let mut base_text = String::new();
            file.read_to_string(&mut base_text).await?;
            let history = History::new(Arc::from(base_text));
            tree.0.write().histories.insert(entry_id, history.clone());
            Ok(history)
        }
    }

    pub fn save<'a>(
        &self,
        entry_id: usize,
        content: Snapshot,
        ctx: &AppContext,
    ) -> Task<Result<()>> {
        let path = self.abs_entry_path(entry_id);
        ctx.background_executor().spawn(async move {
            let buffer_size = content.text_summary().bytes.min(10 * 1024);
            let file = std::fs::File::create(&path?)?;
            let mut writer = std::io::BufWriter::with_capacity(buffer_size, file);
            for chunk in content.fragments() {
                writer.write(chunk.as_bytes())?;
            }
            writer.flush()?;
            Ok(())
        })
    }

    fn scanning(&mut self, _: (), ctx: &mut ModelContext<Self>) {
        if self.0.read().scanning {
            ctx.notify();
        } else {
            ctx.halt_stream();
        }
    }

    fn done_scanning(&mut self, result: io::Result<()>, ctx: &mut ModelContext<Self>) {
        log::info!("done scanning");
        self.0.write().scanning = false;
        if let Err(error) = result {
            log::error!("error populating worktree: {}", error);
        } else {
            ctx.notify();
        }
    }
}

impl fmt::Debug for Worktree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.entry_count() == 0 {
            write!(f, "Empty tree\n")
        } else {
            self.fmt_entry(f, 0, 0)
        }
    }
}

impl Entity for Worktree {
    type Event = ();
}

impl WorktreeState {
    fn root_entry(&self) -> Option<&Entry> {
        self.root_ino
            .and_then(|root_ino| self.entries.get(&root_ino))
    }
}

pub trait WorktreeHandle {
    fn file(&self, entry_id: usize, app: &AppContext) -> Result<FileHandle>;
}

impl WorktreeHandle for ModelHandle<Worktree> {
    fn file(&self, entry_id: usize, app: &AppContext) -> Result<FileHandle> {
        if entry_id >= self.read(app).entry_count() {
            return Err(anyhow!("Entry does not exist in tree"));
        }

        Ok(FileHandle {
            worktree: self.clone(),
            entry_id,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Entry {
    Dir {
        parent: Option<usize>,
        name: OsString,
        ino: u64,
        is_symlink: bool,
        is_ignored: bool,
        children: Vec<usize>,
    },
    File {
        parent: Option<usize>,
        name: OsString,
        ino: u64,
        is_symlink: bool,
        is_ignored: bool,
    },
}

impl Entry {
    fn parent(&self) -> Option<usize> {
        match self {
            Entry::Dir { parent, .. } | Entry::File { parent, .. } => *parent,
        }
    }

    fn name(&self) -> &OsStr {
        match self {
            Entry::Dir { name, .. } | Entry::File { name, .. } => name,
        }
    }
}

#[derive(Clone)]
pub struct FileHandle {
    worktree: ModelHandle<Worktree>,
    entry_id: usize,
}

impl FileHandle {
    pub fn path(&self, app: &AppContext) -> PathBuf {
        self.worktree.read(app).entry_path(self.entry_id).unwrap()
    }

    pub fn load_history(&self, app: &AppContext) -> impl Future<Output = Result<History>> {
        self.worktree.read(app).load_history(self.entry_id)
    }

    pub fn save<'a>(&self, content: Snapshot, ctx: &AppContext) -> Task<Result<()>> {
        let worktree = self.worktree.read(ctx);
        worktree.save(self.entry_id, content, ctx)
    }

    pub fn entry_id(&self) -> (usize, usize) {
        (self.worktree.id(), self.entry_id)
    }
}

struct IterStackEntry {
    entry_id: usize,
    child_idx: usize,
}

pub struct Iter {
    tree: Worktree,
    stack: Vec<IterStackEntry>,
    started: bool,
}

impl Iterator for Iter {
    type Item = Traversal;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.tree.0.read();

        if !self.started {
            self.started = true;

            return if let Some(entry) = state.root_entry().cloned() {
                self.stack.push(IterStackEntry {
                    entry_id: 0,
                    child_idx: 0,
                });

                Some(Traversal::Push { entry_id: 0, entry })
            } else {
                None
            };
        }

        while let Some(parent) = self.stack.last_mut() {
            if let Entry::Dir { children, .. } = &state.entries[&parent.entry_id] {
                if parent.child_idx < children.len() {
                    let child_id = children[post_inc(&mut parent.child_idx)];

                    self.stack.push(IterStackEntry {
                        entry_id: child_id,
                        child_idx: 0,
                    });

                    return Some(Traversal::Push {
                        entry_id: child_id,
                        entry: state.entries[&child_id].clone(),
                    });
                } else {
                    self.stack.pop();

                    return Some(Traversal::Pop);
                }
            } else {
                self.stack.pop();

                return Some(Traversal::Pop);
            }
        }

        None
    }
}

#[derive(Debug)]
pub enum Traversal {
    Push { entry_id: usize, entry: Entry },
    Pop,
}

pub struct FilesIter {
    iter: Iter,
    path: PathBuf,
}

pub struct FilesIterItem {
    pub entry_id: usize,
    pub path: PathBuf,
}

impl Iterator for FilesIter {
    type Item = FilesIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Traversal::Push {
                    entry_id, entry, ..
                }) => match entry {
                    Entry::Dir { name, .. } => {
                        self.path.push(name);
                    }
                    Entry::File { name, .. } => {
                        self.path.push(name);
                        return Some(FilesIterItem {
                            entry_id,
                            path: self.path.clone(),
                        });
                    }
                },
                Some(Traversal::Pop) => {
                    self.path.pop();
                }
                None => {
                    return None;
                }
            }
        }
    }
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

pub fn match_paths(
    trees: &[Worktree],
    query: &str,
    include_ignored: bool,
    smart_case: bool,
    max_results: usize,
    pool: scoped_pool::Pool,
) -> Vec<PathMatch> {
    let tree_states = trees.iter().map(|tree| tree.0.read()).collect::<Vec<_>>();
    fuzzy::match_paths(
        &tree_states
            .iter()
            .map(|tree| {
                let skip_prefix = if trees.len() == 1 {
                    if let Some(Entry::Dir { name, .. }) = tree.root_entry() {
                        let name = name.to_string_lossy();
                        if name == "/" {
                            1
                        } else {
                            name.chars().count() + 1
                        }
                    } else {
                        0
                    }
                } else {
                    0
                };

                (tree.id, skip_prefix, &tree.file_paths[..])
            })
            .collect::<Vec<_>>()[..],
        query,
        include_ignored,
        smart_case,
        max_results,
        pool,
    )
}

#[cfg(test)]
mod test {
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

            let tree = app.add_model(|ctx| Worktree::new(1, root_link_path, Some(ctx)));
            app.finish_pending_tasks().await;

            app.read(|ctx| {
                let tree = tree.read(ctx);
                assert_eq!(tree.file_count(), 4);
                let results = match_paths(&[tree.clone()], "bna", false, false, 10, ctx.scoped_pool().clone())
                    .iter()
                    .map(|result| tree.entry_path(result.entry_id))
                    .collect::<Result<Vec<PathBuf>, _>>()
                    .unwrap();
                assert_eq!(
                    results,
                    vec![
                        PathBuf::from("root_link/banana/carrot/date"),
                        PathBuf::from("root_link/banana/carrot/endive"),
                    ]
                );
            })
        });
    }

    #[test]
    fn test_save_file() {
        App::test_async((), |mut app| async move {
            let dir = temp_tree(json!({
                "file1": "the old contents",
            }));

            let tree = app.add_model(|ctx| Worktree::new(1, dir.path(), Some(ctx)));
            app.finish_pending_tasks().await;

            let buffer = Buffer::new(1, "a line of text.\n".repeat(10 * 1024));

            let entry = app.read(|ctx| {
                let entry = tree.read(ctx).files().next().unwrap();
                assert_eq!(entry.path.file_name().unwrap(), "file1");
                entry
            });
            let file_id = entry.entry_id;

            tree.update(&mut app, |tree, ctx| {
                smol::block_on(tree.save(file_id, buffer.snapshot(), ctx.as_ref())).unwrap()
            });

            let history = app
                .read(|ctx| tree.read(ctx).load_history(file_id))
                .await
                .unwrap();
            assert_eq!(history.base_text.as_ref(), buffer.text());
        });
    }
}
