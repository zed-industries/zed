//! Watcher implementation for Darwin's FSEvents API
//!
//! The FSEvents API provides a mechanism to notify clients about directories they ought to re-scan
//! in order to keep their internal data structures up-to-date with respect to the true state of
//! the file system. (For example, when files or directories are created, modified, or removed.) It
//! sends these notifications "in bulk", possibly notifying the client of changes to several
//! directories in a single callback.
//!
//! For more information see the [FSEvents API reference][ref].
//!
//! TODO: document event translation
//!
//! [ref]: https://developer.apple.com/library/mac/documentation/Darwin/Reference/FSEvents_Ref/

#![allow(non_upper_case_globals, dead_code)]

use crate::paths::{absolute_path, reported_path};
use crate::{event::*, PathOp};
use crate::{
    unbounded, Config, Error, EventHandler, EventKindMask, RecursiveMode, Result, Sender, Watcher,
};
use objc2_core_foundation as cf;
use objc2_core_services as fs;
use std::collections::HashMap;
use std::ffi::{CStr, OsStr};
use std::fmt;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex};
use std::thread;

bitflags::bitflags! {
  #[repr(C)]
  #[derive(Debug)]
  struct StreamFlags: u32 {
    const NONE = fs::kFSEventStreamEventFlagNone;
    const MUST_SCAN_SUBDIRS = fs::kFSEventStreamEventFlagMustScanSubDirs;
    const USER_DROPPED = fs::kFSEventStreamEventFlagUserDropped;
    const KERNEL_DROPPED = fs::kFSEventStreamEventFlagKernelDropped;
    const IDS_WRAPPED = fs::kFSEventStreamEventFlagEventIdsWrapped;
    const HISTORY_DONE = fs::kFSEventStreamEventFlagHistoryDone;
    const ROOT_CHANGED = fs::kFSEventStreamEventFlagRootChanged;
    const MOUNT = fs::kFSEventStreamEventFlagMount;
    const UNMOUNT = fs::kFSEventStreamEventFlagUnmount;
    const ITEM_CREATED = fs::kFSEventStreamEventFlagItemCreated;
    const ITEM_REMOVED = fs::kFSEventStreamEventFlagItemRemoved;
    const INODE_META_MOD = fs::kFSEventStreamEventFlagItemInodeMetaMod;
    const ITEM_RENAMED = fs::kFSEventStreamEventFlagItemRenamed;
    const ITEM_MODIFIED = fs::kFSEventStreamEventFlagItemModified;
    const FINDER_INFO_MOD = fs::kFSEventStreamEventFlagItemFinderInfoMod;
    const ITEM_CHANGE_OWNER = fs::kFSEventStreamEventFlagItemChangeOwner;
    const ITEM_XATTR_MOD = fs::kFSEventStreamEventFlagItemXattrMod;
    const IS_FILE = fs::kFSEventStreamEventFlagItemIsFile;
    const IS_DIR = fs::kFSEventStreamEventFlagItemIsDir;
    const IS_SYMLINK = fs::kFSEventStreamEventFlagItemIsSymlink;
    const OWN_EVENT = fs::kFSEventStreamEventFlagOwnEvent;
    const IS_HARDLINK = fs::kFSEventStreamEventFlagItemIsHardlink;
    const IS_LAST_HARDLINK = fs::kFSEventStreamEventFlagItemIsLastHardlink;
    const ITEM_CLONED = fs::kFSEventStreamEventFlagItemCloned;
  }
}

/// FSEvents-based `Watcher` implementation
pub struct FsEventWatcher {
    paths: cf::CFRetained<cf::CFMutableArray<cf::CFString>>,
    since_when: fs::FSEventStreamEventId,
    latency: cf::CFTimeInterval,
    flags: fs::FSEventStreamCreateFlags,
    event_handler: Arc<Mutex<dyn EventHandler>>,
    runloop: Option<RunLoopHandle>,
    recursive_info: HashMap<PathBuf, WatchInfo>,
    event_kinds: EventKindMask,
}

#[derive(Clone, Debug)]
struct WatchInfo {
    is_recursive: bool,
    reported_path: PathBuf,
}

#[derive(Debug)]
struct RunLoopHandle {
    runloop: cf::CFRetained<cf::CFRunLoop>,
    stop_source: cf::CFRetained<cf::CFRunLoopSource>,
    thread_handle: thread::JoinHandle<()>,
}

impl fmt::Debug for FsEventWatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FsEventWatcher")
            .field("paths", &self.paths)
            .field("since_when", &self.since_when)
            .field("latency", &self.latency)
            .field("flags", &self.flags)
            .field("event_handler", &Arc::as_ptr(&self.event_handler))
            .field("runloop", &self.runloop)
            .field("recursive_info", &self.recursive_info)
            .finish()
    }
}

// FsEventWatcher is not Send/Sync automatically.
// It's Send because the pointer is not used in other threads.
unsafe impl Send for FsEventWatcher {}

// It's Sync because all methods that change the mutable state use `&mut self`.
unsafe impl Sync for FsEventWatcher {}

fn translate_flags(flags: StreamFlags, precise: bool) -> Vec<Event> {
    let mut evs = Vec::new();
    translate_flags_with(flags, precise, |ev| evs.push(ev));
    evs
}

// Keep this in sync with `translate_flags_with`; the callback uses it to avoid path clones.
fn translated_event_count(flags: &StreamFlags, precise: bool) -> usize {
    if flags.contains(StreamFlags::HISTORY_DONE) {
        return 0;
    }

    let mut count = usize::from(flags.contains(StreamFlags::MUST_SCAN_SUBDIRS));
    if !precise {
        return count + 1;
    }

    let root_changed = flags.contains(StreamFlags::ROOT_CHANGED);
    count += usize::from(root_changed);
    count += usize::from(flags.contains(StreamFlags::MOUNT));
    count += usize::from(flags.contains(StreamFlags::UNMOUNT));
    count += usize::from(flags.contains(StreamFlags::ITEM_CREATED));
    count += usize::from(flags.contains(StreamFlags::ITEM_RENAMED) && !root_changed);
    count += usize::from(flags.contains(StreamFlags::INODE_META_MOD));
    count += usize::from(flags.contains(StreamFlags::FINDER_INFO_MOD));
    count += usize::from(flags.contains(StreamFlags::ITEM_CHANGE_OWNER));
    count += usize::from(flags.contains(StreamFlags::ITEM_XATTR_MOD));
    count += usize::from(flags.contains(StreamFlags::ITEM_MODIFIED));
    count += usize::from(flags.contains(StreamFlags::ITEM_REMOVED) && !root_changed);
    count
}

