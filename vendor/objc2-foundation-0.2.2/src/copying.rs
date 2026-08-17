use objc2::mutability::CounterpartOrSelf;
use objc2::rc::Retained;
#[cfg(feature = "NSZone")]
use objc2::runtime::NSZone;
use objc2::{extern_protocol, ProtocolType};

extern_protocol!(
    /// A protocol to provide functional copies of objects.
    ///
    /// This is similar to Rust's [`Clone`] trait, along with sharing a few
    /// similarities to the [`std::borrow::ToOwned`] trait with regards to the
    /// output type.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/foundation/nscopying
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait NSCopying {
        /// Returns a new instance that's a copy of the receiver.
        ///
        /// The output type is the immutable counterpart of the object, which is
        /// usually `Self`, but e.g. `NSMutableString` returns `NSString`.
        #[method_id(copy)]
        #[optional]
        fn copy(&self) -> Retained<Self::Immutable>
        where
            Self: CounterpartOrSelf;

        /// Returns a new instance that's a copy of the receiver.
        ///
        /// This is only used when implementing `NSCopying`, use
        /// [`copy`][NSCopying::copy] instead.
        ///
        ///
        /// # Safety
        ///
        /// The zone pointer must be valid or NULL.
        #[method_id(copyWithZone:)]
        #[cfg(feature = "NSZone")]
        unsafe fn copyWithZone(&self, zone: *mut NSZone) -> Retained<Self::Immutable>
        where
            Self: CounterpartOrSelf;
    }

    unsafe impl ProtocolType for dyn NSCopying {}
);

extern_protocol!(
    /// A protocol to provide mutable copies of objects.
    ///
    /// Only classes that have an “immutable vs. mutable” distinction should adopt
    /// this protocol.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/foundation/nsmutablecopying
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait NSMutableCopying {
        /// Returns a new instance that's a mutable copy of the receiver.
        ///
        /// The output type is the mutable counterpart of the object. E.g. both
        /// `NSString` and `NSMutableString` return `NSMutableString`.
        #[method_id(mutableCopy)]
        #[optional]
        fn mutableCopy(&self) -> Retained<Self::Mutable>
        where
            Self: CounterpartOrSelf;

        /// Returns a new instance that's a mutable copy of the receiver.
        ///
        /// This is only used when implementing `NSMutableCopying`, use
        /// [`mutableCopy`][NSMutableCopying::mutableCopy] instead.
        ///
        ///
        /// # Safety
        ///
        /// The zone pointer must be valid or NULL.
        #[method_id(mutableCopyWithZone:)]
        #[cfg(feature = "NSZone")]
        unsafe fn mutableCopyWithZone(&self, zone: *mut NSZone) -> Retained<Self::Mutable>
        where
            Self: CounterpartOrSelf;
    }

    unsafe impl ProtocolType for dyn NSMutableCopying {}
);
