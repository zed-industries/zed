use crate::error::{self, ProtocolError};
use crate::event::{self, Event};
use crate::ext::{Extension, ExtensionData};
#[cfg(feature = "present")]
use crate::present;
use crate::x::{Atom, Keysym, Setup, Timestamp};
#[cfg(feature = "xinput")]
use crate::xinput;
use crate::{cache_extensions_data, ffi::*};

#[cfg(feature = "xlib_xcb")]
use x11::xlib;

use bitflags::bitflags;

use libc::{c_char, c_int};

use std::cell::RefCell;
use std::ffi::{c_void, CStr, CString};
use std::fmt::{self, Display, Formatter};
use std::marker::{self, PhantomData};
use std::mem;
use std::os::fd::{IntoRawFd, OwnedFd};
use std::os::unix::prelude::{AsRawFd, RawFd};
use std::ptr;
use std::result;
use std::slice;

/// A X resource trait
pub trait Xid {
    /// Build a null X resource
    fn none() -> Self;

    /// Get the underlying id of the resource
    fn resource_id(&self) -> u32;

    /// Check whether this resource is null or not
    fn is_none(&self) -> bool {
        self.resource_id() == 0
    }
}

/// Trait for X resources that can be created directly from `Connection::generate_id`
///
/// The resources that cannot be created that way are the Xid unions, which are created
/// from their underlying resource.
pub trait XidNew: Xid {
    /// Build a new X resource
    ///
    /// # Safety
    /// res_id must be obtained from `xcb_generate_id`. `0` is also a valid value to create a null resource.
    /// Users should not use this function directly but rather use
    /// `Connection::generate_id`
    unsafe fn new(res_id: u32) -> Self;
}

/// Trait for types that own a C allocated pointer and are represented by the data pointed to.
pub trait Raw<T>: Sized {
    /// Build `Self` from a raw pointer
    ///
    /// # Safety
    /// `raw` must be a valid pointer to the representation of Self, and be allocated with `libc::malloc`
    unsafe fn from_raw(raw: *mut T) -> Self;

    /// Convert self into a raw pointer
    ///
    /// Returned value should be freed with `libc::free` or sent back to `from_raw` to avoid memory leak.
    fn into_raw(self) -> *mut T {
        let raw = self.as_raw();
        mem::forget(self);
        raw
    }

    /// Obtain the raw pointer representation
    fn as_raw(&self) -> *mut T;
}

/// Trait for base events (aka. non GE_GENERIC events)
pub trait BaseEvent: Raw<xcb_generic_event_t> {
    /// The extension associated to this event, or `None` for the main protocol
    const EXTENSION: Option<Extension>;

    /// The number associated to this event
    const NUMBER: u32;

    /// Check whether this event was emitted by the SendEvent request
    /// See `[crate::x::SendEvent]`.
    fn is_from_send_event(&self) -> bool {
        let ev = self.as_raw();
        let response_type = unsafe { (*ev).response_type };
        (response_type & 0x80) != 0
    }
}

/// A trait for GE_GENERIC events
///
/// A GE_GENERIC event is an extension event that does not follow
/// the regular `response_type` offset.
/// This system was introduced because the protocol eventually run
/// out of event numbers.
///
/// This should be completely transparent to the user, as [Event] is
/// resolving all types of events together.
pub trait GeEvent: Raw<xcb_ge_generic_event_t> {
    /// The extension associated to this event
    const EXTENSION: Extension;

    /// The number associated to this event
    const NUMBER: u32;
}

/// A trait to designate base protocol errors.
///
/// The base errors follow the usual resolution idiom of `error_code` offset.
///
/// This should be completely transparent to the user, as [ProtocolError] is
/// resolving all types of errors together.
pub trait BaseError: Raw<xcb_generic_error_t> {
    /// The extension associated to this error, or `None` for the main protocol
    const EXTENSION: Option<Extension>;

    /// The number associated to this error
    const NUMBER: u32;
}

/// Trait for the resolution of raw wire event to a unified event enum.
///
/// `Self` is normally an enum of several event subtypes.
/// See [crate::x::Event] and [crate::Event]
pub(crate) trait ResolveWireEvent: Sized {
    /// Resolve a pointer to `xcb_generic_event_t` to `Self`, inferring the correct subtype
    /// using `response_type` field and `first_event`
    ///
    /// # Safety
    /// `event` must be a valid, non-null event returned by `xcb_wait_for_event`
    /// or similar function
    unsafe fn resolve_wire_event(first_event: u8, event: *mut xcb_generic_event_t) -> Option<Self>;
}

/// Trait for the resolution of raw wire GE_GENERIC event to a unified event enum.
///
/// `Self` is normally an enum of several event subtypes.
/// See [crate::xinput::Event] and [crate::Event]
pub(crate) trait ResolveWireGeEvent: Sized {
    /// Resolve a pointer to `xcb_ge_generic_event_t` to `Self`, inferring the correct subtype
    /// using `event_type` field.
    ///
    /// # Panics
    /// Panics if the event subtype cannot be resolved for `Self`. That is, `event_type`
    /// must be checked beforehand to be in range.
    ///
    /// # Safety
    /// `event` must be a valid, non-null event returned by `xcb_wait_for_event`
    /// or similar function
    unsafe fn resolve_wire_ge_event(event: *mut xcb_ge_generic_event_t) -> Self;
}

/// Trait for the resolution of raw wire error to a unified error enum.
///
/// `Self` is normally an enum of several event subtypes.
/// See [crate::x::Error] and [crate::ProtocolError]
pub(crate) trait ResolveWireError: Sized {
    /// Convert a pointer to `xcb_generic_error_t` to `Self`, inferring the correct subtype
    /// using `response_type` field and `first_error`.
    ///
    /// # Safety
    /// `err` must be a valid, non-null error obtained by `xcb_wait_for_reply`
    /// or similar function
    unsafe fn resolve_wire_error(first_error: u8, error: *mut xcb_generic_error_t) -> Option<Self>;
}

/// Trait for types that can serialize themselves to the X wire.
///
/// This trait is used internally for requests serialization.
pub(crate) trait WiredOut {
    /// Compute the length of wired serialized data of self
    fn wire_len(&self) -> usize;

    /// Serialize `self` over the X wire and returns how many bytes were written.
    ///
    /// `wire_buf` must be larger or as long as the value returned by `wire_len`.
    /// `serialize` MUST write the data in its entirety at once, at the begining of `wire_buf`.
    /// That is, it returns the same value as `wire_len`.
    /// The interest in returning the value is that it is easy to compute in `serialize` and allow
    /// to easily chain serialization of fields in a struct.
    ///
    /// # Panics
    /// Panics if `wire_buf` is too small to contain the serialized representation of `self`.
    fn serialize(&self, wire_buf: &mut [u8]) -> usize;
}

/// Trait for types that can unserialize themselves from the X wire.
pub(crate) trait WiredIn {
    /// type of external context necessary to figure out the representation of the data
    type Params: Copy;

    /// Compute the length of serialized data of an instance starting by `ptr`.
    ///
    /// # Safety
    /// This function is highly unsafe as the pointer must point to data that is a valid
    /// wired representation of `Self`. Failure to respect this will lead to dereferencing invalid memory.
    unsafe fn compute_wire_len(ptr: *const u8, params: Self::Params) -> usize;

    /// Unserialize an instance of `Self` from the X wire
    ///
    /// `offset` value is increased by the number of bytes corresponding to the representation of `Self`.
    /// This allows for efficient chaining of unserialization as the data offset is either known at
    /// compile time, or has to be computed anyway.
    ///
    /// # Safety
    /// This function is highly unsafe as the pointer must point to data that is a valid
    /// wired representation of `Self`. Failure to respect this will lead to dereferencing invalid memory.
    unsafe fn unserialize(ptr: *const u8, params: Self::Params, offset: &mut usize) -> Self;
}

macro_rules! impl_wired_simple {
    ($typ:ty) => {
        impl WiredOut for $typ {
            fn wire_len(&self) -> usize {
                mem::size_of::<Self>()
            }

            fn serialize(&self, wire_buf: &mut [u8]) -> usize {
                debug_assert!(wire_buf.len() >= mem::size_of::<Self>());
                let buf_size = self.wire_len();
                let src =
                    unsafe { slice::from_raw_parts(self as *const Self as *const u8, buf_size) };
                wire_buf[..buf_size].copy_from_slice(src);
                buf_size
            }
        }

        impl WiredIn for $typ {
            type Params = ();

            unsafe fn compute_wire_len(_ptr: *const u8, _params: Self::Params) -> usize {
                mem::size_of::<Self>()
            }

            unsafe fn unserialize(
                ptr: *const u8,
                _params: Self::Params,
                offset: &mut usize,
            ) -> Self {
                *offset += mem::size_of::<Self>();
                *(ptr as *const Self)
            }
        }
    };
}

