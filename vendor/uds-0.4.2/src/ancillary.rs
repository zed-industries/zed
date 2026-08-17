#![allow(
    clippy::unnecessary_mut_passed, // CMSG_ macros only exist for *const
    clippy::useless_conversion, // not useless on all platforms
    clippy::match_overlapping_arm, // cumbersome to avoid when using inclusive ranges
    clippy::borrow_deref_ref, // avoid infinite loop in Borrow impl
    dead_code // TODO
)]

use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, BorrowMut};
use std::os::unix::io::RawFd;
use std::io::{self, ErrorKind, IoSlice, IoSliceMut};
use std::alloc::{self, Layout};
use std::convert::TryInto;
use std::{mem, ptr, slice};
use std::marker::PhantomData;

use libc::{c_int, c_uint, c_void};
use libc::{msghdr, iovec, cmsghdr, sockaddr, sockaddr_un};
use libc::{sendmsg, recvmsg, close};
//#[cfg(not(any(target_os="illumos", target_os="solaris")))]
use libc::{MSG_TRUNC, MSG_CTRUNC};
#[cfg(not(any(target_os="illumos", target_os="solaris")))]
use libc::{CMSG_SPACE, CMSG_LEN, CMSG_DATA, CMSG_FIRSTHDR, CMSG_NXTHDR};
//#[cfg(not(any(target_os="illumos", target_os="solaris")))]
use libc::{SOL_SOCKET, SCM_RIGHTS};
#[cfg(any(target_os="linux", target_os="android"))]
use libc::SCM_CREDENTIALS;
#[cfg(not(any(target_vendor="apple", target_os="illumos", target_os="solaris", target_os = "haiku")))]
use libc::MSG_CMSG_CLOEXEC;

use crate::helpers::*;
use crate::UnixSocketAddr;
use crate::credentials::{SendCredentials, ReceivedCredentials};
#[cfg(any(target_os="linux", target_os="android"))]
use crate::credentials::RawReceivedCredentials;

// Type of cmsghdr.cmsg_len, which varies between OSes and C libraries
// (can't get away with using ` as _` because we use the max value in some places.)
// cfg based on libc 0.2.82 source code.
#[cfg(any(
    all(target_os="linux", not(target_env="musl")),
    target_os="android",
    target_env="uclibc",
))]
type ControlLen = usize;
#[cfg(not(any(
    all(target_os="linux", not(target_env="musl")),
    target_os="android",
    target_env="uclibc",
)))]
type ControlLen = libc::socklen_t;

