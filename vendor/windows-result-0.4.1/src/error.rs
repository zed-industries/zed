use super::*;
use core::num::NonZeroI32;

#[expect(unused_imports)]
use core::mem::size_of;

/// An error object consists of both an error code and optional detailed error information for debugging.
///
/// # Extended error info and the `windows_slim_errors` configuration option
///
/// `Error` contains an [`HRESULT`] value that describes the error, as well as an optional
/// `IErrorInfo` COM object. The `IErrorInfo` object is a COM object that can provide detailed information
/// about an error, such as a text string, a `ProgID` of the originator, etc. If the error object
/// was originated in an WinRT component, then additional information such as a stack track may be
/// captured.
///
/// However, many systems based on COM do not use `IErrorInfo`. For these systems, the optional error
/// info within `Error` has no benefits, but has substantial costs because it increases the size of
/// the `Error` object, which also increases the size of `Result<T>`.
///
/// This error information can be disabled at compile time by setting `RUSTFLAGS=--cfg=windows_slim_errors`.
/// This removes the `IErrorInfo` support within the [`Error`] type, which has these benefits:
///
/// * It reduces the size of [`Error`] to 4 bytes (the size of [`HRESULT`]).
///
/// * It reduces the size of `Result<(), Error>` to 4 bytes, allowing it to be returned in a single
///   machine register.
///
/// * The `Error` (and `Result<T, Error>`) types no longer have a [`Drop`] impl. This removes the need
///   for lifetime checking and running drop code when [`Error`] and [`Result`] go out of scope. This
///   significantly reduces code size for codebase that make extensive use of [`Error`].
///
/// Of course, these benefits come with a cost; you lose extended error information for those
/// COM objects that support it.
///
/// This is controlled by a `--cfg` option rather than a Cargo feature because this compilation
/// option sets a policy that applies to an entire graph of crates. Individual crates that take a
/// dependency on the `windows-result` crate are not in a good position to decide whether they want
/// slim errors or full errors.  Cargo features are meant to be additive, but specifying the size
/// and contents of `Error` is not a feature so much as a whole-program policy decision.
///
/// # References
///
/// * [`IErrorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-ierrorinfo)
#[derive(Clone)]
pub struct Error {
    /// The `HRESULT` error code, but represented using [`NonZeroI32`]. [`NonZeroI32`] provides
    /// a "niche" to the Rust compiler, which is a space-saving optimization. This allows the
    /// compiler to use more compact representation for enum variants (such as [`Result`]) that
    /// contain instances of [`Error`].
    code: NonZeroI32,

    /// Contains details about the error, such as error text.
    info: ErrorInfo,
}

/// We remap S_OK to this error because the S_OK representation (zero) is reserved for niche
/// optimizations.
const S_EMPTY_ERROR: NonZeroI32 = const_nonzero_i32(u32::from_be_bytes(*b"S_OK") as i32);

/// Converts an HRESULT into a NonZeroI32. If the input is S_OK (zero), then this is converted to
/// S_EMPTY_ERROR. This is necessary because NonZeroI32, as the name implies, cannot represent the
/// value zero. So we remap it to a value no one should be using, during storage.
const fn const_nonzero_i32(i: i32) -> NonZeroI32 {
    if let Some(nz) = NonZeroI32::new(i) {
        nz
    } else {
        panic!();
    }
}

fn nonzero_hresult(hr: HRESULT) -> NonZeroI32 {
    if let Some(nz) = NonZeroI32::new(hr.0) {
        nz
    } else {
        S_EMPTY_ERROR
    }
}

impl Error {
    /// Creates an error object without any failure information.
    pub const fn empty() -> Self {
        Self {
            code: S_EMPTY_ERROR,
            info: ErrorInfo::empty(),
        }
    }

    /// Creates a new error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new<T: AsRef<str>>(code: HRESULT, message: T) -> Self {
        #[cfg(windows)]
        {
            let message: &str = message.as_ref();
            if message.is_empty() {
                Self::from_hresult(code)
            } else {
                ErrorInfo::originate_error(code, message);
                code.into()
            }
        }
        #[cfg(not(windows))]
        {
            let _ = message;
            Self::from_hresult(code)
        }
    }

    /// Creates a new error object with an error code, but without additional error information.
    pub fn from_hresult(code: HRESULT) -> Self {
        Self {
            code: nonzero_hresult(code),
            info: ErrorInfo::empty(),
        }
    }

    /// Creates a new `Error` from the Win32 error code returned by `GetLastError()`.
    pub fn from_thread() -> Self {
        Self::from_hresult(HRESULT::from_thread())
    }

    /// The error code describing the error.
    pub const fn code(&self) -> HRESULT {
        if self.code.get() == S_EMPTY_ERROR.get() {
            HRESULT(0)
        } else {
            HRESULT(self.code.get())
        }
    }

    /// The error message describing the error.
    pub fn message(&self) -> String {
        if let Some(message) = self.info.message() {
            return message;
        }

        // Otherwise fallback to a generic error code description.
        self.code().message()
    }

    /// The error object describing the error.
    #[cfg(windows)]
    pub fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.info.as_ptr()
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<Error> for HRESULT {
    fn from(error: Error) -> Self {
        let code = error.code();
        error.info.into_thread();
        code
    }
}

