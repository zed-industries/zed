//! Network availability monitoring through the Network framework's
//! `nw_path_monitor` C API.

use std::cell::{Cell, RefCell};
use std::ffi::{c_int, c_void};
use std::rc::Rc;

use block2::RcBlock;
use dispatch2::{DispatchQueue, DispatchRetained};
use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{ForegroundExecutor, NetworkAvailability, Task};

#[link(name = "Network", kind = "framework")]
unsafe extern "C" {
    fn nw_path_monitor_create() -> *mut c_void;
    fn nw_path_monitor_set_update_handler(monitor: *mut c_void, handler: *const c_void);
    fn nw_path_monitor_set_queue(monitor: *mut c_void, queue: *mut c_void);
    fn nw_path_monitor_start(monitor: *mut c_void);
    fn nw_path_monitor_cancel(monitor: *mut c_void);
    fn nw_path_get_status(path: *mut c_void) -> c_int;
    fn nw_release(obj: *mut c_void);
}

const NW_PATH_STATUS_SATISFIED: c_int = 1;

type AvailabilityCallback = Rc<RefCell<Option<Box<dyn FnMut(NetworkAvailability)>>>>;

pub(crate) struct NetworkMonitorState {
    started: bool,
    availability: Rc<Cell<NetworkAvailability>>,
    callback: AvailabilityCallback,
    _monitor: Option<PathMonitor>,
    _receive_task: Option<Task<()>>,
}

impl NetworkMonitorState {
    pub(crate) fn new() -> Self {
        Self {
            started: false,
            availability: Rc::new(Cell::new(NetworkAvailability::Unknown)),
            callback: Rc::new(RefCell::new(None)),
            _monitor: None,
            _receive_task: None,
        }
    }

    pub(crate) fn availability(&mut self, executor: &ForegroundExecutor) -> NetworkAvailability {
        self.start(executor);
        self.availability.get()
    }

    pub(crate) fn on_change(
        &mut self,
        executor: &ForegroundExecutor,
        callback: Box<dyn FnMut(NetworkAvailability)>,
    ) {
        self.start(executor);
        *self.callback.borrow_mut() = Some(callback);
    }

    fn start(&mut self, executor: &ForegroundExecutor) {
        if self.started {
            return;
        }
        self.started = true;

        let (sender, mut receiver) = mpsc::unbounded::<NetworkAvailability>();
        self._monitor = Some(PathMonitor::start(sender));

        // Path updates arrive on the monitor's dispatch queue; this task
        // applies them on the main thread.
        let availability = self.availability.clone();
        let callback = self.callback.clone();
        self._receive_task = Some(executor.spawn(async move {
            while let Some(update) = receiver.next().await {
                if availability.replace(update) == update {
                    continue;
                }
                // Take the callback out for the call: it may re-enter the
                // platform (e.g. to read the availability just reported) or
                // replace itself.
                let taken = callback.borrow_mut().take();
                if let Some(mut taken) = taken {
                    taken(update);
                    callback.borrow_mut().get_or_insert(taken);
                }
            }
        }));
    }
}

struct PathMonitor {
    handle: *mut c_void,
    _block: RcBlock<dyn Fn(*mut c_void)>,
    _queue: DispatchRetained<DispatchQueue>,
}

impl PathMonitor {
    fn start(sender: mpsc::UnboundedSender<NetworkAvailability>) -> Self {
        let block = RcBlock::new(move |path: *mut c_void| {
            // SAFETY: `path` is the `nw_path_t` passed by the Network
            // framework; it is non-null and valid for the duration of the
            // callback.
            let status = unsafe { nw_path_get_status(path) };
            let availability = if status == NW_PATH_STATUS_SATISFIED {
                NetworkAvailability::Online
            } else {
                NetworkAvailability::Offline
            };
            sender.unbounded_send(availability).ok();
        });
        let queue = DispatchQueue::new("gpui.network_monitor", None);

        // SAFETY: `nw_path_monitor_create` returns an owned monitor that we
        // release in `Drop`. The block and queue references stored alongside
        // the handle outlive the monitor, so the framework's retained pointers
        // remain valid until after `nw_path_monitor_cancel`.
        let handle = unsafe {
            let monitor = nw_path_monitor_create();
            nw_path_monitor_set_update_handler(monitor, RcBlock::as_ptr(&block) as *const c_void);
            nw_path_monitor_set_queue(monitor, DispatchRetained::as_ptr(&queue).as_ptr().cast());
            nw_path_monitor_start(monitor);
            monitor
        };

        Self {
            handle,
            _block: block,
            _queue: queue,
        }
    }
}

impl Drop for PathMonitor {
    fn drop(&mut self) {
        // SAFETY: `handle` was obtained from `nw_path_monitor_create`. Cancel
        // before releasing so no in-flight handler runs after the block is
        // freed.
        unsafe {
            nw_path_monitor_cancel(self.handle);
            nw_release(self.handle);
        }
    }
}
