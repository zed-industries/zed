//! Watcher implementation for the inotify Linux API
//!
//! The inotify API provides a mechanism for monitoring filesystem events.  Inotify can be used to
//! monitor individual files, or to monitor directories.  When a directory is monitored, inotify
//! will return events for the directory itself, and for files inside the directory.

use super::event::*;
use super::{Config, Error, ErrorKind, EventHandler, RecursiveMode, Result, Watcher};
use crate::paths::{
    absolute_path, is_preserved_watch_root, preserved_watch_mode, preserved_watch_roots,
    recursive_user_watch_ancestor, reported_path, WatchMetadata, WatchPath,
};
use crate::{bounded, unbounded, BoundSender, Receiver, Sender};
use inotify as inotify_sys;
use inotify_sys::{EventMask, Inotify, WatchDescriptor, WatchMask};
use notify_types::event::EventKindMask;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::metadata;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use walkdir::WalkDir;

const INOTIFY: mio::Token = mio::Token(0);
const MESSAGE: mio::Token = mio::Token(1);

/// Convert an EventKindMask to the corresponding inotify WatchMask.
///
/// When `is_recursive` is true, CREATE and MOVED_TO are always included
/// to enable tracking of newly created subdirectories.
fn event_kind_mask_to_watch_mask(mask: EventKindMask, is_recursive: bool) -> WatchMask {
    let mut watch_mask = WatchMask::empty();

    if is_recursive {
        watch_mask |= WatchMask::CREATE | WatchMask::MOVED_TO;
    }

    if mask.intersects(EventKindMask::CREATE) {
        watch_mask |= WatchMask::CREATE | WatchMask::MOVED_TO;
    }

    if mask.intersects(EventKindMask::REMOVE) {
        watch_mask |= WatchMask::DELETE | WatchMask::MOVED_FROM;
    }

    if mask.intersects(EventKindMask::MODIFY_DATA) {
        // Note: CLOSE_WRITE is intentionally NOT included here because it generates
        // Access(Close(Write)) events, not Modify events. Users who want CLOSE_WRITE
        // events should use ACCESS_CLOSE.
        watch_mask |= WatchMask::MODIFY;
    }

    if mask.intersects(EventKindMask::MODIFY_META) {
        watch_mask |= WatchMask::ATTRIB;
    }

    if mask.intersects(EventKindMask::MODIFY_NAME) {
        watch_mask |= WatchMask::MOVE_SELF;
    }

    if mask.intersects(EventKindMask::ACCESS_OPEN) {
        watch_mask |= WatchMask::OPEN;
    }

    if mask.intersects(EventKindMask::ACCESS_CLOSE) {
        watch_mask |= WatchMask::CLOSE_WRITE;
    }

    if mask.intersects(EventKindMask::ACCESS_CLOSE_NOWRITE) {
        watch_mask |= WatchMask::CLOSE_NOWRITE;
    }

    watch_mask
}

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
    inotify: Option<Inotify>,
    event_handler: Box<dyn EventHandler>,
    /// Absolute path -> inotify descriptor and watch metadata.
    watches: HashMap<PathBuf, Watch>,
    paths: HashMap<WatchDescriptor, PathBuf>,
    rename_event: Option<Event>,
    follow_links: bool,
    event_kind_mask: EventKindMask,
}

struct Watch {
    watch_descriptor: WatchDescriptor,
    watch_mask: WatchMask,
    is_dir: bool,
    metadata: WatchMetadata,
}

/// Watcher implementation based on inotify
#[derive(Debug)]
pub struct INotifyWatcher {
    channel: Sender<EventLoopMsg>,
    waker: Arc<mio::Waker>,
}

enum EventLoopMsg {
    AddWatch(WatchPath, RecursiveMode, Sender<Result<()>>),
    RemoveWatch(PathBuf, Sender<Result<()>>),
    GetWatchedPaths(Sender<Vec<(PathBuf, RecursiveMode)>>),
    Shutdown,
    Configure(Config, BoundSender<Result<bool>>),
}

#[inline]
fn add_watch_by_event(
    path: &PathBuf,
    event: &inotify_sys::Event<&OsStr>,
    watches: &HashMap<PathBuf, Watch>,
    add_watches: &mut Vec<WatchPath>,
) {
    if event.mask.contains(EventMask::ISDIR) {
        if let Some(parent_path) = path.parent() {
            if let Some(watch) = watches.get(parent_path) {
                if watch.metadata.is_recursive {
                    add_watches.push(WatchPath::from_parts(
                        path.to_owned(),
                        reported_path(parent_path, &watch.metadata.reported_path, path),
                    ));
                }
            }
        }
    }
}

#[inline]
fn remove_watch_by_event(
    path: &PathBuf,
    watches: &HashMap<PathBuf, Watch>,
    remove_watches: &mut Vec<PathBuf>,
) {
    if watches.contains_key(path) {
        remove_watches.push(path.to_owned());
    }
}

#[inline]
fn unmount_event(path: PathBuf) -> Event {
    Event::new(EventKind::Remove(RemoveKind::Other))
        .add_path(path)
        .set_info("unmount")
}

impl EventLoop {
    pub fn new(
        inotify: Inotify,
        event_handler: Box<dyn EventHandler>,
        config: &Config,
    ) -> Result<Self> {
        let (event_loop_tx, event_loop_rx) = unbounded::<EventLoopMsg>();
        let poll = mio::Poll::new()?;

        let event_loop_waker = Arc::new(mio::Waker::new(poll.registry(), MESSAGE)?);

        let inotify_fd = inotify.as_raw_fd();
        let mut evented_inotify = mio::unix::SourceFd(&inotify_fd);
        poll.registry()
            .register(&mut evented_inotify, INOTIFY, mio::Interest::READABLE)?;

        let event_loop = EventLoop {
            running: true,
            poll,
            event_loop_waker,
            event_loop_tx,
            event_loop_rx,
            inotify: Some(inotify),
            event_handler,
            watches: HashMap::new(),
            paths: HashMap::new(),
            rename_event: None,
            follow_links: config.follow_symlinks(),
            event_kind_mask: config.event_kinds(),
        };
        Ok(event_loop)
    }

