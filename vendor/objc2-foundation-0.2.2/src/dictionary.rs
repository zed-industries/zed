//! Utilities for the `NSDictionary` and `NSMutableDictionary` classes.
use alloc::vec::Vec;
#[cfg(feature = "NSObject")]
use core::cmp::min;
use core::fmt;
use core::hash::Hash;
use core::mem;
use core::ops::{Index, IndexMut};
use core::ptr::{self, NonNull};

#[cfg(feature = "NSObject")]
use objc2::mutability::IsRetainable;
use objc2::mutability::{CounterpartOrSelf, HasStableHash, IsIdCloneable, IsMutable};
use objc2::rc::Retained;
#[cfg(feature = "NSObject")]
use objc2::runtime::ProtocolObject;
#[cfg(feature = "NSObject")]
use objc2::ClassType;
use objc2::{extern_methods, Message};

#[cfg(feature = "NSEnumerator")]
use super::iter;
use super::util;
#[cfg(feature = "NSObject")]
use crate::Foundation::NSCopying;
use crate::Foundation::{NSDictionary, NSMutableDictionary};

#[cfg(feature = "NSObject")]
fn keys_to_ptr<Q>(keys: &[&Q]) -> *mut NonNull<ProtocolObject<dyn NSCopying>>
where
    Q: Message + NSCopying,
{
    let keys: *mut NonNull<Q> = util::ref_ptr_cast_const(keys.as_ptr());
    // SAFETY: `Q` is `Message + NSCopying`, and is therefore safe to cast to
    // `ProtocolObject<dyn NSCopying>`.
    let keys: *mut NonNull<ProtocolObject<dyn NSCopying>> = keys.cast();
    keys
}

