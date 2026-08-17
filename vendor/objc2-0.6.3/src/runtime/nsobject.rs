use core::fmt;
use core::hash;
use core::ops::Deref;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::ffi::NSUInteger;
use crate::rc::{Allocated, DefaultRetained, Retained};
use crate::runtime::{AnyClass, AnyObject, AnyProtocol, ImplementedBy, ProtocolObject, Sel};
use crate::{
    extern_conformance, extern_methods, msg_send, AnyThread, ClassType, DowncastTarget, Message,
    ProtocolType,
};

/// The root class of most Objective-C class hierarchies.
///
/// This represents the [`NSObject` class][cls]. The name "NSObject" also
/// refers to a protocol, see [`NSObjectProtocol`] for that.
///
/// This class has been defined in `objc` since macOS 10.8, but is also
/// re-exported under `objc2_foundation::NSObject`, you might want to use that
/// path instead.
///
/// [cls]: https://developer.apple.com/documentation/objectivec/nsobject?language=objc
#[repr(C)]
pub struct NSObject {
    __superclass: AnyObject,
}

// Would be _super_ nice to have this kind of impl, but that isn't possible.
// impl Unsize<AnyObject> for NSObject {}

crate::__extern_class_impl_traits! {
    ()
    (unsafe impl)
    (NSObject)
    (AnyObject)
}

// We do not want to expose this type publicly, even though it's exposed in
// the trait impl.
mod private {
    #[derive(PartialEq, Eq, Hash)] // Delegate to NSObject
    pub struct ForDefinedSubclasses(pub(super) super::NSObject);
}

impl fmt::Debug for private::ForDefinedSubclasses {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to NSObject
        fmt::Debug::fmt(&**self, f)
    }
}

impl Deref for private::ForDefinedSubclasses {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SAFETY: `NSObject` is thread safe by itself, the only reason why it
// isn't marked `Send` is because its subclasses may not be thread safe.
//
// But any subclass of NSObject that the user creates is thread safe, provided
// that their ivars are thread safe too, and the class is not configured as
// `MainThreadOnly`.
//
// This special casing of NSObject is similar to what Swift does:
// https://developer.apple.com/documentation/swift/sendable#Sendable-Classes
unsafe impl Send for private::ForDefinedSubclasses {}
// SAFETY: Same as above.
unsafe impl Sync for private::ForDefinedSubclasses {}

// NSObject itself is also unwind safe, and so subclasses should be too,
// unless they use e.g. interior mutability in their ivars.
impl UnwindSafe for private::ForDefinedSubclasses {}
impl RefUnwindSafe for private::ForDefinedSubclasses {}

unsafe impl ClassType for NSObject {
    type Super = AnyObject;
    type ThreadKind = dyn AnyThread;
    const NAME: &'static str = "NSObject";

    #[inline]
    fn class() -> &'static AnyClass {
        crate::__class_inner!("NSObject", "NSObject")
    }

    #[inline]
    fn as_super(&self) -> &Self::Super {
        &self.__superclass
    }

    const __INNER: () = ();

    // Defined subclasses can assume more lenient auto traits.
    type __SubclassingType = private::ForDefinedSubclasses;
}

unsafe impl DowncastTarget for NSObject {}

