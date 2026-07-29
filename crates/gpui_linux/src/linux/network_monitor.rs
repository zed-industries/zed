//! Network availability monitoring through NetworkManager's D-Bus interface.
//!
//! When NetworkManager is unavailable (or gpui_linux is built without a
//! windowing feature, and so without a D-Bus stack), availability stays
//! [`NetworkAvailability::Unknown`].

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{BackgroundExecutor, ForegroundExecutor, NetworkAvailability, Task};

type AvailabilityCallback = Rc<RefCell<Option<Box<dyn FnMut(NetworkAvailability)>>>>;

pub(crate) struct NetworkMonitorState {
    started: bool,
    availability: Rc<Cell<NetworkAvailability>>,
    callback: AvailabilityCallback,
    _watch_task: Option<Task<()>>,
    _receive_task: Option<Task<()>>,
}

impl NetworkMonitorState {
    pub(crate) fn new() -> Self {
        Self {
            started: false,
            availability: Rc::new(Cell::new(NetworkAvailability::Unknown)),
            callback: Rc::new(RefCell::new(None)),
            _watch_task: None,
            _receive_task: None,
        }
    }

    pub(crate) fn availability(
        &mut self,
        foreground_executor: &ForegroundExecutor,
        background_executor: &BackgroundExecutor,
    ) -> NetworkAvailability {
        self.start(foreground_executor, background_executor);
        self.availability.get()
    }

    pub(crate) fn on_change(
        &mut self,
        foreground_executor: &ForegroundExecutor,
        background_executor: &BackgroundExecutor,
        callback: Box<dyn FnMut(NetworkAvailability)>,
    ) {
        self.start(foreground_executor, background_executor);
        *self.callback.borrow_mut() = Some(callback);
    }

    fn start(
        &mut self,
        foreground_executor: &ForegroundExecutor,
        background_executor: &BackgroundExecutor,
    ) {
        if self.started {
            return;
        }
        self.started = true;

        let (sender, mut receiver) = mpsc::unbounded::<NetworkAvailability>();
        self._watch_task = Some(background_executor.spawn(async move {
            #[cfg(any(feature = "wayland", feature = "x11"))]
            if let Err(error) = watch_network_manager(sender).await {
                log::debug!("network monitor: NetworkManager unavailable: {error:#}");
            }

            #[cfg(not(any(feature = "wayland", feature = "x11")))]
            drop(sender);
        }));

        // NetworkManager reports arrive on the D-Bus task; this one applies
        // them on the main thread.
        let availability = self.availability.clone();
        let callback = self.callback.clone();
        self._receive_task = Some(foreground_executor.spawn(async move {
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

#[cfg(any(feature = "wayland", feature = "x11"))]
async fn watch_network_manager(
    sender: mpsc::UnboundedSender<NetworkAvailability>,
) -> anyhow::Result<()> {
    let connection = ashpd::zbus::Connection::system().await?;
    let proxy = ashpd::zbus::Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    )
    .await?;

    let initial: u32 = proxy.get_property("State").await?;
    if sender.unbounded_send(availability(initial)).is_err() {
        return Ok(());
    }

    let mut state_changes = proxy.receive_signal("StateChanged").await?;
    while let Some(message) = state_changes.next().await {
        let Ok(state) = message.body().deserialize::<u32>() else {
            continue;
        };
        if sender.unbounded_send(availability(state)).is_err() {
            break;
        }
    }
    Ok(())
}

// https://networkmanager.dev/docs/api/latest/nm-dbus-types.html#NMState
#[cfg(any(feature = "wayland", feature = "x11"))]
const NM_STATE_CONNECTED_SITE: u32 = 60;
#[cfg(any(feature = "wayland", feature = "x11"))]
const NM_STATE_CONNECTED_GLOBAL: u32 = 70;

#[cfg(any(feature = "wayland", feature = "x11"))]
fn availability(state: u32) -> NetworkAvailability {
    if matches!(state, NM_STATE_CONNECTED_SITE | NM_STATE_CONNECTED_GLOBAL) {
        NetworkAvailability::Online
    } else {
        NetworkAvailability::Offline
    }
}