#[cfg(feature = "NSObject")]
impl<K: Message + Eq + Hash + HasStableHash, V: Message> NSDictionary<K, V> {
    pub fn from_vec<Q>(keys: &[&Q], mut objects: Vec<Retained<V>>) -> Retained<Self>
    where
        Q: Message + NSCopying + CounterpartOrSelf<Immutable = K>,
    {
        // Find the minimum of the two provided lengths, to ensure that we
        // don't read too far in one of the buffers.
        //
        // Note: We could also have chosen to just panic here if the buffers have
        // different lengths, either would be fine.
        let count = min(keys.len(), objects.len());

        let keys = keys_to_ptr(keys);
        let objects = util::retained_ptr_cast(objects.as_mut_ptr());

        // SAFETY:
        // - The objects are valid, similar reasoning as `NSArray::from_vec`.
        //
        // - The key must not be mutated, as that may cause the hash value to
        //   change, which is unsound as stated in:
        //   https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/ObjectMutability/ObjectMutability.html#//apple_ref/doc/uid/TP40010810-CH5-SW69
        //
        //   The dictionary always copies its keys, which is why we require
        //   `NSCopying` and use `CounterpartOrSelf` on all input data - we
        //   want to ensure that it is very clear that it's not actually
        //   `NSMutableString` that is being stored, but `NSString`.
        //
        //   But that is not by itself enough to verify that the key does not
        //   still contain interior mutable objects (e.g. if the copy was only
        //   a shallow copy), which is why we also require `HasStableHash`.
        //
        // - The length is lower than or equal to the length of the two arrays.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }

    pub fn from_id_slice<Q>(keys: &[&Q], objects: &[Retained<V>]) -> Retained<Self>
    where
        Q: Message + NSCopying + CounterpartOrSelf<Immutable = K>,
        V: IsIdCloneable,
    {
        let count = min(keys.len(), objects.len());

        let keys = keys_to_ptr(keys);
        let objects = util::retained_ptr_cast_const(objects.as_ptr());

        // SAFETY: See `NSDictionary::from_vec` and `NSArray::from_id_slice`.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }

    pub fn from_slice<Q>(keys: &[&Q], objects: &[&V]) -> Retained<Self>
    where
        Q: Message + NSCopying + CounterpartOrSelf<Immutable = K>,
        V: IsRetainable,
    {
        let count = min(keys.len(), objects.len());

        let keys = keys_to_ptr(keys);
        let objects = util::ref_ptr_cast_const(objects.as_ptr());

        // SAFETY: See `NSDictionary::from_vec` and `NSArray::from_slice`.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }
}

#[cfg(feature = "NSObject")]
impl<K: Message + Eq + Hash + HasStableHash, V: Message> NSMutableDictionary<K, V> {
    pub fn from_vec<Q>(keys: &[&Q], mut objects: Vec<Retained<V>>) -> Retained<Self>
    where
        Q: Message + NSCopying + CounterpartOrSelf<Immutable = K>,
    {
        let count = min(keys.len(), objects.len());

        let keys: *mut NonNull<Q> = util::ref_ptr_cast_const(keys.as_ptr());
        let keys: *mut NonNull<ProtocolObject<dyn NSCopying>> = keys.cast();
        let objects = util::retained_ptr_cast(objects.as_mut_ptr());

        // SAFETY: See `NSDictionary::from_vec`
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }

    pub fn from_id_slice<Q>(keys: &[&Q], objects: &[Retained<V>]) -> Retained<Self>
    where
        Q: Message + NSCopying + CounterpartOrSelf<Immutable = K>,
        V: IsIdCloneable,
    {
        let count = min(keys.len(), objects.len());

        let keys = keys_to_ptr(keys);
        let objects = util::retained_ptr_cast_const(objects.as_ptr());

        // SAFETY: See `NSDictionary::from_vec` and `NSArray::from_id_slice`.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }

    pub fn from_slice<Q>(keys: &[&Q], objects: &[&V]) -> Retained<Self>
    where
        Q: Message + NSCopying + CounterpartOrSelf<Immutable = K>,
        V: IsRetainable,
    {
        let count = min(keys.len(), objects.len());

        let keys = keys_to_ptr(keys);
        let objects = util::ref_ptr_cast_const(objects.as_ptr());

        // SAFETY: See `NSDictionary::from_vec` and `NSArray::from_slice`.
        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), objects, keys, count) }
    }
}

// Note: We'd like to make getter methods take `K: Borrow<Q>` like
// `std::collections::HashMap`, so that e.g.
// `NSDictionary<NSString, ...>` could take a `&NSObject` as input,
// and still make that work since `NSString` borrows to `NSObject`.
//
// But we can't really, at least not with extra `unsafe` / an extra
// trait, since we don't control how the comparisons happen.
//
// The most useful alternative would probably be to take
// `impl AsRef<K>`, but objc2 classes deref to their superclass anyhow, so
// let's just use a simple normal reference.

extern_methods!(
    unsafe impl<K: Message + Eq + Hash, V: Message> NSDictionary<K, V> {
        #[doc(alias = "objectForKey:")]
        #[method(objectForKey:)]
        pub fn get(&self, key: &K) -> Option<&V>;

        #[doc(alias = "objectForKey:")]
        pub fn get_retained(&self, key: &K) -> Option<Retained<V>>
        where
            V: IsIdCloneable,
        {
            // SAFETY: The object is stored in the dictionary
            self.get(key)
                .map(|obj| unsafe { util::collection_retain(obj) })
        }

        /// Returns a mutable reference to the value corresponding to the key.
        ///
        /// # Examples
        ///
        #[cfg_attr(all(feature = "NSString", feature = "NSObject"), doc = "```")]
        #[cfg_attr(
            not(all(feature = "NSString", feature = "NSObject")),
            doc = "```ignore"
        )]
        /// use objc2_foundation::{ns_string, NSMutableDictionary, NSMutableString, NSString};
        ///
        /// let mut dict = NSMutableDictionary::new();
        /// dict.insert_id(ns_string!("one"), NSMutableString::new());
        /// println!("{:?}", dict.get_mut(ns_string!("one")));
        /// ```
        #[doc(alias = "objectForKey:")]
        #[method(objectForKey:)]
        pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
        where
            V: IsMutable;
    }
);

