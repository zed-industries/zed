#![allow(missing_docs)]
//! Watcher implementation for Windows' directory management APIs
//!
//! For more information see the [ReadDirectoryChangesW reference][ref].
//!
//! [ref]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa363950(v=vs.85).aspx

use crate::paths::{absolute_path, WatchPath};
use crate::{bounded, unbounded, BoundSender, Config, Receiver, Sender};
use crate::{event::*, WatcherKind};
use crate::{Error, EventHandler, RecursiveMode, Result, Watcher, WindowsPathSeparatorStyle};
use std::alloc;
use std::collections::HashMap;
use std::ffi::OsString;
use std::os::raw::c_void;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::ptr;
use std::slice;
use std::sync::{Arc, Mutex};
use std::thread;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_ACCESS_DENIED, ERROR_OPERATION_ABORTED, ERROR_SUCCESS, HANDLE,
    INVALID_HANDLE_VALUE, WAIT_OBJECT_0,
};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, ReadDirectoryChangesW, FILE_ACTION_ADDED, FILE_ACTION_MODIFIED,
    FILE_ACTION_REMOVED, FILE_ACTION_RENAMED_NEW_NAME, FILE_ACTION_RENAMED_OLD_NAME,
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OVERLAPPED, FILE_LIST_DIRECTORY,
    FILE_NOTIFY_CHANGE_ATTRIBUTES, FILE_NOTIFY_CHANGE_CREATION, FILE_NOTIFY_CHANGE_DIR_NAME,
    FILE_NOTIFY_CHANGE_FILE_NAME, FILE_NOTIFY_CHANGE_LAST_WRITE, FILE_NOTIFY_CHANGE_SECURITY,
    FILE_NOTIFY_CHANGE_SIZE, FILE_NOTIFY_INFORMATION, FILE_SHARE_DELETE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows_sys::Win32::System::Threading::{
    CreateSemaphoreW, ReleaseSemaphore, WaitForSingleObjectEx, INFINITE,
};
use windows_sys::Win32::System::IO::{CancelIo, OVERLAPPED};

const BUF_SIZE: u32 = 16384;

#[derive(Clone, Copy)]
enum SeparatorStyle {
    Slash,
    Backslash,
}

impl SeparatorStyle {
    fn resolve(configured_style: WindowsPathSeparatorStyle, path: &Path) -> Self {
        match configured_style {
            WindowsPathSeparatorStyle::Auto => Self::from_path(path),
            WindowsPathSeparatorStyle::Slash => Self::Slash,
            WindowsPathSeparatorStyle::Backslash => Self::Backslash,
        }
    }

    fn from_path(path: &Path) -> Self {
        let mut has_forward_slash = false;
        let mut has_backslash = false;

        for ch in path.as_os_str().encode_wide() {
            if ch == '/' as u16 {
                has_forward_slash = true;
            } else if ch == '\\' as u16 {
                has_backslash = true;
            }

            if has_forward_slash && has_backslash {
                return Self::Backslash;
            }
        }

        if has_forward_slash {
            Self::Slash
        } else {
            Self::Backslash
        }
    }

    fn as_u16(self) -> u16 {
        match self {
            Self::Slash => '/' as u16,
            Self::Backslash => '\\' as u16,
        }
    }
}

fn trim_leading_separators(path: &[u16]) -> &[u16] {
    let mut start = 0;
    while start < path.len() && (path[start] == '/' as u16 || path[start] == '\\' as u16) {
        start += 1;
    }
    &path[start..]
}

fn windows_namespace_prefix_len(path: &[u16]) -> usize {
    let is_separator = |ch: u16| ch == '/' as u16 || ch == '\\' as u16;

    if path.len() >= 4
        && is_separator(path[0])
        && is_separator(path[1])
        && (path[2] == '?' as u16 || path[2] == '.' as u16)
        && is_separator(path[3])
    {
        4
    } else {
        0
    }
}

fn normalize_path_separators(path: PathBuf, separator_style: SeparatorStyle) -> PathBuf {
    let separator = separator_style.as_u16();
    let mut encoded_path: Vec<u16> = path.as_os_str().encode_wide().collect();
    let prefix_len = windows_namespace_prefix_len(&encoded_path);

    for ch in encoded_path.iter_mut().skip(prefix_len) {
        if *ch == '/' as u16 || *ch == '\\' as u16 {
            *ch = separator;
        }
    }

    PathBuf::from(OsString::from_wide(&encoded_path))
}

#[derive(Clone)]
struct ReadData {
    watch_path: PathBuf,   // key used in the server watch map
    dir: PathBuf,          // directory that is being watched
    reported_dir: PathBuf, // directory prefix used in emitted event paths
    file: Option<PathBuf>, // if a file is being watched, this is its reported path
    complete_sem: HANDLE,
    is_recursive: bool,
    separator_style: SeparatorStyle,
}

struct ReadDirectoryRequest {
    event_handler: Arc<Mutex<dyn EventHandler>>,
    event_kinds: EventKindMask,
    buffer: [u8; BUF_SIZE as usize],
    handle: HANDLE,
    data: ReadData,
    action_tx: Sender<Action>,
}

impl ReadDirectoryRequest {
    fn unwatch(&self) {
        let _ = self
            .action_tx
            .send(Action::Unwatch(self.data.watch_path.clone()));
    }
}

