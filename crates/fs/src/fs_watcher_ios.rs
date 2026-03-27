use parking_lot::Mutex;
use std::collections::HashMap;
use std::ffi::c_void;
use std::os::fd::RawFd;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::{PathEvent, PathEventKind, Watcher};

// GCD dispatch source FFI bindings for DISPATCH_SOURCE_TYPE_VNODE
type DispatchSourceRef = *mut c_void;
type DispatchQueueRef = *mut c_void;
type DispatchSourceTypeRef = *const c_void;

unsafe extern "C" {
    static _dispatch_source_type_vnode: c_void;
    fn dispatch_source_create(
        type_: DispatchSourceTypeRef,
        handle: usize,
        mask: u64,
        queue: DispatchQueueRef,
    ) -> DispatchSourceRef;
    fn dispatch_source_set_event_handler_f(
        source: DispatchSourceRef,
        handler: extern "C" fn(*mut c_void),
    );
    fn dispatch_set_context(object: DispatchSourceRef, context: *mut c_void);
    fn dispatch_resume(object: DispatchSourceRef);
    fn dispatch_source_cancel(source: DispatchSourceRef);
    fn dispatch_release(object: DispatchSourceRef);
    fn dispatch_get_global_queue(identifier: i64, flags: u64) -> DispatchQueueRef;
    fn dispatch_source_set_cancel_handler_f(
        source: DispatchSourceRef,
        handler: extern "C" fn(*mut c_void),
    );
    fn dispatch_source_get_data(source: DispatchSourceRef) -> u64;
}

const DISPATCH_VNODE_DELETE: u64 = 0x1;
const DISPATCH_VNODE_WRITE: u64 = 0x2;
const DISPATCH_VNODE_EXTEND: u64 = 0x4;
const DISPATCH_VNODE_ATTRIB: u64 = 0x8;
const DISPATCH_VNODE_RENAME: u64 = 0x20;
const DISPATCH_VNODE_REVOKE: u64 = 0x40;

const VNODE_WATCH_MASK: u64 = DISPATCH_VNODE_DELETE
    | DISPATCH_VNODE_WRITE
    | DISPATCH_VNODE_EXTEND
    | DISPATCH_VNODE_ATTRIB
    | DISPATCH_VNODE_RENAME
    | DISPATCH_VNODE_REVOKE;

/// Shared state for a single watched path. Lives behind an Arc so both the
/// dispatch source event handler and the FsWatcher can access it.
struct WatchState {
    path: PathBuf,
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    /// The currently active dispatch source + fd. Replaced on file rename/delete.
    active: Mutex<Option<ActiveSource>>,
}

struct ActiveSource {
    source: DispatchSourceRef,
    fd: RawFd,
}

// SAFETY: DispatchSourceRef is a GCD object that is thread-safe.
unsafe impl Send for ActiveSource {}
unsafe impl Sync for ActiveSource {}

impl Drop for ActiveSource {
    fn drop(&mut self) {
        unsafe {
            dispatch_source_cancel(self.source);
            dispatch_release(self.source);
            libc::close(self.fd);
        }
    }
}

fn create_vnode_source(
    state: &Arc<WatchState>,
) -> anyhow::Result<ActiveSource> {
    let c_path = std::ffi::CString::new(
        state
            .path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("path is not valid UTF-8: {:?}", state.path))?,
    )?;

    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_EVTONLY) };
    if fd < 0 {
        return Err(anyhow::anyhow!(
            "failed to open {:?} for watching: {}",
            state.path,
            std::io::Error::last_os_error()
        ));
    }

    let queue = unsafe { dispatch_get_global_queue(0, 0) };
    let source = unsafe {
        dispatch_source_create(
            &_dispatch_source_type_vnode as *const _ as DispatchSourceTypeRef,
            fd as usize,
            VNODE_WATCH_MASK,
            queue,
        )
    };

    if source.is_null() {
        unsafe { libc::close(fd) };
        return Err(anyhow::anyhow!(
            "dispatch_source_create failed for {:?}",
            state.path
        ));
    }

    // The context pointer is a raw Arc pointer. The event handler borrows it
    // (increment on clone, no decrement). The cancel handler decrements the
    // strong count so the Arc is eventually freed.
    let ctx_ptr = Arc::into_raw(Arc::clone(state)) as *mut c_void;

    unsafe {
        dispatch_set_context(source, ctx_ptr);
        dispatch_source_set_event_handler_f(source, vnode_event_handler);
        dispatch_source_set_cancel_handler_f(source, vnode_cancel_handler);
        dispatch_resume(source);
    }

    Ok(ActiveSource { source, fd })
}

