//! This module contains `XDG` and `DBus` specific code.
//!
//! it should not be available under any platform other than `(unix, not(target_os = "macos"))`

#[cfg(feature = "dbus")]
use dbus::ffidisp::Connection as DbusConnection;
#[cfg(feature = "zbus")]
use zbus::{block_on, zvariant};

use crate::{error::*, notification::Notification};

pub use crate::response::ActionResponse;
pub use crate::response::{CloseHandler, NotificationResponse, ResponseHandler};

use std::ops::{Deref, DerefMut};

#[cfg(feature = "dbus")]
mod dbus_rs;
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
use dbus_rs::bus;

#[cfg(feature = "zbus")]
mod zbus_rs;
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
use zbus_rs::bus;

#[cfg(all(feature = "dbus", feature = "zbus"))]
mod bus;

// #[cfg(all(feature = "server", feature = "dbus", unix, not(target_os = "macos")))]
// pub mod server_dbus;

// #[cfg(all(feature = "server", feature = "zbus", unix, not(target_os = "macos")))]
// pub mod server_zbus;

// #[cfg(all(feature = "server", unix, not(target_os = "macos")))]
// pub mod server;

#[cfg(not(feature = "debug_namespace"))]
#[doc(hidden)]
pub static NOTIFICATION_DEFAULT_BUS: &str = "org.freedesktop.Notifications";

#[cfg(feature = "debug_namespace")]
#[doc(hidden)]
// #[deprecated]
pub static NOTIFICATION_DEFAULT_BUS: &str = "de.hoodie.Notifications";

#[doc(hidden)]
pub static NOTIFICATION_INTERFACE: &str = "org.freedesktop.Notifications";

#[doc(hidden)]
pub static NOTIFICATION_OBJECTPATH: &str = "/org/freedesktop/Notifications";

pub(crate) use bus::NotificationBus;

#[derive(Debug)]
enum NotificationHandleInner {
    #[cfg(feature = "dbus")]
    Dbus(dbus_rs::DbusNotificationHandle),

    #[cfg(feature = "zbus")]
    Zbus(zbus_rs::ZbusNotificationHandle),
}

/// A handle to a shown notification.
///
/// Keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    inner: NotificationHandleInner,
}

#[allow(dead_code)]
impl NotificationHandle {
    #[cfg(feature = "dbus")]
    pub(crate) fn for_dbus(
        id: u32,
        connection: DbusConnection,
        notification: Notification,
    ) -> NotificationHandle {
        NotificationHandle {
            inner: dbus_rs::DbusNotificationHandle::new(id, connection, notification).into(),
        }
    }

