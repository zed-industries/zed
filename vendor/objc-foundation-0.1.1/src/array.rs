use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::{Index, Range};

use objc::runtime::{Class, Object};
use objc_id::{Id, Owned, Ownership, Shared, ShareId};

use {INSCopying, INSFastEnumeration, INSMutableCopying, INSObject, NSEnumerator};

#[repr(isize)]
#[derive(Clone, Copy)]
pub enum NSComparisonResult {
    Ascending  = -1,
    Same       = 0,
    Descending = 1,
}

impl NSComparisonResult {
    pub fn from_ordering(order: Ordering) -> NSComparisonResult {
        match order {
            Ordering::Less => NSComparisonResult::Ascending,
            Ordering::Equal => NSComparisonResult::Same,
            Ordering::Greater => NSComparisonResult::Descending,
        }
    }

    pub fn as_ordering(&self) -> Ordering {
        match *self {
            NSComparisonResult::Ascending => Ordering::Less,
            NSComparisonResult::Same => Ordering::Equal,
            NSComparisonResult::Descending => Ordering::Greater,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NSRange {
    pub location: usize,
    pub length: usize,
}

impl NSRange {
    pub fn from_range(range: Range<usize>) -> NSRange {
        assert!(range.end >= range.start);
        NSRange { location: range.start, length: range.end - range.start }
    }

    pub fn as_range(&self) -> Range<usize> {
        Range { start: self.location, end: self.location + self.length }
    }
}

unsafe fn from_refs<A>(refs: &[&A::Item]) -> Id<A> where A: INSArray {
    let cls = A::class();
    let obj: *mut A = msg_send![cls, alloc];
    let obj: *mut A = msg_send![obj, initWithObjects:refs.as_ptr()
                                               count:refs.len()];
    Id::from_retained_ptr(obj)
}

pub trait INSArray : INSObject {
    type Item: INSObject;
    type Own: Ownership;

    fn count(&self) -> usize {
        unsafe {
            msg_send![self, count]
        }
    }

    fn object_at(&self, index: usize) -> &Self::Item {
        unsafe {
            let obj: *const Self::Item = msg_send![self, objectAtIndex:index];
            &*obj
        }
    }

    fn first_object(&self) -> Option<&Self::Item> {
        unsafe {
            let obj: *const Self::Item = msg_send![self, firstObject];
            if obj.is_null() { None } else { Some(&*obj) }
        }
    }

    fn last_object(&self) -> Option<&Self::Item> {
        unsafe {
            let obj: *const Self::Item = msg_send![self, lastObject];
            if obj.is_null() { None } else { Some(&*obj) }
        }
    }

    fn object_enumerator(&self) -> NSEnumerator<Self::Item> {
        unsafe {
            let result: *mut Object = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    fn from_vec(vec: Vec<Id<Self::Item, Self::Own>>) -> Id<Self> {
        let refs: Vec<&Self::Item> = vec.iter().map(|obj| &**obj).collect();
        unsafe {
            from_refs(&refs)
        }
    }

    fn objects_in_range(&self, range: Range<usize>) -> Vec<&Self::Item> {
        let range = NSRange::from_range(range);
        let mut vec = Vec::with_capacity(range.length);
        unsafe {
            let _: () = msg_send![self, getObjects:vec.as_ptr() range:range];
            vec.set_len(range.length);
        }
        vec
    }

    fn to_vec(&self) -> Vec<&Self::Item> {
        self.objects_in_range(0..self.count())
    }

    fn into_vec(array: Id<Self>) -> Vec<Id<Self::Item, Self::Own>> {
        array.to_vec().into_iter().map(|obj| unsafe {
            let obj_ptr: *const Self::Item = obj;
            Id::from_ptr(obj_ptr as *mut Self::Item)
        }).collect()
    }

    fn mut_object_at(&mut self, index: usize) -> &mut Self::Item
            where Self: INSArray<Own=Owned> {
        unsafe {
            let result: *mut Self::Item = msg_send![self, objectAtIndex:index];
            &mut *result
        }
    }

    fn shared_object_at(&self, index: usize) -> ShareId<Self::Item>
            where Self: INSArray<Own=Shared> {
        let obj = self.object_at(index);
        unsafe {
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        }
    }

    fn from_slice(slice: &[ShareId<Self::Item>]) -> Id<Self>
            where Self: INSArray<Own=Shared> {
        let refs: Vec<&Self::Item> = slice.iter().map(|obj| &**obj).collect();
        unsafe {
            from_refs(&refs)
        }
    }

    fn to_shared_vec(&self) -> Vec<ShareId<Self::Item>>
            where Self: INSArray<Own=Shared> {
        self.to_vec().into_iter().map(|obj| unsafe {
            let obj_ptr: *const Self::Item = obj;
            Id::from_ptr(obj_ptr as *mut Self::Item)
        }).collect()
    }
}

pub struct NSArray<T, O = Owned> {
    item: PhantomData<Id<T, O>>,
}

object_impl!(NSArray<T, O>);

impl<T, O> INSObject for NSArray<T, O> where T: INSObject, O: Ownership {
    fn class() -> &'static Class {
        Class::get("NSArray").unwrap()
    }
}

impl<T, O> INSArray for NSArray<T, O> where T: INSObject, O: Ownership {
    type Item = T;
    type Own = O;
}

impl<T> INSCopying for NSArray<T, Shared> where T: INSObject {
    type Output = NSSharedArray<T>;
}

impl<T> INSMutableCopying for NSArray<T, Shared> where T: INSObject {
    type Output = NSMutableSharedArray<T>;
}

impl<T, O> INSFastEnumeration for NSArray<T, O>
        where T: INSObject, O: Ownership {
    type Item = T;
}

impl<T, O> Index<usize> for NSArray<T, O> where T: INSObject, O: Ownership {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.object_at(index)
    }
}

pub type NSSharedArray<T> = NSArray<T, Shared>;

pub trait INSMutableArray : INSArray {
    fn add_object(&mut self, obj: Id<Self::Item, Self::Own>) {
        unsafe {
            let _: () = msg_send![self, addObject:&*obj];
        }
    }

    fn insert_object_at(&mut self, index: usize, obj: Id<Self::Item, Self::Own>) {
        unsafe {
            let _: () = msg_send![self, insertObject:&*obj atIndex:index];
        }
    }

    fn replace_object_at(&mut self, index: usize, obj: Id<Self::Item, Self::Own>) ->
            Id<Self::Item, Self::Own> {
        let old_obj = unsafe {
            let obj = self.object_at(index);
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        };
        unsafe {
            let _: () = msg_send![self, replaceObjectAtIndex:index
                                                  withObject:&*obj];
        }
        old_obj
    }

    fn remove_object_at(&mut self, index: usize) -> Id<Self::Item, Self::Own> {
        let obj = unsafe {
            let obj = self.object_at(index);
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        };
        unsafe {
            let _: () = msg_send![self, removeObjectAtIndex:index];
        }
        obj
    }

    fn remove_last_object(&mut self) -> Id<Self::Item, Self::Own> {
        let obj = self.last_object().map(|obj| unsafe {
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        });
        unsafe {
            let _: () = msg_send![self, removeLastObject];
        }
        // removeLastObject would have failed if the array is empty,
        // so we know this won't be None
        obj.unwrap()
    }

    fn remove_all_objects(&mut self) {
        unsafe {
            let _: () = msg_send![self, removeAllObjects];
        }
    }

    fn sort_by<F>(&mut self, compare: F)
            where F: FnMut(&Self::Item, &Self::Item) -> Ordering {
        extern fn compare_with_closure<T, F>(obj1: &T, obj2: &T,
                compare: &mut F) -> NSComparisonResult
                where F: FnMut(&T, &T) -> Ordering {
            NSComparisonResult::from_ordering((*compare)(obj1, obj2))
        }

        let f: extern fn(&Self::Item, &Self::Item, &mut F) -> NSComparisonResult =
            compare_with_closure;
        let mut closure = compare;
        unsafe {
            let _: () = msg_send![self, sortUsingFunction:f
                                                  context:&mut closure];
        }
    }
}

pub struct NSMutableArray<T, O = Owned> {
    item: PhantomData<Id<T, O>>,
}

object_impl!(NSMutableArray<T, O>);

impl<T, O> INSObject for NSMutableArray<T, O> where T: INSObject, O: Ownership {
    fn class() -> &'static Class {
        Class::get("NSMutableArray").unwrap()
    }
}

impl<T, O> INSArray for NSMutableArray<T, O> where T: INSObject, O: Ownership {
    type Item = T;
    type Own = O;
}

impl<T, O> INSMutableArray for NSMutableArray<T, O>
        where T: INSObject, O: Ownership { }

impl<T> INSCopying for NSMutableArray<T, Shared> where T: INSObject {
    type Output = NSSharedArray<T>;
}

impl<T> INSMutableCopying for NSMutableArray<T, Shared> where T: INSObject {
    type Output = NSMutableSharedArray<T>;
}

impl<T, O> INSFastEnumeration for NSMutableArray<T, O>
        where T: INSObject, O: Ownership {
    type Item = T;
}

impl<T, O> Index<usize> for NSMutableArray<T, O>
        where T: INSObject, O: Ownership {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.object_at(index)
    }
}

pub type NSMutableSharedArray<T> = NSMutableArray<T, Shared>;

#[cfg(test)]
mod tests {
    use objc_id::Id;
    use {INSObject, INSString, NSObject, NSString};
    use super::{INSArray, INSMutableArray, NSArray, NSMutableArray};

