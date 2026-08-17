//! The underlying system reaper.
//!
//! There are two backends:
//!
//! - signal, which waits for SIGCHLD.
//! - wait, which waits directly on a process handle.
//!
//! "wait" is preferred, but is not available on all supported Linuxes. So we
//! test to see if pidfd is supported first. If it is, we use wait. If not, we use
//! signal.

#![allow(irrefutable_let_patterns)]

/// Enable the waiting reaper.
#[cfg(any(windows, target_os = "linux"))]
macro_rules! cfg_wait {
    ($($tt:tt)*) => {$($tt)*};
}

/// Enable the waiting reaper.
#[cfg(not(any(windows, target_os = "linux")))]
macro_rules! cfg_wait {
    ($($tt:tt)*) => {};
}

/// Enable signals.
#[cfg(not(windows))]
macro_rules! cfg_signal {
    ($($tt:tt)*) => {$($tt)*};
}

/// Enable signals.
#[cfg(windows)]
macro_rules! cfg_signal {
    ($($tt:tt)*) => {};
}

cfg_wait! {
    mod wait;
}

cfg_signal! {
    mod signal;
}

use std::io;
use std::sync::Mutex;

/// The underlying system reaper.
pub(crate) enum Reaper {
    #[cfg(any(windows, target_os = "linux"))]
    /// The reaper based on the wait backend.
    Wait(wait::Reaper),

    /// The reaper based on the signal backend.
    #[cfg(not(windows))]
    Signal(signal::Reaper),
}

/// The wrapper around a child.
pub(crate) enum ChildGuard {
    #[cfg(any(windows, target_os = "linux"))]
    /// The child guard based on the wait backend.
    Wait(wait::ChildGuard),

    /// The child guard based on the signal backend.
    #[cfg(not(windows))]
    Signal(signal::ChildGuard),
}

/// A lock on the reaper.
pub(crate) enum Lock {
    #[cfg(any(windows, target_os = "linux"))]
    /// The wait-based reaper needs no lock.
    Wait,

    /// The lock for the signal-based reaper.
    #[cfg(not(windows))]
    Signal(signal::Lock),
}

impl Reaper {
    /// Create a new reaper.
    pub(crate) fn new() -> Self {
        cfg_wait! {
            if wait::available() && !cfg!(async_process_force_signal_backend) {
                return Self::Wait(wait::Reaper::new());
            }
        }

        // Return the signal-based reaper.
        cfg_signal! {
            return Self::Signal(signal::Reaper::new());
        }

        #[allow(unreachable_code)]
        {
            panic!("neither the signal backend nor the waiter backend is available")
        }
    }

    /// Lock the driver thread.
    ///
    /// This makes it so only one thread can reap at once.
    pub(crate) async fn lock(&'static self) -> Lock {
        cfg_wait! {
            if let Self::Wait(_this) = self {
                // No locking needed.
                return Lock::Wait;
            }
        }

        cfg_signal! {
            if let Self::Signal(this) = self {
                // We need to lock.
                return Lock::Signal(this.lock().await);
            }
        }

        unreachable!()
    }

    /// Reap zombie processes forever.
    pub(crate) async fn reap(&'static self, lock: Lock) -> ! {
        cfg_wait! {
            if let (Self::Wait(this), Lock::Wait) = (self, &lock) {
                this.reap().await;
            }
        }

        cfg_signal! {
            if let (Self::Signal(this), Lock::Signal(lock)) = (self, lock) {
                this.reap(lock).await;
            }
        }

        unreachable!()
    }

    /// Register a child into this reaper.
    pub(crate) fn register(&'static self, child: std::process::Child) -> io::Result<ChildGuard> {
        cfg_wait! {
            if let Self::Wait(this) = self {
                return this.register(child).map(ChildGuard::Wait);
            }
        }

        cfg_signal! {
            if let Self::Signal(this) = self {
                return this.register(child).map(ChildGuard::Signal);
            }
        }

        unreachable!()
    }

    /// Register a child that was spawned outside of `std` into this reaper by its raw pid.
    #[cfg(target_os = "macos")]
    pub(crate) fn adopt(&'static self, pid: rustix::process::Pid) -> ChildGuard {
        let Self::Signal(this) = self;
        ChildGuard::Signal(this.adopt(pid))
    }

    /// Wait for the inner child to complete.
    pub(crate) async fn status(
        &'static self,
        child: &Mutex<crate::ChildGuard>,
    ) -> io::Result<std::process::ExitStatus> {
        cfg_wait! {
            if let Self::Wait(this) = self {
                return this.status(child).await;
            }
        }

        cfg_signal! {
            if let Self::Signal(this) = self {
                return this.status(child).await;
            }
        }

        unreachable!()
    }

    /// Do we have any registered zombie processes?
    pub(crate) fn has_zombies(&'static self) -> bool {
        cfg_wait! {
            if let Self::Wait(this) = self {
                return this.has_zombies();
            }
        }

        cfg_signal! {
            if let Self::Signal(this) = self {
                return this.has_zombies();
            }
        }

        unreachable!()
    }
}

impl ChildGuard {
    /// Get the OS-assigned process identifier of the inner process.
    pub(crate) fn id(&mut self) -> u32 {
        cfg_wait! {
            if let Self::Wait(this) = self {
                return this.get_mut().id();
            }
        }

        cfg_signal! {
            if let Self::Signal(this) = self {
                return this.id();
            }
        }

        unreachable!()
    }

    /// Kill the inner process.
    pub(crate) fn kill(&mut self) -> io::Result<()> {
        cfg_wait! {
            if let Self::Wait(this) = self {
                return this.get_mut().kill();
            }
        }

        cfg_signal! {
            if let Self::Signal(this) = self {
                return this.kill();
            }
        }

        unreachable!()
    }

    /// Get the raw handle of the inner process.
    #[cfg(windows)]
    pub(crate) fn as_raw_handle(&mut self) -> std::os::windows::io::RawHandle {
        use std::os::windows::io::AsRawHandle;

        let Self::Wait(this) = self;
        this.get_mut().as_raw_handle()
    }

    /// Check whether the inner process has exited.
    pub(crate) fn try_wait(&mut self) -> io::Result<Option<std::process::ExitStatus>> {
        cfg_wait! {
            if let Self::Wait(this) = self {
                return this.get_mut().try_wait();
            }
        }

        cfg_signal! {
            if let Self::Signal(this) = self {
                return this.try_wait();
            }
        }

        unreachable!()
    }

    /// Start reaping this child process.
    pub(crate) fn reap(&mut self, reaper: &'static Reaper) {
        cfg_wait! {
            if let (Self::Wait(this), Reaper::Wait(reaper)) = (&mut *self, reaper) {
                this.reap(reaper);
                return;
            }
        }

        cfg_signal! {
            if let (Self::Signal(this), Reaper::Signal(reaper)) = (self, reaper) {
                this.reap(reaper);
                return;
            }
        }

        unreachable!()
    }
}