    #[cfg(feature = "zbus")]
    pub(crate) fn for_zbus(
        id: u32,
        connection: zbus::Connection,
        notification: Notification,
    ) -> NotificationHandle {
        NotificationHandle {
            inner: zbus_rs::ZbusNotificationHandle::new(id, connection, notification).into(),
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => {
                let _ = inner.wait_for_action(|response: &NotificationResponse| match response {
                    NotificationResponse::Default => invocation_closure("default"),
                    NotificationResponse::Action(ref action) => invocation_closure(action),
                    NotificationResponse::Reply(_) => { /* XDG does not support inline replies */ }
                    NotificationResponse::Closed(_) => invocation_closure("__closed"),
                });
            }

            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                block_on(
                    inner.wait_for_action(|response: &NotificationResponse| match response {
                        NotificationResponse::Default => invocation_closure("default"),
                        NotificationResponse::Action(ref action) => invocation_closure(action),
                        NotificationResponse::Reply(_) => { /* XDG does not support inline replies */ }
                        NotificationResponse::Closed(_) => invocation_closure("__closed"), // FIXME: remove backward compatibility with 5.0
                    }),
                );
            }
        };
    }

    /// Waits for the user to act on a notification and then calls `handler`
    /// with a typed [`NotificationResponse`].
    ///
    /// This is the typed, forward-compatible replacement for [`wait_for_action`](Self::wait_for_action).
    pub fn wait_for_response(self, handler: impl ResponseHandler) -> Result<()> {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => inner.wait_for_action(handler),
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                block_on(inner.wait_for_action(handler));
                Ok(())
            }
        }
    }

    /// Returns a future that waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    ///
    /// # Panics
    ///
    /// Panics if called with a [`Dbus`](DbusStack::Dbus) backend.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use notify_rust::*;
    /// # use async_std::task::sleep;
    /// # use std::time::Duration;
    /// # use futures_lite::future::zip;
    /// # async fn wait_for_action_async_example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handle: NotificationHandle = Notification::new()
    ///     .action("do-stuff", "my fancy button")
    ///     .show_async()
    ///     .await?;
    ///
    /// let wait_future = handle.wait_for_action_async(|action| {
    ///     // handle action
    /// #   let _ = action;
    /// });
    /// let close_future = async {
    ///     sleep(Duration::from_secs(5)).await;
    ///     handle.close_async();
    /// };
    ///
    /// // run both futures concurrently
    /// # let _ =
    /// zip(wait_future, close_future).await;
    /// # Ok(())
    /// # }
    /// ```
    // TODO: make this consume `self` in 5.0
    #[cfg(feature = "zbus")]
    pub async fn wait_for_action_async<F>(&self, invocation_closure: F)
    where
        F: FnOnce(&NotificationResponse),
    {
        match &self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(_) => {
                unimplemented!("async methods are not supported with the `dbus` backend");
            }
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => inner.wait_for_action(invocation_closure).await,
        }
    }

    /// Manually close the notification
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use notify_rust::*;
    /// let handle: NotificationHandle = Notification::new()
    ///     .summary("oh no")
    ///     .hint(notify_rust::Hint::Transient(true))
    ///     .body("I'll be here till you close me!")
    ///     .hint(Hint::Resident(true)) // does not work on kde
    ///     .timeout(Timeout::Never) // works on kde and gnome
    ///     .show()
    ///     .unwrap();
    /// // ... and then later
    /// handle.close();
    /// ```
    pub fn close(self) {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => inner.close(),
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => block_on(inner.close()),
        }
    }

    /// Async version of [`close`](Self::close).
    ///
    /// # Panics
    ///
    /// Panics if called with a [`Dbus`](DbusStack::Dbus) backend.
    #[cfg(feature = "zbus")]
    pub async fn close_async(&self) {
        match &self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(_) => {
                unimplemented!("async methods are not supported with the `dbus` backend");
            }
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => inner.close().await,
        }
    }

    /// Executes a closure after the notification has closed.
    ///
    /// ## Example 1: *I don't care about why it closed* (the good ole API)
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|| println!("closed"));
    /// ```
    ///
    /// ## Example 2: *I **do** care about why it closed* (added in v4.5.0)
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|reason| println!("closed: {:?}", reason));
    /// ```
    // #[deprecated(
    //     since = "4.18.0",
    //     note = "Use `wait_for_response()` and match on `ActionResponse::Closed` instead"
    // )]
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => {
                let _ = inner.wait_for_action(|action: &NotificationResponse| {
                    if let NotificationResponse::Closed(reason) = action {
                        handler.call(*reason);
                    }
                });
            }
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                block_on(inner.wait_for_action(|action: &NotificationResponse| {
                    if let NotificationResponse::Closed(reason) = action {
                        handler.call(*reason);
                    }
                }));
            }
        };
    }

    /// Replace the original notification with an updated version
    /// ## Example
    /// ```no_run
    /// # use notify_rust::Notification;
    /// let mut notification = Notification::new().summary("Latest News")
    ///                                           .body("Bayern Dortmund 3:2")
    ///                                           .show()
    ///                                           .unwrap();
    ///
    /// std::thread::sleep_ms(1_500);
    ///
    /// notification.summary("Latest News (Correction)")
    ///             .body("Bayern Dortmund 3:3");
    ///
    /// notification.update().unwrap();
    /// ```
    /// Watch out for different implementations of the
    /// notification server! On plasma5 for instance, you should also change the appname, so the old
    /// message is really replaced and not just amended. Xfce behaves well, all others have not
    /// been tested by the developer.
    pub fn update(&mut self) -> Result<()> {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref mut inner) => inner.update(),
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => inner.update(),
        }
    }

    /// Returns the handle's id.
    pub fn id(&self) -> u32 {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref inner) => inner.id,
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref inner) => inner.id,
        }
    }
}

