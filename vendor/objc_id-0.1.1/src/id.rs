use std::any::Any;
use std::fmt;
use std::hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use objc::Message;
use objc::rc::{StrongPtr, WeakPtr};
use objc::runtime::Object;

/// A type used to mark that a struct owns the object(s) it contains,
/// so it has the sole references to them.
pub enum Owned { }
/// A type used to mark that the object(s) a struct contains are shared,
/// so there may be other references to them.
pub enum Shared { }

/// A type that marks what type of ownership a struct has over the object(s)
/// it contains; specifically, either `Owned` or `Shared`.
pub trait Ownership: Any { }
impl Ownership for Owned { }
impl Ownership for Shared { }

/// A pointer type for Objective-C's reference counted objects.
///
/// The object of an `Id` is retained and sent a `release` message when
/// the `Id` is dropped.
///
/// An `Id` may be either `Owned` or `Shared`, represented by the types `Id`
/// and `ShareId`, respectively. If owned, there are no other references to the
/// object and the `Id` can be mutably dereferenced. `ShareId`, however, can
/// only be immutably dereferenced because there may be other references to the
/// object, but a `ShareId` can be cloned to provide more references to the
/// object. An owned `Id` can be "downgraded" freely to a `ShareId`, but there
/// is no way to safely upgrade back.
pub struct Id<T, O = Owned> {
    ptr: StrongPtr,
    item: PhantomData<T>,
    own: PhantomData<O>,
}

impl<T, O> Id<T, O> where T: Message, O: Ownership {
    unsafe fn new(ptr: StrongPtr) -> Id<T, O> {
        Id { ptr: ptr, item: PhantomData, own: PhantomData }
    }

    /// Constructs an `Id` from a pointer to an unretained object and
    /// retains it. Panics if the pointer is null.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn from_ptr(ptr: *mut T) -> Id<T, O> {
        assert!(!ptr.is_null(), "Attempted to construct an Id from a null pointer");
        Id::new(StrongPtr::retain(ptr as *mut Object))
    }

    /// Constructs an `Id` from a pointer to a retained object; this won't
    /// retain the pointer, so the caller must ensure the object has a +1
    /// retain count. Panics if the pointer is null.
    /// Unsafe because the pointer must be to a valid object and
    /// the caller must ensure the ownership is correct.
    pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T, O> {
        assert!(!ptr.is_null(), "Attempted to construct an Id from a null pointer");
        Id::new(StrongPtr::new(ptr as *mut Object))
    }
}

impl<T> Id<T, Owned> where T: Message {
    /// "Downgrade" an owned `Id` to a `ShareId`, allowing it to be cloned.
    pub fn share(self) -> ShareId<T> {
        let Id { ptr, .. } = self;
        unsafe { Id::new(ptr) }
    }
}

impl<T> Clone for Id<T, Shared> where T: Message {
    fn clone(&self) -> ShareId<T> {
        unsafe {
            Id::new(self.ptr.clone())
        }
    }
}

unsafe impl<T, O> Sync for Id<T, O> where T: Sync { }

unsafe impl<T> Send for Id<T, Owned> where T: Send { }

unsafe impl<T> Send for Id<T, Shared> where T: Sync { }

impl<T, O> Deref for Id<T, O> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(*self.ptr as *mut T) }
    }
}

impl<T> DerefMut for Id<T, Owned> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *(*self.ptr as *mut T) }
    }
}

impl<T, O> PartialEq for Id<T, O> where T: PartialEq {
    fn eq(&self, other: &Id<T, O>) -> bool {
        self.deref() == other.deref()
    }

    fn ne(&self, other: &Id<T, O>) -> bool {
        self.deref() != other.deref()
    }
}

impl<T, O> Eq for Id<T, O> where T: Eq { }

impl<T, O> hash::Hash for Id<T, O> where T: hash::Hash {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.deref().hash(state)
    }
}

impl<T, O> fmt::Debug for Id<T, O> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T, O> fmt::Pointer for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

/// A convenient alias for a shared `Id`.
pub type ShareId<T> = Id<T, Shared>;

/// A pointer type for a weak reference to an Objective-C reference counted
/// object.
pub struct WeakId<T> {
    ptr: WeakPtr,
    item: PhantomData<T>,
}

impl<T> WeakId<T> where T: Message {
    /// Construct a new `WeakId` referencing the given `ShareId`.
    pub fn new(obj: &ShareId<T>) -> WeakId<T> {
        WeakId {
            ptr: obj.ptr.weak(),
            item: PhantomData,
        }
    }

    /// Load a `ShareId` from the `WeakId` if the object still exists.
    /// Returns `None` if the object has been deallocated.
    pub fn load(&self) -> Option<ShareId<T>> {
        let obj = self.ptr.load();
        if obj.is_null() {
            None
        } else {
            Some(unsafe { Id::new(obj) })
        }
    }
}

unsafe impl<T> Sync for WeakId<T> where T: Sync { }

unsafe impl<T> Send for WeakId<T> where T: Sync { }

#[cfg(test)]
mod tests {
    use objc::runtime::Object;
    use super::{Id, ShareId, WeakId};

    fn retain_count(obj: &Object) -> usize {
        unsafe { msg_send![obj, retainCount] }
    }

    #[test]
    fn test_clone() {
        let cls = class!(NSObject);
        let obj: Id<Object> = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        };
        assert!(retain_count(&obj) == 1);

        let obj = obj.share();
        assert!(retain_count(&obj) == 1);

        let cloned = obj.clone();
        assert!(retain_count(&cloned) == 2);
        assert!(retain_count(&obj) == 2);

        drop(obj);
        assert!(retain_count(&cloned) == 1);
    }

    #[test]
    fn test_weak() {
        let cls = class!(NSObject);
        let obj: ShareId<Object> = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        };

        let weak = WeakId::new(&obj);
        let strong = weak.load().unwrap();
        let strong_ptr: *const Object = &*strong;
        let obj_ptr: *const Object = &*obj;
        assert!(strong_ptr == obj_ptr);
        drop(strong);

        drop(obj);
        assert!(weak.load().is_none());
    }
}