extern "C" fn vnode_event_handler(context: *mut c_void) {
    // SAFETY: context is an Arc<WatchState> pointer kept alive by the cancel handler.
    let state = unsafe { &*(context as *const WatchState) };

    let event = PathEvent {
        path: state.path.clone(),
        kind: Some(PathEventKind::Changed),
    };
    {
        let mut pending = state.pending_path_events.lock();
        if pending.is_empty() {
            state.tx.try_send(()).ok();
        }
        pending.push(event);
    }

    // If the file was deleted or renamed (atomic_write does rename), we need
    // to re-establish the watch on the new inode at the same path.
    // We read the event flags from the source to check. The source pointer
    // is in `state.active`, but we can also just try to re-watch unconditionally
    // after delete/rename — the old source will be dropped, closing the stale fd.
    let source = {
        let guard = state.active.lock();
        guard.as_ref().map(|a| a.source)
    };
    if let Some(source) = source {
        let flags = unsafe { dispatch_source_get_data(source) };
        if flags & (DISPATCH_VNODE_DELETE | DISPATCH_VNODE_RENAME | DISPATCH_VNODE_REVOKE) != 0 {
            // Clone the Arc so we can call create_vnode_source.
            // SAFETY: we're borrowing from the Arc pointer in context; we manually
            // increment the ref count for the clone.
            let state_arc = unsafe {
                Arc::increment_strong_count(context as *const WatchState);
                Arc::from_raw(context as *const WatchState)
            };

            // Small delay to let the rename settle before re-opening.
            std::thread::sleep(std::time::Duration::from_millis(50));

            match create_vnode_source(&state_arc) {
                Ok(new_active) => {
                    // Replace the active source; dropping the old one cancels it
                    // and releases the stale fd.
                    let mut guard = state.active.lock();
                    *guard = Some(new_active);
                }
                Err(err) => {
                    log::warn!("failed to re-watch {:?} after rename: {err}", state.path);
                }
            }
        }
    }
}

extern "C" fn vnode_cancel_handler(context: *mut c_void) {
    // SAFETY: Decrement the Arc strong count. This matches the Arc::into_raw
    // in create_vnode_source.
    unsafe {
        Arc::decrement_strong_count(context as *const WatchState);
    }
}

pub struct FsWatcher {
    tx: smol::channel::Sender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    watches: Mutex<HashMap<PathBuf, Arc<WatchState>>>,
}

impl FsWatcher {
    pub fn new(
        tx: smol::channel::Sender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        FsWatcher {
            tx,
            pending_path_events,
            watches: Default::default(),
        }
    }
}

impl Watcher for FsWatcher {
    fn add(&self, path: &Path) -> anyhow::Result<()> {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if self.watches.lock().contains_key(&canonical) {
            return Ok(());
        }

        let state = Arc::new(WatchState {
            path: canonical.clone(),
            tx: self.tx.clone(),
            pending_path_events: self.pending_path_events.clone(),
            active: Mutex::new(None),
        });

        let active = create_vnode_source(&state)?;
        *state.active.lock() = Some(active);

        self.watches.lock().insert(canonical, state);
        Ok(())
    }

    fn remove(&self, path: &Path) -> anyhow::Result<()> {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        // Dropping the Arc<WatchState> will eventually drop the ActiveSource,
        // which cancels the dispatch source and closes the fd.
        self.watches.lock().remove(&canonical);
        Ok(())
    }
}
