//! Type dispatched analog of `enum{*AnyVecRaw, *AnyVec<Traits>}`.

use core::marker::PhantomData;
use core::ptr::NonNull;
use crate::any_value::Unknown;
use crate::any_vec_raw::AnyVecRaw;
use crate::AnyVec;
use crate::mem::{MemBuilder};
use crate::traits::Trait;

pub trait IAnyVecRawPtr: Copy{
    /// Known element type of AnyVec
    type Element: 'static/* = Unknown*/;
    type M: MemBuilder;
    unsafe fn any_vec_raw<'a>(&self) -> &'a AnyVecRaw<Self::M>;
    unsafe fn any_vec_raw_mut<'a>(&mut self) -> &'a mut AnyVecRaw<Self::M>;
}
pub trait IAnyVecPtr: IAnyVecRawPtr{
    type Traits: ?Sized + Trait;
    unsafe fn any_vec<'a>(&self) -> &'a AnyVec<Self::Traits, Self::M>;
    unsafe fn any_vec_mut<'a>(&mut self) -> &'a mut AnyVec<Self::Traits, Self::M>;
}


pub struct AnyVecRawPtr<Type: 'static, M: MemBuilder>{
    ptr: NonNull<AnyVecRaw<M>>,
    phantom: PhantomData<*mut Type>
}
impl<Type, M: MemBuilder> From<NonNull<AnyVecRaw<M>>> for AnyVecRawPtr<Type, M>{
    #[inline]
    fn from(ptr: NonNull<AnyVecRaw<M>>) -> Self {
        Self{ptr, phantom: PhantomData}
    }
}
impl<Type, M: MemBuilder> Copy for AnyVecRawPtr<Type, M> {}
impl<Type, M: MemBuilder> Clone for AnyVecRawPtr<Type, M> {
    #[inline]
    fn clone(&self) -> Self {
        Self{
            ptr: self.ptr,
            phantom: PhantomData
        }
    }
}

impl<Type, M: MemBuilder> IAnyVecRawPtr for AnyVecRawPtr<Type, M>{
    type Element = Type;
    type M = M;

    #[inline]
    unsafe fn any_vec_raw<'a>(&self) -> &'a AnyVecRaw<Self::M> {
        self.ptr.as_ref()
    }

    #[inline]
    unsafe fn any_vec_raw_mut<'a>(&mut self) -> &'a mut AnyVecRaw<Self::M> {
        self.ptr.as_mut()
    }
}


pub struct AnyVecPtr<Traits: ?Sized + Trait, M: MemBuilder>{
    ptr: NonNull<AnyVec<Traits, M>>
}
impl<Traits: ?Sized + Trait, M: MemBuilder> From<NonNull<AnyVec<Traits, M>>> for AnyVecPtr<Traits, M> {
    #[inline]
    fn from(ptr: NonNull<AnyVec<Traits, M>>) -> Self {
        Self{ptr}
    }
}
impl<Traits: ?Sized + Trait, M: MemBuilder> From<&mut AnyVec<Traits, M>> for AnyVecPtr<Traits, M> {
    #[inline]
    fn from(reference: &mut AnyVec<Traits, M>) -> Self {
        Self{ptr: NonNull::from(reference)}
    }
}
impl<Traits: ?Sized + Trait, M: MemBuilder> From<&AnyVec<Traits, M>> for AnyVecPtr<Traits, M> {
    #[inline]
    fn from(reference: &AnyVec<Traits, M>) -> Self {
        Self{ptr: NonNull::from(reference)}
    }
}
impl<Traits: ?Sized + Trait, M: MemBuilder> Clone for AnyVecPtr<Traits, M>{
    #[inline]
    fn clone(&self) -> Self {
        Self{ptr: self.ptr}
    }
}
impl<Traits: ?Sized + Trait, M: MemBuilder> Copy for AnyVecPtr<Traits, M>{}

impl<Traits: ?Sized + Trait, M: MemBuilder> IAnyVecRawPtr for AnyVecPtr<Traits, M> {
    type Element = Unknown;
    type M = M;

    #[inline]
    unsafe fn any_vec_raw<'a>(&self) -> &'a AnyVecRaw<Self::M> {
        &self.ptr.as_ref().raw
    }

    #[inline]
    unsafe fn any_vec_raw_mut<'a>(&mut self) -> &'a mut AnyVecRaw<Self::M> {
        &mut self.ptr.as_mut().raw
    }
}
impl<Traits: ?Sized + Trait, M: MemBuilder> IAnyVecPtr for AnyVecPtr<Traits, M> {
    type Traits = Traits;

