//! Watcher implementation for the kqueue API
//!
//! The kqueue() system call provides a generic method of notifying the user
//! when an event happens or a condition holds, based on the results of small
//! pieces of kernel code termed filters.

use super::event::*;
use super::{
    Config, Error, ErrorKind, EventHandler, EventKindMask, RecursiveMode, Result, Watcher,
};
use crate::paths::{
    absolute_path, is_preserved_watch_root, preserved_watch_mode, preserved_watch_roots,
    recursive_user_watch_ancestor, reported_path, WatchMetadata as Watch, WatchPath,
};
use crate::{unbounded, Receiver, Sender};
use kqueue::{EventData, EventFilter, FilterFlag, Ident};
use std::collections::HashMap;
use std::fs::metadata;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use walkdir::WalkDir;

const KQUEUE: mio::Token = mio::Token(0);
const MESSAGE: mio::Token = mio::Token(1);

// The EventLoop will set up a mio::Poll and use it to wait for the following:
//
// -  messages telling it what to do
//
// -  events telling it that something has happened on one of the watched files.
struct EventLoop {
    running: bool,
    poll: mio::Poll,
    event_loop_waker: Arc<mio::Waker>,
    event_loop_tx: Sender<EventLoopMsg>,
    event_loop_rx: Receiver<EventLoopMsg>,
    kqueue: kqueue::Watcher,
    event_handler: Box<dyn EventHandler>,
    watches: HashMap<PathBuf, Watch>,
    follow_symlinks: bool,
    event_kinds: EventKindMask,
}

/// Watcher implementation based on inotify
#[derive(Debug)]
pub struct KqueueWatcher {
    channel: Sender<EventLoopMsg>,
    waker: Arc<mio::Waker>,
}

enum EventLoopMsg {
    AddWatch(WatchPath, RecursiveMode, Sender<Result<()>>),
    RemoveWatch(PathBuf, Sender<Result<()>>),
    GetWatchedPaths(Sender<Vec<(PathBuf, RecursiveMode)>>),
    Shutdown,
}

impl EventLoop {
    pub fn new(
        kqueue: kqueue::Watcher,
        event_handler: Box<dyn EventHandler>,
        follow_symlinks: bool,
        event_kinds: EventKindMask,
    ) -> Result<Self> {
        let (event_loop_tx, event_loop_rx) = unbounded::<EventLoopMsg>();
        let poll = mio::Poll::new()?;

        let event_loop_waker = Arc::new(mio::Waker::new(poll.registry(), MESSAGE)?);

        let kqueue_fd = kqueue.as_raw_fd();
        let mut evented_kqueue = mio::unix::SourceFd(&kqueue_fd);
        poll.registry()
            .register(&mut evented_kqueue, KQUEUE, mio::Interest::READABLE)?;

        let event_loop = EventLoop {
            running: true,
            poll,
            event_loop_waker,
            event_loop_tx,
            event_loop_rx,
            kqueue,
            event_handler,
            watches: HashMap::new(),
            follow_symlinks,
            event_kinds,
        };
        Ok(event_loop)
    }

    // Run the event loop.
    pub fn run(self) {
        let _ = thread::Builder::new()
            .name("notify-rs kqueue loop".to_string())
            .spawn(|| self.event_loop_thread());
    }

    fn event_loop_thread(mut self) {
        let mut events = mio::Events::with_capacity(16);
        loop {
            // Wait for something to happen.
            match self.poll.poll(&mut events, None) {
                Err(ref e) if matches!(e.kind(), std::io::ErrorKind::Interrupted) => {
                    // System call was interrupted, we will retry
                    // TODO: Not covered by tests (to reproduce likely need to setup signal handlers)
                }
                Err(e) => panic!("poll failed: {e}"),
                Ok(()) => {}
            }

            // Process whatever happened.
            for event in &events {
                self.handle_event(event);
            }

            // Stop, if we're done.
            if !self.running {
                break;
            }
        }
    }

    // Handle a single event.
    fn handle_event(&mut self, event: &mio::event::Event) {
        match event.token() {
            MESSAGE => {
                // The channel is readable - handle messages.
                self.handle_messages()
            }
            KQUEUE => {
                // inotify has something to tell us.
                self.handle_kqueue()
            }
            _ => unreachable!(),
        }
    }

