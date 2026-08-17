use core::marker::PhantomData;
use crate::any_vec_ptr::IAnyVecRawPtr;
use crate::any_vec_ptr::utils::element_ptr_at;
use crate::any_vec_raw::AnyVecRaw;
use crate::ops::temp::Operation;

pub struct Pop<'a, AnyVecPtr: IAnyVecRawPtr>{
    any_vec_ptr: AnyVecPtr,
    phantom: PhantomData<&'a mut AnyVecRaw<AnyVecPtr::M>>,
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> Pop<'a, AnyVecPtr>{
    #[inline]
    pub(crate) fn new(mut any_vec_ptr: AnyVecPtr) -> Self{
        let any_vec_raw = unsafe{ any_vec_ptr.any_vec_raw_mut() };

        // mem::forget and element drop panic "safety".
        debug_assert!(any_vec_raw.len > 0);
        any_vec_raw.len -= 1;

        Self{
            any_vec_ptr,
            phantom: PhantomData
        }
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> Operation for Pop<'a, AnyVecPtr>{
    type AnyVecPtr = AnyVecPtr;

    #[inline]
    fn any_vec_ptr(&self) -> Self::AnyVecPtr {
        self.any_vec_ptr
    }

    #[inline]
    fn bytes(&self) -> *const u8 {
        unsafe{
            let any_vec_raw =  self.any_vec_ptr.any_vec_raw();
            let index = any_vec_raw.len;
            element_ptr_at(self.any_vec_ptr, index)
        }
    }

    #[inline]
    fn consume(&mut self) {
        // do nothing.
    }
}