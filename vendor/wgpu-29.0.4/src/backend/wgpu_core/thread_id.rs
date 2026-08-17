//! Implementation of thread IDs for error scope tracking.
//!
//! Supports both std and no_std environments, though
//! the no_std implementation is a stub that does not
//! actually distinguish between threads.

#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(std::thread::ThreadId);

#[cfg(feature = "std")]
impl ThreadId {
    pub fn current() -> Self {
        ThreadId(std::thread::current().id())
    }
}

#[cfg(not(feature = "std"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(());

#[cfg(not(feature = "std"))]
impl ThreadId {
    pub fn current() -> Self {
        // A simple stub implementation for non-std environments. On
        // no_std but multithreaded platforms, this will work, but
        // make error scope global rather than thread-local.
        ThreadId(())
    }
}
