use std::ptr;
use std::slice;

use crate::CapacityError;

/// Implements basic arrayvec methods - based on a few required methods
/// for length and element access.
pub(crate) trait ArrayVecImpl {
    type Item;
    const CAPACITY: usize;

    fn len(&self) -> usize;

    unsafe fn set_len(&mut self, new_len: usize);

    /// Return a slice containing all elements of the vector.
    fn as_slice(&self) -> &[Self::Item] {
        let len = self.len();
        unsafe {
            slice::from_raw_parts(self.as_ptr(), len)
        }
    }

    /// Return a mutable slice containing all elements of the vector.
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        let len = self.len();
        unsafe {
            std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
        }
    }

    /// Return a raw pointer to the vector's buffer.
    fn as_ptr(&self) -> *const Self::Item;

    /// Return a raw mutable pointer to the vector's buffer.
    fn as_mut_ptr(&mut self) -> *mut Self::Item;

    #[track_caller]
    fn push(&mut self, element: Self::Item) {
        self.try_push(element).unwrap()
    }

    fn try_push(&mut self, element: Self::Item) -> Result<(), CapacityError<Self::Item>> {
        if self.len() < Self::CAPACITY {
            unsafe {
                self.push_unchecked(element);
            }
            Ok(())
        } else {
            Err(CapacityError::new(element))
        }
    }

    unsafe fn push_unchecked(&mut self, element: Self::Item) {
        let len = self.len();
        debug_assert!(len < Self::CAPACITY);
        ptr::write(self.as_mut_ptr().add(len), element);
        self.set_len(len + 1);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        unsafe {
            let new_len = self.len() - 1;
            self.set_len(new_len);
            Some(ptr::read(self.as_ptr().add(new_len)))
        }
    }

    fn clear(&mut self) {
        self.truncate(0)
    }

    fn truncate(&mut self, new_len: usize) {
        unsafe {
            let len = self.len();
            if new_len < len {
                self.set_len(new_len);
                let tail = slice::from_raw_parts_mut(self.as_mut_ptr().add(new_len), len - new_len);
                ptr::drop_in_place(tail);
            }
        }
    }
}

