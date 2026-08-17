use core::marker::PhantomData;
use crate::copy_nonoverlapping_value;
use crate::any_vec_ptr::IAnyVecRawPtr;
use crate::any_vec_ptr::utils::{element_mut_ptr_at, element_ptr_at};
use crate::any_vec_raw::AnyVecRaw;
use crate::ops::temp::Operation;

pub struct SwapRemove<'a, AnyVecPtr: IAnyVecRawPtr>{
    any_vec_ptr: AnyVecPtr,
    element: *mut u8,
    last_index: usize,
    phantom: PhantomData<&'a mut AnyVecRaw<AnyVecPtr::M>>,
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> SwapRemove<'a, AnyVecPtr>{
    #[inline]
    pub(crate) fn new(mut any_vec_ptr: AnyVecPtr, index: usize) -> Self{
        let element = unsafe{ element_mut_ptr_at(any_vec_ptr, index) };

        // 1. mem::forget and element drop panic "safety".
        let any_vec_raw = unsafe{ any_vec_ptr.any_vec_raw_mut() };
        let last_index = any_vec_raw.len - 1;
        any_vec_raw.len = index;

        Self{ any_vec_ptr, element, last_index, phantom: PhantomData }
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr> Operation for SwapRemove<'a, AnyVecPtr>{
    type AnyVecPtr = AnyVecPtr;

    #[inline]
    fn any_vec_ptr(&self) -> Self::AnyVecPtr {
        self.any_vec_ptr
    }

    #[inline]
    fn bytes(&self) -> *const u8 {
        self.element
    }

    #[inline]
    fn consume(&mut self) {
    unsafe{
        // 2. overwrite with last element
        let last_element = element_ptr_at(self.any_vec_ptr, self.last_index);
        let any_vec_raw = self.any_vec_ptr.any_vec_raw_mut();

        if self.element as *const u8 != last_element {
            copy_nonoverlapping_value::<AnyVecPtr::Element>(
                last_element,
                self.element,
                any_vec_raw.element_layout().size()
            );
        }

        // 3. shrink len `self.any_vec.len -= 1`
        any_vec_raw.len = self.last_index;
    }
    }
}