impl_wired_simple!(u8);
impl_wired_simple!(u16);
impl_wired_simple!(u32);
impl_wired_simple!(u64);
impl_wired_simple!(i8);
impl_wired_simple!(i16);
impl_wired_simple!(i32);
impl_wired_simple!(f32);

impl<T: Xid> WiredOut for T {
    fn wire_len(&self) -> usize {
        4
    }

    fn serialize(&self, wire_buf: &mut [u8]) -> usize {
        debug_assert!(wire_buf.len() >= 4);
        let buf_size = self.wire_len();
        let resource_id = self.resource_id();
        let src =
            unsafe { slice::from_raw_parts(&resource_id as *const u32 as *const u8, buf_size) };
        wire_buf[..buf_size].copy_from_slice(src);
        buf_size
    }
}

impl<T: XidNew> WiredIn for T {
    type Params = ();

    unsafe fn compute_wire_len(_ptr: *const u8, _params: Self::Params) -> usize {
        4
    }

    unsafe fn unserialize(ptr: *const u8, _params: Self::Params, offset: &mut usize) -> Self {
        *offset += 4;
        let xid = *(ptr as *const u32);
        T::new(xid)
    }
}

/// Trait for request replies
pub trait Reply {
    /// Build the reply struct from a raw pointer.
    ///
    /// # Safety
    /// `raw` must be a pointer to a valid wire representation of `Self`, allocated with [`libc::malloc`].
    unsafe fn from_raw(raw: *const u8) -> Self;

    /// Consume the reply struct into a raw pointer.
    ///
    /// # Safety
    /// The returned pointer must be freed with [`libc::free`] to avoid any memory leak, or be used
    /// to build another reply.
    unsafe fn into_raw(self) -> *const u8;

    /// Get the raw pointer of the XCB reply.
    ///
    /// # Safety
    /// The returned pointer must NOT be freed. Passing this pointer to [`libc::free`] will result in a double free
    /// when the reply is dropped.
    unsafe fn as_raw(&self) -> *const u8;
}

/// General trait for cookies returned by requests.
pub trait Cookie {
    /// # Safety
    /// `seq` must be a valid cookie for a given `Request` or `Reply`.
    unsafe fn from_sequence(seq: u64) -> Self;

    /// The raw sequence number associated with the cookie.
    fn sequence(&self) -> u64;
}

/// A marker trait for a cookie that allows synchronized error checking.
///
/// # Safety
/// Cookies not implementing this trait acknowledge that the error is sent to the event loop
///
/// See also [Connection::send_request], [Connection::send_request_checked] and [Connection::check_request]
pub unsafe trait CookieChecked: Cookie {}

/// A trait for checked cookies of requests that send a reply.
///
/// # Safety
/// Cookies implementing this trait acknowledge that their error is checked when the reply is fetched from the server.
/// This is the default cookie type for requests with reply.
///
/// See also [Connection::send_request], [Connection::wait_for_reply]
pub unsafe trait CookieWithReplyChecked: CookieChecked {
    /// The reply type associated with the cookie
    type Reply: Reply;
}

/// A trait for unchecked cookies of requests that send a reply.
///
/// # Safety
/// Cookies implementing this trait acknowledge that their error is not checked when the reply is fetched from the server
/// but in the event loop.
///
/// See also [Connection::send_request_unchecked], [Connection::wait_for_event]
pub unsafe trait CookieWithReplyUnchecked: Cookie {
    /// The reply type associated with the cookie
    type Reply: Reply;
}

/// The default cookie type returned by void-requests.
///
/// See [Connection::send_request]
#[derive(Debug)]
pub struct VoidCookie {
    seq: u64,
}

impl Cookie for VoidCookie {
    unsafe fn from_sequence(seq: u64) -> Self {
        VoidCookie { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

/// The checked cookie type returned by void-requests.
///
/// See [Connection::send_request_checked]
#[derive(Debug)]
pub struct VoidCookieChecked {
    seq: u64,
}

impl Cookie for VoidCookieChecked {
    unsafe fn from_sequence(seq: u64) -> Self {
        VoidCookieChecked { seq }
    }

    fn sequence(&self) -> u64 {
        self.seq
    }
}

unsafe impl CookieChecked for VoidCookieChecked {}

/// Trait implemented by all requests to send the serialized data over the wire.
///
/// # Safety
/// Types implementing this trait acknowledge that the returned value of `raw_request` correspond
/// to a cookie for `Self` request and is checked or unchecked depending on the `checked` flag value.
pub unsafe trait RawRequest {
    /// Actual implementation of the request sending
    ///
    /// Send the request over the `conn` wire and return a cookie sequence fitting with the `checked` flag
    /// of `Self`
    fn raw_request(&self, conn: &Connection, checked: bool) -> u64;
}

/// Trait implemented by requests types.
///
/// See [crate::x::CreateWindow] as an example.
pub trait Request: RawRequest {
    /// The default cookie associated to this request.
    type Cookie: Cookie;

    /// `false` if the request returns a reply, `true` otherwise.
    const IS_VOID: bool;
}

/// Marker trait for requests that do not return a reply.
///
/// These trait is implicitely associated with [`VoidCookie`] and [`VoidCookieChecked`].
pub trait RequestWithoutReply: Request {}

/// Trait for requests that return a reply.
pub trait RequestWithReply: Request {
    /// Reply associated with the request
    type Reply: Reply;
    /// Default cookie type for the request, as returned by [Connection::send_request].
    type Cookie: CookieWithReplyChecked<Reply = Self::Reply>;
    /// Unchecked cookie type for the request, as returned by [Connection::send_request_unchecked].
    type CookieUnchecked: CookieWithReplyUnchecked<Reply = Self::Reply>;
}

/// Determines whether Xlib or XCB owns the event queue of [`Connection`].
///
/// This item is behind the `xlib_xcb` cargo feature.
///
/// See [`Connection::set_event_queue_owner`].
#[cfg(feature = "xlib_xcb")]
#[derive(Debug)]
pub enum EventQueueOwner {
    /// XCB owns the event queue
    Xcb,
    /// Xlib owns the event queue
    Xlib,
}

/// Container for authentication information to connect to the X server
///
/// See [Connection::connect_to_display_with_auth_info] and [Connection::connect_to_fd].
#[derive(Copy, Clone, Debug)]
pub struct AuthInfo<'a> {
    /// String containing the authentication protocol name,
    /// such as "MIT-MAGIC-COOKIE-1" or "XDM-AUTHORIZATION-1".
    pub name: &'a str,
    /// data interpreted in a protocol specific manner
    pub data: &'a str,
}

/// Display info returned by [`parse_display`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DisplayInfo {
    /// The hostname
    pub host: String,
    /// The display number
    pub display: i32,
    /// The screen number
    pub screen: i32,
}

/// Parses a display string in the form documented by [X (7x)](https://linux.die.net/man/7/x).
///
/// Returns `Some(DisplayInfo)` on success and `None` otherwise.
/// Has no side effects on failure.
///
/// If `name` empty, it uses the environment variable `DISPLAY`.
///
/// If `name` does not contain a screen number, `DisplayInfo::screen` is set to `0`.
///
/// # Example
/// ```
/// use xcb::{DisplayInfo, parse_display};
///
/// assert_eq!(parse_display(":0"), Some(DisplayInfo {
///     host: "".to_string(),
///     display: 0,
///     screen: 0,
/// }));
/// assert_eq!(parse_display("localhost:0.1"), Some(DisplayInfo {
///     host: "localhost".to_string(),
///     display: 0,
///     screen: 1,
/// }));
///
/// assert!(parse_display("0").is_none());
/// ```
pub fn parse_display(name: &str) -> Option<DisplayInfo> {
    let name = CString::new(name).unwrap();
    let mut hostp: *mut c_char = ptr::null_mut();
    let mut display = 0i32;
    let mut screen = 0i32;

    let success = unsafe {
        xcb_parse_display(
            name.as_ptr(),
            &mut hostp as *mut _,
            &mut display as *mut _,
            &mut screen as *mut _,
        )
    };

    if success != 0 {
        let host = unsafe { CStr::from_ptr(hostp as *const _) }
            .to_str()
            .unwrap()
            .to_string();

        unsafe {
            libc::free(hostp as *mut _);
        }

        Some(DisplayInfo {
            host,
            display,
            screen,
        })
    } else {
        None
    }
}

/// A struct that serve as an identifier for internal special queue in XCB
///
/// See [Connection::register_for_special_xge].
#[deprecated(note = "Broken API: use `SpecialEvent` instead")]
#[cfg(any(feature = "xinput", feature = "present"))]
#[derive(Debug)]
pub struct SpecialEventId {
    raw: *mut xcb_special_event_t,
    stamp: Timestamp,
}

#[allow(deprecated)]
#[cfg(any(feature = "xinput", feature = "present"))]
impl SpecialEventId {
    /// The X timestamp associated with this special event Id
    pub fn stamp(&self) -> Timestamp {
        self.stamp
    }
}

/// A struct that serve as an identifier for internal special queue in XCB
///
/// See [Connection::register_for_special_event].
#[cfg(any(feature = "xinput", feature = "present"))]
#[derive(Debug)]
pub struct SpecialEvent {
    raw: *mut xcb_special_event_t,
}

// safe because XCB is thread safe.
#[cfg(any(feature = "xinput", feature = "present"))]
unsafe impl Send for SpecialEvent {}

#[cfg(any(feature = "xinput", feature = "present"))]
unsafe impl Sync for SpecialEvent {}

/// Error type that is returned by `Connection::has_error`.
#[derive(Debug)]
pub enum ConnError {
    /// xcb connection errors because of socket, pipe and other stream errors.
    Connection,
    /// xcb connection shutdown because of extension not supported
    ClosedExtNotSupported,
    /// malloc(), calloc() and realloc() error upon failure, for eg ENOMEM
    ClosedMemInsufficient,
    /// Connection closed, exceeding request length that server accepts.
    ClosedReqLenExceed,
    /// Connection closed, error during parsing display string.
    ClosedParseErr,
    /// Connection closed because the server does not have a screen
    /// matching the display.
    ClosedInvalidScreen,
    /// Connection closed because some file descriptor passing operation failed.
    ClosedFdPassingFailed,
    /// XOpenDisplay returned NULL
    #[cfg(feature = "xlib_xcb")]
    XOpenDisplay,
}

impl ConnError {
    fn to_str(&self) -> &str {
        match *self {
            ConnError::Connection => "Connection error, possible I/O error",
            ConnError::ClosedExtNotSupported => "Connection closed, X extension not supported",
            ConnError::ClosedMemInsufficient => "Connection closed, insufficient memory",
            ConnError::ClosedReqLenExceed => {
                "Connection closed, exceeded request length that server accepts."
            }
            ConnError::ClosedParseErr => "Connection closed, error during parsing display string",
            ConnError::ClosedInvalidScreen => {
                "Connection closed, the server does not have a screen matching the display"
            }
            ConnError::ClosedFdPassingFailed => {
                "Connection closed, some file descriptor passing operation failed"
            }
            #[cfg(feature = "xlib_xcb")]
            ConnError::XOpenDisplay => {
                "XOpenDisplay failed to open a display. Check the $DISPLAY env var"
            }
        }
    }
}

impl Display for ConnError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.to_str().fmt(f)
    }
}

