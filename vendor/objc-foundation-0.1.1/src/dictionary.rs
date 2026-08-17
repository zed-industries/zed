use std::cmp::min;
use std::marker::PhantomData;
use std::ops::Index;
use std::ptr;

use objc::runtime::Class;
use objc_id::{Id, Owned, Ownership, ShareId};

use {
    INSArray, INSFastEnumeration, INSCopying, INSObject,
    NSArray, NSSharedArray, NSEnumerator,
};

unsafe fn from_refs<D, T>(keys: &[&T], vals: &[&D::Value]) -> Id<D>
        where D: INSDictionary, T: INSCopying<Output=D::Key> {
    let cls = D::class();
    let count = min(keys.len(), vals.len());
    let obj: *mut D = msg_send![cls, alloc];
    let obj: *mut D = msg_send![obj, initWithObjects:vals.as_ptr()
                                             forKeys:keys.as_ptr()
                                               count:count];
    Id::from_retained_ptr(obj)
}

pub trait INSDictionary : INSObject {
    type Key: INSObject;
    type Value: INSObject;
    type Own: Ownership;

    fn count(&self) -> usize {
        unsafe {
            msg_send![self, count]
        }
    }

    fn object_for(&self, key: &Self::Key) -> Option<&Self::Value> {
        unsafe {
            let obj: *mut Self::Value = msg_send![self, objectForKey:key];
            if obj.is_null() { None } else { Some(&*obj) }
        }
    }

    fn keys(&self) -> Vec<&Self::Key> {
        let len = self.count();
        let mut keys = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![self, getObjects:ptr::null_mut::<Self::Value>()
                                           andKeys:keys.as_mut_ptr()];
            keys.set_len(len);
        }
        keys
    }

    fn values(&self) -> Vec<&Self::Value> {
        let len = self.count();
        let mut vals = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![self, getObjects:vals.as_mut_ptr()
                                           andKeys:ptr::null_mut::<Self::Key>()];
            vals.set_len(len);
        }
        vals
    }

    fn keys_and_objects(&self) -> (Vec<&Self::Key>, Vec<&Self::Value>) {
        let len = self.count();
        let mut keys = Vec::with_capacity(len);
        let mut objs = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![self, getObjects:objs.as_mut_ptr()
                                           andKeys:keys.as_mut_ptr()];
            keys.set_len(len);
            objs.set_len(len);
        }
        (keys, objs)
    }

    fn key_enumerator(&self) -> NSEnumerator<Self::Key> {
        unsafe {
            let result = msg_send![self, keyEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    fn object_enumerator(&self) -> NSEnumerator<Self::Value> {
        unsafe {
            let result = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    fn keys_array(&self) -> Id<NSSharedArray<Self::Key>> {
        unsafe {
            let keys: *mut NSSharedArray<Self::Key> = msg_send![self, allKeys];
            Id::from_ptr(keys)
        }
    }

    fn from_keys_and_objects<T>(keys: &[&T],
            vals: Vec<Id<Self::Value, Self::Own>>) -> Id<Self>
            where T: INSCopying<Output=Self::Key> {
        let vals_refs: Vec<&Self::Value> = vals.iter().map(|obj| &**obj).collect();
        unsafe {
            from_refs(keys, &vals_refs)
        }
    }

    fn into_values_array(dict: Id<Self>) -> Id<NSArray<Self::Value, Self::Own>> {
        unsafe {
            let vals = msg_send![dict, allValues];
            Id::from_ptr(vals)
        }
    }
}

pub struct NSDictionary<K, V> {
    key: PhantomData<ShareId<K>>,
    obj: PhantomData<Id<V>>,
}

object_impl!(NSDictionary<K, V>);

impl<K, V> INSObject for NSDictionary<K, V> where K: INSObject, V: INSObject {
    fn class() -> &'static Class {
        Class::get("NSDictionary").unwrap()
    }
}

impl<K, V> INSDictionary for NSDictionary<K, V>
        where K: INSObject, V: INSObject {
    type Key = K;
    type Value = V;
    type Own = Owned;
}

impl<K, V> INSFastEnumeration for NSDictionary<K, V>
        where K: INSObject, V: INSObject {
    type Item = K;
}

impl<'a, K, V> Index<&'a K> for NSDictionary<K, V> where K: INSObject, V: INSObject {
    type Output = V;

    fn index(&self, index: &K) -> &V {
        self.object_for(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use objc_id::Id;
    use {INSArray, INSObject, INSString, NSObject, NSString};
    use super::{INSDictionary, NSDictionary};

    fn sample_dict(key: &str) -> Id<NSDictionary<NSString, NSObject>> {
        let string = NSString::from_str(key);
        let obj = NSObject::new();
        NSDictionary::from_keys_and_objects(&[&*string], vec![obj])
    }

    #[test]
    fn test_count() {
        let dict = sample_dict("abcd");
        assert!(dict.count() == 1);
    }

    #[test]
    fn test_object_for() {
        let dict = sample_dict("abcd");

        let string = NSString::from_str("abcd");
        assert!(dict.object_for(&string).is_some());

        let string = NSString::from_str("abcde");
        assert!(dict.object_for(&string).is_none());
    }

    #[test]
    fn test_keys() {
        let dict = sample_dict("abcd");
        let keys = dict.keys();

        assert!(keys.len() == 1);
        assert!(keys[0].as_str() == "abcd");
    }

    #[test]
    fn test_values() {
        let dict = sample_dict("abcd");
        let vals = dict.values();

        assert!(vals.len() == 1);
    }

    #[test]
    fn test_keys_and_objects() {
        let dict = sample_dict("abcd");
        let (keys, objs) = dict.keys_and_objects();

        assert!(keys.len() == 1);
        assert!(objs.len() == 1);
        assert!(keys[0].as_str() == "abcd");
        assert!(objs[0] == dict.object_for(keys[0]).unwrap());
    }

    #[test]
    fn test_key_enumerator() {
        let dict = sample_dict("abcd");
        assert!(dict.key_enumerator().count() == 1);
        assert!(dict.key_enumerator().next().unwrap().as_str() == "abcd");
    }

    #[test]
    fn test_object_enumerator() {
        let dict = sample_dict("abcd");
        assert!(dict.object_enumerator().count() == 1);
    }

    #[test]
    fn test_arrays() {
        let dict = sample_dict("abcd");

        let keys = dict.keys_array();
        assert!(keys.count() == 1);
        assert!(keys.object_at(0).as_str() == "abcd");

        let objs = INSDictionary::into_values_array(dict);
        assert!(objs.count() == 1);
    }
}