fn translate_flags_with(flags: StreamFlags, precise: bool, mut emit: impl FnMut(Event)) {
    // «Denotes a sentinel event sent to mark the end of the "historical" events
    // sent as a result of specifying a `sinceWhen` value in the FSEvents.Create
    // call that created this event stream. After invoking the client's callback
    // with all the "historical" events that occurred before now, the client's
    // callback will be invoked with an event where the HistoryDone flag is set.
    // The client should ignore the path supplied in this callback.»
    // — https://www.mbsplugins.eu/FSEventsNextEvent.shtml
    //
    // As a result, we just stop processing here and return an empty vec, which
    // will ignore this completely and not emit any Events whatsoever.
    if flags.contains(StreamFlags::HISTORY_DONE) {
        return;
    }

    // `ITEM_CLONED` can be present alongside other flags (including create/modify/remove).
    // Preserve any existing `info` (like "root changed"), but annotate otherwise so downstream
    // can detect and filter clone-related events. See https://github.com/notify-rs/notify/issues/465.
    let clone_related = precise && flags.contains(StreamFlags::ITEM_CLONED);
    let own_process_id = if precise && flags.contains(StreamFlags::OWN_EVENT) {
        Some(std::process::id())
    } else {
        None
    };

    let mut emit_event = |mut ev: Event| {
        if clone_related && ev.info().is_none() {
            ev.attrs.set_info("is: clone");
        }
        if let Some(process_id) = own_process_id {
            ev.attrs.set_process_id(process_id);
        }
        emit(ev);
    };

    // FSEvents provides two possible hints as to why events were dropped,
    // however documentation on what those mean is scant, so we just pass them
    // through in the info attr field. The intent is clear enough, and the
    // additional information is provided if the user wants it.
    if flags.contains(StreamFlags::MUST_SCAN_SUBDIRS) {
        let e = Event::new(EventKind::Other).set_flag(Flag::Rescan);
        emit_event(if flags.contains(StreamFlags::USER_DROPPED) {
            e.set_info("rescan: user dropped")
        } else if flags.contains(StreamFlags::KERNEL_DROPPED) {
            e.set_info("rescan: kernel dropped")
        } else {
            e
        });
    }

    // In imprecise mode, let's not even bother parsing the kind of the event
    // except for the above very special events.
    if !precise {
        emit(Event::new(EventKind::Any));
        return;
    }

    // A watched root changed (renamed or removed). If the flags provide a hint,
    // prefer that over guessing. Otherwise, treat it as a removal to avoid
    // misclassifying a delete as a rename.
    let root_changed = flags.contains(StreamFlags::ROOT_CHANGED);
    if root_changed {
        let kind = if flags.contains(StreamFlags::ITEM_REMOVED) {
            if flags.contains(StreamFlags::IS_DIR) {
                EventKind::Remove(RemoveKind::Folder)
            } else if flags.contains(StreamFlags::IS_FILE) {
                EventKind::Remove(RemoveKind::File)
            } else {
                EventKind::Remove(RemoveKind::Any)
            }
        } else if flags.contains(StreamFlags::ITEM_RENAMED) {
            EventKind::Modify(ModifyKind::Name(RenameMode::From))
        } else {
            EventKind::Remove(RemoveKind::Any)
        };

        emit_event(Event::new(kind).set_info("root changed"));
    }

    // A path was mounted at the event path; we treat that as a create.
    if flags.contains(StreamFlags::MOUNT) {
        emit_event(Event::new(EventKind::Create(CreateKind::Other)).set_info("mount"));
    }

    // A path was unmounted at the event path; we treat that as a remove.
    if flags.contains(StreamFlags::UNMOUNT) {
        emit_event(Event::new(EventKind::Remove(RemoveKind::Other)).set_info("mount"));
    }

    if flags.contains(StreamFlags::ITEM_CREATED) {
        emit_event(if flags.contains(StreamFlags::IS_DIR) {
            Event::new(EventKind::Create(CreateKind::Folder))
        } else if flags.contains(StreamFlags::IS_FILE) {
            Event::new(EventKind::Create(CreateKind::File))
        } else {
            let e = Event::new(EventKind::Create(CreateKind::Other));
            if flags.contains(StreamFlags::IS_SYMLINK) {
                e.set_info("is: symlink")
            } else if flags.contains(StreamFlags::IS_HARDLINK) {
                e.set_info("is: hardlink")
            } else if flags.contains(StreamFlags::ITEM_CLONED) {
                e.set_info("is: clone")
            } else {
                Event::new(EventKind::Create(CreateKind::Any))
            }
        });
    }

    // FSEvents provides no mechanism to associate the old and new sides of a
    // rename event.
    // Avoid emitting duplicate events around a root change by checking `root_changed`.
    if flags.contains(StreamFlags::ITEM_RENAMED) && !root_changed {
        emit_event(Event::new(EventKind::Modify(ModifyKind::Name(
            RenameMode::Any,
        ))));
    }

    // This is only described as "metadata changed", but it may be that it's
    // only emitted for some more precise subset of events... if so, will need
    // amending, but for now we have an Any-shaped bucket to put it in.
    if flags.contains(StreamFlags::INODE_META_MOD) {
        emit_event(Event::new(EventKind::Modify(ModifyKind::Metadata(
            MetadataKind::Any,
        ))));
    }

    if flags.contains(StreamFlags::FINDER_INFO_MOD) {
        emit_event(
            Event::new(EventKind::Modify(ModifyKind::Metadata(MetadataKind::Other)))
                .set_info("meta: finder info"),
        );
    }

    if flags.contains(StreamFlags::ITEM_CHANGE_OWNER) {
        emit_event(Event::new(EventKind::Modify(ModifyKind::Metadata(
            MetadataKind::Ownership,
        ))));
    }

    if flags.contains(StreamFlags::ITEM_XATTR_MOD) {
        emit_event(Event::new(EventKind::Modify(ModifyKind::Metadata(
            MetadataKind::Extended,
        ))));
    }

    // This is specifically described as a data change, which we take to mean
    // is a content change.
    if flags.contains(StreamFlags::ITEM_MODIFIED) {
        emit_event(Event::new(EventKind::Modify(ModifyKind::Data(
            DataChange::Content,
        ))));
    }

    // Avoid emitting duplicate events around a root change by checking `root_changed`.
    if flags.contains(StreamFlags::ITEM_REMOVED) && !root_changed {
        emit_event(if flags.contains(StreamFlags::IS_DIR) {
            Event::new(EventKind::Remove(RemoveKind::Folder))
        } else if flags.contains(StreamFlags::IS_FILE) {
            Event::new(EventKind::Remove(RemoveKind::File))
        } else {
            let e = Event::new(EventKind::Remove(RemoveKind::Other));
            if flags.contains(StreamFlags::IS_SYMLINK) {
                e.set_info("is: symlink")
            } else if flags.contains(StreamFlags::IS_HARDLINK) {
                e.set_info("is: hardlink")
            } else if flags.contains(StreamFlags::ITEM_CLONED) {
                e.set_info("is: clone")
            } else {
                Event::new(EventKind::Remove(RemoveKind::Any))
            }
        });
    }
}

