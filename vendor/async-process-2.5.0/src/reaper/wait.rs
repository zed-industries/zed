//! A version of the reaper that waits on some polling primitive.
//!
//! This uses:
//!
//! - pidfd on Linux
//! - Waitable objects on Windows

use async_channel::{Receiver, Sender};
use async_task::Runnable;
use futures_lite::future;

use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::task::{Context, Poll};

/// The zombie process reaper.
pub(crate) struct Reaper {
    /// The channel for sending new runnables.
    sender: Sender<Runnable>,

    /// The channel for receiving new runnables.
    recv: Receiver<Runnable>,

    /// Number of zombie processes.
    zombies: AtomicUsize,
}

impl Reaper {
    /// Create a new reaper.
    pub(crate) fn new() -> Self {
        let (sender, recv) = async_channel::unbounded();
        Self {
            sender,
            recv,
            zombies: AtomicUsize::new(0),
        }
    }

    /// Reap zombie processes forever.
    pub(crate) async fn reap(&'static self) -> ! {
        loop {
            // Fetch the next task.
            let task = match self.recv.recv().await {
                Ok(task) => task,
                Err(_) => panic!("sender should never be closed"),
            };

            // Poll the task.
            task.run();
        }
    }

    /// Register a child into this reaper.
    pub(crate) fn register(&'static self, child: std::process::Child) -> io::Result<ChildGuard> {
        Ok(ChildGuard {
            inner: Some(WaitableChild::new(child)?),
        })
    }

    /// Wait for a child to complete.
    pub(crate) async fn status(
        &'static self,
        child: &Mutex<crate::ChildGuard>,
    ) -> io::Result<std::process::ExitStatus> {
        future::poll_fn(|cx| {
            // Lock the child.
            let mut child = child.lock().unwrap();

            // Get the inner child value.
            #[allow(clippy::infallible_destructuring_match)] // false positive: should respect cfg
            let inner = match &mut child.inner {
                super::ChildGuard::Wait(inner) => inner,
                #[cfg(not(windows))]
                _ => unreachable!(),
            };

            // Poll for the next value.
            inner.inner.as_mut().unwrap().poll_wait(cx)
        })
        .await
    }

    /// Do we have any registered zombie processes?
    pub(crate) fn has_zombies(&'static self) -> bool {
        self.zombies.load(Ordering::SeqCst) > 0
    }
}

/// The wrapper around the child.
pub(crate) struct ChildGuard {
    inner: Option<WaitableChild>,
}

impl ChildGuard {
    /// Get a mutable reference to the inner child.
    pub(crate) fn get_mut(&mut self) -> &mut std::process::Child {
        self.inner.as_mut().unwrap().get_mut()
    }

    /// Begin the reaping process for this child.
    pub(crate) fn reap(&mut self, reaper: &'static Reaper) {
        // Create a future for polling this child.
        let future = {
            let mut inner = self.inner.take().unwrap();
            async move {
                // Increment the zombie count.
                reaper.zombies.fetch_add(1, Ordering::Relaxed);

                // Decrement the zombie count once we are done.
                let _guard = crate::CallOnDrop(|| {
                    reaper.zombies.fetch_sub(1, Ordering::SeqCst);
                });

                // Wait on this child forever.
                let result = future::poll_fn(|cx| inner.poll_wait(cx)).await;
                if let Err(_e) = result {
                    #[cfg(feature = "tracing")]
                    tracing::error!("error while polling zombie process: {}", _e);
                }
            }
        };

        // Create a function for scheduling this future.
        let schedule = move |runnable| {
            reaper.sender.try_send(runnable).ok();
        };

        // Spawn the task and run it forever.
        let (runnable, task) = async_task::spawn(future, schedule);
        task.detach();
        runnable.schedule();
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        use async_io::Async;
        use rustix::process;
        use std::os::unix::io::OwnedFd;

        /// Waitable version of `std::process::Child`
        struct WaitableChild {
            child: std::process::Child,
            handle: Async<OwnedFd>,
        }

        impl WaitableChild {
            fn new(child: std::process::Child) -> io::Result<Self> {
                let pidfd = process::pidfd_open(
                    process::Pid::from_child(&child),
                    process::PidfdFlags::empty()
                )?;

                Ok(Self {
                    child,
                    handle: Async::new(pidfd)?
                })
            }

            fn get_mut(&mut self) -> &mut std::process::Child {
                &mut self.child
            }

            fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<std::process::ExitStatus>> {
                loop {
                    if let Some(status) = self.child.try_wait()? {
                        return Poll::Ready(Ok(status));
                    }

                    // Wait for us to become readable.
                    futures_lite::ready!(self.handle.poll_readable(cx))?;
                }
            }
        }

        /// Tell if we are able to use this backend.
        pub(crate) fn available() -> bool {
            // Create a Pidfd for the current process and see if it works.
            let result = process::pidfd_open(
                process::getpid(),
                process::PidfdFlags::empty()
            );

            // Tell if it was okay or not.
            result.is_ok()
        }
    } else if #[cfg(windows)] {
        use async_io::os::windows::Waitable;

        /// Waitable version of `std::process::Child`.
        struct WaitableChild {
            inner: Waitable<std::process::Child>,
        }

        impl WaitableChild {
            fn new(child: std::process::Child) -> io::Result<Self> {
                Ok(Self {
                    inner: Waitable::new(child)?
                })
            }

            fn get_mut(&mut self) -> &mut std::process::Child {
                // SAFETY: We never move the child out.
                unsafe {
                    self.inner.get_mut()
                }
            }

            fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<std::process::ExitStatus>> {
                loop {
                    if let Some(status) = self.get_mut().try_wait()? {
                        return Poll::Ready(Ok(status));
                    }

                    // Wait for us to become readable.
                    futures_lite::ready!(self.inner.poll_ready(cx))?;
                }
            }
        }

        /// Tell if we are able to use this backend.
        pub(crate) fn available() -> bool {
            true
        }
    }
}