impl std::error::Error for ConnError {
    fn description(&self) -> &str {
        self.to_str()
    }
}

/// The result type associated with [ConnError].
pub type ConnResult<T> = result::Result<T, ConnError>;

/// The result type associated with [ProtocolError].
pub type ProtocolResult<T> = result::Result<T, ProtocolError>;

/// The general error type for Rust-XCB.
#[derive(Debug)]
pub enum Error {
    /// I/O error issued from the connection.
    Connection(ConnError),
    /// A protocol related error issued by the X server.
    Protocol(ProtocolError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Connection(_) => f.write_str("xcb connection error"),
            Error::Protocol(_) => f.write_str("xcb protocol error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Connection(err) => Some(err),
            Error::Protocol(err) => Some(err),
        }
    }
}

impl From<ConnError> for Error {
    fn from(err: ConnError) -> Error {
        Error::Connection(err)
    }
}

impl From<ProtocolError> for Error {
    fn from(err: ProtocolError) -> Error {
        Error::Protocol(err)
    }
}

/// The general result type for Rust-XCB.
pub type Result<T> = result::Result<T, Error>;

/// `Connection` is the central object of XCB.
///
/// It handles all communications with the X server.
/// It dispatches the requests, receives the replies, poll/wait the events.
/// It also resolves the errors and events from X server.
///
/// `Connection` is thread safe.
///
/// It internally wraps an `xcb_connection_t` object and
/// will call `xcb_disconnect` when the `Connection` goes out of scope.
pub struct Connection {
    c: *mut xcb_connection_t,

    #[cfg(feature = "xlib_xcb")]
    dpy: *mut xlib::Display,

    ext_data: Vec<ExtensionData>,

    // Following field is used to handle the
    // rare (if existing) cases of multiple connections
    // per application.
    // Only the first established connections is used
    // to print the name of atoms during Debug
    #[cfg(feature = "debug_atom_names")]
    dbg_atom_names: bool,
}

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

