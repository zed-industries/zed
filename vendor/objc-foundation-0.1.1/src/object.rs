use std::any::Any;

use objc::Message;
use objc::runtime::{BOOL, Class, NO};
use objc_id::{Id, ShareId};

use NSString;

/*
 The Sized bound is unfortunate; ideally, objc objects would not be
 treated as Sized. However, rust won't allow casting a dynamically-sized type
 pointer to an Object pointer, because dynamically-sized types can have fat
 pointers (two words) instead of real pointers.
 */
pub trait INSObject : Any + Sized + Message {
    fn class() -> &'static Class;

    fn hash_code(&self) -> usize {
        unsafe {
            msg_send![self, hash]
        }
    }

    fn is_equal<T>(&self, other: &T) -> bool where T: INSObject {
        let result: BOOL = unsafe {
            msg_send![self, isEqual:other]
        };
        result != NO
    }

    fn description(&self) -> ShareId<NSString> {
        unsafe {
            let result: *mut NSString = msg_send![self, description];
            Id::from_ptr(result)
        }
    }

    fn is_kind_of(&self, cls: &Class) -> bool {
        let result: BOOL = unsafe {
            msg_send![self, isKindOfClass:cls]
        };
        result != NO
    }

    fn new() -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        }
    }
}

object_struct!(NSObject);

#[cfg(test)]
mod tests {
    use {INSString, NSString};
    use super::{INSObject, NSObject};

    #[test]
    fn test_is_equal() {
        let obj1 = NSObject::new();
        assert!(obj1.is_equal(&*obj1));

        let obj2 = NSObject::new();
        assert!(!obj1.is_equal(&*obj2));
    }

    #[test]
    fn test_hash_code() {
        let obj = NSObject::new();
        assert!(obj.hash_code() == obj.hash_code());
    }

    #[test]
    fn test_description() {
        let obj = NSObject::new();
        let description = obj.description();
        let expected = format!("<NSObject: {:p}>", &*obj);
        assert!(description.as_str() == &*expected);
    }

    #[test]
    fn test_is_kind_of() {
        let obj = NSObject::new();
        assert!(obj.is_kind_of(NSObject::class()));
        assert!(!obj.is_kind_of(NSString::class()));
    }
}
