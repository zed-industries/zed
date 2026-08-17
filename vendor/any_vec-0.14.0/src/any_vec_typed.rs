use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::{Range, RangeBounds};
use core::ptr::NonNull;
use core::{fmt, slice};
use crate::any_value::{AnyValueSizeless, AnyValueWrapper};
use crate::any_vec_raw::AnyVecRaw;
use crate::ops::{Iter, pop, remove, swap_remove, TempValue};
use crate::any_vec_ptr::AnyVecRawPtr;
use crate::into_range;
use crate::iter::ElementIterator;
use crate::mem::{MemBuilder, Mem, MemResizable};
use crate::ops::drain::Drain;
use crate::ops::splice::Splice;

/// Concrete type [`AnyVec`] representation.
///
/// Obtained by dereferencing [`AnyVecRef<T>`] or [`AnyVecMut<T>`].
///
/// Operations with concrete type are somewhat faster, due to
/// the fact, that compiler are able to optimize harder with full
/// type knowledge.
///
/// [`AnyVec`]: crate::AnyVec
/// [`AnyVec::downcast_`]: crate::AnyVec::downcast_ref
/// [`AnyVecRef<T>`]: crate::AnyVecRef
/// [`AnyVecMut<T>`]: crate::AnyVecMut
pub struct AnyVecTyped<'a, T: 'static, M: MemBuilder + 'a>{
    // NonNull - to have one struct for both & and &mut
    any_vec: NonNull<AnyVecRaw<M>>,
    phantom: PhantomData<&'a mut T>
}

unsafe impl<'a, T: 'static + Send, M: MemBuilder + Send> Send for AnyVecTyped<'a, T, M>
    where M::Mem: Send
{}
unsafe impl<'a, T: 'static + Sync, M: MemBuilder + Sync> Sync for AnyVecTyped<'a, T, M>
    where M::Mem: Sync
{}

impl<'a, T: 'static, M: MemBuilder + 'a> AnyVecTyped<'a, T, M>{
    /// # Safety
    ///
    /// Unsafe, because type not checked
    #[inline]
    pub(crate) unsafe fn new(any_vec: NonNull<AnyVecRaw<M>>) -> Self {
        Self{any_vec, phantom: PhantomData}
    }

    /// AnyVecTyped should not be Clone, because it can be both & and &mut.
    #[inline]
    pub(crate) fn clone(&self) -> Self {
        Self{any_vec: self.any_vec, phantom: PhantomData}
    }

    #[inline]
    fn this(&self) -> &'a AnyVecRaw<M> {
        unsafe{ self.any_vec.as_ref() }
    }

    #[inline]
    fn this_mut(&mut self) -> &'a mut AnyVecRaw<M> {
        unsafe{ self.any_vec.as_mut() }
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize)
        where M::Mem: MemResizable
    {
        self.this_mut().reserve(additional)
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize)
        where M::Mem: MemResizable
    {
        self.this_mut().reserve_exact(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self)
        where M::Mem: MemResizable
    {
        self.this_mut().shrink_to_fit()
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize)
        where M::Mem: MemResizable
    {
        self.this_mut().shrink_to(min_capacity)
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.this_mut().set_len(new_len);
    }

    #[inline]
    pub fn insert(&mut self, index: usize, value: T){
        unsafe{
            self.this_mut().insert_unchecked(index, AnyValueWrapper::new(value));
        }
    }

    #[inline]
    pub fn push(&mut self, value: T){
        unsafe{
            self.this_mut().push_unchecked(AnyValueWrapper::new(value));
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty(){
            None
        } else {
            let value = unsafe{
                TempValue::new(pop::Pop::new(
                    AnyVecRawPtr::<T, M>::from(self.any_vec)
                )).downcast_unchecked::<T>()
            };
            Some(value)
        }
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        self.this().index_check(index);
        unsafe{
            TempValue::new(remove::Remove::new(
                AnyVecRawPtr::<T, M>::from(self.any_vec),
                index
            )).downcast_unchecked::<T>()
        }
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.this().index_check(index);
        unsafe{
            TempValue::new(swap_remove::SwapRemove::new(
                AnyVecRawPtr::<T, M>::from(self.any_vec),
                index
            )).downcast_unchecked::<T>()
        }
    }

    #[inline]
    pub fn drain(&mut self, range: impl RangeBounds<usize>)
        -> impl ElementIterator<Item = T> + 'a
    {
        let Range{start, end} = into_range(self.len(), range);
        Iter(Drain::new(
            AnyVecRawPtr::<T, M>::from(self.any_vec),
            start,
            end
        )).map(|e| unsafe{
            e.downcast_unchecked::<T>()
        })
    }

    #[inline]
    pub fn splice<I>(&mut self, range: impl RangeBounds<usize>, replace_with: I)
        -> impl ElementIterator<Item = T> + 'a
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator + 'a,
    {
        let Range{start, end} = into_range(self.len(), range);
        let replace_with = replace_with.into_iter()
            .map(|e| AnyValueWrapper::new(e));

        Iter(Splice::new(
            AnyVecRawPtr::<T, M>::from(self.any_vec),
            start,
            end,
            replace_with
        )).map(|e| unsafe{
            e.downcast_unchecked::<T>()
        })
    }

    #[inline]
    pub fn clear(&mut self){
        self.this_mut().clear();
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<'a, T> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'a, T> {
        self.as_mut_slice().iter_mut()
    }

    #[inline]
    pub fn at(&self, index: usize) -> &'a T{
        self.get(index).unwrap()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.as_slice().get(index)
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> &'a T {
        self.as_slice().get_unchecked(index)
    }

    #[inline]
    pub fn at_mut(&mut self, index: usize) -> &'a mut T{
        self.get_mut(index).unwrap()
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&'a mut T>{
        self.as_mut_slice().get_mut(index)
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &'a mut T {
        self.as_mut_slice().get_unchecked_mut(index)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.this().mem.as_ptr().cast::<T>()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.this_mut().mem.as_mut_ptr().cast::<T>()
    }

    #[inline]
    pub fn as_slice(&self) -> &'a [T] {
        unsafe{
            slice::from_raw_parts(
                self.as_ptr(),
                self.len(),
            )
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &'a mut[T] {
        unsafe{
            slice::from_raw_parts_mut(
                self.as_mut_ptr(),
                self.len(),
            )
        }
    }

    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &'a mut[MaybeUninit<T>] {
        unsafe {
            slice::from_raw_parts_mut(
                self.as_mut_ptr().add(self.len()) as *mut MaybeUninit<T>,
                self.capacity() - self.len(),
            )
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.this().len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.this().capacity()
    }
}

impl<'a, T: 'static + Debug, M: MemBuilder> Debug for AnyVecTyped<'a, T, M>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (*self.as_slice()).fmt(f)
    }
}

// Do not implement Index, since we can't do the same for AnyVec
/*
impl<'a, T: 'static, I: SliceIndex<[T]>> Index<I> for AnyVecTyped<'a, T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &'a Self::Output {
        self.as_slice().index(index)
    }
}

impl<'a, T: 'static, I: SliceIndex<[T]>> IndexMut<I> for AnyVecTyped<'a, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &'a mut Self::Output {
        self.as_mut_slice().index_mut(index)
    }
}
 */