//! # Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::game_mode::GameMode;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = GameMode::new().await?;
//!
//!     println!("{:#?}", proxy.register(246612).await?);
//!     println!("{:#?}", proxy.query_status(246612).await?);
//!     println!("{:#?}", proxy.unregister(246612).await?);
//!     println!("{:#?}", proxy.query_status(246612).await?);
//!
//!     Ok(())
//! }
//! ```

use std::{fmt::Debug, os::fd::AsFd};

use serde_repr::Deserialize_repr;
use zbus::zvariant::{Fd, Type};

use crate::{Error, Pid, error::PortalError, proxy::Proxy};

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdGameModeStatus"))]
#[derive(Deserialize_repr, PartialEq, Eq, Debug, Clone, Copy, Type)]
#[repr(i32)]
/// The status of the game mode.
pub enum Status {
    /// GameMode is inactive.
    Inactive = 0,
    /// GameMode is active.
    Active = 1,
    /// GameMode is active and `pid` is registered.
    Registered = 2,
    /// The query failed inside GameMode.
    Rejected = -1,
}

#[derive(Deserialize_repr, PartialEq, Eq, Debug, Type)]
#[repr(i32)]
/// The status of a (un-)register game mode request.
enum RegisterStatus {
    /// If the game was successfully (un-)registered.
    Success = 0,
    /// If the request was rejected by GameMode.
    Rejected = -1,
}