/// Safe wrapper around `sendmsg()`.
pub fn send_ancillary(
    socket: RawFd,  to: Option<&UnixSocketAddr>,  flags: c_int,
    bytes: &[IoSlice],  fds: &[RawFd],  creds: Option<SendCredentials>
) -> Result<usize, io::Error> {
    #[cfg(not(any(target_os="linux", target_os="android")))]
    let _ = creds; // silence `unused` warning
    unsafe {
        let mut msg: msghdr = mem::zeroed();
        msg.msg_name = ptr::null_mut();
        msg.msg_namelen = 0;
        msg.msg_iov = bytes.as_ptr() as *mut iovec;
        msg.msg_iovlen = match bytes.len().try_into() {
            Ok(len) => len,
            Err(_) => {
                return Err(io::Error::new(ErrorKind::InvalidInput, "too many byte slices"));
            }
        };
        msg.msg_flags = 0;
        msg.msg_control = ptr::null_mut();
        msg.msg_controllen = 0;

        if let Some(addr) = to {
            let (addr, len) = addr.as_raw();
            msg.msg_name = addr as *const sockaddr_un as *const c_void as *mut c_void;
            msg.msg_namelen = len;
        }

        let mut needed_capacity = 0;
        #[cfg(any(target_os="linux", target_os="android"))]
        let creds = creds.map(|creds| {
            let creds = creds.into_raw();
            needed_capacity += CMSG_SPACE(mem::size_of_val(&creds) as u32);
            creds
        });
        if fds.len() > 0 {
            if fds.len() > 0xff_ff_ff {
                // need to prevent truncation.
                // I use a lower limit in case the macros don't handle overflow.
                return Err(io::Error::new(ErrorKind::InvalidInput, "too many file descriptors"));
            }
            #[cfg(not(any(target_os="illumos", target_os="solaris")))] {
                needed_capacity += CMSG_SPACE(mem::size_of_val::<[RawFd]>(fds) as u32);
            }
            #[cfg(any(target_os="illumos", target_os="solaris"))] {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    "ancillary data support is not implemented yet for Illumos or Solaris"
                ))
            }
        }
        // stack buffer which should be big enough for most scenarios
        #[repr(C)]
        struct AncillaryFixedBuf(/*for alignment*/[cmsghdr; 0], [u8; 256]);
        let mut ancillary_buf = AncillaryFixedBuf([], [0; 256]);

        msg.msg_controllen = needed_capacity as ControlLen;
        if needed_capacity != 0 {
            if needed_capacity as usize <= mem::size_of::<AncillaryFixedBuf>() {
                msg.msg_control = &mut ancillary_buf.1 as *mut [u8; 256] as *mut c_void;
            } else {
                let layout = Layout::from_size_align(
                    needed_capacity as usize,
                    mem::align_of::<cmsghdr>()
                ).unwrap();
                msg.msg_control = alloc::alloc(layout) as *mut c_void;
            }

            #[cfg(not(any(target_os="illumos", target_os="solaris")))] {
                let header_ptr = CMSG_FIRSTHDR(&mut msg);
                assert!(!header_ptr.is_null(), "CMSG_FIRSTHDR returned unexpected NULL pointer");
                #[allow(unused_mut)]
                let mut header = &mut*header_ptr;
                #[cfg(any(target_os="linux", target_os="android"))] {
                    if let Some(creds) = creds {
                        header.cmsg_level = SOL_SOCKET;
                        header.cmsg_type = SCM_CREDENTIALS;
                        header.cmsg_len = CMSG_LEN(mem::size_of_val(&creds) as u32) as ControlLen;
                        *(CMSG_DATA(header) as *mut c_void as *mut _) = creds;
                        let header_ptr = CMSG_NXTHDR(&mut msg, header);
                        assert!(!header_ptr.is_null(), "CMSG_NXTHDR returned unexpected NULL pointer");
                        header = &mut*header_ptr;
                    }
                }

                if fds.len() > 0 {
                    header.cmsg_level = SOL_SOCKET;
                    header.cmsg_type = SCM_RIGHTS;
                    header.cmsg_len = CMSG_LEN(mem::size_of_val(fds) as u32) as ControlLen;
                    let mut dst = CMSG_DATA(header) as *mut c_void as *mut RawFd;
                    for &fd in fds {
                        ptr::write_unaligned(dst, fd);
                        dst = dst.add(1);
                    }
                }
            }
        }

        let result = cvt_r!(sendmsg(socket, &msg, flags | MSG_NOSIGNAL));

        if needed_capacity as usize > mem::size_of::<AncillaryFixedBuf>() {
            let layout = Layout::from_size_align(needed_capacity as usize, mem::align_of::<cmsghdr>()).unwrap();
            alloc::dealloc(msg.msg_control as *mut u8, layout);
        }

        result.map(|sent| sent as usize )
    }
}



