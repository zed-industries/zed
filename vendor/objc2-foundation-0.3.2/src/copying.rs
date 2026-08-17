use objc2::extern_protocol;
use objc2::rc::Retained;
use objc2::runtime::NSZone;
use objc2::runtime::ProtocolObject;
use objc2::Message;

/// A helper type for implementing [`NSCopying`].
///
/// `NSCopying` and `NSMutableCopying` do not in their signatures describe the
/// result type from the copying operation. This is problematic, as it means
/// that using them ends up falling back to [`AnyObject`], which makes copying
/// much less useful and ergonomic.
///
/// To properly describe this, we need an associated type which describes the
/// actual result type from a copy. The associated type can't be present
/// directly on the protocol traits themselves, however, since we want to use
/// them as e.g. `ProtocolObject<dyn NSCopying>`, so we introduce this helper
/// trait instead. See [`MutableCopyingHelper`] for the mutable variant.
///
/// We might be able to get rid of this hack once [associated type defaults]
/// are stabilized.
///
/// [`AnyObject`]: objc2::runtime::AnyObject
/// [associated type defaults]: https://github.com/rust-lang/rust/issues/29661
///
///
/// # Safety
///
/// The [`Result`] type must be correct.
///
/// [`Result`]: Self::Result
pub unsafe trait CopyingHelper: Message {
    /// The immutable counterpart of the type, or `Self` if the type has no
    /// immutable counterpart.
    ///
    /// The implementation for `NSString` has itself (`NSString`) here, while
    /// `NSMutableString` instead has `NSString`.
    type Result: Message;
}

/// A helper type for implementing [`NSMutableCopying`].
///
/// See [`CopyingHelper`] for the immutable variant, and more details in
/// general. These traits are split to allow implementing
/// `MutableCopyingHelper` only when the mutable class is available.
///
///
/// # Safety
///
/// The [`Result`] type must be correct.
///
/// [`Result`]: Self::Result
pub unsafe trait MutableCopyingHelper: Message {
    /// The mutable counterpart of the type, or `Self` if the type has no
    /// mutable counterpart.
    ///
    /// The implementation for `NSString` has `NSMutableString` here, while
    /// `NSMutableString` has itself (`NSMutableString`).
    type Result: Message;
}

// SAFETY: Superclasses are not in general required to implement the same
// traits as their subclasses, but we're not dealing with normal classes and
// arbitrary protocols, we're dealing with with immutable/mutable class
// counterparts, and the `NSCopying`/`NSMutableCopying` protocols, which
// _will_ be implemented on superclasses.
unsafe impl<P: ?Sized> CopyingHelper for ProtocolObject<P> {
    type Result = Self;
}

// SAFETY: Subclasses are required to always implement the same traits as
// their superclasses, so a mutable subclass is required to implement the same
// traits too.
unsafe impl<P: ?Sized> MutableCopyingHelper for ProtocolObject<P> {
    type Result = Self;
}

