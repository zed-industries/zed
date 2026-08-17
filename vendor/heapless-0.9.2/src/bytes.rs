//! Implementations of `bytes` traits for `heapless` types.

use crate::{
    len_type::LenType,
    vec::{VecInner, VecStorage},
};
use bytes::{buf::UninitSlice, BufMut};

unsafe impl<S: VecStorage<u8> + ?Sized, LenT: LenType> BufMut for VecInner<u8, LenT, S> {
    #[inline]
    fn remaining_mut(&self) -> usize {
        self.capacity() - self.len()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let len = self.len();
        let pos = len + cnt;
        if pos >= self.capacity() {
            panic!("Advance out of range");
        }
        self.set_len(pos);
    }

    #[inline]
    fn chunk_mut(&mut self) -> &mut UninitSlice {
        let len = self.len();
        let ptr = self.as_mut_ptr();
        unsafe { &mut UninitSlice::from_raw_parts_mut(ptr, self.capacity())[len..] }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vec, VecView};
    use bytes::BufMut;

    #[test]
    #[should_panic]
    fn buf_mut_advance_mut_out_of_bounds() {
        let mut vec: Vec<u8, 8> = Vec::new();
        unsafe { vec.advance_mut(9) };
    }

    #[test]
    fn buf_mut_remaining_mut() {
        let mut vec: Vec<u8, 8> = Vec::new();
        assert_eq!(vec.remaining_mut(), 8);
        vec.push(42).unwrap();
        assert_eq!(vec.remaining_mut(), 7);
    }

    #[test]
    fn buf_mut_chunk_mut() {
        let mut vec: Vec<u8, 8> = Vec::new();
        assert_eq!(vec.chunk_mut().len(), 8);
        unsafe { vec.advance_mut(1) };
        assert_eq!(vec.chunk_mut().len(), 7);
    }

    #[test]
    #[should_panic]
    fn buf_mut_advance_mut_out_of_bounds_view() {
        let vec: &mut VecView<u8, u8> = &mut Vec::<u8, 8, u8>::new();
        unsafe { vec.advance_mut(9) };
    }

    #[test]
    fn buf_mut_remaining_mut_view() {
        let vec: &mut VecView<u8, u8> = &mut Vec::<u8, 8, u8>::new();
        assert_eq!(vec.remaining_mut(), 8);
        vec.push(42).unwrap();
        assert_eq!(vec.remaining_mut(), 7);
    }

    #[test]
    fn buf_mut_chunk_mut_view() {
        let vec: &mut VecView<u8, u8> = &mut Vec::<u8, 8, u8>::new();
        assert_eq!(vec.chunk_mut().len(), 8);
        unsafe { vec.advance_mut(1) };
        assert_eq!(vec.chunk_mut().len(), 7);
    }
}