struct StreamContextInfo {
    event_handler: Arc<Mutex<dyn EventHandler>>,
    recursive_info: HashMap<PathBuf, WatchInfo>,
    event_kinds: EventKindMask,
}

// Free the context when the stream created by `FSEventStreamCreate` is released.
unsafe extern "C-unwind" fn release_context(info: *const libc::c_void) {
    // Safety:
    // - The [documentation] for `FSEventStreamContext` states that `release` is only
    //   called when the stream is deallocated, so it is safe to convert `info` back into a
    //   box and drop it.
    //
    // [docs]: https://developer.apple.com/documentation/coreservices/fseventstreamcontext?language=objc
    unsafe {
        drop(Box::from_raw(
            info as *const StreamContextInfo as *mut StreamContextInfo,
        ));
    }
}

// Runs on the watcher thread, inside the running runloop, where
// `CFRunLoopStop` is guaranteed to take effect.
unsafe extern "C-unwind" fn stop_runloop_perform(_info: *mut std::ffi::c_void) {
    if let Some(runloop) = cf::CFRunLoop::current() {
        runloop.stop();
    }
}

impl FsEventWatcher {
    fn from_event_handler(
        event_handler: Arc<Mutex<dyn EventHandler>>,
        event_kinds: EventKindMask,
        latency: cf::CFTimeInterval,
    ) -> Result<Self> {
        Ok(FsEventWatcher {
            paths: cf::CFMutableArray::empty(),
            since_when: fs::kFSEventStreamEventIdSinceNow,
            latency,
            flags: fs::kFSEventStreamCreateFlagFileEvents
                | fs::kFSEventStreamCreateFlagNoDefer
                | fs::kFSEventStreamCreateFlagWatchRoot,
            event_handler,
            runloop: None,
            recursive_info: HashMap::new(),
            event_kinds,
        })
    }

    fn watch_inner(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        let result = self.append_path(path, recursive_mode);
        self.restart()?;
        result
    }

    fn unwatch_inner(&mut self, path: &Path) -> Result<()> {
        let result = self.remove_path(path);
        self.restart()?;
        result
    }

    fn update_paths_inner(
        &mut self,
        ops: Vec<crate::PathOp>,
    ) -> crate::StdResult<(), crate::UpdatePathsError> {
        let result = crate::update_paths(ops, |op| match op {
            crate::PathOp::Watch(path, config) => self
                .append_path(&path, config.recursive_mode())
                .map_err(|e| (PathOp::Watch(path, config), e)),
            crate::PathOp::Unwatch(path) => self
                .remove_path(&path)
                .map_err(|e| (PathOp::Unwatch(path), e)),
        });

        match self.restart() {
            Err(run_error) => match result {
                Ok(()) => Err(crate::UpdatePathsError {
                    source: run_error,
                    origin: None,
                    remaining: Default::default(),
                }),
                Err(path_op_error) => {
                    log::error!(
                        "Unable to run fsevents watcher after updating paths error: {run_error:?}"
                    );
                    Err(path_op_error)
                }
            },
            Ok(()) => result,
        }
    }

    #[inline]
    fn is_running(&self) -> bool {
        self.runloop.is_some()
    }

    // Replace the running stream (if any) with one watching the current path
    // set. The new stream is started before the old one is stopped, so that a
    // stream is watching at every moment: events firing inside a stop-then-start
    // window would be silently dropped for every watched path, not just the one
    // being (un)watched. An event can be delivered through both streams during
    // the swap; duplicates are fine, losses are not. Resuming the new stream
    // from an older event id was tried and made things worse: historical replay
    // stalls live delivery. If the new stream fails to start, the old one is
    // kept running so the previous path set keeps delivering events.
    fn restart(&mut self) -> Result<()> {
        let old_runloop = self.runloop.take();
        let result = self.run();
        match &result {
            Ok(()) => {
                if let Some(handle) = old_runloop {
                    Self::stop_handle(handle);
                }
            }
            Err(_) => {
                debug_assert!(self.runloop.is_none());
                self.runloop = old_runloop;
            }
        }
        result
    }

    fn stop(&mut self) {
        if !self.is_running() {
            return;
        }

        if let Some(handle) = self.runloop.take() {
            Self::stop_handle(handle);
        }
    }

    fn stop_handle(handle: RunLoopHandle) {
        let RunLoopHandle {
            runloop,
            stop_source,
            thread_handle,
        } = handle;
        {
            // Calling `CFRunLoopStop` directly here would race: it only takes effect
            // while the runloop is actually running, so a stop landing in the window
            // before the watcher thread enters `CFRunLoopRun` would be lost and the
            // `join` below would deadlock. Signaling a runloop source instead is
            // sticky: the signal stays pending until the loop runs, and the source's
            // `perform` callback then stops the loop from the inside, where the stop
            // cannot be lost. The wake-up covers the case where the loop is already
            // asleep.
            stop_source.signal();
            runloop.wake_up();
            // Wait for the thread to shut down.
            thread_handle.join().expect("thread to shut down");
        }
    }

    fn remove_path(&mut self, path: &Path) -> Result<()> {
        let p = path
            .canonicalize()
            .ok()
            .or_else(|| {
                self.recursive_info
                    .iter()
                    .find(|(_, info)| info.reported_path == path)
                    .map(|(path, _)| path.clone())
            })
            .or_else(|| absolute_path(path).ok())
            .unwrap_or_else(|| path.to_owned());
        self.remove_cf_path(&p)?;

        match self.recursive_info.remove(&p) {
            Some(_) => Ok(()),
            None => Err(Error::watch_not_found()),
        }
    }

    fn remove_cf_path(&mut self, path: &Path) -> Result<()> {
        let mut err: *mut cf::CFError = ptr::null_mut();
        let Some(cf_path) = (unsafe { path_to_cfstring_ref(path, &mut err) }) else {
            if let Some(err) = NonNull::new(err) {
                let _ = unsafe { cf::CFRetained::from_raw(err) };
            }
            return Err(Error::watch_not_found().add_path(path.into()));
        };

        let mut to_remove = Vec::new();
        for (idx, item) in self.paths.iter().enumerate() {
            if item.compare(
                Some(&cf_path),
                cf::CFStringCompareFlags::CompareCaseInsensitive,
            ) == cf::CFComparisonResult::CompareEqualTo
            {
                to_remove.push(idx as cf::CFIndex);
            }
        }

        for idx in to_remove.iter().rev() {
            unsafe {
                cf::CFMutableArray::remove_value_at_index(Some(self.paths.as_opaque()), *idx)
            };
        }
        Ok(())
    }