    fn sample_array(len: usize) -> Id<NSArray<NSObject>> {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(NSObject::new());
        }
        NSArray::from_vec(vec)
    }

    #[test]
    fn test_count() {
        let empty_array = NSArray::<NSObject>::new();
        assert!(empty_array.count() == 0);

        let array = sample_array(4);
        assert!(array.count() == 4);
    }

    #[test]
    fn test_object_at() {
        let array = sample_array(4);
        assert!(array.object_at(0) != array.object_at(3));
        assert!(array.first_object().unwrap() == array.object_at(0));
        assert!(array.last_object().unwrap() == array.object_at(3));

        let empty_array: Id<NSArray<NSObject>> = INSObject::new();
        assert!(empty_array.first_object().is_none());
        assert!(empty_array.last_object().is_none());
    }

    #[test]
    fn test_object_enumerator() {
        let array = sample_array(4);

        assert!(array.object_enumerator().count() == 4);
        assert!(array.object_enumerator()
                     .enumerate()
                     .all(|(i, obj)| obj == array.object_at(i)));
    }

    #[test]
    fn test_objects_in_range() {
        let array = sample_array(4);

        let middle_objs = array.objects_in_range(1..3);
        assert!(middle_objs.len() == 2);
        assert!(middle_objs[0] == array.object_at(1));
        assert!(middle_objs[1] == array.object_at(2));

        let empty_objs = array.objects_in_range(1..1);
        assert!(empty_objs.len() == 0);

        let all_objs = array.objects_in_range(0..4);
        assert!(all_objs.len() == 4);
    }

