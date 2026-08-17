#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::{borrow::Borrow, ffi::c_void, hash::Hash};

use crate::{
    kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionary, CFIndex,
    CFMutableDictionary, CFRetained, Type,
};

/// Roughly same as `failed_creating_array`.
#[cold]
fn failed_creating_dictionary(len: CFIndex) -> ! {
    #[cfg(feature = "alloc")]
    {
        use alloc::alloc::{handle_alloc_error, Layout};
        use core::mem::align_of;

        let layout =
            Layout::array::<(*const (), *const ())>(len as usize).unwrap_or_else(|_| unsafe {
                Layout::from_size_align_unchecked(0, align_of::<*const ()>())
            });

        handle_alloc_error(layout)
    }
    #[cfg(not(feature = "alloc"))]
    {
        panic!("failed allocating CFDictionary holding {len} elements")
    }
}

/// These usually doesn't _have_ to be bound by `K: Type`, all that matters is
/// that they're valid for the dictionary at hand.
///
/// But let's keep the bound for now, it might turn out to be necessary.
#[inline]
fn to_void<K: ?Sized + Type>(key: &K) -> *const c_void {
    let key: *const K = key;
    key.cast()
}

/// Convenience creation methods.
impl<K: ?Sized, V: ?Sized> CFDictionary<K, V> {
    /// Create a new empty dictionary.
    #[inline]
    #[doc(alias = "CFDictionaryCreate")]
    pub fn empty() -> CFRetained<Self>
    where
        K: Type + PartialEq + Hash,
        V: Type,
    {
        Self::from_slices(&[], &[])
    }

    /// Create a new dictionary from slices of keys and values.
    ///
    /// # Panics
    ///
    /// Panics if the slices have different lengths.
    #[inline]
    #[doc(alias = "CFDictionaryCreate")]
    pub fn from_slices(keys: &[&K], values: &[&V]) -> CFRetained<Self>
    where
        K: Type + PartialEq + Hash,
        V: Type,
    {
        assert_eq!(
            keys.len(),
            values.len(),
            "key and object slices must have the same length",
        );
        // Can never happen, allocations in Rust cannot be this large.
        debug_assert!(keys.len() < CFIndex::MAX as usize);
        let len = keys.len() as CFIndex;

        // `&T` has the same layout as `*const c_void`, and is non-NULL.
        let keys = keys.as_ptr().cast::<*const c_void>().cast_mut();
        let values = values.as_ptr().cast::<*const c_void>().cast_mut();

        // SAFETY: The keys and values are CFTypes (`K: Type` and `V: Type`
        // bounds), and the dictionary callbacks are thus correct.
        //
        // The keys and values are retained internally by the dictionary, so
        // we do not need to keep them alive ourselves after this.
        let dictionary = unsafe {
            CFDictionary::new(
                None,
                keys,
                values,
                len,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            )
        }
        .unwrap_or_else(|| failed_creating_dictionary(len));

        // SAFETY: The dictionary contains no keys and values yet, and thus
        // it's safe to cast them to `K` and `V` (as the dictionary callbacks
        // are valid for these types).
        unsafe { CFRetained::cast_unchecked::<Self>(dictionary) }
    }
}

/// Convenience creation methods.
impl<K: ?Sized, V: ?Sized> CFMutableDictionary<K, V> {
    /// Create a new empty mutable dictionary.
    #[inline]
    #[doc(alias = "CFDictionaryCreateMutable")]
    pub fn empty() -> CFRetained<Self>
    where
        K: Type + PartialEq + Hash,
        V: Type,
    {
        Self::with_capacity(0)
    }

