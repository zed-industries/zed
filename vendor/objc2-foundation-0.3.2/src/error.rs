use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

use crate::{util, NSError};

impl UnwindSafe for NSError {}
impl RefUnwindSafe for NSError {}

/// Creation methods.
impl NSError {
    /// Construct a new [`NSError`] with the given code in the given domain.
    #[cfg(feature = "NSDictionary")]
    #[cfg(feature = "NSString")]
    pub fn new(
        code: objc2::ffi::NSInteger,
        domain: &crate::NSErrorDomain,
    ) -> objc2::rc::Retained<Self> {
        use objc2::AnyThread;
        // SAFETY: `domain` and `user_info` are copied to the error object, so
        // even if the `&NSString` came from a `&mut NSMutableString`, we're
        // still good!
        unsafe { Self::initWithDomain_code_userInfo(Self::alloc(), domain, code, None) }
    }
}

/// Accessor methods.
impl NSError {
    #[cfg(feature = "NSString")]
    pub fn NSLocalizedDescriptionKey() -> &'static crate::NSErrorUserInfoKey {
        unsafe { crate::NSLocalizedDescriptionKey }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NSError {}

impl fmt::Debug for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("NSError");
        debug.field("code", &self.code());

        #[cfg(feature = "NSString")]
        debug.field("localizedDescription", &self.localizedDescription());

        #[cfg(feature = "NSString")]
        debug.field("domain", &self.domain());

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        debug.field("userInfo", &self.userInfo());

        debug.finish_non_exhaustive()
    }
}

impl fmt::Display for NSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc: Retained<NSObject> = if cfg!(feature = "gnustep-1-7") {
            // Can return NULL:
            // https://github.com/gnustep/libs-base/issues/486
            let desc: Option<Retained<NSObject>> = unsafe { msg_send![self, localizedDescription] };
            if let Some(desc) = desc {
                desc
            } else {
                let domain: Retained<NSObject> = unsafe { msg_send![self, domain] };
                // SAFETY: `domain` returns `NSErrorDomain`, which is `NSString`.
                unsafe { util::display_string(&domain, f)? };
                write!(f, " {}", self.code())?;

                return Ok(());
            }
        } else {
            unsafe { msg_send![self, localizedDescription] }
        };

        // SAFETY: `localizedDescription` returns `NSString`.
        unsafe { util::display_string(&desc, f) }
    }
}