extern_protocol!(
    /// A protocol to provide functional copies of objects.
    ///
    /// This is similar to Rust's [`Clone`] trait, along with sharing a few
    /// similarities to the [`std::borrow::ToOwned`] trait with regards to the
    /// output type.
    ///
    /// To allow using this in a meaningful way in Rust, we have to "enrich"
    /// the implementation by also specifying the resulting type, see
    /// [`CopyingHelper`] for details.
    ///
    /// See also [Apple's documentation][apple-doc].
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/foundation/nscopying
    ///
    ///
    /// # Examples
    ///
    /// Implement `NSCopying` for an externally defined class.
    ///
    /// ```
    /// use objc2::extern_class;
    /// use objc2_foundation::{CopyingHelper, NSCopying, NSObject};
    ///
    /// extern_class!(
    ///     #[unsafe(super(NSObject))]
    ///     # #[name = "NSData"]
    ///     struct ExampleClass;
    /// );
    ///
    /// unsafe impl NSCopying for ExampleClass {}
    ///
    /// // Copying ExampleClass returns another ExampleClass.
    /// unsafe impl CopyingHelper for ExampleClass {
    ///     type Result = Self;
    /// }
    /// ```
    ///
    /// Implement `NSCopying` for a custom class.
    ///
    /// ```
    /// use objc2::{define_class, msg_send, AnyThread, DefinedClass};
    /// use objc2::rc::Retained;
    /// use objc2::runtime::NSZone;
    /// use objc2_foundation::{CopyingHelper, NSCopying, NSObject};
    ///
    /// define_class!(
    ///     #[unsafe(super(NSObject))]
    ///     struct CustomClass;
    ///
    ///     unsafe impl NSCopying for CustomClass {
    ///         #[unsafe(method_id(copyWithZone:))]
    ///         fn copyWithZone(&self, _zone: *const NSZone) -> Retained<Self> {
    ///             // Create new class, and transfer ivars
    ///             let new = Self::alloc().set_ivars(self.ivars().clone());
    ///             unsafe { msg_send![super(new), init] }
    ///         }
    ///     }
    /// );
    ///
    /// // Copying CustomClass returns another CustomClass.
    /// unsafe impl CopyingHelper for CustomClass {
    ///     type Result = Self;
    /// }
    /// ```
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait NSCopying {
        /// Returns a new instance that's a copy of the receiver.
        ///
        /// The output type is the immutable counterpart of the object, which
        /// is usually `Self`, but e.g. `NSMutableString` returns `NSString`.
        #[unsafe(method(copy))]
        #[unsafe(method_family = copy)]
        #[optional]
        fn copy(&self) -> Retained<Self::Result>
        where
            Self: CopyingHelper;

        /// Returns a new instance that's a copy of the receiver.
        ///
        /// This is only used when implementing `NSCopying`, call
        /// [`copy`][NSCopying::copy] instead.
        ///
        ///
        /// # Safety
        ///
        /// The zone pointer must be valid or NULL.
        #[unsafe(method(copyWithZone:))]
        #[unsafe(method_family = copy)]
        unsafe fn copyWithZone(&self, zone: *mut NSZone) -> Retained<Self::Result>
        where
            Self: CopyingHelper;
    }
);

extern_protocol!(
    /// A protocol to provide mutable copies of objects.
    ///
    /// Only classes that have an “immutable vs. mutable” distinction should
    /// adopt this protocol. Use the [`MutableCopyingHelper`] trait to specify
    /// the return type after copying.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/foundation/nsmutablecopying
    ///
    ///
    /// # Example
    ///
    /// Implement [`NSCopying`] and [`NSMutableCopying`] for a class pair like
    /// `NSString` and `NSMutableString`.
    ///
    /// ```ignore
    /// // Immutable copies return NSString
    ///
    /// unsafe impl NSCopying for NSString {}
    /// unsafe impl CopyingHelper for NSString {
    ///     type Result = NSString;
    /// }
    /// unsafe impl NSCopying for NSMutableString {}
    /// unsafe impl CopyingHelper for NSMutableString {
    ///     type Result = NSString;
    /// }
    ///
    /// // Mutable copies return NSMutableString
    ///
    /// unsafe impl NSMutableCopying for NSString {}
    /// unsafe impl MutableCopyingHelper for NSString {
    ///     type Result = NSMutableString;
    /// }
    /// unsafe impl NSMutableCopying for NSMutableString {}
    /// unsafe impl MutableCopyingHelper for NSMutableString {
    ///     type Result = NSMutableString;
    /// }
    /// ```
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait NSMutableCopying {
        /// Returns a new instance that's a mutable copy of the receiver.
        ///
        /// The output type is the mutable counterpart of the object. E.g. both
        /// `NSString` and `NSMutableString` return `NSMutableString`.
        #[unsafe(method(mutableCopy))]
        #[unsafe(method_family = mutableCopy)]
        #[optional]
        fn mutableCopy(&self) -> Retained<Self::Result>
        where
            Self: MutableCopyingHelper;

        /// Returns a new instance that's a mutable copy of the receiver.
        ///
        /// This is only used when implementing `NSMutableCopying`, call
        /// [`mutableCopy`][NSMutableCopying::mutableCopy] instead.
        ///
        ///
        /// # Safety
        ///
        /// The zone pointer must be valid or NULL.
        #[unsafe(method(mutableCopyWithZone:))]
        #[unsafe(method_family = mutableCopy)]
        unsafe fn mutableCopyWithZone(&self, zone: *mut NSZone) -> Retained<Self::Result>
        where
            Self: MutableCopyingHelper;
    }
);