    fn handle_messages(&mut self) {
        while let Ok(msg) = self.event_loop_rx.try_recv() {
            match msg {
                EventLoopMsg::AddWatch(path, recursive_mode, tx) => {
                    let _ = tx.send(self.add_watch(path, recursive_mode.is_recursive(), true));
                }
                EventLoopMsg::RemoveWatch(path, tx) => {
                    let _ = tx.send(self.remove_watch(path, false));
                }
                EventLoopMsg::GetWatchedPaths(tx) => {
                    let _ = tx.send(
                        self.watches
                            .iter()
                            .filter(|(_path, watch)| watch.is_user_watch)
                            .map(|(_path, watch)| {
                                (
                                    watch.reported_path.clone(),
                                    if watch.user_is_recursive {
                                        RecursiveMode::Recursive
                                    } else {
                                        RecursiveMode::NonRecursive
                                    },
                                )
                            })
                            .collect(),
                    );
                }
                EventLoopMsg::Shutdown => {
                    self.running = false;
                    break;
                }
            }
        }
    }

    fn handle_kqueue(&mut self) {
        let mut add_watches = Vec::new();
        let mut remove_watches = Vec::new();

        while let Some(event) = self.kqueue.poll(None) {
            log::trace!("kqueue event: {event:?}");

            match event {
                kqueue::Event {
                    data: EventData::Vnode(data),
                    ident: Ident::Filename(_, path),
                } => {
                    let path = PathBuf::from(path);
                    let watch = self.watches.get(&path);
                    let event_path = watch
                        .map(|watch| watch.reported_path.clone())
                        .unwrap_or_else(|| path.clone());
                    let is_user_watch = watch.is_some_and(|watch| watch.is_user_watch);
                    let event = match data {
                        /*
                        TODO: Differentiate folders and files
                        kqueue doesn't tell us if this was a file or a dir, so we
                        could only emulate this inotify behavior if we keep track of
                        all files and directories internally and then perform a
                        lookup.
                        */
                        kqueue::Vnode::Delete => {
                            remove_watches.push((path.clone(), true));
                            Ok(Event::new(EventKind::Remove(RemoveKind::Any)).add_path(event_path))
                        }

                        // A write to a recursively watched directory may mean that a new file
                        // was created in it. Non-recursive directory watches do not track
                        // children, so guessing from read_dir would be unreliable.
                        // FIXME: harden guessing for non-recursive watches.
                        // Context: https://github.com/notify-rs/notify/issues/644
                        kqueue::Vnode::Write
                            if watch.is_some_and(|watch| watch.is_recursive)
                                && if self.follow_symlinks {
                                    path.is_dir()
                                } else {
                                    std::fs::symlink_metadata(&path)
                                        .is_ok_and(|metadata| metadata.is_dir())
                                } =>
                        {
                            // find which file is new in the directory by comparing it with our
                            // list of known watches
                            std::fs::read_dir(&path)
                                .map(|dir| {
                                    dir.filter_map(std::result::Result::ok)
                                        .map(|f| f.path())
                                        .find(|f| !self.watches.contains_key(f))
                                })
                                .map(|file| {
                                    if let Some(file) = file {
                                        // watch this new file
                                        let reported_file =
                                            reported_path(&path, &event_path, &file);
                                        add_watches.push((
                                            WatchPath::from_parts(
                                                file.clone(),
                                                reported_file.clone(),
                                            ),
                                            false,
                                        ));

                                        Event::new(EventKind::Create(if file.is_dir() {
                                            CreateKind::Folder
                                        } else if file.is_file() {
                                            CreateKind::File
                                        } else {
                                            CreateKind::Other
                                        }))
                                        .add_path(reported_file)
                                    } else {
                                        Event::new(EventKind::Modify(ModifyKind::Data(
                                            DataChange::Any,
                                        )))
                                        .add_path(event_path)
                                    }
                                })
                                .map_err(Into::into)
                        }

                        // data was written to this file
                        kqueue::Vnode::Write => Ok(Event::new(EventKind::Modify(
                            ModifyKind::Data(DataChange::Any),
                        ))
                        .add_path(event_path)),

                        /*
                        Extend and Truncate are just different names for the same
                        operation, extend is only used on FreeBSD, truncate everywhere
                        else
                        */
                        kqueue::Vnode::Extend | kqueue::Vnode::Truncate => Ok(Event::new(
                            EventKind::Modify(ModifyKind::Data(DataChange::Size)),
                        )
                        .add_path(event_path)),

                        /*
                        this kevent has the same problem as the delete kevent. The
                        only way i can think of providing "better" event with more
                        information is to do the diff our self, while this maybe do
                        able of delete. In this case it would somewhat expensive to
                        keep track and compare ever peace of metadata for every file
                        */
                        kqueue::Vnode::Attrib => Ok(Event::new(EventKind::Modify(
                            ModifyKind::Metadata(MetadataKind::Any),
                        ))
                        .add_path(event_path)),

                        /*
                        The link count on a file changed => subdirectory created or
                        delete.
                        */
                        kqueue::Vnode::Link => {
                            // As we currently don't have a solution that would allow us
                            // to only add/remove the new/delete directory and that dosn't include a
                            // possible race condition. On possible solution would be to
                            // create a `HashMap<PathBuf, Vec<PathBuf>>` which would
                            // include every directory and this content add the time of
                            // adding it to kqueue. While this should allow us to do the
                            // diff and only add/remove the files necessary. This would
                            // also introduce a race condition, where multiple files could
                            // all ready be remove from the directory, and we could get out
                            // of sync.
                            // So for now, until we find a better solution, let remove and
                            // readd the whole directory.
                            // This is a expensive operation, as we recursive through all
                            // subdirectories.
                            remove_watches.push((path.clone(), false));
                            add_watches.push((
                                WatchPath::from_parts(path.clone(), event_path.clone()),
                                is_user_watch,
                            ));
                            Ok(Event::new(EventKind::Modify(ModifyKind::Any)).add_path(event_path))
                        }

                        // Kqueue not provide us with the information necessary to provide
                        // the new file name to the event.
                        kqueue::Vnode::Rename => {
                            remove_watches.push((path.clone(), true));
                            Ok(
                                Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Any)))
                                    .add_path(event_path),
                            )
                        }

                        // Access to the file was revoked via revoke(2) or the underlying file system was unmounted.
                        kqueue::Vnode::Revoke => {
                            remove_watches.push((path.clone(), true));
                            Ok(Event::new(EventKind::Remove(RemoveKind::Any)).add_path(event_path))
                        }

                        // On different BSD variants, different extra events may be present
                        #[allow(unreachable_patterns)]
                        _ => Ok(Event::new(EventKind::Other)),
                    };
                    // Filter events based on EventKindMask
                    // Errors always pass through, OK events only if they match the mask
                    match &event {
                        Ok(e) if !self.event_kinds.matches(&e.kind) => {
                            // Event filtered out
                        }
                        _ => self.event_handler.handle_event(event),
                    }
                }
                // as we don't add any other EVFILTER to kqueue we should never get here
                kqueue::Event { ident: _, data: _ } => unreachable!(),
            }
        }

        for (path, remove_recursive) in remove_watches {
            self.remove_watch(path, remove_recursive).ok();
        }

        for (path, is_user_watch) in add_watches {
            self.add_watch(path, true, is_user_watch).ok();
        }
    }

    fn add_watch(
        &mut self,
        path: WatchPath,
        is_recursive: bool,
        is_user_watch: bool,
    ) -> Result<()> {
        let path_is_dir = metadata(&path.absolute).map_err(Error::io)?.is_dir();
        let requested_is_recursive = is_recursive && path_is_dir;
        if is_user_watch {
            if let Some(watch) = self
                .watches
                .get(&path.absolute)
                .filter(|watch| watch.is_user_watch)
            {
                if watch.user_is_recursive == requested_is_recursive
                    && watch.reported_path == path.requested
                {
                    return Ok(());
                }

                // Rewatching an explicit user watch replaces its requested mode and reported path
                // instead of merging with the previous metadata. If the current entry also carries
                // recursive coverage from an ancestor, remember that ancestor before removal so we
                // can rebuild that inherited coverage below.
                let inherited_recursive_root =
                    if !requested_is_recursive && path_is_dir && watch.is_recursive {
                        recursive_user_watch_ancestor(&path.absolute, self.watches.iter())
                    } else {
                        None
                    };
                let replaced_path = path.absolute.clone();
                self.remove_watch(replaced_path.clone(), false)?;

                if let Some((ancestor_path, ancestor_reported_path)) = inherited_recursive_root {
                    // Removing a directory watch removes its recursively inherited children too.
                    // Re-add them as non-user watches so the ancestor recursive watch still covers
                    // this subtree after the user watch is replaced.
                    for entry in WalkDir::new(&replaced_path)
                        .follow_links(self.follow_symlinks)
                        .into_iter()
                    {
                        let absolute = match entry {
                            Ok(entry) => entry.into_path(),
                            Err(err) if walkdir_error_is_not_found(&err) => continue,
                            Err(err) => return Err(map_walkdir_error(err)),
                        };
                        let requested =
                            reported_path(&ancestor_path, &ancestor_reported_path, &absolute);
                        let result = self.add_single_watch(
                            WatchPath::from_parts(absolute, requested),
                            true,
                            false,
                        );
                        if let Err(err) = result {
                            if !error_is_not_found(&err) {
                                return Err(err);
                            }
                        }
                    }
                }
            }
        }

        // If the watch is not recursive, or if we determine (by stat'ing the path to get its
        // metadata) that the watched path is not a directory, add a single path watch.
        if !requested_is_recursive {
            self.add_single_watch(path, false, is_user_watch)?;
        } else {
            let root = path;
            let mut first = true;
            for entry in WalkDir::new(&root.absolute)
                .follow_links(self.follow_symlinks)
                .into_iter()
            {
                let entry = entry.map_err(map_walkdir_error)?;
                // WalkDir yields the root first; only it is the user-requested watch.
                self.add_single_watch(
                    root.child(entry.into_path()),
                    is_recursive,
                    is_user_watch && first,
                )?;
                first = false;
            }
        }

        // Only make a single `kevent` syscall to add all the watches.
        self.kqueue.watch()?;

        Ok(())
    }

    /// Adds a single watch to the kqueue.
    ///
    /// The caller of this function must call `self.kqueue.watch()` afterwards to register the new watch.
    fn add_single_watch(
        &mut self,
        path: WatchPath,
        is_recursive: bool,
        is_user_watch: bool,
    ) -> Result<()> {
        let event_filter = EventFilter::EVFILT_VNODE;
        let filter_flags = FilterFlag::NOTE_DELETE
            | FilterFlag::NOTE_WRITE
            | FilterFlag::NOTE_EXTEND
            | FilterFlag::NOTE_ATTRIB
            | FilterFlag::NOTE_LINK
            | FilterFlag::NOTE_RENAME
            | FilterFlag::NOTE_REVOKE;

        log::trace!("adding kqueue watch: {}", path.absolute.display());

        self.kqueue
            .add_filename(&path.absolute, event_filter, filter_flags)
            .map_err(|e| Error::io(e).add_path(path.requested.clone()))?;
        let existing_watch = self.watches.get(&path.absolute);
        let watch = Watch::new(
            &path,
            is_recursive,
            is_user_watch,
            existing_watch,
            self.watches.iter(),
        );
        self.watches.insert(path.absolute, watch);

        Ok(())
    }

    fn remove_watch(&mut self, path: PathBuf, remove_recursive: bool) -> Result<()> {
        log::trace!("removing kqueue watch: {}", path.display());

        let preserved_roots = preserved_watch_roots(&path, remove_recursive, self.watches.iter());

        match self.watches.remove(&path) {
            None => return Err(Error::watch_not_found()),
            Some(watch) => {
                if watch.is_recursive || remove_recursive {
                    self.kqueue
                        .remove_filename(&path, EventFilter::EVFILT_VNODE)
                        .map_err(|e| Error::io(e).add_path(path.clone()))?;

                    let mut remove_list = Vec::new();
                    let mut reset_list = Vec::new();
                    for p in self.watches.keys().filter(|p| p.starts_with(&path)) {
                        if let Some(user_is_recursive) = preserved_watch_mode(p, &preserved_roots) {
                            if !user_is_recursive || is_preserved_watch_root(p, &preserved_roots) {
                                reset_list.push(p.clone());
                            }
                            continue;
                        }

                        remove_list.push(p.clone());
                    }

                    for p in &remove_list {
                        self.kqueue
                            .remove_filename(p, EventFilter::EVFILT_VNODE)
                            .map_err(|e| Error::io(e).add_path(p.clone()))?;
                    }
                    for p in remove_list {
                        self.watches.remove(&p);
                    }
                    for p in reset_list {
                        if let Some(watch) = self.watches.get_mut(&p) {
                            watch.is_recursive = watch.user_is_recursive;
                        }
                    }
                } else {
                    self.kqueue
                        .remove_filename(&path, EventFilter::EVFILT_VNODE)
                        .map_err(|e| Error::io(e).add_path(path.clone()))?;
                }

                self.kqueue.watch()?;
            }
        }
        Ok(())
    }
}