    /// Create a new mutable dictionary with the given capacity.
    #[inline]
    #[doc(alias = "CFDictionaryCreateMutable")]
    pub fn with_capacity(capacity: usize) -> CFRetained<Self>
    where
        K: Type + PartialEq + Hash,
        V: Type,
    {
        let capacity = capacity.try_into().expect("capacity too high");

        // SAFETY: The keys and values are CFTypes (`K: Type` and `V: Type`
        // bounds), and the dictionary callbacks are thus correct.
        let dictionary = unsafe {
            CFMutableDictionary::new(
                None,
                capacity,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            )
        }
        .unwrap_or_else(|| failed_creating_dictionary(capacity));

        // SAFETY: The dictionary contains no keys and values yet, and thus
        // it's safe to cast them to `K` and `V` (as the dictionary callbacks
        // are valid for these types).
        unsafe { CFRetained::cast_unchecked::<Self>(dictionary) }
    }
}

/// Direct, unsafe object accessors.
///
/// CFDictionary stores its keys and values directly, and you can get
/// references to those without having to retain them first - but only if the
/// dictionary isn't mutated while doing so - otherwise, you might end up
/// accessing a deallocated object.
impl<K: ?Sized, V: ?Sized> CFDictionary<K, V> {
    /// Get a direct reference to one of the dictionary's values.
    ///
    /// Consider using the [`get`](Self::get) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The dictionary must not be mutated while the returned reference is
    /// live.
    #[inline]
    #[doc(alias = "CFDictionaryGetValue")]
    pub unsafe fn get_unchecked(&self, key: &K) -> Option<&V>
    where
        K: Type + Sized,
        V: Type + Sized,
    {
        // SAFETY: The key is valid for this dictionary.
        //
        // The values are CoreFoundation types, and thus cannot be NULL, so
        // no need to use `CFDictionaryGetValueIfPresent`.
        let value = unsafe { self.as_opaque().value(to_void(key)) };
        // SAFETY: The dictionary's values are of type `V`.
        //
        // Caller ensures that the dictionary isn't mutated for the lifetime
        // of the reference.
        unsafe { value.cast::<V>().as_ref() }
    }

    /// A vector containing direct references to the dictionary's keys and
    /// values.
    ///
    /// Consider using the [`to_vecs`](Self::to_vecs) method instead, unless
    /// you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the returned references are alive.
    #[cfg(feature = "alloc")]
    pub unsafe fn to_vecs_unchecked(&self) -> (Vec<&K>, Vec<&V>)
    where
        K: Type,
        V: Type,
    {
        let len = self.len();
        let mut keys = Vec::<&K>::with_capacity(len);
        let mut values = Vec::<&V>::with_capacity(len);

        // `&K`/`&V` has the same layout as `*const c_void`.
        let keys_ptr = keys.as_mut_ptr().cast::<*const c_void>();
        let values_ptr = values.as_mut_ptr().cast::<*const c_void>();

        // SAFETY: The arrays are both of the right size.
        unsafe { self.as_opaque().keys_and_values(keys_ptr, values_ptr) };

        // SAFETY: Just initialized the `Vec`s above.
        unsafe {
            keys.set_len(len);
            values.set_len(len);
        }

        (keys, values)
    }
}

/// Various accessor methods.
impl<K: ?Sized, V: ?Sized> CFDictionary<K, V> {
    /// The amount of elements in the dictionary.
    #[inline]
    #[doc(alias = "CFDictionaryGetCount")]
    pub fn len(&self) -> usize {
        // Fine to cast here, the count is never negative.
        self.as_opaque().count() as _
    }

    /// Whether the dictionary is empty or not.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieve the object at the given index.
    ///
    /// Returns `None` if the index was out of bounds.
    #[doc(alias = "CFDictionaryGetValue")]
    pub fn get(&self, key: &K) -> Option<CFRetained<V>>
    where
        K: Type + Sized + PartialEq + Hash,
        V: Type + Sized,
    {
        // SAFETY: We retain the value right away, so we know the reference is
        // valid for the duration we use it.
        unsafe { self.get_unchecked(key) }.map(V::retain)
    }

