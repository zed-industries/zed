#[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
use core::fmt;
use core::hint::unreachable_unchecked;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::exception::Exception;
use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol};
use objc2::{extern_methods, sel};

use crate::Foundation::NSException;

// SAFETY: Exception objects are immutable data containers, and documented as
// thread safe.
unsafe impl Sync for NSException {}
unsafe impl Send for NSException {}

impl UnwindSafe for NSException {}
impl RefUnwindSafe for NSException {}

extern_methods!(
    unsafe impl NSException {
        #[method(raise)]
        unsafe fn raise_raw(&self);
    }
);

impl NSException {
    /// Create a new [`NSException`] object.
    ///
    /// Returns `None` if the exception couldn't be created (example: If the
    /// process is out of memory).
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    #[cfg(feature = "NSDictionary")]
    pub fn new(
        name: &crate::Foundation::NSExceptionName,
        reason: Option<&crate::Foundation::NSString>,
        user_info: Option<&crate::Foundation::NSDictionary>,
    ) -> Option<Retained<Self>> {
        use objc2::ClassType;

        unsafe {
            objc2::msg_send_id![
                Self::alloc(),
                initWithName: name,
                reason: reason,
                userInfo: user_info,
            ]
        }
    }

    /// Raises the exception, causing program flow to jump to the local
    /// exception handler.
    ///
    /// This is equivalent to using `objc2::exception::throw`.
    ///
    ///
    /// # Safety
    ///
    /// Same as `objc2::exception::throw`.
    pub unsafe fn raise(&self) -> ! {
        // SAFETY: `NSException` is immutable, so it is safe to give to
        // the place where `@catch` receives it.
        unsafe { self.raise_raw() };
        // SAFETY: `raise` will throw an exception, or abort if something
        // unexpected happened.
        unsafe { unreachable_unchecked() }
    }

    /// Convert this into an [`Exception`] object.
    pub fn into_exception(this: Retained<Self>) -> Retained<Exception> {
        // SAFETY: Downcasting to "subclass"
        unsafe { Retained::cast(this) }
    }

    fn is_nsexception(obj: &Exception) -> bool {
        if obj.class().responds_to(sel!(isKindOfClass:)) {
            // SAFETY: We only use `isKindOfClass:` on NSObject
            let obj: *const Exception = obj;
            let obj = unsafe { obj.cast::<NSObject>().as_ref().unwrap() };
            obj.is_kind_of::<Self>()
        } else {
            false
        }
    }

    /// Create this from an [`Exception`] object.
    ///
    /// This should be considered a hint; it may return `Err` in very, very
    /// few cases where the object is actually an instance of `NSException`.
    pub fn from_exception(obj: Retained<Exception>) -> Result<Retained<Self>, Retained<Exception>> {
        if Self::is_nsexception(&obj) {
            // SAFETY: Just checked the object is an NSException
            Ok(unsafe { Retained::cast::<Self>(obj) })
        } else {
            Err(obj)
        }
    }
}

#[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
impl fmt::Debug for NSException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &objc2::runtime::AnyObject = self.as_ref();
        write!(f, "{obj:?} '{}'", self.name())?;
        if let Some(reason) = self.reason() {
            write!(f, " reason:{reason}")?;
        } else {
            write!(f, " reason:(NULL)")?;
        }
        Ok(())
    }
}