/// An ancillary data buffer that supports any capacity.
///
/// For reasonable ancillary capacities it uses a stack-based array.
#[repr(C)]
pub struct AncillaryBuf {
    capacity: ControlLen,
    ptr: *mut u8,
    _align: [cmsghdr; 0],
    on_stack: [u8; Self::MAX_STACK_CAPACITY],
}
impl Drop for AncillaryBuf {
    fn drop(&mut self) {
        unsafe {
            if self.capacity as usize > Self::MAX_STACK_CAPACITY {
                let layout = Layout::from_size_align(
                    self.capacity as usize,
                    mem::align_of::<cmsghdr>()
                ).unwrap();
                alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
impl AncillaryBuf {
    pub const MAX_STACK_CAPACITY: usize = 256;
    pub const MAX_CAPACITY: usize = ControlLen::max_value() as usize;
    pub fn with_capacity(bytes: usize) -> Self {
        Self {
            capacity: bytes as ControlLen,
            ptr: match bytes {
                0..=Self::MAX_STACK_CAPACITY => ptr::null_mut(),
                0..=Self::MAX_CAPACITY => unsafe {
                    let layout = Layout::from_size_align(
                        bytes as usize,
                        mem::align_of::<cmsghdr>()
                    ).unwrap();
                    alloc::alloc_zeroed(layout)
                },
                _ => panic!("capacity is too high"),
            },
            _align: [],
            on_stack: [0; Self::MAX_STACK_CAPACITY],
        }
    }
    pub fn with_fd_capacity(num_fds: usize) -> Self {
        #[cfg(not(any(target_os="illumos", target_os="solaris")))]
        unsafe {
            // To prevent truncation or overflow (in CMSG macros or elsewhere)
            // cmsghdr having bigger alignment than RawFd isn't a problem,
            //  as that doesn't affect the maximum capacity.
            // If the size of cmsghdr is not divisible by the size of RawFd
            //  (which could theoretically happen if all three cmsghdr fields
            //  are u16 or u8 somewher), then some bytes will not be usable.
            //  But dividing by the size of RawFd silently takes care of that
            //  as it rounds down.
            // FIXME This should ideally be a constant, but it's not really a
            //  problem. (libc doesn't have a const_fn feature, probably
            //  because old compilers wouldn't be able to even parse it.
            let max_fds =
                (c_uint::max_value() - CMSG_SPACE(0)) as usize
                / mem::size_of::<RawFd>();
            if num_fds == 0 {
                Self::with_capacity(0)
            } else if num_fds <= max_fds {
                let payload_bytes = num_fds * mem::size_of::<RawFd>();
                Self::with_capacity(CMSG_SPACE(payload_bytes as c_uint) as usize)
            } else {
                panic!("too many file descriptors for ancillary buffer length")
            }
        }
        #[cfg(any(target_os="illumos", target_os="solaris"))] {
            Self::with_capacity(num_fds) // any non-zero value is not supported
        }
    }
}
impl Default for AncillaryBuf {
    fn default() -> Self {
        Self {
            capacity: Self::MAX_STACK_CAPACITY as ControlLen,
            ptr: ptr::null_mut(),
            _align: [],
            on_stack: [0; Self::MAX_STACK_CAPACITY],
        }
    }
}

impl Deref for AncillaryBuf {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            self.on_stack.get(..self.capacity as usize)
                .unwrap_or_else(|| slice::from_raw_parts(self.ptr, self.capacity as usize) )
        }
    }
}
impl DerefMut for AncillaryBuf {
    fn deref_mut(&mut self) -> &mut[u8] {
        unsafe {
            match self.on_stack.get_mut(..self.capacity as usize) {
                Some(on_stack) => on_stack,
                None => slice::from_raw_parts_mut(self.ptr, self.capacity as usize)
            }
        }
    }
}
impl Borrow<[u8]> for AncillaryBuf {
    fn borrow(&self) -> &[u8] {
        &*self
    }
}
impl BorrowMut<[u8]> for AncillaryBuf {
    fn borrow_mut(&mut self) -> &mut[u8] {
        &mut*self
    }
}
impl AsRef<[u8]> for AncillaryBuf {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}
impl AsMut<[u8]> for AncillaryBuf {
    fn as_mut(&mut self) -> &mut[u8] {
        &mut*self
    }
}

pub struct FdSliceIterator<'a> {
    pos: usize,
    slice: &'a FdSlice<'a>,
}
impl<'a> Iterator for FdSliceIterator<'a> {
    type Item = RawFd;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.len > self.pos {
            // SAFETY: Safe as long as FdSlice is created correctly
            let ret = unsafe {
                self.slice.unaligned_ptr.add(self.pos).read_unaligned()
            };
            self.pos += 1;
            Some(ret)
        } else {
            None
        }
    }
}

pub struct FdSlice<'a> {
    /// The underlying buffer as ptr
    unaligned_ptr: *const RawFd,
    /// The amount of [`RawFd`]s in this [`FdSlice`]
    len: usize,
    /// The lifetime of the underlying buffer
    _borrow: PhantomData<&'a RawFd>,
}
impl<'a> FdSlice<'a> {
    /// Creates a new [`FdSlice`] with the lifetime from a `unaligned_ptr` and a `len`.
    ///
    /// # Safety
    /// The unaligned_ptr does not need to be properly aligned, but it needs to point to at least `len` [`RawFd`]s.
    /// The unaligned_ptr may not be null.
    unsafe fn new(unaligned_ptr: *const RawFd, len: usize) -> Self {
        debug_assert!(!unaligned_ptr.is_null(), "No NULL pointer for FdSlice");
        Self {
            unaligned_ptr,
            len,
            _borrow: PhantomData,
        }
    }

    /// The amount of [`RawFd`] in this [`FdSlice`]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns an iterator over the elements of this [`FdSlice`]
    pub fn iter(&self) -> FdSliceIterator {
        (&self).into_iter()
    }
}
impl<'a> IntoIterator for &'a FdSlice<'a> {
    type Item = RawFd;
    type IntoIter = FdSliceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FdSliceIterator {
            pos: 0,
            slice: self,
        }
    }
}