enum Action {
    Watch(WatchPath, RecursiveMode, SeparatorStyle),
    // Internal self-unwatch from the completion callback.
    Unwatch(PathBuf),
    // Public `Watcher::unwatch` path. This variant must ack only after `remove_watch` finishes so
    // the caller does not observe events after `unwatch()` returns.
    UnwatchAck(PathBuf),
    GetWatchedPaths(Sender<Vec<(PathBuf, RecursiveMode)>>),
    Stop,
    Configure(Config, BoundSender<Result<bool>>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MetaEvent {
    SingleWatchComplete,
    WatcherAwakened,
}

struct WatchState {
    dir_handle: HANDLE,
    complete_sem: HANDLE,
    recursive_mode: RecursiveMode,
    reported_path: PathBuf,
}

struct ReadDirectoryChangesServer {
    tx: Sender<Action>,
    rx: Receiver<Action>,
    event_handler: Arc<Mutex<dyn EventHandler>>,
    event_kinds: EventKindMask,
    meta_tx: Sender<MetaEvent>,
    cmd_tx: Sender<Result<PathBuf>>,
    watches: HashMap<PathBuf, WatchState>,
    wakeup_sem: HANDLE,
}

impl ReadDirectoryChangesServer {
    fn start(
        event_handler: Arc<Mutex<dyn EventHandler>>,
        event_kinds: EventKindMask,
        meta_tx: Sender<MetaEvent>,
        cmd_tx: Sender<Result<PathBuf>>,
        wakeup_sem: HANDLE,
    ) -> Sender<Action> {
        let (action_tx, action_rx) = unbounded();
        // it is, in fact, ok to send the semaphore across threads
        let sem_temp = wakeup_sem as u64;
        let _ = thread::Builder::new()
            .name("notify-rs windows loop".to_string())
            .spawn({
                let tx = action_tx.clone();
                move || {
                    let wakeup_sem = sem_temp as HANDLE;
                    let server = ReadDirectoryChangesServer {
                        tx,
                        rx: action_rx,
                        event_handler,
                        event_kinds,
                        meta_tx,
                        cmd_tx,
                        watches: HashMap::new(),
                        wakeup_sem,
                    };
                    server.run();
                }
            });
        action_tx
    }

    fn run(mut self) {
        loop {
            // process all available actions first
            let mut stopped = false;

            while let Ok(action) = self.rx.try_recv() {
                match action {
                    Action::Watch(path, recursive_mode, separator_style) => {
                        let res =
                            self.add_watch(path, recursive_mode.is_recursive(), separator_style);
                        let _ = self.cmd_tx.send(res);
                    }
                    Action::Unwatch(path) => self.remove_watch(path),
                    Action::UnwatchAck(path) => {
                        self.remove_watch(path.clone());
                        let _ = self.cmd_tx.send(Ok(path));
                    }
                    Action::GetWatchedPaths(tx) => {
                        let _ = tx.send(
                            self.watches
                                .iter()
                                .map(|(_path, state)| {
                                    (state.reported_path.clone(), state.recursive_mode)
                                })
                                .collect(),
                        );
                    }
                    Action::Stop => {
                        stopped = true;
                        for ws in self.watches.values() {
                            stop_watch(ws, &self.meta_tx);
                        }
                        break;
                    }
                    Action::Configure(config, tx) => {
                        self.configure_raw_mode(config, tx);
                    }
                }
            }

            if stopped {
                break;
            }

            unsafe {
                // wait with alertable flag so that the completion routine fires
                let waitres = WaitForSingleObjectEx(self.wakeup_sem, 100, 1);
                if waitres == WAIT_OBJECT_0 {
                    let _ = self.meta_tx.send(MetaEvent::WatcherAwakened);
                }
            }
        }

        // we have to clean this up, since the watcher may be long gone
        unsafe {
            CloseHandle(self.wakeup_sem);
        }
    }

    fn add_watch(
        &mut self,
        path: WatchPath,
        is_recursive: bool,
        separator_style: SeparatorStyle,
    ) -> Result<PathBuf> {
        // path must exist and be either a file or directory
        if !path.absolute.is_dir() && !path.absolute.is_file() {
            return Err(
                Error::generic("Input watch path is neither a file nor a directory.")
                    .add_path(path.requested),
            );
        }

        let (watching_file, dir_target) = {
            if path.absolute.is_dir() {
                (false, path.absolute.clone())
            } else {
                // emulate file watching by watching the parent directory
                (true, path.absolute.parent().unwrap().to_path_buf())
            }
        };
        let reported_dir = if watching_file {
            path.requested
                .parent()
                .map_or_else(PathBuf::new, Path::to_path_buf)
        } else {
            path.requested.clone()
        };

        let encoded_path: Vec<u16> = dir_target
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        let handle;
        unsafe {
            handle = CreateFileW(
                encoded_path.as_ptr(),
                FILE_LIST_DIRECTORY,
                FILE_SHARE_READ | FILE_SHARE_DELETE | FILE_SHARE_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OVERLAPPED,
                ptr::null_mut(),
            );

            if handle == INVALID_HANDLE_VALUE {
                return Err(if watching_file {
                    Error::generic(
                        "You attempted to watch a single file, but parent \
                         directory could not be opened.",
                    )
                    .add_path(path.requested)
                } else {
                    // TODO: Call GetLastError for better error info?
                    Error::path_not_found().add_path(path.requested)
                });
            }
        }
        let wf = if watching_file {
            Some(normalize_path_separators(
                path.requested.clone(),
                separator_style,
            ))
        } else {
            None
        };
        let watched_path = path.absolute.clone();
        // every watcher gets its own semaphore to signal completion
        let semaphore = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };
        if semaphore.is_null() || semaphore == INVALID_HANDLE_VALUE {
            unsafe {
                CloseHandle(handle);
            }
            return Err(
                Error::generic("Failed to create semaphore for watch.").add_path(path.requested)
            );
        }
        let rd = ReadData {
            watch_path: watched_path.clone(),
            dir: dir_target,
            reported_dir,
            file: wf,
            complete_sem: semaphore,
            is_recursive,
            separator_style,
        };
        let ws = WatchState {
            dir_handle: handle,
            complete_sem: semaphore,
            recursive_mode: if is_recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            },
            reported_path: path.requested,
        };
        if let Err(err) = start_read(
            &rd,
            self.event_handler.clone(),
            self.event_kinds,
            handle,
            self.tx.clone(),
        ) {
            unsafe {
                CloseHandle(handle);
                CloseHandle(semaphore);
            }
            return Err(err);
        }
        if let Some(ws) = self.watches.remove(&watched_path) {
            stop_watch(&ws, &self.meta_tx);
        }
        self.watches.insert(watched_path.clone(), ws);
        Ok(watched_path)
    }