    // https://github.com/thibaudgg/rb-fsevent/blob/master/ext/fsevent_watch/main.c
    fn append_path(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        if !path.exists() {
            return Err(Error::path_not_found().add_path(path.into()));
        }
        let canonical_path = path.to_path_buf().canonicalize()?;
        let mut err: *mut cf::CFError = ptr::null_mut();
        let Some(cf_path) = (unsafe { path_to_cfstring_ref(&canonical_path, &mut err) }) else {
            if let Some(err) = NonNull::new(err) {
                let _ = unsafe { cf::CFRetained::from_raw(err) };
            }
            // Most likely the directory was deleted, or permissions changed,
            // while the above code was running.
            return Err(Error::path_not_found().add_path(path.into()));
        };
        if self.recursive_info.contains_key(&canonical_path) {
            self.remove_cf_path(&canonical_path)?;
        }
        self.paths.append(&cf_path);

        self.recursive_info.insert(
            canonical_path,
            WatchInfo {
                is_recursive: recursive_mode.is_recursive(),
                reported_path: path.to_path_buf(),
            },
        );
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        if self.paths.is_empty() {
            return Ok(());
        }

        // We need to associate the stream context with our callback in order to propagate events
        // to the rest of the system. This will be owned by the stream, and will be freed when the
        // stream is closed. This means we will leak the context if we panic before reaching
        // `FSEventStreamRelease`.
        let context = Box::into_raw(Box::new(StreamContextInfo {
            event_handler: self.event_handler.clone(),
            recursive_info: self.recursive_info.clone(),
            event_kinds: self.event_kinds,
        }));

        let stream_context = fs::FSEventStreamContext {
            version: 0,
            info: context as *mut libc::c_void,
            retain: None,
            release: Some(release_context),
            copyDescription: None,
        };

        let stream = unsafe {
            fs::FSEventStreamCreate(
                cf::kCFAllocatorDefault,
                Some(callback),
                &stream_context as *const _ as *mut _,
                self.paths.as_opaque(),
                self.since_when,
                self.latency,
                self.flags,
            )
        };

        // Wrapper to help send CFRunLoop types across threads.
        struct CFRunLoopSendWrapper(
            cf::CFRetained<cf::CFRunLoop>,
            cf::CFRetained<cf::CFRunLoopSource>,
        );

        // Safety:
        // - According to the Apple documentation, it's safe to move `CFRunLoop`s and
        //   `CFRunLoopSource`s across threads.
        //   https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html
        unsafe impl Send for CFRunLoopSendWrapper {}

        // Wrapper to help send FSEventStreamRef types across threads.
        struct FSEventStreamSendWrapper(fs::FSEventStreamRef);

        // TODO: Write docs for the safety of this impl.
        // SAFETY: Unclear?
        unsafe impl Send for FSEventStreamSendWrapper {}

        // move into thread
        let stream = FSEventStreamSendWrapper(stream);

        // channel to pass runloop around
        let (rl_tx, rl_rx) = unbounded();

        let thread_handle = thread::Builder::new()
            .name("notify-rs fsevents loop".to_string())
            .spawn(move || {
                let _ = &stream;
                let stream = stream.0;

                unsafe {
                    // Safety:
                    // This may panic if OOM occurs.
                    // Related: https://github.com/madsmtm/objc2/issues/797
                    let cur_runloop =
                        cf::CFRunLoop::current().expect("Failed to get current runloop");

                    #[allow(deprecated)]
                    fs::FSEventStreamScheduleWithRunLoop(
                        stream,
                        &cur_runloop,
                        cf::kCFRunLoopDefaultMode.expect("Failed to get default runloop mode"),
                    );
                    if !fs::FSEventStreamStart(stream) {
                        fs::FSEventStreamInvalidate(stream);
                        fs::FSEventStreamRelease(stream);
                        rl_tx
                            .send(Err(Error::generic("unable to start FSEvent stream")))
                            .expect("Unable to send error for FSEventStreamStart");
                        return;
                    }

                    // The source through which stop() asks this thread to shut down. It
                    // must be created and added to the runloop before the handles are
                    // published below, so the caller can never signal a source that is
                    // not registered yet.
                    let mut stop_source_context = cf::CFRunLoopSourceContext {
                        version: 0,
                        info: ptr::null_mut(),
                        retain: None,
                        release: None,
                        copyDescription: None,
                        equal: None,
                        hash: None,
                        schedule: None,
                        cancel: None,
                        perform: Some(stop_runloop_perform),
                    };
                    let stop_source = cf::CFRunLoopSource::new(
                        cf::kCFAllocatorDefault,
                        0,
                        &mut stop_source_context,
                    )
                    .expect("Failed to create stop source");
                    cur_runloop.add_source(Some(&stop_source), cf::kCFRunLoopDefaultMode);

                    // `stop()` will signal `stop_source`, wake the runloop, and then join
                    // this thread.
                    rl_tx
                        .send(Ok(CFRunLoopSendWrapper(cur_runloop, stop_source.clone())))
                        .expect("Unable to send runloop to watcher");

                    // Block until the pending signal on `stop_source` (if any) or a later
                    // one stops the loop from the inside; see `stop()`.
                    cf::CFRunLoop::run();

                    stop_source.invalidate();
                    fs::FSEventStreamStop(stream);
                    fs::FSEventStreamInvalidate(stream);
                    fs::FSEventStreamRelease(stream);
                }
            })?;
        // block until runloop has been sent
        let runloop_wrapper = match rl_rx.recv() {
            Ok(Ok(runloop_wrapper)) => runloop_wrapper,
            Ok(Err(err)) => {
                thread_handle
                    .join()
                    .expect("thread to shut down after FSEvent stream startup failure");
                return Err(err);
            }
            Err(_) => {
                thread_handle
                    .join()
                    .expect("thread to shut down after FSEvent stream startup channel close");
                return Err(Error::generic(
                    "unable to receive FSEvent stream startup result",
                ));
            }
        };
        self.runloop = Some(RunLoopHandle {
            runloop: runloop_wrapper.0,
            stop_source: runloop_wrapper.1,
            thread_handle,
        });

        Ok(())
    }

    fn configure_raw_mode(&mut self, _config: Config, tx: Sender<Result<bool>>) {
        tx.send(Ok(false))
            .expect("configuration channel disconnect");
    }
}

unsafe extern "C-unwind" fn callback(
    stream_ref: fs::ConstFSEventStreamRef,
    info: *mut libc::c_void,
    num_events: libc::size_t,                          // size_t numEvents
    event_paths: NonNull<libc::c_void>,                // void *eventPaths
    event_flags: NonNull<fs::FSEventStreamEventFlags>, // const FSEventStreamEventFlags eventFlags[]
    event_ids: NonNull<fs::FSEventStreamEventId>,      // const FSEventStreamEventId eventIds[]
) {
    // Never unwind into CoreServices; if something goes wrong, drop the events and log.
    // This also protects against panics from user-provided `EventHandler` implementations.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        callback_impl(
            stream_ref,
            info,
            num_events,
            event_paths,
            event_flags,
            event_ids,
        )
    }))
    .map_err(|_| {
        log::error!("panic in FSEvents callback; dropping pending events");
    });
}

