//! The Development interface lets any client, possibly in a sandbox if it has
//! access to the session helper, spawn a process on the host, outside any
//! sandbox.

use std::{collections::HashMap, os::fd::AsFd, path::Path};

use enumflags2::{BitFlags, bitflags};
use futures_util::Stream;
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{Fd, Type};

use crate::{Error, FilePath, Pid, proxy::Proxy};

#[bitflags]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Copy, Clone, Debug, Type)]
#[repr(u32)]
/// Flags affecting the running of commands on the host
pub enum HostCommandFlags {
    #[doc(alias = "FLATPAK_HOST_COMMAND_FLAGS_CLEAR_ENV")]
    /// Clear the environment.
    ClearEnv,
    #[doc(alias = "FLATPAK_HOST_COMMAND_FLAGS_WATCH_BUS")]
    /// Kill the sandbox when the caller disappears from the session bus.
    WatchBus,
}

/// The Development interface lets any client, possibly in a sandbox if it has
/// access to the session helper, spawn a process on the host, outside any
/// sandbox.
///
/// Wrapper of the DBus interface: [`org.freedesktop.Flatpak.Development`](https://docs.flatpak.org/en/latest/libflatpak-api-reference.html#gdbus-org.freedesktop.Flatpak.Development)
#[derive(Debug)]
#[doc(alias = "org.freedesktop.Flatpak.Development")]
pub struct Development(Proxy<'static>);

impl Development {
    /// Create a new instance of [`Development`]
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_flatpak_development("org.freedesktop.Flatpak.Development").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`Development`]
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy = Proxy::new_flatpak_development_with_connection(
            connection,
            "org.freedesktop.Flatpak.Development",
        )
        .await?;
        Ok(Self(proxy))
    }

    /// Emitted when a process started by
    /// [`host_command()`][`Development::host_command`] exits.
    ///
    /// # Specifications
    ///
    /// See also [`HostCommandExited`](https://docs.flatpak.org/en/latest/libflatpak-api-reference.html#gdbus-signal-org-freedesktop-Flatpak-Development.HostCommandExited).
    #[doc(alias = "HostCommandExited")]
    pub async fn receive_spawn_exited(&self) -> Result<impl Stream<Item = (u32, u32)>, Error> {
        self.0.signal("HostCommandExited").await
    }

    /// This method lets trusted applications (insider or outside a sandbox) run
    /// arbitrary commands in the user's session, outside any sandbox.
    ///
    /// # Arguments
    ///
    /// * `cwd_path` - The working directory for the new process.
    /// * `argv` - The argv for the new process, starting with the executable to
    ///   launch.
    /// * `fds` - Array of file descriptors to pass to the new process.
    /// * `envs` - Array of variable/value pairs for the environment of the new
    ///   process.
    /// * `flags`
    ///
    /// # Returns
    ///
    /// The PID of the new process.
    ///
    /// # Specifications
    ///
    /// See also [`HostCommand`](https://docs.flatpak.org/en/latest/libflatpak-api-reference.html#gdbus-method-org-freedesktop-Flatpak-Development.HostCommand).
    pub async fn host_command(
        &self,
        cwd_path: impl AsRef<Path>,
        argv: &[impl AsRef<Path>],
        fds: HashMap<u32, impl AsFd>,
        envs: HashMap<&str, &str>,
        flags: BitFlags<HostCommandFlags>,
    ) -> Result<u32, Error> {
        let cwd_path = FilePath::new(cwd_path)?;
        let argv = argv
            .iter()
            .map(FilePath::new)
            .collect::<Result<Vec<FilePath>, _>>()?;
        let fds: HashMap<u32, Fd> = fds.iter().map(|(k, val)| (*k, Fd::from(val))).collect();
        self.0
            .call("HostCommand", &(cwd_path, argv, fds, envs, flags))
            .await
    }

    /// This methods let you send a Unix signal to a process that was started
    /// [`host_command()`][`Development::host_command`].
    ///
    /// # Arguments
    ///
    /// * `pid` - The PID of the process to send the signal to.
    /// * `signal` - The signal to send.
    /// * `to_process_group` - Whether to send the signal to the process group.
    ///
    /// # Specifications
    ///
    /// See also [`HostCommandSignal`](https://docs.flatpak.org/en/latest/libflatpak-api-reference.html#gdbus-method-org-freedesktop-Flatpak-Development.HostCommandSignal).
    #[doc(alias = "SpawnSignal")]
    #[doc(alias = "xdp_portal_spawn_signal")]
    pub async fn host_command_signal(
        &self,
        pid: Pid,
        signal: u32,
        to_process_group: bool,
    ) -> Result<(), Error> {
        self.0
            .call("HostCommandSignal", &(pid, signal, to_process_group))
            .await
    }
}

impl std::ops::Deref for Development {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
