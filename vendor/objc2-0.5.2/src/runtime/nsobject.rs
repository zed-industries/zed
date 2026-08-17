use core::fmt;
use core::hash;

use crate::ffi::NSUInteger;
use crate::mutability::Root;
use crate::rc::{Allocated, DefaultRetained, Retained};
use crate::runtime::{AnyClass, AnyObject, AnyProtocol, ImplementedBy, ProtocolObject, Sel};
use crate::{extern_methods, msg_send, msg_send_id, Message};
use crate::{ClassType, ProtocolType};

/// The root class of most Objective-C class hierarchies.
///
/// This represents the [`NSObject` class][cls]. The name "NSObject" also
/// refers to a protocol, see [`NSObjectProtocol`] for that.
///
/// Since this class is only available with the `Foundation` framework,
/// `objc2` links to it for you.
///
/// This is exported under `objc2_foundation::NSObject`, you probably
/// want to use that path instead.
///
/// [cls]: https://developer.apple.com/documentation/objectivec/nsobject?language=objc
#[repr(C)]
pub struct NSObject {
    __inner: AnyObject,
}

crate::__extern_class_impl_traits! {
    unsafe impl () for NSObject {
        INHERITS = [AnyObject];

        fn as_super(&self) {
            &self.__inner
        }

        fn as_super_mut(&mut self) {
            &mut self.__inner
        }
    }
}

unsafe impl ClassType for NSObject {
    type Super = AnyObject;
    type Mutability = Root;
    const NAME: &'static str = "NSObject";

    #[inline]
    fn class() -> &'static AnyClass {
        crate::__class_inner!("NSObject", "NSObject")
    }

    #[inline]
    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    #[inline]
    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}

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
//
// Note: Most of the methods on this must remain `unsafe` to override,
// including `isEqual` and `hash`, since hashing collections like
// `NSDictionary` and `NSSet` rely on it being stable.
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
    fn isEqual(&self, other: &AnyObject) -> bool
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
    /// See [Apple's documentation][apple-doc] for more details on what you
    /// may (and what you may not) do with this information.
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
    #[doc(alias = "isKindOfClass")]
    #[doc(alias = "isKindOfClass:")]
    // TODO: Consider deprecating this
    fn is_kind_of<T: ClassType>(&self) -> bool
    where
        Self: Sized + Message,
    {
        self.isKindOfClass(T::class())
    }

    // Note: We don't provide a method to convert `NSObject` to `T` based on
    // `is_kind_of`, since that is not possible to do in general!
    //
    // For example, something may have a return type of `NSString`, while
    // behind the scenes they really return `NSMutableString` and expect it to
    // not be modified.

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
    /// [apple-doc]: https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector?language=objc
    ///
    ///
    /// # Example
    ///
    /// Check whether `NSApplication` has the [`effectiveAppearance`] method
    /// before calling it, to support systems older than macOS 10.14 where the
    /// method was added.
    ///
    /// ```
    /// # #[cfg(available_in_frameworks)]
    /// use objc2_app_kit::{NSApplication, NSAppearance, NSAppearanceNameAqua};
    /// use objc2::runtime::NSObjectProtocol;
    /// use objc2::sel;
    ///
    /// # let obj = objc2::runtime::NSObject::new();
    /// # assert!(!obj.respondsToSelector(sel!(effectiveAppearance)));
    /// #
    /// # #[cfg(available_in_frameworks)] {
    /// let appearance = if obj.respondsToSelector(sel!(effectiveAppearance)) {
    ///     NSApplication::sharedApplication(mtm).effectiveAppearance()
    /// } else {
    ///     unsafe { NSAppearance::appearanceNamed(NSAppearanceNameAqua).unwrap() }
    /// };
    /// # }
    /// ```
    ///
    /// [`effectiveAppearance`]: https://developer.apple.com/documentation/appkit/nsapplication/2967171-effectiveappearance?language=objc
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
    /// let desc: Retained<NSString> = unsafe { Retained::cast(obj.description()) };
    /// println!("{desc:?}");
    /// ```
    fn description(&self) -> Retained<NSObject>
    where
        Self: Sized + Message,
    {
        unsafe { msg_send_id![self, description] }
    }

    /// A textual representation of the object to use when debugging.
    ///
    /// Like with [`description`][Self::description], the return type of this
    /// is always `NSString`.
    ///
    /// LLVM's po command uses this property to create a textual
    /// representation of the object. The default implemention returns the
    /// same value as `description`. Override either to provide custom object
    /// descriptions.
    // optional, introduced in macOS 10.8
    fn debugDescription(&self) -> Retained<NSObject>
    where
        Self: Sized + Message,
    {
        unsafe { msg_send_id![self, debugDescription] }
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
}

