//! Unix-specific I/O extensions.

cfg_not_docs! {
    pub use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
    
    cfg_io_safety! {
        pub use std::os::unix::io::{AsFd, BorrowedFd, OwnedFd};
    }
}

cfg_docs! {
    /// Raw file descriptors.
    pub type RawFd = std::os::raw::c_int;

    /// A trait to extract the raw unix file descriptor from an underlying
    /// object.
    ///
    /// This is only available on unix platforms and must be imported in order
    /// to call the method. Windows platforms have a corresponding `AsRawHandle`
    /// and `AsRawSocket` set of traits.
    pub trait AsRawFd {
        /// Extracts the raw file descriptor.
        ///
        /// This method does **not** pass ownership of the raw file descriptor
        /// to the caller. The descriptor is only guaranteed to be valid while
        /// the original object has not yet been destroyed.
        fn as_raw_fd(&self) -> RawFd;
    }

    /// A trait to express the ability to construct an object from a raw file
    /// descriptor.
    pub trait FromRawFd {
        /// Constructs a new instance of `Self` from the given raw file
        /// descriptor.
        ///
        /// This function **consumes ownership** of the specified file
        /// descriptor. The returned object will take responsibility for closing
        /// it when the object goes out of scope.
        ///
        /// This function is also unsafe as the primitives currently returned
        /// have the contract that they are the sole owner of the file
        /// descriptor they are wrapping. Usage of this function could
        /// accidentally allow violating this contract which can cause memory
        /// unsafety in code that relies on it being true.
        unsafe fn from_raw_fd(fd: RawFd) -> Self;
    }

    /// A trait to express the ability to consume an object and acquire ownership of
    /// its raw file descriptor.
    pub trait IntoRawFd {
        /// Consumes this object, returning the raw underlying file descriptor.
        ///
        /// This function **transfers ownership** of the underlying file descriptor
        /// to the caller. Callers are then the unique owners of the file descriptor
        /// and must close the descriptor once it's no longer needed.
        fn into_raw_fd(self) -> RawFd;
    }

    cfg_io_safety! {
        #[doc(inline)]
        pub use std::os::unix::io::{AsFd, BorrowedFd, OwnedFd};
    }
}
