use alloc::string::ToString;
use core::fmt;
use core::hash;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::str;
use std::ffi::{CStr, CString};

use objc2::encode::Encode;
use objc2::rc::Retained;
use objc2::ClassType;

use crate::Foundation::NSValue;

// We can't implement any auto traits for NSValue, since it can contain an
// arbitary object!

/// Creation methods.
impl NSValue {
    /// Create a new `NSValue` containing the given type.
    ///
    /// Be careful when using this since you may accidentally pass a reference
    /// when you wanted to pass a concrete type instead.
    ///
    ///
    /// # Examples
    ///
    /// Create an `NSValue` containing an `i32`.
    ///
    /// ```
    /// use objc2_foundation::NSValue;
    ///
    /// let val = NSValue::new(42i32);
    /// ```
    ///
    /// [`NSPoint`]: crate::Foundation::NSPoint
    pub fn new<T: 'static + Copy + Encode>(value: T) -> Retained<Self> {
        let bytes: NonNull<T> = NonNull::from(&value);
        let encoding = CString::new(T::ENCODING.to_string()).unwrap();
        unsafe {
            Self::initWithBytes_objCType(
                Self::alloc(),
                bytes.cast(),
                NonNull::new(encoding.as_ptr() as *mut _).unwrap(),
            )
        }
    }
}

/// Getter methods.
impl NSValue {
    /// Retrieve the data contained in the `NSValue`.
    ///
    /// Note that this is broken on GNUStep for some types, see
    /// [gnustep/libs-base#216].
    ///
    /// [gnustep/libs-base#216]: https://github.com/gnustep/libs-base/pull/216
    ///
    ///
    /// # Safety
    ///
    /// The type of `T` must be what the NSValue actually stores, and any
    /// safety invariants that the value has must be upheld.
    ///
    /// Note that it may be enough, although is not always, to check whether
    /// [`contains_encoding`] returns `true`. For example, `NonNull<T>` have
    /// the same encoding as `*const T`, but `NonNull<T>` is clearly not
    /// safe to return from this function even if you've checked the encoding
    /// beforehand.
    ///
    /// [`contains_encoding`]: Self::contains_encoding
    ///
    ///
    /// # Examples
    ///
    /// Store a pointer in `NSValue`, and retrieve it again afterwards.
    ///
    /// ```
    /// use std::ffi::c_void;
    /// use std::ptr;
    /// use objc2_foundation::NSValue;
    ///
    /// let val = NSValue::new::<*const c_void>(ptr::null());
    /// // SAFETY: The value was just created with a pointer
    /// let res = unsafe { val.get::<*const c_void>() };
    /// assert!(res.is_null());
    /// ```
    pub unsafe fn get<T: 'static + Copy + Encode>(&self) -> T {
        debug_assert!(
        self.contains_encoding::<T>(),
        "wrong encoding. NSValue tried to return something with encoding {}, but the encoding of the given type was {}",
        self.encoding().unwrap_or("(NULL)"),
        T::ENCODING,
    );
        let mut value = MaybeUninit::<T>::uninit();
        let ptr: NonNull<T> = NonNull::new(value.as_mut_ptr()).unwrap();
        #[allow(deprecated)]
        unsafe {
            self.getValue(ptr.cast())
        };
        // SAFETY: We know that `getValue:` initialized the value, and user
        // ensures that it is safe to access.
        unsafe { value.assume_init() }
    }

    #[cfg(feature = "NSRange")]
    pub fn get_range(&self) -> Option<crate::Foundation::NSRange> {
        if self.contains_encoding::<crate::Foundation::NSRange>() {
            // SAFETY: We just checked that this contains an NSRange
            Some(unsafe { self.rangeValue() })
        } else {
            None
        }
    }

    #[cfg(feature = "NSGeometry")]
    pub fn get_point(&self) -> Option<crate::Foundation::NSPoint> {
        if self.contains_encoding::<crate::Foundation::NSPoint>() {
            // SAFETY: We just checked that this contains an NSPoint
            //
            // Note: The documentation says that `pointValue`, `sizeValue` and
            // `rectValue` is only available on macOS, but turns out that they
            // are actually available everywhere!
            Some(unsafe { self.pointValue() })
        } else {
            None
        }
    }

    #[cfg(feature = "NSGeometry")]
    pub fn get_size(&self) -> Option<crate::Foundation::NSSize> {
        if self.contains_encoding::<crate::Foundation::NSSize>() {
            // SAFETY: We just checked that this contains an NSSize
            Some(unsafe { self.sizeValue() })
        } else {
            None
        }
    }

    #[cfg(feature = "NSGeometry")]
    pub fn get_rect(&self) -> Option<crate::Foundation::NSRect> {
        if self.contains_encoding::<crate::Foundation::NSRect>() {
            // SAFETY: We just checked that this contains an NSRect
            Some(unsafe { self.rectValue() })
        } else {
            None
        }
    }

    pub fn encoding(&self) -> Option<&str> {
        let ptr = self.objCType().as_ptr();
        Some(unsafe { CStr::from_ptr(ptr) }.to_str().unwrap())
    }

    pub fn contains_encoding<T: 'static + Copy + Encode>(&self) -> bool {
        T::ENCODING.equivalent_to_str(self.encoding().unwrap())
    }
}

impl hash::Hash for NSValue {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl PartialEq for NSValue {
    #[doc(alias = "isEqualToValue:")]
    fn eq(&self, other: &Self) -> bool {
        // Use isEqualToValue: instaed of isEqual: since it is faster
        self.isEqualToValue(other)
    }
}

impl fmt::Debug for NSValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let enc = self.encoding().unwrap_or("(NULL)");
        let bytes = &**self; // Delegate to -[NSObject description]
        f.debug_struct("NSValue")
            .field("encoding", &enc)
            .field("bytes", bytes)
            .finish()
    }
}