    fn remove_watch(&mut self, path: PathBuf) {
        if let Some(ws) = self.watches.remove(&path) {
            stop_watch(&ws, &self.meta_tx);
        }
    }

    fn configure_raw_mode(&mut self, _config: Config, tx: BoundSender<Result<bool>>) {
        tx.send(Ok(false))
            .expect("configuration channel disconnect");
    }
}

fn stop_watch(ws: &WatchState, meta_tx: &Sender<MetaEvent>) {
    unsafe {
        let cio = CancelIo(ws.dir_handle);
        let ch = CloseHandle(ws.dir_handle);
        // have to wait for it, otherwise we leak the memory allocated for there read request
        if cio != 0 && ch != 0 {
            while WaitForSingleObjectEx(ws.complete_sem, INFINITE, 1) != WAIT_OBJECT_0 {
                // drain the apc queue, fix for https://github.com/notify-rs/notify/issues/287#issuecomment-801465550
            }
        }
        CloseHandle(ws.complete_sem);
    }
    let _ = meta_tx.send(MetaEvent::SingleWatchComplete);
}

fn start_read(
    rd: &ReadData,
    event_handler: Arc<Mutex<dyn EventHandler>>,
    event_kinds: EventKindMask,
    handle: HANDLE,
    action_tx: Sender<Action>,
) -> Result<()> {
    let request = Box::new(ReadDirectoryRequest {
        event_handler,
        event_kinds,
        handle,
        buffer: [0u8; BUF_SIZE as usize],
        data: rd.clone(),
        action_tx,
    });

    let flags = FILE_NOTIFY_CHANGE_FILE_NAME
        | FILE_NOTIFY_CHANGE_DIR_NAME
        | FILE_NOTIFY_CHANGE_ATTRIBUTES
        | FILE_NOTIFY_CHANGE_SIZE
        | FILE_NOTIFY_CHANGE_LAST_WRITE
        | FILE_NOTIFY_CHANGE_CREATION
        | FILE_NOTIFY_CHANGE_SECURITY;

    let monitor_subdir = if request.data.file.is_none() && request.data.is_recursive {
        1
    } else {
        0
    };

    unsafe {
        let overlapped = alloc::alloc_zeroed(alloc::Layout::new::<OVERLAPPED>()) as *mut OVERLAPPED;
        // When using callback based async requests, we are allowed to use the hEvent member
        // for our own purposes

        let request = Box::leak(request);
        (*overlapped).hEvent = request as *mut _ as _;

        // This is using an asynchronous call with a completion routine for receiving notifications
        // An I/O completion port would probably be more performant
        let ret = ReadDirectoryChangesW(
            handle,
            request.buffer.as_mut_ptr() as *mut c_void,
            BUF_SIZE,
            monitor_subdir,
            flags,
            &mut 0u32 as *mut u32, // not used for async reqs
            overlapped,
            Some(handle_event),
        );

        if ret == 0 {
            let err = std::io::Error::last_os_error();
            // error reading. retransmute request memory to allow drop.
            // Because of the error, ownership of the `overlapped` alloc was not passed
            // over to `ReadDirectoryChangesW`.
            // So we can claim ownership back.
            let _overlapped = Box::from_raw(overlapped);
            let request = Box::from_raw(request);
            let path = request
                .data
                .file
                .clone()
                .unwrap_or_else(|| request.data.reported_dir.clone());
            ReleaseSemaphore(request.data.complete_sem, 1, ptr::null_mut());
            return Err(Error::io(err).add_path(path));
        }
    }

    Ok(())
}

