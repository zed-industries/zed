//! # Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::memory_monitor::MemoryMonitor;
//! use futures_util::StreamExt;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = MemoryMonitor::new().await?;
//!     let level = proxy
//!         .receive_low_memory_warning()
//!         .await?
//!         .next()
//!         .await
//!         .expect("Stream exhausted");
//!     println!("{}", level);
//!     Ok(())
//! }
//! ```

use futures_util::Stream;

use crate::{Error, proxy::Proxy};

/// The interface provides information about low system memory to sandboxed
/// applications.
///
/// It is not a portal in the strict sense, since it does not involve user
/// interaction.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.MemoryMonitor`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.MemoryMonitor.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.MemoryMonitor")]
pub struct MemoryMonitor(Proxy<'static>);

impl MemoryMonitor {
    /// Create a new instance of [`MemoryMonitor`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.MemoryMonitor").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`MemoryMonitor`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.MemoryMonitor")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Signal emitted when a particular low memory situation happens
    /// with 0 being the lowest level of memory availability warning, and 255
    /// being the highest.
    ///
    /// # Specifications
    ///
    /// See also [`LowMemoryWarning`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.MemoryMonitor.html#org-freedesktop-portal-memorymonitor-lowmemorywarning).
    #[doc(alias = "LowMemoryWarning")]
    pub async fn receive_low_memory_warning(&self) -> Result<impl Stream<Item = i32>, Error> {
        self.0.signal("LowMemoryWarning").await
    }
}

impl std::ops::Deref for MemoryMonitor {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
