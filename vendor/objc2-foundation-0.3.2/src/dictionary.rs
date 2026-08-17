//! Utilities for the `NSDictionary` and `NSMutableDictionary` classes.
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::fmt;
use core::mem;
use core::ptr::NonNull;
use objc2::msg_send;

use objc2::rc::Retained;
#[cfg(feature = "NSObject")]
use objc2::runtime::ProtocolObject;
#[cfg(feature = "NSObject")]
use objc2::AnyThread;
use objc2::Message;

#[cfg(feature = "NSEnumerator")]
use crate::iter;
#[cfg(feature = "NSObject")]
use crate::{util, CopyingHelper, NSCopying};
use crate::{NSDictionary, NSMutableDictionary};

#[cfg(feature = "NSObject")]
fn keys_to_ptr<CopiedKey>(keys: &[&CopiedKey]) -> *mut NonNull<ProtocolObject<dyn NSCopying>>
where
    CopiedKey: Message + NSCopying,
{
    let keys: *mut NonNull<CopiedKey> = util::ref_ptr_cast_const(keys.as_ptr());
    // SAFETY: `CopiedKey` is `Message + NSCopying`, and is therefore safe to cast to
    // `ProtocolObject<dyn NSCopying>`.
    let keys: *mut NonNull<ProtocolObject<dyn NSCopying>> = keys.cast();
    keys
}

/// Convenience creation methods.
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    /// Create a new dictionary from a slice of keys, and a slice of objects.
    ///
    /// This is a safe interface to `initWithObjects:forKeys:count:`.
    ///
    /// # Panics
    ///
    /// Panics if the slices have different lengths, as this is likely a
    /// programmer error.
    ///
    /// # Example
    ///
    /// ```
    /// use objc2_foundation::{NSDictionary, ns_string};
    ///
    /// let dict = NSDictionary::from_slices(
    ///     &[ns_string!("key1"), ns_string!("key2"), ns_string!("key3")],
    ///     &[ns_string!("value1"), ns_string!("value2"), ns_string!("value3")],
    /// );
    ///
    /// assert_eq!(&*dict.objectForKey(ns_string!("key2")).unwrap(), ns_string!("value2"));
    /// ```
    #[cfg(feature = "NSObject")]
    pub fn from_slices<CopiedKey>(keys: &[&CopiedKey], objects: &[&ObjectType]) -> Retained<Self>
    where
        // The dictionary copies its keys, which is why we require `NSCopying`
        // and use `CopyingHelper` on all input data - we want to ensure that
        // the type-system knows that it's not actually e.g. `NSMutableString`
        // that is being stored, but instead `NSString`.
        CopiedKey: Message + NSCopying + CopyingHelper<Result = KeyType>,
    {
        // Ensure that we don't read too far into one of the buffers.
        assert_eq!(
            keys.len(),
            objects.len(),
            "key slice and object slice should have the same length",
        );
        let count = keys.len();

        let keys = keys_to_ptr(keys);
        let objects = util::ref_ptr_cast_const(objects.as_ptr());

        // SAFETY:
        // - All types that are `Message` use interior mutability, and the
        //   dictionary extends the lifetime of them internally by retaining
        //   them.
        //
        // - The pointers are valid until the method has finished executing,
        //   at which point the dictionary will have created its own internal
        //   storage for holding the keys and objects.
        //
        // - The length is lower than or equal to the length of the two
        //   pointers.
        //
        // - While recommended against in the below link, the key _can_ be
        //   mutated, it'll "just" corrupt the collection's invariants (but
        //   won't cause undefined behaviour).
        //
        //   This is tested via fuzzing in `collection_interior_mut.rs`.
        //
        //   <https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/ObjectMutability/ObjectMutability.html#//apple_ref/doc/uid/TP40010810-CH5-SW69>
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }

    #[cfg(feature = "NSObject")]
    pub fn from_retained_objects<CopiedKey>(
        keys: &[&CopiedKey],
        objects: &[Retained<ObjectType>],
    ) -> Retained<Self>
    where
        CopiedKey: Message + NSCopying + CopyingHelper<Result = KeyType>,
    {
        // Ensure that we don't read too far into one of the buffers.
        assert_eq!(
            keys.len(),
            objects.len(),
            "key slice and object slice should have the same length",
        );
        let count = keys.len();

        let keys = keys_to_ptr(keys);
        let objects = util::retained_ptr_cast_const(objects.as_ptr());

        // SAFETY: Same as `from_slices`.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }
}

