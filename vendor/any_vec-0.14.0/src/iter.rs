use core::iter::{FusedIterator};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use crate::any_vec_ptr::{AnyVecPtr, AnyVecRawPtr, IAnyVecRawPtr};
use crate::any_vec_ptr::utils::element_ptr_at;
use crate::any_vec_raw::AnyVecRaw;
use crate::{AnyVec, AnyVecTyped};
use crate::element::{ElementPointer, ElementMut, ElementRef};
use crate::mem::MemBuilder;
use crate::traits::Trait;

// TODO :Additional [`AnyVec`] Iterator operations.
/*pub trait AnyVecIterator: Iterator{
    fn lazy_cloned(self) -> impl
}*/

pub trait ElementIterator:
    DoubleEndedIterator + ExactSizeIterator + FusedIterator
{}

impl<T> ElementIterator for T
where
    T: DoubleEndedIterator + ExactSizeIterator + FusedIterator
{}

/// [`AnyVec`] iterator.
///
/// Return [`Element`], [`ElementRef`] or [`ElementMut`] items, depending on `IterItem`.
/// Cloneable for `Ref` and `Mut` versions.
///
/// [`AnyVec`]: crate::AnyVec
/// [`Element`]: crate::element::Element
/// [`ElementRef`]: crate::element::ElementRef
/// [`ElementMut`]: crate::element::ElementMut
pub struct Iter<'a,
    AnyVecPtr: IAnyVecRawPtr,
    IterItem: IteratorItem<'a, AnyVecPtr> = ElementIterItem<'a, AnyVecPtr>>
{
    pub(crate) any_vec_ptr: AnyVecPtr,

    pub(crate) index: usize,
    pub(crate) end: usize,

    phantom: PhantomData<(&'a AnyVecRaw<AnyVecPtr::M>, IterItem)>
}

impl<'a, AnyVecPtr: IAnyVecRawPtr, IterItem: IteratorItem<'a, AnyVecPtr>>
    Iter<'a, AnyVecPtr, IterItem>
{
    #[inline]
    pub(crate) fn new(any_vec_ptr: AnyVecPtr, start: usize, end: usize) -> Self {
        Self{any_vec_ptr, index: start, end, phantom: PhantomData}
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr, IterItem: IteratorItem<'a, AnyVecPtr> + Clone>
    Clone
for
    Iter<'a, AnyVecPtr, IterItem>
{
    #[inline]
    fn clone(&self) -> Self {
        Self{
            any_vec_ptr: self.any_vec_ptr,
            index: self.index,
            end: self.end,
            phantom: PhantomData
        }
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr, IterItem: IteratorItem<'a, AnyVecPtr>> Iterator
    for Iter<'a, AnyVecPtr, IterItem>
{
    type Item = IterItem::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end{
            None
        } else {
            let element = ElementPointer::new(
                self.any_vec_ptr,
                unsafe{NonNull::new_unchecked(
                    element_ptr_at(self.any_vec_ptr, self.index) as *mut u8
                )}
            );

            self.index += 1;
            Some(IterItem::element_to_item(element))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.end - self.index;
        (size, Some(size))
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr, IterItem: IteratorItem<'a, AnyVecPtr>> DoubleEndedIterator
    for Iter<'a, AnyVecPtr, IterItem>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == self.index{
            None
        } else {
            self.end -= 1;
            let element = ElementPointer::new(
                self.any_vec_ptr,
                unsafe{NonNull::new_unchecked(
                    element_ptr_at(self.any_vec_ptr, self.end) as *mut u8
                )}
            );

            Some(IterItem::element_to_item(element))
        }
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr, IterItem: IteratorItem<'a, AnyVecPtr>> ExactSizeIterator
    for Iter<'a, AnyVecPtr, IterItem>
{
    #[inline]
    fn len(&self) -> usize {
        self.end - self.index
    }
}

impl<'a, AnyVecPtr: IAnyVecRawPtr, IterItem: IteratorItem<'a, AnyVecPtr>> FusedIterator
    for Iter<'a, AnyVecPtr, IterItem>
{}


#[allow(renamed_and_removed_lints, suspicious_auto_trait_impls)]
unsafe impl<'a, Traits, M, IterItem> Send
for
    Iter<'a, AnyVecPtr<Traits, M>, IterItem>
where
    Traits: ?Sized + Trait, 
    M: MemBuilder,
    IterItem: IteratorItem<'a, AnyVecPtr<Traits, M>>,
    AnyVec<Traits, M>: Send
{}
#[allow(renamed_and_removed_lints, suspicious_auto_trait_impls)]
unsafe impl<'a, T, M, IterItem> Send
for
    Iter<'a, AnyVecRawPtr<T, M>, IterItem>
where
    M: MemBuilder, 
    IterItem: IteratorItem<'a, AnyVecRawPtr<T, M>>,
    AnyVecTyped<'a, T, M>: Send
{}

unsafe impl<'a, Traits, M, IterItem> Sync
for
    Iter<'a, AnyVecPtr<Traits, M>, IterItem>
where
    Traits: ?Sized + Trait, 
    M: MemBuilder, 
    IterItem: IteratorItem<'a, AnyVecPtr<Traits, M>>,    
    AnyVec<Traits, M>: Sync
{}
unsafe impl<'a, T, M, IterItem> Sync
for
    Iter<'a, AnyVecRawPtr<T, M>, IterItem>
where
    T: Sync,
    M: MemBuilder, 
    IterItem: IteratorItem<'a, AnyVecRawPtr<T, M>>,
    AnyVecTyped<'a, T, M>: Sync
{}


pub trait IteratorItem<'a, AnyVecPtr: IAnyVecRawPtr>{
    type Item;
    fn element_to_item(element: ElementPointer<'a, AnyVecPtr>) -> Self::Item;
}

/// Default
pub struct ElementIterItem<'a, AnyVecPtr: IAnyVecRawPtr>(
    pub(crate) PhantomData<ElementPointer<'a, AnyVecPtr>>
);
impl<'a, AnyVecPtr: IAnyVecRawPtr> IteratorItem<'a, AnyVecPtr> for ElementIterItem<'a, AnyVecPtr>{
    type Item = ElementPointer<'a, AnyVecPtr>;

    #[inline]
    fn element_to_item(element: ElementPointer<'a, AnyVecPtr>) -> Self::Item {
        element
    }
}

/// Ref
pub struct ElementRefIterItem<'a, Traits: ?Sized + Trait, M: MemBuilder>(
    pub(crate) PhantomData<ElementPointer<'a, AnyVecPtr<Traits, M>>>
);
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> IteratorItem<'a, AnyVecPtr<Traits, M>> for ElementRefIterItem<'a, Traits, M>{
    type Item = ElementRef<'a, Traits, M>;

    #[inline]
    fn element_to_item(element: ElementPointer<'a, AnyVecPtr<Traits, M>>) -> Self::Item {
        ElementRef(ManuallyDrop::new(element))
    }
}
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> Clone for ElementRefIterItem<'a, Traits, M>{
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}


/// Mut
pub struct ElementMutIterItem<'a, Traits: ?Sized + Trait, M: MemBuilder>(
    pub(crate) PhantomData<ElementPointer<'a, AnyVecPtr<Traits, M>>>
);
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> IteratorItem<'a, AnyVecPtr<Traits, M>> for ElementMutIterItem<'a, Traits, M>{
    type Item = ElementMut<'a, Traits, M>;

    #[inline]
    fn element_to_item(element: ElementPointer<'a, AnyVecPtr<Traits, M>>) -> Self::Item {
        ElementMut(ManuallyDrop::new(element))
    }
}
impl<'a, Traits: ?Sized + Trait, M: MemBuilder> Clone for ElementMutIterItem<'a, Traits, M>{
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}


/// Reference [`AnyVec`] iterator. Return [`ElementRef`] items.
///
/// [`AnyVec`]: crate::AnyVec
/// [`ElementRef`]: crate::element::ElementRef
pub type IterRef<'a, Traits, M> = Iter<'a, AnyVecPtr<Traits, M>, ElementRefIterItem<'a, Traits, M>>;

/// Mutable reference [`AnyVec`] iterator. Return [`ElementMut`] items.
///
/// [`AnyVec`]: crate::AnyVec
/// [`ElementMut`]: crate::element::ElementMut
pub type IterMut<'a, Traits, M> = Iter<'a, AnyVecPtr<Traits, M>, ElementMutIterItem<'a, Traits, M>>;