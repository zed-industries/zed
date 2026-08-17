//! Typed views using temporary objects.
//!
//! This module defines the return types for [`AsFilelike::as_filelike_view`]
//! and [`AsSocketlike::as_socketlike_view`].
//!
//! [`AsSocketlike::as_socketlike_view`]: crate::AsSocketlike::as_socketlike_view

use crate::raw::{
    AsRawFilelike, AsRawSocketlike, FromRawFilelike, FromRawSocketlike, IntoRawFilelike,
    IntoRawSocketlike, RawFilelike, RawSocketlike,
};
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
use crate::OwnedFd;
use crate::{
    AsFilelike, AsSocketlike, FromFilelike, FromSocketlike, IntoFilelike, IntoSocketlike,
    OwnedFilelike, OwnedSocketlike,
};
#[cfg(windows)]
use crate::{OwnedHandle, OwnedSocket};
use std::fmt;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;

/// Declare that a type is safe to use in a [`FilelikeView`].
///
/// # Safety
///
/// Types implementing this trait declare that if they are constructed with
/// [`FromFilelike`] and consumed with [`IntoFilelike`], their `IntoFilelike`
/// will return the same `OwnedFd` value that was passed to their
/// `FromFilelike`.
pub unsafe trait FilelikeViewType: FromFilelike + IntoFilelike {}

/// Declare that a type is safe to use in a [`SocketlikeView`].
///
/// # Safety
///
/// Types implementing this trait declare that if they are constructed with
/// [`FromSocketlike`] and consumed with [`IntoSocketlike`], their
/// `IntoSocketlike` will return the same `OwnedFd` value that was passed to
/// their `FromSocketlike`.
pub unsafe trait SocketlikeViewType: FromSocketlike + IntoSocketlike {}

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsFilelike::as_filelike_view`].
pub struct FilelikeView<'filelike, Target: FilelikeViewType> {
    /// The value to dereference to. This is a `ManuallyDrop` so that we can
    /// consume it in our `Drop` impl.
    target: ManuallyDrop<Target>,

    /// `FilelikeViewType` implementors guarantee that their `Into<OwnedFd>`
    /// returns the same fd as their `From<OwnedFd>` gave them. This field
    /// allows us to verify this.
    #[cfg(debug_assertions)]
    orig: RawFilelike,

    /// This field exists because we don't otherwise explicitly use
    /// `'filelike`.
    _phantom: PhantomData<&'filelike OwnedFilelike>,
}

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsSocketlike::as_socketlike_view`].
pub struct SocketlikeView<'socketlike, Target: SocketlikeViewType> {
    /// The value to dereference to. This is a `ManuallyDrop` so that we can
    /// consume it in our `Drop` impl.
    target: ManuallyDrop<Target>,

    /// `SocketlikeViewType` implementors guarantee that their `Into<OwnedFd>`
    /// returns the same fd as their `From<OwnedFd>` gave them. This field
    /// allows us to verify this.
    #[cfg(debug_assertions)]
    orig: RawSocketlike,

    /// This field exists because we don't otherwise explicitly use
    /// `'socketlike`.
    _phantom: PhantomData<&'socketlike OwnedSocketlike>,
}

impl<Target: FilelikeViewType> FilelikeView<'_, Target> {
    /// Construct a temporary `Target` and wrap it in a `FilelikeView` object.
    #[inline]
    pub(crate) fn new<T: AsFilelike>(filelike: &T) -> Self {
        // Safety: The returned `FilelikeView` is scoped to the lifetime of
        // `filelike`, which we've borrowed here, so the view won't outlive
        // the object it's borrowed from.
        unsafe { Self::view_raw(filelike.as_filelike().as_raw_filelike()) }
    }

    /// Construct a temporary `Target` from raw and wrap it in a `FilelikeView`
    /// object.
    ///
    /// # Safety
    ///
    /// `raw` must be a valid raw filelike referencing a resource that outlives
    /// the resulting view.
    #[inline]
    pub unsafe fn view_raw(raw: RawFilelike) -> Self {
        let owned = OwnedFilelike::from_raw_filelike(raw);
        Self {
            target: ManuallyDrop::new(Target::from_filelike(owned)),
            #[cfg(debug_assertions)]
            orig: raw,
            _phantom: PhantomData,
        }
    }
}

impl<Target: SocketlikeViewType> SocketlikeView<'_, Target> {
    /// Construct a temporary `Target` and wrap it in a `SocketlikeView`
    /// object.
    #[inline]
    pub(crate) fn new<T: AsSocketlike>(socketlike: &T) -> Self {
        // Safety: The returned `SocketlikeView` is scoped to the lifetime of
        // `socketlike`, which we've borrowed here, so the view won't outlive
        // the object it's borrowed from.
        unsafe { Self::view_raw(socketlike.as_socketlike().as_raw_socketlike()) }
    }

    /// Construct a temporary `Target` from raw and wrap it in a
    /// `SocketlikeView` object.
    ///
    /// # Safety
    ///
    /// `raw` must be a valid raw socketlike referencing a resource that
    /// outlives the resulting view.
    #[inline]
    pub unsafe fn view_raw(raw: RawSocketlike) -> Self {
        let owned = OwnedSocketlike::from_raw_socketlike(raw);
        Self {
            target: ManuallyDrop::new(Target::from_socketlike(owned)),
            #[cfg(debug_assertions)]
            orig: raw,
            _phantom: PhantomData,
        }
    }
}