unsafe extern "system" fn handle_event(
    error_code: u32,
    _bytes_written: u32,
    overlapped: *mut OVERLAPPED,
) {
    let overlapped: Box<OVERLAPPED> = Box::from_raw(overlapped);
    let request: Box<ReadDirectoryRequest> = Box::from_raw(overlapped.hEvent as *mut _);

    fn emit_event(event_handler: &Mutex<dyn EventHandler>, res: Result<Event>) {
        if let Ok(mut guard) = event_handler.lock() {
            let f: &mut dyn EventHandler = &mut *guard;
            f.handle_event(res);
        }
    }

    match error_code {
        ERROR_OPERATION_ABORTED => {
            // received when dir is unwatched or watcher is shutdown; return and let overlapped/request get drop-cleaned
            ReleaseSemaphore(request.data.complete_sem, 1, ptr::null_mut());
            return;
        }
        ERROR_ACCESS_DENIED => {
            // ReadDirectoryChangesW returns ERROR_ACCESS_DENIED both when the handle
            // has been invalidated (usually because the watched dir was deleted) and
            // when access has been revoked; use successful dir absence to tell which.
            // For directory watches, emit a Remove event so consumers learn the
            // watched path is gone, matching inotify IN_DELETE_SELF (#540) and
            // FSEvents ROOT_CHANGED+ITEM_REMOVED. File watches are excluded because
            // FILE_ACTION_REMOVED already fires for the file's parent.
            if matches!(request.data.dir.try_exists(), Ok(false)) {
                if request.data.file.is_none() {
                    const KIND: EventKind = EventKind::Remove(RemoveKind::Folder);
                    if request.event_kinds.matches(&KIND) {
                        let path = normalize_path_separators(
                            request.data.reported_dir.clone(),
                            request.data.separator_style,
                        );
                        let event = Event::new(KIND).add_path(path);
                        emit_event(&request.event_handler, Ok(event));
                    }
                }
                request.unwatch();
                ReleaseSemaphore(request.data.complete_sem, 1, ptr::null_mut());
                return;
            }
        }
        ERROR_SUCCESS => {
            // Success, continue to handle the event
        }
        _ => {
            // Some unidentified error occurred, log and unwatch the directory, then return.
            log::error!(
                "unknown error in ReadDirectoryChangesW for directory {}: {}",
                request.data.dir.display(),
                error_code
            );
            request.unwatch();
            ReleaseSemaphore(request.data.complete_sem, 1, ptr::null_mut());
            return;
        }
    }

    // Get the next request queued up as soon as possible
    let rearm_error = start_read(
        &request.data,
        request.event_handler.clone(),
        request.event_kinds,
        request.handle,
        request.action_tx.clone(),
    )
    .err();

    // The FILE_NOTIFY_INFORMATION struct has a variable length due to the variable length
    // string as its last member. Each struct contains an offset for getting the next entry in
    // the buffer.
    let mut cur_offset: *const u8 = request.buffer.as_ptr();
    // In Wine, FILE_NOTIFY_INFORMATION structs are packed placed in the buffer;
    // they are aligned to 16bit (WCHAR) boundary instead of 32bit required by FILE_NOTIFY_INFORMATION.
    // Hence, we need to use `read_unaligned` here to avoid UB.
    let mut cur_entry = ptr::read_unaligned(cur_offset as *const FILE_NOTIFY_INFORMATION);
    loop {
        // filename length is size in bytes, so / 2
        let len = cur_entry.FileNameLength as usize / 2;
        let encoded_path: &[u16] = slice::from_raw_parts(
            cur_offset.offset(std::mem::offset_of!(FILE_NOTIFY_INFORMATION, FileName) as isize)
                as _,
            len,
        );
        // prepend root to get a full path
        let relative_path =
            PathBuf::from(OsString::from_wide(trim_leading_separators(encoded_path)));
        let absolute_path = request.data.dir.join(&relative_path);
        let path = normalize_path_separators(
            request.data.reported_dir.join(relative_path),
            request.data.separator_style,
        );

        // if we are watching a single file, ignore the event unless the path is exactly
        // the watched file
        let skip = match request.data.file {
            None => false,
            Some(ref watch_path) => *watch_path != path,
        };

        if !skip {
            log::trace!(
                "Event: path = `{}`, action = {:?}",
                path.display(),
                cur_entry.Action
            );

            let newe = Event::new(EventKind::Any).add_path(path);

            // Filter events based on EventKindMask
            let event_kinds = request.event_kinds;
            let event_handler = |res: Result<Event>| {
                match &res {
                    // Errors always pass through
                    Err(_) => emit_event(&request.event_handler, res),
                    // OK events only if they match the mask
                    Ok(e) if event_kinds.matches(&e.kind) => {
                        emit_event(&request.event_handler, res)
                    }
                    // Event filtered out
                    Ok(_) => {}
                }
            };

            if cur_entry.Action == FILE_ACTION_RENAMED_OLD_NAME {
                let mode = RenameMode::From;
                let kind = ModifyKind::Name(mode);
                let kind = EventKind::Modify(kind);
                let ev = newe.set_kind(kind);
                event_handler(Ok(ev))
            } else {
                match cur_entry.Action {
                    FILE_ACTION_RENAMED_NEW_NAME => {
                        let kind = EventKind::Modify(ModifyKind::Name(RenameMode::To));
                        let ev = newe.set_kind(kind);
                        event_handler(Ok(ev));
                    }
                    FILE_ACTION_ADDED => {
                        let kind =
                            std::fs::metadata(&absolute_path).map_or(CreateKind::Any, |metadata| {
                                if metadata.is_dir() {
                                    CreateKind::Folder
                                } else if metadata.is_file() {
                                    CreateKind::File
                                } else {
                                    CreateKind::Any
                                }
                            });
                        let kind = EventKind::Create(kind);
                        let ev = newe.set_kind(kind);
                        event_handler(Ok(ev));
                    }
                    FILE_ACTION_REMOVED => {
                        let kind = EventKind::Remove(RemoveKind::Any);
                        let ev = newe.set_kind(kind);
                        event_handler(Ok(ev));
                    }
                    FILE_ACTION_MODIFIED => {
                        let kind = EventKind::Modify(ModifyKind::Any);
                        let ev = newe.set_kind(kind);
                        event_handler(Ok(ev));
                    }
                    _ => (),
                };
            }
        }

        if cur_entry.NextEntryOffset == 0 {
            break;
        }
        cur_offset = cur_offset.offset(cur_entry.NextEntryOffset as isize);
        cur_entry = ptr::read_unaligned(cur_offset as *const FILE_NOTIFY_INFORMATION);
    }

    if let Some(err) = rearm_error {
        emit_event(&request.event_handler, Err(err));
        request.unwatch();
    }
}