    // Run the event loop.
    pub fn run(self) {
        let _ = thread::Builder::new()
            .name("notify-rs inotify loop".to_string())
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
            INOTIFY => {
                // inotify has something to tell us.
                self.handle_inotify()
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
                            .filter(|(_path, watch)| watch.metadata.is_user_watch)
                            .map(|(_path, watch)| {
                                (
                                    watch.metadata.reported_path.clone(),
                                    if watch.metadata.user_is_recursive {
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
                    let _ = self.remove_all_watches();
                    if let Some(inotify) = self.inotify.take() {
                        let _ = inotify.close();
                    }
                    self.running = false;
                    break;
                }
                EventLoopMsg::Configure(config, tx) => {
                    self.configure_raw_mode(config, tx);
                }
            }
        }
    }

    fn configure_raw_mode(&mut self, _config: Config, tx: BoundSender<Result<bool>>) {
        tx.send(Ok(false))
            .expect("configuration channel disconnected");
    }

    fn handle_inotify(&mut self) {
        let mut add_watches = Vec::new();
        let mut remove_watches = Vec::new();
        let mut remove_watches_no_syscall = Vec::new();

        if let Some(ref mut inotify) = self.inotify {
            let mut buffer = [0; 1024];
            // Read all buffers available.
            loop {
                match inotify.read_events(&mut buffer) {
                    Ok(events) => {
                        let mut num_events = 0;
                        for event in events {
                            log::trace!("inotify event: {event:?}");

                            num_events += 1;
                            if event.mask.contains(EventMask::Q_OVERFLOW) {
                                let ev = Ok(Event::new(EventKind::Other).set_flag(Flag::Rescan));
                                self.event_handler.handle_event(ev);
                            }

                            let paths = self.paths.get(&event.wd).and_then(|root| {
                                self.watches.get(root).map(|watch| match event.name {
                                    Some(name) => {
                                        let path = root.join(name);
                                        let reported_path = reported_path(
                                            root,
                                            &watch.metadata.reported_path,
                                            &path,
                                        );
                                        (path, reported_path)
                                    }
                                    None => (root.clone(), watch.metadata.reported_path.clone()),
                                })
                            });

                            let (path, event_path) = match paths {
                                Some(paths) => paths,
                                None => {
                                    log::debug!("inotify event with unknown descriptor: {event:?}");
                                    continue;
                                }
                            };

                            let mut evs = Vec::new();

                            if event.mask.contains(EventMask::MOVED_FROM) {
                                remove_watch_by_event(&path, &self.watches, &mut remove_watches);

                                let event = Event::new(EventKind::Modify(ModifyKind::Name(
                                    RenameMode::From,
                                )))
                                .add_path(event_path.clone())
                                .set_tracker(event.cookie as usize);

                                self.rename_event = Some(event.clone());

                                evs.push(event);
                            } else if event.mask.contains(EventMask::MOVED_TO) {
                                evs.push(
                                    Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::To)))
                                        .set_tracker(event.cookie as usize)
                                        .add_path(event_path.clone()),
                                );

                                let trackers_match =
                                    self.rename_event.as_ref().and_then(|e| e.tracker())
                                        == Some(event.cookie as usize);

                                if trackers_match {
                                    let rename_event = self.rename_event.take().unwrap(); // unwrap is safe because `rename_event` must be set at this point
                                    evs.push(
                                        Event::new(EventKind::Modify(ModifyKind::Name(
                                            RenameMode::Both,
                                        )))
                                        .set_tracker(event.cookie as usize)
                                        .add_some_path(rename_event.paths.first().cloned())
                                        .add_path(event_path.clone()),
                                    );
                                }
                                add_watch_by_event(&path, &event, &self.watches, &mut add_watches);
                            }
                            if event.mask.contains(EventMask::MOVE_SELF) {
                                evs.push(
                                    Event::new(EventKind::Modify(ModifyKind::Name(
                                        RenameMode::From,
                                    )))
                                    .add_path(event_path.clone()),
                                );
                                // TODO stat the path and get to new path
                                // - emit To and Both events
                                // - change prefix for further events
                            }
                            if event.mask.contains(EventMask::CREATE) {
                                evs.push(
                                    Event::new(EventKind::Create(
                                        if event.mask.contains(EventMask::ISDIR) {
                                            CreateKind::Folder
                                        } else {
                                            CreateKind::File
                                        },
                                    ))
                                    .add_path(event_path.clone()),
                                );
                                add_watch_by_event(&path, &event, &self.watches, &mut add_watches);
                            }
                            if event.mask.contains(EventMask::DELETE) {
                                evs.push(
                                    Event::new(EventKind::Remove(
                                        if event.mask.contains(EventMask::ISDIR) {
                                            RemoveKind::Folder
                                        } else {
                                            RemoveKind::File
                                        },
                                    ))
                                    .add_path(event_path.clone()),
                                );
                                remove_watch_by_event(&path, &self.watches, &mut remove_watches);
                            }
                            if event.mask.contains(EventMask::DELETE_SELF) {
                                let remove_kind = match self.watches.get(&path) {
                                    Some(watch) if watch.is_dir => RemoveKind::Folder,
                                    Some(_) => RemoveKind::File,
                                    None => RemoveKind::Other,
                                };
                                evs.push(
                                    Event::new(EventKind::Remove(remove_kind))
                                        .add_path(event_path.clone()),
                                );
                                remove_watch_by_event(&path, &self.watches, &mut remove_watches);
                            }
                            if event.mask.contains(EventMask::UNMOUNT) {
                                evs.push(unmount_event(event_path.clone()));
                                // The kernel has already removed this watch descriptor and will
                                // emit IGNORED; clean up internal state without inotify_rm_watch.
                                // ref. https://www.man7.org/linux/man-pages/man7/inotify.7.html
                                remove_watch_by_event(
                                    &path,
                                    &self.watches,
                                    &mut remove_watches_no_syscall,
                                );
                            }
                            if event.mask.contains(EventMask::MODIFY) {
                                evs.push(
                                    Event::new(EventKind::Modify(ModifyKind::Data(
                                        DataChange::Any,
                                    )))
                                    .add_path(event_path.clone()),
                                );
                            }
                            if event.mask.contains(EventMask::CLOSE_WRITE) {
                                evs.push(
                                    Event::new(EventKind::Access(AccessKind::Close(
                                        AccessMode::Write,
                                    )))
                                    .add_path(event_path.clone()),
                                );
                            }
                            if event.mask.contains(EventMask::CLOSE_NOWRITE) {
                                evs.push(
                                    Event::new(EventKind::Access(AccessKind::Close(
                                        AccessMode::Read,
                                    )))
                                    .add_path(event_path.clone()),
                                );
                            }
                            if event.mask.contains(EventMask::ATTRIB) {
                                evs.push(
                                    Event::new(EventKind::Modify(ModifyKind::Metadata(
                                        MetadataKind::Any,
                                    )))
                                    .add_path(event_path.clone()),
                                );
                            }
                            if event.mask.contains(EventMask::OPEN) {
                                evs.push(
                                    Event::new(EventKind::Access(AccessKind::Open(
                                        AccessMode::Any,
                                    )))
                                    .add_path(event_path.clone()),
                                );
                            }

                            // Filter events based on EventKindMask before delivery
                            for ev in evs {
                                if self.event_kind_mask.matches(&ev.kind) {
                                    self.event_handler.handle_event(Ok(ev));
                                }
                            }
                        }

                        // All events read. Break out.
                        if num_events == 0 {
                            break;
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No events read. Break out.
                        break;
                    }
                    Err(e) => {
                        self.event_handler.handle_event(Err(Error::io(e)));
                    }
                }
            }
        }

        for path in remove_watches_no_syscall {
            if let Err(err) = self.remove_watch_without_os_call(path, true) {
                log::warn!("Unable to remove the path from the watches: {err:?}");
            }
        }

        for path in remove_watches {
            if let Err(err) = self.remove_watch(path, true) {
                log::warn!("Unable to remove the path from the watches: {err:?}");
            }
        }

        for path in add_watches {
            if let Err(add_watch_error) = self.add_watch(path, true, false) {
                // The handler should be notified if we have reached the limit.
                // Otherwise, the user might expect that a recursive watch
                // is continuing to work correctly, but it's not.
                if let ErrorKind::MaxFilesWatch = add_watch_error.kind {
                    self.event_handler.handle_event(Err(add_watch_error));

                    // After that kind of a error we should stop adding watches,
                    // because the limit has already reached and all next calls
                    // will return us only the same error.
                    break;
                }
            }
        }
    }

