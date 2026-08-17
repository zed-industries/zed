mod pthread_interpose;

// Force this function to be linked, but it shouldn't actually be called by
// users directly as it interposes the libc `pthread_create`
#[doc(hidden)]
#[cfg(not(miri))]
pub use pthread_interpose::pthread_create;