    /// Two vectors containing respectively the dictionary's keys and values.
    #[cfg(feature = "alloc")]
    #[doc(alias = "CFDictionaryGetKeysAndValues")]
    pub fn to_vecs(&self) -> (Vec<CFRetained<K>>, Vec<CFRetained<V>>)
    where
        K: Type + Sized,
        V: Type + Sized,
    {
        // SAFETY: We retain the elements below, so that we know that the
        // dictionary isn't mutated while they are alive.
        let (keys, objects) = unsafe { self.to_vecs_unchecked() };
        (
            keys.into_iter().map(K::retain).collect(),
            objects.into_iter().map(V::retain).collect(),
        )
    }

    /// Whether the key is in the dictionary.
    #[inline]
    #[doc(alias = "CFDictionaryContainsKey")]
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Type + Sized + PartialEq + Hash,
    {
        // SAFETY: The key is bound by `K: Type`, and thus know to be valid
        // for the callbacks in the dictionary.
        unsafe { self.as_opaque().contains_ptr_key(to_void(key)) }
    }

    /// Whether the value can be found anywhere in the dictionary.
    #[inline]
    #[doc(alias = "CFDictionaryContainsValue")]
    pub fn contains_value(&self, value: &V) -> bool
    where
        V: Type + Sized + PartialEq,
    {
        // SAFETY: The value is bound by `V: Type`, and thus know to be valid
        // for the callbacks in the dictionary.
        unsafe { self.as_opaque().contains_ptr_value(to_void(value)) }
    }
}

/// Various mutation methods.
impl<K: ?Sized, V: ?Sized> CFMutableDictionary<K, V> {
    /// Add the key-value pair to the dictionary if no such key already exist.
    #[inline]
    #[doc(alias = "CFDictionaryAddValue")]
    pub fn add(&self, key: &K, value: &V)
    where
        K: Type + Sized + PartialEq + Hash,
        V: Type + Sized,
    {
        unsafe {
            CFMutableDictionary::add_value(Some(self.as_opaque()), to_void(key), to_void(value))
        }
    }

    /// Set the value of the key in the dictionary.
    #[inline]
    #[doc(alias = "CFDictionarySetValue")]
    pub fn set(&self, key: &K, value: &V)
    where
        K: Type + Sized + PartialEq + Hash,
        V: Type + Sized,
    {
        unsafe {
            CFMutableDictionary::set_value(Some(self.as_opaque()), to_void(key), to_void(value))
        }
    }

    /// Replace the value of the key in the dictionary.
    #[inline]
    #[doc(alias = "CFDictionaryReplaceValue")]
    pub fn replace(&self, key: &K, value: &V)
    where
        K: Type + Sized + PartialEq + Hash,
        V: Type + Sized,
    {
        unsafe {
            CFMutableDictionary::replace_value(Some(self.as_opaque()), to_void(key), to_void(value))
        }
    }

    /// Remove the value from the dictionary associated with the key.
    #[inline]
    #[doc(alias = "CFDictionaryRemoveValue")]
    pub fn remove(&self, key: &K)
    where
        K: Type + Sized + PartialEq + Hash,
    {
        unsafe { CFMutableDictionary::remove_value(Some(self.as_opaque()), to_void(key)) }
    }

    /// Remove all keys and values from the dictionary.
    #[inline]
    #[doc(alias = "CFDictionaryRemoveAllValues")]
    pub fn clear(&self) {
        CFMutableDictionary::remove_all_values(Some(self.as_opaque()))
    }
}

// Allow easy conversion from `&CFDictionary<T>` to `&CFDictionary`.
// Requires `Type` bound because of reflexive impl in `cf_type!`.
impl<K: ?Sized + Type, V: ?Sized + Type> AsRef<CFDictionary> for CFDictionary<K, V> {
    fn as_ref(&self) -> &CFDictionary {
        self.as_opaque()
    }
}
impl<K: ?Sized + Type, V: ?Sized + Type> AsRef<CFMutableDictionary> for CFMutableDictionary<K, V> {
    fn as_ref(&self) -> &CFMutableDictionary {
        self.as_opaque()
    }
}