unsafe fn callback_impl(
    _stream_ref: fs::ConstFSEventStreamRef,
    info: *mut libc::c_void,
    num_events: libc::size_t,                          // size_t numEvents
    event_paths: NonNull<libc::c_void>,                // void *eventPaths
    event_flags: NonNull<fs::FSEventStreamEventFlags>, // const FSEventStreamEventFlags eventFlags[]
    _event_ids: NonNull<fs::FSEventStreamEventId>,     // const FSEventStreamEventId eventIds[]
) {
    let event_paths = event_paths.as_ptr() as *const *const libc::c_char;
    let info = info as *const StreamContextInfo;
    let event_handler_mutex = &(*info).event_handler;
    let event_kinds = (*info).event_kinds;
    let mut event_handler_guard = None;

    for p in 0..num_events {
        // Paths are not guaranteed to be valid UTF-8 (e.g. NFS); keep them as raw bytes.
        let path = CStr::from_ptr(*event_paths.add(p));
        let path = Path::new(OsStr::from_bytes(path.to_bytes()));

        let raw_flag = *event_flags.as_ptr().add(p) as u32;
        let flag = StreamFlags::from_bits_truncate(raw_flag);
        let unknown_bits = raw_flag & !StreamFlags::all().bits();
        if unknown_bits != 0 {
            // `FSEventStreamEventFlags` is an extensible bitfield; tolerate future flags.
            log::trace!("unknown FSEventStreamEventFlags bits: 0x{unknown_bits:08x}");
        }

        let mut watch_match = None;
        for (watch_path, watch_info) in &(*info).recursive_info {
            if path.starts_with(watch_path) {
                let matches_watch = if watch_info.is_recursive || path == watch_path {
                    true
                } else if let Some(parent_path) = path.parent() {
                    parent_path == watch_path
                } else {
                    false
                };

                if matches_watch
                    && watch_match.as_ref().is_none_or(
                        |(matched_path, _): &(&PathBuf, &WatchInfo)| {
                            watch_path.as_os_str().as_bytes().len()
                                > matched_path.as_os_str().as_bytes().len()
                        },
                    )
                {
                    watch_match = Some((watch_path, watch_info));
                }
            }
        }

        let Some((watch_path, watch_info)) = watch_match else {
            continue;
        };
        let translated_count = translated_event_count(&flag, true);
        if translated_count == 0 {
            continue;
        }
        // Most FSEvents flags produce one Event; move the reported path in that case.
        let mut event_path = Some(reported_path(watch_path, &watch_info.reported_path, path));
        let single_translated_event = translated_count == 1;

        log::trace!("FSEvent: path = `{}`, flag = {:?}", path.display(), flag);

        translate_flags_with(flag, true, |mut ev| {
            // Filter events based on EventKindMask before adding the path.
            if !event_kinds.matches(&ev.kind) {
                return;
            }
            if single_translated_event {
                ev.paths.push(
                    event_path.take().unwrap_or_else(|| {
                        reported_path(watch_path, &watch_info.reported_path, path)
                    }),
                );
            } else {
                ev.paths
                    .push(event_path.as_ref().expect("translated event path").clone());
            }

            let event_handler =
                event_handler_guard.get_or_insert_with(|| match event_handler_mutex.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                });
            // Protect against panicking event handlers, which would otherwise unwind into
            // the CoreServices callback.
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                event_handler.handle_event(Ok(ev));
            }))
            .map_err(|_| {
                log::error!("panic in FSEvents event handler; dropping event");
            });
        });
    }
}

impl Watcher for FsEventWatcher {
    /// Create a new watcher.
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self> {
        Self::from_event_handler(
            Arc::new(Mutex::new(event_handler)),
            config.event_kinds(),
            config.fsevent_latency().as_secs_f64(),
        )
    }

    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        self.watch_inner(path, recursive_mode)
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.unwatch_inner(path)
    }

    fn update_paths(&mut self, ops: Vec<PathOp>) -> crate::StdResult<(), crate::UpdatePathsError> {
        self.update_paths_inner(ops)
    }

    fn configure(&mut self, config: Config) -> Result<bool> {
        let (tx, rx) = unbounded();
        self.configure_raw_mode(config, tx);
        rx.recv()?
    }

    fn watched_paths(&self) -> Result<Vec<(PathBuf, RecursiveMode)>> {
        // Unlike the channel-based backends, FSEvents keeps watch state on the watcher itself.
        // The runloop callback gets a cloned snapshot in `StreamContextInfo`, so it does not
        // mutate or read this map concurrently.
        Ok(self
            .recursive_info
            .iter()
            .map(|(_path, info)| {
                (
                    info.reported_path.clone(),
                    if info.is_recursive {
                        RecursiveMode::Recursive
                    } else {
                        RecursiveMode::NonRecursive
                    },
                )
            })
            .collect())
    }

    fn kind() -> crate::WatcherKind {
        crate::WatcherKind::Fsevent
    }
}