    fn add_watch(&mut self, path: WatchPath, is_recursive: bool, watch_self: bool) -> Result<()> {
        let path_is_dir = metadata(&path.absolute).map_err(Error::io_watch)?.is_dir();
        let requested_is_recursive = is_recursive && path_is_dir;
        if watch_self {
            if let Some(watch) = self
                .watches
                .get(&path.absolute)
                .filter(|watch| watch.metadata.is_user_watch)
            {
                if watch.metadata.user_is_recursive == requested_is_recursive
                    && watch.metadata.reported_path == path.requested
                {
                    return Ok(());
                }

                // Rewatching an explicit user watch replaces its requested mode and reported path
                // instead of merging with the previous metadata. If the current entry also carries
                // recursive coverage from an ancestor, remember that ancestor before removal so we
                // can rebuild that inherited coverage below.
                let inherited_recursive_root =
                    if !requested_is_recursive && path_is_dir && watch.metadata.is_recursive {
                        recursive_user_watch_ancestor(
                            &path.absolute,
                            self.watches
                                .iter()
                                .map(|(path, watch)| (path, &watch.metadata)),
                        )
                    } else {
                        None
                    };
                let replaced_path = path.absolute.clone();
                self.remove_watch(replaced_path.clone(), false)?;

                if let Some((ancestor_path, ancestor_reported_path)) = inherited_recursive_root {
                    // Removing a directory watch removes its recursively inherited children too.
                    // Re-add them as non-user watches so the ancestor recursive watch still covers
                    // this subtree after the user watch is replaced.
                    let entries = WalkDir::new(&replaced_path)
                        .follow_links(self.follow_links)
                        .into_iter()
                        .filter_map(filter_dir)
                        .map(|entry| {
                            let absolute = entry.into_path();
                            let requested =
                                reported_path(&ancestor_path, &ancestor_reported_path, &absolute);
                            WatchPath::from_parts(absolute, requested)
                        });
                    self.add_watches_for_paths(entries, true, false)?;
                }
            }
        }

        // If the watch is not recursive, or if we determine (by stat'ing the path to get its
        // metadata) that the watched path is not a directory, add a single path watch.
        if !requested_is_recursive {
            return self.add_single_watch(path, false, true);
        }

        let root = path.clone();
        let entries = WalkDir::new(&root.absolute)
            .follow_links(self.follow_links)
            .into_iter()
            .filter_map(filter_dir)
            .map(move |entry| root.child(entry.into_path()));

        self.add_watches_for_paths(entries, is_recursive, watch_self)
    }

    fn add_watches_for_paths<I>(
        &mut self,
        paths: I,
        is_recursive: bool,
        mut watch_self: bool,
    ) -> Result<()>
    where
        I: IntoIterator<Item = WatchPath>,
    {
        for path in paths {
            match self.add_single_watch(path, is_recursive, watch_self) {
                Ok(()) => {}
                // TOCTOU: a subdirectory can disappear between walkdir listing it and us adding an
                // inotify watch for it. This should not fail the overall recursive watch call.
                Err(err) if !watch_self && matches!(err.kind, ErrorKind::PathNotFound) => {}
                Err(err) => return Err(err),
            }
            watch_self = false;
        }

        Ok(())
    }