impl Connection {
    /// Connects to the X server.
    ///
    /// Connects to the X server specified by `display_name.` If
    /// `display_name` is `None,` uses the value of the `DISPLAY` environment
    /// variable.
    ///
    /// If no screen is preferred, the second member of the tuple is set to 0.
    ///
    /// # Example
    /// ```no_run
    /// fn main() -> xcb::Result<()> {
    ///     let (conn, screen) = xcb::Connection::connect(None)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect(display_name: Option<&str>) -> ConnResult<(Connection, i32)> {
        Self::connect_with_extensions(display_name, &[], &[])
    }

    /// Connects to the X server and cache extension data.
    ///
    /// Connects to the X server specified by `display_name.` If
    /// `display_name` is `None,` uses the value of the `DISPLAY` environment
    /// variable.
    ///
    /// Extension data specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// If no screen is preferred, the second member of the tuple is set to 0.
    ///
    /// # Panics
    /// Panics if one of the mandatory extension is not present.
    ///
    /// # Example
    /// ```no_run
    /// fn main() -> xcb::Result<()> {
    ///     let (conn, screen) = xcb::Connection::connect_with_extensions(
    ///         None, &[xcb::Extension::Input, xcb::Extension::Xkb], &[]
    ///     )?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect_with_extensions(
        display_name: Option<&str>,
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> ConnResult<(Connection, i32)> {
        let mut screen_num: c_int = 0;
        let displayname = display_name.map(|s| CString::new(s).unwrap());
        unsafe {
            let conn = if let Some(display) = displayname {
                xcb_connect(display.as_ptr(), &mut screen_num)
            } else {
                xcb_connect(ptr::null(), &mut screen_num)
            };

            check_connection_error(conn)?;

            let conn = Self::from_raw_conn_and_extensions(conn, mandatory, optional);
            conn.has_error().map(|_| (conn, screen_num as i32))
        }
    }

    /// Open a new connection with Xlib.
    ///
    /// The event queue owner defaults to Xlib.
    /// One would need to open an XCB connection with Xlib in order to use
    /// OpenGL.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    #[cfg(feature = "xlib_xcb")]
    pub fn connect_with_xlib_display() -> ConnResult<(Connection, i32)> {
        unsafe {
            let dpy = xlib::XOpenDisplay(ptr::null());
            if dpy.is_null() {
                return Err(ConnError::XOpenDisplay);
            }

            check_connection_error(XGetXCBConnection(dpy))?;

            let conn = Self::from_xlib_display(dpy);

            conn.has_error()
                .map(|_| (conn, xlib::XDefaultScreen(dpy) as i32))
        }
    }

    /// Open a new connection with Xlib and cache the provided extensions data.
    ///
    /// Data of extensions specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// The event queue owner defaults to Xlib.
    /// One would need to open an XCB connection with Xlib in order to use
    /// OpenGL.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    ///
    /// # Panics
    /// Panics if one of the mandatory extension is not present.
    #[cfg(feature = "xlib_xcb")]
    pub fn connect_with_xlib_display_and_extensions(
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> ConnResult<(Connection, i32)> {
        unsafe {
            let dpy = xlib::XOpenDisplay(ptr::null());
            if dpy.is_null() {
                return Err(ConnError::XOpenDisplay);
            }

            check_connection_error(XGetXCBConnection(dpy))?;

            let conn = Self::from_xlib_display_and_extensions(dpy, mandatory, optional);

            conn.has_error()
                .map(|_| (conn, xlib::XDefaultScreen(dpy) as i32))
        }
    }

    /// Connects to the X server with an open socket file descriptor and optional authentification info.
    ///
    /// Connects to an X server, given the open socket fd and the
    /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
    /// If the connection should be unauthenticated, `auth_info` must be `None`.
    #[deprecated(note = "unsound API, use `connect_with_fd` instead")]
    #[doc(hidden)]
    pub fn connect_to_fd(fd: RawFd, auth_info: Option<AuthInfo<'_>>) -> ConnResult<Self> {
        #[allow(deprecated)]
        Self::connect_to_fd_with_extensions(fd, auth_info, &[], &[])
    }

    /// Connects to the X server with an open socket file descriptor and optional authentification info.
    ///
    /// Extension data specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// Connects to an X server, given the open socket fd and the
    /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
    /// If the connection should be unauthenticated, `auth_info` must be `None`.
    ///
    /// # Panics
    /// Panics if one of the mandatory extension is not present.
    #[deprecated(note = "unsound API, use `connect_with_fd_and_extensions` instead")]
    #[doc(hidden)]
    pub fn connect_to_fd_with_extensions(
        fd: RawFd,
        auth_info: Option<AuthInfo<'_>>,
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> ConnResult<Self> {
        let mut auth_info = auth_info.map(|auth_info| {
            let auth_name = CString::new(auth_info.name).unwrap();
            let auth_data = CString::new(auth_info.data).unwrap();

            let auth_info = xcb_auth_info_t {
                namelen: auth_name.as_bytes().len() as _,
                name: auth_name.as_ptr() as *mut _,
                datalen: auth_data.as_bytes().len() as _,
                data: auth_data.as_ptr() as *mut _,
            };
            // return the strings too otherwise they would drop
            (auth_info, auth_name, auth_data)
        });

        let ai_ptr = if let Some(auth_info) = auth_info.as_mut() {
            &mut auth_info.0 as *mut _
        } else {
            ptr::null_mut()
        };

        let conn = unsafe {
            let conn = xcb_connect_to_fd(fd, ai_ptr);
            check_connection_error(conn)?;

            Self::from_raw_conn_and_extensions(conn, mandatory, optional)
        };

        conn.has_error().map(|_| conn)
    }

    /// Connects to the X server with an open socket file descriptor and optional authentification info.
    ///
    /// Connects to an X server, given the open socket fd and the
    /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
    /// If the connection should be unauthenticated, `auth_info` must be `None`.
    pub fn connect_with_fd(fd: OwnedFd, auth_info: Option<AuthInfo<'_>>) -> ConnResult<Self> {
        Self::connect_with_fd_and_extensions(fd, auth_info, &[], &[])
    }

    /// Connects to the X server with an open socket file descriptor and optional authentification info.
    ///
    /// Extension data specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// Connects to an X server, given the open socket fd and the
    /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
    /// If the connection should be unauthenticated, `auth_info` must be `None`.
    ///
    /// # Panics
    /// Panics if one of the mandatory extension is not present.
    pub fn connect_with_fd_and_extensions(
        fd: OwnedFd,
        auth_info: Option<AuthInfo<'_>>,
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> ConnResult<Self> {
        let mut auth_info = auth_info.map(|auth_info| {
            let auth_name = CString::new(auth_info.name).unwrap();
            let auth_data = CString::new(auth_info.data).unwrap();

            let auth_info = xcb_auth_info_t {
                namelen: auth_name.as_bytes().len() as _,
                name: auth_name.as_ptr() as *mut _,
                datalen: auth_data.as_bytes().len() as _,
                data: auth_data.as_ptr() as *mut _,
            };
            // return the strings too otherwise they would drop
            (auth_info, auth_name, auth_data)
        });

        let ai_ptr = if let Some(auth_info) = auth_info.as_mut() {
            &mut auth_info.0 as *mut _
        } else {
            ptr::null_mut()
        };

        let fd = fd.into_raw_fd();

        let conn = unsafe {
            let conn = xcb_connect_to_fd(fd, ai_ptr);
            check_connection_error(conn)?;

            Self::from_raw_conn_and_extensions(conn, mandatory, optional)
        };

        conn.has_error().map(|_| conn)
    }

    /// Connects to the X server, using an authorization information.
    ///
    /// Connects to the X server specified by `display_name`, using the
    /// authorization `auth_info`. If a particular screen on that server, it is
    /// returned in the second tuple member, which is otherwise set to `0`.
    pub fn connect_to_display_with_auth_info(
        display_name: Option<&str>,
        auth_info: AuthInfo<'_>,
    ) -> ConnResult<(Connection, i32)> {
        Self::connect_to_display_with_auth_info_and_extensions(display_name, auth_info, &[], &[])
    }

    /// Connects to the X server, using an authorization information.
    ///
    /// Extension data specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// Connects to the X server specified by `display_name`, using the
    /// authorization `auth_info`. If a particular screen on that server, it is
    /// returned in the second tuple member, which is otherwise set to `0`.
    ///
    /// # Panics
    /// Panics if one of the mandatory extension is not present.
    pub fn connect_to_display_with_auth_info_and_extensions(
        display_name: Option<&str>,
        auth_info: AuthInfo<'_>,
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> ConnResult<(Connection, i32)> {
        let mut screen_num: c_int = 0;
        let display_name = display_name.map(|s| CString::new(s).unwrap());

        unsafe {
            let display_name = if let Some(display_name) = &display_name {
                display_name.as_ptr()
            } else {
                ptr::null()
            };

            let auth_name = CString::new(auth_info.name).unwrap();
            let auth_data = CString::new(auth_info.data).unwrap();

            let mut auth_info = xcb_auth_info_t {
                namelen: auth_name.as_bytes().len() as _,
                name: auth_name.as_ptr() as *mut _,
                datalen: auth_data.as_bytes().len() as _,
                data: auth_data.as_ptr() as *mut _,
            };

            let conn = xcb_connect_to_display_with_auth_info(
                display_name,
                &mut auth_info as *mut _,
                &mut screen_num as *mut _,
            );

            check_connection_error(conn)?;

            let conn = Self::from_raw_conn_and_extensions(conn, mandatory, optional);
            conn.has_error().map(|_| (conn, screen_num as i32))
        }
    }

    /// builds a new Connection object from an available connection
    ///
    /// # Safety
    /// The `conn` pointer must point to a valid `xcb_connection_t`
    pub unsafe fn from_raw_conn(conn: *mut xcb_connection_t) -> Connection {
        Self::from_raw_conn_and_extensions(conn, &[], &[])
    }

    /// Builds a new `Connection` object from an available connection and cache the extension data
    ///
    /// Extension data specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// # Panics
    /// Panics if the connection is null or in error state.
    /// Panics if one of the mandatory extension is not present.
    ///
    /// # Safety
    /// The `conn` pointer must point to a valid `xcb_connection_t`
    pub unsafe fn from_raw_conn_and_extensions(
        conn: *mut xcb_connection_t,
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> Connection {
        assert!(!conn.is_null());
        assert!(check_connection_error(conn).is_ok());

        #[cfg(feature = "debug_atom_names")]
        let dbg_atom_names = {
            if dan::DAN_CONN.is_null() {
                dan::DAN_CONN = conn;
                true
            } else {
                false
            }
        };

        let ext_data = cache_extensions_data(conn, mandatory, optional);

        #[cfg(not(feature = "xlib_xcb"))]
        #[cfg(not(feature = "debug_atom_names"))]
        return Connection { c: conn, ext_data };

        #[cfg(not(feature = "xlib_xcb"))]
        #[cfg(feature = "debug_atom_names")]
        return Connection {
            c: conn,
            ext_data,
            dbg_atom_names,
        };

        #[cfg(feature = "xlib_xcb")]
        #[cfg(not(feature = "debug_atom_names"))]
        return Connection {
            c: conn,
            dpy: ptr::null_mut(),
            ext_data,
        };

        #[cfg(feature = "xlib_xcb")]
        #[cfg(feature = "debug_atom_names")]
        return Connection {
            c: conn,
            dpy: ptr::null_mut(),
            ext_data,
            dbg_atom_names,
        };
    }

    /// Initialize a new `Connection` from an existing Xlib display.
    ///
    /// Wraps a `xlib::Display` and get an XCB connection from an exisiting object
    /// `xlib::XCloseDisplay` will be called when the returned object is dropped.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    ///
    /// # Safety
    /// The `dpy` pointer must be a pointer to a valid `xlib::Display`
    #[cfg(feature = "xlib_xcb")]
    pub unsafe fn from_xlib_display(dpy: *mut xlib::Display) -> Connection {
        Self::from_xlib_display_and_extensions(dpy, &[], &[])
    }

    /// Initialize a new `Connection` from an existing Xlib display.
    ///
    /// Wraps a `xlib::Display` and get an XCB connection from an exisiting object
    /// `xlib::XCloseDisplay` will be called when the returned object is dropped.
    ///
    /// Extension data specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    ///
    /// # Panics
    /// Panics if the connection is null or in error state.
    ///
    /// # Safety
    /// The `dpy` pointer must be a pointer to a valid `xlib::Display`.
    #[cfg(feature = "xlib_xcb")]
    pub unsafe fn from_xlib_display_and_extensions(
        dpy: *mut xlib::Display,
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> Connection {
        assert!(!dpy.is_null(), "attempt connect with null display");
        let c = XGetXCBConnection(dpy);

        assert!(check_connection_error(c).is_ok());

        #[cfg(feature = "debug_atom_names")]
        let dbg_atom_names = {
            if dan::DAN_CONN.is_null() {
                dan::DAN_CONN = c;
                true
            } else {
                false
            }
        };

        let ext_data = cache_extensions_data(c, mandatory, optional);

        #[cfg(feature = "debug_atom_names")]
        return Connection {
            c,
            dpy,
            ext_data,
            dbg_atom_names,
        };

        #[cfg(not(feature = "debug_atom_names"))]
        return Connection { c, dpy, ext_data };
    }

    /// Get the extensions activated for this connection.
    ///
    /// You may use this to check if an optional extension is present or not.
    ///
    /// # Example
    /// ```no_run
    /// # fn main() -> xcb::Result<()> {
    ///     // Xkb is mandatory, Input is optional
    ///     let (conn, screen) = xcb::Connection::connect_with_extensions(
    ///         None, &[xcb::Extension::Xkb], &[xcb::Extension::Input]
    ///     )?;
    ///     // now we check if Input is present or not
    ///     let has_input_ext = conn.active_extensions().any(|e| e == xcb::Extension::Input);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn active_extensions(&self) -> impl Iterator<Item = Extension> + '_ {
        self.ext_data.iter().map(|eed| eed.ext)
    }

    /// Get the data of the extensions activated for this connection.
    ///
    /// You may use this to manually resolve an event or an error with
    /// `xcb::event::resolve_event` or `xcb::error::resolve_error`.
    pub fn active_extensions_data(&self) -> &[ExtensionData] {
        &self.ext_data
    }

    /// Returns the inner ffi `xcb_connection_t` pointer
    pub fn get_raw_conn(&self) -> *mut xcb_connection_t {
        self.c
    }

    /// Consumes this object, returning the inner ffi `xcb_connection_t` pointer
    pub fn into_raw_conn(self) -> *mut xcb_connection_t {
        let c = self.c;
        mem::forget(self);
        c
    }

    /// Returns the inner ffi `xlib::Display` pointer.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    #[cfg(feature = "xlib_xcb")]
    pub fn get_raw_dpy(&self) -> *mut xlib::Display {
        self.dpy
    }

    /// Sets the owner of the event queue in the case if the connection is opened
    /// with the Xlib interface. In that case, the default owner is Xlib.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    #[cfg(feature = "xlib_xcb")]
    pub fn set_event_queue_owner(&self, owner: EventQueueOwner) {
        debug_assert!(!self.dpy.is_null());
        unsafe {
            XSetEventQueueOwner(
                self.dpy,
                match owner {
                    EventQueueOwner::Xcb => XCBOwnsEventQueue,
                    EventQueueOwner::Xlib => XlibOwnsEventQueue,
                },
            );
        }
    }

    /// Returns the maximum request length that this server accepts.
    ///
    /// In the absence of the BIG-REQUESTS extension, returns the
    /// maximum request length field from the connection setup data, which
    /// may be as much as 65535. If the server supports BIG-REQUESTS, then
    /// the maximum request length field from the reply to the
    /// BigRequestsEnable request will be returned instead.
    ///
    /// Note that this length is measured in four-byte units, making the
    /// theoretical maximum lengths roughly 256kB without BIG-REQUESTS and
    /// 16GB with.
    pub fn get_maximum_request_length(&self) -> u32 {
        unsafe { xcb_get_maximum_request_length(self.c) }
    }

    /// Prefetch the maximum request length without blocking.
    ///
    /// Without blocking, does as much work as possible toward computing
    /// the maximum request length accepted by the X server.
    ///
    /// Invoking this function may send the [crate::bigreq::Enable] request,
    /// but will not block waiting for the reply.
    /// [Connection::get_maximum_request_length] will return the prefetched data
    /// after possibly blocking while the reply is retrieved.
    ///
    /// Note that in order for this function to be fully non-blocking, the
    /// application must previously have called [crate::bigreq::prefetch_extension_data].
    pub fn prefetch_maximum_request_length(&self) {
        unsafe {
            xcb_prefetch_maximum_request_length(self.c);
        }
    }

    /// Allocates an XID for a new object.
    ///
    /// Returned value is typically used in requests such as `CreateWindow`.
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// # let conn = xcb::Connection::connect(None)?.0;
    /// let window: x::Window = conn.generate_id();
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate_id<T: XidNew>(&self) -> T {
        unsafe { XidNew::new(xcb_generate_id(self.c)) }
    }

    /// Forces any buffered output to be written to the server.
    ///
    /// Forces any buffered output to be written to the server. Blocks
    /// until the write is complete.
    ///
    /// There are several occasions ones want to flush the connection.
    /// One of them is before entering or re-entering the event loop after performing unchecked requests.
    ///
    /// The main difference between `flush` and `check_request` is that `flush` will not report protocol errors.
    /// If a protocol error is emitted by an unchecked void request, it will be reported through the event loop.
    ///
    /// See also: [wait_for_event](Connection::wait_for_event), [check_request](Connection::check_request),
    /// [send_and_check_request](Connection::send_and_check_request).
    pub fn flush(&self) -> ConnResult<()> {
        unsafe {
            let ret = xcb_flush(self.c);
            if ret > 0 {
                Ok(())
            } else {
                self.has_error()?;
                unreachable!()
            }
        }
    }

    /// Resolve an xcb_generic_event_t pointer into an Event.
    /// # Safety
    /// The caller is repsonsible to ensure that the `ev` pointer is not NULL.
    /// The ownership of the pointer is effectively transferred to the
    /// returned Event and it will be destroyed when the Event is
    /// dropped.
    pub unsafe fn resolve_event(&self, ev: &mut xcb_generic_event_t) -> Event {
        event::resolve_event(ev, &self.ext_data)
    }

    /// Resolve an xcb_generic_error_t pointer into an Error.
    /// # Safety
    /// The caller is repsonsible to ensure that the `err` pointer is not NULL.
    /// The ownership of the pointer is effectively transferred to the
    /// returned Error and it will be destroyed when the Error is
    /// dropped.
    pub unsafe fn resolve_error(&self, err: &mut xcb_generic_error_t) -> error::ProtocolError {
        error::resolve_error(err, &self.ext_data)
    }

    /// Blocks and returns the next event or error from the server.
    ///
    /// # Example
    /// ```no_run
    /// use xcb::x;
    /// fn main() -> xcb::Result<()> {
    /// #   let conn = xcb::Connection::connect(None)?.0;
    ///     // ...
    ///     loop {
    ///         let event = match conn.wait_for_event() {
    ///             Err(xcb::Error::Connection(err)) => {
    ///                 panic!("unexpected I/O error: {}", err);
    ///             }
    ///             Err(xcb::Error::Protocol(xcb::ProtocolError::X(x::Error::Font(err), _req_name))) => {
    ///                 // may be this particular error is fine?
    ///                 continue;
    ///             }
    ///             Err(xcb::Error::Protocol(err)) => {
    ///                 panic!("unexpected protocol error: {:#?}", err);
    ///             }
    ///             Ok(event) => event,
    ///         };
    ///         match event {
    ///             xcb::Event::X(x::Event::KeyPress(ev)) => {
    ///                 // do stuff with the key press
    ///             }
    ///             // handle other events
    ///             _ => {
    ///                 break Ok(());
    ///             }
    ///         }
    ///     }
    ///  }
    /// ```
    pub fn wait_for_event(&self) -> Result<Event> {
        unsafe {
            let ev = xcb_wait_for_event(self.c);
            self.handle_wait_for_event(ev)
        }
    }

    /// Returns the next event or error from the server without blocking.
    ///
    /// Returns the next event or error from the server, if one is
    /// available. If no event is available, that
    /// might be because an I/O error like connection close occurred while
    /// attempting to read the next event, in which case the connection is
    /// shut down when this function returns.
    pub fn poll_for_event(&self) -> Result<Option<Event>> {
        unsafe {
            let ev = xcb_poll_for_event(self.c);
            self.handle_poll_for_event(ev)
        }
    }

    /// Returns the next event without reading from the connection.
    ///
    /// This is a version of [Connection::poll_for_event] that only examines the
    /// event queue for new events. The function doesn't try to read new
    /// events from the connection if no queued events are found.
    ///
    /// This function is useful for callers that know in advance that all
    /// interesting events have already been read from the connection. For
    /// example, callers might use [Connection::wait_for_reply] and be interested
    /// only of events that preceded a specific reply.
    pub fn poll_for_queued_event(&self) -> ProtocolResult<Option<Event>> {
        unsafe {
            let ev = xcb_poll_for_queued_event(self.c);
            if ev.is_null() {
                Ok(None)
            } else if is_error(ev) {
                Err(error::resolve_error(ev as *mut _, &self.ext_data))
            } else {
                Ok(Some(event::resolve_event(ev, &self.ext_data)))
            }
        }
    }

    /// Start listening for a special event.
    ///
    /// Effectively creates an internal special queue for this event
    /// XGE events are only defined in the `xinput` and `present` extensions
    ///
    /// This function is present only if either of the `xinput` or `present` cargo features are active.
    #[deprecated(note = "Broken API: use `register_for_special_event` instead")]
    #[cfg(any(feature = "xinput", feature = "present"))]
    #[allow(deprecated)]
    pub fn register_for_special_xge<XGE: GeEvent>(&self) -> SpecialEventId {
        unsafe {
            let ext: *mut xcb_extension_t = match XGE::EXTENSION {
                #[cfg(feature = "xinput")]
                Extension::Input => ptr::addr_of_mut!(xinput::FFI_EXT),
                #[cfg(feature = "present")]
                Extension::Present => ptr::addr_of_mut!(present::FFI_EXT),
                _ => unreachable!("only Input and Present have XGE events"),
            };

            let mut stamp: Timestamp = 0;

            let raw = xcb_register_for_special_xge(self.c, ext, XGE::NUMBER, &mut stamp as *mut _);

            SpecialEventId { raw, stamp }
        }
    }

    /// Stop listening to a special event
    #[deprecated(note = "use `unregister_for_special_event` instead")]
    #[cfg(any(feature = "xinput", feature = "present"))]
    #[allow(deprecated)]
    pub fn unregister_for_special_xge(&self, se: SpecialEventId) {
        unsafe {
            xcb_unregister_for_special_event(self.c, se.raw);
        }
    }

    /// Returns the next event from a special queue, blocking until one arrives
    #[deprecated(note = "Broken API: use `wait_for_special_event2` instead")]
    #[cfg(any(feature = "xinput", feature = "present"))]
    #[allow(deprecated)]
    pub fn wait_for_special_event(&self, se: SpecialEventId) -> Result<Event> {
        unsafe {
            let ev = xcb_wait_for_special_event(self.c, se.raw);
            self.handle_wait_for_event(ev)
        }
    }

    /// Returns the next event from a special queue
    #[deprecated(note = "Broken API: use `poll_for_special_event2` instead")]
    #[cfg(any(feature = "xinput", feature = "present"))]
    #[allow(deprecated)]
    pub fn poll_for_special_event(&self, se: SpecialEventId) -> Result<Option<Event>> {
        unsafe {
            let ev = xcb_poll_for_special_event(self.c, se.raw);
            self.handle_poll_for_event(ev)
        }
    }

    /// Start listening for a special event.
    ///
    /// Effectively creates an internal special queue for this event
    /// XGE events are only defined in the `xinput` and `present` extensions
    ///
    /// This function is present only if either of the `xinput` or `present` cargo features are active.
    #[cfg(any(feature = "xinput", feature = "present"))]
    pub fn register_for_special_event<EID: Xid>(
        &self,
        extension: Extension,
        eid: EID,
    ) -> SpecialEvent {
        unsafe {
            let ext: *mut xcb_extension_t = match extension {
                #[cfg(feature = "xinput")]
                Extension::Input => ptr::addr_of_mut!(xinput::FFI_EXT),
                #[cfg(feature = "present")]
                Extension::Present => ptr::addr_of_mut!(present::FFI_EXT),
                _ => unreachable!("only Input and Present have XGE events"),
            };

            let raw = xcb_register_for_special_xge(self.c, ext, eid.resource_id(), ptr::null_mut());

            SpecialEvent { raw }
        }
    }

    /// Stop listening to a special event
    #[cfg(any(feature = "xinput", feature = "present"))]
    pub fn unregister_for_special_event(&self, se: SpecialEvent) {
        unsafe {
            xcb_unregister_for_special_event(self.c, se.raw);
        }
    }

    /// Returns the next event from a special queue, blocking until one arrives
    #[cfg(any(feature = "xinput", feature = "present"))]
    pub fn wait_for_special_event2(&self, se: &SpecialEvent) -> Result<Event> {
        unsafe {
            let ev = xcb_wait_for_special_event(self.c, se.raw);
            self.handle_wait_for_event(ev)
        }
    }

    /// Returns the next event from a special queue
    #[cfg(any(feature = "xinput", feature = "present"))]
    pub fn poll_for_special_event2(&self, se: &SpecialEvent) -> Result<Option<Event>> {
        unsafe {
            let ev = xcb_poll_for_special_event(self.c, se.raw);
            self.handle_poll_for_event(ev)
        }
    }

    /// Discards the reply for a request.
    ///
    /// Discards the reply for a request. Additionally, any error generated
    /// by the request is also discarded (unless it was an _unchecked request
    /// and the error has already arrived).
    ///
    /// This function will not block even if the reply is not yet available.
    fn discard_reply<C: Cookie>(&self, cookie: C) {
        unsafe {
            xcb_discard_reply64(self.c, cookie.sequence());
        }
    }

    /// Access the data returned by the server.
    ///
    /// Accessor for the data returned by the server when the connection
    /// was initialized. This data includes
    /// - the server's required format for images,
    /// - a list of available visuals,
    /// - a list of available screens,
    /// - the server's maximum request length (in the absence of the
    /// BIG-REQUESTS extension),
    /// - and other assorted information.
    ///
    /// See the X protocol specification for more details.
    pub fn get_setup(&self) -> &Setup {
        unsafe {
            let ptr = xcb_get_setup(self.c);
            // let len = <&Setup as WiredIn>::compute_wire_len(ptr, ());
            let mut _offset = 0;
            <&Setup as WiredIn>::unserialize(ptr, (), &mut _offset)
        }
    }

    /// Test whether the connection has shut down due to a fatal error.
    ///
    /// Some errors that occur in the context of a connection
    /// are unrecoverable. When such an error occurs, the
    /// connection is shut down and further operations on the
    /// connection have no effect.
    pub fn has_error(&self) -> ConnResult<()> {
        unsafe { check_connection_error(self.c) }
    }

    /// Send a request to the X server.
    ///
    /// This function never blocks. A cookie is returned to keep track of the request.
    /// If the request expect a reply, the cookie can be used to retrieve the reply with
    /// [Connection::wait_for_reply].
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
    /// #   let setup = conn.get_setup();
    /// #   let screen = setup.roots().nth(screen_num as usize).unwrap();
    /// #   let window: x::Window = conn.generate_id();
    ///     // Example of void request.
    ///     // Error (if any) will be sent to the event loop (see `wait_for_event`).
    ///     // In this case, the cookie can be discarded.
    ///     conn.send_request(&x::CreateWindow {
    ///         depth: x::COPY_FROM_PARENT as u8,
    ///         wid: window,
    ///         parent: screen.root(),
    ///         x: 0,
    ///         y: 0,
    ///         width: 150,
    ///         height: 150,
    ///         border_width: 10,
    ///         class: x::WindowClass::InputOutput,
    ///         visual: screen.root_visual(),
    ///         value_list: &[
    ///             x::Cw::BackPixel(screen.white_pixel()),
    ///             x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
    ///         ],
    ///     });
    ///
    ///     // Example of request with reply. The error (if any) is obtained with the reply.
    ///     let cookie = conn.send_request(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_PROTOCOLS",
    ///     });
    ///     let wm_protocols_atom: x::Atom = conn
    ///             .wait_for_reply(cookie)?
    ///             .atom();
    /// #   Ok(())
    /// # }
    /// ```
    pub fn send_request<R>(&self, req: &R) -> R::Cookie
    where
        R: Request,
    {
        unsafe { R::Cookie::from_sequence(req.raw_request(self, !R::IS_VOID)) }
    }

    /// Send a checked request to the X server.
    ///
    /// Checked requests do not expect a reply, but the returned cookie can be used to check for
    /// errors using `Connection::check_request`.
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
    /// #   let window: x::Window = conn.generate_id();
    ///     let cookie = conn.send_request_checked(&x::MapWindow { window });
    ///     conn.check_request(cookie)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn send_request_checked<R>(&self, req: &R) -> VoidCookieChecked
    where
        R: RequestWithoutReply,
    {
        unsafe { VoidCookieChecked::from_sequence(req.raw_request(self, true)) }
    }

    /// Send an unchecked request to the X server.
    ///
    /// Unchecked requests expect a reply that is to be retrieved by [Connection::wait_for_reply_unchecked].
    /// Unchecked means that the error is not checked when the reply is fetched. Instead, the error will
    /// be sent to the event loop
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
    ///     let cookie = conn.send_request_unchecked(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_PROTOCOLS",
    ///     });
    ///     let wm_protocols_atom: Option<x::Atom> = conn
    ///             .wait_for_reply_unchecked(cookie)?
    ///             .map(|rep| rep.atom());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn send_request_unchecked<R>(&self, req: &R) -> R::CookieUnchecked
    where
        R: RequestWithReply,
    {
        unsafe { R::CookieUnchecked::from_sequence(req.raw_request(self, false)) }
    }

    /// Check a checked request for errors.
    ///
    /// The cookie supplied to this function must have resulted
    /// from a call to [Connection::send_request_checked].  This function will block
    /// until one of two conditions happens.  If an error is received, it will be
    /// returned.  If a reply to a subsequent request has already arrived, no error
    /// can arrive for this request, so this function will return `Ok(())`.
    ///
    /// Note that this function will perform a sync if needed to ensure that the
    /// sequence number will advance beyond that provided in cookie; this is a
    /// convenience to avoid races in determining whether the sync is needed.
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
    /// #   let window: x::Window = conn.generate_id();
    ///     conn.check_request(conn.send_request_checked(&x::MapWindow { window }))?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn check_request(&self, cookie: VoidCookieChecked) -> ProtocolResult<()> {
        let cookie = xcb_void_cookie_t {
            seq: cookie.sequence() as u32,
        };
        let error = unsafe { xcb_request_check(self.c, cookie) };
        if error.is_null() {
            Ok(())
        } else {
            unsafe {
                let res = error::resolve_error(error, &self.ext_data);
                Err(res)
            }
        }
    }

    /// Send the request to the server and check it.
    ///
    /// This is a sugar for `conn.check_request(conn.send_request_checked(req))`
    ///
    /// This method is useful as well in place of code sending a void request
    /// and flushing the connection right after. Checking the request effectively
    /// flushes the connection, but in addition reports possible protocol errors
    /// at the calling site instead of reporting them through the event loop.
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
    /// #   let window: x::Window = conn.generate_id();
    ///     conn.send_and_check_request(&x::MapWindow { window })?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn send_and_check_request<R>(&self, req: &R) -> ProtocolResult<()>
    where
        R: RequestWithoutReply,
    {
        self.check_request(self.send_request_checked(req))
    }

    /// Gets the reply of a previous request, or an error if one occurred.
    ///
    /// This is blocking; it does not return until the reply has been received. For the non-blocking
    /// version, see [`poll_for_reply`].
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
    ///     let cookie = conn.send_request(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_PROTOCOLS",
    ///     });
    ///     let wm_protocols_atom: x::Atom = conn
    ///             .wait_for_reply(cookie)?
    ///             .atom();
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// [`poll_for_reply`]: Self::poll_for_reply
    pub fn wait_for_reply<C>(&self, cookie: C) -> Result<C::Reply>
    where
        C: CookieWithReplyChecked,
    {
        unsafe {
            let mut error: *mut xcb_generic_error_t = ptr::null_mut();
            let reply = xcb_wait_for_reply64(self.c, cookie.sequence(), &mut error as *mut _);
            self.handle_reply_checked::<C>(reply, error)
        }
    }

    /// Get the reply of a previous unchecked request.
    ///
    /// If an error occurred, `None` is returned and the error will be delivered to the event loop.
    ///
    /// This is blocking; it does not return until the reply has been received. For the non-blocking
    /// version, see [`poll_for_reply_unchecked`].
    ///
    /// # Example
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
    ///     let cookie = conn.send_request_unchecked(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_PROTOCOLS",
    ///     });
    ///     let wm_protocols_atom: Option<x::Atom> = conn
    ///             .wait_for_reply_unchecked(cookie)?  // connection error may happen
    ///             .map(|rep| rep.atom());
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// [`poll_for_reply_unchecked`]: Self::poll_for_reply_unchecked
    pub fn wait_for_reply_unchecked<C>(&self, cookie: C) -> ConnResult<Option<C::Reply>>
    where
        C: CookieWithReplyUnchecked,
    {
        unsafe {
            let reply = xcb_wait_for_reply64(self.c, cookie.sequence(), ptr::null_mut());
            self.handle_reply_unchecked::<C>(reply)
        }
    }

    /// Gets the reply of a previous request if it has been received, or an error if one occurred.
    ///
    /// This is non-blocking; if no reply has been received yet, it returns [`None`]. For the
    /// blocking version, see [`wait_for_reply`].
    ///
    /// # Examples
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #     let (conn, screen_num) = xcb::Connection::connect(None)?;
    /// let (wm_protocols_cookie, wm_name_cookie) = (
    ///     conn.send_request(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_PROTOCOLS",
    ///     }),
    ///     conn.send_request(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_NAME",
    ///     }),
    /// );
    /// let (wm_protocols_atom, wm_name_atom) = {
    ///     let (
    ///         mut wm_protocols_atom,
    ///         mut wm_name_atom,
    ///     ) = (None, None);
    ///
    ///     loop {
    ///         // If `wm_protocols_atom` is yet to be received, poll for it.
    ///         if wm_protocols_atom.is_none() {
    ///             wm_protocols_atom = conn
    ///                 .poll_for_reply(&wm_protocols_cookie)
    ///                 .transpose()?
    ///                 .map(|reply| reply.atom());
    ///         }
    ///         // If `wm_name_atom` is yet to be received, poll for it.
    ///         if wm_name_atom.is_none() {
    ///             wm_name_atom = conn
    ///                 .poll_for_reply(&wm_name_cookie)
    ///                 .transpose()?
    ///                 .map(|reply| reply.atom());
    ///         }
    ///
    ///         // If both `wm_protocols_atom` and `wm_name_atom` have been
    ///         // received, break from the loop.
    ///         if let (
    ///             Some(wm_protocols_atom),
    ///             Some(wm_name_atom),
    ///         ) = (wm_protocols_atom, wm_name_atom) {
    ///             break (wm_protocols_atom, wm_name_atom);
    ///         }
    ///     }
    /// };
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`wait_for_reply`]: Self::wait_for_reply
    pub fn poll_for_reply<C>(&self, cookie: &C) -> Option<Result<C::Reply>>
    where
        C: CookieWithReplyChecked,
    {
        unsafe {
            let mut error: *mut xcb_generic_error_t = ptr::null_mut();
            let mut reply: *mut c_void = ptr::null_mut();

            let received = xcb_poll_for_reply64(
                self.c,
                cookie.sequence(),
                &mut reply as *mut _,
                &mut error as *mut _,
            );

            match received {
                0 => None,
                1 => Some(self.handle_reply_checked::<C>(reply, error)),
                _ => panic!("unexpected return value from xcb_poll_for_reply64"),
            }
        }
    }

    /// Gets the reply of a previous unchecked request if it has been received.
    ///
    /// If an error occurred, [`None`] is returned and the error is delivered to the event loop.
    ///
    /// This is non-blocking; if no reply has been received yet, it returns
    /// <code>[Some]\([None])</code>. For the blocking version, see [`wait_for_reply_unchecked`].
    ///
    /// # Examples
    /// ```no_run
    /// # use xcb::x;
    /// # fn main() -> xcb::Result<()> {
    /// #     let (conn, screen_num) = xcb::Connection::connect(None)?;
    /// let (wm_protocols_cookie, wm_name_cookie) = (
    ///     conn.send_request_unchecked(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_PROTOCOLS",
    ///     }),
    ///     conn.send_request_unchecked(&x::InternAtom {
    ///         only_if_exists: true,
    ///         name: b"WM_NAME",
    ///     }),
    /// );
    /// let (wm_protocols_atom, wm_name_atom) = {
    ///     let (
    ///         mut wm_protocols_atom,
    ///         mut wm_name_atom,
    ///     ) = (Some(None), Some(None));
    ///
    ///     loop {
    ///         // If `wm_protocols_atom` is yet to be received, poll for it.
    ///         if let Some(None) = wm_protocols_atom {
    ///             wm_protocols_atom = conn
    ///                 // connection error may happen
    ///                 .poll_for_reply_unchecked(&wm_protocols_cookie)
    ///                 .transpose()?
    ///                 .map(|result| result.map(|reply| reply.atom()));
    ///         }
    ///         // If `wm_name_atom` is yet to be received, poll for it.
    ///         if let Some(None) = wm_name_atom {
    ///             wm_name_atom = conn
    ///                 // connection error may happen
    ///                 .poll_for_reply_unchecked(&wm_name_cookie)
    ///                 .transpose()?
    ///                 .map(|result| result.map(|reply| reply.atom()));
    ///         }
    ///
    ///         match (wm_protocols_atom, wm_name_atom) {
    ///             // If either `wm_protocols_atom` or `wm_name_atom` hasn't
    ///             // been received, continue the loop.
    ///             (Some(None), _) | (_, Some(None)) => continue,
    ///
    ///             // Otherwise, if both have been received, break from the
    ///             // loop.
    ///             (
    ///                 wm_protocols_atom,
    ///                 wm_name_atom,
    ///             ) => break (
    ///                 wm_protocols_atom.flatten(),
    ///                 wm_name_atom.flatten(),
    ///             ),
    ///         }
    ///     }
    /// };
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`wait_for_reply_unchecked`]: Self::wait_for_reply_unchecked
    pub fn poll_for_reply_unchecked<C>(&self, cookie: &C) -> Option<ConnResult<Option<C::Reply>>>
    where
        C: CookieWithReplyUnchecked,
    {
        unsafe {
            let mut reply: *mut c_void = ptr::null_mut();

            let received = xcb_poll_for_reply64(
                self.c,
                cookie.sequence(),
                &mut reply as *mut _,
                ptr::null_mut(),
            );

            match received {
                0 => None,
                1 => Some(self.handle_reply_unchecked::<C>(reply)),
                _ => panic!("unexpected return value from xcb_poll_for_reply64"),
            }
        }
    }

    /// Obtain number of bytes read from the connection.
    ///
    /// Returns cumulative number of bytes received from the connection.
    ///
    /// This retrieves the total number of bytes read from this connection,
    /// to be used for diagnostic/monitoring/informative purposes.
    ///
    /// Since: libxcb 1.14
    #[cfg(feature = "libxcb_v1_14")]
    pub fn total_read(&self) -> usize {
        unsafe { xcb_total_read(self.c) as usize }
    }

    /// Obtain number of bytes written to the connection.
    ///
    /// Returns cumulative number of bytes sent to the connection.
    ///
    /// This retrieves the total number of bytes written to this connection,
    /// to be used for diagnostic/monitoring/informative purposes.
    ///
    /// Since: libxcb 1.14
    #[cfg(feature = "libxcb_v1_14")]
    pub fn total_written(&self) -> usize {
        unsafe { xcb_total_written(self.c) as usize }
    }
}

impl Connection {
    unsafe fn handle_wait_for_event(&self, ev: *mut xcb_generic_event_t) -> Result<Event> {
        if ev.is_null() {
            self.has_error()?;
            panic!("xcb_wait_for_event returned null with I/O error");
        } else if is_error(ev) {
            Err(error::resolve_error(ev as *mut _, &self.ext_data).into())
        } else {
            Ok(event::resolve_event(ev, &self.ext_data))
        }
    }

    unsafe fn handle_poll_for_event(&self, ev: *mut xcb_generic_event_t) -> Result<Option<Event>> {
        if ev.is_null() {
            self.has_error()?;
            Ok(None)
        } else if is_error(ev) {
            Err(error::resolve_error(ev as *mut _, &self.ext_data).into())
        } else {
            Ok(Some(event::resolve_event(ev, &self.ext_data)))
        }
    }

    unsafe fn handle_reply_checked<C>(
        &self,
        reply: *mut c_void,
        error: *mut xcb_generic_error_t,
    ) -> Result<C::Reply>
    where
        C: CookieWithReplyChecked,
    {
        match (reply.is_null(), error.is_null()) {
            (true, true) => {
                self.has_error()?;
                unreachable!("xcb_wait_for_reply64 returned null without I/O error");
            }
            (true, false) => {
                let error = error::resolve_error(error, &self.ext_data);
                Err(error.into())
            }
            (false, true) => Ok(C::Reply::from_raw(reply as *const u8)),
            (false, false) => unreachable!("xcb_wait_for_reply64 returned two pointers"),
        }
    }

    unsafe fn handle_reply_unchecked<C>(&self, reply: *mut c_void) -> ConnResult<Option<C::Reply>>
    where
        C: CookieWithReplyUnchecked,
    {
        if reply.is_null() {
            self.has_error()?;
            Ok(None)
        } else {
            Ok(Some(C::Reply::from_raw(reply as *const u8)))
        }
    }
}

impl AsRef<Connection> for Connection {
    fn as_ref(&self) -> &Connection {
        self
    }
}

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { xcb_get_file_descriptor(self.c) }
    }
}