impl Drop for FsEventWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Grabbed from <https://docs.rs/fsevent-sys/4.1.0/src/fsevent_sys/core_foundation.rs.html#149-230>.
///
/// TODO: Could we simplify this?
unsafe fn path_to_cfstring_ref(
    source: &Path,
    err: &mut *mut cf::CFError,
) -> Option<cf::CFRetained<cf::CFString>> {
    let url = cf::CFURL::from_file_path(source)?;

    let mut placeholder = url.absolute_url()?;

    let imaginary = cf::CFMutableArray::empty();

    while !unsafe { placeholder.resource_is_reachable(err) } {
        if let Some(child) = placeholder.last_path_component() {
            imaginary.insert(0, &*child);
        }

        placeholder = cf::CFURL::new_copy_deleting_last_path_component(None, Some(&placeholder))?;
    }

    let url = unsafe { cf::CFURL::new_file_reference_url(None, Some(&placeholder), err) }?;

    let mut placeholder = unsafe { cf::CFURL::new_file_path_url(None, Some(&url), err) }?;

    for component in imaginary {
        placeholder = cf::CFURL::new_copy_appending_path_component(
            None,
            Some(&placeholder),
            Some(&component),
            false,
        )?;
    }

    placeholder.file_system_path(cf::CFURLPathStyle::CFURLPOSIXPathStyle)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{ErrorKind, WatchPathConfig};

    use super::*;
    use crate::test::*;

    fn watcher() -> (TestWatcher<FsEventWatcher>, Receiver) {
        channel()
    }

    #[test]
    fn rewatching_same_path_replaces_recursive_info() {
        let dir = tempfile::tempdir().unwrap();
        let mut watcher = FsEventWatcher::new(|_| {}, Config::default()).unwrap();

        watcher
            .append_path(dir.path(), RecursiveMode::Recursive)
            .expect("watch recursively");
        watcher
            .append_path(dir.path(), RecursiveMode::NonRecursive)
            .expect("rewatch non-recursively");

        let watched = watcher.watched_paths().expect("watched paths");
        assert_eq!(
            watched,
            vec![(dir.path().to_path_buf(), RecursiveMode::NonRecursive)]
        );
        assert_eq!(watcher.paths.iter().count(), 1);
    }

    #[test]
    fn stop_does_not_wait_for_runloop_to_be_waiting() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::mpsc;
        use std::time::Instant;

        // Regression test for a shutdown hang where `stop()` waited for
        // `CFRunLoopIsWaiting()` to become true. If the runloop is busy (e.g. under
        // heavy event load), it may never enter a waiting state.

        let dir = tempfile::tempdir().unwrap();

        let (tx, _rx) = mpsc::channel::<crate::Result<Event>>();
        let mut watcher = FsEventWatcher::new(tx, Default::default()).unwrap();
        watcher.watch(dir.path(), RecursiveMode::Recursive).unwrap();

        let runloop = watcher
            .runloop
            .as_ref()
            .expect("watcher to be running")
            .runloop
            .clone();
        let mode = unsafe { cf::kCFRunLoopDefaultMode.expect("default runloop mode") };

        // Keep the runloop continuously "busy" by creating a source that signals itself in its
        // perform callback.
        struct SourceHammer {
            source: *const cf::CFRunLoopSource,
            fires: AtomicUsize,
        }

        unsafe extern "C-unwind" fn hammer_source(info: *mut std::ffi::c_void) {
            let Some(hammer) = (info as *const SourceHammer).as_ref() else {
                return;
            };
            hammer.fires.fetch_add(1, Ordering::Relaxed);

            // Signal the source again so the runloop has more work to do.
            let Some(source) = (hammer.source as *const cf::CFRunLoopSource).as_ref() else {
                return;
            };
            source.signal();
        }

        let mut hammer = Box::new(SourceHammer {
            source: std::ptr::null(),
            fires: AtomicUsize::new(0),
        });

        let mut ctx = cf::CFRunLoopSourceContext {
            version: 0,
            info: (&mut *hammer as *mut SourceHammer).cast(),
            retain: None,
            release: None,
            copyDescription: None,
            equal: None,
            hash: None,
            schedule: None,
            cancel: None,
            perform: Some(hammer_source),
        };

        let source = unsafe {
            cf::CFRunLoopSource::new(cf::kCFAllocatorDefault, 0, &mut ctx)
                .expect("source to be created")
        };
        hammer.source = cf::CFRetained::as_ptr(&source).as_ptr();

        runloop.add_source(Some(&source), Some(mode));
        source.signal();
        runloop.wake_up();

        // Ensure our setup actually made the runloop busy.
        let setup_start = Instant::now();
        while hammer.fires.load(Ordering::Relaxed) == 0
            && setup_start.elapsed() < Duration::from_secs(1)
        {
            std::thread::yield_now();
        }
        assert!(
            hammer.fires.load(Ordering::Relaxed) > 0,
            "runloop source never fired; test setup failed"
        );

        let (done_tx, done_rx) = mpsc::channel::<()>();
        std::thread::spawn(move || {
            drop(watcher);
            let _ = done_tx.send(());
        });

        // If shutdown regresses, this would hang indefinitely; keep the test bounded.
        done_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("dropping FsEventWatcher timed out (possible shutdown hang)");

        // No cleanup: The source is owned by the runloop; removing sources cross-thread can be
        // sensitive on some systems. Dropping the last reference to the runloop will release it.
    }

    // Regression test for a lost `CFRunLoopStop`: stopping is a no-op while the
    // runloop thread is between publishing its handles and actually entering
    // `CFRunLoopRun`, so a single stop could leave the thread parked forever and
    // deadlock the join in `stop()`. Rapid watch/unwatch cycles maximize pressure
    // on that window.
    #[test]
    fn rapid_watch_unwatch_does_not_hang() {
        use std::sync::mpsc;

        let tmpdir = tempfile::tempdir().unwrap();
        let dir_a = tmpdir.path().join("a");
        let dir_b = tmpdir.path().join("b");
        std::fs::create_dir(&dir_a).expect("create_dir a");
        std::fs::create_dir(&dir_b).expect("create_dir b");

        let (done_tx, done_rx) = mpsc::channel::<()>();
        let stress_thread = thread::spawn(move || {
            let (tx, _rx) = mpsc::channel::<crate::Result<Event>>();
            let mut watcher = FsEventWatcher::new(tx, Default::default()).unwrap();
            for _ in 0..500 {
                // Errors are tolerated: under load (e.g. the 4096-path test running
                // concurrently) fseventsd transiently refuses stream starts even for
                // tiny path sets. The property under test is purely that none of
                // these operations hangs.
                let _ = watcher.watch(&dir_a, RecursiveMode::NonRecursive);
                let _ = watcher.watch(&dir_b, RecursiveMode::NonRecursive);
                let _ = watcher.unwatch(&dir_a);
                let _ = watcher.unwatch(&dir_b);
            }
            let _ = done_tx.send(());
        });

        done_rx
            .recv_timeout(Duration::from_secs(120))
            .expect("rapid watch/unwatch timed out (lost CFRunLoopStop?)");
        stress_thread.join().expect("stress thread to shut down");
    }

    // Deterministic test of the race window above: a stop source signaled
    // before `CFRunLoopRun` is entered must still stop the loop, because
    // source signals stay pending until the loop processes them.
    #[test]
    fn stop_source_signaled_before_runloop_run_still_stops_loop() {
        use std::sync::mpsc;

        struct CFSendWrapper<T: cf::Type>(cf::CFRetained<T>);
        unsafe impl<T: cf::Type> Send for CFSendWrapper<T> {}

        let (handles_tx, handles_rx) = mpsc::channel();
        let (signaled_tx, signaled_rx) = mpsc::channel::<()>();
        let (done_tx, done_rx) = mpsc::channel::<()>();

        let loop_thread = thread::spawn(move || {
            let cur_runloop = cf::CFRunLoop::current().expect("current runloop");

            let mut stop_source_context = cf::CFRunLoopSourceContext {
                version: 0,
                info: ptr::null_mut(),
                retain: None,
                release: None,
                copyDescription: None,
                equal: None,
                hash: None,
                schedule: None,
                cancel: None,
                perform: Some(stop_runloop_perform),
            };
            let stop_source = unsafe {
                cf::CFRunLoopSource::new(cf::kCFAllocatorDefault, 0, &mut stop_source_context)
                    .expect("stop source to be created")
            };
            let mode = unsafe { cf::kCFRunLoopDefaultMode.expect("default runloop mode") };
            cur_runloop.add_source(Some(&stop_source), Some(mode));

            handles_tx
                .send((
                    CFSendWrapper(cur_runloop),
                    CFSendWrapper(stop_source.clone()),
                ))
                .expect("send runloop handles");

            signaled_rx
                .recv()
                .expect("wait for the stop source to be signaled");

            cf::CFRunLoop::run();

            stop_source.invalidate();
            let _ = done_tx.send(());
        });

        let (runloop, stop_source) = handles_rx.recv().expect("receive runloop handles");
        stop_source.0.signal();
        runloop.0.wake_up();
        signaled_tx.send(()).expect("release the loop thread");

        done_rx
            .recv_timeout(Duration::from_secs(10))
            .expect("CFRunLoopRun did not exit; pre-run stop source signal was lost");
        loop_thread.join().expect("loop thread to shut down");
    }

    #[test]
    fn test_fsevent_watcher_drop() {
        use super::*;
        use std::time::Duration;

        let dir = tempfile::tempdir().unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        {
            let mut watcher = FsEventWatcher::new(tx, Default::default()).unwrap();
            watcher.watch(dir.path(), RecursiveMode::Recursive).unwrap();
            thread::sleep(Duration::from_millis(2000));
            println!("is running -> {}", watcher.is_running());

            thread::sleep(Duration::from_millis(1000));
            watcher.unwatch(dir.path()).unwrap();
            println!("is running -> {}", watcher.is_running());
        }

        thread::sleep(Duration::from_millis(1000));

        for res in rx {
            let e = res.unwrap();
            println!("debug => {:?} {:?}", e.kind, e.paths);
        }

        println!("in test: {} works", file!());
    }

    #[test]
    fn test_steam_context_info_send_and_sync() {
        fn check_send<T: Send + Sync>() {}
        check_send::<StreamContextInfo>();
    }

    #[test]
    fn callback_impl_handles_non_utf8_paths_without_panicking() {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        use std::ptr;

        let (tx, rx) = std::sync::mpsc::channel::<crate::Result<Event>>();
        let event_handler: Arc<Mutex<dyn EventHandler>> = Arc::new(Mutex::new(tx));

        let mut recursive_info = HashMap::new();
        recursive_info.insert(
            PathBuf::from("/tmp"),
            WatchInfo {
                is_recursive: true,
                reported_path: PathBuf::from("/tmp"),
            },
        );

        let context = Box::new(StreamContextInfo {
            event_handler,
            recursive_info,
            event_kinds: EventKindMask::ALL,
        });
        let context_ptr = Box::into_raw(context) as *mut libc::c_void;

        let bytes = b"/tmp/\xff";
        let c_path = CString::new(bytes.as_slice()).expect("cstring");
        let path_ptrs = [c_path.as_ptr()];
        let event_paths = NonNull::new(path_ptrs.as_ptr() as *mut libc::c_void).unwrap();

        let flags_arr = [StreamFlags::ITEM_CREATED.bits() as fs::FSEventStreamEventFlags];
        let event_flags =
            NonNull::new(flags_arr.as_ptr() as *mut fs::FSEventStreamEventFlags).unwrap();

        let ids_arr = [0 as fs::FSEventStreamEventId];
        let event_ids = NonNull::new(ids_arr.as_ptr() as *mut fs::FSEventStreamEventId).unwrap();

        let res = std::panic::catch_unwind(|| unsafe {
            callback_impl(
                ptr::null(),
                context_ptr,
                1,
                event_paths,
                event_flags,
                event_ids,
            );
        });
        unsafe {
            drop(Box::from_raw(context_ptr as *mut StreamContextInfo));
        }

        assert!(res.is_ok(), "callback_impl should not panic");

        let event = rx
            .recv_timeout(Duration::from_secs(1))
            .expect("expected event")
            .expect("expected Ok(Event)");
        assert!(
            event.kind.is_create(),
            "expected create event, got {event:?}"
        );
        assert_eq!(event.paths.len(), 1);
        assert_eq!(event.paths[0].as_os_str().as_bytes(), bytes);
    }

    #[test]
    fn callback_impl_ignores_unknown_flag_bits_without_panicking() {
        use std::ffi::CString;
        use std::ptr;

        let (tx, rx) = std::sync::mpsc::channel::<crate::Result<Event>>();
        let event_handler: Arc<Mutex<dyn EventHandler>> = Arc::new(Mutex::new(tx));

        let mut recursive_info = HashMap::new();
        recursive_info.insert(
            PathBuf::from("/tmp"),
            WatchInfo {
                is_recursive: true,
                reported_path: PathBuf::from("/tmp"),
            },
        );

        let context = Box::new(StreamContextInfo {
            event_handler,
            recursive_info,
            event_kinds: EventKindMask::ALL,
        });
        let context_ptr = Box::into_raw(context) as *mut libc::c_void;

        let c_path = CString::new("/tmp/file").expect("cstring");
        let path_ptrs = [c_path.as_ptr()];
        let event_paths = NonNull::new(path_ptrs.as_ptr() as *mut libc::c_void).unwrap();

        // Include an unknown bit so the old `from_bits(...).unwrap_or_else(panic!)` behavior
        // would have panicked. New behavior should tolerate it.
        let unknown_mask = !StreamFlags::all().bits();
        let unknown_bit = unknown_mask & unknown_mask.wrapping_neg();
        assert_ne!(unknown_bit, 0, "StreamFlags unexpectedly uses all bits");
        let raw_flag = StreamFlags::ITEM_CREATED.bits() | unknown_bit;
        assert!(
            StreamFlags::from_bits(raw_flag).is_none(),
            "raw_flag must include an unknown bit for this test to be meaningful"
        );

        let flags_arr = [raw_flag as fs::FSEventStreamEventFlags];
        let event_flags =
            NonNull::new(flags_arr.as_ptr() as *mut fs::FSEventStreamEventFlags).unwrap();

        let ids_arr = [0 as fs::FSEventStreamEventId];
        let event_ids = NonNull::new(ids_arr.as_ptr() as *mut fs::FSEventStreamEventId).unwrap();

        let res = std::panic::catch_unwind(|| unsafe {
            callback_impl(
                ptr::null(),
                context_ptr,
                1,
                event_paths,
                event_flags,
                event_ids,
            );
        });
        unsafe {
            drop(Box::from_raw(context_ptr as *mut StreamContextInfo));
        }

        assert!(res.is_ok(), "callback_impl should not panic");

        let event = rx
            .recv_timeout(Duration::from_secs(1))
            .expect("expected event")
            .expect("expected Ok(Event)");
        assert!(
            event.kind.is_create(),
            "expected create event, got {event:?}"
        );
    }

    #[test]
    fn translate_flags_ignores_is_file_only_events() {
        assert!(translate_flags(StreamFlags::IS_FILE, true).is_empty());
        assert!(
            translate_flags(StreamFlags::IS_FILE | StreamFlags::ITEM_CLONED, true).is_empty(),
            "type-only clone flags should not produce events"
        );
    }

    #[test]
    fn translate_flags_sets_clone_info_for_file_events() {
        let create = translate_flags(
            StreamFlags::ITEM_CREATED | StreamFlags::IS_FILE | StreamFlags::ITEM_CLONED,
            true,
        );
        assert_eq!(create.len(), 1);
        assert_eq!(create[0].kind, EventKind::Create(CreateKind::File));
        assert_eq!(create[0].info(), Some("is: clone"));

        let modify = translate_flags(
            StreamFlags::INODE_META_MOD
                | StreamFlags::ITEM_MODIFIED
                | StreamFlags::IS_FILE
                | StreamFlags::ITEM_CLONED,
            true,
        );
        assert_eq!(modify.len(), 2);
        assert!(modify
            .iter()
            .any(|e| matches!(e.kind, EventKind::Modify(ModifyKind::Metadata(_)))));
        assert!(modify
            .iter()
            .any(|e| matches!(e.kind, EventKind::Modify(ModifyKind::Data(_)))));
        assert!(
            modify.iter().all(|e| e.info() == Some("is: clone")),
            "all events should be annotated as clone-related: {modify:?}"
        );
    }

    #[test]
    fn translate_flags_does_not_override_existing_info() {
        let evs = translate_flags(
            StreamFlags::ROOT_CHANGED
                | StreamFlags::ITEM_REMOVED
                | StreamFlags::IS_FILE
                | StreamFlags::ITEM_CLONED,
            true,
        );
        assert_eq!(evs.len(), 1);
        assert_eq!(evs[0].info(), Some("root changed"));
    }

    #[test]
    fn does_not_crash_with_empty_path() {
        let mut watcher = FsEventWatcher::new(|_| {}, Default::default()).unwrap();

        let watch_result = watcher.watch(Path::new(""), RecursiveMode::Recursive);
        assert!(
            matches!(
                watch_result,
                Err(Error {
                    kind: ErrorKind::PathNotFound,
                    paths: _
                })
            ),
            "actual: {watch_result:#?}"
        );

        let unwatch_result = watcher.unwatch(Path::new(""));
        assert!(
            matches!(
                unwatch_result,
                Err(Error {
                    kind: ErrorKind::WatchNotFound,
                    paths: _
                })
            ),
            "actual: {unwatch_result:#?}"
        );
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

        rx.wait_unordered([expected(&path).modify_data_content()]);
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

        rx.wait_unordered([expected(&path).modify_meta_owner()]);
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

        rx.wait_unordered([expected(path).rename_any(), expected(new_path).rename_any()]);
    }

    #[test]
    fn delete_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::remove_file(&file).expect("remove");

        rx.wait_unordered([expected(&file).remove_file()]);
    }

    #[test]
    fn delete_self_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&file);

        std::fs::remove_file(&file).expect("remove");

        rx.wait_unordered([expected(file).remove_file()]);
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
        let (mut watcher, mut rx) = watcher();
        let overwritten_file = tmpdir.path().join("overwritten_file");
        let overwriting_file = tmpdir.path().join("overwriting_file");
        std::fs::write(&overwritten_file, "123").expect("write1");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::File::create(&overwriting_file).expect("create");
        std::fs::write(&overwriting_file, "321").expect("write2");
        std::fs::rename(&overwriting_file, &overwritten_file).expect("rename");

        rx.wait_unordered([
            expected(&overwriting_file).create(),
            expected(&overwriting_file).modify_data_content().multiple(),
            expected(&overwriting_file).rename_any(),
            expected(&overwritten_file).rename_any(),
        ]);
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

        rx.wait_unordered([expected(&path).modify_meta_owner()]);
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
            expected(&path).rename_any(),
            expected(&new_path).rename_any(),
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

        rx.wait_unordered([expected(path).remove_folder()]);
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

        rx.wait_unordered([
            expected(&path).rename_any(),
            expected(&new_path).rename_any(),
            expected(&new_path2).rename_any(),
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
    #[ignore = "https://github.com/notify-rs/notify/issues/729"]
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
            expected(&new_path2).rename_any(),
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

        rx.wait_unordered([expected(path).modify_data_content()]);
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

        rx.wait_unordered([expected(file).modify_data_content()]);
    }

    #[test]
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

    #[test]
    fn fsevent_watcher_respects_event_kind_mask() {
        use crate::Watcher;
        use notify_types::event::EventKindMask;

        let tmpdir = testdir();
        let (tx, rx) = std::sync::mpsc::channel();

        // Create watcher with CREATE-only mask (no MODIFY events)
        let config = Config::default().with_event_kinds(EventKindMask::CREATE);

        let mut watcher = FsEventWatcher::new(tx, config).expect("create watcher");
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
            "Expected CREATE event, got: {:?}",
            events
        );

        // Should NOT have MODIFY event (filtered out)
        assert!(
            !events.iter().any(|e| e.kind.is_modify()),
            "Should not receive MODIFY events with CREATE-only mask, got: {:?}",
            events
        );
    }

    // fsevents seems to not allow watching more than 4096 paths at once.
    // https://github.com/fsnotify/fsevents/issues/48
    // Based on https://github.com/fsnotify/fsevents/commit/3899270de121c963202e6fed46aa31d5ec7b3908
    #[test]
    fn error_properly_on_stream_start_failure() {
        let tmpdir = testdir();
        let (mut watcher, _rx) = watcher();

        let mut paths = Vec::new();

        for i in 0..=4096 {
            let path = tmpdir.path().join(format!("dir_{i}/subdir"));
            std::fs::create_dir_all(&path).expect("create_dir");
            paths.push(PathOp::Watch(
                path,
                WatchPathConfig::new(RecursiveMode::NonRecursive),
            ));
        }

        assert!(watcher.watcher.update_paths(paths).is_err());

        // Best-effort cleanup: on macOS + recent rustc, `remove_dir_all` can
        // panic with `closedir: Bad file descriptor` while tearing down the
        // 4097 directories created above (likely an interaction with fsevents
        // having held FDs on those paths). Bypass `TempDir`'s Drop and swallow
        // the potential panic so the test does not flake.
        let path = tmpdir.path().to_path_buf();
        std::mem::forget(tmpdir);
        let _ = std::panic::catch_unwind(|| {
            let _ = std::fs::remove_dir_all(&path);
        });
    }

    #[test]
    fn rename_then_remove_remove_event_must_be_the_last_one() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_recursively(&tmpdir);
        let new_path1 = tmpdir.path().join("renamed1");
        let new_path2 = tmpdir.path().join("renamed2");

        std::fs::rename(&path, &new_path1).expect("rename1");
        std::fs::rename(&new_path1, &new_path2).expect("rename2");

        std::fs::remove_file(&new_path2).expect("remove_file");

        loop {
            let ev = rx.recv();
            if matches!(ev.kind, EventKind::Remove(RemoveKind::File)) {
                assert_eq!(&ev.paths, &[new_path2]);
                break;
            }
        }

        rx.ensure_empty();
    }
}
