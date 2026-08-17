use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_ulong;
use std::ptr;
use std::slice;

use objc::runtime::Object;
use objc_id::Id;

use INSObject;

pub struct NSEnumerator<'a, T> where T: INSObject {
    id: Id<Object>,
    item: PhantomData<&'a T>,
}

impl<'a, T> NSEnumerator<'a, T> where T: INSObject {
    pub unsafe fn from_ptr(ptr: *mut Object) -> NSEnumerator<'a, T> {
        NSEnumerator { id: Id::from_ptr(ptr), item: PhantomData }
    }
}

impl<'a, T> Iterator for NSEnumerator<'a, T> where T: INSObject {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            let obj: *mut T = msg_send![self.id, nextObject];
            if obj.is_null() { None } else { Some(&*obj) }
        }
    }
}

pub trait INSFastEnumeration: INSObject {
    type Item: INSObject;

    fn enumerator(&self) -> NSFastEnumerator<Self> {
        NSFastEnumerator::new(self)
    }
}

#[repr(C)]
struct NSFastEnumerationState<T> {
    state: c_ulong,
    items_ptr: *const *const T,
    mutations_ptr: *mut c_ulong,
    extra: [c_ulong; 5],
}

fn enumerate<'a, 'b: 'a, C: INSFastEnumeration>(object: &'b C,
        state: &mut NSFastEnumerationState<C::Item>,
        buf: &'a mut [*const C::Item]) -> Option<&'a [*const C::Item]> {
    let count: usize = unsafe {
        // Reborrow state so that we don't move it
        let state = &mut *state;
        msg_send![object, countByEnumeratingWithState:state
                                              objects:buf.as_mut_ptr()
                                                count:buf.len()]
    };

    if count > 0 {
        unsafe { Some(slice::from_raw_parts(state.items_ptr, count)) }
    } else {
        None
    }
}

const FAST_ENUM_BUF_SIZE: usize = 16;

pub struct NSFastEnumerator<'a, C: 'a + INSFastEnumeration> {
    object: &'a C,

    ptr: *const *const C::Item,
    end: *const *const C::Item,

    state: NSFastEnumerationState<C::Item>,
    buf: [*const C::Item; FAST_ENUM_BUF_SIZE],
}

impl<'a, C: INSFastEnumeration> NSFastEnumerator<'a, C> {
    fn new(object: &C) -> NSFastEnumerator<C> {
        NSFastEnumerator {
            object: object,

            ptr: ptr::null(),
            end: ptr::null(),

            state: unsafe { mem::zeroed() },
            buf: [ptr::null(); FAST_ENUM_BUF_SIZE],
        }
    }

    fn update_buf(&mut self) -> bool {
        // If this isn't our first time enumerating, record the previous value
        // from the mutations pointer.
        let mutations = if !self.ptr.is_null() {
            Some(unsafe { *self.state.mutations_ptr })
        } else {
            None
        };

        let next_buf = enumerate(self.object, &mut self.state, &mut self.buf);
        if let Some(buf) = next_buf {
            // Check if the collection was mutated
            if let Some(mutations) = mutations {
                assert!(mutations == unsafe { *self.state.mutations_ptr },
                    "Mutation detected during enumeration of object {:p}",
                    self.object);
            }

            self.ptr = buf.as_ptr();
            self.end = unsafe { self.ptr.offset(buf.len() as isize) };
            true
        } else {
            self.ptr = ptr::null();
            self.end = ptr::null();
            false
        }
    }
}

impl<'a, C: INSFastEnumeration> Iterator for NSFastEnumerator<'a, C> {
    type Item = &'a C::Item;

    fn next(&mut self) -> Option<&'a C::Item> {
        if self.ptr == self.end && !self.update_buf() {
            None
        } else {
            unsafe {
                let obj = *self.ptr;
                self.ptr = self.ptr.offset(1);
                Some(&*obj)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use {INSArray, INSValue, NSArray, NSValue};
    use super::INSFastEnumeration;

    #[test]
    fn test_enumerator() {
        let vec = (0u32..4).map(NSValue::from_value).collect();
        let array = NSArray::from_vec(vec);

        let enumerator = array.object_enumerator();
        assert!(enumerator.count() == 4);

        let enumerator = array.object_enumerator();
        assert!(enumerator.enumerate().all(|(i, obj)| obj.value() == i as u32));
    }

    #[test]
    fn test_fast_enumerator() {
        let vec = (0u32..4).map(NSValue::from_value).collect();
        let array = NSArray::from_vec(vec);

        let enumerator = array.enumerator();
        assert!(enumerator.count() == 4);

        let enumerator = array.enumerator();
        assert!(enumerator.enumerate().all(|(i, obj)| obj.value() == i as u32));
    }
}