/// The methods that are fundamental to most Objective-C objects.
///
/// This represents the [`NSObject` protocol][proto].
///
/// You should rarely need to use this for anything other than as a trait
/// bound in [`extern_protocol!`], to allow your protocol to implement `Debug`
/// `Hash`, `PartialEq` and `Eq`.
///
/// This trait is exported under `objc2_foundation::NSObjectProtocol`, you
/// probably want to use that path instead.
///
/// [proto]: https://developer.apple.com/documentation/objectivec/1418956-nsobject?language=objc
/// [`extern_protocol!`]: crate::extern_protocol!
///
///
/// # Safety
///
/// Like with [other protocols](ProtocolType), the type must represent a class
/// that implements the `NSObject` protocol.
#[allow(non_snake_case)] // Follow the naming scheme in framework crates
pub unsafe trait NSObjectProtocol {
    /// Check whether the object is equal to an arbitrary other object.
    ///
    /// Most objects that implement `NSObjectProtocol` also implements the
    /// [`PartialEq`] trait. If the objects you are comparing are of the same
    /// type, you likely want to use that instead.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418795-isequal?language=objc
    #[doc(alias = "isEqual:")]
    fn isEqual(&self, other: Option<&AnyObject>) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, isEqual: other] }
    }

    /// An integer that can be used as a table address in a hash table
    /// structure.
    ///
    /// Most objects that implement `NSObjectProtocol` also implements the
    /// [`Hash`][std::hash::Hash] trait, you likely want to use that instead.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash?language=objc
    fn hash(&self) -> NSUInteger
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, hash] }
    }

    /// Check if the object is an instance of the class, or one of its
    /// subclasses.
    ///
    /// See [`AnyObject::downcast_ref`] or [`Retained::downcast`] if your
    /// intention is to use this to cast an object to another, and see
    /// [Apple's documentation][apple-doc] for more details on what you may
    /// (and what you may not) do with this information in general.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418511-iskindofclass?language=objc
    #[doc(alias = "isKindOfClass:")]
    fn isKindOfClass(&self, cls: &AnyClass) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, isKindOfClass: cls] }
    }

    /// Check if the object is an instance of the class type, or one of its
    /// subclasses.
    ///
    /// See [`isKindOfClass`][Self::isKindOfClass] for details.
    #[deprecated = "use `isKindOfClass` directly, or cast your objects with `AnyObject::downcast_ref`"]
    // TODO: Use extern_protocol! once we get rid of this
    fn is_kind_of<T: ClassType>(&self) -> bool
    where
        Self: Sized + Message,
    {
        self.isKindOfClass(T::class())
    }

    /// Check if the object is an instance of a specific class, without
    /// checking subclasses.
    ///
    /// Note that this is rarely what you want, the specific class of an
    /// object is considered a private implementation detail. Use
    /// [`isKindOfClass`][Self::isKindOfClass] instead to check whether an
    /// object is an instance of a given class.
    ///
    /// See [Apple's documentation][apple-doc] for more details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418766-ismemberofclass?language=objc
    #[doc(alias = "isMemberOfClass:")]
    fn isMemberOfClass(&self, cls: &AnyClass) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, isMemberOfClass: cls] }
    }

    /// Check whether the object implements or inherits a method with the
    /// given selector.
    ///
    /// See [Apple's documentation][apple-doc] for more details.
    ///
    /// If using this for availability checking, you might want to consider
    /// using the [`available!`] macro instead, as it is often more
    /// performant than this runtime check.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector?language=objc
    /// [`available!`]: crate::available
    #[doc(alias = "respondsToSelector:")]
    fn respondsToSelector(&self, aSelector: Sel) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, respondsToSelector: aSelector] }
    }

    /// Check whether the object conforms to a given protocol.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/nsobject/1418893-conformstoprotocol?language=objc
    #[doc(alias = "conformsToProtocol:")]
    fn conformsToProtocol(&self, aProtocol: &AnyProtocol) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, conformsToProtocol: aProtocol] }
    }

    /// A textual representation of the object.
    ///
    /// The returned class is `NSString`, but since that is defined in
    /// `objc2-foundation`, and `NSObjectProtocol` is defined in `objc2`, the
    /// declared return type is unfortunately restricted to be [`NSObject`].
    /// It is always safe to cast the return value of this to `NSString`.
    ///
    /// You might want to use the [`Debug`][fmt::Debug] impl of the object
    /// instead, or if the object implements [`Display`][fmt::Display], the
    /// [`to_string`][std::string::ToString::to_string] method.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::rc::Retained;
    /// # use objc2::runtime::{NSObjectProtocol, NSObject, NSObject as NSString};
    /// # #[cfg(available_in_foundation)]
    /// use objc2_foundation::{NSObject, NSObjectProtocol, NSString};
    ///
    /// # let obj = NSObject::new();
    /// // SAFETY: Descriptions are always `NSString`.
    /// let desc: Retained<NSString> = unsafe { Retained::cast_unchecked(obj.description()) };
    /// println!("{desc:?}");
    /// ```
    //
    // Only safe to override if the user-provided return type is NSString.
    fn description(&self) -> Retained<NSObject>
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, description] }
    }

    /// A textual representation of the object to use when debugging.
    ///
    /// Like with [`description`][Self::description], the return type of this
    /// is always `NSString`.
    ///
    /// LLVM's po command uses this property to create a textual
    /// representation of the object. The default implementation returns the
    /// same value as `description`. Override either to provide custom object
    /// descriptions.
    // optional, introduced in macOS 10.8
    //
    // Only safe to override if the user-provided return type is NSString.
    fn debugDescription(&self) -> Retained<NSObject>
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, debugDescription] }
    }

    /// Check whether the receiver is a subclass of the `NSProxy` root class
    /// instead of the usual [`NSObject`].
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418528-isproxy?language=objc
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::runtime::{NSObject, NSObjectProtocol};
    ///
    /// let obj = NSObject::new();
    /// assert!(!obj.isProxy());
    /// ```
    fn isProxy(&self) -> bool
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, isProxy] }
    }

    /// The reference count of the object.
    ///
    /// This can rarely be useful when debugging memory management issues,
    /// though beware that in most real-world scenarios, your object may be
    /// retained by several autorelease pools, especially when debug
    /// assertions are enabled, so this value may not represent what you'd
    /// expect.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::ClassType;
    /// use objc2::runtime::{NSObject, NSObjectProtocol};
    ///
    /// let obj = NSObject::new();
    /// assert_eq!(obj.retainCount(), 1);
    /// let obj2 = obj.clone();
    /// assert_eq!(obj.retainCount(), 2);
    /// drop(obj2);
    /// assert_eq!(obj.retainCount(), 1);
    /// ```
    fn retainCount(&self) -> NSUInteger
    where
        Self: Sized + Message,
    {
        unsafe { msg_send![self, retainCount] }
    }

    // retain, release and autorelease below to this protocol.
}

