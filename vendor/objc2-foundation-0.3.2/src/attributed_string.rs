use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::Retained;
use objc2::AnyThread;

use crate::*;

// Same reasoning as `NSString`.
impl UnwindSafe for NSAttributedString {}
impl RefUnwindSafe for NSAttributedString {}

impl NSAttributedString {
    /// Creates a new attributed string from the given string and attributes.
    ///
    /// The attributes are associated with every UTF-16 code unit in the
    /// string.
    ///
    /// # Safety
    ///
    /// The attributes must be valid.
    #[doc(alias = "initWithString:")]
    #[cfg(feature = "NSDictionary")]
    #[cfg(feature = "NSString")]
    pub unsafe fn new_with_attributes(
        string: &NSString,
        attributes: &NSDictionary<NSAttributedStringKey, objc2::runtime::AnyObject>,
    ) -> Retained<Self> {
        unsafe { Self::initWithString_attributes(Self::alloc(), string, Some(attributes)) }
    }

    /// Creates a new attributed string without any attributes.
    #[doc(alias = "initWithString:")]
    #[cfg(feature = "NSString")]
    pub fn from_nsstring(string: &NSString) -> Retained<Self> {
        Self::initWithString(Self::alloc(), string)
    }
}

impl NSMutableAttributedString {
    // TODO: new_with_attributes

    #[doc(alias = "initWithString:")]
    #[cfg(feature = "NSString")]
    pub fn from_nsstring(string: &NSString) -> Retained<Self> {
        Self::initWithString(Self::alloc(), string)
    }

    #[doc(alias = "initWithAttributedString:")]
    pub fn from_attributed_nsstring(attributed_string: &NSAttributedString) -> Retained<Self> {
        Self::initWithAttributedString(Self::alloc(), attributed_string)
    }
}
