use crate::any_vec_ptr::IAnyVecRawPtr;
use crate::iter::Iter;
use crate::any_vec_ptr;
use crate::ops::iter::Iterable;

pub struct Drain<'a, AnyVecPtr: IAnyVecRawPtr>
{
    iter: Iter<'a, AnyVecPtr>,
    start: usize,
    original_len: usize
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> Drain<'a, AnyVecPtr>
{
    #[inline]
    pub(crate) fn new(mut any_vec_ptr: AnyVecPtr, start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        let any_vec_raw = unsafe{ any_vec_ptr.any_vec_raw_mut() };
        let original_len = any_vec_raw.len;
        debug_assert!(end <= original_len);

        // mem::forget and element drop panic "safety".
        any_vec_raw.len = start;

        Self{
            iter: Iter::new(any_vec_ptr, start, end),
            start,
            original_len
        }
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> Iterable
for
    Drain<'a, AnyVecPtr>
{
    type Iter = Iter<'a, AnyVecPtr>;

    #[inline]
    fn iter(&self) -> &Self::Iter {
        &self.iter
    }

    #[inline]
    fn iter_mut(&mut self) -> &mut Self::Iter {
        &mut self.iter
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> Drop for Drain<'a, AnyVecPtr>
{
    fn drop(&mut self) {
        use any_vec_ptr::utils::*;

        // 1. drop the rest of the elements
        unsafe{
            drop_elements_range(
                self.iter.any_vec_ptr,
                self.iter.index,
                self.iter.end
            );
        }

        // 2. mem move
        unsafe{
            let elements_left = self.original_len - self.iter.end;
            move_elements_at(
                self.iter.any_vec_ptr,
                self.iter.end,
                self.start,
                elements_left
            );
        }

        // 3. len
        let distance = self.iter.end - self.start;
        let any_vec_raw = unsafe{ self.iter.any_vec_ptr.any_vec_raw_mut() };
        any_vec_raw.len = self.original_len - distance;
    }
}