// SAFETY: Same as in extern_protocol!
unsafe impl<T> NSObjectProtocol for ProtocolObject<T> where T: ?Sized + NSObjectProtocol {}
// SAFETY: Same as in extern_protocol!
unsafe impl ProtocolType for dyn NSObjectProtocol {
    const NAME: &'static str = "NSObject";
    const __INNER: () = ();
}
// SAFETY: Same as in extern_protocol!
unsafe impl<T> ImplementedBy<T> for dyn NSObjectProtocol
where
    T: ?Sized + Message + NSObjectProtocol,
{
    const __INNER: () = ();
}

// SAFETY: Anything that implements `NSObjectProtocol` and is `Send` is valid
// to convert to `ProtocolObject<dyn NSObjectProtocol + Send>`.
unsafe impl<T> ImplementedBy<T> for dyn NSObjectProtocol + Send
where
    T: ?Sized + Message + NSObjectProtocol + Send,
{
    const __INNER: () = ();
}

// SAFETY: Anything that implements `NSObjectProtocol` and is `Sync` is valid
// to convert to `ProtocolObject<dyn NSObjectProtocol + Sync>`.
unsafe impl<T> ImplementedBy<T> for dyn NSObjectProtocol + Sync
where
    T: ?Sized + Message + NSObjectProtocol + Sync,
{
    const __INNER: () = ();
}

// SAFETY: Anything that implements `NSObjectProtocol` and is `Send + Sync` is
// valid to convert to `ProtocolObject<dyn NSObjectProtocol + Send + Sync>`.
unsafe impl<T> ImplementedBy<T> for dyn NSObjectProtocol + Send + Sync
where
    T: ?Sized + Message + NSObjectProtocol + Send + Sync,
{
    const __INNER: () = ();
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSObject {}
);

#[allow(non_snake_case)] // Follow the naming scheme in framework crates
impl NSObject {
    extern_methods!(
        /// Create a new empty `NSObject`.
        ///
        /// This method is a shorthand for calling [`alloc`][AnyThread::alloc]
        /// and then [`init`][Self::init].
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;

        /// Initialize an already allocated object.
        ///
        /// See [Apple's documentation][apple-doc] for details.
        ///
        /// [apple-doc]: https://developer.apple.com/documentation/objectivec/nsobject/1418641-init?language=objc
        ///
        ///
        /// # Example
        ///
        /// ```
        /// use objc2::runtime::NSObject;
        /// use objc2::AnyThread;
        ///
        /// let obj = NSObject::init(NSObject::alloc());
        /// ```
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(doesNotRecognizeSelector:))]
        #[unsafe(method_family = none)]
        fn doesNotRecognizeSelector_inner(&self, sel: Sel);
    );

    /// Handle messages the object doesn’t recognize.
    ///
    /// See [Apple's documentation][apple-doc] for details.
    ///
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/nsobject/1418637-doesnotrecognizeselector?language=objc
    pub fn doesNotRecognizeSelector(&self, sel: Sel) -> ! {
        self.doesNotRecognizeSelector_inner(sel);
        unreachable!("doesNotRecognizeSelector: should not return")
    }
}

/// Objective-C equality has approximately the same semantics as Rust
/// equality (although less aptly specified).
///
/// At the very least, equality is _expected_ to be symmetric and
/// transitive, and that's about the best we can do.
///
/// See also <https://nshipster.com/equality/>
impl PartialEq for NSObject {
    #[inline]
    #[doc(alias = "isEqual:")]
    fn eq(&self, other: &Self) -> bool {
        self.isEqual(Some(other))
    }
}

/// Most types' equality is reflexive.
impl Eq for NSObject {}