impl From<HRESULT> for Error {
    fn from(code: HRESULT) -> Self {
        Self {
            code: nonzero_hresult(code),
            info: ErrorInfo::from_thread(),
        }
    }
}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(from: Error) -> Self {
        Self::from_raw_os_error(from.code().0)
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(from: std::io::Error) -> Self {
        match from.raw_os_error() {
            Some(status) => HRESULT::from_win32(status as u32).into(),
            None => HRESULT(E_UNEXPECTED).into(),
        }
    }
}

impl From<alloc::string::FromUtf16Error> for Error {
    fn from(_: alloc::string::FromUtf16Error) -> Self {
        Self::from_hresult(HRESULT::from_win32(ERROR_NO_UNICODE_TRANSLATION))
    }
}

impl From<alloc::string::FromUtf8Error> for Error {
    fn from(_: alloc::string::FromUtf8Error) -> Self {
        Self::from_hresult(HRESULT::from_win32(ERROR_NO_UNICODE_TRANSLATION))
    }
}

impl From<core::num::TryFromIntError> for Error {
    fn from(_: core::num::TryFromIntError) -> Self {
        Self::from_hresult(HRESULT::from_win32(ERROR_INVALID_DATA))
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("Error");
        debug
            .field("code", &self.code())
            .field("message", &self.message())
            .finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let message = self.message();
        if message.is_empty() {
            core::write!(fmt, "{}", self.code())
        } else {
            core::write!(fmt, "{} ({})", message, self.code())
        }
    }
}

impl core::hash::Hash for Error {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
        // We do not hash the error info.
    }
}

// Equality tests only the HRESULT, not the error info (if any).
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Eq for Error {}

impl PartialOrd for Error {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Error {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.code.cmp(&other.code)
    }
}

use error_info::*;

#[cfg(all(windows, not(windows_slim_errors)))]
mod error_info {
    use super::*;
    use crate::com::ComPtr;

    /// This type stores error detail, represented by a COM `IErrorInfo` object.
    ///
    /// # References
    ///
    /// * [`IErrorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-ierrorinfo)
    #[derive(Clone, Default)]
    pub(crate) struct ErrorInfo {
        pub(super) ptr: Option<ComPtr>,
    }

    impl ErrorInfo {
        pub(crate) const fn empty() -> Self {
            Self { ptr: None }
        }

        pub(crate) fn from_thread() -> Self {
            unsafe {
                let mut ptr = core::mem::MaybeUninit::zeroed();
                crate::bindings::GetErrorInfo(0, ptr.as_mut_ptr() as *mut _);
                Self {
                    ptr: ptr.assume_init(),
                }
            }
        }

        pub(crate) fn into_thread(self) {
            if let Some(ptr) = self.ptr {
                unsafe {
                    crate::bindings::SetErrorInfo(0, ptr.as_raw());
                }
            }
        }

        pub(crate) fn originate_error(code: HRESULT, message: &str) {
            let message: Vec<_> = message.encode_utf16().collect();
            unsafe {
                RoOriginateErrorW(code.0, message.len() as u32, message.as_ptr());
            }
        }

        pub(crate) fn message(&self) -> Option<String> {
            use crate::bstr::BasicString;

            let ptr = self.ptr.as_ref()?;

            let mut message = BasicString::default();

            // First attempt to retrieve the restricted error information.
            if let Some(info) = ptr.cast(&IID_IRestrictedErrorInfo) {
                let mut fallback = BasicString::default();
                let mut code = 0;

                unsafe {
                    com_call!(
                        IRestrictedErrorInfo_Vtbl,
                        info.GetErrorDetails(
                            &mut fallback as *mut _ as _,
                            &mut code,
                            &mut message as *mut _ as _,
                            &mut BasicString::default() as *mut _ as _
                        )
                    );
                }

                if message.is_empty() {
                    message = fallback
                };
            }

            // Next attempt to retrieve the regular error information.
            if message.is_empty() {
                unsafe {
                    com_call!(
                        IErrorInfo_Vtbl,
                        ptr.GetDescription(&mut message as *mut _ as _)
                    );
                }
            }

            Some(String::from_utf16_lossy(wide_trim_end(&message)))
        }

        pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
            if let Some(info) = self.ptr.as_ref() {
                info.as_raw()
            } else {
                core::ptr::null_mut()
            }
        }
    }

    unsafe impl Send for ErrorInfo {}
    unsafe impl Sync for ErrorInfo {}
}

#[cfg(not(all(windows, not(windows_slim_errors))))]
mod error_info {
    use super::*;

    // We use this name so that the NatVis <Type> element for ErrorInfo does *not* match this type.
    // This prevents the NatVis description from failing to load.
    #[derive(Clone, Default)]
    pub(crate) struct EmptyErrorInfo;

    pub(crate) use EmptyErrorInfo as ErrorInfo;

    impl EmptyErrorInfo {
        pub(crate) const fn empty() -> Self {
            Self
        }

        pub(crate) fn from_thread() -> Self {
            Self
        }

        pub(crate) fn into_thread(self) {}

        #[cfg(windows)]
        pub(crate) fn originate_error(_code: HRESULT, _message: &str) {}

        pub(crate) fn message(&self) -> Option<String> {
            None
        }

        #[cfg(windows)]
        pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
            core::ptr::null_mut()
        }
    }
}