    #[inline]
    unsafe fn any_vec<'a>(&self) -> &'a AnyVec<Traits, M> {
        self.ptr.as_ref()
    }

    #[inline]
    unsafe fn any_vec_mut<'a>(&mut self) -> &'a mut AnyVec<Traits, M> {
        self.ptr.as_mut()
    }
}


/// Type knowledge optimized operations.
///
/// All unsafe, because dereferencing pointer is unsafe.
pub(crate) mod utils{
    use core::{mem, ptr};
    use core::any::TypeId;
    use core::mem::size_of;
    use core::ptr::NonNull;
    use crate::any_value::Unknown;
    use crate::any_vec_ptr::IAnyVecRawPtr;
    use crate::AnyVecTyped;

    #[inline]
    pub unsafe fn element_typeid<AnyVecPtr: IAnyVecRawPtr>(any_vec_ptr: AnyVecPtr) -> TypeId
    {
        if Unknown::is::<AnyVecPtr::Element>(){
            let any_vec_raw = any_vec_ptr.any_vec_raw();
            any_vec_raw.type_id
        } else {
            TypeId::of::<AnyVecPtr::Element>()
        }
    }

    /// In bytes.
    #[inline]
    pub unsafe fn element_size<AnyVecPtr: IAnyVecRawPtr>(any_vec_ptr: AnyVecPtr) -> usize
    {
        if Unknown::is::<AnyVecPtr::Element>(){
            let any_vec_raw = any_vec_ptr.any_vec_raw();
            any_vec_raw.element_layout().size()
        } else {
            size_of::<AnyVecPtr::Element>()
        }
    }

    #[inline]
    pub unsafe fn element_ptr_at<AnyVecPtr: IAnyVecRawPtr>(any_vec_ptr: AnyVecPtr, index: usize)
        -> *const u8
    {
        let any_vec_raw = any_vec_ptr.any_vec_raw();
        if Unknown::is::<AnyVecPtr::Element>(){
            any_vec_raw.get_unchecked(index)
        } else {
            // AnyVecTyped::get_unchecked cause MIRI error
            AnyVecTyped::<AnyVecPtr::Element, _>::new(NonNull::from(any_vec_raw))
                .as_ptr().add(index)
                as *const _ as *const u8
        }
    }

    #[inline]
    pub unsafe fn element_mut_ptr_at<AnyVecPtr: IAnyVecRawPtr>(mut any_vec_ptr: AnyVecPtr, index: usize)
        -> *mut u8
    {
        let any_vec_raw = any_vec_ptr.any_vec_raw_mut();
        if Unknown::is::<AnyVecPtr::Element>(){
            any_vec_raw.get_unchecked_mut(index)
        } else {
            // AnyVecTyped::get_unchecked_mut cause MIRI error
            AnyVecTyped::<AnyVecPtr::Element, _>::new(NonNull::from(any_vec_raw))
                .as_mut_ptr().add(index)
                as *mut _ as *mut u8
        }
    }

    #[inline]
    pub unsafe fn move_elements_at<AnyVecPtr: IAnyVecRawPtr>
        (any_vec_ptr: AnyVecPtr, src_index: usize, dst_index: usize, len: usize)
    {
        let src = element_ptr_at(any_vec_ptr, src_index);
        let dst = element_mut_ptr_at(any_vec_ptr, dst_index);
        if Unknown::is::<AnyVecPtr::Element>(){
            let any_vec_raw = any_vec_ptr.any_vec_raw();
            ptr::copy(
                src,
                dst,
                any_vec_raw.element_layout().size() * len
            );
        } else {
            ptr::copy(
                src as * const AnyVecPtr::Element,
                dst as *mut AnyVecPtr::Element,
                len
            );
        }
    }

    #[inline]
    pub unsafe fn drop_elements_range<AnyVecPtr: IAnyVecRawPtr>
        (any_vec_ptr: AnyVecPtr, start_index: usize, end_index: usize)
    {
        debug_assert!(start_index <= end_index);

        if Unknown::is::<AnyVecPtr::Element>(){
            let any_vec_raw = any_vec_ptr.any_vec_raw();
            if let Some(drop_fn) = any_vec_raw.drop_fn{
                (drop_fn)(
                    element_mut_ptr_at(any_vec_ptr, start_index),
                    end_index - start_index
                );
            }
        } else if mem::needs_drop::<AnyVecPtr::Element>(){
            // drop as slice. This is marginally faster then one by one.
            let start_ptr = element_mut_ptr_at(any_vec_ptr, start_index) as *mut AnyVecPtr::Element;
            let to_drop = ptr::slice_from_raw_parts_mut(start_ptr, end_index - start_index);
            ptr::drop_in_place(to_drop);
        }
    }
}