impl<Target: FilelikeViewType> Deref for FilelikeView<'_, Target> {
    type Target = Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl<Target: SocketlikeViewType> Deref for SocketlikeView<'_, Target> {
    type Target = Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl<Target: FilelikeViewType> Drop for FilelikeView<'_, Target> {
    #[inline]
    fn drop(&mut self) {
        // Use `Into*` to consume `self.target` without freeing its resource.
        //
        // Safety: Using `ManuallyDrop::take` requires us to ensure that
        // `self.target` is not used again. We don't use it again here, and
        // this is the `drop` function, so we know it's not used afterward.
        let _raw = unsafe { ManuallyDrop::take(&mut self.target) }
            .into_filelike()
            .into_raw_filelike();

        #[cfg(debug_assertions)]
        debug_assert_eq!(self.orig, _raw);
    }
}

impl<Target: SocketlikeViewType> Drop for SocketlikeView<'_, Target> {
    #[inline]
    fn drop(&mut self) {
        // Use `Into*` to consume `self.target` without freeing its resource.
        //
        // Safety: Using `ManuallyDrop::take` requires us to ensure that
        // `self.target` is not used again. We don't use it again here, and
        // this is the `drop` function, so we know it's not used afterward.
        let _raw = unsafe { ManuallyDrop::take(&mut self.target) }
            .into_socketlike()
            .into_raw_socketlike();

        #[cfg(debug_assertions)]
        debug_assert_eq!(self.orig, _raw);
    }
}

impl<Target: FilelikeViewType> fmt::Debug for FilelikeView<'_, Target> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilelikeView")
            .field("target", &*self)
            .finish()
    }
}

impl<Target: SocketlikeViewType> fmt::Debug for SocketlikeView<'_, Target> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SocketlikeView")
            .field("target", &*self)
            .finish()
    }
}

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
unsafe impl FilelikeViewType for OwnedFd {}
#[cfg(windows)]
unsafe impl FilelikeViewType for OwnedHandle {}
#[cfg(windows)]
unsafe impl SocketlikeViewType for OwnedSocket {}
unsafe impl FilelikeViewType for std::fs::File {}
unsafe impl SocketlikeViewType for std::net::TcpStream {}
unsafe impl SocketlikeViewType for std::net::TcpListener {}
unsafe impl SocketlikeViewType for std::net::UdpSocket {}
#[cfg(unix)]
unsafe impl SocketlikeViewType for std::os::unix::net::UnixStream {}
#[cfg(unix)]
unsafe impl SocketlikeViewType for std::os::unix::net::UnixListener {}

#[cfg(unix)]
unsafe impl SocketlikeViewType for std::os::unix::net::UnixDatagram {}
#[cfg(not(any(target_os = "wasi", target_os = "hermit")))]
#[cfg(feature = "os_pipe")]
unsafe impl FilelikeViewType for os_pipe::PipeWriter {}
#[cfg(not(any(target_os = "wasi", target_os = "hermit")))]
#[cfg(feature = "os_pipe")]
unsafe impl FilelikeViewType for os_pipe::PipeReader {}

#[cfg(not(any(target_os = "wasi", target_os = "hermit")))]
#[cfg(feature = "socket2")]
unsafe impl SocketlikeViewType for socket2::Socket {}

#[cfg(not(any(target_os = "wasi", target_os = "hermit")))]
#[cfg(feature = "async-std")]
unsafe impl SocketlikeViewType for async_std::net::TcpStream {}
#[cfg(not(any(target_os = "wasi", target_os = "hermit")))]
#[cfg(feature = "async-std")]
unsafe impl SocketlikeViewType for async_std::net::TcpListener {}
#[cfg(not(any(target_os = "wasi", target_os = "hermit")))]
#[cfg(feature = "async-std")]
unsafe impl SocketlikeViewType for async_std::net::UdpSocket {}
#[cfg(unix)]
#[cfg(feature = "async-std")]
unsafe impl SocketlikeViewType for async_std::os::unix::net::UnixStream {}
#[cfg(unix)]
#[cfg(feature = "async-std")]
unsafe impl SocketlikeViewType for async_std::os::unix::net::UnixListener {}
#[cfg(unix)]
#[cfg(feature = "async-std")]
unsafe impl SocketlikeViewType for async_std::os::unix::net::UnixDatagram {}

#[cfg(feature = "mio")]
unsafe impl SocketlikeViewType for mio::net::TcpStream {}
#[cfg(feature = "mio")]
unsafe impl SocketlikeViewType for mio::net::TcpListener {}
#[cfg(feature = "mio")]
unsafe impl SocketlikeViewType for mio::net::UdpSocket {}
#[cfg(unix)]
#[cfg(feature = "mio")]
unsafe impl SocketlikeViewType for mio::net::UnixDatagram {}
#[cfg(unix)]
#[cfg(feature = "mio")]
unsafe impl SocketlikeViewType for mio::net::UnixListener {}
#[cfg(unix)]
#[cfg(feature = "mio")]
unsafe impl SocketlikeViewType for mio::net::UnixStream {}
#[cfg(unix)]
#[cfg(feature = "mio")]
unsafe impl FilelikeViewType for mio::unix::pipe::Receiver {}
#[cfg(unix)]
#[cfg(feature = "mio")]
unsafe impl FilelikeViewType for mio::unix::pipe::Sender {}