/// Watcher implementation based on ReadDirectoryChanges
#[derive(Debug)]
pub struct ReadDirectoryChangesWatcher {
    tx: Sender<Action>,
    cmd_rx: Receiver<Result<PathBuf>>,
    wakeup_sem: HANDLE,
    windows_path_separator_style: WindowsPathSeparatorStyle,
}

impl ReadDirectoryChangesWatcher {
    pub fn create(
        event_handler: Arc<Mutex<dyn EventHandler>>,
        event_kinds: EventKindMask,
        meta_tx: Sender<MetaEvent>,
    ) -> Result<ReadDirectoryChangesWatcher> {
        Self::create_inner(
            event_handler,
            event_kinds,
            WindowsPathSeparatorStyle::Auto,
            meta_tx,
        )
    }

    fn create_inner(
        event_handler: Arc<Mutex<dyn EventHandler>>,
        event_kinds: EventKindMask,
        windows_path_separator_style: WindowsPathSeparatorStyle,
        meta_tx: Sender<MetaEvent>,
    ) -> Result<ReadDirectoryChangesWatcher> {
        let (cmd_tx, cmd_rx) = unbounded();

        let wakeup_sem = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };
        if wakeup_sem.is_null() || wakeup_sem == INVALID_HANDLE_VALUE {
            return Err(Error::generic("Failed to create wakeup semaphore."));
        }

        let action_tx = ReadDirectoryChangesServer::start(
            event_handler,
            event_kinds,
            meta_tx,
            cmd_tx,
            wakeup_sem,
        );

        Ok(ReadDirectoryChangesWatcher {
            tx: action_tx,
            cmd_rx,
            wakeup_sem,
            windows_path_separator_style,
        })
    }

    fn wakeup_server(&self) {
        // breaks the server out of its wait state.  right now this is really just an optimization,
        // so that if you add a watch you don't block for 100ms in watch() while the
        // server sleeps.
        unsafe {
            ReleaseSemaphore(self.wakeup_sem, 1, ptr::null_mut());
        }
    }

    fn send_action_require_ack(&mut self, action: Action, pb: &PathBuf) -> Result<()> {
        self.tx
            .send(action)
            .map_err(|_| Error::generic("Error sending to internal channel"))?;

        // wake 'em up, we don't want to wait around for the ack
        self.wakeup_server();

        let ack_pb = self
            .cmd_rx
            .recv()
            .map_err(|_| Error::generic("Error receiving from command channel"))??;

        if pb.as_path() != ack_pb.as_path() {
            Err(Error::generic(&format!(
                "Expected ack for {:?} but got \
                 ack for {:?}",
                pb, ack_pb
            )))
        } else {
            Ok(())
        }
    }

    fn watch_inner(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        let separator_style = SeparatorStyle::resolve(self.windows_path_separator_style, path);
        let pb = WatchPath::new(path)?;
        // path must exist and be either a file or directory
        if !pb.absolute.is_dir() && !pb.absolute.is_file() {
            return Err(Error::generic(
                "Input watch path is neither a file nor a directory.",
            ));
        }
        self.send_action_require_ack(
            Action::Watch(pb.clone(), recursive_mode, separator_style),
            &pb.absolute,
        )
    }

    fn unwatch_inner(&mut self, path: &Path) -> Result<()> {
        let pb = absolute_path(path)?;
        self.send_action_require_ack(Action::UnwatchAck(pb.clone()), &pb)
    }

    fn watched_paths_inner(&self) -> Result<Vec<(PathBuf, RecursiveMode)>> {
        let (tx, rx) = unbounded();
        self.tx
            .send(Action::GetWatchedPaths(tx))
            .map_err(|_| Error::generic("Error sending to internal channel"))?;
        self.wakeup_server();
        rx.recv().map_err(Error::from)
    }
}

impl Watcher for ReadDirectoryChangesWatcher {
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self> {
        // create dummy channel for meta event
        // TODO: determine the original purpose of this - can we remove it?
        let (meta_tx, _) = unbounded();
        let event_handler = Arc::new(Mutex::new(event_handler));
        Self::create_inner(
            event_handler,
            config.event_kinds(),
            config.windows_path_separator_style(),
            meta_tx,
        )
    }

    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        self.watch_inner(path, recursive_mode)
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.unwatch_inner(path)
    }

    fn configure(&mut self, config: Config) -> Result<bool> {
        let (tx, rx) = bounded(1);
        self.tx.send(Action::Configure(config, tx))?;
        rx.recv()?
    }

    fn watched_paths(&self) -> Result<Vec<(PathBuf, RecursiveMode)>> {
        self.watched_paths_inner()
    }

    fn kind() -> crate::WatcherKind {
        WatcherKind::ReadDirectoryChangesWatcher
    }
}

impl Drop for ReadDirectoryChangesWatcher {
    fn drop(&mut self) {
        let _ = self.tx.send(Action::Stop);
        // better wake it up
        self.wakeup_server();
    }
}