/// One ancillary message produced by [`Ancillary`](#struct.Ancillary)
pub enum AncillaryItem<'a> {
    /// One or more file descriptors sent by the peer.
    ///
    /// Consumer of the iterator is responsible for closing them.
    Fds(FdSlice<'a>),
    /// Credentials of the sending process.
    #[allow(unused)]
    Credentials(ReceivedCredentials),
    //Timestamp(),
    //SecurityContext(&'a[u8]),
    /// An unknown or unsupported ancillary message type was received.
    ///
    /// It's up to you whether to ignore or treat as an error.
    Unsupported
}

/// An iterator over ancillary messages received with `recv_ancillary()`.
pub struct Ancillary<'a> {
    // addr and bytes are not used here:
    // * addr is usually placed on the stack by the calling wrapper method,
    //   which means that its lifetime ends when this struct is returned.
    // * the iovec is incremented by Linux, but possibly not others.
    msg: msghdr,

    _ancillary_buf: PhantomData<&'a[u8]>,
    /// The next message, initialized with CMSG_FIRSTHDR()
    #[cfg(not(any(target_os="illumos", target_os="solaris")))]
    next_message: *mut cmsghdr,
}
impl<'a> Iterator for Ancillary<'a> {
    type Item = AncillaryItem<'a>;
    #[cfg(not(any(target_os="illumos", target_os="solaris")))]
    fn next(&mut self) -> Option<AncillaryItem<'a>> {
        unsafe {
            if self.next_message.is_null() {
                return None;
            }
            let msg_bytes = (*self.next_message).cmsg_len as usize;
            let payload_bytes = msg_bytes - CMSG_LEN(0) as usize;
            let item = match ((*self.next_message).cmsg_level, (*self.next_message).cmsg_type) {
                (SOL_SOCKET, SCM_RIGHTS) => {
                    let num_fds = payload_bytes / mem::size_of::<RawFd>();
                    // pointer is aligned due to the cmsg header
                    let first_fd = CMSG_DATA(self.next_message) as *const c_void;
                    let first_fd = first_fd.cast::<RawFd>();
                    #[cfg(any(target_vendor="apple", target_os="freebsd"))] {
                        // set cloexec
                        // This is necessary on FreeBSD as MSG_CMSG_CLOEXEC
                        // appears to have no effect.
                        // FIXME this should be done in a separate iteration
                        // when the fds are received, and not after user code
                        // has had a chance to run.
                        // SAFETY: It's safe to create FdSlice twice from valid values. The values are valid.
                        let fds = FdSlice::new(first_fd, num_fds);
                        for fd in &fds {
                            // might fail if fd has not been kept alive by the
                            // sender, so ignore errors.
                            let _ = set_cloexec(fd, true);
                        }
                    }
                    let fds = FdSlice::new(first_fd, num_fds);
                    AncillaryItem::Fds(fds)
                }
                #[cfg(any(target_os="linux", target_os="android"))]
                (SOL_SOCKET, SCM_CREDENTIALS) => {
                    // FIXME check payload size?
                    let creds_ptr = CMSG_DATA(self.next_message) as *const c_void;
                    debug_assert!(
                        creds_ptr as usize & (mem::align_of::<RawReceivedCredentials>()-1) == 0,
                        "CMSG_DATA() is aligned"
                    );
                    let creds_ptr = creds_ptr as *const RawReceivedCredentials;
                    AncillaryItem::Credentials(ReceivedCredentials::from_raw(*creds_ptr))
                }
                _ => AncillaryItem::Unsupported,
            };
            self.next_message = CMSG_NXTHDR(&mut self.msg, self.next_message);
            Some(item)
        }
    }
    #[cfg(any(target_os="illumos", target_os="solaris"))]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
impl<'a> Drop for Ancillary<'a> {
    fn drop(&mut self) {
        // close all remaining file descriptors
        for ancillary in self {
            if let AncillaryItem::Fds(fds) = ancillary {
                for fd in &fds {
                    unsafe { close(fd) };
                }
            }
        }
    }
}
impl<'a> Ancillary<'a> {
    /// Returns `true` if the non-ancillary part of the datagram or packet was truncated.
    ///
    /// If the provided byte buffer(s) are shorter than the datagram or packet
    /// that was sent, the bytes that couldn't be stored are discarded.
    ///
    /// This function is not meaningful for streams.
    pub fn message_truncated(&self) -> bool {
        self.msg.msg_flags & MSG_TRUNC != 0
    }
    /// Returns `true` if ancillary messages were dropped due to a too short ancillary buffer.
    #[allow(unused)] // type is not yet exposed
    pub fn ancillary_truncated(&self) -> bool {
        self.msg.msg_flags & MSG_CTRUNC != 0
    }
}