// SAFETY: We provide a valid xcb_connection_t that is valid for as long as required by the trait.
#[cfg(feature = "as-raw-xcb-connection")]
unsafe impl as_raw_xcb_connection::AsRawXcbConnection for Connection {
    fn as_raw_xcb_connection(&self) -> *mut as_raw_xcb_connection::xcb_connection_t {
        self.get_raw_conn().cast()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        #[cfg(feature = "debug_atom_names")]
        if self.dbg_atom_names {
            unsafe {
                dan::DAN_CONN = ptr::null_mut();
            }
        }

        #[cfg(not(feature = "xlib_xcb"))]
        unsafe {
            xcb_disconnect(self.c);
        }

        #[cfg(feature = "xlib_xcb")]
        unsafe {
            if self.dpy.is_null() {
                xcb_disconnect(self.c);
            } else {
                xlib::XCloseDisplay(self.dpy);
            }
        }
    }
}

#[cfg(feature = "debug_atom_names")]
mod dan {
    use super::{Connection, Xid};
    use crate::ffi::base::xcb_connection_t;
    use crate::x;

    use std::fmt;
    use std::mem;
    use std::ptr;
    use std::str;

    pub(crate) static mut DAN_CONN: *mut xcb_connection_t = ptr::null_mut();

    impl fmt::Debug for x::Atom {
        #[allow(clippy::print_in_format_impl)]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.resource_id() == 0 {
                return f.write_str("ATOM_NONE");
            }