/// Convenience creation methods.
impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
    #[cfg(feature = "NSObject")]
    pub fn from_slices<CopiedKey>(keys: &[&CopiedKey], objects: &[&ObjectType]) -> Retained<Self>
    where
        CopiedKey: Message + NSCopying + CopyingHelper<Result = KeyType>,
    {
        // Ensure that we don't read too far into one of the buffers.
        assert_eq!(
            keys.len(),
            objects.len(),
            "key slice and object slice should have the same length",
        );
        let count = keys.len();

        let keys = keys_to_ptr(keys);
        let objects = util::ref_ptr_cast_const(objects.as_ptr());

        // SAFETY: Same as `NSDictionary::from_slices`.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }

    #[cfg(feature = "NSObject")]
    pub fn from_retained_objects<CopiedKey>(
        keys: &[&CopiedKey],
        objects: &[Retained<ObjectType>],
    ) -> Retained<Self>
    where
        CopiedKey: Message + NSCopying + CopyingHelper<Result = KeyType>,
    {
        // Ensure that we don't read too far into one of the buffers.
        assert_eq!(
            keys.len(),
            objects.len(),
            "key slice and object slice should have the same length",
        );
        let count = keys.len();

        let keys = keys_to_ptr(keys);
        let objects = util::retained_ptr_cast_const(objects.as_ptr());

        // SAFETY: Same as `NSDictionary::from_retained_objects`.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }
}

// Note: We'd like to make getter methods take `KeyType: Borrow<KeyType>`
// like `std::collections::HashMap`, so that e.g. `NSDictionary<NSString, V>`
// could take a `&NSObject` as input, and still make that work since
// `NSString` borrows to `NSObject`.
//
// But we can't really, at least not with extra `unsafe` / an extra trait,
// since we don't control how the comparisons happen.
//
// The most useful alternative would probably be to take `impl AsRef<KeyType>`, but
// objc2 classes deref to their superclass anyhow, so let's just use a simple,
// normal reference.

/// Direct, unsafe object accessors.
///
/// Foundation's collection types store their items in such a way that they
/// can give out references to their data without having to autorelease it
/// first, see [the docs][collections-own].
///
/// This means that we can more efficiently access the dictionary's keys and
/// objects, but _only_ if the dictionary isn't mutated via e.g.
/// `NSMutableDictionary` methods while doing so - otherwise, we might end up
/// accessing a deallocated object.
///
/// [collections-own]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmPractical.html#//apple_ref/doc/uid/TP40004447-SW12
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    /// Get a direct reference to the object corresponding to the key.
    ///
    /// Consider using the [`objectForKey`](Self::objectForKey) method
    /// instead, unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The dictionary must not be mutated while the reference is live.
    #[doc(alias = "objectForKey:")]
    #[inline]
    pub unsafe fn objectForKey_unchecked(&self, key: &KeyType) -> Option<&ObjectType> {
        unsafe { msg_send![self, objectForKey: key] }
    }

    /// Two vectors containing direct references to respectively the
    /// dictionary's keys and objects.
    ///
    /// Consider using the [`to_vecs`](Self::to_vecs) method instead, unless
    /// you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The dictionary must not be mutated while the returned references are
    /// alive.
    #[doc(alias = "getObjects:andKeys:")]
    #[cfg(feature = "alloc")]
    pub unsafe fn to_vecs_unchecked(&self) -> (Vec<&KeyType>, Vec<&ObjectType>) {
        let len = self.len();
        let mut keys = Vec::with_capacity(len);
        let mut objs = Vec::with_capacity(len);

        // SAFETY: The pointers are valid.
        unsafe {
            // No reason to use `getObjects:andKeys:count:`, the dictionary is
            // not thread safe, so we know it won't change within this scope.
            #[allow(deprecated)]
            self.getObjects_andKeys(objs.as_mut_ptr(), keys.as_mut_ptr());
        }

        // SAFETY: The vecs were just initialized by `getObjects:andKeys:`.
        unsafe {
            keys.set_len(len);
            objs.set_len(len);
        }

        // SAFETY: `NonNull<T>` and `&T` have the same memory layout, and the
        // lifetime is upheld by the caller.
        unsafe {
            (
                mem::transmute::<Vec<NonNull<KeyType>>, Vec<&KeyType>>(keys),
                mem::transmute::<Vec<NonNull<ObjectType>>, Vec<&ObjectType>>(objs),
            )
        }
    }

    /// Iterate over the dictionary's keys without retaining them.
    ///
    /// Consider using the [`keys`](Self::keys) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The dictionary must not be mutated for the lifetime of the iterator,
    /// or the elements it returns.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "keyEnumerator")]
    #[inline]
    pub unsafe fn keys_unchecked(&self) -> KeysUnchecked<'_, KeyType, ObjectType> {
        KeysUnchecked(iter::IterUnchecked::new(self))
    }

    /// Iterate over the dictionary's objects / values without retaining them.
    ///
    /// Consider using the [`objects`](Self::objects) method instead, unless
    /// you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The dictionary must not be mutated for the lifetime of the iterator,
    /// or the elements it returns.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub unsafe fn objects_unchecked(&self) -> ObjectsUnchecked<'_, KeyType, ObjectType> {
        // SAFETY: Avoiding mutation is upheld by caller.
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        ObjectsUnchecked(unsafe { iter::IterUncheckedWithBackingEnum::new(self, enumerator) })
    }
}