// `Eq`, `Ord` and `Hash` have the same semantics.
impl<K: ?Sized + Type, V: ?Sized + Type> Borrow<CFDictionary> for CFDictionary<K, V> {
    fn borrow(&self) -> &CFDictionary {
        self.as_opaque()
    }
}
impl<K: ?Sized + Type, V: ?Sized + Type> Borrow<CFMutableDictionary> for CFMutableDictionary<K, V> {
    fn borrow(&self) -> &CFMutableDictionary {
        self.as_opaque()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CFType;

    #[test]
    fn empty() {
        let dict1 = CFDictionary::<CFType, CFType>::empty();
        let dict2 = CFDictionary::<CFType, CFType>::empty();
        assert_eq!(dict1, dict2);
        assert_eq!(dict1.len(), 0);
        assert_eq!(dict1.get(dict1.as_ref()), None);
        assert!(!dict1.contains_key(dict1.as_ref()));
        assert!(!dict1.contains_value(dict1.as_ref()));
        #[cfg(feature = "alloc")]
        assert_eq!(dict1.to_vecs(), (alloc::vec![], alloc::vec![]));
    }

    #[test]
    #[cfg(feature = "CFString")]
    fn mutable_dictionary() {
        use crate::CFString;

        let dict = CFMutableDictionary::<CFString, CFString>::empty();
        dict.add(&CFString::from_str("a"), &CFString::from_str("b"));
        dict.add(&CFString::from_str("c"), &CFString::from_str("d"));
        assert_eq!(dict.len(), 2);

        dict.add(&CFString::from_str("c"), &CFString::from_str("e"));
        assert_eq!(dict.len(), 2);
        assert_eq!(
            dict.get(&CFString::from_str("c")),
            Some(CFString::from_str("d")),
        );

        dict.replace(&CFString::from_str("c"), &CFString::from_str("f"));
        assert_eq!(dict.len(), 2);
        assert_eq!(
            dict.get(&CFString::from_str("c")),
            Some(CFString::from_str("f"))
        );

        dict.remove(&CFString::from_str("a"));
        assert_eq!(dict.len(), 1);

        dict.clear();
        assert_eq!(dict.len(), 0);
    }

    #[test]
    #[cfg(feature = "CFString")]
    fn contains() {
        use crate::CFString;

        let dict = CFDictionary::from_slices(
            &[&*CFString::from_str("key")],
            &[&*CFString::from_str("value")],
        );

        assert!(dict.contains_key(&CFString::from_str("key")));
        assert!(dict.get(&CFString::from_str("key")).is_some());

        assert!(!dict.contains_key(&CFString::from_str("invalid key")));
        assert!(dict.get(&CFString::from_str("invalid key")).is_none());

        assert!(dict.contains_value(&CFString::from_str("value")));
        assert!(!dict.contains_value(&CFString::from_str("invalid value")));
    }

    #[test]
    #[cfg(all(feature = "CFString", feature = "CFNumber"))]
    fn heterogenous() {
        use crate::{CFBoolean, CFNumber, CFString, CFType};

        let dict = CFDictionary::<CFType, CFType>::from_slices(
            &[
                CFString::from_str("string key").as_ref(),
                CFNumber::new_isize(4).as_ref(),
                CFBoolean::new(true).as_ref(),
            ],
            &[
                CFString::from_str("a string value").as_ref(),
                CFNumber::new_isize(2).as_ref(),
                CFBoolean::new(false).as_ref(),
            ],
        );

        assert_eq!(
            dict.get(&CFString::from_str("string key")),
            Some(CFString::from_str("a string value").into())
        );
        assert_eq!(
            dict.get(&CFNumber::new_isize(4)),
            Some(CFNumber::new_isize(2).into())
        );
        assert_eq!(
            dict.get(CFBoolean::new(true)),
            Some(CFBoolean::new(false).into())
        );
        assert_eq!(dict.get(CFBoolean::new(false)), None);
    }

    #[test]
    #[cfg(feature = "CFString")]
    #[should_panic = "key and object slices must have the same length"]
    fn from_slices_not_same_length() {
        use crate::CFString;
        let _dict =
            CFDictionary::<CFString, CFString>::from_slices(&[&*CFString::from_str("key")], &[]);
    }
}