fn map_walkdir_error(e: walkdir::Error) -> Error {
    if e.io_error().is_some() {
        // save to unwrap otherwise we whouldn't be in this branch
        Error::io(e.into_io_error().unwrap())
    } else {
        Error::generic(&e.to_string())
    }
}

fn walkdir_error_is_not_found(e: &walkdir::Error) -> bool {
    e.io_error()
        .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound)
}

fn error_is_not_found(e: &Error) -> bool {
    matches!(&e.kind, ErrorKind::PathNotFound)
        || matches!(&e.kind, ErrorKind::Io(io_err) if io_err.kind() == std::io::ErrorKind::NotFound)
}

impl KqueueWatcher {
    fn from_event_handler(
        event_handler: Box<dyn EventHandler>,
        follow_symlinks: bool,
        event_kinds: EventKindMask,
    ) -> Result<Self> {
        let kqueue = kqueue::Watcher::new()?;
        let event_loop = EventLoop::new(kqueue, event_handler, follow_symlinks, event_kinds)?;
        let channel = event_loop.event_loop_tx.clone();
        let waker = event_loop.event_loop_waker.clone();
        event_loop.run();
        Ok(KqueueWatcher { channel, waker })
    }

    fn watch_inner(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        let pb = WatchPath::new(path)?;
        let (tx, rx) = unbounded();
        let msg = EventLoopMsg::AddWatch(pb, recursive_mode, tx);

        self.channel
            .send(msg)
            .map_err(|e| Error::generic(&e.to_string()))?;
        self.waker
            .wake()
            .map_err(|e| Error::generic(&e.to_string()))?;
        rx.recv()
            .unwrap()
            .map_err(|e| Error::generic(&e.to_string()))
    }