/// Various accessor methods.
impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
    /// The amount of elements in the dictionary.
    #[doc(alias = "count")]
    #[inline]
    pub fn len(&self) -> usize {
        self.count()
    }

    /// Whether the dictionary is empty or not.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Two vectors containing respectively the dictionary's keys and objects.
    ///
    /// # Example
    ///
    /// Iterate over the keys and values of the dictionary.
    ///
    /// ```
    /// use objc2_foundation::{NSDictionary, ns_string};
    ///
    /// let dict = NSDictionary::from_slices(
    ///     &[ns_string!("a"), ns_string!("b")],
    ///     &[ns_string!("a"), ns_string!("b")],
    /// );
    /// let (keys, objects) = dict.to_vecs();
    /// for (key, obj) in keys.into_iter().zip(objects) {
    ///     assert_eq!(key, obj);
    /// }
    /// ```
    #[doc(alias = "getObjects:")]
    #[cfg(feature = "alloc")]
    pub fn to_vecs(&self) -> (Vec<Retained<KeyType>>, Vec<Retained<ObjectType>>) {
        // SAFETY: We retain the elements below, so that we know that the
        // dictionary isn't mutated while they are alive.
        let (keys, objects) = unsafe { self.to_vecs_unchecked() };
        (
            keys.into_iter().map(KeyType::retain).collect(),
            objects.into_iter().map(ObjectType::retain).collect(),
        )
    }

    /// Iterate over the dictionary's keys.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "keyEnumerator")]
    #[inline]
    pub fn keys(&self) -> Keys<'_, KeyType, ObjectType> {
        Keys(iter::Iter::new(self))
    }

    /// Iterate over the dictionary's objects / values.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "NSString", doc = "```")]
    #[cfg_attr(not(feature = "NSString"), doc = "```ignore")]
    /// use objc2_foundation::{ns_string, NSMutableDictionary, NSString};
    ///
    /// let dict = NSMutableDictionary::new();
    /// dict.insert(ns_string!("key1"), ns_string!("value1"));
    /// dict.insert(ns_string!("key2"), ns_string!("value2"));
    /// for obj in dict.objects() {
    ///     assert!(obj.hasPrefix(ns_string!("value")));
    /// }
    /// ```
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn objects(&self) -> Objects<'_, KeyType, ObjectType> {
        // SAFETY: The iterator checks for mutation while enumerating.
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        Objects(unsafe { iter::IterWithBackingEnum::new(self, enumerator) })
    }
}