impl<K: Message, V: Message> NSDictionary<K, V> {
    pub fn len(&self) -> usize {
        self.count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "getObjects:andKeys:")]
    pub fn keys_vec(&self) -> Vec<&K> {
        let len = self.len();
        let mut keys = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(ptr::null_mut(), keys.as_mut_ptr());
            keys.set_len(len);
            mem::transmute::<Vec<NonNull<K>>, Vec<&K>>(keys)
        }
    }

    // We don't provide `keys_mut_vec`, since keys are immutable.

    #[doc(alias = "getObjects:andKeys:")]
    pub fn values_vec(&self) -> Vec<&V> {
        let len = self.len();
        let mut vals = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(vals.as_mut_ptr(), ptr::null_mut());
            vals.set_len(len);
            mem::transmute::<Vec<NonNull<V>>, Vec<&V>>(vals)
        }
    }

    /// Returns a vector of mutable references to the values in the dictionary.
    ///
    /// # Examples
    ///
    #[cfg_attr(all(feature = "NSString", feature = "NSObject"), doc = "```")]
    #[cfg_attr(
        not(all(feature = "NSString", feature = "NSObject")),
        doc = "```ignore"
    )]
    /// use objc2_foundation::{ns_string, NSMutableDictionary, NSMutableString, NSString};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert_id(ns_string!("one"), NSMutableString::from_str("two"));
    /// for val in dict.values_mut() {
    ///     println!("{:?}", val);
    /// }
    /// ```
    #[doc(alias = "getObjects:andKeys:")]
    pub fn values_vec_mut(&mut self) -> Vec<&mut V>
    where
        V: IsMutable,
    {
        let len = self.len();
        let mut vals = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(vals.as_mut_ptr(), ptr::null_mut());
            vals.set_len(len);
            mem::transmute::<Vec<NonNull<V>>, Vec<&mut V>>(vals)
        }
    }

    #[doc(alias = "getObjects:andKeys:")]
    pub fn to_vecs(&self) -> (Vec<&K>, Vec<&V>) {
        let len = self.len();
        let mut keys = Vec::with_capacity(len);
        let mut objs = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(objs.as_mut_ptr(), keys.as_mut_ptr());
            keys.set_len(len);
            objs.set_len(len);
            (
                mem::transmute::<Vec<NonNull<K>>, Vec<&K>>(keys),
                mem::transmute::<Vec<NonNull<V>>, Vec<&V>>(objs),
            )
        }
    }

    /// Returns an [`NSArray`] containing the dictionary's values.
    ///
    /// [`NSArray`]: crate::Foundation::NSArray
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSMutableDictionary, NSObject, NSString};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert_id(ns_string!("one"), NSObject::new());
    /// let array = dict.to_array();
    /// assert_eq!(array.len(), 1);
    /// ```
    #[cfg(feature = "NSArray")]
    pub fn to_array(&self) -> Retained<crate::Foundation::NSArray<V>>
    where
        V: IsIdCloneable,
    {
        // SAFETY: The elements are retainable behind `Retained<V>`, so getting
        // another reference to them (via. `NSArray`) is sound.
        unsafe { self.allValues() }
    }
}

impl<K: Message + Eq + Hash + HasStableHash, V: Message> NSMutableDictionary<K, V> {
    /// Inserts a key-value pair into the dictionary.
    ///
    /// If the dictionary did not have this key present, None is returned.
    /// If the dictionary did have this key present, the value is updated,
    /// and the old value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSMutableDictionary, NSObject, ns_string};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert_id(ns_string!("one"), NSObject::new());
    /// ```
    #[cfg(feature = "NSObject")]
    #[doc(alias = "setObject:forKey:")]
    pub fn insert_id(&mut self, key: &K, value: Retained<V>) -> Option<Retained<V>>
    where
        K: NSCopying + CounterpartOrSelf<Immutable = K>,
    {
        // SAFETY: We remove the object from the dictionary below
        let old_obj = self
            .get(key)
            .map(|old_obj| unsafe { util::mutable_collection_retain_removed(old_obj) });

        let key = ProtocolObject::from_ref(key);
        // SAFETY: We have ownership over the value.
        unsafe { self.setObject_forKey(&value, key) };
        old_obj
    }