    #[test]
    fn test_into_vec() {
        let array = sample_array(4);

        let vec = INSArray::into_vec(array);
        assert!(vec.len() == 4);
    }

    #[test]
    fn test_add_object() {
        let mut array = NSMutableArray::new();
        let obj = NSObject::new();
        array.add_object(obj);

        assert!(array.count() == 1);
        assert!(array.object_at(0) == array.object_at(0));

        let obj = NSObject::new();
        array.insert_object_at(0, obj);
        assert!(array.count() == 2);
    }

    #[test]
    fn test_replace_object() {
        let mut array = NSMutableArray::new();
        let obj = NSObject::new();
        array.add_object(obj);

        let obj = NSObject::new();
        let old_obj = array.replace_object_at(0, obj);
        assert!(&*old_obj != array.object_at(0));
    }

    #[test]
    fn test_remove_object() {
        let mut array = NSMutableArray::new();
        for _ in 0..4 {
            array.add_object(NSObject::new());
        }

        array.remove_object_at(1);
        assert!(array.count() == 3);

        array.remove_last_object();
        assert!(array.count() == 2);

        array.remove_all_objects();
        assert!(array.count() == 0);
    }

    #[test]
    fn test_sort() {
        let strings = vec![
            NSString::from_str("hello"),
            NSString::from_str("hi"),
        ];
        let mut strings = NSMutableArray::from_vec(strings);

        strings.sort_by(|s1, s2| s1.as_str().len().cmp(&s2.as_str().len()));
        assert!(strings[0].as_str() == "hi");
        assert!(strings[1].as_str() == "hello");
    }
}
