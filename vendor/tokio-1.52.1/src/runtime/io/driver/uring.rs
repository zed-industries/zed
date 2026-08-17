use io_uring::{squeue::Entry, IoUring, Probe};
use mio::unix::SourceFd;
use slab::Slab;

use crate::runtime::driver::op::CancelData;
use crate::runtime::driver::op::CqeResult;
use crate::runtime::driver::op::{Cancellable, Lifecycle};
use crate::{io::Interest, loom::sync::Mutex};

use super::{Handle, TOKEN_WAKEUP};

use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::{io, mem, task::Waker};

const DEFAULT_RING_SIZE: u32 = 256;

pub(crate) struct UringContext {
    pub(crate) uring: Option<io_uring::IoUring>,
    pub(crate) ops: slab::Slab<Lifecycle>,
}

impl UringContext {
    pub(crate) fn new() -> Self {
        Self {
            ops: Slab::new(),
            uring: None,
        }
    }

    pub(crate) fn ring(&self) -> &io_uring::IoUring {
        self.uring.as_ref().expect("io_uring not initialized")
    }

    pub(crate) fn ring_mut(&mut self) -> &mut io_uring::IoUring {
        self.uring.as_mut().expect("io_uring not initialized")
    }

    /// Perform `io_uring_setup` system call, and Returns true if this
    /// actually initialized the io_uring.
    ///
    /// If the machine doesn't support io_uring, then this will return an
    /// `ENOSYS` error.
    pub(crate) fn try_init(&mut self, probe: &mut Probe) -> io::Result<bool> {
        if self.uring.is_some() {
            // Already initialized.
            return Ok(false);
        }

        let uring = IoUring::new(DEFAULT_RING_SIZE)?;

        match uring.submitter().register_probe(probe) {
            Ok(_) => {}
            Err(e) if e.raw_os_error() == Some(libc::EINVAL) => {
                // The kernel does not support IORING_REGISTER_PROBE.
                return Err(io::Error::from_raw_os_error(libc::ENOSYS));
            }
            Err(e) => return Err(e),
        }

        self.uring.replace(uring);

        Ok(true)
    }

    pub(crate) fn dispatch_completions(&mut self) {
        let ops = &mut self.ops;
        let Some(mut uring) = self.uring.take() else {
            // Uring is not initialized yet.
            return;
        };

        let cq = uring.completion();

        for cqe in cq {
            let idx = cqe.user_data() as usize;

            match ops.get_mut(idx) {
                Some(Lifecycle::Waiting(waker)) => {
                    waker.wake_by_ref();
                    *ops.get_mut(idx).unwrap() = Lifecycle::Completed(cqe);
                }
                Some(Lifecycle::Cancelled(cancel_data)) => {
                    if let CancelData::Open(_) = cancel_data {
                        if let Ok(fd) = CqeResult::from(cqe).result {
                            // SAFETY: the successful CQE result provides
                            // a non-negative integer, and the event is
                            // related to an open operation.
                            unsafe { OwnedFd::from_raw_fd(fd as i32) };
                        }
                    }
                    // Op future was cancelled, so we discard the result.
                    ops.remove(idx);
                }
                Some(other) => {
                    panic!("unexpected lifecycle for slot {idx}: {other:?}");
                }
                None => {
                    panic!("no op at index {idx}");
                }
            }
        }

        self.uring.replace(uring);

        // `cq`'s drop gets called here, updating the latest head pointer
    }