    /// Inserts a key-value pair into the dictionary.
    ///
    /// If the dictionary did not have this key present, None is returned.
    /// If the dictionary did have this key present, the value is updated,
    /// and the old value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSCopying, NSMutableDictionary};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert_id(ns_string!("key"), ns_string!("value").copy());
    /// ```
    #[cfg(feature = "NSObject")]
    #[doc(alias = "setObject:forKey:")]
    pub fn insert(&mut self, key: &K, value: &V) -> Option<Retained<V>>
    where
        K: NSCopying + CounterpartOrSelf<Immutable = K>,
        V: IsRetainable,
    {
        // SAFETY: We remove the object from the dictionary below
        let old_obj = self
            .get(key)
            .map(|old_obj| unsafe { util::mutable_collection_retain_removed(old_obj) });

        let key = ProtocolObject::from_ref(key);
        // SAFETY: The value is `IsRetainable`, and hence safe for the
        // collection to retain.
        unsafe { self.setObject_forKey(value, key) };
        old_obj
    }

    /// Removes a key from the dictionary, returning the value at the key
    /// if the key was previously in the dictionary.
    ///
    /// # Examples
    ///
    #[cfg_attr(all(feature = "NSString", feature = "NSObject"), doc = "```")]
    #[cfg_attr(
        not(all(feature = "NSString", feature = "NSObject")),
        doc = "```ignore"
    )]
    /// use objc2_foundation::{ns_string, NSMutableDictionary, NSObject};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert_id(ns_string!("one"), NSObject::new());
    /// dict.remove(ns_string!("one"));
    /// assert!(dict.is_empty());
    /// ```
    #[doc(alias = "removeObjectForKey:")]
    pub fn remove(&mut self, key: &K) -> Option<Retained<V>>
    where
        K: CounterpartOrSelf<Immutable = K>,
    {
        // SAFETY: We remove the object from the dictionary below
        let old_obj = self
            .get(key)
            .map(|old_obj| unsafe { util::mutable_collection_retain_removed(old_obj) });
        self.removeObjectForKey(key);
        old_obj
    }
}

