//! Windows-specific I/O extensions.

cfg_not_docs! {
    pub use std::os::windows::io::{
        AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle,
        AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket,
    };

    cfg_io_safety! {
        pub use std::os::windows::io::{
            AsHandle, BorrowedHandle, OwnedHandle,
            AsSocket, BorrowedSocket, OwnedSocket,
        };
    }
}

cfg_docs! {
    /// Raw HANDLEs.
    pub type RawHandle = *mut std::os::raw::c_void;

    /// Raw SOCKETs.
    pub type RawSocket = u64;

    /// Extracts raw handles.
    pub trait AsRawHandle {
        /// Extracts the raw handle, without taking any ownership.
        fn as_raw_handle(&self) -> RawHandle;
    }

    /// Construct I/O objects from raw handles.
    pub trait FromRawHandle {
        /// Constructs a new I/O object from the specified raw handle.
        ///
        /// This function will **consume ownership** of the handle given,
        /// passing responsibility for closing the handle to the returned
        /// object.
        ///
        /// This function is also unsafe as the primitives currently returned
        /// have the contract that they are the sole owner of the file
        /// descriptor they are wrapping. Usage of this function could
        /// accidentally allow violating this contract which can cause memory
        /// unsafety in code that relies on it being true.
        unsafe fn from_raw_handle(handle: RawHandle) -> Self;
    }

    /// A trait to express the ability to consume an object and acquire ownership of
    /// its raw `HANDLE`.
    pub trait IntoRawHandle {
        /// Consumes this object, returning the raw underlying handle.
        ///
        /// This function **transfers ownership** of the underlying handle to the
        /// caller. Callers are then the unique owners of the handle and must close
        /// it once it's no longer needed.
        fn into_raw_handle(self) -> RawHandle;
    }

    /// Creates I/O objects from raw sockets.
    pub trait FromRawSocket {
        /// Creates a new I/O object from the given raw socket.
        ///
        /// This function will consume ownership of the socket provided and it will be closed when the returned object goes out of scope.
        ///
        /// This function is also unsafe as the primitives currently returned have the contract that they are the sole owner of the
        /// file descriptor they are wrapping. Usage of this function could accidentally allow violating this contract which can cause
        /// memory unsafety in code that relies on it being true.
        unsafe fn from_raw_socket(sock: RawSocket) -> Self;
    }

    /// Extracts raw sockets.
    pub trait AsRawSocket {
        /// Extracts the underlying raw socket from this object.
        fn as_raw_socket(&self) -> RawSocket;
    }

    /// A trait to express the ability to consume an object and acquire ownership of
    /// its raw `SOCKET`.
    pub trait IntoRawSocket {
        /// Consumes this object, returning the raw underlying socket.
        ///
        /// This function **transfers ownership** of the underlying socket to the
        /// caller. Callers are then the unique owners of the socket and must close
        /// it once it's no longer needed.
        fn into_raw_socket(self) -> RawSocket;
    }

    cfg_io_safety! {
        #[doc(inline)]
        pub use std::os::windows::io::{
            AsHandle, BorrowedHandle, OwnedHandle,
            AsSocket, BorrowedSocket, OwnedSocket,
        };
    }
}