    pub(crate) fn submit(&mut self) -> io::Result<()> {
        loop {
            // Errors from io_uring_enter: https://man7.org/linux/man-pages/man2/io_uring_enter.2.html#ERRORS
            match self.ring().submit() {
                Ok(_) => {
                    return Ok(());
                }

                // If the submission queue is full, we dispatch completions and try again.
                Err(ref e) if e.raw_os_error() == Some(libc::EBUSY) => {
                    self.dispatch_completions();
                }
                // For other errors, we currently return the error as is.
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub(crate) fn remove_op(&mut self, index: usize) -> Lifecycle {
        self.ops.remove(index)
    }
}

/// Drop the driver, cancelling any in-progress ops and waiting for them to terminate.
impl Drop for UringContext {
    fn drop(&mut self) {
        if self.uring.is_none() {
            // Uring is not initialized or not supported.
            return;
        }

        // Make sure we flush the submission queue before dropping the driver.
        while !self.ring_mut().submission().is_empty() {
            self.submit().expect("Internal error when dropping driver");
        }

        let mut ops = std::mem::take(&mut self.ops);

        // Remove all completed ops since we don't need to wait for them.
        ops.retain(|_, lifecycle| !matches!(lifecycle, Lifecycle::Completed(_)));

        while !ops.is_empty() {
            // Wait until at least one completion is available.
            self.ring_mut()
                .submit_and_wait(1)
                .expect("Internal error when dropping driver");

            for cqe in self.ring_mut().completion() {
                let idx = cqe.user_data() as usize;

                if let Some(Lifecycle::Cancelled(CancelData::Open(_))) = ops.get_mut(idx) {
                    if let Ok(fd) = CqeResult::from(cqe).result {
                        // SAFETY: the successful CQE result provides
                        // a non-negative integer, and the event is
                        // related to an open operation.
                        unsafe { OwnedFd::from_raw_fd(fd as i32) };
                    }
                };

                ops.remove(idx);
            }
        }
    }
}

impl Handle {
    fn add_uring_source(&self, uringfd: RawFd) -> io::Result<()> {
        let mut source = SourceFd(&uringfd);
        self.registry
            .register(&mut source, TOKEN_WAKEUP, Interest::READABLE.to_mio())
    }

    pub(crate) fn get_uring(&self) -> &Mutex<UringContext> {
        &self.uring_context
    }

    /// Returns `true` if io_uring has already been initialized and the given
    /// opcode is supported. Returns `false` if io_uring hasn't been
    /// initialized yet or is unsupported. Unlike `check_and_init`, this
    /// doesn't attempt initialization.
    #[cfg_attr(test, allow(dead_code))]
    pub(crate) fn is_uring_ready(&self, opcode: u8) -> bool {
        self.uring_probe
            .get()
            .and_then(|opt| opt.as_ref())
            .is_some_and(|probe| probe.is_supported(opcode))
    }

    /// Returns `true` if the io_uring probe has already been attempted
    /// (regardless of whether io_uring is supported). Returns `false` if
    /// no probe has been attempted yet.
    #[cfg_attr(test, allow(dead_code))]
    pub(crate) fn is_uring_probed(&self) -> bool {
        self.uring_probe.get().is_some()
    }

    /// Check if the io_uring context is initialized. If not, it will try to initialize it.
    /// Then, check if the provided opcode is supported.
    ///
    /// If both the context initialization succeeds and the opcode is supported,
    /// this returns `Ok(true)`.
    /// If either io_uring is unsupported or the opcode is unsupported,
    /// this returns `Ok(false)`.
    /// An error is returned if an io_uring syscall returns an unexpected error value.
    ///
    /// TODO: This would like to be a synchronous function,
    /// but we require `OnceLock::get_or_try_init`.
    /// <https://github.com/rust-lang/rust/issues/109737>
    pub(crate) async fn check_and_init(&self, opcode: u8) -> io::Result<bool> {
        let probe = self
            .uring_probe
            .get_or_try_init(|| async {
                let mut probe = Probe::new();
                match self.try_init(&mut probe) {
                    Ok(()) => Ok(Some(probe)),
                    // If the system doesn't support io_uring, we set the probe to `None`.
                    Err(e) if e.raw_os_error() == Some(libc::ENOSYS) => Ok(None),
                    // If we get EPERM, io-uring syscalls may be blocked (for example, by seccomp).
                    // In this case, we try to fall back to spawn_blocking for this and future operations.
                    // See also: https://github.com/tokio-rs/tokio/issues/7691
                    Err(e) if e.raw_os_error() == Some(libc::EPERM) => Ok(None),
                    // For other system errors, we just return it.
                    Err(e) => Err(e),
                }
            })
            .await?;

        Ok(probe
            .as_ref()
            .is_some_and(|probe| probe.is_supported(opcode)))
    }

    /// Initialize the io_uring context if it hasn't been initialized yet.
    fn try_init(&self, probe: &mut Probe) -> io::Result<()> {
        let mut guard = self.get_uring().lock();
        if guard.try_init(probe)? {
            self.add_uring_source(guard.ring().as_raw_fd())?;
        }

        Ok(())
    }

    /// Register an operation with the io_uring.
    ///
    /// If this is the first io_uring operation, it will also initialize the io_uring context.
    /// If io_uring isn't supported, this function returns an `ENOSYS` error, so the caller can
    /// perform custom handling, such as falling back to an alternative mechanism.
    ///
    /// # Safety
    ///
    /// Callers must ensure that parameters of the entry (such as buffer) are valid and will
    /// be valid for the entire duration of the operation, otherwise it may cause memory problems.
    pub(crate) unsafe fn register_op(&self, entry: Entry, waker: Waker) -> io::Result<usize> {
        assert!(self.uring_probe.initialized());

        // Uring is initialized.

        let mut guard = self.get_uring().lock();
        let ctx = &mut *guard;
        let index = ctx.ops.insert(Lifecycle::Waiting(waker));
        let entry = entry.user_data(index as u64);

        let submit_or_remove = |ctx: &mut UringContext| -> io::Result<()> {
            if let Err(e) = ctx.submit() {
                // Submission failed, remove the entry from the slab and return the error
                ctx.remove_op(index);
                return Err(e);
            }
            Ok(())
        };

        // SAFETY: entry is valid for the entire duration of the operation
        while unsafe { ctx.ring_mut().submission().push(&entry).is_err() } {
            // If the submission queue is full, flush it to the kernel
            submit_or_remove(ctx)?;
        }

        // Ensure that the completion queue is not full before submitting the entry.
        while ctx.ring_mut().completion().is_full() {
            ctx.dispatch_completions();
        }

        // Note: For now, we submit the entry immediately without utilizing batching.
        submit_or_remove(ctx)?;

        Ok(index)
    }

    pub(crate) fn cancel_op<T: Cancellable>(&self, index: usize, data: Option<T>) {
        let mut guard = self.get_uring().lock();
        let ctx = &mut *guard;
        let ops = &mut ctx.ops;
        let Some(lifecycle) = ops.get_mut(index) else {
            // The corresponding index doesn't exist anymore, so this Op is already complete.
            return;
        };

        // This Op will be cancelled. Here, we don't remove the lifecycle from the slab to keep
        // uring data alive until the operation completes.

        let cancel_data = data.expect("Data should be present").cancel();
        match mem::replace(lifecycle, Lifecycle::Cancelled(cancel_data)) {
            Lifecycle::Submitted | Lifecycle::Waiting(_) => (),
            // The driver saw the completion, but it was never polled.
            Lifecycle::Completed(cqe) => {
                if let Lifecycle::Cancelled(CancelData::Open(_)) = lifecycle {
                    if let Ok(fd) = CqeResult::from(cqe).result {
                        // SAFETY: the successful CQE result provides
                        // a non-negative integer, and the event is
                        // related to an open operation.
                        unsafe { OwnedFd::from_raw_fd(fd as i32) };
                    }
                }
                // We can safely remove the entry from the slab, as it has already been completed.
                ops.remove(index);
            }
            prev => panic!("Unexpected state: {prev:?}"),
        };
    }
}