impl<K: Message, V: Message> NSDictionary<K, V> {
    #[doc(alias = "keyEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(iter::Iter::new(self))
    }

    #[doc(alias = "keyEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    pub fn keys_retained(&self) -> KeysRetained<'_, K, V>
    where
        K: IsIdCloneable,
    {
        KeysRetained(iter::IterRetained::new(self))
    }

    // TODO: Is this ever useful?
    // pub fn into_keys(this: Retained<Self>) -> IntoKeys<K, V> {
    //     todo!()
    // }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    pub fn values(&self) -> Values<'_, K, V> {
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        Values(unsafe { iter::IterWithBackingEnum::new(self, enumerator) })
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V>
    where
        V: IsMutable,
    {
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        ValuesMut(unsafe { iter::IterMutWithBackingEnum::new(self, enumerator) })
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    pub fn values_retained(&self) -> ValuesRetained<'_, K, V>
    where
        V: IsIdCloneable,
    {
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        ValuesRetained(unsafe { iter::IterRetainedWithBackingEnum::new(self, enumerator) })
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    pub fn into_values(this: Retained<Self>) -> IntoValues<K, V>
    where
        V: IsIdCloneable,
    {
        let enumerator = unsafe { this.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        IntoValues(unsafe { iter::IntoIterWithBackingEnum::new_immutable(this, enumerator) })
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<K: Message, V: Message> iter::FastEnumerationHelper for NSDictionary<K, V> {
    // Fast enumeration for dictionaries returns the keys.
    type Item = K;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<K: Message, V: Message> iter::FastEnumerationHelper for NSMutableDictionary<K, V> {
    // The same goes for mutable dictionaries.
    type Item = K;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

// We also cfg-gate `Keys` behind `NSEnumerator` for symmetry, even though we
// don't necessarily need to.
#[cfg(feature = "NSEnumerator")]
mod iter_helpers {
    use super::*;
    use crate::Foundation::NSEnumerator;

    /// An iterator over the keys of a `NSDictionary`.
    #[derive(Debug)]
    pub struct Keys<'a, K: Message, V: Message>(pub(super) iter::Iter<'a, NSDictionary<K, V>>);

    __impl_iter! {
        impl<'a, K: Message, V: Message> Iterator<Item = &'a K> for Keys<'a, K, V> { ... }
    }

    /// An iterator that retains the keys of a `NSDictionary`.
    #[derive(Debug)]
    pub struct KeysRetained<'a, K: Message, V: Message>(
        pub(super) iter::IterRetained<'a, NSDictionary<K, V>>,
    );

    __impl_iter! {
        impl<'a, K: Message + IsIdCloneable, V: Message> Iterator<Item = Retained<K>> for KeysRetained<'a, K, V> { ... }
    }

    /// An iterator over the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct Values<'a, K: Message, V: Message>(
        pub(super) iter::IterWithBackingEnum<'a, NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<'a, K: Message, V: Message> Iterator<Item = &'a V> for Values<'a, K, V> { ... }
    }

    /// A mutable iterator over the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct ValuesMut<'a, K: Message, V: Message>(
        pub(super) iter::IterMutWithBackingEnum<'a, NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<'a, K: Message, V: Message + IsMutable> Iterator<Item = &'a mut V> for ValuesMut<'a, K, V> { ... }
    }

    /// A iterator that retains the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct ValuesRetained<'a, K: Message, V: Message>(
        pub(super) iter::IterRetainedWithBackingEnum<'a, NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<'a, K: Message, V: Message + IsIdCloneable> Iterator<Item = Retained<V>> for ValuesRetained<'a, K, V> { ... }
    }

    /// A consuming iterator over the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct IntoValues<K: Message, V: Message>(
        pub(super) iter::IntoIterWithBackingEnum<NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<K: Message, V: Message> Iterator<Item = Retained<V>> for IntoValues<K, V> { ... }
    }
}

#[cfg(feature = "NSEnumerator")]
pub use self::iter_helpers::*;

impl<'a, K: Message + Eq + Hash, V: Message> Index<&'a K> for NSDictionary<K, V> {
    type Output = V;

    fn index<'s>(&'s self, index: &'a K) -> &'s V {
        self.get(index).unwrap()
    }
}

impl<'a, K: Message + Eq + Hash, V: Message> Index<&'a K> for NSMutableDictionary<K, V> {
    type Output = V;

    fn index<'s>(&'s self, index: &'a K) -> &'s V {
        self.get(index).unwrap()
    }
}

impl<'a, K: Message + Eq + Hash, V: Message + IsMutable> IndexMut<&'a K> for NSDictionary<K, V> {
    fn index_mut<'s>(&'s mut self, index: &'a K) -> &'s mut V {
        self.get_mut(index).unwrap()
    }
}

impl<'a, K: Message + Eq + Hash, V: Message + IsMutable> IndexMut<&'a K>
    for NSMutableDictionary<K, V>
{
    fn index_mut<'s>(&'s mut self, index: &'a K) -> &'s mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug for NSDictionary<K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (keys, values) = self.to_vecs();
        let iter = keys.into_iter().zip(values);
        f.debug_map().entries(iter).finish()
    }
}

impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug for NSMutableDictionary<K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