/// A safe (but incomplete) wrapper around `recvmsg()`.
pub fn recv_ancillary<'ancillary_buf>(
    socket: RawFd,  from: Option<&mut UnixSocketAddr>,  mut flags: c_int,
    bufs: &mut[IoSliceMut],  ancillary_buf: &'ancillary_buf mut[u8],
) -> Result<(usize, Ancillary<'ancillary_buf>), io::Error> {
    unsafe {
        let mut msg: msghdr = mem::zeroed();
        msg.msg_name = ptr::null_mut();
        msg.msg_namelen = 0;
        msg.msg_iov = bufs.as_mut_ptr() as *mut iovec;
        msg.msg_iovlen = match bufs.len().try_into() {
            Ok(len) => len,
            Err(_) => {
                return Err(io::Error::new(ErrorKind::InvalidInput, "too many content buffers"));
            }
        };
        msg.msg_flags = 0;
        msg.msg_control = ptr::null_mut();
        msg.msg_controllen = 0;

        if ancillary_buf.len() > 0 {
            #[cfg(any(target_os="illumos", target_os="solaris"))] {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    "ancillary message support is not implemented yet on Illumos or Solaris, sorry"
                ))
            }
            if ancillary_buf.as_ptr() as usize % mem::align_of::<cmsghdr>() != 0 {
                let msg = "ancillary buffer is not properly aligned";
                return Err(io::Error::new(ErrorKind::InvalidInput, msg));
            }
            if ancillary_buf.len() > ControlLen::max_value() as usize {
                let msg = "ancillary buffer is too big";
                return Err(io::Error::new(ErrorKind::InvalidInput, msg));
            }
            msg.msg_control = ancillary_buf.as_mut_ptr() as *mut c_void;
            msg.msg_controllen = ancillary_buf.len() as ControlLen;
        }
        flags |= MSG_NOSIGNAL;
        #[cfg(not(any(target_vendor="apple", target_os="illumos", target_os="solaris", target_os = "haiku")))] {
            flags |= MSG_CMSG_CLOEXEC;
        }

        let received = match from {
            Some(addrbuf) => {
                let (received, addr) = UnixSocketAddr::new_from_ffi(|addr, len| {
                    msg.msg_name = addr as *mut sockaddr as *mut c_void;
                    msg.msg_namelen = *len;
                    let received = cvt_r!(recvmsg(socket, &mut msg, flags))? as usize;
                    *len = msg.msg_namelen;
                    Ok(received)
                })?;
                *addrbuf = addr;
                received
            }
            None => cvt_r!(recvmsg(socket, &mut msg, flags))? as usize
        };

        let ancillary_iterator = Ancillary {
            msg,
            _ancillary_buf: PhantomData,
            #[cfg(not(any(target_os="illumos", target_os="solaris")))]
            next_message: CMSG_FIRSTHDR(&msg),
        };
        Ok((received, ancillary_iterator))
    }
}

pub fn recv_fds(
        fd: RawFd,  from: Option<&mut UnixSocketAddr>,
        bufs: &mut[IoSliceMut],  fd_buf: &mut[RawFd]
) -> Result<(usize, bool, usize), io::Error> {
    let mut ancillary_buf = AncillaryBuf::with_fd_capacity(fd_buf.len());
    let (num_bytes, mut ancillary) = recv_ancillary(fd, from, 0, bufs, &mut ancillary_buf)?;
    let mut num_fds = 0;
    for message in &mut ancillary {
        if let AncillaryItem::Fds(fds) = message {
            // Due to alignment of cmsg_len in glibc the minimum payload
            // capacity is on Linux (and probably Android) 8 bytes,
            // which means we might receive two file descriptors even though
            // we only want one.
            let can_keep = fds.len().min(fd_buf.len()-num_fds);
            let mut fd_iter = (&fds).iter();
            for i in 0..can_keep {
                fd_buf[num_fds + i] = fd_iter.next().unwrap();
            }
            num_fds += can_keep;
            // read the rest of fds
            for unwanted in fd_iter {
                unsafe { close(unwanted) };
            }
        }
    }
    Ok((num_bytes, ancillary.message_truncated(), num_fds))
}
