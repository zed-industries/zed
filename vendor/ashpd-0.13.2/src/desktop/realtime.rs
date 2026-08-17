//! Set threads to realtime.
//!
//! Wrapper of the DBus interface: [`org.freedesktop.portal.Realtime`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Realtime.html).

use crate::{Error, Pid, proxy::Proxy};

/// Interface for setting a thread to realtime from within the sandbox.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Realtime`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Realtime.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Realtime")]
pub struct Realtime(Proxy<'static>);

impl Realtime {
    /// Create a new instance of [`Realtime`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Realtime").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`Realtime`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Realtime")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    #[doc(alias = "MakeThreadRealtimeWithPID")]
    #[allow(missing_docs)]
    pub async fn max_thread_realtime_with_pid(
        &self,
        process: Pid,
        thread: u64,
        priority: u32,
    ) -> Result<(), Error> {
        self.0
            .call(
                "MakeThreadRealtimeWithPID",
                &(process as u64, thread, priority),
            )
            .await
    }

    #[doc(alias = "MakeThreadHighPriorityWithPID")]
    #[allow(missing_docs)]
    pub async fn max_thread_high_priority_with_pid(
        &self,
        process: Pid,
        thread: u64,
        priority: i32,
    ) -> Result<(), Error> {
        self.0
            .call(
                "MakeThreadHighPriorityWithPID",
                &(process as u64, thread, priority),
            )
            .await
    }

    #[doc(alias = "MaxRealtimePriority")]
    #[allow(missing_docs)]
    pub async fn max_realtime_priority(&self) -> Result<i64, Error> {
        self.0.property("MaxRealtimePriority").await
    }

    #[doc(alias = "MinNiceLevel")]
    #[allow(missing_docs)]
    pub async fn min_nice_level(&self) -> Result<u32, Error> {
        self.0.property("MinNiceLevel").await
    }

    #[doc(alias = "RTTimeUSecMax")]
    #[allow(missing_docs)]
    pub async fn rt_time_usec_max(&self) -> Result<u32, Error> {
        self.0.property("RTTimeUSecMax").await
    }
}

impl std::ops::Deref for Realtime {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