/// Convenience mutation methods.
impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
    /// Inserts a key-value pair into the dictionary.
    ///
    /// If the dictionary did not have this key present, the value is
    /// inserted. If the dictionary already had this key present, the value
    /// and the key is updated.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSMutableDictionary, NSObject};
    ///
    /// let dict = NSMutableDictionary::new();
    /// dict.insert(ns_string!("key"), &*NSObject::new());
    /// ```
    #[cfg(feature = "NSObject")]
    #[doc(alias = "setObject:forKey:")]
    #[inline]
    pub fn insert<CopiedKey>(&self, key: &CopiedKey, object: &ObjectType)
    where
        CopiedKey: Message + NSCopying + CopyingHelper<Result = KeyType>,
    {
        let key = ProtocolObject::from_ref(key);
        // SAFETY: The key is copied, and then has the correct type `KeyType`.
        unsafe { self.setObject_forKey(object, key) };
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<KeyType: Message, ObjectType: Message> iter::FastEnumerationHelper
    for NSDictionary<KeyType, ObjectType>
{
    // Fast enumeration for dictionaries returns the keys.
    type Item = KeyType;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<KeyType: Message, ObjectType: Message> iter::FastEnumerationHelper
    for NSMutableDictionary<KeyType, ObjectType>
{
    // The same goes for mutable dictionaries.
    type Item = KeyType;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the keys of a dictionary.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct Keys<'a, KeyType: Message, ObjectType: Message>(
    iter::Iter<'a, NSDictionary<KeyType, ObjectType>>,
);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, KeyType: Message, ObjectType: Message> Iterator<Item = Retained<KeyType>> for Keys<'a, KeyType, ObjectType> { ... }
}

/// An iterator over unretained keys of a dictionary.
///
/// # Safety
///
/// The dictionary must not be mutated while this is alive.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct KeysUnchecked<'a, KeyType: Message, ObjectType: Message>(
    iter::IterUnchecked<'a, NSDictionary<KeyType, ObjectType>>,
);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, KeyType: Message, ObjectType: Message> Iterator<Item = &'a KeyType> for KeysUnchecked<'a, KeyType, ObjectType> { ... }
}

/// An iterator over the objects / values in a dictionary.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct Objects<'a, KeyType: Message, ObjectType: Message>(
    iter::IterWithBackingEnum<
        'a,
        NSDictionary<KeyType, ObjectType>,
        crate::NSEnumerator<ObjectType>,
    >,
);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, KeyType: Message, ObjectType: Message> Iterator<Item = Retained<ObjectType>> for Objects<'a, KeyType, ObjectType> { ... }
}

/// An iterator over unretained objects / values of a dictionary.
///
/// # Safety
///
/// The dictionary must not be mutated while this is alive.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct ObjectsUnchecked<'a, KeyType: Message, ObjectType: Message + 'a>(
    iter::IterUncheckedWithBackingEnum<
        'a,
        NSDictionary<KeyType, ObjectType>,
        crate::NSEnumerator<ObjectType>,
    >,
);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, KeyType: Message, ObjectType: Message> Iterator<Item = &'a ObjectType> for ObjectsUnchecked<'a, KeyType, ObjectType> { ... }
}

impl<KeyType: fmt::Debug + Message, ObjectType: fmt::Debug + Message> fmt::Debug
    for NSDictionary<KeyType, ObjectType>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: Unsound, use `to_vecs` instead when that doesn't have extra bounds
        let (keys, objects) = unsafe { self.to_vecs_unchecked() };
        let iter = keys.into_iter().zip(objects);
        f.debug_map().entries(iter).finish()
    }
}

impl<KeyType: fmt::Debug + Message, ObjectType: fmt::Debug + Message> fmt::Debug
    for NSMutableDictionary<KeyType, ObjectType>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

/// NSDictionaryCreation with known `NSString` key (because they load PLists).
#[cfg(feature = "NSString")]
impl<ObjectType: Message> NSDictionary<crate::NSString, ObjectType> {
    objc2::extern_methods!(
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[unsafe(method(initWithContentsOfURL:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithContentsOfURL_error(
            this: objc2::rc::Allocated<Self>,
            url: &crate::NSURL,
        ) -> Result<Retained<Self>, Retained<crate::NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[unsafe(method(dictionaryWithContentsOfURL:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn dictionaryWithContentsOfURL_error(
            url: &crate::NSURL,
        ) -> Result<Retained<Self>, Retained<crate::NSError>>;
    );
}