/// Hashing in Objective-C has the exact same requirement as in Rust:
///
/// > If two objects are equal (as determined by the isEqual: method),
/// > they must have the same hash value.
///
/// See <https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash>
impl hash::Hash for NSObject {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        <Self as NSObjectProtocol>::hash(self).hash(state);
    }
}

impl fmt::Debug for NSObject {
    #[inline]
    #[doc(alias = "description")]
    #[doc(alias = "debugDescription")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &ProtocolObject<dyn NSObjectProtocol> = ProtocolObject::from_ref(self);
        obj.fmt(f)
    }
}

impl DefaultRetained for NSObject {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    use crate::extern_class;
    use crate::rc::RcTestObject;

    extern_class!(
        #[unsafe(super(NSObject))]
        #[name = "NSObject"]
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct FakeSubclass;
    );

    impl FakeSubclass {
        fn new() -> Retained<Self> {
            Retained::downcast(NSObject::new()).unwrap()
        }
    }

    #[test]
    fn test_deref() {
        let obj: Retained<FakeSubclass> = FakeSubclass::new();
        let _: &FakeSubclass = &obj;
        let _: &NSObject = &obj;
        let _: &AnyObject = &obj;
    }

    #[test]
    fn test_as_ref_borrow() {
        use core::borrow::Borrow;

        fn impls_as_ref_borrow<T: AsRef<U> + Borrow<U> + ?Sized, U: ?Sized>(_: &T) {}

        let obj = FakeSubclass::new();
        impls_as_ref_borrow::<Retained<FakeSubclass>, FakeSubclass>(&obj);
        impls_as_ref_borrow::<FakeSubclass, FakeSubclass>(&obj);
        impls_as_ref_borrow::<NSObject, NSObject>(&obj);
        impls_as_ref_borrow::<NSObject, AnyObject>(&obj);

        let obj = NSObject::new();
        impls_as_ref_borrow::<Retained<NSObject>, NSObject>(&obj);

        fn impls_as_ref<T: AsRef<U> + ?Sized, U: ?Sized>(_: &T) {}

        let obj = FakeSubclass::new();
        impls_as_ref::<Retained<FakeSubclass>, FakeSubclass>(&obj);
        impls_as_ref::<Retained<FakeSubclass>, NSObject>(&obj);
        impls_as_ref::<Retained<FakeSubclass>, AnyObject>(&obj);
    }

    #[test]
    fn test_equality() {
        let obj1 = NSObject::new();
        assert_eq!(obj1, obj1);

        let obj2 = NSObject::new();
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_hash() {
        use core::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let obj1 = NSObject::new();

        let mut hashstate1 = DefaultHasher::new();
        let mut hashstate2 = DefaultHasher::new();

        obj1.hash(&mut hashstate1);
        obj1.hash(&mut hashstate2);

        assert_eq!(hashstate1.finish(), hashstate2.finish());

        let obj2 = NSObject::new();
        let mut hashstate2 = DefaultHasher::new();
        obj2.hash(&mut hashstate2);
        assert_ne!(hashstate1.finish(), hashstate2.finish());
    }

    #[test]
    fn test_debug() {
        let obj = NSObject::new();
        let expected = format!("<NSObject: {:p}>", &*obj);
        assert_eq!(format!("{obj:?}"), expected);
    }

    #[test]
    fn test_is_kind_of() {
        let obj = NSObject::new();
        assert!(obj.isKindOfClass(NSObject::class()));
        assert!(!obj.isKindOfClass(RcTestObject::class()));

        let obj = RcTestObject::new();
        assert!(obj.isKindOfClass(NSObject::class()));
        assert!(obj.isKindOfClass(RcTestObject::class()));
    }

    #[test]
    fn test_retain_same() {
        let obj1 = NSObject::new();
        let ptr1 = Retained::as_ptr(&obj1);

        let obj2 = obj1.clone();
        let ptr2 = Retained::as_ptr(&obj2);

        assert_eq!(ptr1, ptr2);
    }

    #[test]
    fn conforms_to_nsobjectprotocol() {
        let protocol = <dyn NSObjectProtocol>::protocol().unwrap();
        assert!(NSObject::class().conforms_to(protocol));
    }

    // Ensure that importing `NSObjectProtocol::hash` does not cause conflicts
    // when using `Hash::hash` on normal types.
    mod hash_does_not_overlap_with_normal_hash_method {
        #[allow(unused_imports)]
        use crate::runtime::NSObjectProtocol;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        #[test]
        fn inner() {
            let integer = 5;
            let mut hasher = DefaultHasher::new();
            integer.hash(&mut hasher);
        }
    }
}