            let conn = unsafe { Connection::from_raw_conn(DAN_CONN) };

            let cookie = conn.send_request(&x::GetAtomName { atom: *self });
            let reply = conn.wait_for_reply(cookie).map_err(|err| {
                eprintln!(
                    "Error during fmt::Debug of x::Atom (fetching atom name): {:#?}",
                    err
                );
                fmt::Error
            })?;

            let name = reply.name().to_utf8();
            f.write_fmt(format_args!("Atom(\"{}\" ; {})", name, self.resource_id()))?;

            mem::forget(conn);
            Ok(())
        }
    }
}

unsafe fn check_connection_error(conn: *mut xcb_connection_t) -> ConnResult<()> {
    match xcb_connection_has_error(conn) {
        0 => Ok(()),
        XCB_CONN_ERROR => Err(ConnError::Connection),
        XCB_CONN_CLOSED_EXT_NOTSUPPORTED => Err(ConnError::ClosedExtNotSupported),
        XCB_CONN_CLOSED_MEM_INSUFFICIENT => Err(ConnError::ClosedMemInsufficient),
        XCB_CONN_CLOSED_REQ_LEN_EXCEED => Err(ConnError::ClosedReqLenExceed),
        XCB_CONN_CLOSED_PARSE_ERR => Err(ConnError::ClosedParseErr),
        XCB_CONN_CLOSED_INVALID_SCREEN => Err(ConnError::ClosedInvalidScreen),
        XCB_CONN_CLOSED_FDPASSING_FAILED => Err(ConnError::ClosedFdPassingFailed),
        code => unreachable!("unexpected error code from XCB: {}", code),
    }
}

