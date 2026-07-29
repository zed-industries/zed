//! Network availability monitoring through the WinRT
//! `NetworkInformation` API.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{ForegroundExecutor, NetworkAvailability, Task};
use windows::Networking::Connectivity::{
    NetworkConnectivityLevel, NetworkInformation, NetworkStatusChangedEventHandler,
};

type AvailabilityCallback = Rc<RefCell<Option<Box<dyn FnMut(NetworkAvailability)>>>>;

pub(crate) struct NetworkMonitorState {
    started: Cell<bool>,
    availability: Rc<Cell<NetworkAvailability>>,
    callback: AvailabilityCallback,
    _receive_task: RefCell<Option<Task<()>>>,
    change_token: Cell<Option<i64>>,
}

impl NetworkMonitorState {
    pub(crate) fn new() -> Self {
        Self {
            started: Cell::new(false),
            availability: Rc::new(Cell::new(NetworkAvailability::Unknown)),
            callback: Rc::new(RefCell::new(None)),
            _receive_task: RefCell::new(None),
            change_token: Cell::new(None),
        }
    }

    pub(crate) fn availability(&self, executor: &ForegroundExecutor) -> NetworkAvailability {
        self.start(executor);
        self.availability.get()
    }

    pub(crate) fn on_change(
        &self,
        executor: &ForegroundExecutor,
        callback: Box<dyn FnMut(NetworkAvailability)>,
    ) {
        self.start(executor);
        *self.callback.borrow_mut() = Some(callback);
    }

    fn start(&self, executor: &ForegroundExecutor) {
        if self.started.replace(true) {
            return;
        }

        let (sender, mut receiver) = mpsc::unbounded::<NetworkAvailability>();

        let handler_sender = sender.clone();
        let handler = NetworkStatusChangedEventHandler::new(move |_sender| {
            handler_sender.unbounded_send(current_availability()).ok();
            Ok(())
        });
        match NetworkInformation::NetworkStatusChanged(&handler) {
            Ok(token) => self.change_token.set(Some(token)),
            Err(error) => log::warn!("failed to observe network status changes: {error}"),
        }
        sender.unbounded_send(current_availability()).ok();

        // Status changes arrive on WinRT threads; this task applies them on
        // the main thread.
        let availability = self.availability.clone();
        let callback = self.callback.clone();
        *self._receive_task.borrow_mut() = Some(executor.spawn(async move {
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

impl Drop for NetworkMonitorState {
    fn drop(&mut self) {
        if let Some(token) = self.change_token.take()
            && let Err(error) = NetworkInformation::RemoveNetworkStatusChanged(token)
        {
            log::warn!("failed to remove network status change handler: {error}");
        }
    }
}

fn current_availability() -> NetworkAvailability {
    let Ok(profile) = NetworkInformation::GetInternetConnectionProfile() else {
        return NetworkAvailability::Offline;
    };
    match profile.GetNetworkConnectivityLevel() {
        Ok(NetworkConnectivityLevel::InternetAccess)
        | Ok(NetworkConnectivityLevel::ConstrainedInternetAccess) => NetworkAvailability::Online,
        _ => NetworkAvailability::Offline,
    }
}