    fn add_single_watch(
        &mut self,
        path: WatchPath,
        is_recursive: bool,
        watch_self: bool,
    ) -> Result<()> {
        // Build watch mask from configured event kinds for kernel-level filtering
        let mut watchmask = event_kind_mask_to_watch_mask(self.event_kind_mask, is_recursive);

        if watch_self {
            watchmask.insert(WatchMask::DELETE_SELF);
            watchmask.insert(WatchMask::MOVE_SELF);
        }

        let existing_watch = self.watches.get(&path.absolute);
        if let Some(watch) = existing_watch {
            watchmask.insert(watch.watch_mask);
            watchmask.insert(WatchMask::MASK_ADD);
        }

        if let Some(ref mut inotify) = self.inotify {
            log::trace!("adding inotify watch: {}", path.absolute.display());

            match inotify.watches().add(&path.absolute, watchmask) {
                Err(e) => {
                    Err(if e.raw_os_error() == Some(libc::ENOSPC) {
                        // do not report inotify limits as "no more space" on linux #266
                        Error::new(ErrorKind::MaxFilesWatch)
                    } else if e.kind() == std::io::ErrorKind::NotFound {
                        Error::new(ErrorKind::PathNotFound)
                    } else {
                        Error::io(e)
                    }
                    .add_path(path.requested))
                }
                Ok(w) => {
                    watchmask.remove(WatchMask::MASK_ADD);
                    let is_dir = match metadata(&path.absolute) {
                        Ok(metadata) => metadata.is_dir(),
                        Err(e) => {
                            // Avoid leaking an inotify watch if we can't stat after adding it.
                            // This can happen due to racy deletions.
                            let _ = inotify.watches().remove(w.clone());
                            return Err(Error::io_watch(e).add_path(path.requested));
                        }
                    };
                    let metadata = if let Some(existing_watch) = existing_watch {
                        WatchMetadata::new(
                            &path,
                            is_recursive,
                            watch_self,
                            Some(&existing_watch.metadata),
                            self.watches
                                .iter()
                                .map(|(path, watch)| (path, &watch.metadata)),
                        )
                    } else {
                        WatchMetadata {
                            is_recursive,
                            reported_path: path.requested.clone(),
                            is_user_watch: watch_self,
                            user_is_recursive: watch_self && is_recursive,
                        }
                    };

                    self.watches.insert(
                        path.absolute.clone(),
                        Watch {
                            watch_descriptor: w.clone(),
                            watch_mask: watchmask,
                            is_dir,
                            metadata,
                        },
                    );
                    self.paths.insert(w, path.absolute);
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    fn remove_watch(&mut self, path: PathBuf, remove_recursive: bool) -> Result<()> {
        let preserved_roots = preserved_watch_roots(
            &path,
            remove_recursive,
            self.watches
                .iter()
                .map(|(path, watch)| (path, &watch.metadata)),
        );

        match self.watches.remove(&path) {
            None => return Err(Error::watch_not_found().add_path(path)),
            Some(watch) => {
                if let Some(ref mut inotify) = self.inotify {
                    let mut inotify_watches = inotify.watches();
                    log::trace!("removing inotify watch for {path:?}, remove_recursive: {remove_recursive:?}");

                    Self::remove_single_descriptor(
                        &mut inotify_watches,
                        watch.watch_descriptor.clone(),
                    );
                    self.paths.remove(&watch.watch_descriptor);

                    if watch.metadata.is_recursive || remove_recursive {
                        let mut remove_list = Vec::new();
                        let mut reset_list = Vec::new();
                        for (w, p) in &self.paths {
                            if p.starts_with(&path) {
                                if let Some(user_is_recursive) =
                                    preserved_watch_mode(p, &preserved_roots)
                                {
                                    if !user_is_recursive
                                        || is_preserved_watch_root(p, &preserved_roots)
                                    {
                                        reset_list.push(p.clone());
                                    }
                                    continue;
                                }

                                Self::remove_single_descriptor(&mut inotify_watches, w.clone());
                                self.watches.remove(p);
                                remove_list.push(w.clone());
                            }
                        }
                        for w in remove_list {
                            self.paths.remove(&w);
                        }
                        for p in reset_list {
                            if let Some(watch) = self.watches.get_mut(&p) {
                                watch.metadata.is_recursive = watch.metadata.user_is_recursive;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn remove_watch_without_os_call(
        &mut self,
        path: PathBuf,
        remove_recursive: bool,
    ) -> Result<()> {
        let preserved_roots = preserved_watch_roots(
            &path,
            remove_recursive,
            self.watches
                .iter()
                .map(|(path, watch)| (path, &watch.metadata)),
        );

        match self.watches.remove(&path) {
            None => return Err(Error::watch_not_found().add_path(path)),
            Some(watch) => {
                self.paths.remove(&watch.watch_descriptor);

                if watch.metadata.is_recursive || remove_recursive {
                    let mut remove_list = Vec::new();
                    let mut reset_list = Vec::new();
                    for (w, p) in &self.paths {
                        if p.starts_with(&path) {
                            if let Some(user_is_recursive) =
                                preserved_watch_mode(p, &preserved_roots)
                            {
                                if !user_is_recursive
                                    || is_preserved_watch_root(p, &preserved_roots)
                                {
                                    reset_list.push(p.clone());
                                }
                                continue;
                            }

                            self.watches.remove(p);
                            remove_list.push(w.clone());
                        }
                    }
                    for w in remove_list {
                        self.paths.remove(&w);
                    }
                    for p in reset_list {
                        if let Some(watch) = self.watches.get_mut(&p) {
                            watch.metadata.is_recursive = watch.metadata.user_is_recursive;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// As long as we use the `inotify` crate its behaviour is specified by the documentation of
    /// a [`inotify::Watches::remove`] method:
    /// ```text
    /// Directly returns the error from the call to [inotify_rm_watch].
    /// Returns an [io::Error] with [ErrorKind]::InvalidInput,
    /// if the given WatchDescriptor did not originate from this [Inotify] instance.
    /// ```
    ///
    /// inotify documentation says, that `inotify_rm_watch` may fail with two specific errors:
    /// * EBADF - fd is not a valid file descriptor.
    /// * EINVAL - The watch descriptor wd is not valid or fd is not an inotify file descriptor.
    ///
    /// Therefore, we can ignore this errors (and log it), because
    /// * in the case, when we are removing a watch because of an caught `DELETE` or `DELETE_SELF` event we want the
    ///   path to be not watched, and in error cases it's already done (unknown file descriptor == it is not watched)
    /// * in the case, when user is trying to remove the watch, they can do nothing with that kind of an error,
    ///   it's totally internal. BUT, if there are no "strange" states (like races between user call and internal call,
    ///   when internal inotify file descriptor has already been invalidated, but the event still hasn't been handled)
    ///   they will get an [`ErrorKind::WatchNotFound`] error and can deal with it
    ///
    /// Log level is info, because it is not a "real" error. Expectedly, it may occurred only by race condition
    /// (like described above), in other cases it is a bug (but we aren't able to distinguish that states)
    fn remove_single_descriptor(watches: &mut inotify::Watches, wd: WatchDescriptor) {
        if let Err(err) = watches.remove(wd) {
            log::info!("unable to remove watch descriptor from inotify: {err:?}");
        }
    }

    fn remove_all_watches(&mut self) -> Result<()> {
        if let Some(ref mut inotify) = self.inotify {
            let mut inotify_watches = inotify.watches();
            for (w, p) in &self.paths {
                inotify_watches
                    .remove(w.clone())
                    .map_err(|e| Error::io(e).add_path(p.into()))?;
            }
            self.watches.clear();
            self.paths.clear();
        }
        Ok(())
    }
}

/// return `DirEntry` when it is a directory
fn filter_dir(e: walkdir::Result<walkdir::DirEntry>) -> Option<walkdir::DirEntry> {
    if let Ok(e) = e {
        if e.file_type().is_dir() {
            return Some(e);
        }
    }
    None
}

impl INotifyWatcher {
    fn from_event_handler(event_handler: Box<dyn EventHandler>, config: &Config) -> Result<Self> {
        let inotify = Inotify::init()?;
        let event_loop = EventLoop::new(inotify, event_handler, config)?;
        let channel = event_loop.event_loop_tx.clone();
        let waker = event_loop.event_loop_waker.clone();
        event_loop.run();
        Ok(INotifyWatcher { channel, waker })
    }

    fn watch_inner(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        let pb = WatchPath::new(path)?;
        let (tx, rx) = unbounded();
        let msg = EventLoopMsg::AddWatch(pb, recursive_mode, tx);

        self.channel.send(msg)?;
        self.waker.wake()?;
        rx.recv().map_err(Error::from)?
    }

    fn unwatch_inner(&mut self, path: &Path) -> Result<()> {
        let pb = absolute_path(path)?;
        let (tx, rx) = unbounded();
        let msg = EventLoopMsg::RemoveWatch(pb, tx);

        self.channel.send(msg)?;
        self.waker.wake()?;
        rx.recv().map_err(Error::from)?
    }

    fn watched_paths_inner(&self) -> Result<Vec<(PathBuf, RecursiveMode)>> {
        let (tx, rx) = unbounded();
        self.channel.send(EventLoopMsg::GetWatchedPaths(tx))?;
        self.waker.wake()?;
        rx.recv().map_err(Error::from)
    }
}

impl Watcher for INotifyWatcher {
    /// Create a new watcher.
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self> {
        Self::from_event_handler(Box::new(event_handler), &config)
    }

    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        self.watch_inner(path, recursive_mode)
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.unwatch_inner(path)
    }

    fn configure(&mut self, config: Config) -> Result<bool> {
        let (tx, rx) = bounded(1);
        self.channel.send(EventLoopMsg::Configure(config, tx))?;
        self.waker.wake()?;
        rx.recv()?
    }

    fn watched_paths(&self) -> Result<Vec<(PathBuf, RecursiveMode)>> {
        self.watched_paths_inner()
    }

    fn kind() -> crate::WatcherKind {
        crate::WatcherKind::Inotify
    }
}

impl Drop for INotifyWatcher {
    fn drop(&mut self) {
        // we expect the event loop to live => unwrap must not panic
        self.channel.send(EventLoopMsg::Shutdown).unwrap();
        self.waker.wake().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::{atomic::AtomicBool, mpsc, Arc},
        thread::{self, available_parallelism},
        time::Duration,
    };

    use super::inotify_sys::WatchMask;
    use super::{
        Config, Error, ErrorKind, Event, EventKind, EventLoop, INotifyWatcher, RecursiveMode,
        Result, WatchPath, Watcher,
    };
    use notify_types::event::{EventKindMask, RemoveKind};

    use crate::test::*;

    fn watcher() -> (TestWatcher<INotifyWatcher>, Receiver) {
        channel()
    }

    /// Create a watcher configured to receive ALL events including Access events.
    /// Use this for tests that verify Access event behavior.
    fn watcher_with_all_events() -> (TestWatcher<INotifyWatcher>, Receiver) {
        channel_with_config(
            ChannelConfig::default()
                .with_timeout(std::time::Duration::from_secs(1))
                .with_watcher_config(Config::default().with_event_kinds(EventKindMask::ALL)),
        )
    }

    #[test]
    fn inotify_watcher_is_send_and_sync() {
        fn check<T: Send + Sync>() {}
        check::<INotifyWatcher>();
    }

    #[test]
    fn native_error_type_on_missing_path() {
        let mut watcher = INotifyWatcher::new(|_| {}, Config::default()).unwrap();

        let result = watcher.watch(
            &PathBuf::from("/some/non/existant/path"),
            RecursiveMode::NonRecursive,
        );

        assert!(matches!(
            result,
            Err(Error {
                paths: _,
                kind: ErrorKind::PathNotFound
            })
        ))
    }

    // Regression test for https://github.com/notify-rs/notify/issues/579.
    #[test]
    fn recursive_watch_ignores_missing_subdir_during_initial_scan() {
        use std::fs;

        let tmpdir = tempfile::tempdir().unwrap();
        let root = tmpdir.path().to_path_buf();
        let disappearing = root.join("disappearing");
        fs::create_dir(&disappearing).unwrap();
        fs::remove_dir_all(&disappearing).unwrap();

        let inotify = super::inotify_sys::Inotify::init().unwrap();
        let mut event_loop = EventLoop::new(inotify, Box::new(|_| {}), &Config::default()).unwrap();

        // Simulate the TOCTOU: we *intend* to watch a subdirectory discovered during initial scan,
        // but it's already gone by the time we call `inotify_add_watch`.
        let result = event_loop.add_watches_for_paths(
            [root, disappearing]
                .into_iter()
                .map(|path| WatchPath::new(&path).unwrap()),
            true,
            true,
        );
        assert!(
            result.is_ok(),
            "expected recursive watch to succeed, got: {result:?}"
        );
    }

    #[test]
    fn rewatching_same_path_replaces_recursive_state() {
        let tmpdir = tempfile::tempdir().unwrap();
        let root = tmpdir.path().to_path_buf();
        let child = root.join("child");
        std::fs::create_dir(&child).unwrap();

        let inotify = super::inotify_sys::Inotify::init().unwrap();
        let mut event_loop = EventLoop::new(inotify, Box::new(|_| {}), &Config::default()).unwrap();

        event_loop
            .add_watch(WatchPath::new(&root).unwrap(), true, true)
            .expect("watch recursively");
        assert!(event_loop.watches.contains_key(&child));

        event_loop
            .add_watch(WatchPath::new(&root).unwrap(), false, true)
            .expect("rewatch non-recursively");

        let watch = event_loop.watches.get(&root).expect("root watch");
        assert!(watch.metadata.is_user_watch);
        assert!(!watch.metadata.user_is_recursive);
        assert!(!watch.metadata.is_recursive);
        assert!(!event_loop.watches.contains_key(&child));
    }

    #[test]
    fn rewatching_child_preserves_recursive_parent_state() {
        let tmpdir = tempfile::tempdir().unwrap();
        let root = tmpdir.path().to_path_buf();
        let child = root.join("child");
        let grandchild = child.join("grandchild");
        std::fs::create_dir_all(&grandchild).unwrap();

        let inotify = super::inotify_sys::Inotify::init().unwrap();
        let mut event_loop = EventLoop::new(inotify, Box::new(|_| {}), &Config::default()).unwrap();

        event_loop
            .add_watch(WatchPath::new(&root).unwrap(), true, true)
            .expect("watch root recursively");
        event_loop
            .add_watch(WatchPath::new(&child).unwrap(), false, true)
            .expect("watch child non-recursively");
        event_loop
            .add_watch(
                WatchPath::from_parts(child.clone(), PathBuf::from("reported-child")),
                false,
                true,
            )
            .expect("rewatch child non-recursively");

        let child_watch = event_loop.watches.get(&child).expect("child watch");
        assert!(child_watch.metadata.is_user_watch);
        assert!(!child_watch.metadata.user_is_recursive);
        assert!(child_watch.metadata.is_recursive);
        assert_eq!(
            child_watch.metadata.reported_path,
            PathBuf::from("reported-child")
        );

        let grandchild_watch = event_loop
            .watches
            .get(&grandchild)
            .expect("grandchild still covered by recursive parent");
        assert!(!grandchild_watch.metadata.is_user_watch);
        assert!(grandchild_watch.metadata.is_recursive);
    }

    #[test]
    fn rewatching_carved_out_child_does_not_restore_parent_recursive_state() {
        let tmpdir = tempfile::tempdir().unwrap();
        let root = tmpdir.path().to_path_buf();
        let child = root.join("child");
        let grandchild = child.join("grandchild");
        std::fs::create_dir_all(&grandchild).unwrap();

        let inotify = super::inotify_sys::Inotify::init().unwrap();
        let mut event_loop = EventLoop::new(inotify, Box::new(|_| {}), &Config::default()).unwrap();

        event_loop
            .add_watch(WatchPath::new(&root).unwrap(), true, true)
            .expect("watch root recursively");
        event_loop
            .remove_watch(child.clone(), false)
            .expect("carve out child");
        event_loop
            .add_watch(WatchPath::new(&child).unwrap(), false, true)
            .expect("watch child non-recursively");
        event_loop
            .add_watch(
                WatchPath::from_parts(child.clone(), PathBuf::from("reported-child")),
                false,
                true,
            )
            .expect("rewatch child non-recursively");

        let child_watch = event_loop.watches.get(&child).expect("child watch");
        assert!(child_watch.metadata.is_user_watch);
        assert!(!child_watch.metadata.user_is_recursive);
        assert!(!child_watch.metadata.is_recursive);
        assert_eq!(
            child_watch.metadata.reported_path,
            PathBuf::from("reported-child")
        );
        assert!(!event_loop.watches.contains_key(&grandchild));
    }

    /// Runs manually.
    ///
    /// * Save actual value of the limit: `MAX_USER_WATCHES=$(sysctl -n fs.inotify.max_user_watches)`
    /// * Run the test.
    /// * Set the limit to 0: `sudo sysctl fs.inotify.max_user_watches=0` while test is running
    /// * Wait for the test to complete
    /// * Restore the limit `sudo sysctl fs.inotify.max_user_watches=$MAX_USER_WATCHES`
    #[test]
    #[ignore = "requires changing sysctl fs.inotify.max_user_watches while test is running"]
    fn recursive_watch_calls_handler_if_creating_a_file_raises_max_files_watch() {
        use std::time::Duration;

        let tmpdir = tempfile::tempdir().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        let (proc_changed_tx, proc_changed_rx) = std::sync::mpsc::channel();
        let proc_path = Path::new("/proc/sys/fs/inotify/max_user_watches");
        let mut watcher = INotifyWatcher::new(
            move |result: Result<Event>| match result {
                Ok(event) => {
                    if event.paths.first().is_some_and(|path| path == proc_path) {
                        proc_changed_tx.send(()).unwrap();
                    }
                }
                Err(e) => tx.send(e).unwrap(),
            },
            Config::default(),
        )
        .unwrap();

        watcher
            .watch(tmpdir.path(), RecursiveMode::Recursive)
            .unwrap();
        watcher
            .watch(proc_path, RecursiveMode::NonRecursive)
            .unwrap();

        // give the time to set the limit
        proc_changed_rx
            .recv_timeout(Duration::from_secs(30))
            .unwrap();

        let child_dir = tmpdir.path().join("child");
        std::fs::create_dir(child_dir).unwrap();

        let result = rx.recv_timeout(Duration::from_millis(500));

        assert!(
            matches!(
                &result,
                Ok(Error {
                    kind: ErrorKind::MaxFilesWatch,
                    paths: _,
                })
            ),
            "expected {:?}, found: {:#?}",
            ErrorKind::MaxFilesWatch,
            result
        );
    }

    /// https://github.com/notify-rs/notify/issues/678
    #[test]
    fn race_condition_on_unwatch_and_pending_events_with_deleted_descriptor() {
        let tmpdir = tempfile::tempdir().expect("tmpdir");
        let (tx, rx) = mpsc::channel();
        // Use CORE to exclude access events - the parallel threads opening files
        // would otherwise flood the queue with OPEN events causing Rescan
        let mut inotify = INotifyWatcher::new(
            move |e: Result<Event>| {
                let e = match e {
                    Ok(e) if e.paths.is_empty() => e,
                    Ok(_) | Err(_) => return,
                };
                let _ = tx.send(e);
            },
            Config::default().with_event_kinds(EventKindMask::CORE),
        )
        .expect("inotify creation");

        let dir_path = tmpdir.path();
        let file_path = dir_path.join("foo");
        std::fs::File::create(&file_path).unwrap();

        let stop = Arc::new(AtomicBool::new(false));

        let handles: Vec<_> = (0..available_parallelism().unwrap().get().max(4))
            .map(|_| {
                let file_path = file_path.clone();
                let stop = stop.clone();
                thread::spawn(move || {
                    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                        let _ = std::fs::File::open(&file_path).unwrap();
                    }
                })
            })
            .collect();

        let non_recursive = RecursiveMode::NonRecursive;
        for _ in 0..(handles.len() * 4) {
            inotify.watch(dir_path, non_recursive).unwrap();
            inotify.unwatch(dir_path).unwrap();
        }

        stop.store(true, std::sync::atomic::Ordering::Relaxed);
        handles
            .into_iter()
            .for_each(|handle| handle.join().ok().unwrap_or_default());

        drop(inotify);

        let events: Vec<_> = rx.into_iter().map(|e| format!("{e:?}")).collect();

        const LOG_LEN: usize = 10;
        let events_len = events.len();
        assert!(
            events.is_empty(),
            "expected no events without path, but got {events_len}. first 10: {:#?}",
            &events[..LOG_LEN.min(events_len)]
        );
    }

    /// https://github.com/notify-rs/notify/issues/709
    #[test]
    fn remove_a_subdir_in_a_recursively_watched_parent() {
        let tmpdir = tempfile::tempdir().expect("tmpdir");
        let subdirectory_path_1 = tmpdir.path().join("subdir");
        let subdirectory_path_2 = subdirectory_path_1.join("nested");
        std::fs::create_dir(&subdirectory_path_1).expect("unable to create a subdir");
        std::fs::create_dir(&subdirectory_path_2).expect("unable to create a nested dir");

        let mut watcher =
            INotifyWatcher::new(|_| (), Config::default()).expect("unable to create watcher");
        watcher
            .watch(tmpdir.path(), RecursiveMode::Recursive)
            .expect("unable to watch");
        std::fs::remove_dir_all(&subdirectory_path_1).expect("unable to remove a subdir");
        let unwatch_result = watcher.unwatch(tmpdir.path());

        assert!(
            matches!(unwatch_result, Ok(())),
            "error: {unwatch_result:#?}"
        );
    }

    #[test]
    fn unmount_event_maps_to_remove_other_with_unmount_info() {
        let path = PathBuf::from("/tmp/notify-unmount");
        let event = super::unmount_event(path.clone());

        assert_eq!(event.kind, EventKind::Remove(RemoveKind::Other));
        assert_eq!(event.paths, vec![path]);
        assert_eq!(event.info(), Some("unmount"));
    }

    #[test]
    fn remove_watch_without_os_call_removes_internal_state() {
        let tmpdir = tempfile::tempdir().unwrap();
        let watched = tmpdir.path().join("watched");
        std::fs::create_dir(&watched).unwrap();

        let inotify = super::inotify_sys::Inotify::init().unwrap();
        let mut event_loop = EventLoop::new(inotify, Box::new(|_| {}), &Config::default()).unwrap();

        event_loop
            .add_watch(WatchPath::new(&watched).unwrap(), false, true)
            .expect("add_watch");

        event_loop
            .remove_watch_without_os_call(watched.clone(), true)
            .expect("remove_watch_without_os_call");

        let result = event_loop.remove_watch(watched.clone(), false);
        assert!(
            matches!(
                result,
                Err(Error {
                    kind: ErrorKind::WatchNotFound,
                    ..
                })
            ),
            "expected WatchNotFound, got: {result:?}"
        );
    }

    #[test]
    fn create_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).create_file(),
            expected(&path).access_open_any(),
            expected(&path).access_close_write(),
        ]);
    }

    #[test]
    fn write_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_recursively(&tmpdir);
        std::fs::write(&path, b"123").expect("write");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).access_open_any(),
            expected(&path).modify_data_any().multiple(),
            expected(&path).access_close_write(),
        ]);
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

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([expected(&path).modify_meta_any()]);
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

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).rename_from(),
            expected(&new_path).rename_to(),
            expected([path, new_path]).rename_both(),
        ])
        .ensure_trackers_len(1);
    }

    #[test]
    fn delete_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::remove_file(&file).expect("remove");

        rx.wait_ordered_exact([expected(&file).remove_file()]);
    }

    #[test]
    fn delete_self_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&file);

        std::fs::remove_file(&file).expect("remove");

        rx.wait_ordered_exact([
            expected(&file).modify_meta_any(),
            expected(&file).remove_file(),
        ]);
    }

    #[test]
    fn delete_self_dir() {
        let tmpdir = testdir();
        let dir = tmpdir.path().join("dir");
        std::fs::create_dir(&dir).expect("create");

        let (mut watcher, mut rx) = watcher();
        watcher.watch_nonrecursively(&dir);

        std::fs::remove_dir(&dir).expect("remove");

        rx.wait_unordered([expected(&dir).remove_folder()]);
    }

    #[test]
    fn create_write_overwrite() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();
        let overwritten_file = tmpdir.path().join("overwritten_file");
        let overwriting_file = tmpdir.path().join("overwriting_file");
        std::fs::write(&overwritten_file, "123").expect("write1");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::File::create(&overwriting_file).expect("create");
        std::fs::write(&overwriting_file, "321").expect("write2");
        std::fs::rename(&overwriting_file, &overwritten_file).expect("rename");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&overwriting_file).create_file(),
            expected(&overwriting_file).access_open_any(),
            expected(&overwriting_file).access_close_write(),
            expected(&overwriting_file).access_open_any(),
            expected(&overwriting_file).modify_data_any().multiple(),
            expected(&overwriting_file).access_close_write(),
            expected(&overwriting_file).rename_from(),
            expected(&overwritten_file).rename_to(),
            expected([overwriting_file, overwritten_file]).rename_both(),
        ])
        .ensure_trackers_len(1);
    }

    #[test]
    fn create_dir() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::create_dir(&path).expect("create");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([expected(&path).create_folder()]);
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

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).access_open_any().optional(),
            expected(&path).modify_meta_any(),
            expected(&path).modify_meta_any(),
        ]);
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

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).access_open_any().optional(),
            expected(&path).rename_from(),
            expected(&new_path).rename_to(),
            expected([path, new_path]).rename_both(),
        ])
        .ensure_trackers_len(1);
    }

    #[test]
    fn delete_dir() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::create_dir(&path).expect("create_dir");

        watcher.watch_recursively(&tmpdir);
        std::fs::remove_dir(&path).expect("remove");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).access_open_any().optional(),
            expected(&path).remove_folder(),
        ]);
    }

    #[test]
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

        // Use wait_ordered (not _exact) because we may get extra events
        // due to directory traversal/rescan on rename
        rx.wait_ordered([
            expected(&path).access_open_any().optional(),
            expected(&path).rename_from(),
            expected(&new_path).rename_to(),
            expected([&path, &new_path]).rename_both(),
            expected(&new_path).access_open_any().optional(),
            expected(&new_path).rename_from(),
            expected(&new_path2).rename_to(),
            expected([&new_path, &new_path2]).rename_both(),
        ])
        .ensure_trackers_len(2);
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

        // With ALL events, we may get Access events on the directory.
        // Skip Access events to find the rename event.
        let event = loop {
            let event = rx.recv();
            if !matches!(event.kind, EventKind::Access(_)) {
                break event;
            }
        };
        let tracker = event.attrs.tracker();
        assert_eq!(event, expected(&path).rename_from());
        assert!(tracker.is_some(), "tracker is none: {event:#?}");
    }

    #[test]
    fn create_write_write_rename_write_remove() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();

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

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&file1).create_file(),
            expected(&file1).access_open_any(),
            expected(&file1).modify_data_any().multiple(),
            expected(&file1).access_close_write(),
            expected(&file2).access_open_any(),
            expected(&file2).modify_data_any().multiple(),
            expected(&file2).access_close_write(),
            expected(&file1).access_open_any().optional(),
            expected(&file1).rename_from(),
            expected(&new_path).rename_to(),
            expected([&file1, &new_path]).rename_both(),
            expected(&new_path).access_open_any(),
            expected(&new_path).modify_data_any().multiple(),
            expected(&new_path).access_close_write(),
            expected(&new_path).remove_file(),
        ]);
    }

    #[test]
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

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).access_open_any().optional(),
            expected(&path).rename_from(),
            expected(&new_path1).rename_to(),
            expected([&path, &new_path1]).rename_both(),
            expected(&new_path1).access_open_any().optional(),
            expected(&new_path1).rename_from(),
            expected(&new_path2).rename_to(),
            expected([&new_path1, &new_path2]).rename_both(),
        ])
        .ensure_trackers_len(2);
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

        // With ALL events, we may get Access events on the directory.
        // Skip Access events to find the modify event.
        let event = loop {
            let event = rx.recv();
            if !matches!(event.kind, EventKind::Access(_)) {
                break event;
            }
        };
        assert_eq!(event, expected(&path).modify_data_any());
    }

    #[test]
    fn write_file_non_recursive_watch() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_nonrecursively(&path);

        std::fs::write(&path, b"123").expect("write");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&path).access_open_any(),
            expected(&path).modify_data_any().multiple(),
            expected(&path).access_close_write(),
        ]);
    }

    #[test]
    fn watch_recursively_then_unwatch_child_stops_events_from_child() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();

        let subdir = tmpdir.path().join("subdir");
        let file = subdir.join("file");
        std::fs::create_dir(&subdir).expect("create");

        watcher.watch_recursively(&tmpdir);

        std::fs::File::create(&file).expect("create");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&subdir).access_open_any().optional(),
            expected(&file).create_file(),
            expected(&file).access_open_any(),
            expected(&file).access_close_write(),
        ]);

        watcher.watcher.unwatch(&subdir).expect("unwatch");

        std::fs::write(&file, b"123").expect("write");

        std::fs::remove_dir_all(&subdir).expect("remove_dir_all");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&subdir).access_open_any().optional(),
            expected(&subdir).remove_folder(),
        ]);
    }

    #[test]
    fn write_to_a_hardlink_pointed_to_the_watched_file_triggers_an_event() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();

        let subdir = tmpdir.path().join("subdir");
        let file = subdir.join("file");
        let hardlink = tmpdir.path().join("hardlink");

        std::fs::create_dir(&subdir).expect("create");
        std::fs::write(&file, "").expect("file");
        std::fs::hard_link(&file, &hardlink).expect("hardlink");

        watcher.watch_nonrecursively(&file);

        std::fs::write(&hardlink, "123123").expect("write to the hard link");

        // Use wait_ordered (not _exact) because with ALL events we may get
        // directory access events that are timing-dependent
        rx.wait_ordered([
            expected(&file).access_open_any(),
            expected(&file).modify_data_any().multiple(),
            expected(&file).access_close_write(),
        ]);
    }

    #[test]
    fn write_to_a_hardlink_pointed_to_the_file_in_the_watched_dir_doesnt_trigger_an_event() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let subdir = tmpdir.path().join("subdir");
        let file = subdir.join("file");
        let hardlink = tmpdir.path().join("hardlink");

        std::fs::create_dir(&subdir).expect("create");
        std::fs::write(&file, "").expect("file");
        std::fs::hard_link(&file, &hardlink).expect("hardlink");

        watcher.watch_nonrecursively(&subdir);

        std::fs::write(&hardlink, "123123").expect("write to the hard link");

        // With ALL events, we may get Access events on the watched directory.
        // Filter those out - we only care about non-Access events on the file.
        let events: Vec<_> = rx
            .iter()
            .filter(|e| !matches!(e.kind, EventKind::Access(_)))
            .collect();
        assert!(events.is_empty(), "unexpected events: {events:#?}");
    }

    #[test]
    #[ignore = "see https://github.com/notify-rs/notify/issues/727"]
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
        rx.wait_ordered([
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

    // ============================================================
    // EventKindMask filtering tests
    // ============================================================

    /// Test that CORE config does not produce Access events.
    #[test]
    fn event_kind_mask_core_config_no_access_events() {
        let tmpdir = testdir();
        // Use explicit CORE mask to exclude access events
        let (mut watcher, mut rx) = channel_with_config::<INotifyWatcher>(
            ChannelConfig::default()
                .with_timeout(std::time::Duration::from_secs(1))
                .with_watcher_config(Config::default().with_event_kinds(EventKindMask::CORE)),
        );
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        // With CORE config, we should only get create event, no access events
        rx.wait_ordered_exact([expected(&path).create_file()])
            .ensure_no_tail();
    }

    /// Test that EventKindMask::ALL config produces Access events.
    #[test]
    fn event_kind_mask_all_config_produces_access_events() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher_with_all_events();
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        // With ALL config, we should get create + access events
        // Use wait_ordered (not _exact) because we may get directory access events too
        rx.wait_ordered([
            expected(&path).create_file(),
            expected(&path).access_open_any(),
            expected(&path).access_close_write(),
        ]);
    }

    /// Test that CREATE | REMOVE only mask filters out Modify events.
    #[test]
    fn event_kind_mask_create_remove_only_no_modify() {
        let tmpdir = testdir();
        let mask = EventKindMask::CREATE | EventKindMask::REMOVE;
        let (mut watcher, mut rx) = channel_with_config::<INotifyWatcher>(
            ChannelConfig::default()
                .with_timeout(std::time::Duration::from_secs(1))
                .with_watcher_config(Config::default().with_event_kinds(mask)),
        );
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::write(&path, b"123").expect("write");

        // With CREATE | REMOVE mask, we should only get create event, no modify events
        rx.wait_ordered_exact([expected(&path).create_file()])
            .ensure_no_tail();
    }

    /// Test unit tests for event_kind_mask_to_watch_mask helper function.
    #[test]
    fn event_kind_mask_to_watch_mask_core() {
        use super::event_kind_mask_to_watch_mask;

        let mask = EventKindMask::CORE;
        let watch_mask = event_kind_mask_to_watch_mask(mask, false);

        // CORE includes CREATE, REMOVE, MODIFY_DATA, MODIFY_META, MODIFY_NAME
        assert!(watch_mask.intersects(WatchMask::CREATE));
        assert!(watch_mask.intersects(WatchMask::MOVED_TO));
        assert!(watch_mask.intersects(WatchMask::DELETE));
        assert!(watch_mask.intersects(WatchMask::MOVED_FROM));
        assert!(watch_mask.intersects(WatchMask::MODIFY));
        assert!(watch_mask.intersects(WatchMask::ATTRIB));
        assert!(watch_mask.intersects(WatchMask::MOVE_SELF));

        // CORE does NOT include ACCESS (OPEN, CLOSE_WRITE, CLOSE_NOWRITE)
        // Note: CLOSE_WRITE generates Access events, not Modify events
        assert!(!watch_mask.intersects(WatchMask::OPEN));
        assert!(!watch_mask.intersects(WatchMask::CLOSE_WRITE));
        assert!(!watch_mask.intersects(WatchMask::CLOSE_NOWRITE));
    }

    #[test]
    fn event_kind_mask_to_watch_mask_all() {
        use super::event_kind_mask_to_watch_mask;

        let mask = EventKindMask::ALL;
        let watch_mask = event_kind_mask_to_watch_mask(mask, false);

        // ALL includes everything from CORE plus ACCESS
        assert!(watch_mask.intersects(WatchMask::OPEN));
        assert!(watch_mask.intersects(WatchMask::CLOSE_WRITE));
        assert!(watch_mask.intersects(WatchMask::CLOSE_NOWRITE));
    }

    #[test]
    fn event_kind_mask_to_watch_mask_empty() {
        use super::event_kind_mask_to_watch_mask;

        let mask = EventKindMask::empty();
        let watch_mask = event_kind_mask_to_watch_mask(mask, false);

        // Empty mask should produce empty watch mask
        assert!(watch_mask.is_empty());
    }

    #[test]
    fn event_kind_mask_to_watch_mask_access_only() {
        use super::event_kind_mask_to_watch_mask;

        // ACCESS_CLOSE only maps to CLOSE_WRITE (not CLOSE_NOWRITE)
        let mask = EventKindMask::ACCESS_OPEN | EventKindMask::ACCESS_CLOSE;
        let watch_mask = event_kind_mask_to_watch_mask(mask, false);

        assert!(watch_mask.intersects(WatchMask::OPEN));
        assert!(watch_mask.intersects(WatchMask::CLOSE_WRITE));
        assert!(!watch_mask.intersects(WatchMask::CLOSE_NOWRITE));

        // Should NOT have create/modify/remove
        assert!(!watch_mask.intersects(WatchMask::CREATE));
        assert!(!watch_mask.intersects(WatchMask::DELETE));
        assert!(!watch_mask.intersects(WatchMask::MODIFY));
        assert!(!watch_mask.intersects(WatchMask::ATTRIB));
    }

    #[test]
    fn event_kind_mask_to_watch_mask_all_access() {
        use super::event_kind_mask_to_watch_mask;

        // ALL_ACCESS includes OPEN, CLOSE_WRITE, and CLOSE_NOWRITE
        let mask = EventKindMask::ALL_ACCESS;
        let watch_mask = event_kind_mask_to_watch_mask(mask, false);

        assert!(watch_mask.intersects(WatchMask::OPEN));
        assert!(watch_mask.intersects(WatchMask::CLOSE_WRITE));
        assert!(watch_mask.intersects(WatchMask::CLOSE_NOWRITE));
    }

    #[test]
    fn event_kind_mask_to_watch_mask_recursive_includes_create() {
        use super::event_kind_mask_to_watch_mask;

        // Recursive mode includes CREATE|MOVED_TO even without CREATE in mask
        let watch_mask = event_kind_mask_to_watch_mask(EventKindMask::MODIFY_DATA, true);
        assert!(watch_mask.intersects(WatchMask::CREATE));
        assert!(watch_mask.intersects(WatchMask::MOVED_TO));
        assert!(watch_mask.intersects(WatchMask::MODIFY));

        // Empty mask with recursive still includes CREATE|MOVED_TO
        let watch_mask = event_kind_mask_to_watch_mask(EventKindMask::empty(), true);
        assert!(watch_mask.intersects(WatchMask::CREATE));
        assert!(watch_mask.intersects(WatchMask::MOVED_TO));
        assert!(!watch_mask.intersects(WatchMask::MODIFY));

        // Non-recursive mode does not include CREATE when not requested
        let watch_mask = event_kind_mask_to_watch_mask(EventKindMask::MODIFY_DATA, false);
        assert!(!watch_mask.intersects(WatchMask::CREATE));
        assert!(!watch_mask.intersects(WatchMask::MOVED_TO));
        assert!(watch_mask.intersects(WatchMask::MODIFY));
    }

    /// Recursive watches with MODIFY-only mask still track new subdirectories.
    #[test]
    fn recursive_watch_tracks_subdirs_without_create_mask() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = channel_with_config::<INotifyWatcher>(
            ChannelConfig::default()
                .with_timeout(std::time::Duration::from_secs(2))
                .with_watcher_config(
                    Config::default().with_event_kinds(EventKindMask::MODIFY_DATA),
                ),
        );
        watcher.watch_recursively(&tmpdir);

        let subdir = tmpdir.path().join("subdir");
        std::fs::create_dir(&subdir).expect("create subdir");

        // Wait for watch to be added on new subdirectory
        std::thread::sleep(std::time::Duration::from_millis(50));

        let file_path = subdir.join("file.txt");
        let mut file = std::fs::File::create_new(&file_path).expect("create file");

        use std::io::Write;
        file.write_all(b"hello").expect("write");
        file.flush().expect("flush");
        drop(file);

        // Receives MODIFY from subdir (tracking works), no CREATE events (filtered)
        rx.wait_ordered_exact([expected(&file_path).modify_data()])
            .ensure_no_tail();
    }
}