// `ReadDirectoryChangesWatcher` is not Send/Sync because of the semaphore Handle.
// As said elsewhere it's perfectly safe to send it across threads.
unsafe impl Send for ReadDirectoryChangesWatcher {}
// Because all public methods are `&mut self` it's also perfectly safe to share references.
unsafe impl Sync for ReadDirectoryChangesWatcher {}

#[cfg(test)]
pub mod tests {
    use std::env;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;
    use tempfile::{tempdir, tempdir_in};

    use super::{normalize_path_separators, trim_leading_separators, SeparatorStyle};
    use crate::{
        test::*, ReadDirectoryChangesWatcher, RecursiveMode, Watcher, WindowsPathSeparatorStyle,
    };

    use std::time::Duration;

    fn watcher() -> (TestWatcher<ReadDirectoryChangesWatcher>, Receiver) {
        channel()
    }

    #[test]
    fn trash_dir() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let child_dir = dir.path().join("child");
        std::fs::create_dir(&child_dir)?;

        let mut watcher = crate::recommended_watcher(|_| {
            // Do something with the event
        })?;
        watcher.watch(&child_dir, RecursiveMode::NonRecursive)?;

        trash::delete(&child_dir)?;

        watcher.watch(dir.path(), RecursiveMode::NonRecursive)?;