    fn unwatch_inner(&mut self, path: &Path) -> Result<()> {
        let pb = absolute_path(path)?;
        let (tx, rx) = unbounded();
        let msg = EventLoopMsg::RemoveWatch(pb, tx);

        self.channel
            .send(msg)
            .map_err(|e| Error::generic(&e.to_string()))?;
        self.waker
            .wake()
            .map_err(|e| Error::generic(&e.to_string()))?;
        rx.recv()
            .unwrap()
            .map_err(|e| Error::generic(&e.to_string()))
    }

    fn watched_paths_inner(&self) -> Result<Vec<(PathBuf, RecursiveMode)>> {
        let (tx, rx) = unbounded();
        self.channel.send(EventLoopMsg::GetWatchedPaths(tx))?;
        self.waker.wake()?;
        rx.recv().map_err(Error::from)
    }
}

impl Watcher for KqueueWatcher {
    /// Create a new watcher.
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self> {
        Self::from_event_handler(
            Box::new(event_handler),
            config.follow_symlinks(),
            config.event_kinds(),
        )
    }

    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        self.watch_inner(path, recursive_mode)
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.unwatch_inner(path)
    }

    fn watched_paths(&self) -> Result<Vec<(PathBuf, RecursiveMode)>> {
        self.watched_paths_inner()
    }

    fn kind() -> crate::WatcherKind {
        crate::WatcherKind::Kqueue
    }
}

