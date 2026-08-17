//! A version of the reaper that waits for a signal to check for process progress.

use async_lock::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard};
use async_signal::{Signal, Signals};
use event_listener::Event;
use futures_lite::{future, prelude::*};

use std::io;
use std::mem;
use std::sync::Mutex;

pub(crate) type Lock = AsyncMutexGuard<'static, ()>;

/// The zombie process reaper.
pub(crate) struct Reaper {
    /// An event delivered every time the SIGCHLD signal occurs.
    sigchld: Event,

    /// The list of zombie processes.
    zombies: Mutex<Vec<AnyChild>>,

    /// The pipe that delivers signal notifications.
    pipe: Pipe,

    /// Locking this mutex indicates that we are polling the SIGCHLD event.
    driver_guard: AsyncMutex<()>,
}

impl Reaper {
    /// Create a new reaper.
    pub(crate) fn new() -> Self {
        Reaper {
            sigchld: Event::new(),
            zombies: Mutex::new(Vec::new()),
            pipe: Pipe::new().expect("cannot create SIGCHLD pipe"),
            driver_guard: AsyncMutex::new(()),
        }
    }

    /// Lock the driver thread.
    pub(crate) async fn lock(&self) -> AsyncMutexGuard<'_, ()> {
        self.driver_guard.lock().await
    }

    /// Reap zombie processes forever.
    pub(crate) async fn reap(&'static self, _driver_guard: async_lock::MutexGuard<'_, ()>) -> ! {
        loop {
            // Wait for the next SIGCHLD signal.
            self.pipe.wait().await;

            // Notify all listeners waiting on the SIGCHLD event.
            self.sigchld.notify(usize::MAX);

            // Reap zombie processes, but make sure we don't hold onto the lock for too long!
            let mut zombies = mem::take(&mut *self.zombies.lock().unwrap());
            let mut i = 0;
            'reap_zombies: loop {
                for _ in 0..50 {
                    if i >= zombies.len() {
                        break 'reap_zombies;
                    }

                    if let Ok(None) = zombies[i].try_wait() {
                        i += 1;
                    } else {
                        #[allow(clippy::zombie_processes)]
                        // removed only when process done or errored
                        zombies.swap_remove(i);
                    }
                }

                // Be a good citizen; yield if there are a lot of processes.
                //
                // After we yield, check if there are more zombie processes.
                future::yield_now().await;
                zombies.append(&mut self.zombies.lock().unwrap());
            }

            // Put zombie processes back.
            self.zombies.lock().unwrap().append(&mut zombies);
        }
    }

    /// Register a process with this reaper.
    pub(crate) fn register(&'static self, child: std::process::Child) -> io::Result<ChildGuard> {
        self.pipe.register(&child)?;
        Ok(ChildGuard {
            inner: Some(AnyChild::Std(child)),
        })
    }

    /// Register a process that was spawned outside of `std` by its raw pid.
    #[cfg(target_os = "macos")]
    pub(crate) fn adopt(&'static self, pid: rustix::process::Pid) -> ChildGuard {
        ChildGuard {
            inner: Some(AnyChild::Raw(RawChild { pid, status: None })),
        }
    }

    /// Wait for an event to occur for a child process.
    pub(crate) async fn status(
        &'static self,
        child: &Mutex<crate::ChildGuard>,
    ) -> io::Result<std::process::ExitStatus> {
        loop {
            // Wait on the child process.
            if let Some(status) = child.lock().unwrap().inner.try_wait()? {
                return Ok(status);
            }

            // Start listening.
            event_listener::listener!(self.sigchld => listener);

            // Try again.
            if let Some(status) = child.lock().unwrap().inner.try_wait()? {
                return Ok(status);
            }

            // Wait on the listener.
            listener.await;
        }
    }

    /// Do we have any registered zombie processes?
    pub(crate) fn has_zombies(&'static self) -> bool {
        !self
            .zombies
            .lock()
            .unwrap_or_else(|x| x.into_inner())
            .is_empty()
    }
}

/// The wrapper around the child.
pub(crate) struct ChildGuard {
    inner: Option<AnyChild>,
}

impl ChildGuard {
    /// Get the OS-assigned process identifier of the inner child.
    pub(crate) fn id(&self) -> u32 {
        self.inner.as_ref().unwrap().id()
    }

    /// Kill the inner child.
    pub(crate) fn kill(&mut self) -> io::Result<()> {
        self.inner.as_mut().unwrap().kill()
    }

    /// Check whether the inner child has exited.
    pub(crate) fn try_wait(&mut self) -> io::Result<Option<std::process::ExitStatus>> {
        self.inner.as_mut().unwrap().try_wait()
    }

    /// Begin the reaping process for this child.
    pub(crate) fn reap(&mut self, reaper: &'static Reaper) {
        if let Ok(None) = self.try_wait() {
            reaper
                .zombies
                .lock()
                .unwrap()
                .push(self.inner.take().unwrap());
        }
    }
}

/// A child process handle, either spawned through `std` or adopted by raw pid.
enum AnyChild {
    Std(std::process::Child),
    #[cfg(target_os = "macos")]
    Raw(RawChild),
}

impl AnyChild {
    fn id(&self) -> u32 {
        match self {
            Self::Std(child) => child.id(),
            #[cfg(target_os = "macos")]
            Self::Raw(child) => child.pid.as_raw_nonzero().get() as u32,
        }
    }

    fn kill(&mut self) -> io::Result<()> {
        match self {
            Self::Std(child) => child.kill(),
            #[cfg(target_os = "macos")]
            Self::Raw(child) => child.kill(),
        }
    }

    fn try_wait(&mut self) -> io::Result<Option<std::process::ExitStatus>> {
        match self {
            Self::Std(child) => child.try_wait(),
            #[cfg(target_os = "macos")]
            Self::Raw(child) => child.try_wait(),
        }
    }
}

/// An adopted child process for which we only have a pid.
///
/// Like `std::process::Child`, the exit status is cached after a successful
/// wait so that subsequent calls don't wait on a reused pid.
#[cfg(target_os = "macos")]
struct RawChild {
    pid: rustix::process::Pid,
    status: Option<std::process::ExitStatus>,
}

#[cfg(target_os = "macos")]
impl RawChild {
    fn kill(&mut self) -> io::Result<()> {
        if self.status.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid argument: can't kill an exited process",
            ));
        }

        rustix::process::kill_process(self.pid, rustix::process::Signal::KILL)?;
        Ok(())
    }

    fn try_wait(&mut self) -> io::Result<Option<std::process::ExitStatus>> {
        use std::os::unix::process::ExitStatusExt;

        if self.status.is_none() {
            let wait_status =
                rustix::process::waitpid(Some(self.pid), rustix::process::WaitOptions::NOHANG)?;
            if let Some((_, wait_status)) = wait_status {
                self.status = Some(std::process::ExitStatus::from_raw(wait_status.as_raw()));
            }
        }

        Ok(self.status)
    }
}

/// Waits for the next SIGCHLD signal.
struct Pipe {
    /// The iterator over SIGCHLD signals.
    signals: Signals,
}

impl Pipe {
    /// Creates a new pipe.
    fn new() -> io::Result<Pipe> {
        Ok(Pipe {
            signals: Signals::new(Some(Signal::Child))?,
        })
    }

    /// Waits for the next SIGCHLD signal.
    async fn wait(&self) {
        (&self.signals).next().await;
    }

    /// Register a process object into this pipe.
    fn register(&self, _child: &std::process::Child) -> io::Result<()> {
        Ok(())
    }
}