crate::__inner_extern_protocol!(
    ()
    (NSObjectProtocol)
    (dyn NSObjectProtocol)
    ("NSObject")
);

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

unsafe impl NSObjectProtocol for NSObject {}

extern_methods!(
    #[allow(non_snake_case)] // Follow the naming scheme in framework crates
    unsafe impl NSObject {
        /// Create a new empty `NSObject`.
        ///
        /// This method is a shorthand for calling [`alloc`][ClassType::alloc]
        /// and then [`init`][Self::init].
        #[method_id(new)]
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
        /// use objc2::ClassType;
        ///
        /// let obj = NSObject::init(NSObject::alloc());
        /// ```
        #[method_id(init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(doesNotRecognizeSelector:)]
        fn doesNotRecognizeSelector_inner(&self, sel: Sel);

        /// Handle messages the object doesn’t recognize.
        ///
        /// See [Apple's documentation][apple-doc] for details.
        ///
        /// [apple-doc]: https://developer.apple.com/documentation/objectivec/nsobject/1418637-doesnotrecognizeselector?language=objc
        pub fn doesNotRecognizeSelector(&self, sel: Sel) -> ! {
            self.doesNotRecognizeSelector_inner(sel);
            unreachable!("doesNotRecognizeSelector: should not return")
        }

        // TODO: `methodForSelector:`, but deprecated, showing how you should do without?
    }
);

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
        self.isEqual(other)
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
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    use crate::extern_class;
    use crate::mutability::Mutable;
    use crate::rc::RcTestObject;

    extern_class!(
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct NSObjectMutable;

        unsafe impl ClassType for NSObjectMutable {
            type Super = NSObject;
            type Mutability = Mutable;
            const NAME: &'static str = "NSObject";
        }
    );

    impl NSObjectMutable {
        fn new() -> Retained<Self> {
            unsafe { Retained::cast(NSObject::new()) }
        }
    }

    #[test]
    fn test_deref() {
        let obj: Retained<NSObject> = NSObject::new();
        let _: &NSObject = &obj;
        let _: &AnyObject = &obj;
    }

    #[test]
    fn test_deref_mut() {
        let mut obj: Retained<NSObjectMutable> = NSObjectMutable::new();
        let _: &NSObjectMutable = &obj;
        let _: &mut NSObjectMutable = &mut obj;
        let _: &NSObject = &obj;
        let _: &mut NSObject = &mut obj;
        let _: &AnyObject = &obj;
        let _: &mut AnyObject = &mut obj;
    }

    #[test]
    fn test_as_ref_borrow() {
        use core::borrow::{Borrow, BorrowMut};

        fn impls_as_ref<T: AsRef<U> + Borrow<U> + ?Sized, U: ?Sized>(_: &T) {}
        fn impls_as_mut<T: AsMut<U> + BorrowMut<U> + ?Sized, U: ?Sized>(_: &mut T) {}

        let mut obj = NSObjectMutable::new();
        impls_as_ref::<Retained<NSObjectMutable>, NSObjectMutable>(&obj);
        impls_as_mut::<Retained<NSObjectMutable>, NSObjectMutable>(&mut obj);
        impls_as_ref::<NSObjectMutable, NSObjectMutable>(&obj);
        impls_as_mut::<NSObjectMutable, NSObjectMutable>(&mut obj);
        impls_as_ref::<NSObject, NSObject>(&obj);
        impls_as_mut::<NSObject, NSObject>(&mut obj);
        impls_as_ref::<NSObject, AnyObject>(&obj);
        impls_as_mut::<NSObject, AnyObject>(&mut obj);

        let obj = NSObject::new();
        impls_as_ref::<Retained<NSObject>, NSObject>(&obj);
        impls_as_ref::<NSObject, NSObject>(&obj);
        impls_as_ref::<NSObject, AnyObject>(&obj);
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
        assert!(obj.is_kind_of::<NSObject>());
        assert!(!obj.is_kind_of::<RcTestObject>());

        let obj = RcTestObject::new();
        assert!(obj.is_kind_of::<NSObject>());
        assert!(obj.is_kind_of::<RcTestObject>());
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