/// The interface lets sandboxed applications access GameMode from within the
/// sandbox.
///
/// It is analogous to the `com.feralinteractive.GameMode` interface and will
/// proxy request there, but with additional permission checking and pid
/// mapping. The latter is necessary in the case that sandbox has pid namespace
/// isolation enabled. See the man page for pid_namespaces(7) for more details,
/// but briefly, it means that the sandbox has its own process id namespace
/// which is separated from the one on the host. Thus there will be two separate
/// process ids (pids) within two different namespaces that both identify same
/// process. One id from the pid namespace inside the sandbox and one id from
/// the host pid namespace. Since GameMode expects pids from the host pid
/// namespace but programs inside the sandbox can only know pids from the
/// sandbox namespace, process ids need to be translated from the portal to the
/// host namespace. The portal will do that transparently for all calls where
/// this is necessary.
///
/// Note: GameMode will monitor active clients, i.e. games and other programs
/// that have successfully called [`GameMode::register`]. In the event
/// that a client terminates without a call to the
/// [`GameMode::unregister`] method, GameMode will automatically
/// un-register the client. This might happen with a (small) delay.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.GameMode`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.GameMode")]
pub struct GameMode(Proxy<'static>);

impl GameMode {
    /// Create a new instance of [`GameMode`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.GameMode").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`GameMode`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.GameMode")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Query the GameMode status for a process.
    /// If the caller is running inside a sandbox with pid namespace isolation,
    /// the pid will be translated to the respective host pid.
    ///
    /// # Arguments
    ///
    /// * `pid` - Process id to query the GameMode status of.
    ///
    /// # Specifications
    ///
    /// See also [`QueryStatus`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-querystatus).
    #[doc(alias = "QueryStatus")]
    pub async fn query_status(&self, pid: Pid) -> Result<Status, Error> {
        self.0.call("QueryStatus", &(pid as i32)).await
    }

    /// Query the GameMode status for a process.
    ///
    /// # Arguments
    ///
    /// * `target` - Pid file descriptor to query the GameMode status of.
    /// * `requester` - Pid file descriptor of the process requesting the
    ///   information.
    ///
    /// # Specifications
    ///
    /// See also [`QueryStatusByPIDFd`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-querystatusbypidfd).
    #[doc(alias = "QueryStatusByPIDFd")]
    pub async fn query_status_by_pidfd(
        &self,
        target: &impl AsFd,
        requester: &impl AsFd,
    ) -> Result<Status, Error> {
        self.0
            .call(
                "QueryStatusByPIDFd",
                &(Fd::from(target), Fd::from(requester)),
            )
            .await
    }

    /// Query the GameMode status for a process.
    ///
    /// # Arguments
    ///
    /// * `target` - Process id to query the GameMode status of.
    /// * `requester` - Process id of the process requesting the information.
    ///
    /// # Specifications
    ///
    /// See also [`QueryStatusByPid`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-querystatusbypid).
    #[doc(alias = "QueryStatusByPid")]
    pub async fn query_status_by_pid(&self, target: Pid, requester: Pid) -> Result<Status, Error> {
        self.0
            .call("QueryStatusByPid", &(target as i32, requester as i32))
            .await
    }

    /// Register a game with GameMode and thus request GameMode to be activated.
    /// If the caller is running inside a sandbox with pid namespace isolation,
    /// the pid will be translated to the respective host pid. See the general
    /// introduction for details. If the GameMode has already been requested
    /// for pid before, this call will fail.
    ///
    /// # Arguments
    ///
    /// * `pid` - Process id of the game to register.
    ///
    /// # Specifications
    ///
    /// See also [`RegisterGame`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-registergame).
    #[doc(alias = "RegisterGame")]
    pub async fn register(&self, pid: Pid) -> Result<(), Error> {
        let status = self.0.call("RegisterGame", &(pid as i32)).await?;
        match status {
            RegisterStatus::Success => Ok(()),
            RegisterStatus::Rejected => Err(Error::Portal(PortalError::Failed(format!(
                "Failed to register game for `{pid}`"
            )))),
        }
    }

    /// Register a game with GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Process file descriptor of the game to register.
    /// * `requester` - Process file descriptor of the process requesting the
    ///   registration.
    ///
    /// # Specifications
    ///
    /// See also [`RegisterGameByPIDFd`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-registergamebypidfd).
    #[doc(alias = "RegisterGameByPIDFd")]
    pub async fn register_by_pidfd(
        &self,
        target: &impl AsFd,
        requester: &impl AsFd,
    ) -> Result<(), Error> {
        let status = self
            .0
            .call(
                "RegisterGameByPIDFd",
                &(Fd::from(target), Fd::from(requester)),
            )
            .await?;
        match status {
            RegisterStatus::Success => Ok(()),
            RegisterStatus::Rejected => Err(Error::Portal(PortalError::Failed(
                "Failed to register by pidfd".to_string(),
            ))),
        }
    }

    /// Register a game with GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Process id of the game to register.
    /// * `requester` - Process id of the process requesting the registration.
    ///
    /// # Specifications
    ///
    /// See also [`RegisterGameByPid`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-registergamebypid).
    #[doc(alias = "RegisterGameByPid")]
    pub async fn register_by_pid(&self, target: Pid, requester: Pid) -> Result<(), Error> {
        let status = self
            .0
            .call("RegisterGameByPid", &(target as i32, requester as i32))
            .await?;
        match status {
            RegisterStatus::Success => Ok(()),
            RegisterStatus::Rejected => Err(Error::Portal(PortalError::Failed(format!(
                "Failed to register by pid for target=`{target}` requester=`{requester}`"
            )))),
        }
    }

    /// Un-register a game from GameMode.
    /// if the call is successful and there are no other games or clients
    /// registered, GameMode will be deactivated. If the caller is running
    /// inside a sandbox with pid namespace isolation, the pid will be
    /// translated to the respective host pid.
    ///
    /// # Arguments
    ///
    /// * `pid` - Process id of the game to un-register.
    ///
    /// # Specifications
    ///
    /// See also [`UnregisterGame`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-unregistergame).
    #[doc(alias = "UnregisterGame")]
    pub async fn unregister(&self, pid: Pid) -> Result<(), Error> {
        let status = self.0.call("UnregisterGame", &(pid as i32)).await?;
        match status {
            RegisterStatus::Success => Ok(()),
            RegisterStatus::Rejected => Err(Error::Portal(PortalError::Failed(format!(
                "Failed to unregister for `{pid}`"
            )))),
        }
    }

    /// Un-register a game from GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Pid file descriptor of the game to un-register.
    /// * `requester` - Pid file descriptor of the process requesting the
    ///   un-registration.
    ///
    /// # Specifications
    ///
    /// See also [`UnregisterGameByPIDFd`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-unregistergamebypidfd).
    #[doc(alias = "UnregisterGameByPIDFd")]
    pub async fn unregister_by_pidfd(
        &self,
        target: &impl AsFd,
        requester: &impl AsFd,
    ) -> Result<(), Error> {
        let status = self
            .0
            .call(
                "UnregisterGameByPIDFd",
                &(Fd::from(target), Fd::from(requester)),
            )
            .await?;
        match status {
            RegisterStatus::Success => Ok(()),
            RegisterStatus::Rejected => Err(Error::Portal(PortalError::Failed(
                "Failed to unregister by pidfd`".to_string(),
            ))),
        }
    }

    /// Un-register a game from GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Process id of the game to un-register.
    /// * `requester` - Process id of the process requesting the
    ///   un-registration.
    ///
    /// # Specifications
    ///
    /// See also [`UnregisterGameByPid`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-unregistergamebypid).
    #[doc(alias = "UnregisterGameByPid")]
    pub async fn unregister_by_pid(&self, target: Pid, requester: Pid) -> Result<(), Error> {
        let status = self
            .0
            .call("UnregisterGameByPid", &(target as i32, requester as i32))
            .await?;
        match status {
            RegisterStatus::Success => Ok(()),
            RegisterStatus::Rejected => Err(Error::Portal(PortalError::Failed(format!(
                "Failed to unregister by pid for target=`{target}` requester=`{requester}`"
            )))),
        }
    }

    /// Whether any pid on the system has enabled Game Mode.
    ///
    /// # Specifications
    ///
    /// See also [`Active`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GameMode.html#org-freedesktop-portal-gamemode-active).
    #[doc(alias = "Active")]
    pub async fn is_active(&self) -> Result<bool, Error> {
        let is_active = self.0.property("Active").await?;
        Ok(is_active)
    }
}

impl std::ops::Deref for GameMode {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