impl Drop for KqueueWatcher {
    fn drop(&mut self) {
        // we expect the event loop to live => unwrap must not panic
        self.channel.send(EventLoopMsg::Shutdown).unwrap();
        self.waker.wake().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::test::{self, *};

    fn watcher() -> (TestWatcher<KqueueWatcher>, test::Receiver) {
        channel()
    }

    #[test]
    fn test_remove_recursive() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from("src");

        let mut watcher = KqueueWatcher::new(|event| println!("{event:?}"), Config::default())?;
        watcher.watch(&path, RecursiveMode::Recursive)?;
        let result = watcher.unwatch(&path);
        assert!(
            result.is_ok(),
            "unwatch yielded error: {}",
            result.unwrap_err()
        );
        Ok(())
    }

    #[test]
    fn internal_recursive_refresh_preserves_explicit_child(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let child = dir.path().join("child");
        std::fs::create_dir(&child)?;

        let kqueue = kqueue::Watcher::new()?;
        let mut event_loop = EventLoop::new(kqueue, Box::new(|_| {}), false, EventKindMask::ALL)?;

        event_loop.add_watch(WatchPath::new(dir.path())?, true, true)?;
        event_loop.add_watch(WatchPath::new(&child)?, false, true)?;

        event_loop.remove_watch(dir.path().to_path_buf(), false)?;
        assert!(
            event_loop
                .watches
                .get(&child)
                .is_some_and(|watch| watch.is_user_watch && !watch.user_is_recursive),
            "internal refresh removed explicit child watch"
        );

        event_loop.add_watch(WatchPath::new(dir.path())?, true, true)?;

        let watched: HashMap<_, _> = event_loop
            .watches
            .iter()
            .filter(|(_path, watch)| watch.is_user_watch)
            .map(|(path, watch)| (path.clone(), watch.user_is_recursive))
            .collect();
        assert_eq!(watched.get(dir.path()), Some(&true));
        assert_eq!(watched.get(&child), Some(&false));

        Ok(())
    }

    #[test]
    fn recursive_remove_uses_tracked_watches() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let dir = tempfile::tempdir()?;
        let child = dir.path().join("child");
        std::fs::write(&child, "")?;

        let kqueue = kqueue::Watcher::new()?;
        let mut event_loop = EventLoop::new(kqueue, Box::new(|_| {}), false, EventKindMask::ALL)?;

        event_loop.add_watch(WatchPath::new(dir.path())?, true, true)?;
        assert!(event_loop.watches.contains_key(&child));

        std::fs::remove_file(&child)?;
        event_loop.remove_watch(dir.path().to_path_buf(), false)?;

        assert!(!event_loop.watches.contains_key(&child));