        Ok(())
    }

    #[test]
    fn watcher_is_send_and_sync() {
        fn check<T: Send + Sync>() {}
        check::<ReadDirectoryChangesWatcher>();
    }

    #[test]
    fn posix_watch_path_uses_slash_separator_style() {
        let style = SeparatorStyle::from_path(Path::new("G:/Feature film/"));
        assert!(matches!(style, SeparatorStyle::Slash));
    }

    #[test]
    fn explicit_separator_style_overrides_watch_path_style() {
        let style = SeparatorStyle::resolve(
            WindowsPathSeparatorStyle::Backslash,
            Path::new("G:/Feature film/"),
        );
        assert!(matches!(style, SeparatorStyle::Backslash));
    }

    #[test]
    fn trim_leading_separators_removes_root_separators() {
        let input = [
            '\\' as u16,
            '/' as u16,
            's' as u16,
            'u' as u16,
            'b' as u16,
            '\\' as u16,
            'f' as u16,
            'i' as u16,
            'l' as u16,
            'e' as u16,
        ];
        let trimmed = trim_leading_separators(&input);
        assert_eq!(
            trimmed,
            [
                's' as u16,
                'u' as u16,
                'b' as u16,
                '\\' as u16,
                'f' as u16,
                'i' as u16,
                'l' as u16,
                'e' as u16,
            ]
        );
    }

    #[test]
    fn normalize_joined_event_path_for_posix_watch_path() {
        let dir = PathBuf::from("G:/Feature");
        let raw_event_name: Vec<u16> = "\\22.mp4".encode_utf16().collect();
        let relative = PathBuf::from(OsString::from_wide(trim_leading_separators(
            &raw_event_name,
        )));
        let path = normalize_path_separators(dir.join(relative), SeparatorStyle::Slash);

        assert_eq!(path, PathBuf::from("G:/Feature/22.mp4"));
    }

    #[test]
    fn normalize_path_separators_keeps_windows_namespace_prefix() {
        let path = PathBuf::from(r"\\?\C:\very\long\file");
        let normalized = normalize_path_separators(path, SeparatorStyle::Slash);
        assert_eq!(normalized, PathBuf::from(r"\\?\C:/very/long/file"));
    }

    #[test]
    fn access_denied_existence_error_does_not_emit_remove() {
        use crate::event::{EventKind, RemoveKind};
        use std::ptr;
        use windows_sys::Win32::Foundation::{
            CloseHandle, ERROR_ACCESS_DENIED, INVALID_HANDLE_VALUE,
        };
        use windows_sys::Win32::System::Threading::CreateSemaphoreW;
        use windows_sys::Win32::System::IO::OVERLAPPED;

        let invalid_path = PathBuf::from(OsString::from_wide(&[0]));
        assert!(invalid_path.try_exists().is_err());

        let complete_sem = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };
        assert!(!complete_sem.is_null());
        assert_ne!(complete_sem, INVALID_HANDLE_VALUE);

        let (event_tx, event_rx) = mpsc::channel();
        let (action_tx, _action_rx) = crate::unbounded();
        let event_handler: Arc<Mutex<dyn crate::EventHandler>> = Arc::new(Mutex::new(event_tx));
        let request = Box::new(super::ReadDirectoryRequest {
            event_handler,
            event_kinds: crate::EventKindMask::ALL,
            buffer: [0u8; super::BUF_SIZE as usize],
            handle: INVALID_HANDLE_VALUE,
            data: super::ReadData {
                watch_path: invalid_path.clone(),
                dir: invalid_path.clone(),
                reported_dir: invalid_path,
                file: None,
                complete_sem,
                is_recursive: false,
                separator_style: SeparatorStyle::Backslash,
            },
            action_tx,
        });
        let mut overlapped = Box::new(unsafe { std::mem::zeroed::<OVERLAPPED>() });
        overlapped.hEvent = Box::into_raw(request) as _;

        unsafe {
            super::handle_event(ERROR_ACCESS_DENIED, 0, Box::into_raw(overlapped));
            CloseHandle(complete_sem);
        }

        let events = event_rx.try_iter().collect::<Vec<_>>();
        assert!(
            !events.iter().any(|res| matches!(
                res,
                Ok(event) if event.kind == EventKind::Remove(RemoveKind::Folder)
            )),
            "unexpected remove event: {events:#?}"
        );
    }

    #[test]
    fn request_unwatch_uses_watch_key_not_read_directory() {
        use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;

        let watch_path = PathBuf::from(r"C:\tmp\file.txt");
        let read_dir = PathBuf::from(r"C:\tmp");
        let (event_tx, _event_rx) = mpsc::channel::<crate::Result<crate::Event>>();
        let (action_tx, action_rx) = crate::unbounded();
        let event_handler: Arc<Mutex<dyn crate::EventHandler>> = Arc::new(Mutex::new(event_tx));

        let request = super::ReadDirectoryRequest {
            event_handler,
            event_kinds: crate::EventKindMask::ALL,
            buffer: [0u8; super::BUF_SIZE as usize],
            handle: INVALID_HANDLE_VALUE,
            data: super::ReadData {
                watch_path: watch_path.clone(),
                dir: read_dir,
                reported_dir: PathBuf::from(r"C:\tmp"),
                file: Some(watch_path.clone()),
                complete_sem: INVALID_HANDLE_VALUE,
                is_recursive: false,
                separator_style: SeparatorStyle::Backslash,
            },
            action_tx,
        };

        request.unwatch();

        match action_rx.recv().expect("receive action") {
            super::Action::Unwatch(path) => assert_eq!(path, watch_path),
            _ => panic!("unexpected action"),
        }
    }

    #[test]
    fn auto_separator_style_keeps_relative_slash_watch_style(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let cwd = env::current_dir()?;
        let root = tempdir_in(&cwd)?;
        let watched_dir = root.path().join("sub").join("dir");
        std::fs::create_dir_all(&watched_dir)?;
        let watched_file = watched_dir.join("entry");
        let relative_watch_path = watched_dir.strip_prefix(&cwd)?;
        let slash_watch_path =
            PathBuf::from(relative_watch_path.to_string_lossy().replace('\\', "/"));

        let (mut watcher, mut rx) = channel_with_config::<ReadDirectoryChangesWatcher>(
            ChannelConfig::default().with_watcher_config(
                crate::Config::default()
                    .with_windows_path_separator_style(WindowsPathSeparatorStyle::Auto),
            ),
        );
        watcher.watch_nonrecursively(&slash_watch_path);

        std::fs::File::create_new(&watched_file)?;

        let expected_path =
            normalize_path_separators(slash_watch_path.join("entry"), SeparatorStyle::Slash);
        rx.wait_unordered([expected(&expected_path).create_file()]);

        Ok(())
    }

    #[test]
    fn single_file_filter_matches_with_explicit_separator_style() {
        let watched_file = PathBuf::from(r"G:\Feature\a.txt");
        let watched_dir = PathBuf::from(r"G:\Feature");

        let separator_style =
            SeparatorStyle::resolve(WindowsPathSeparatorStyle::Slash, &watched_file);
        let normalized_watch_path =
            normalize_path_separators(watched_file.clone(), separator_style);

        let raw_event_name: Vec<u16> = "\\a.txt".encode_utf16().collect();
        let relative_path = PathBuf::from(OsString::from_wide(trim_leading_separators(
            &raw_event_name,
        )));
        let normalized_event_path =
            normalize_path_separators(watched_dir.join(relative_path), separator_style);

        assert_eq!(normalized_watch_path, normalized_event_path);
        assert_eq!(normalized_event_path, PathBuf::from("G:/Feature/a.txt"));
    }

    #[test]
    fn single_file_filter_matches_bare_relative_file() {
        let watched_file = PathBuf::from("a.txt");
        let reported_dir = watched_file
            .parent()
            .map_or_else(PathBuf::new, Path::to_path_buf);
        let separator_style =
            SeparatorStyle::resolve(WindowsPathSeparatorStyle::Auto, &watched_file);
        let normalized_watch_path =
            normalize_path_separators(watched_file.clone(), separator_style);

        let raw_event_name: Vec<u16> = "a.txt".encode_utf16().collect();
        let relative_path = PathBuf::from(OsString::from_wide(trim_leading_separators(
            &raw_event_name,
        )));
        let normalized_event_path =
            normalize_path_separators(reported_dir.join(relative_path), separator_style);

        assert_eq!(normalized_watch_path, normalized_event_path);
        assert_eq!(normalized_event_path, watched_file);
    }

    #[test]
    fn create_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        watcher.watch_recursively(&tmpdir);

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        rx.wait_ordered_exact([expected(&path).create_file()])
            .ensure_no_tail();
    }

    #[test]
    fn recursive_temp_dir_write_reports_created_file_kind() {
        let tmpdir = tempdir().expect("create tempdir");
        let (mut watcher, mut rx) = watcher();
        watcher.watch_recursively(tmpdir.path());

        let path = tmpdir.path().join("new.txt");
        std::fs::write(&path, b"hello").expect("write");

        rx.wait_ordered([expected(&path).create_file()]);
    }

    #[test]
    fn write_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_recursively(&tmpdir);
        std::fs::write(&path, b"123").expect("write");

        rx.wait_ordered_exact([expected(&path).modify_any().multiple()])
            .ensure_no_tail();
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

        rx.wait_ordered_exact([expected(&path).modify_any()])
            .ensure_no_tail();
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

        rx.wait_ordered_exact([
            expected(&path).rename_from(),
            expected(&new_path).rename_to(),
        ])
        .ensure_no_tail();
    }

    #[test]
    fn delete_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&tmpdir);

        std::fs::remove_file(&file).expect("remove");

        rx.wait_ordered_exact([expected(&file).remove_any()])
            .ensure_no_tail();
    }

    #[test]
    fn delete_self_file() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();
        let file = tmpdir.path().join("file");
        std::fs::write(&file, "").expect("write");

        watcher.watch_nonrecursively(&file);

        std::fs::remove_file(&file).expect("remove");

        rx.wait_ordered_exact([expected(&file).remove_any()]);
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

        rx.wait_ordered_exact([
            expected(&overwriting_file).create(),
            expected(&overwriting_file).modify_any().multiple(),
            expected(&overwritten_file).remove_any(),
            expected(&overwriting_file).rename_from(),
            expected(&overwritten_file).rename_to(),
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

        rx.wait_ordered_exact([expected(&path).create_folder()])
            .ensure_no_tail();
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

        rx.wait_ordered_exact([expected(&path).modify_any()])
            .ensure_no_tail();
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

        rx.wait_ordered_exact([
            expected(&path).rename_from(),
            expected(&new_path).rename_to(),
        ])
        .ensure_no_tail();
    }

    #[test]
    fn delete_dir() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::create_dir(&path).expect("create_dir");

        watcher.watch_recursively(&tmpdir);
        std::fs::remove_dir(&path).expect("remove");

        rx.wait_ordered_exact([expected(&path).remove_any()])
            .ensure_no_tail();
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

        rx.wait_ordered_exact([
            expected(&path).rename_from(),
            expected(&new_path).rename_to(),
            expected(&new_path).rename_from(),
            expected(&new_path2).rename_to(),
        ])
        .ensure_no_tail();
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

        let event = rx.recv();
        assert_eq!(event, expected(path).remove_any());
        rx.ensure_empty();
    }

    #[test]
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

        rx.wait_ordered_exact([
            expected(&file1).create(),
            expected(&file1).modify_any().multiple(),
            expected(&file2).modify_any().multiple(),
            expected(&file1).rename_from(),
            expected(&new_path).rename_to(),
            expected(&new_path).modify_any().multiple(),
            expected(&new_path).remove_any(),
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

        rx.wait_ordered_exact([
            expected(&path).rename_from(),
            expected(&new_path1).rename_to(),
            expected(&new_path1).rename_from(),
            expected(&new_path2).rename_to(),
        ])
        .ensure_no_tail();
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

        assert_eq!(rx.recv(), expected(&path).modify_any());
        rx.ensure_empty();
    }

    #[test]
    fn write_file_non_recursive_watch() {
        let tmpdir = testdir();
        let (mut watcher, mut rx) = watcher();

        let path = tmpdir.path().join("entry");
        std::fs::File::create_new(&path).expect("create");

        watcher.watch_nonrecursively(&path);

        std::fs::write(&path, b"123").expect("write");

        rx.wait_ordered_exact([expected(&path).modify_any().multiple()])
            .ensure_no_tail();
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

        let events = rx.iter().collect::<Vec<_>>();
        assert!(events.is_empty(), "unexpected events: {events:#?}");
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
    fn unwatch_waits_for_pending_callback_before_returning() {
        let tmpdir = testdir();
        let watched_dir = tmpdir.path().join("watched");
        std::fs::create_dir(&watched_dir).expect("create watched dir");

        let first = watched_dir.join("new_dir");
        let second = watched_dir.join("should_not_be_seen");
        let first_for_handler = first.clone();

        let (event_tx, event_rx) = mpsc::channel();
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let (unwatch_done_tx, unwatch_done_rx) = mpsc::channel();
        let (finish_tx, finish_rx) = mpsc::channel();
        let blocked_once = Arc::new(AtomicBool::new(false));
        let blocked_once_for_handler = blocked_once.clone();

        let mut watcher = ReadDirectoryChangesWatcher::new(
            move |res: crate::Result<crate::Event>| {
                if let Ok(event) = &res {
                    if event.paths.iter().any(|path| path == &first_for_handler)
                        && !blocked_once_for_handler.swap(true, Ordering::SeqCst)
                    {
                        started_tx.send(()).expect("signal callback start");
                        release_rx.recv().expect("release callback");
                    }
                }

                event_tx.send(res).expect("forward event");
            },
            crate::Config::default(),
        )
        .expect("create watcher");
        watcher
            .watch(&watched_dir, RecursiveMode::NonRecursive)
            .expect("watch dir");

        std::fs::create_dir(&first).expect("create first dir");
        started_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("wait for callback to block");

        let unwatch_path = watched_dir.clone();
        let join = thread::spawn(move || {
            let mut watcher = watcher;
            let result = watcher.unwatch(&unwatch_path);
            unwatch_done_tx.send(result).expect("send unwatch result");
            finish_rx.recv().expect("finish watcher thread");
        });

        assert!(
            unwatch_done_rx
                .recv_timeout(Duration::from_millis(100))
                .is_err(),
            "unwatch returned before the pending callback finished"
        );

        release_tx.send(()).expect("release callback");
        unwatch_done_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("wait for unwatch result")
            .expect("unwatch dir");

        std::fs::create_dir(&second).expect("create second dir");

        let first_event = event_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("receive first event")
            .expect("first event result");
        assert_eq!(first_event, expected(&first).create_folder());

        while let Ok(res) = event_rx.recv_timeout(Duration::from_millis(200)) {
            let event = res.expect("event result");
            assert!(
                !event.paths.iter().any(|path| path == &second),
                "unexpected event after unwatch: {event:#?}"
            );
        }

        finish_tx.send(()).expect("finish watcher thread");
        join.join().expect("join watcher thread");
    }
}