unsafe fn is_error(ev: *mut xcb_generic_event_t) -> bool {
    debug_assert!(!ev.is_null());
    (*ev).response_type == 0
}

bitflags! {
    pub(crate) struct RequestFlags: u32 {
        const NONE = 0;
        const CHECKED = 1;
        const RAW = 2;
        const DISCARD_REPLY = 4;
        const REPLY_FDS = 8;
    }
}

/// Compute the necessary padding after `base` to have `align` alignment
pub(crate) fn align_pad(base: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two(), "`align` must be a power of two");

    let base = base as isize;
    let align = align as isize;
    (-base & (align - 1)) as usize
}

#[test]
fn test_align_pad() {
    // align 1
    assert_eq!(align_pad(0, 1), 0);
    assert_eq!(align_pad(1234, 1), 0);
    assert_eq!(align_pad(1235, 1), 0);
    // align 2
    assert_eq!(align_pad(0, 2), 0);
    assert_eq!(align_pad(1233, 2), 1);
    assert_eq!(align_pad(1234, 2), 0);
    // align 4
    assert_eq!(align_pad(0, 4), 0);
    assert_eq!(align_pad(12, 4), 0);
    assert_eq!(align_pad(13, 4), 3);
    assert_eq!(align_pad(14, 4), 2);
    assert_eq!(align_pad(15, 4), 1);
    assert_eq!(align_pad(16, 4), 0);
}