        Ok(())
    }

    #[test]
    fn rewatching_same_path_replaces_recursive_state(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let child = dir.path().join("child");
        std::fs::create_dir(&child)?;

        let kqueue = kqueue::Watcher::new()?;
        let mut event_loop = EventLoop::new(kqueue, Box::new(|_| {}), false, EventKindMask::ALL)?;

        event_loop.add_watch(WatchPath::new(dir.path())?, true, true)?;
        assert!(event_loop.watches.contains_key(&child));

        event_loop.add_watch(WatchPath::new(dir.path())?, false, true)?;

        let watch = event_loop.watches.get(dir.path()).expect("root watch");
        assert!(watch.is_user_watch);
        assert!(!watch.user_is_recursive);
        assert!(!watch.is_recursive);
        assert!(!event_loop.watches.contains_key(&child));

        Ok(())
    }

    #[test]
    fn rewatching_child_preserves_recursive_parent_state(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let child = dir.path().join("child");
        let grandchild = child.join("grandchild");
        std::fs::create_dir_all(&grandchild)?;

        let kqueue = kqueue::Watcher::new()?;
        let mut event_loop = EventLoop::new(kqueue, Box::new(|_| {}), false, EventKindMask::ALL)?;

        event_loop.add_watch(WatchPath::new(dir.path())?, true, true)?;
        event_loop.add_watch(WatchPath::new(&child)?, false, true)?;
        event_loop.add_watch(
            WatchPath::from_parts(child.clone(), PathBuf::from("reported-child")),
            false,
            true,
        )?;

        let child_watch = event_loop.watches.get(&child).expect("child watch");
        assert!(child_watch.is_user_watch);
        assert!(!child_watch.user_is_recursive);
        assert!(child_watch.is_recursive);
        assert_eq!(child_watch.reported_path, PathBuf::from("reported-child"));

        let grandchild_watch = event_loop
            .watches
            .get(&grandchild)
            .expect("grandchild still covered by recursive parent");
        assert!(!grandchild_watch.is_user_watch);
        assert!(grandchild_watch.is_recursive);

        Ok(())
    }

    #[test]
    fn rewatching_carved_out_child_does_not_restore_parent_recursive_state(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let child = dir.path().join("child");
        let grandchild = child.join("grandchild");
        std::fs::create_dir_all(&grandchild)?;

        let kqueue = kqueue::Watcher::new()?;
        let mut event_loop = EventLoop::new(kqueue, Box::new(|_| {}), false, EventKindMask::ALL)?;

        event_loop.add_watch(WatchPath::new(dir.path())?, true, true)?;
        event_loop.remove_watch(child.clone(), false)?;
        event_loop.add_watch(WatchPath::new(&child)?, false, true)?;
        event_loop.add_watch(
            WatchPath::from_parts(child.clone(), PathBuf::from("reported-child")),
            false,
            true,
        )?;

        let child_watch = event_loop.watches.get(&child).expect("child watch");
        assert!(child_watch.is_user_watch);
        assert!(!child_watch.user_is_recursive);
        assert!(!child_watch.is_recursive);
        assert_eq!(child_watch.reported_path, PathBuf::from("reported-child"));
        assert!(!event_loop.watches.contains_key(&grandchild));

        Ok(())
    }

    #[test]
    fn create_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        rx.wait_unordered([expected(path).create_file()]);
    }

    #[test]
    fn write_file() {
        let tmpdir = testdir();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        let (mut watcher, mut rx) = watcher();

        watcher.watch_recursively(&tmpdir);

        std::fs::write(&path, b"123").expect("write");

        rx.wait_unordered([expected(&path).modify_data_any()]);
    }

    #[test]
    fn chmod_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        let file = std::fs::File::create_new(&path).expect("create");
        let mut permissions = file.metadata().expect("metadata").permissions();
        permissions.set_readonly(true);

        watcher.watch_recursively(&tmpdir);
        file.set_permissions(permissions).expect("set_permissions");

        rx.wait_unordered([expected(&path).modify_meta_any()]);
    }

    #[test]
    fn rename_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_recursively(&tmpdir);
        let new_path = tmpdir.path().join("renamed");

        std::fs::rename(&path, &new_path).expect("rename");

        rx.wait_unordered([expected(path).rename_any(), expected(new_path).create()]);
    }

    #[test]
    fn delete_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::remove_file(&file).expect("remove");

        // kqueue reports a write event on the directory when a file is deleted
        rx.wait_unordered([expected(tmpdir.path()).modify_data_any()]);
    }

    #[test]
    fn create_file_in_non_recursive_directory_with_existing_child() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let existing = tmpdir.path().join("existing");
        let created = tmpdir.path().join("created");
        std::fs::write(&existing, "").expect("write");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::write(&created, "").expect("write");

        // kqueue does not report which directory entry changed, so the backend
        // must not guess an arbitrary pre-existing child as the created path.
        rx.wait_unordered([expected(tmpdir.path()).modify_data_any()]);
    }

    #[test]
    fn delete_self_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&file);

        std::fs::remove_file(&file).expect("remove");

        rx.wait_unordered([expected(file).remove_any()]);
    }

    #[test]
    fn delete_self_dir() {
        let tmpdir = testdir();
        let dir = tmpdir.path().join("dir");
        std::fs::create_dir(&dir).expect("create");

        let (mut watcher, mut rx) = watcher();
        watcher.watch_nonrecursively(&dir);

        std::fs::remove_dir(&dir).expect("remove");

        rx.wait_unordered([expected(&dir).remove_any()]);
    }

    #[test]
    #[ignore = "FIXME"]
    fn create_write_overwrite() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let overwritten_file = tmpdir.path().join("overwritten_file");
        let overwriting_file = tmpdir.path().join("overwriting_file");
        std::fs::write(&overwritten_file, "123").expect("write1");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::File::create(&overwriting_file).expect("create");
        std::fs::write(&overwriting_file, "321").expect("write2");
        std::fs::rename(&overwriting_file, &overwritten_file).expect("rename");

        rx.wait_unordered([
            expected(&overwriting_file).create_file(),
            expected(&overwriting_file).modify_data_any().multiple(),
            expected(&overwriting_file).rename_any(),
            expected(&overwritten_file).rename_any(),
        ])
        .ensure_no_tail();
    }

    #[test]
    fn create_dir() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::create_dir(&path).expect("create");

        rx.wait_unordered([expected(&path).create_folder()]);
    }

    #[test]
    fn chmod_dir() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::create_dir(&path).expect("create_dir");
        let mut permissions = std::fs::metadata(&path).expect("metadata").permissions();
        permissions.set_readonly(true);

        watcher.watch_recursively(&tmpdir);
        std::fs::set_permissions(&path, permissions).expect("set_permissions");

        rx.wait_unordered([expected(&path).modify_meta_any()]);
    }

    #[test]
    fn rename_dir() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        let new_path = tmpdir.path().join("new_path");
        std::fs::create_dir(&path).expect("create_dir");

        watcher.watch_recursively(&tmpdir);
        std::fs::rename(&path, &new_path).expect("rename");

        rx.wait_ordered([
            expected(&new_path).create_folder(),
            expected(&path).rename_any(),
        ]);
    }

    #[test]
    fn delete_dir() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::create_dir(&path).expect("create_dir");

        watcher.watch_recursively(&tmpdir);
        std::fs::remove_dir(&path).expect("remove");

        rx.wait_unordered([expected(path).remove_any()]);
    }

    #[test]
    #[ignore = "FIXME"]
    fn rename_dir_twice() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        let new_path = tmpdir.path().join("new_path");
        let new_path2 = tmpdir.path().join("new_path2");
        std::fs::create_dir(&path).expect("create_dir");

        watcher.watch_recursively(&tmpdir);
        std::fs::rename(&path, &new_path).expect("rename");
        std::fs::rename(&new_path, &new_path2).expect("rename2");

        rx.wait_unordered([
            expected(&path).rename_any(),
            expected(&new_path).create_folder(),
            expected(&new_path).rename_any(),
            expected(&new_path2).create_folder(),
        ]);
    }

    #[test]
    fn move_out_of_watched_dir() {
        let tmpdir = testdir();
        let subdir = tmpdir.path().join("subdir");
        let (mut watcher, mut rx) = watcher();

        let path = subdir.join("entry");
        std::fs::create_dir_all(&subdir).expect("create_dir_all");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_recursively(&subdir);
        let new_path = tmpdir.path().join("entry");

        std::fs::rename(&path, &new_path).expect("rename");

        rx.wait_unordered([expected(path).rename_any()]);
    }

    #[test]
    #[ignore = "FIXME"]
    fn create_write_write_rename_write_remove() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let file1 = tmpdir.path().join("entry");
        let file2 = tmpdir.path().join("entry2");
        std::fs::File::create_new(&file2).expect("create file2");
        let new_path = tmpdir.path().join("renamed");

        watcher.watch_recursively(&tmpdir);
        std::fs::write(&file1, "123").expect("write 1");
        std::fs::write(&file2, "321").expect("write 2");
        std::fs::rename(&file1, &new_path).expect("rename");
        std::fs::write(&new_path, b"1").expect("write 3");
        std::fs::remove_file(&new_path).expect("remove");

        rx.wait_ordered([
            expected(&file1).create_file(),
            expected(&file1).modify_data_content(),
            expected(&file2).modify_data_content(),
            expected(&file1).rename_any(),
            expected(&new_path).rename_any(),
            expected(&new_path).modify_data_content(),
            expected(&new_path).remove_file(),
        ]);
    }

    #[test]
    #[ignore = "FIXME"]
    fn rename_twice() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_recursively(&tmpdir);
        let new_path1 = tmpdir.path().join("renamed1");
        let new_path2 = tmpdir.path().join("renamed2");

        std::fs::rename(&path, &new_path1).expect("rename1");
        std::fs::rename(&new_path1, &new_path2).expect("rename2");

        rx.wait_unordered([
            expected(&path).rename_any(),
            expected(&new_path1).rename_any(),
            expected(&new_path2).create(),
        ]);
    }

    #[test]
    fn set_file_mtime() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        let file = std::fs::File::create_new(&path).expect("create");

        watcher.watch_recursively(&tmpdir);

        file.set_modified(
            std::time::SystemTime::now()
                .checked_sub(Duration::from_secs(60 * 60))
                .expect("time"),
        )
        .expect("set_time");

        rx.wait_unordered([expected(&path).modify_meta_any()]);
    }

    #[test]
    fn write_file_non_recursive_watch() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_nonrecursively(&path);

        std::fs::write(&path, b"123").expect("write");

        rx.wait_unordered([expected(path).modify_data_any()]);
    }

    #[test]
    fn write_to_a_hardlink_pointed_to_the_watched_file_triggers_an_event() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let subdir = tmpdir.path().join("subdir");
        let file = subdir.join("file");
        let hardlink = tmpdir.path().join("hardlink");

        std::fs::create_dir(&subdir).expect("create");
        std::fs::write(&file, "").expect("file");
        std::fs::hard_link(&file, &hardlink).expect("hardlink");

        watcher.watch_nonrecursively(&file);

        std::fs::write(&hardlink, "123123").expect("write to the hard link");

        rx.wait_unordered([expected(file).modify_data_any()]);
    }

    #[test]
    #[ignore = "FIXME"]
    fn recursive_creation() {
        let tmpdir = testdir();
        let nested1 = tmpdir.path().join("1");
        let nested2 = tmpdir.path().join("1/2");
        let nested3 = tmpdir.path().join("1/2/3");
        let nested4 = tmpdir.path().join("1/2/3/4");
        let nested5 = tmpdir.path().join("1/2/3/4/5");
        let nested6 = tmpdir.path().join("1/2/3/4/5/6");
        let nested7 = tmpdir.path().join("1/2/3/4/5/6/7");
        let nested8 = tmpdir.path().join("1/2/3/4/5/6/7/8");
        let nested9 = tmpdir.path().join("1/2/3/4/5/6/7/8/9");

        let (mut watcher, mut rx) = watcher();

        watcher.watch_recursively(&tmpdir);

        std::fs::create_dir_all(&nested9).expect("create_dir_all");

        rx.wait_unordered([
            expected(&nested1).create_folder(),
            expected(&nested2).create_folder(),
            expected(&nested3).create_folder(),
            expected(&nested4).create_folder(),
            expected(&nested5).create_folder(),
            expected(&nested6).create_folder(),
            expected(&nested7).create_folder(),
            expected(&nested8).create_folder(),
            expected(&nested9).create_folder(),
        ]);
    }

    #[test]
    fn kqueue_watcher_respects_event_kind_mask() {
        use crate::Watcher;
        use notify_types::event::EventKindMask;

        let tmpdir = testdir();
        let (tx, rx) = std::sync::mpsc::channel();

        // Create watcher with CREATE-only mask (no MODIFY events)
        let config = Config::default().with_event_kinds(EventKindMask::CREATE);

        let mut watcher = KqueueWatcher::new(tx, config).expect("create watcher");
        watcher
            .watch(tmpdir.path(), crate::RecursiveMode::Recursive)
            .expect("watch");

        let path = tmpdir.path().join("test_file");

        // Create a file - should generate CREATE event
        std::fs::File::create_new(&path).expect("create");

        // Small delay to let events propagate
        std::thread::sleep(Duration::from_millis(100));

        // Modify the file - should NOT generate event (filtered by mask)
        std::fs::write(&path, "modified content").expect("write modified");

        std::thread::sleep(Duration::from_millis(100));

        // Collect all events
        let events: Vec<_> = rx.try_iter().filter_map(|r| r.ok()).collect();

        // Should have CREATE event
        assert!(
            events.iter().any(|e| e.kind.is_create()),
            "Expected CREATE event, got: {events:?}"
        );

        // Should NOT have MODIFY event (filtered out)
        assert!(
            !events.iter().any(|e| e.kind.is_modify()),
            "Should not receive MODIFY events with CREATE-only mask, got: {events:?}"
        );
    }
}