/// Required for [`DerefMut`].
impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref inner) => &inner.notification,
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref inner) => &inner.notification,
        }
    }
}

/// Allows easy modification of notification properties.
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref mut inner) => &mut inner.notification,
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => &mut inner.notification,
        }
    }
}

#[cfg(feature = "dbus")]
impl From<dbus_rs::DbusNotificationHandle> for NotificationHandleInner {
    fn from(handle: dbus_rs::DbusNotificationHandle) -> NotificationHandleInner {
        NotificationHandleInner::Dbus(handle)
    }
}

#[cfg(feature = "zbus")]
impl From<zbus_rs::ZbusNotificationHandle> for NotificationHandleInner {
    fn from(handle: zbus_rs::ZbusNotificationHandle) -> NotificationHandleInner {
        NotificationHandleInner::Zbus(handle)
    }
}

#[cfg(feature = "dbus")]
impl From<dbus_rs::DbusNotificationHandle> for NotificationHandle {
    fn from(handle: dbus_rs::DbusNotificationHandle) -> NotificationHandle {
        NotificationHandle {
            inner: handle.into(),
        }
    }
}

#[cfg(feature = "zbus")]
impl From<zbus_rs::ZbusNotificationHandle> for NotificationHandle {
    fn from(handle: zbus_rs::ZbusNotificationHandle) -> NotificationHandle {
        NotificationHandle {
            inner: handle.into(),
        }
    }
}

// here be public functions

// TODO: breaking change, wait for 5.0
// #[cfg(all(feature = "dbus", feature = "zbus"))]
//compile_error!("the z and d features are mutually exclusive");

#[cfg(all(
    not(any(feature = "dbus", feature = "zbus")),
    unix,
    not(target_os = "macos")
))]
compile_error!("you have to build with either zbus or dbus turned on");

/// Which D-Bus implementation is in use.
#[derive(Copy, Clone, Debug)]
pub enum DbusStack {
    /// Using [dbus-rs](https://docs.rs/dbus-rs).
    Dbus,
    /// Using [zbus](https://docs.rs/zbus).
    Zbus,
}

#[cfg(all(feature = "dbus", feature = "zbus"))]
const DBUS_SWITCH_VAR: &str = "DBUSRS";

#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    block_on(zbus_rs::connect_and_send_notification(notification)).map(Into::into)
}

#[cfg(feature = "zbus")]
pub(crate) async fn show_notification_async(
    notification: &Notification,
) -> Result<NotificationHandle> {
    zbus_rs::connect_and_send_notification(notification)
        .await
        .map(Into::into)
}

#[cfg(feature = "zbus")]
pub(crate) async fn show_notification_async_at_bus(
    notification: &Notification,
    bus: NotificationBus,
) -> Result<NotificationHandle> {
    zbus_rs::connect_and_send_notification_at_bus(notification, bus)
        .await
        .map(Into::into)
}

#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    dbus_rs::connect_and_send_notification(notification).map(Into::into)
}

#[cfg(all(feature = "dbus", feature = "zbus"))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::connect_and_send_notification(notification).map(Into::into)
    } else {
        block_on(zbus_rs::connect_and_send_notification(notification)).map(Into::into)
    }
}

/// Get the currently active [`DbusStack`].
///
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(DbusStack::Zbus)
}

/// Get the currently active [`DbusStack`].
///
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(DbusStack::Dbus)
}

/// Get the currently active [`DbusStack`].
///
/// Both dbus-rs and zbus are compiled in; switch via the `$DBUSRS` environment variable.
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        DbusStack::Dbus
    } else {
        DbusStack::Zbus
    })
}

/// Get the currently active [`DbusStack`].
///
/// Neither `zbus` nor `dbus-rs` are configured; always returns `None`.
#[cfg(all(not(feature = "dbus"), not(feature = "zbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    None
}

/// Returns a list of all capabilities of the running notification server.
///
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn get_capabilities() -> Result<Vec<String>> {
    block_on(zbus_rs::get_capabilities())
}

/// Returns a list of all capabilities of the running notification server.
///
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn get_capabilities() -> Result<Vec<String>> {
    dbus_rs::get_capabilities()
}

/// Returns a list of all capabilities of the running notification server.
///
/// Both dbus-rs and zbus are compiled in; switch via the `$DBUSRS` environment variable.
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn get_capabilities() -> Result<Vec<String>> {
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::get_capabilities()
    } else {
        block_on(zbus_rs::get_capabilities())
    }
}

/// Returns a [`ServerInformation`] struct describing the running notification server.
///
/// The struct contains `name`, `vendor`, `version`, and `spec_version`.
///
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn get_server_information() -> Result<ServerInformation> {
    block_on(zbus_rs::get_server_information())
}

/// Returns a [`ServerInformation`] struct describing the running notification server.
///
/// The struct contains `name`, `vendor`, `version`, and `spec_version`.
///
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn get_server_information() -> Result<ServerInformation> {
    dbus_rs::get_server_information()
}

/// Returns a [`ServerInformation`] struct describing the running notification server.
///
/// The struct contains `name`, `vendor`, `version`, and `spec_version`.
///
/// Both dbus-rs and zbus are compiled in; switch via the `$DBUSRS` environment variable.
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn get_server_information() -> Result<ServerInformation> {
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::get_server_information()
    } else {
        block_on(zbus_rs::get_server_information())
    }
}

/// Return value of [`get_server_information()`].
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
pub struct ServerInformation {
    /// The product name of the server.
    pub name: String,
    /// The vendor name.
    pub vendor: String,
    /// The server's version string.
    pub version: String,
    /// The specification version the server is compliant with.
    pub spec_version: String,
}

// /// Strictly internal.
// /// The NotificationServer implemented here exposes a "Stop" function.
// /// stops the notification server
// #[cfg(all(feature = "server", unix, not(target_os = "macos")))]
// #[doc(hidden)]
// pub fn stop_server() {
//     #[cfg(feature = "dbus")]
//     dbus_rs::stop_server()
// }

/// Listens for the `ActionInvoked(UInt32, String)` signal.
///
/// Prefer [`NotificationHandle::wait_for_action`] instead.
/// (xdg only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
// #[deprecated(note="please use [`NotificationHandle::wait_for_action`]")]
pub fn handle_action<F>(id: u32, func: F) -> Result<()>
where
    F: FnOnce(&ActionResponse<'_>),
{
    block_on(zbus_rs::handle_action(id, action_response_adapter(func)));
    Ok(())
}

/// Listens for the `ActionInvoked(UInt32, String)` signal.
///
/// Prefer [`NotificationHandle::wait_for_action`] instead.
/// (xdg only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
// #[deprecated(note="please use `NotificationHandle::wait_for_action`")]
pub fn handle_action<F>(id: u32, func: F) -> Result<()>
where
    F: FnOnce(&ActionResponse<'_>),
{
    dbus_rs::handle_action(id, action_response_adapter(func))
}

/// Listens for the `ActionInvoked(UInt32, String)` signal.
///
/// Prefer [`NotificationHandle::wait_for_action`] instead.
/// Both dbus-rs and zbus are compiled in; switch via the `$DBUSRS` environment variable.
#[cfg(all(feature = "dbus", feature = "zbus"))]
// #[deprecated(note="please use `NotificationHandle::wait_for_action`")]
pub fn handle_action<F>(id: u32, func: F) -> Result<()>
where
    F: FnOnce(&ActionResponse<'_>),
{
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::handle_action(id, action_response_adapter(func))
    } else {
        block_on(zbus_rs::handle_action(id, action_response_adapter(func)));
        Ok(())
    }
}

/// Wraps an old-style `FnOnce(&ActionResponse)` into a new-style `FnOnce(&NotificationResponse)`
/// so legacy callers of [`handle_action`] keep working.
fn action_response_adapter<F>(func: F) -> impl FnOnce(&NotificationResponse)
where
    F: FnOnce(&ActionResponse<'_>),
{
    move |response: &NotificationResponse| match response {
        NotificationResponse::Default => func(&ActionResponse::Custom("default")),
        NotificationResponse::Action(ref s) => func(&ActionResponse::Custom(s.as_str())),
        NotificationResponse::Reply(_) => { /* XDG does not support inline replies */ }
        NotificationResponse::Closed(r) => func(&ActionResponse::Closed(*r)),
    }
}
